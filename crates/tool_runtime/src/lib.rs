//! System-driven tool execution boundaries for the Rust sandbox.
//!
//! Models may request tools, but execution remains a system concern.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use workspace_core::{ProjectRoot, WorkspaceRoot};

const GLOBAL_META_CATALOG_RELATIVE_PATH: &str = "../../resources/tool_runtime/meta_catalog.json";
const PROJECT_CATALOG_POLICY_RELATIVE_PATH: &str = "config/catalog_policy.json";
const GLOBAL_LLM_TOOL_POLICY_RELATIVE_PATH: &str =
    "../../resources/tool_runtime/llm_tool_policy.json";
const PROJECT_LLM_TOOL_POLICY_RELATIVE_PATH: &str = "config/llm_tool_policy.json";
const LLM_TOOL_POLICY_SCHEMA_VERSION: &str = "1";
const TEXT_MEASURE_TOOL_ID: &str = "text.measure";
const TEXT_MEASURE_TOOL_KIND: &str = "operational";
const TEXT_MEASURE_RUNTIME_SCOPE: &str = "single_tool_local_deterministic_sandbox";
const TEXT_MEASURE_SCHEMA_VERSION: &str = "1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerRef(String);

impl OwnerRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Governed text input accepted by the first F12.2B runtime slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextMeasureInput {
    TextRef { input_ref: String, text: String },
}

impl TextMeasureInput {
    pub fn input_ref(&self) -> &str {
        match self {
            Self::TextRef { input_ref, .. } => input_ref,
        }
    }

    pub fn text(&self) -> &str {
        match self {
            Self::TextRef { text, .. } => text,
        }
    }
}

/// Minimal input contract for the first governed F12.2B text.measure runtime slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextMeasureRequest {
    pub tool_id: String,
    pub owner_ref: OwnerRef,
    pub trace_ref: String,
    pub input: TextMeasureInput,
    pub output_root: PathBuf,
}

impl TextMeasureRequest {
    /// Build a legacy-compatible text measurement request rooted in workspace output.
    pub fn new(
        mode: impl Into<String>,
        text: impl Into<String>,
        owner_ref: impl Into<String>,
    ) -> Self {
        let mode = mode.into();
        let text = text.into();
        let owner_ref = owner_ref.into();
        Self {
            tool_id: TEXT_MEASURE_TOOL_ID.to_owned(),
            owner_ref: OwnerRef::new(owner_ref.clone()),
            trace_ref: "trace://legacy-text-measure".to_owned(),
            input: TextMeasureInput::TextRef {
                input_ref: format!("legacy://{mode}"),
                text,
            },
            output_root: PathBuf::new(),
        }
    }

    /// Build the explicit F12.2B governed text.measure request.
    pub fn governed_text_ref(
        tool_id: impl Into<String>,
        owner_ref: OwnerRef,
        trace_ref: impl Into<String>,
        input_ref: impl Into<String>,
        text: impl Into<String>,
        output_root: impl Into<PathBuf>,
    ) -> Self {
        Self {
            tool_id: tool_id.into(),
            owner_ref,
            trace_ref: trace_ref.into(),
            input: TextMeasureInput::TextRef {
                input_ref: input_ref.into(),
                text: text.into(),
            },
            output_root: output_root.into(),
        }
    }

    pub fn tool_id(&self) -> &str {
        &self.tool_id
    }

    pub fn mode(&self) -> &str {
        "text_ref"
    }

    pub fn text(&self) -> &str {
        self.input.text()
    }

    pub fn input_ref(&self) -> &str {
        self.input.input_ref()
    }

    pub fn owner_ref(&self) -> &str {
        self.owner_ref.as_str()
    }

    pub fn trace_ref(&self) -> &str {
        &self.trace_ref
    }

    pub fn output_root(&self) -> &Path {
        &self.output_root
    }
}

/// Minimal result payload persisted for the first governed F9 operational slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextMeasureResult {
    pub char_count: usize,
    pub byte_count: usize,
    pub line_count: usize,
    pub word_count: usize,
}

/// Minimal successful run outcome returned after persistence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextMeasureRunOutcome {
    run_id: String,
    run_directory: PathBuf,
    result_path: PathBuf,
    manifest_path: PathBuf,
    result: TextMeasureResult,
}

impl TextMeasureRunOutcome {
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn run_directory(&self) -> &Path {
        &self.run_directory
    }

    pub fn result_path(&self) -> &Path {
        &self.result_path
    }

    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    pub fn result(&self) -> &TextMeasureResult {
        &self.result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextMeasureRunOutput {
    pub result_path: PathBuf,
    pub manifest_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ToolRunOwnerScope {
    owner_kind_path: &'static str,
    owner_id_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ToolRunErrorRecord {
    kind: String,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ToolRunTraceRecord {
    trace_ref: String,
    trace_record_status: String,
    execution_slice: String,
    mode: String,
    deterministic: bool,
    llm_used: bool,
    external_binaries_used: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TextMeasureInputsSummary {
    mode: String,
    input_ref: String,
    owner_ref: String,
    trace_ref: String,
    text_char_count: usize,
    text_byte_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ToolRunOutputRecord {
    result_path: Option<String>,
    manifest_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ToolRunManifest {
    schema_version: String,
    run_id: String,
    tool_id: String,
    tool_kind: String,
    capability_id: String,
    runtime_scope: String,
    owner_ref: String,
    trace_ref: String,
    status: String,
    started_at: String,
    finished_at: String,
    inputs_summary: TextMeasureInputsSummary,
    outputs: ToolRunOutputRecord,
    trace: ToolRunTraceRecord,
    #[serde(default)]
    errors: Vec<ToolRunErrorRecord>,
    #[serde(default)]
    warnings: Vec<String>,
}

/// Small meta-catalog domain descriptor preserved across effective filtering.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CatalogTool {
    id: String,
    label: String,
}

impl CatalogTool {
    /// Build a simple catalog tool descriptor.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }

    /// Return the stable tool identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the human-readable tool label.
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// Small declarative recipe descriptor preserved across effective filtering.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CatalogRecipe {
    id: String,
    label: String,
}

impl CatalogRecipe {
    /// Build a simple catalog recipe descriptor.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }

    /// Return the stable recipe identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the human-readable recipe label.
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// Explicit selection result for a declarative entry inside a catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogEntrySelection {
    Tool(CatalogTool),
    Recipe(CatalogRecipe),
}

impl CatalogEntrySelection {
    /// Return the kind of selectable entry resolved from the catalog.
    pub fn kind(&self) -> CatalogEntryKind {
        match self {
            Self::Tool(_) => CatalogEntryKind::Tool,
            Self::Recipe(_) => CatalogEntryKind::Recipe,
        }
    }

    /// Return the stable identifier of the selected declarative entry.
    pub fn id(&self) -> &str {
        match self {
            Self::Tool(tool) => tool.id(),
            Self::Recipe(recipe) => recipe.id(),
        }
    }
}

/// Minimal kind tag for declarative selectable entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogEntryKind {
    Tool,
    Recipe,
}

/// Minimal declarative request for selecting an entry inside a catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogSelectionRequest {
    catalog_id: String,
    entry_id: String,
}

impl CatalogSelectionRequest {
    /// Build a minimal selection request from a catalog id and entry id.
    pub fn new(catalog_id: impl Into<String>, entry_id: impl Into<String>) -> Self {
        Self {
            catalog_id: catalog_id.into(),
            entry_id: entry_id.into(),
        }
    }

    /// Return the requested catalog identifier.
    pub fn catalog_id(&self) -> &str {
        &self.catalog_id
    }

    /// Return the requested entry identifier.
    pub fn entry_id(&self) -> &str {
        &self.entry_id
    }
}

/// Fully resolved declarative selection result for a catalog request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedCatalogEntry {
    catalog: CatalogDomain,
    entry: CatalogEntrySelection,
}

impl ResolvedCatalogEntry {
    /// Build a resolved catalog entry from a catalog and resolved entry.
    pub fn new(catalog: CatalogDomain, entry: CatalogEntrySelection) -> Self {
        Self { catalog, entry }
    }

    /// Return the resolved catalog.
    pub fn catalog(&self) -> &CatalogDomain {
        &self.catalog
    }

    /// Return the resolved entry.
    pub fn entry(&self) -> &CatalogEntrySelection {
        &self.entry
    }

    /// Return the resolved entry kind.
    pub fn entry_kind(&self) -> CatalogEntryKind {
        self.entry.kind()
    }

    /// Return the resolved catalog identifier.
    pub fn catalog_id(&self) -> &str {
        self.catalog.id()
    }

    /// Return the resolved entry identifier.
    pub fn entry_id(&self) -> &str {
        self.entry.id()
    }
}

/// Minimal execution outcome for the first runner boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolExecutionOutcome {
    Accepted,
}

/// Minimal typed result returned by the first runner boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRuntimeResult {
    catalog_id: String,
    entry_id: String,
    entry_kind: CatalogEntryKind,
    outcome: ToolExecutionOutcome,
}

impl ToolRuntimeResult {
    /// Build a typed runner result from a resolved declarative entry.
    pub fn new(
        catalog_id: impl Into<String>,
        entry_id: impl Into<String>,
        entry_kind: CatalogEntryKind,
        outcome: ToolExecutionOutcome,
    ) -> Self {
        Self {
            catalog_id: catalog_id.into(),
            entry_id: entry_id.into(),
            entry_kind,
            outcome,
        }
    }

    /// Return the catalog identifier accepted by the runner.
    pub fn catalog_id(&self) -> &str {
        &self.catalog_id
    }

    /// Return the entry identifier accepted by the runner.
    pub fn entry_id(&self) -> &str {
        &self.entry_id
    }

    /// Return the accepted entry kind.
    pub fn entry_kind(&self) -> CatalogEntryKind {
        self.entry_kind
    }

    /// Return the explicit runner outcome.
    pub fn outcome(&self) -> ToolExecutionOutcome {
        self.outcome
    }
}

/// Minimal runner boundary over an already resolved declarative entry.
#[derive(Debug, Default, Clone, Copy)]
pub struct ToolRuntimeRunner;

impl ToolRuntimeRunner {
    /// Build a zero-configuration runner for the first execution boundary.
    pub fn new() -> Self {
        Self
    }

    /// Accept a resolved catalog entry and return a typed runner result.
    ///
    /// This method does not execute any business logic yet. It only validates
    /// the declarative resolution and performs minimal typed dispatch.
    pub fn dispatch(
        &self,
        resolved_entry: &ResolvedCatalogEntry,
    ) -> Result<ToolRuntimeResult, ToolRuntimeError> {
        validate_catalog_entry_resolution(resolved_entry)?;

        Ok(ToolRuntimeResult::new(
            resolved_entry.catalog_id(),
            resolved_entry.entry_id(),
            resolved_entry.entry_kind(),
            ToolExecutionOutcome::Accepted,
        ))
    }
}

/// Small meta-catalog domain descriptor preserved across effective filtering.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CatalogDomain {
    id: String,
    label: String,
    #[serde(default)]
    tools: Vec<CatalogTool>,
    #[serde(default)]
    recipes: Vec<CatalogRecipe>,
}

impl CatalogDomain {
    /// Build a simple catalog domain descriptor.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            tools: Vec::new(),
            recipes: Vec::new(),
        }
    }

    /// Build a domain descriptor with its declarative tool list.
    pub fn with_entries(
        id: impl Into<String>,
        label: impl Into<String>,
        tools: Vec<CatalogTool>,
        recipes: Vec<CatalogRecipe>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            tools,
            recipes,
        }
    }

    /// Return the stable domain identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the human-readable domain label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Return the declared tools for this catalog domain.
    pub fn tools(&self) -> &[CatalogTool] {
        &self.tools
    }

    /// Return the declared recipes for this catalog domain.
    pub fn recipes(&self) -> &[CatalogRecipe] {
        &self.recipes
    }
}

/// Global meta-catalog available before any project-specific filtering.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MetaCatalog {
    domains: Vec<CatalogDomain>,
}

impl MetaCatalog {
    /// Build a global meta-catalog from its ordered domain list.
    pub fn new(domains: Vec<CatalogDomain>) -> Self {
        Self { domains }
    }

    /// Return the ordered catalog domains.
    pub fn domains(&self) -> &[CatalogDomain] {
        &self.domains
    }
}

/// Optional project-level policy for disabling meta-catalog domains.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct CatalogPolicy {
    #[serde(default, alias = "disabled_domains")]
    disabled_catalogs: Vec<String>,
}

impl CatalogPolicy {
    /// Build a minimal policy from disabled catalog identifiers.
    pub fn new(disabled_catalogs: Vec<String>) -> Self {
        Self { disabled_catalogs }
    }

    /// Return the disabled catalog identifiers.
    pub fn disabled_catalogs(&self) -> &[String] {
        &self.disabled_catalogs
    }
}

/// Effective meta-catalog after applying the project-level policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveMetaCatalog {
    domains: Vec<CatalogDomain>,
}

impl EffectiveMetaCatalog {
    /// Build an effective catalog from the filtered ordered domains.
    pub fn new(domains: Vec<CatalogDomain>) -> Self {
        Self { domains }
    }

    /// Return the domains still available after filtering.
    pub fn domains(&self) -> &[CatalogDomain] {
        &self.domains
    }
}

/// Declarative tool-governance policy shared by the LLM tools surface.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LlmToolPolicy {
    #[serde(default)]
    catalog_overrides: BTreeMap<String, bool>,
    #[serde(default)]
    entry_overrides: BTreeMap<String, BTreeMap<String, bool>>,
}

impl LlmToolPolicy {
    /// Build an empty policy that inherits the global allowed surface.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the catalog overrides keyed by catalog id.
    pub fn catalog_overrides(&self) -> &BTreeMap<String, bool> {
        &self.catalog_overrides
    }

    /// Return the nested entry overrides keyed by catalog id and entry id.
    pub fn entry_overrides(&self) -> &BTreeMap<String, BTreeMap<String, bool>> {
        &self.entry_overrides
    }

    /// Return the explicit override for one catalog, if present.
    pub fn catalog_override(&self, catalog_id: &str) -> Option<bool> {
        self.catalog_overrides.get(catalog_id).copied()
    }

    /// Return the explicit override for one entry, if present.
    pub fn entry_override(&self, catalog_id: &str, entry_id: &str) -> Option<bool> {
        self.entry_overrides
            .get(catalog_id)
            .and_then(|entries| entries.get(entry_id))
            .copied()
    }

    /// Set or replace a catalog override.
    pub fn set_catalog_override(&mut self, catalog_id: impl Into<String>, enabled: bool) {
        self.catalog_overrides.insert(catalog_id.into(), enabled);
    }

    /// Set or replace an entry override.
    pub fn set_entry_override(
        &mut self,
        catalog_id: impl Into<String>,
        entry_id: impl Into<String>,
        enabled: bool,
    ) {
        self.entry_overrides
            .entry(catalog_id.into())
            .or_default()
            .insert(entry_id.into(), enabled);
    }
}

/// Project-level override state shown by upper layers without becoming source of truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmToolProjectState {
    Inherit,
    Enabled,
    Disabled,
}

/// Effective enabled/disabled state after global plus optional project policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmToolEffectiveState {
    Enabled,
    Disabled,
}

/// Effective catalog-level policy row after resolving global and optional project policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveLlmCatalogPolicyRow {
    catalog_id: String,
    label: String,
    global_state: LlmToolEffectiveState,
    project_state: LlmToolProjectState,
    effective_state: LlmToolEffectiveState,
}

impl EffectiveLlmCatalogPolicyRow {
    /// Build a small effective catalog policy row.
    pub fn new(
        catalog_id: impl Into<String>,
        label: impl Into<String>,
        global_state: LlmToolEffectiveState,
        project_state: LlmToolProjectState,
        effective_state: LlmToolEffectiveState,
    ) -> Self {
        Self {
            catalog_id: catalog_id.into(),
            label: label.into(),
            global_state,
            project_state,
            effective_state,
        }
    }

    pub fn catalog_id(&self) -> &str {
        &self.catalog_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn global_state(&self) -> LlmToolEffectiveState {
        self.global_state
    }

    pub fn project_state(&self) -> LlmToolProjectState {
        self.project_state
    }

    pub fn effective_state(&self) -> LlmToolEffectiveState {
        self.effective_state
    }
}

/// Effective entry-level policy row after resolving global and optional project policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveLlmEntryPolicyRow {
    catalog_id: String,
    entry_id: String,
    label: String,
    entry_kind: CatalogEntryKind,
    global_state: LlmToolEffectiveState,
    project_state: LlmToolProjectState,
    effective_state: LlmToolEffectiveState,
}

impl EffectiveLlmEntryPolicyRow {
    /// Build a small effective entry policy row.
    pub fn new(
        catalog_id: impl Into<String>,
        entry_id: impl Into<String>,
        label: impl Into<String>,
        entry_kind: CatalogEntryKind,
        global_state: LlmToolEffectiveState,
        project_state: LlmToolProjectState,
        effective_state: LlmToolEffectiveState,
    ) -> Self {
        Self {
            catalog_id: catalog_id.into(),
            entry_id: entry_id.into(),
            label: label.into(),
            entry_kind,
            global_state,
            project_state,
            effective_state,
        }
    }

    pub fn catalog_id(&self) -> &str {
        &self.catalog_id
    }

    pub fn entry_id(&self) -> &str {
        &self.entry_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn entry_kind(&self) -> CatalogEntryKind {
        self.entry_kind
    }

    pub fn global_state(&self) -> LlmToolEffectiveState {
        self.global_state
    }

    pub fn project_state(&self) -> LlmToolProjectState {
        self.project_state
    }

    pub fn effective_state(&self) -> LlmToolEffectiveState {
        self.effective_state
    }
}

/// Fully resolved LLM tool policy summary consumed by UI and CLI layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveLlmToolPolicy {
    catalogs: Vec<EffectiveLlmCatalogPolicyRow>,
    entries: Vec<EffectiveLlmEntryPolicyRow>,
}

impl EffectiveLlmToolPolicy {
    /// Build a resolved LLM policy summary from effective catalog and entry rows.
    pub fn new(
        catalogs: Vec<EffectiveLlmCatalogPolicyRow>,
        entries: Vec<EffectiveLlmEntryPolicyRow>,
    ) -> Self {
        Self { catalogs, entries }
    }

    pub fn catalogs(&self) -> &[EffectiveLlmCatalogPolicyRow] {
        &self.catalogs
    }

    pub fn entries(&self) -> &[EffectiveLlmEntryPolicyRow] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StoredLlmToolPolicy {
    schema_version: String,
    #[serde(flatten)]
    policy: LlmToolPolicy,
}

struct LlmEntryPolicyContext {
    global_entry_override: Option<bool>,
    project_entry_override: Option<bool>,
    global_catalog_enabled: bool,
    project_catalog_override: Option<bool>,
}

/// Small tool-runtime error for catalog loading and policy parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolRuntimeError {
    GlobalMetaCatalogRead {
        path: PathBuf,
        message: String,
    },
    GlobalMetaCatalogParse {
        path: PathBuf,
        message: String,
    },
    CatalogPolicyRead {
        path: PathBuf,
        message: String,
    },
    CatalogPolicyParse {
        path: PathBuf,
        message: String,
    },
    GlobalLlmToolPolicyRead {
        path: PathBuf,
        message: String,
    },
    GlobalLlmToolPolicyParse {
        path: PathBuf,
        message: String,
    },
    GlobalLlmToolPolicyWrite {
        path: PathBuf,
        message: String,
    },
    ProjectLlmToolPolicyRead {
        path: PathBuf,
        message: String,
    },
    ProjectLlmToolPolicyParse {
        path: PathBuf,
        message: String,
    },
    ProjectLlmToolPolicyWrite {
        path: PathBuf,
        message: String,
    },
    DuplicateCatalogId {
        id: String,
    },
    DuplicateCatalogEntryId {
        catalog_id: String,
        entry_id: String,
    },
    UnknownCatalogIdsInPolicy {
        ids: Vec<String>,
    },
    UnknownCatalogIdsInLlmToolPolicy {
        ids: Vec<String>,
    },
    UnknownEntryIdsInLlmToolPolicy {
        ids: Vec<String>,
    },
    CatalogNotFound {
        id: String,
    },
    ToolNotFound {
        catalog_id: String,
        tool_id: String,
    },
    CatalogEntryNotFound {
        catalog_id: String,
        entry_id: String,
    },
    InvalidCatalogEntryResolution {
        catalog_id: String,
        entry_id: String,
        reason: &'static str,
    },
    TextMeasureValidation {
        reason: &'static str,
    },
    ToolRunOutputDirectoryCreate {
        path: PathBuf,
        message: String,
    },
    ToolRunFileWrite {
        path: PathBuf,
        message: String,
    },
    ToolRunSerialize {
        target: &'static str,
        message: String,
    },
    ToolRunRelativePath {
        path: PathBuf,
        message: String,
    },
}

impl std::fmt::Display for ToolRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlobalMetaCatalogRead { path, message } => {
                write!(
                    f,
                    "[tool_runtime:global_meta_catalog_read] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::GlobalMetaCatalogParse { path, message } => {
                write!(
                    f,
                    "[tool_runtime:global_meta_catalog_parse] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::CatalogPolicyRead { path, message } => {
                write!(
                    f,
                    "[tool_runtime:catalog_policy_read] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::CatalogPolicyParse { path, message } => {
                write!(
                    f,
                    "[tool_runtime:catalog_policy_parse] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::GlobalLlmToolPolicyRead { path, message } => {
                write!(
                    f,
                    "[tool_runtime:global_llm_tool_policy_read] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::GlobalLlmToolPolicyParse { path, message } => {
                write!(
                    f,
                    "[tool_runtime:global_llm_tool_policy_parse] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::GlobalLlmToolPolicyWrite { path, message } => {
                write!(
                    f,
                    "[tool_runtime:global_llm_tool_policy_write] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::ProjectLlmToolPolicyRead { path, message } => {
                write!(
                    f,
                    "[tool_runtime:project_llm_tool_policy_read] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::ProjectLlmToolPolicyParse { path, message } => {
                write!(
                    f,
                    "[tool_runtime:project_llm_tool_policy_parse] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::ProjectLlmToolPolicyWrite { path, message } => {
                write!(
                    f,
                    "[tool_runtime:project_llm_tool_policy_write] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::DuplicateCatalogId { id } => {
                write!(
                    f,
                    "[tool_runtime:duplicate_catalog_id] duplicate catalog id: {id}"
                )
            }
            Self::DuplicateCatalogEntryId {
                catalog_id,
                entry_id,
            } => {
                write!(
                    f,
                    "[tool_runtime:duplicate_catalog_entry_id] duplicate catalog entry id in {catalog_id}: {entry_id}"
                )
            }
            Self::UnknownCatalogIdsInPolicy { ids } => {
                write!(
                    f,
                    "[tool_runtime:unknown_catalog_ids_in_policy] unknown catalog ids: {}",
                    ids.join(", ")
                )
            }
            Self::UnknownCatalogIdsInLlmToolPolicy { ids } => {
                write!(
                    f,
                    "[tool_runtime:unknown_catalog_ids_in_llm_tool_policy] unknown catalog ids: {}",
                    ids.join(", ")
                )
            }
            Self::UnknownEntryIdsInLlmToolPolicy { ids } => {
                write!(
                    f,
                    "[tool_runtime:unknown_entry_ids_in_llm_tool_policy] unknown entry ids: {}",
                    ids.join(", ")
                )
            }
            Self::CatalogNotFound { id } => {
                write!(
                    f,
                    "[tool_runtime:catalog_not_found] catalog not found: {id}"
                )
            }
            Self::ToolNotFound {
                catalog_id,
                tool_id,
            } => {
                write!(
                    f,
                    "[tool_runtime:tool_not_found] tool not found in catalog {catalog_id}: {tool_id}"
                )
            }
            Self::CatalogEntryNotFound {
                catalog_id,
                entry_id,
            } => {
                write!(
                    f,
                    "[tool_runtime:catalog_entry_not_found] catalog entry not found in {catalog_id}: {entry_id}"
                )
            }
            Self::InvalidCatalogEntryResolution {
                catalog_id,
                entry_id,
                reason,
            } => {
                write!(
                    f,
                    "[tool_runtime:invalid_catalog_entry_resolution] invalid resolved entry in {catalog_id} for {entry_id}: {reason}"
                )
            }
            Self::TextMeasureValidation { reason } => {
                write!(f, "[tool_runtime:text_measure_validation] {reason}")
            }
            Self::ToolRunOutputDirectoryCreate { path, message } => {
                write!(
                    f,
                    "[tool_runtime:tool_run_output_directory_create] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::ToolRunFileWrite { path, message } => {
                write!(
                    f,
                    "[tool_runtime:tool_run_file_write] {} ({})",
                    message,
                    path.display()
                )
            }
            Self::ToolRunSerialize { target, message } => {
                write!(f, "[tool_runtime:tool_run_serialize] {} ({})", message, target)
            }
            Self::ToolRunRelativePath { path, message } => {
                write!(
                    f,
                    "[tool_runtime:tool_run_relative_path] {} ({})",
                    message,
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for ToolRuntimeError {}

/// Execute the minimum governed F9 operational slice for `text.measure`.
pub fn execute_text_measure(
    workspace_root: &WorkspaceRoot,
    request: &TextMeasureRequest,
) -> Result<TextMeasureRunOutcome, ToolRuntimeError> {
    let owner_scope = validate_text_measure_request(workspace_root, request)?;

    let run_id = build_text_measure_run_id();
    let output_root = resolve_text_measure_output_root(workspace_root, request);
    let run_directory = build_owner_scoped_run_directory(&output_root, &owner_scope, &run_id);
    fs::create_dir_all(&run_directory).map_err(|error| ToolRuntimeError::ToolRunOutputDirectoryCreate {
        path: run_directory.clone(),
        message: error.to_string(),
    })?;
    let run_directory =
        fs::canonicalize(&run_directory).map_err(|error| ToolRuntimeError::ToolRunOutputDirectoryCreate {
            path: run_directory.clone(),
            message: error.to_string(),
        })?;

    let started_at = run_id.clone();
    let result = measure_text(request.text());
    let result_path = run_directory.join("result.json");
    if let Err(error) = write_json_file(&result_path, &result, "result.json") {
        persist_failed_text_measure_manifest(
            workspace_root,
            &run_directory,
            &owner_scope,
            &run_id,
            &started_at,
            request,
            Some(&result_path),
            &error,
        );
        return Err(error);
    }

    let manifest_path = run_directory.join("tool_run_manifest.json");
    let manifest = match build_completed_text_measure_manifest(
        workspace_root,
        &manifest_path,
        &result_path,
        &run_id,
        &started_at,
        request,
        &result,
    ) {
        Ok(manifest) => manifest,
        Err(error) => {
            persist_failed_text_measure_manifest(
                workspace_root,
                &run_directory,
                &owner_scope,
                &run_id,
                &started_at,
                request,
                Some(&result_path),
                &error,
            );
            return Err(error);
        }
    };
    if let Err(error) = write_json_file(&manifest_path, &manifest, "tool_run_manifest.json") {
        persist_failed_text_measure_manifest(
            workspace_root,
            &run_directory,
            &owner_scope,
            &run_id,
            &started_at,
            request,
            Some(&result_path),
            &error,
        );
        return Err(error);
    }

    Ok(TextMeasureRunOutcome {
        run_id,
        run_directory,
        result_path,
        manifest_path,
        result,
    })
}

fn validate_text_measure_request(
    workspace_root: &WorkspaceRoot,
    request: &TextMeasureRequest,
) -> Result<ToolRunOwnerScope, ToolRuntimeError> {
    if request.tool_id() != TEXT_MEASURE_TOOL_ID {
        return Err(ToolRuntimeError::TextMeasureValidation {
            reason: "tool_id must be text.measure",
        });
    }

    if request.owner_ref().trim().is_empty() {
        return Err(ToolRuntimeError::TextMeasureValidation {
            reason: "owner_ref is required",
        });
    }

    if request.trace_ref().trim().is_empty() {
        return Err(ToolRuntimeError::TextMeasureValidation {
            reason: "trace_ref is required",
        });
    }

    if request.input_ref().trim().is_empty() {
        return Err(ToolRuntimeError::TextMeasureValidation {
            reason: "input_ref is required",
        });
    }

    validate_portable_ref(request.trace_ref(), "trace_ref")?;
    validate_portable_ref(request.input_ref(), "input_ref")?;
    let owner_scope = resolve_tool_run_owner_scope(request.owner_ref())?;
    validate_text_measure_output_root(workspace_root, request.output_root())?;

    Ok(owner_scope)
}

fn validate_text_measure_output_root(
    workspace_root: &WorkspaceRoot,
    output_root: &Path,
) -> Result<(), ToolRuntimeError> {
    if output_root.as_os_str().is_empty() {
        return Ok(());
    }

    if !output_root.is_absolute() {
        return Err(ToolRuntimeError::TextMeasureValidation {
            reason: "output_root must be an absolute owner-scoped path",
        });
    }

    if output_root
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(ToolRuntimeError::TextMeasureValidation {
            reason: "output_root must not contain parent-directory traversal",
        });
    }

    let governed_output_root = workspace_root.as_path().join("user").join("output");
    let normalized_output_root =
        fs::canonicalize(output_root).unwrap_or_else(|_| output_root.to_path_buf());
    let normalized_governed_output_root = fs::canonicalize(&governed_output_root).map_err(|_| {
        ToolRuntimeError::TextMeasureValidation {
            reason: "output_root must exist under user/output",
        }
    })?;

    if normalized_output_root != normalized_governed_output_root
        && !normalized_output_root.starts_with(&normalized_governed_output_root)
    {
        return Err(ToolRuntimeError::TextMeasureValidation {
            reason: "output_root must stay under user/output",
        });
    }

    Ok(())
}

fn validate_portable_ref(value: &str, label: &'static str) -> Result<(), ToolRuntimeError> {
    if value.contains('\\') || value == "." || value == ".." {
        return Err(ToolRuntimeError::TextMeasureValidation {
            reason: match label {
                "trace_ref" => "trace_ref must be portable and path-safe",
                "input_ref" => "input_ref must be portable and path-safe",
                _ => "ref must be portable and path-safe",
            },
        });
    }

    Ok(())
}

fn resolve_tool_run_owner_scope(owner_ref: &str) -> Result<ToolRunOwnerScope, ToolRuntimeError> {
    let (owner_kind, owner_id) =
        owner_ref
            .split_once("://")
            .ok_or(ToolRuntimeError::TextMeasureValidation {
                reason: "owner_ref must use chat://, documentholder://, or knowledge://",
            })?;

    let owner_kind_path = match owner_kind.to_ascii_lowercase().as_str() {
        "chat" => "chat",
        "documentholder" => "document_holder",
        "knowledge" => "knowledge",
        _ => {
            return Err(ToolRuntimeError::TextMeasureValidation {
                reason: "owner_ref must use chat://, documentholder://, or knowledge://",
            });
        }
    };

    let owner_id = owner_id.trim();
    if owner_id.is_empty() {
        return Err(ToolRuntimeError::TextMeasureValidation {
            reason: "owner_ref must include a non-empty owner id",
        });
    }

    if owner_id.contains('/')
        || owner_id.contains('\\')
        || owner_id.contains(':')
        || owner_id == "."
        || owner_id == ".."
    {
        return Err(ToolRuntimeError::TextMeasureValidation {
            reason: "owner_ref owner id must be portable and path-safe",
        });
    }

    if !owner_id
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(ToolRuntimeError::TextMeasureValidation {
            reason: "owner_ref owner id must be portable and path-safe",
        });
    }

    Ok(ToolRunOwnerScope {
        owner_kind_path,
        owner_id_path: owner_id.to_owned(),
    })
}

fn build_owner_scoped_run_directory(
    output_root: &Path,
    owner_scope: &ToolRunOwnerScope,
    run_id: &str,
) -> PathBuf {
    output_root
        .join("tool_runs")
        .join(owner_scope.owner_kind_path)
        .join(&owner_scope.owner_id_path)
        .join(run_id)
}

fn resolve_text_measure_output_root(
    workspace_root: &WorkspaceRoot,
    request: &TextMeasureRequest,
) -> PathBuf {
    if request.output_root().as_os_str().is_empty() {
        workspace_root.as_path().join("user").join("output")
    } else {
        request.output_root().to_path_buf()
    }
}

fn measure_text(text: &str) -> TextMeasureResult {
    TextMeasureResult {
        char_count: text.chars().count(),
        byte_count: text.len(),
        line_count: if text.is_empty() { 0 } else { text.lines().count() },
        word_count: text.split_whitespace().count(),
    }
}

fn build_completed_text_measure_manifest(
    workspace_root: &WorkspaceRoot,
    manifest_path: &Path,
    result_path: &Path,
    run_id: &str,
    started_at: &str,
    request: &TextMeasureRequest,
    result: &TextMeasureResult,
) -> Result<ToolRunManifest, ToolRuntimeError> {
    Ok(ToolRunManifest {
        schema_version: TEXT_MEASURE_SCHEMA_VERSION.to_owned(),
        run_id: run_id.to_owned(),
        tool_id: TEXT_MEASURE_TOOL_ID.to_owned(),
        tool_kind: TEXT_MEASURE_TOOL_KIND.to_owned(),
        capability_id: TEXT_MEASURE_TOOL_ID.to_owned(),
        runtime_scope: TEXT_MEASURE_RUNTIME_SCOPE.to_owned(),
        owner_ref: request.owner_ref().to_owned(),
        trace_ref: request.trace_ref().to_owned(),
        status: "completed".to_owned(),
        started_at: started_at.to_owned(),
        finished_at: started_at.to_owned(),
        inputs_summary: TextMeasureInputsSummary {
            mode: request.mode().to_owned(),
            input_ref: request.input_ref().to_owned(),
            owner_ref: request.owner_ref().to_owned(),
            trace_ref: request.trace_ref().to_owned(),
            text_char_count: request.text().chars().count(),
            text_byte_count: request.text().len(),
        },
        outputs: ToolRunOutputRecord {
            result_path: Some(portable_relative_path(workspace_root, result_path)?),
            manifest_path: portable_relative_path(workspace_root, manifest_path)?,
        },
        trace: ToolRunTraceRecord {
            trace_ref: request.trace_ref().to_owned(),
            trace_record_status: "metadata_created".to_owned(),
            execution_slice: "f12_2b_minimal_text_measure_runtime".to_owned(),
            mode: request.mode().to_owned(),
            deterministic: true,
            llm_used: false,
            external_binaries_used: false,
        },
        errors: Vec::new(),
        warnings: if result.word_count == 0 && !request.text().is_empty() {
            vec!["word_count may be approximate for non-whitespace languages".to_owned()]
        } else {
            Vec::new()
        },
    })
}

fn build_failed_text_measure_manifest(
    workspace_root: &WorkspaceRoot,
    manifest_path: &Path,
    result_path: Option<&Path>,
    owner_scope: &ToolRunOwnerScope,
    run_id: &str,
    started_at: &str,
    request: &TextMeasureRequest,
    error: &ToolRuntimeError,
) -> ToolRunManifest {
    ToolRunManifest {
        schema_version: TEXT_MEASURE_SCHEMA_VERSION.to_owned(),
        run_id: run_id.to_owned(),
        tool_id: TEXT_MEASURE_TOOL_ID.to_owned(),
        tool_kind: TEXT_MEASURE_TOOL_KIND.to_owned(),
        capability_id: TEXT_MEASURE_TOOL_ID.to_owned(),
        runtime_scope: TEXT_MEASURE_RUNTIME_SCOPE.to_owned(),
        owner_ref: request.owner_ref().to_owned(),
        trace_ref: request.trace_ref().to_owned(),
        status: "failed".to_owned(),
        started_at: started_at.to_owned(),
        finished_at: started_at.to_owned(),
        inputs_summary: TextMeasureInputsSummary {
            mode: request.mode().to_owned(),
            input_ref: request.input_ref().to_owned(),
            owner_ref: request.owner_ref().to_owned(),
            trace_ref: request.trace_ref().to_owned(),
            text_char_count: request.text().chars().count(),
            text_byte_count: request.text().len(),
        },
        outputs: ToolRunOutputRecord {
            result_path: result_path.and_then(|path| portable_relative_path(workspace_root, path).ok()),
            manifest_path: portable_relative_path(workspace_root, manifest_path)
                .unwrap_or_else(|_| {
                    format!(
                        "user/output/tool_runs/{}/{}/{}/tool_run_manifest.json",
                        owner_scope.owner_kind_path, owner_scope.owner_id_path, run_id
                    )
                }),
        },
        trace: ToolRunTraceRecord {
            trace_ref: request.trace_ref().to_owned(),
            trace_record_status: "metadata_created".to_owned(),
            execution_slice: "f12_2b_minimal_text_measure_runtime".to_owned(),
            mode: request.mode().to_owned(),
            deterministic: true,
            llm_used: false,
            external_binaries_used: false,
        },
        errors: vec![ToolRunErrorRecord {
            kind: "execution_error".to_owned(),
            message: error.to_string(),
        }],
        warnings: Vec::new(),
    }
}

fn persist_failed_text_measure_manifest(
    workspace_root: &WorkspaceRoot,
    run_directory: &Path,
    owner_scope: &ToolRunOwnerScope,
    run_id: &str,
    started_at: &str,
    request: &TextMeasureRequest,
    result_path: Option<&Path>,
    error: &ToolRuntimeError,
) {
    let manifest_path = run_directory.join("tool_run_manifest.json");
    let manifest = build_failed_text_measure_manifest(
        workspace_root,
        &manifest_path,
        result_path,
        owner_scope,
        run_id,
        started_at,
        request,
        error,
    );
    let _ = write_json_file(&manifest_path, &manifest, "tool_run_manifest.json");
}

fn write_json_file<T: Serialize>(
    path: &Path,
    value: &T,
    target: &'static str,
) -> Result<(), ToolRuntimeError> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| ToolRuntimeError::ToolRunSerialize {
        target,
        message: error.to_string(),
    })?;
    fs::write(path, bytes).map_err(|error| ToolRuntimeError::ToolRunFileWrite {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

fn portable_relative_path(
    workspace_root: &WorkspaceRoot,
    path: &Path,
) -> Result<String, ToolRuntimeError> {
    workspace_root
        .relative_path_for(path)
        .map(|path| path.portable_text())
        .map_err(|error| ToolRuntimeError::ToolRunRelativePath {
            path: path.to_path_buf(),
            message: error.to_string(),
        })
}

fn build_text_measure_run_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();

    format!("text_measure_{nanos}")
}

/// Load the current global meta-catalog from the fixed declarative JSON resource.
pub fn load_global_meta_catalog() -> Result<MetaCatalog, ToolRuntimeError> {
    let meta_catalog_path = global_meta_catalog_path();
    let contents = fs::read_to_string(&meta_catalog_path).map_err(|error| {
        ToolRuntimeError::GlobalMetaCatalogRead {
            path: meta_catalog_path.clone(),
            message: error.to_string(),
        }
    })?;
    let meta_catalog = parse_meta_catalog(&contents, &meta_catalog_path)?;
    validate_unique_catalog_ids(&meta_catalog)?;
    Ok(meta_catalog)
}

/// Return the fixed project policy path used by the first tool catalog layer.
pub fn catalog_policy_path(project_root: &ProjectRoot) -> PathBuf {
    project_root
        .as_path()
        .join(PROJECT_CATALOG_POLICY_RELATIVE_PATH)
}

/// Load the optional project catalog policy from the fixed project-local path.
pub fn load_catalog_policy(
    project_root: &ProjectRoot,
) -> Result<Option<CatalogPolicy>, ToolRuntimeError> {
    let policy_path = catalog_policy_path(project_root);
    if !policy_path.exists() {
        return Ok(None);
    }

    let contents =
        fs::read_to_string(&policy_path).map_err(|error| ToolRuntimeError::CatalogPolicyRead {
            path: policy_path.clone(),
            message: error.to_string(),
        })?;
    let policy = serde_json::from_str::<CatalogPolicy>(&contents).map_err(|error| {
        ToolRuntimeError::CatalogPolicyParse {
            path: policy_path,
            message: error.to_string(),
        }
    })?;

    Ok(Some(policy))
}

/// Return the fixed project-local path used by the LLM tool policy override.
pub fn project_llm_tool_policy_path(project_root: &ProjectRoot) -> PathBuf {
    project_root
        .as_path()
        .join(PROJECT_LLM_TOOL_POLICY_RELATIVE_PATH)
}

/// Load the global LLM tool policy from the fixed declarative resource.
pub fn load_global_llm_tool_policy() -> Result<LlmToolPolicy, ToolRuntimeError> {
    let policy_path = global_llm_tool_policy_path();
    let contents = fs::read_to_string(&policy_path).map_err(|error| {
        ToolRuntimeError::GlobalLlmToolPolicyRead {
            path: policy_path.clone(),
            message: error.to_string(),
        }
    })?;

    parse_stored_llm_tool_policy(
        &contents,
        &policy_path,
        ToolRuntimeError::GlobalLlmToolPolicyParse {
            path: policy_path.clone(),
            message: String::new(),
        },
    )
}

/// Load the optional project-local LLM tool policy override.
pub fn load_project_llm_tool_policy(
    project_root: &ProjectRoot,
) -> Result<Option<LlmToolPolicy>, ToolRuntimeError> {
    let policy_path = project_llm_tool_policy_path(project_root);
    if !policy_path.exists() {
        return Ok(None);
    }

    let contents = fs::read_to_string(&policy_path).map_err(|error| {
        ToolRuntimeError::ProjectLlmToolPolicyRead {
            path: policy_path.clone(),
            message: error.to_string(),
        }
    })?;

    parse_stored_llm_tool_policy(
        &contents,
        &policy_path,
        ToolRuntimeError::ProjectLlmToolPolicyParse {
            path: policy_path.clone(),
            message: String::new(),
        },
    )
    .map(Some)
}

/// Persist the current global LLM tool policy to the fixed resource path.
pub fn save_global_llm_tool_policy(policy: &LlmToolPolicy) -> Result<(), ToolRuntimeError> {
    let policy_path = global_llm_tool_policy_path();
    let stored_policy = StoredLlmToolPolicy {
        schema_version: LLM_TOOL_POLICY_SCHEMA_VERSION.to_owned(),
        policy: policy.clone(),
    };
    write_stored_llm_tool_policy(
        &policy_path,
        &stored_policy,
        ToolRuntimeError::GlobalLlmToolPolicyWrite {
            path: policy_path.clone(),
            message: String::new(),
        },
    )
}

/// Persist the project-local LLM tool policy override under the fixed config path.
pub fn save_project_llm_tool_policy(
    project_root: &ProjectRoot,
    policy: &LlmToolPolicy,
) -> Result<(), ToolRuntimeError> {
    let policy_path = project_llm_tool_policy_path(project_root);
    let stored_policy = StoredLlmToolPolicy {
        schema_version: LLM_TOOL_POLICY_SCHEMA_VERSION.to_owned(),
        policy: policy.clone(),
    };
    write_stored_llm_tool_policy(
        &policy_path,
        &stored_policy,
        ToolRuntimeError::ProjectLlmToolPolicyWrite {
            path: policy_path.clone(),
            message: String::new(),
        },
    )
}

/// Build the effective meta-catalog by conservatively filtering disabled domains.
pub fn build_effective_meta_catalog(
    meta_catalog: &MetaCatalog,
    policy: Option<&CatalogPolicy>,
) -> Result<EffectiveMetaCatalog, ToolRuntimeError> {
    validate_unique_catalog_ids(meta_catalog)?;
    validate_unique_catalog_entry_ids(meta_catalog)?;

    let disabled = policy
        .map(|policy| {
            policy
                .disabled_catalogs()
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();

    let available_catalog_ids = meta_catalog
        .domains()
        .iter()
        .map(|domain| domain.id().to_owned())
        .collect::<BTreeSet<_>>();
    let unknown_catalog_ids = disabled
        .iter()
        .filter(|catalog_id| !available_catalog_ids.contains(*catalog_id))
        .cloned()
        .collect::<Vec<_>>();
    if !unknown_catalog_ids.is_empty() {
        return Err(ToolRuntimeError::UnknownCatalogIdsInPolicy {
            ids: unknown_catalog_ids,
        });
    }

    Ok(EffectiveMetaCatalog::new(
        meta_catalog
            .domains()
            .iter()
            .filter(|domain| !disabled.contains(domain.id()))
            .cloned()
            .collect(),
    ))
}

/// Resolve the effective LLM tool policy from the global policy plus optional project override.
pub fn resolve_effective_llm_tool_policy(
    meta_catalog: &MetaCatalog,
    global_policy: &LlmToolPolicy,
    project_policy: Option<&LlmToolPolicy>,
) -> Result<EffectiveLlmToolPolicy, ToolRuntimeError> {
    validate_unique_catalog_ids(meta_catalog)?;
    validate_unique_catalog_entry_ids(meta_catalog)?;
    validate_llm_tool_policy_ids(meta_catalog, global_policy)?;
    if let Some(project_policy) = project_policy {
        validate_llm_tool_policy_ids(meta_catalog, project_policy)?;
    }

    let catalogs = meta_catalog
        .domains()
        .iter()
        .map(|catalog| {
            let global_state = resolve_effective_bool(
                global_policy.catalog_override(catalog.id()).unwrap_or(true),
            );
            let project_state = project_policy_state(project_policy, catalog.id(), None);
            let effective_state = match (global_state, project_state) {
                (LlmToolEffectiveState::Enabled, LlmToolProjectState::Disabled) => {
                    LlmToolEffectiveState::Disabled
                }
                (LlmToolEffectiveState::Enabled, _) => LlmToolEffectiveState::Enabled,
                (LlmToolEffectiveState::Disabled, _) => LlmToolEffectiveState::Disabled,
            };

            EffectiveLlmCatalogPolicyRow::new(
                catalog.id(),
                catalog.label(),
                global_state,
                project_state,
                effective_state,
            )
        })
        .collect::<Vec<_>>();

    let mut entries = Vec::new();
    for catalog in meta_catalog.domains() {
        let global_catalog_enabled = global_policy.catalog_override(catalog.id()).unwrap_or(true);
        let project_catalog_state =
            project_policy.and_then(|policy| policy.catalog_override(catalog.id()));

        for tool in catalog.tools() {
            entries.push(build_effective_llm_entry_row(
                catalog,
                tool.id(),
                tool.label(),
                CatalogEntryKind::Tool,
                LlmEntryPolicyContext {
                    global_entry_override: global_policy.entry_override(catalog.id(), tool.id()),
                    project_entry_override: project_policy
                        .and_then(|policy| policy.entry_override(catalog.id(), tool.id())),
                    global_catalog_enabled,
                    project_catalog_override: project_catalog_state,
                },
            ));
        }

        for recipe in catalog.recipes() {
            entries.push(build_effective_llm_entry_row(
                catalog,
                recipe.id(),
                recipe.label(),
                CatalogEntryKind::Recipe,
                LlmEntryPolicyContext {
                    global_entry_override: global_policy.entry_override(catalog.id(), recipe.id()),
                    project_entry_override: project_policy
                        .and_then(|policy| policy.entry_override(catalog.id(), recipe.id())),
                    global_catalog_enabled,
                    project_catalog_override: project_catalog_state,
                },
            ));
        }
    }

    Ok(EffectiveLlmToolPolicy::new(catalogs, entries))
}

/// Resolve a catalog selection against the already filtered effective catalog.
pub fn resolve_catalog_selection(
    effective_meta_catalog: &EffectiveMetaCatalog,
    catalog_id: &str,
) -> Result<CatalogDomain, ToolRuntimeError> {
    effective_meta_catalog
        .domains()
        .iter()
        .find(|domain| domain.id() == catalog_id)
        .cloned()
        .ok_or_else(|| ToolRuntimeError::CatalogNotFound {
            id: catalog_id.to_owned(),
        })
}

/// Resolve a tool selection inside an already resolved catalog domain.
pub fn resolve_tool_selection(
    catalog: &CatalogDomain,
    tool_id: &str,
) -> Result<CatalogTool, ToolRuntimeError> {
    catalog
        .tools()
        .iter()
        .find(|tool| tool.id() == tool_id)
        .cloned()
        .ok_or_else(|| ToolRuntimeError::ToolNotFound {
            catalog_id: catalog.id().to_owned(),
            tool_id: tool_id.to_owned(),
        })
}

/// Resolve a declarative entry selection inside an already resolved catalog domain.
pub fn resolve_catalog_entry_selection(
    catalog: &CatalogDomain,
    entry_id: &str,
) -> Result<CatalogEntrySelection, ToolRuntimeError> {
    if let Some(tool) = catalog
        .tools()
        .iter()
        .find(|tool| tool.id() == entry_id)
        .cloned()
    {
        return Ok(CatalogEntrySelection::Tool(tool));
    }

    if let Some(recipe) = catalog
        .recipes()
        .iter()
        .find(|recipe| recipe.id() == entry_id)
        .cloned()
    {
        return Ok(CatalogEntrySelection::Recipe(recipe));
    }

    Err(ToolRuntimeError::CatalogEntryNotFound {
        catalog_id: catalog.id().to_owned(),
        entry_id: entry_id.to_owned(),
    })
}

/// Resolve a full declarative catalog request against the effective catalog.
pub fn resolve_catalog_request(
    effective_meta_catalog: &EffectiveMetaCatalog,
    request: &CatalogSelectionRequest,
) -> Result<ResolvedCatalogEntry, ToolRuntimeError> {
    let catalog = resolve_catalog_selection(effective_meta_catalog, request.catalog_id())?;
    let entry = resolve_catalog_entry_selection(&catalog, request.entry_id())?;
    Ok(ResolvedCatalogEntry::new(catalog, entry))
}

/// Validate the minimal declarative consistency of a resolved catalog entry.
pub fn validate_catalog_entry_resolution(
    resolved_entry: &ResolvedCatalogEntry,
) -> Result<(), ToolRuntimeError> {
    let catalog = resolved_entry.catalog();

    match resolved_entry.entry() {
        CatalogEntrySelection::Tool(tool) => {
            validate_entry_identity(catalog.id(), tool.id(), tool.label())?;

            if catalog.tools().iter().any(|declared| declared == tool) {
                Ok(())
            } else {
                Err(ToolRuntimeError::InvalidCatalogEntryResolution {
                    catalog_id: catalog.id().to_owned(),
                    entry_id: tool.id().to_owned(),
                    reason: "resolved tool is not declared in the resolved catalog",
                })
            }
        }
        CatalogEntrySelection::Recipe(recipe) => {
            validate_entry_identity(catalog.id(), recipe.id(), recipe.label())?;

            if catalog.recipes().iter().any(|declared| declared == recipe) {
                Ok(())
            } else {
                Err(ToolRuntimeError::InvalidCatalogEntryResolution {
                    catalog_id: catalog.id().to_owned(),
                    entry_id: recipe.id().to_owned(),
                    reason: "resolved recipe is not declared in the resolved catalog",
                })
            }
        }
    }
}

/// Load the global mock and optional project policy, then build the effective catalog.
pub fn load_effective_meta_catalog(
    project_root: &ProjectRoot,
) -> Result<EffectiveMetaCatalog, ToolRuntimeError> {
    let meta_catalog = load_global_meta_catalog()?;
    let policy = load_catalog_policy(project_root)?;
    build_effective_meta_catalog(&meta_catalog, policy.as_ref())
}

fn global_llm_tool_policy_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GLOBAL_LLM_TOOL_POLICY_RELATIVE_PATH)
}

fn global_meta_catalog_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GLOBAL_META_CATALOG_RELATIVE_PATH)
}

fn parse_meta_catalog(contents: &str, path: &Path) -> Result<MetaCatalog, ToolRuntimeError> {
    serde_json::from_str::<MetaCatalog>(contents).map_err(|error| {
        ToolRuntimeError::GlobalMetaCatalogParse {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    })
}

fn parse_stored_llm_tool_policy(
    contents: &str,
    path: &Path,
    parse_error: ToolRuntimeError,
) -> Result<LlmToolPolicy, ToolRuntimeError> {
    serde_json::from_str::<StoredLlmToolPolicy>(contents)
        .map(|stored| stored.policy)
        .map_err(|error| match parse_error {
            ToolRuntimeError::GlobalLlmToolPolicyParse { .. } => {
                ToolRuntimeError::GlobalLlmToolPolicyParse {
                    path: path.to_path_buf(),
                    message: error.to_string(),
                }
            }
            ToolRuntimeError::ProjectLlmToolPolicyParse { .. } => {
                ToolRuntimeError::ProjectLlmToolPolicyParse {
                    path: path.to_path_buf(),
                    message: error.to_string(),
                }
            }
            _ => parse_error,
        })
}

fn write_stored_llm_tool_policy(
    path: &Path,
    stored_policy: &StoredLlmToolPolicy,
    write_error: ToolRuntimeError,
) -> Result<(), ToolRuntimeError> {
    let parent = path.parent().expect("llm tool policy parent");
    fs::create_dir_all(parent).map_err(|error| match &write_error {
        ToolRuntimeError::GlobalLlmToolPolicyWrite { .. } => {
            ToolRuntimeError::GlobalLlmToolPolicyWrite {
                path: path.to_path_buf(),
                message: error.to_string(),
            }
        }
        ToolRuntimeError::ProjectLlmToolPolicyWrite { .. } => {
            ToolRuntimeError::ProjectLlmToolPolicyWrite {
                path: path.to_path_buf(),
                message: error.to_string(),
            }
        }
        _ => write_error.clone(),
    })?;

    let contents =
        serde_json::to_string_pretty(stored_policy).map_err(|error| match &write_error {
            ToolRuntimeError::GlobalLlmToolPolicyWrite { .. } => {
                ToolRuntimeError::GlobalLlmToolPolicyWrite {
                    path: path.to_path_buf(),
                    message: error.to_string(),
                }
            }
            ToolRuntimeError::ProjectLlmToolPolicyWrite { .. } => {
                ToolRuntimeError::ProjectLlmToolPolicyWrite {
                    path: path.to_path_buf(),
                    message: error.to_string(),
                }
            }
            _ => write_error.clone(),
        })?;

    fs::write(path, format!("{contents}\n")).map_err(|error| match write_error {
        ToolRuntimeError::GlobalLlmToolPolicyWrite { .. } => {
            ToolRuntimeError::GlobalLlmToolPolicyWrite {
                path: path.to_path_buf(),
                message: error.to_string(),
            }
        }
        ToolRuntimeError::ProjectLlmToolPolicyWrite { .. } => {
            ToolRuntimeError::ProjectLlmToolPolicyWrite {
                path: path.to_path_buf(),
                message: error.to_string(),
            }
        }
        _ => write_error,
    })
}

fn validate_unique_catalog_ids(meta_catalog: &MetaCatalog) -> Result<(), ToolRuntimeError> {
    let mut seen_catalog_ids = BTreeSet::new();
    for domain in meta_catalog.domains() {
        let catalog_id = domain.id().to_owned();
        if !seen_catalog_ids.insert(catalog_id.clone()) {
            return Err(ToolRuntimeError::DuplicateCatalogId { id: catalog_id });
        }
    }
    Ok(())
}

fn validate_unique_catalog_entry_ids(meta_catalog: &MetaCatalog) -> Result<(), ToolRuntimeError> {
    for domain in meta_catalog.domains() {
        let mut seen_entry_ids = BTreeSet::new();

        for tool in domain.tools() {
            let entry_id = tool.id().to_owned();
            if !seen_entry_ids.insert(entry_id.clone()) {
                return Err(ToolRuntimeError::DuplicateCatalogEntryId {
                    catalog_id: domain.id().to_owned(),
                    entry_id,
                });
            }
        }

        for recipe in domain.recipes() {
            let entry_id = recipe.id().to_owned();
            if !seen_entry_ids.insert(entry_id.clone()) {
                return Err(ToolRuntimeError::DuplicateCatalogEntryId {
                    catalog_id: domain.id().to_owned(),
                    entry_id,
                });
            }
        }
    }

    Ok(())
}

fn validate_llm_tool_policy_ids(
    meta_catalog: &MetaCatalog,
    policy: &LlmToolPolicy,
) -> Result<(), ToolRuntimeError> {
    let known_catalog_ids = meta_catalog
        .domains()
        .iter()
        .map(|domain| domain.id().to_owned())
        .collect::<BTreeSet<_>>();
    let unknown_catalog_ids = policy
        .catalog_overrides()
        .keys()
        .filter(|catalog_id| !known_catalog_ids.contains(*catalog_id))
        .cloned()
        .collect::<Vec<_>>();
    if !unknown_catalog_ids.is_empty() {
        return Err(ToolRuntimeError::UnknownCatalogIdsInLlmToolPolicy {
            ids: unknown_catalog_ids,
        });
    }

    let known_entry_ids = meta_catalog
        .domains()
        .iter()
        .flat_map(|domain| {
            domain
                .tools()
                .iter()
                .map(|tool| format!("{}::{}", domain.id(), tool.id()))
                .chain(
                    domain
                        .recipes()
                        .iter()
                        .map(|recipe| format!("{}::{}", domain.id(), recipe.id())),
                )
                .collect::<Vec<_>>()
        })
        .collect::<BTreeSet<_>>();
    let unknown_entry_ids = policy
        .entry_overrides()
        .iter()
        .flat_map(|(catalog_id, entries)| {
            entries
                .keys()
                .map(|entry_id| format!("{catalog_id}::{entry_id}"))
                .collect::<Vec<_>>()
        })
        .filter(|entry_key| !known_entry_ids.contains(entry_key))
        .collect::<Vec<_>>();
    if !unknown_entry_ids.is_empty() {
        return Err(ToolRuntimeError::UnknownEntryIdsInLlmToolPolicy {
            ids: unknown_entry_ids,
        });
    }

    Ok(())
}

fn resolve_effective_bool(enabled: bool) -> LlmToolEffectiveState {
    if enabled {
        LlmToolEffectiveState::Enabled
    } else {
        LlmToolEffectiveState::Disabled
    }
}

fn project_policy_state(
    project_policy: Option<&LlmToolPolicy>,
    catalog_id: &str,
    entry_id: Option<&str>,
) -> LlmToolProjectState {
    match project_policy.and_then(|policy| {
        entry_id
            .and_then(|entry_id| policy.entry_override(catalog_id, entry_id))
            .or_else(|| policy.catalog_override(catalog_id))
    }) {
        Some(true) => LlmToolProjectState::Enabled,
        Some(false) => LlmToolProjectState::Disabled,
        None => LlmToolProjectState::Inherit,
    }
}

fn build_effective_llm_entry_row(
    catalog: &CatalogDomain,
    entry_id: &str,
    label: &str,
    entry_kind: CatalogEntryKind,
    context: LlmEntryPolicyContext,
) -> EffectiveLlmEntryPolicyRow {
    let global_enabled =
        context.global_catalog_enabled && context.global_entry_override.unwrap_or(true);
    let project_state = match context
        .project_entry_override
        .or(context.project_catalog_override)
    {
        Some(true) => LlmToolProjectState::Enabled,
        Some(false) => LlmToolProjectState::Disabled,
        None => LlmToolProjectState::Inherit,
    };
    let effective_enabled = global_enabled
        && context.project_catalog_override.unwrap_or(true)
        && context.project_entry_override.unwrap_or(true);

    EffectiveLlmEntryPolicyRow::new(
        catalog.id(),
        entry_id,
        label,
        entry_kind,
        resolve_effective_bool(global_enabled),
        project_state,
        resolve_effective_bool(effective_enabled),
    )
}

fn validate_entry_identity(
    catalog_id: &str,
    entry_id: &str,
    entry_label: &str,
) -> Result<(), ToolRuntimeError> {
    if entry_id.trim().is_empty() {
        return Err(ToolRuntimeError::InvalidCatalogEntryResolution {
            catalog_id: catalog_id.to_owned(),
            entry_id: entry_id.to_owned(),
            reason: "entry id must not be empty",
        });
    }

    if entry_label.trim().is_empty() {
        return Err(ToolRuntimeError::InvalidCatalogEntryResolution {
            catalog_id: catalog_id.to_owned(),
            entry_id: entry_id.to_owned(),
            reason: "entry label must not be empty",
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        build_effective_meta_catalog, catalog_policy_path, execute_text_measure,
        global_meta_catalog_path,
        load_catalog_policy, load_effective_meta_catalog, load_global_llm_tool_policy,
        load_global_meta_catalog, load_project_llm_tool_policy, parse_meta_catalog,
        project_llm_tool_policy_path, resolve_catalog_entry_selection, resolve_catalog_request,
        resolve_catalog_selection, resolve_effective_llm_tool_policy, resolve_tool_selection,
        save_project_llm_tool_policy, validate_catalog_entry_resolution,
        validate_unique_catalog_entry_ids, validate_unique_catalog_ids, CatalogDomain,
        CatalogEntryKind, CatalogEntrySelection, CatalogPolicy, CatalogRecipe,
        CatalogSelectionRequest, CatalogTool, EffectiveMetaCatalog, LlmToolEffectiveState,
        LlmToolPolicy, LlmToolProjectState, MetaCatalog, OwnerRef, ResolvedCatalogEntry,
        TextMeasureRequest, ToolExecutionOutcome, ToolRuntimeError, ToolRuntimeResult,
        ToolRuntimeRunner,
    };
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use workspace_core::{ProjectRoot, WorkspaceRoot};

    #[test]
    fn builds_effective_catalog_without_policy() {
        let meta_catalog = MetaCatalog::new(vec![
            CatalogDomain::new("project", "Project"),
            CatalogDomain::new("knowledge", "Knowledge"),
        ]);

        let effective =
            build_effective_meta_catalog(&meta_catalog, None).expect("effective catalog");

        assert_eq!(
            effective
                .domains()
                .iter()
                .map(|domain| domain.id())
                .collect::<Vec<_>>(),
            vec!["project", "knowledge"]
        );
    }

    #[test]
    fn removes_domains_disabled_by_policy() {
        let meta_catalog = MetaCatalog::new(vec![
            CatalogDomain::new("project", "Project"),
            CatalogDomain::new("workspace", "Workspace"),
            CatalogDomain::new("knowledge", "Knowledge"),
        ]);
        let policy = CatalogPolicy::new(vec!["workspace".to_owned()]);

        let effective =
            build_effective_meta_catalog(&meta_catalog, Some(&policy)).expect("effective catalog");

        assert_eq!(
            effective
                .domains()
                .iter()
                .map(|domain| domain.id())
                .collect::<Vec<_>>(),
            vec!["project", "knowledge"]
        );
    }

    #[test]
    fn loads_optional_catalog_policy_from_fixed_project_path() {
        let workspace_dir = unique_temp_dir("tool_runtime_policy_load");
        let project_root = create_project_root(&workspace_dir);
        let policy_path = catalog_policy_path(&project_root);
        fs::create_dir_all(policy_path.parent().expect("policy parent"))
            .expect("create config dir");
        fs::write(
            &policy_path,
            r#"{
  "disabled_catalogs": ["knowledge"]
}"#,
        )
        .expect("write catalog policy");

        let policy = load_catalog_policy(&project_root)
            .expect("load catalog policy")
            .expect("catalog policy present");

        assert_eq!(policy.disabled_catalogs(), &["knowledge".to_owned()]);

        let workspace_path = workspace_dir.clone();
        fs::remove_dir_all(workspace_path).expect("cleanup workspace dir");
    }

    #[test]
    fn load_effective_catalog_uses_mock_and_optional_project_policy() {
        let workspace_dir = unique_temp_dir("tool_runtime_effective_catalog");
        let project_root = create_project_root(&workspace_dir);
        let policy_path = catalog_policy_path(&project_root);
        fs::create_dir_all(policy_path.parent().expect("policy parent"))
            .expect("create config dir");
        fs::write(
            &policy_path,
            r#"{
  "disabled_catalogs": ["workspace"]
}"#,
        )
        .expect("write catalog policy");

        let effective = load_effective_meta_catalog(&project_root).expect("effective meta catalog");

        assert_eq!(
            load_global_meta_catalog()
                .expect("load global meta catalog")
                .domains()
                .len()
                - effective.domains().len(),
            1
        );
        assert_eq!(
            effective
                .domains()
                .iter()
                .map(|domain| domain.id())
                .collect::<Vec<_>>(),
            vec!["project", "knowledge"]
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn loads_global_meta_catalog_from_fixed_declarative_json() {
        let meta_catalog = load_global_meta_catalog().expect("load global meta catalog");

        assert_eq!(
            global_meta_catalog_path()
                .file_name()
                .and_then(|name| name.to_str()),
            Some("meta_catalog.json")
        );
        assert_eq!(
            meta_catalog
                .domains()
                .iter()
                .map(|domain| domain.id())
                .collect::<Vec<_>>(),
            vec!["project", "workspace", "knowledge"]
        );
        assert_eq!(
            meta_catalog.domains()[0]
                .tools()
                .iter()
                .map(|tool| tool.id())
                .collect::<Vec<_>>(),
            vec!["manifest", "surface"]
        );
        assert_eq!(
            meta_catalog.domains()[0]
                .recipes()
                .iter()
                .map(|recipe| recipe.id())
                .collect::<Vec<_>>(),
            vec!["project_summary"]
        );
    }

    #[test]
    fn loads_global_llm_tool_policy_from_fixed_resource() {
        let policy = load_global_llm_tool_policy().expect("load global llm tool policy");

        assert!(policy.catalog_overrides().is_empty());
        assert!(policy.entry_overrides().is_empty());
    }

    #[test]
    fn saves_and_loads_project_llm_tool_policy_override() {
        let workspace_dir = unique_temp_dir("tool_runtime_llm_project_policy");
        let project_root = create_project_root(&workspace_dir);
        let mut policy = LlmToolPolicy::new();
        policy.set_catalog_override("knowledge", false);
        policy.set_entry_override("project", "manifest", false);

        save_project_llm_tool_policy(&project_root, &policy).expect("save project llm policy");
        let loaded = load_project_llm_tool_policy(&project_root)
            .expect("load project llm policy")
            .expect("project llm policy present");

        assert_eq!(loaded, policy);
        assert_eq!(
            project_llm_tool_policy_path(&project_root)
                .file_name()
                .and_then(|name| name.to_str()),
            Some("llm_tool_policy.json")
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn resolves_effective_llm_tool_policy_from_global_and_project_override() {
        let meta_catalog = load_global_meta_catalog().expect("load meta catalog");
        let global_policy = load_global_llm_tool_policy().expect("load global llm tool policy");
        let mut project_policy = LlmToolPolicy::new();
        project_policy.set_catalog_override("knowledge", false);
        project_policy.set_entry_override("project", "surface", false);

        let effective =
            resolve_effective_llm_tool_policy(&meta_catalog, &global_policy, Some(&project_policy))
                .expect("resolve effective llm policy");

        let knowledge_catalog = effective
            .catalogs()
            .iter()
            .find(|row| row.catalog_id() == "knowledge")
            .expect("knowledge catalog");
        assert_eq!(
            knowledge_catalog.global_state(),
            LlmToolEffectiveState::Enabled
        );
        assert_eq!(
            knowledge_catalog.project_state(),
            LlmToolProjectState::Disabled
        );
        assert_eq!(
            knowledge_catalog.effective_state(),
            LlmToolEffectiveState::Disabled
        );

        let surface_entry = effective
            .entries()
            .iter()
            .find(|row| row.catalog_id() == "project" && row.entry_id() == "surface")
            .expect("surface entry");
        assert_eq!(surface_entry.project_state(), LlmToolProjectState::Disabled);
        assert_eq!(
            surface_entry.effective_state(),
            LlmToolEffectiveState::Disabled
        );
    }

    #[test]
    fn project_policy_cannot_enable_entry_globally_disabled() {
        let meta_catalog = load_global_meta_catalog().expect("load meta catalog");
        let mut global_policy = LlmToolPolicy::new();
        global_policy.set_entry_override("project", "manifest", false);
        let mut project_policy = LlmToolPolicy::new();
        project_policy.set_entry_override("project", "manifest", true);

        let effective =
            resolve_effective_llm_tool_policy(&meta_catalog, &global_policy, Some(&project_policy))
                .expect("resolve effective llm policy");

        let manifest_entry = effective
            .entries()
            .iter()
            .find(|row| row.catalog_id() == "project" && row.entry_id() == "manifest")
            .expect("manifest entry");
        assert_eq!(manifest_entry.project_state(), LlmToolProjectState::Enabled);
        assert_eq!(
            manifest_entry.global_state(),
            LlmToolEffectiveState::Disabled
        );
        assert_eq!(
            manifest_entry.effective_state(),
            LlmToolEffectiveState::Disabled
        );
    }

    #[test]
    fn rejects_unknown_catalog_ids_in_policy() {
        let meta_catalog = MetaCatalog::new(vec![
            CatalogDomain::new("project", "Project"),
            CatalogDomain::new("knowledge", "Knowledge"),
        ]);
        let policy = CatalogPolicy::new(vec!["llm".to_owned()]);

        let error =
            build_effective_meta_catalog(&meta_catalog, Some(&policy)).expect_err("unknown ids");

        assert_eq!(
            error,
            ToolRuntimeError::UnknownCatalogIdsInPolicy {
                ids: vec!["llm".to_owned()]
            }
        );
    }

    #[test]
    fn rejects_duplicate_catalog_ids_in_meta_catalog() {
        let duplicate_json = r#"{
  "domains": [
    { "id": "project", "label": "Project" },
    { "id": "project", "label": "Project Duplicate" }
  ]
}"#;
        let path = PathBuf::from("resources/tool_runtime/meta_catalog.json");
        let meta_catalog = parse_meta_catalog(duplicate_json, &path).expect("parse duplicate json");

        let error = validate_unique_catalog_ids(&meta_catalog).expect_err("duplicate ids");

        assert_eq!(
            error,
            ToolRuntimeError::DuplicateCatalogId {
                id: "project".to_owned()
            }
        );
    }

    #[test]
    fn rejects_duplicate_catalog_entry_ids_in_same_domain() {
        let duplicate_json = r#"{
  "domains": [
    {
      "id": "project",
      "label": "Project",
      "tools": [{ "id": "manifest", "label": "Manifest" }],
      "recipes": [{ "id": "manifest", "label": "Project Summary" }]
    }
  ]
}"#;
        let path = PathBuf::from("resources/tool_runtime/meta_catalog.json");
        let meta_catalog = parse_meta_catalog(duplicate_json, &path).expect("parse duplicate json");

        let error =
            validate_unique_catalog_entry_ids(&meta_catalog).expect_err("duplicate entry ids");

        assert_eq!(
            error,
            ToolRuntimeError::DuplicateCatalogEntryId {
                catalog_id: "project".to_owned(),
                entry_id: "manifest".to_owned()
            }
        );
    }

    #[test]
    fn accepts_legacy_disabled_domains_alias_in_policy() {
        let workspace_dir = unique_temp_dir("tool_runtime_policy_alias");
        let project_root = create_project_root(&workspace_dir);
        let policy_path = catalog_policy_path(&project_root);
        fs::create_dir_all(policy_path.parent().expect("policy parent"))
            .expect("create config dir");
        fs::write(
            &policy_path,
            r#"{
  "disabled_domains": ["knowledge"]
}"#,
        )
        .expect("write catalog policy");

        let policy = load_catalog_policy(&project_root)
            .expect("load catalog policy")
            .expect("catalog policy present");

        assert_eq!(policy.disabled_catalogs(), &["knowledge".to_owned()]);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn resolves_catalog_selection_when_catalog_exists() {
        let effective_meta_catalog = EffectiveMetaCatalog::new(vec![
            CatalogDomain::new("project", "Project"),
            CatalogDomain::new("knowledge", "Knowledge"),
        ]);

        let catalog =
            resolve_catalog_selection(&effective_meta_catalog, "knowledge").expect("catalog");

        assert_eq!(catalog.id(), "knowledge");
        assert_eq!(catalog.label(), "Knowledge");
    }

    #[test]
    fn fails_when_selected_catalog_does_not_exist() {
        let effective_meta_catalog = EffectiveMetaCatalog::new(vec![
            CatalogDomain::new("project", "Project"),
            CatalogDomain::new("knowledge", "Knowledge"),
        ]);

        let error =
            resolve_catalog_selection(&effective_meta_catalog, "workspace").expect_err("missing");

        assert_eq!(
            error,
            ToolRuntimeError::CatalogNotFound {
                id: "workspace".to_owned()
            }
        );
    }

    #[test]
    fn resolves_only_catalogs_left_enabled_after_policy_filtering() {
        let meta_catalog = MetaCatalog::new(vec![
            CatalogDomain::with_entries(
                "project",
                "Project",
                vec![CatalogTool::new("manifest", "Manifest")],
                vec![],
            ),
            CatalogDomain::new("workspace", "Workspace"),
            CatalogDomain::with_entries(
                "knowledge",
                "Knowledge",
                vec![CatalogTool::new("search", "Search")],
                vec![],
            ),
        ]);
        let policy = CatalogPolicy::new(vec!["workspace".to_owned()]);
        let effective_meta_catalog =
            build_effective_meta_catalog(&meta_catalog, Some(&policy)).expect("effective catalog");

        let catalog =
            resolve_catalog_selection(&effective_meta_catalog, "knowledge").expect("catalog");
        let missing =
            resolve_catalog_selection(&effective_meta_catalog, "workspace").expect_err("missing");

        assert_eq!(catalog.id(), "knowledge");
        assert_eq!(
            missing,
            ToolRuntimeError::CatalogNotFound {
                id: "workspace".to_owned()
            }
        );
    }

    #[test]
    fn resolves_tool_selection_when_tool_exists() {
        let catalog = CatalogDomain::with_entries(
            "project",
            "Project",
            vec![
                CatalogTool::new("manifest", "Manifest"),
                CatalogTool::new("surface", "Surface"),
            ],
            vec![],
        );

        let tool = resolve_tool_selection(&catalog, "surface").expect("tool");

        assert_eq!(tool.id(), "surface");
        assert_eq!(tool.label(), "Surface");
    }

    #[test]
    fn fails_when_selected_tool_does_not_exist() {
        let catalog = CatalogDomain::with_entries(
            "project",
            "Project",
            vec![CatalogTool::new("manifest", "Manifest")],
            vec![],
        );

        let error = resolve_tool_selection(&catalog, "viewer").expect_err("missing tool");

        assert_eq!(
            error,
            ToolRuntimeError::ToolNotFound {
                catalog_id: "project".to_owned(),
                tool_id: "viewer".to_owned()
            }
        );
    }

    #[test]
    fn resolves_tool_selection_from_declarative_catalog_structure() {
        let meta_catalog = load_global_meta_catalog().expect("load global meta catalog");
        let effective_meta_catalog =
            build_effective_meta_catalog(&meta_catalog, None).expect("effective catalog");
        let project_catalog =
            resolve_catalog_selection(&effective_meta_catalog, "project").expect("project catalog");

        let tool = resolve_tool_selection(&project_catalog, "manifest").expect("manifest tool");

        assert_eq!(project_catalog.label(), "Project");
        assert_eq!(tool.id(), "manifest");
        assert_eq!(tool.label(), "Manifest");
    }

    #[test]
    fn resolves_catalog_entry_selection_as_tool_when_tool_exists() {
        let catalog = CatalogDomain::with_entries(
            "project",
            "Project",
            vec![CatalogTool::new("manifest", "Manifest")],
            vec![CatalogRecipe::new("project_summary", "Project Summary")],
        );

        let entry = resolve_catalog_entry_selection(&catalog, "manifest").expect("tool entry");

        assert_eq!(entry.kind(), CatalogEntryKind::Tool);
        assert_eq!(
            entry,
            CatalogEntrySelection::Tool(CatalogTool::new("manifest", "Manifest"))
        );
    }

    #[test]
    fn resolves_catalog_entry_selection_as_recipe_when_recipe_exists() {
        let catalog = CatalogDomain::with_entries(
            "project",
            "Project",
            vec![CatalogTool::new("manifest", "Manifest")],
            vec![CatalogRecipe::new("project_summary", "Project Summary")],
        );

        let entry =
            resolve_catalog_entry_selection(&catalog, "project_summary").expect("recipe entry");

        assert_eq!(entry.kind(), CatalogEntryKind::Recipe);
        assert_eq!(
            entry,
            CatalogEntrySelection::Recipe(CatalogRecipe::new("project_summary", "Project Summary"))
        );
    }

    #[test]
    fn fails_when_selected_catalog_entry_does_not_exist() {
        let catalog = CatalogDomain::with_entries(
            "project",
            "Project",
            vec![CatalogTool::new("manifest", "Manifest")],
            vec![CatalogRecipe::new("project_summary", "Project Summary")],
        );

        let error = resolve_catalog_entry_selection(&catalog, "viewer").expect_err("missing entry");

        assert_eq!(
            error,
            ToolRuntimeError::CatalogEntryNotFound {
                catalog_id: "project".to_owned(),
                entry_id: "viewer".to_owned()
            }
        );
    }

    #[test]
    fn resolves_catalog_request_for_valid_tool_selection() {
        let meta_catalog = load_global_meta_catalog().expect("load global meta catalog");
        let effective_meta_catalog =
            build_effective_meta_catalog(&meta_catalog, None).expect("effective catalog");
        let request = CatalogSelectionRequest::new("project", "manifest");

        let resolved =
            resolve_catalog_request(&effective_meta_catalog, &request).expect("resolved request");

        assert_eq!(resolved.catalog().id(), "project");
        assert_eq!(resolved.entry_kind(), CatalogEntryKind::Tool);
        assert_eq!(
            resolved,
            ResolvedCatalogEntry::new(
                resolve_catalog_selection(&effective_meta_catalog, "project")
                    .expect("project catalog"),
                CatalogEntrySelection::Tool(CatalogTool::new("manifest", "Manifest"))
            )
        );
    }

    #[test]
    fn resolves_catalog_request_for_valid_recipe_selection() {
        let meta_catalog = load_global_meta_catalog().expect("load global meta catalog");
        let effective_meta_catalog =
            build_effective_meta_catalog(&meta_catalog, None).expect("effective catalog");
        let request = CatalogSelectionRequest::new("project", "project_summary");

        let resolved =
            resolve_catalog_request(&effective_meta_catalog, &request).expect("resolved request");

        assert_eq!(resolved.catalog().id(), "project");
        assert_eq!(resolved.entry_kind(), CatalogEntryKind::Recipe);
        assert_eq!(
            resolved,
            ResolvedCatalogEntry::new(
                resolve_catalog_selection(&effective_meta_catalog, "project")
                    .expect("project catalog"),
                CatalogEntrySelection::Recipe(CatalogRecipe::new(
                    "project_summary",
                    "Project Summary"
                ))
            )
        );
    }

    #[test]
    fn fails_when_catalog_request_references_unknown_catalog() {
        let effective_meta_catalog = EffectiveMetaCatalog::new(vec![CatalogDomain::with_entries(
            "project",
            "Project",
            vec![CatalogTool::new("manifest", "Manifest")],
            vec![CatalogRecipe::new("project_summary", "Project Summary")],
        )]);
        let request = CatalogSelectionRequest::new("knowledge", "search");

        let error = resolve_catalog_request(&effective_meta_catalog, &request)
            .expect_err("missing catalog");

        assert_eq!(
            error,
            ToolRuntimeError::CatalogNotFound {
                id: "knowledge".to_owned()
            }
        );
    }

    #[test]
    fn fails_when_catalog_request_references_unknown_entry_inside_catalog() {
        let effective_meta_catalog = EffectiveMetaCatalog::new(vec![CatalogDomain::with_entries(
            "project",
            "Project",
            vec![CatalogTool::new("manifest", "Manifest")],
            vec![CatalogRecipe::new("project_summary", "Project Summary")],
        )]);
        let request = CatalogSelectionRequest::new("project", "viewer");

        let error =
            resolve_catalog_request(&effective_meta_catalog, &request).expect_err("missing entry");

        assert_eq!(
            error,
            ToolRuntimeError::CatalogEntryNotFound {
                catalog_id: "project".to_owned(),
                entry_id: "viewer".to_owned()
            }
        );
    }

    #[test]
    fn validates_resolved_tool_entry_when_declarative_shape_is_valid() {
        let catalog = CatalogDomain::with_entries(
            "project",
            "Project",
            vec![CatalogTool::new("manifest", "Manifest")],
            vec![CatalogRecipe::new("project_summary", "Project Summary")],
        );
        let resolved = ResolvedCatalogEntry::new(
            catalog,
            CatalogEntrySelection::Tool(CatalogTool::new("manifest", "Manifest")),
        );

        let result = validate_catalog_entry_resolution(&resolved);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn validates_resolved_recipe_entry_when_declarative_shape_is_valid() {
        let catalog = CatalogDomain::with_entries(
            "project",
            "Project",
            vec![CatalogTool::new("manifest", "Manifest")],
            vec![CatalogRecipe::new("project_summary", "Project Summary")],
        );
        let resolved = ResolvedCatalogEntry::new(
            catalog,
            CatalogEntrySelection::Recipe(CatalogRecipe::new("project_summary", "Project Summary")),
        );

        let result = validate_catalog_entry_resolution(&resolved);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn fails_when_resolved_entry_is_malformed() {
        let catalog = CatalogDomain::with_entries(
            "project",
            "Project",
            vec![CatalogTool::new("manifest", "Manifest")],
            vec![],
        );
        let resolved = ResolvedCatalogEntry::new(
            catalog,
            CatalogEntrySelection::Tool(CatalogTool::new("manifest", "")),
        );

        let error =
            validate_catalog_entry_resolution(&resolved).expect_err("malformed resolved entry");

        assert_eq!(
            error,
            ToolRuntimeError::InvalidCatalogEntryResolution {
                catalog_id: "project".to_owned(),
                entry_id: "manifest".to_owned(),
                reason: "entry label must not be empty",
            }
        );
    }

    #[test]
    fn fails_when_resolved_entry_is_not_declared_in_catalog() {
        let catalog = CatalogDomain::with_entries(
            "project",
            "Project",
            vec![CatalogTool::new("manifest", "Manifest")],
            vec![],
        );
        let resolved = ResolvedCatalogEntry::new(
            catalog,
            CatalogEntrySelection::Recipe(CatalogRecipe::new("project_summary", "Project Summary")),
        );

        let error =
            validate_catalog_entry_resolution(&resolved).expect_err("undeclared resolved entry");

        assert_eq!(
            error,
            ToolRuntimeError::InvalidCatalogEntryResolution {
                catalog_id: "project".to_owned(),
                entry_id: "project_summary".to_owned(),
                reason: "resolved recipe is not declared in the resolved catalog",
            }
        );
    }

    #[test]
    fn runner_accepts_valid_resolved_tool_entry() {
        let runner = ToolRuntimeRunner::new();
        let resolved = ResolvedCatalogEntry::new(
            CatalogDomain::with_entries(
                "project",
                "Project",
                vec![CatalogTool::new("manifest", "Manifest")],
                vec![CatalogRecipe::new("project_summary", "Project Summary")],
            ),
            CatalogEntrySelection::Tool(CatalogTool::new("manifest", "Manifest")),
        );

        let result = runner.dispatch(&resolved).expect("runner result");

        assert_eq!(result.catalog_id(), "project");
        assert_eq!(result.entry_id(), "manifest");
        assert_eq!(result.entry_kind(), CatalogEntryKind::Tool);
        assert_eq!(result.outcome(), ToolExecutionOutcome::Accepted);
    }

    #[test]
    fn runner_accepts_valid_resolved_recipe_entry() {
        let runner = ToolRuntimeRunner::new();
        let resolved = ResolvedCatalogEntry::new(
            CatalogDomain::with_entries(
                "project",
                "Project",
                vec![CatalogTool::new("manifest", "Manifest")],
                vec![CatalogRecipe::new("project_summary", "Project Summary")],
            ),
            CatalogEntrySelection::Recipe(CatalogRecipe::new("project_summary", "Project Summary")),
        );

        let result = runner.dispatch(&resolved).expect("runner result");

        assert_eq!(result.catalog_id(), "project");
        assert_eq!(result.entry_id(), "project_summary");
        assert_eq!(result.entry_kind(), CatalogEntryKind::Recipe);
        assert_eq!(result.outcome(), ToolExecutionOutcome::Accepted);
    }

    #[test]
    fn runner_returns_typed_result_without_executing_real_logic() {
        let runner = ToolRuntimeRunner::new();
        let resolved = ResolvedCatalogEntry::new(
            CatalogDomain::with_entries(
                "knowledge",
                "Knowledge",
                vec![CatalogTool::new("search", "Search")],
                vec![],
            ),
            CatalogEntrySelection::Tool(CatalogTool::new("search", "Search")),
        );

        let result = runner.dispatch(&resolved).expect("runner result");

        assert_eq!(
            result,
            ToolRuntimeResult::new(
                "knowledge",
                "search",
                CatalogEntryKind::Tool,
                ToolExecutionOutcome::Accepted,
            )
        );
    }

    #[test]
    fn runner_rejects_invalid_resolved_entry_state() {
        let runner = ToolRuntimeRunner::new();
        let resolved = ResolvedCatalogEntry::new(
            CatalogDomain::with_entries(
                "project",
                "Project",
                vec![CatalogTool::new("manifest", "Manifest")],
                vec![],
            ),
            CatalogEntrySelection::Recipe(CatalogRecipe::new("project_summary", "Project Summary")),
        );

        let error = runner
            .dispatch(&resolved)
            .expect_err("invalid resolved entry must fail");

        assert_eq!(
            error,
            ToolRuntimeError::InvalidCatalogEntryResolution {
                catalog_id: "project".to_owned(),
                entry_id: "project_summary".to_owned(),
                reason: "resolved recipe is not declared in the resolved catalog",
            }
        );
    }

    #[test]
    fn executes_text_measure_and_persists_result_and_manifest() {
        let workspace_dir = unique_temp_dir("tool_runtime_text_measure");
        let user_output_dir = workspace_dir.join("user").join("output");
        fs::create_dir_all(&user_output_dir).expect("create user output dir");
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let request = TextMeasureRequest::governed_text_ref(
            "text.measure",
            OwnerRef::new("chat://alpha"),
            "trace://trace-alpha",
            "text://input-alpha",
            "hello\nworld secret",
            user_output_dir.clone(),
        );

        let outcome = execute_text_measure(&workspace_root, &request).expect("text measure");

        assert!(outcome.run_id().starts_with("text_measure_"));
        assert_eq!(outcome.result().char_count, 18);
        assert_eq!(outcome.result().byte_count, 18);
        assert_eq!(outcome.result().line_count, 2);
        assert_eq!(outcome.result().word_count, 3);
        assert_eq!(outcome.result_path(), outcome.run_directory().join("result.json"));
        assert_eq!(
            outcome.manifest_path(),
            outcome.run_directory().join("tool_run_manifest.json")
        );
        let expected_run_directory = workspace_dir
            .join("user")
            .join("output")
            .join("tool_runs")
            .join("chat")
            .join("alpha")
            .join(outcome.run_id());
        let expected_run_directory =
            fs::canonicalize(expected_run_directory).expect("canonical expected run directory");
        assert_eq!(outcome.run_directory(), expected_run_directory.as_path());
        assert!(outcome.run_directory().join("result.json").exists());
        assert!(outcome.run_directory().join("tool_run_manifest.json").exists());

        let manifest = fs::read_to_string(outcome.run_directory().join("tool_run_manifest.json"))
            .expect("read manifest");
        let manifest_json: serde_json::Value =
            serde_json::from_str(&manifest).expect("parse manifest");
        assert!(manifest.contains("\"tool_id\": \"text.measure\""));
        assert_eq!(manifest_json["tool_id"], "text.measure");
        assert_eq!(manifest_json["tool_kind"], "operational");
        assert_eq!(
            manifest_json["runtime_scope"],
            "single_tool_local_deterministic_sandbox"
        );
        assert_eq!(manifest_json["outputs"]["result_path"].as_str().is_some(), true);
        assert_eq!(
            manifest_json["outputs"]["manifest_path"],
            format!(
                "user/output/tool_runs/chat/alpha/{}/tool_run_manifest.json",
                outcome.run_id()
            )
        );
        assert!(manifest.contains("\"owner_ref\": \"chat://alpha\""));
        assert!(manifest.contains("\"trace_ref\": \"trace://trace-alpha\""));
        assert!(manifest.contains("\"input_ref\": \"text://input-alpha\""));
        assert!(manifest.contains("\"status\": \"completed\""));
        assert!(manifest.contains("user/output/tool_runs/chat/alpha/"));
        assert!(!manifest.contains(&workspace_dir.display().to_string()));
        assert!(!manifest.contains("hello"));
        assert!(!manifest.contains("world"));
        assert!(!manifest.contains("secret"));

        let result = fs::read_to_string(outcome.run_directory().join("result.json"))
            .expect("read result");
        assert!(!result.contains("hello"));
        assert!(!result.contains("world"));
        assert!(!result.contains("secret"));

        let mut created_files = fs::read_dir(outcome.run_directory())
            .expect("read run directory")
            .map(|entry| {
                entry
                    .expect("run directory entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<_>>();
        created_files.sort();
        assert_eq!(
            created_files,
            vec![
                "result.json".to_owned(),
                "tool_run_manifest.json".to_owned()
            ]
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn rejects_text_measure_without_owner_ref() {
        let workspace_dir = unique_temp_dir("tool_runtime_text_measure_missing_owner");
        let user_output_dir = workspace_dir.join("user").join("output");
        fs::create_dir_all(&user_output_dir).expect("create user output dir");
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let request = TextMeasureRequest::governed_text_ref(
            "text.measure",
            OwnerRef::new(""),
            "trace://trace-alpha",
            "text://input-alpha",
            "hello",
            user_output_dir,
        );

        let error = execute_text_measure(&workspace_root, &request).expect_err("missing owner");

        assert_eq!(
            error,
            ToolRuntimeError::TextMeasureValidation {
                reason: "owner_ref is required",
            }
        );
        assert!(!workspace_dir.join("user").join("output").join("tool_runs").exists());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn rejects_text_measure_with_non_portable_owner_ref() {
        let workspace_dir = unique_temp_dir("tool_runtime_text_measure_invalid_owner");
        let user_output_dir = workspace_dir.join("user").join("output");
        fs::create_dir_all(&user_output_dir).expect("create user output dir");
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let request = TextMeasureRequest::governed_text_ref(
            "text.measure",
            OwnerRef::new("chat://alpha/beta"),
            "trace://trace-alpha",
            "text://input-alpha",
            "hello",
            user_output_dir,
        );

        let error = execute_text_measure(&workspace_root, &request).expect_err("invalid owner");

        assert_eq!(
            error,
            ToolRuntimeError::TextMeasureValidation {
                reason: "owner_ref owner id must be portable and path-safe",
            }
        );
        assert!(!workspace_dir.join("user").join("output").join("tool_runs").exists());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn rejects_text_measure_without_trace_ref() {
        let workspace_dir = unique_temp_dir("tool_runtime_text_measure_missing_trace");
        let user_output_dir = workspace_dir.join("user").join("output");
        fs::create_dir_all(&user_output_dir).expect("create user output dir");
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let request = TextMeasureRequest::governed_text_ref(
            "text.measure",
            OwnerRef::new("chat://alpha"),
            "",
            "text://input-alpha",
            "hello",
            user_output_dir,
        );

        let error = execute_text_measure(&workspace_root, &request).expect_err("missing trace");

        assert_eq!(
            error,
            ToolRuntimeError::TextMeasureValidation {
                reason: "trace_ref is required",
            }
        );
        assert!(!workspace_dir.join("user").join("output").join("tool_runs").exists());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn rejects_non_text_measure_tool_id() {
        let workspace_dir = unique_temp_dir("tool_runtime_text_measure_bad_tool");
        let user_output_dir = workspace_dir.join("user").join("output");
        fs::create_dir_all(&user_output_dir).expect("create user output dir");
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let request = TextMeasureRequest::governed_text_ref(
            "merge_pdfs",
            OwnerRef::new("chat://alpha"),
            "trace://trace-alpha",
            "text://input-alpha",
            "hello",
            user_output_dir,
        );

        let error = execute_text_measure(&workspace_root, &request).expect_err("wrong tool");

        assert_eq!(
            error,
            ToolRuntimeError::TextMeasureValidation {
                reason: "tool_id must be text.measure",
            }
        );
        assert!(!workspace_dir.join("user").join("output").join("tool_runs").exists());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn rejects_text_measure_output_root_escape() {
        let workspace_dir = unique_temp_dir("tool_runtime_text_measure_escape");
        let user_output_dir = workspace_dir.join("user").join("output");
        fs::create_dir_all(&user_output_dir).expect("create user output dir");
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let request = TextMeasureRequest::governed_text_ref(
            "text.measure",
            OwnerRef::new("chat://alpha"),
            "trace://trace-alpha",
            "text://input-alpha",
            "hello",
            workspace_dir.join("outputs"),
        );

        let error = execute_text_measure(&workspace_root, &request).expect_err("escaped output");

        assert_eq!(
            error,
            ToolRuntimeError::TextMeasureValidation {
                reason: "output_root must stay under user/output",
            }
        );
        assert!(!workspace_dir.join("outputs").exists());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn rejects_text_measure_output_root_parent_traversal() {
        let workspace_dir = unique_temp_dir("tool_runtime_text_measure_traversal");
        let user_output_dir = workspace_dir.join("user").join("output");
        fs::create_dir_all(&user_output_dir).expect("create user output dir");
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let request = TextMeasureRequest::governed_text_ref(
            "text.measure",
            OwnerRef::new("chat://alpha"),
            "trace://trace-alpha",
            "text://input-alpha",
            "hello",
            user_output_dir.join("..").join("escaped"),
        );

        let error = execute_text_measure(&workspace_root, &request).expect_err("traversal output");

        assert_eq!(
            error,
            ToolRuntimeError::TextMeasureValidation {
                reason: "output_root must not contain parent-directory traversal",
            }
        );
        assert!(!workspace_dir.join("user").join("escaped").exists());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    fn create_project_root(workspace_dir: &PathBuf) -> ProjectRoot {
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        fs::create_dir_all(&project_dir).expect("create project dir");
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        ProjectRoot::new(&workspace_root, project_dir).expect("project root")
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after unix epoch")
            .as_nanos();
        path.push(format!("rust_portable_app_{prefix}_{nanos}"));
        path
    }
}
