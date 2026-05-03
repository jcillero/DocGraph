use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const CRATES_DIR: &str = "crates";
const SCRIPTS_DIR: &str = "dev/scripts";
const SNAPSHOT_PATH: &str = "user/output/rust_status_snapshot.md";
const AI_SPECS_REPORT_PATH: &str = "user/output/validate_ai_specs_report.json";

const EXPECTED_CRATES: &[&str] = &[
    "app_services",
    "cli_app",
    "core_domain",
    "io_runtime",
    "llm_cloud",
    "llm_core",
    "llm_local",
    "project_runtime",
    "spec_runtime",
    "tool_runtime",
    "ui_core",
    "ui_i18n",
    "ui_slint",
    "ui_theme",
    "workspace_core",
];

const CRITICAL_SCRIPTS: &[&str] = &["cargo_check.bat", "cargo_test.bat", "cargo_all.bat"];
const ADVISORY_SCRIPTS: &[&str] = &[
    "cargo_clippy.bat",
    "cargo_fmt.bat",
    "cargo_strict.bat",
    "generate_status_snapshot.bat",
    "SCRIPTS_INDEX.md",
];

const EXPECTED_DOCS: &[&str] = &[
    "README.md",
    "GOVERNANCE.md",
    "ARCHITECTURE.md",
    "MIGRATION_BASELINE.md",
    "WORKSPACE_RULES.md",
    "docs/ENGINEERING_NOTES.md",
];

const APP_SERVICES_FILE: &str = "crates/app_services/src/lib.rs";
const PROJECT_RUNTIME_FILE: &str = "crates/project_runtime/src/lib.rs";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OverallStatus {
    Ok,
    OkWithWarnings,
    Fail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VerificationMode {
    Quick,
    Full,
}

impl VerificationMode {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "quick" => Some(Self::Quick),
            "full" => Some(Self::Full),
            _ => None,
        }
    }

    fn required_scripts(self) -> &'static [&'static str] {
        match self {
            Self::Quick => &["cargo_check.bat", "cargo_test.bat"],
            Self::Full => &[
                "cargo_fmt.bat",
                "cargo_clippy.bat",
                "cargo_check.bat",
                "cargo_test.bat",
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FindingSeverity {
    Warning,
    Fail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StepStatus {
    Pass,
    Fail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationStepReport {
    pub script: String,
    pub status: StepStatus,
    pub exit_code: i32,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MechanicalValidationReport {
    pub status: OverallStatus,
    pub mode: VerificationMode,
    pub steps: Vec<ValidationStepReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceStructureReport {
    pub status: OverallStatus,
    pub present_crates: Vec<String>,
    pub missing_critical_crates: Vec<String>,
    pub present_scripts: Vec<String>,
    pub missing_critical_scripts: Vec<String>,
    pub missing_advisory_scripts: Vec<String>,
    pub extra_crates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DocumentationChecksReport {
    pub status: OverallStatus,
    pub present_docs: Vec<String>,
    pub missing_docs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArchitectureWatchpoint {
    pub id: String,
    pub severity: FindingSeverity,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProgressVerificationReport {
    pub overall_status: OverallStatus,
    pub mode: VerificationMode,
    pub workspace_root: String,
    pub timestamp: String,
    pub mechanical_validation: MechanicalValidationReport,
    pub workspace_structure: WorkspaceStructureReport,
    pub architecture_watchpoints: Vec<ArchitectureWatchpoint>,
    pub documentation_checks: DocumentationChecksReport,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AiSpecsValidationReport {
    pub status: AiSpecsValidationStatus,
    pub json_checked_count: usize,
    pub prompt_specs_checked_count: usize,
    pub semantic_specs_checked_count: usize,
    pub policies_checked_count: usize,
    pub schemas_checked_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub errors: Vec<AiSpecsValidationFinding>,
    pub warnings: Vec<AiSpecsValidationFinding>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiSpecsValidationStatus {
    Ok,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AiSpecsValidationFinding {
    pub code: String,
    pub file: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceInventory {
    crates: BTreeSet<String>,
    scripts: BTreeSet<String>,
    files: BTreeSet<String>,
}

impl WorkspaceInventory {
    pub fn new(
        crates: BTreeSet<String>,
        scripts: BTreeSet<String>,
        files: BTreeSet<String>,
    ) -> Self {
        Self {
            crates,
            scripts,
            files,
        }
    }

    pub fn collect(workspace_root: &Path) -> std::io::Result<Self> {
        let crates = collect_directory_names(&workspace_root.join(CRATES_DIR))?;
        let scripts = collect_directory_names(&workspace_root.join(SCRIPTS_DIR))?;
        let files = EXPECTED_DOCS
            .iter()
            .chain(std::iter::once(&SNAPSHOT_PATH))
            .filter_map(|relative| {
                let path = workspace_root.join(relative);
                path.exists().then(|| relative.replace('\\', "/"))
            })
            .collect::<BTreeSet<_>>();

        Ok(Self::new(crates, scripts, files))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptRunOutcome {
    pub exit_code: i32,
    pub output: String,
}

impl ScriptRunOutcome {
    fn success(&self) -> bool {
        self.exit_code == 0
    }
}

pub trait ScriptRunner {
    fn run_script(&self, workspace_root: &Path, script_name: &str) -> ScriptRunOutcome;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct WrapperScriptRunner;

impl ScriptRunner for WrapperScriptRunner {
    fn run_script(&self, workspace_root: &Path, script_name: &str) -> ScriptRunOutcome {
        let script_path = workspace_root.join(SCRIPTS_DIR).join(script_name);
        if !script_path.exists() {
            return ScriptRunOutcome {
                exit_code: 1,
                output: format!("wrapper script not found: {}", script_path.display()),
            };
        }

        let output = Command::new("cmd")
            .args(["/C", &script_path.to_string_lossy()])
            .current_dir(workspace_root)
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let summary = summarize_command_output(&stdout, &stderr);

                ScriptRunOutcome {
                    exit_code: output.status.code().unwrap_or(1),
                    output: summary,
                }
            }
            Err(error) => ScriptRunOutcome {
                exit_code: 1,
                output: format!("failed to execute wrapper: {error}"),
            },
        }
    }
}

pub fn verify_progress(
    workspace_root: &Path,
    mode: VerificationMode,
    runner: &dyn ScriptRunner,
) -> Result<ProgressVerificationReport, String> {
    let workspace_root = fs::canonicalize(workspace_root).map_err(|error| {
        format!(
            "failed to canonicalize workspace_root '{}': {}",
            workspace_root.display(),
            error
        )
    })?;

    let mechanical_validation = run_mechanical_validation(&workspace_root, mode, runner);
    let inventory = WorkspaceInventory::collect(&workspace_root).map_err(|error| {
        format!(
            "failed to inspect workspace structure at '{}': {}",
            workspace_root.display(),
            error
        )
    })?;
    let workspace_structure = evaluate_workspace_structure(&inventory);
    let documentation_checks = evaluate_documentation_checks(&inventory);
    let snapshot_contents = read_optional_text(&workspace_root.join(SNAPSHOT_PATH));
    let architecture_watchpoints = evaluate_architecture_watchpoints(
        &workspace_root,
        &inventory,
        snapshot_contents.as_deref(),
    );
    let notes = build_notes(mode, &workspace_structure, &documentation_checks);
    let overall_status = derive_overall_status(
        &mechanical_validation,
        &workspace_structure,
        &documentation_checks,
        &architecture_watchpoints,
    );

    Ok(ProgressVerificationReport {
        overall_status,
        mode,
        workspace_root: workspace_root.display().to_string(),
        timestamp: current_timestamp_text(),
        mechanical_validation,
        workspace_structure,
        architecture_watchpoints,
        documentation_checks,
        notes,
    })
}

pub fn report_to_pretty_json(report: &ProgressVerificationReport) -> Result<String, String> {
    serde_json::to_string_pretty(report).map_err(|error| error.to_string())
}

pub fn ai_specs_report_to_pretty_json(
    report: &AiSpecsValidationReport,
) -> Result<String, String> {
    serde_json::to_string_pretty(report).map_err(|error| error.to_string())
}

pub fn report_to_markdown(report: &ProgressVerificationReport) -> String {
    let mut lines = vec![
        "# Progress Verification".to_owned(),
        String::new(),
        format!("- overall_status: `{:?}`", report.overall_status),
        format!("- mode: `{:?}`", report.mode),
        format!("- workspace_root: `{}`", report.workspace_root),
        String::new(),
        "## Mechanical validation".to_owned(),
        String::new(),
    ];

    for step in &report.mechanical_validation.steps {
        lines.push(format!(
            "- `{}`: `{:?}` ({})",
            step.script, step.status, step.detail
        ));
    }

    lines.push(String::new());
    lines.push("## Structure".to_owned());
    lines.push(String::new());
    lines.push(format!(
        "- missing critical crates: {}",
        report.workspace_structure.missing_critical_crates.len()
    ));
    lines.push(format!(
        "- missing critical scripts: {}",
        report.workspace_structure.missing_critical_scripts.len()
    ));
    lines.push(format!(
        "- missing docs: {}",
        report.documentation_checks.missing_docs.len()
    ));
    lines.push(String::new());
    lines.push("## Watchpoints".to_owned());
    lines.push(String::new());

    if report.architecture_watchpoints.is_empty() {
        lines.push("- none".to_owned());
    } else {
        for watchpoint in &report.architecture_watchpoints {
            lines.push(format!(
                "- `{:?}` {}: {}",
                watchpoint.severity, watchpoint.id, watchpoint.message
            ));
        }
    }

    lines.join("\n")
}

pub fn write_text_output(path: &Path, contents: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "failed to create output directory '{}': {}",
                parent.display(),
                error
            )
        })?;
    }

    fs::write(path, contents)
        .map_err(|error| format!("failed to write '{}': {}", path.display(), error))
}

pub fn validate_ai_specs(workspace_root: &Path) -> Result<AiSpecsValidationReport, String> {
    let workspace_root = fs::canonicalize(workspace_root).map_err(|error| {
        format!(
            "failed to canonicalize workspace_root '{}': {}",
            workspace_root.display(),
            error
        )
    })?;
    let mut validator = AiSpecsValidator::new(workspace_root);
    validator.validate();
    Ok(validator.into_report())
}

pub fn write_ai_specs_report(
    workspace_root: &Path,
    report: &AiSpecsValidationReport,
) -> Result<(), String> {
    let json = ai_specs_report_to_pretty_json(report)?;
    write_text_output(&workspace_root.join(AI_SPECS_REPORT_PATH), &json)
}

struct AiSpecsValidator {
    workspace_root: std::path::PathBuf,
    errors: Vec<AiSpecsValidationFinding>,
    warnings: Vec<AiSpecsValidationFinding>,
    json_checked_count: usize,
    prompt_specs_checked_count: usize,
    semantic_specs_checked_count: usize,
    policies_checked_count: usize,
    schemas_checked_count: usize,
    prompt_specs: BTreeMap<String, (std::path::PathBuf, serde_json::Value)>,
}

impl AiSpecsValidator {
    fn new(workspace_root: std::path::PathBuf) -> Self {
        Self {
            workspace_root,
            errors: Vec::new(),
            warnings: Vec::new(),
            json_checked_count: 0,
            prompt_specs_checked_count: 0,
            semantic_specs_checked_count: 0,
            policies_checked_count: 0,
            schemas_checked_count: 0,
            prompt_specs: BTreeMap::new(),
        }
    }

    fn validate(&mut self) {
        let parsed = self.parse_ai_json_files();

        for (path, value) in &parsed {
            if self.relative(path).starts_with("resources/ai/prompt_specs/") {
                self.validate_prompt_spec(path, value);
            }
        }

        let mut semantic_ids = BTreeSet::new();
        for (path, value) in &parsed {
            let relative = self.relative(path);
            if relative.starts_with("resources/ai/semantic_specs/") {
                if let Some(id) = string_field(value, "semantic_spec_id") {
                    if !semantic_ids.insert(id.to_owned()) {
                        self.error(
                            "DUPLICATE_SEMANTIC_SPEC_ID",
                            path,
                            format!("semantic_spec_id duplicates {id}"),
                        );
                    }
                }
                self.validate_semantic_spec(path, value);
            } else if relative.starts_with("resources/ai/policies/") {
                self.validate_policy(path, value);
            } else if relative.starts_with("resources/ai/schemas/") {
                self.validate_schema(path, value);
                if path.file_name().and_then(|name| name.to_str())
                    == Some("ai_trace_record.schema.json")
                {
                    self.validate_trace_schema(path, value);
                }
                if path.file_name().and_then(|name| name.to_str())
                    == Some("semantic_proposal.schema.json")
                {
                    self.validate_semantic_proposal_schema(path, value);
                }
            }
        }
    }

    fn into_report(self) -> AiSpecsValidationReport {
        let error_count = self.errors.len();
        let warning_count = self.warnings.len();
        AiSpecsValidationReport {
            status: if error_count == 0 {
                AiSpecsValidationStatus::Ok
            } else {
                AiSpecsValidationStatus::Error
            },
            json_checked_count: self.json_checked_count,
            prompt_specs_checked_count: self.prompt_specs_checked_count,
            semantic_specs_checked_count: self.semantic_specs_checked_count,
            policies_checked_count: self.policies_checked_count,
            schemas_checked_count: self.schemas_checked_count,
            error_count,
            warning_count,
            errors: self.errors,
            warnings: self.warnings,
        }
    }

    fn parse_ai_json_files(&mut self) -> Vec<(std::path::PathBuf, serde_json::Value)> {
        let ai_root = self.workspace_root.join("resources").join("ai");
        if !ai_root.exists() {
            self.error(
                "MISSING_AI_ROOT",
                &self.workspace_root.join("resources"),
                "resources/ai does not exist",
            );
            return Vec::new();
        }

        let mut json_files = Vec::new();
        collect_json_files(&ai_root, &mut json_files);
        json_files.sort();

        let mut parsed = Vec::new();
        for path in json_files {
            match fs::read_to_string(&path) {
                Ok(contents) => match serde_json::from_str::<serde_json::Value>(&contents) {
                    Ok(value) if value.is_object() => {
                        self.json_checked_count += 1;
                        self.validate_common_declared_only(&path, &value);
                        parsed.push((path, value));
                    }
                    Ok(_) => self.error("INVALID_JSON_ROOT", &path, "JSON root must be an object"),
                    Err(error) => self.error("INVALID_JSON", &path, format!("invalid JSON: {error}")),
                },
                Err(error) => self.error("READ_ERROR", &path, format!("cannot read file: {error}")),
            }
        }
        parsed
    }

    fn validate_common_declared_only(&mut self, path: &Path, value: &serde_json::Value) {
        self.require(value.get("schema_version").is_some(), "MISSING_SCHEMA_VERSION", path, "schema_version is required");
        self.require(value.get("version").is_some(), "MISSING_VERSION", path, "version is required");
        if let Some(status) = value.get("status") {
            self.require(status == "declared_only", "INVALID_STATUS", path, "status must be declared_only");
        }
        if let Some(execution_enabled) = value.get("execution_enabled") {
            self.require(execution_enabled == false, "EXECUTION_ENABLED", path, "execution_enabled must be false");
        }
    }

    fn validate_prompt_spec(&mut self, path: &Path, value: &serde_json::Value) {
        self.prompt_specs_checked_count += 1;
        let prompt_id = string_field(value, "prompt_id").unwrap_or_default();
        self.require(!prompt_id.trim().is_empty(), "EMPTY_PROMPT_ID", path, "prompt_id must be non-empty");
        if !prompt_id.trim().is_empty() {
            if let Some((existing_path, _)) = self.prompt_specs.get(prompt_id) {
                self.error(
                    "DUPLICATE_PROMPT_ID",
                    path,
                    format!("prompt_id duplicates {}", self.relative(existing_path)),
                );
            }
            self.prompt_specs
                .insert(prompt_id.to_owned(), (path.to_path_buf(), value.clone()));
        }

        let template = value.get("prompt_template").and_then(serde_json::Value::as_object);
        self.require(template.is_some(), "MISSING_PROMPT_TEMPLATE", path, "prompt_template object is required");
        if let Some(template) = template {
            self.require(template.get("type") == Some(&serde_json::Value::String("structured".to_owned())), "INVALID_PROMPT_TEMPLATE_TYPE", path, "prompt_template.type must be structured");
            self.require(template.get("template").and_then(serde_json::Value::as_str).is_some(), "MISSING_PROMPT_TEMPLATE_TEXT", path, "prompt_template.template string is required");
        }

        self.require(nested_string(value, &["input_contract", "type"]).is_some(), "INVALID_INPUT_CONTRACT", path, "input_contract.type is required");
        self.require(nested_string(value, &["output_contract", "type"]).is_some(), "INVALID_OUTPUT_CONTRACT", path, "output_contract.type is required");
        self.require(nested_bool(value, &["constraints", "structured_output"]) == Some(true), "STRUCTURED_OUTPUT_REQUIRED", path, "constraints.structured_output must be true");
        self.require(nested_bool(value, &["constraints", "free_text_allowed"]) == Some(false), "FREE_TEXT_FORBIDDEN", path, "constraints.free_text_allowed must be false");
        self.require(
            contains_all_string_array(value, "forbidden", &["execute_llm", "call_tools", "mutate_documents", "write_files"]),
            "MISSING_PROMPT_FORBIDDEN",
            path,
            "forbidden must contain execute_llm, call_tools, mutate_documents, write_files",
        );
    }

    fn validate_semantic_spec(&mut self, path: &Path, value: &serde_json::Value) {
        self.semantic_specs_checked_count += 1;
        let semantic_id = string_field(value, "semantic_spec_id").unwrap_or_default();
        self.require(!semantic_id.trim().is_empty(), "EMPTY_SEMANTIC_SPEC_ID", path, "semantic_spec_id must be non-empty");
        self.require(string_field(value, "source_layer") == Some("document_text_runtime"), "INVALID_SOURCE_LAYER", path, "source_layer must be document_text_runtime");
        self.require(string_field(value, "source_kind") == Some("document_text_chunks"), "INVALID_SOURCE_KIND", path, "source_kind must be document_text_chunks");
        self.require(matches!(nested_string(value, &["granularity", "unit"]), Some("per_chunk" | "batch")), "INVALID_GRANULARITY_UNIT", path, "granularity.unit must be per_chunk or batch");
        self.require(nested_bool(value, &["granularity", "independent"]).is_some(), "INVALID_GRANULARITY_INDEPENDENT", path, "granularity.independent must be boolean");

        let prompt_ref = string_field(value, "prompt_ref").unwrap_or_default();
        self.require(!prompt_ref.trim().is_empty(), "EMPTY_PROMPT_REF", path, "prompt_ref must be non-empty");
        if let Some((_, prompt)) = self.prompt_specs.get(prompt_ref) {
            self.require(
                nested_string(value, &["expected_output", "type"])
                    == nested_string(prompt, &["output_contract", "type"]),
                "OUTPUT_CONTRACT_MISMATCH",
                path,
                "expected_output.type must match referenced PromptSpec output_contract.type",
            );
        } else {
            self.error("UNKNOWN_PROMPT_REF", path, format!("prompt_ref does not match a PromptSpec: {prompt_ref}"));
        }

        self.require(nested_bool(value, &["human_review", "required"]) == Some(true), "HUMAN_REVIEW_REQUIRED", path, "human_review.required must be true");
        self.require(
            nested_contains_all_string_array(value, &["human_review", "allowed_states"], &["Generated", "NeedsReview", "Approved", "Rejected", "Stale", "Expired"]),
            "MISSING_REVIEW_STATES",
            path,
            "allowed_states must contain Generated, NeedsReview, Approved, Rejected, Stale, Expired",
        );
        self.require(
            contains_all_string_array(value, "forbidden", &["duplicate_project_pipeline", "invoke_llm", "persist_semantic_graph"]),
            "MISSING_SEMANTIC_FORBIDDEN",
            path,
            "forbidden must contain duplicate_project_pipeline, invoke_llm, persist_semantic_graph",
        );
    }

    fn validate_policy(&mut self, path: &Path, value: &serde_json::Value) {
        self.policies_checked_count += 1;
        let policy_id = string_field(value, "policy_id").unwrap_or_default();
        self.require(!policy_id.trim().is_empty(), "EMPTY_POLICY_ID", path, "policy_id must be non-empty");

        let levels = string_array_field(value, "levels");
        self.require(["public", "internal", "confidential", "restricted"].iter().all(|level| levels.contains(level)), "MISSING_SENSITIVITY_LEVELS", path, "levels must contain public, internal, confidential, restricted");
        self.require(string_field(value, "default_level").is_some_and(|level| levels.contains(&level)), "INVALID_DEFAULT_LEVEL", path, "default_level must belong to levels");

        for level in ["public", "internal", "confidential", "restricted"] {
            self.require(policy_rules_for_level(value, level).len() == 1, "INVALID_RULE_CARDINALITY", path, format!("level {level} must have exactly one rule"));
        }

        if let Some(rule) = policy_rules_for_level(value, "confidential").first() {
            self.require(rule.get("llm_allowed").and_then(serde_json::Value::as_bool) == Some(false), "CONFIDENTIAL_LLM_ALLOWED", path, "confidential.llm_allowed must be false");
        }
        if let Some(rule) = policy_rules_for_level(value, "restricted").first() {
            self.require(rule.get("llm_allowed").and_then(serde_json::Value::as_bool) == Some(false), "RESTRICTED_LLM_ALLOWED", path, "restricted.llm_allowed must be false");
            self.require(rule.get("allowed_modes").and_then(serde_json::Value::as_array).is_some_and(Vec::is_empty), "RESTRICTED_ALLOWED_MODES", path, "restricted.allowed_modes must be []");
        }
        self.require(
            contains_all_string_array(value, "forbidden", &["bypass_human_review", "cloud_execution_for_confidential", "cloud_execution_for_restricted"]),
            "MISSING_POLICY_FORBIDDEN",
            path,
            "forbidden must contain bypass_human_review, cloud_execution_for_confidential, cloud_execution_for_restricted",
        );
    }

    fn validate_schema(&mut self, path: &Path, value: &serde_json::Value) {
        self.schemas_checked_count += 1;
        self.require(string_field(value, "type") == Some("object"), "INVALID_SCHEMA_ROOT", path, "schema root type must be object");
    }

    fn validate_trace_schema(&mut self, path: &Path, value: &serde_json::Value) {
        let properties = value.get("properties").and_then(serde_json::Value::as_object);
        self.require(properties.is_some(), "TRACE_SCHEMA_PROPERTIES", path, "AiTraceRecord schema must define properties");
        let Some(properties) = properties else {
            return;
        };
        for key in ["trace_id", "prompt_ref", "semantic_spec_ref", "input_refs", "output_ref", "execution"] {
            self.require(properties.contains_key(key), "TRACE_SCHEMA_MISSING_FIELD", path, format!("AiTraceRecord schema must define {key}"));
        }
        self.require(properties.get("input_refs").and_then(|v| v.get("type")).and_then(serde_json::Value::as_str) == Some("array"), "TRACE_SCHEMA_INPUT_REFS", path, "input_refs must be modeled as array");
        let output_types = properties.get("output_ref").and_then(|v| v.get("type")).and_then(serde_json::Value::as_array);
        self.require(output_types.is_some_and(|items| {
            let values = items.iter().filter_map(serde_json::Value::as_str).collect::<BTreeSet<_>>();
            values.contains("object") && values.contains("null")
        }), "TRACE_SCHEMA_OUTPUT_REF", path, "output_ref must be modeled as object or null");
        self.require(nested_bool(value, &["properties", "execution", "properties", "execution_enabled", "const"]) == Some(false), "TRACE_SCHEMA_EXECUTION_ENABLED", path, "execution.execution_enabled must be const false");
        self.require(nested_bool(value, &["properties", "execution", "properties", "executed", "const"]) == Some(false), "TRACE_SCHEMA_EXECUTED", path, "execution.executed must be const false");
    }

    fn validate_semantic_proposal_schema(&mut self, path: &Path, value: &serde_json::Value) {
        let properties = value.get("properties").and_then(serde_json::Value::as_object);
        self.require(properties.is_some(), "SEMANTIC_PROPOSAL_SCHEMA_PROPERTIES", path, "SemanticProposal schema must define properties");
        let Some(properties) = properties else {
            return;
        };
        for key in [
            "proposal_id",
            "subject_ref",
            "predicate",
            "object_ref",
            "graph_hint",
            "source_ref",
            "prompt_ref",
            "semantic_spec_ref",
            "trace_ref",
            "confidence_label",
            "review_state",
            "created_by",
            "execution_enabled",
        ] {
            self.require(properties.contains_key(key), "SEMANTIC_PROPOSAL_SCHEMA_MISSING_FIELD", path, format!("SemanticProposal schema must define {key}"));
        }
        self.require(nested_string(value, &["properties", "confidence_label", "const"]) == Some("mock_only"), "SEMANTIC_PROPOSAL_CONFIDENCE", path, "confidence_label must be const mock_only");
        self.require(nested_bool(value, &["properties", "execution_enabled", "const"]) == Some(false), "SEMANTIC_PROPOSAL_EXECUTION", path, "execution_enabled must be const false");
        self.require(nested_bool(value, &["properties", "future_graph", "properties", "oxigraph_enabled", "const"]) == Some(false), "SEMANTIC_PROPOSAL_OXIGRAPH", path, "future_graph.oxigraph_enabled must be const false");
        self.require(nested_bool(value, &["properties", "future_graph", "properties", "rdf_export_enabled", "const"]) == Some(false), "SEMANTIC_PROPOSAL_RDF_EXPORT", path, "future_graph.rdf_export_enabled must be const false");
        self.require(nested_bool(value, &["properties", "future_graph", "properties", "semantic_store_enabled", "const"]) == Some(false), "SEMANTIC_PROPOSAL_STORE", path, "future_graph.semantic_store_enabled must be const false");
        self.require(nested_contains_all_string_array(value, &["properties", "review_state", "enum"], &["Generated", "NeedsReview", "Approved", "Rejected", "Stale", "Expired"]), "SEMANTIC_PROPOSAL_REVIEW_STATES", path, "review_state enum must contain the governed lifecycle states");
        self.require(nested_contains_all_string_array(value, &["properties", "graph_hint", "enum"], &["domain_knowledge", "system_governance", "ai_governance", "document_governance", "lume_policy"]), "SEMANTIC_PROPOSAL_GRAPH_HINTS", path, "graph_hint enum must contain all future named graph hints");
    }

    fn require(&mut self, condition: bool, code: &str, path: &Path, message: impl Into<String>) {
        if !condition {
            self.error(code, path, message);
        }
    }

    fn error(&mut self, code: &str, path: &Path, message: impl Into<String>) {
        self.errors.push(AiSpecsValidationFinding {
            code: code.to_owned(),
            file: self.relative(path),
            message: message.into(),
        });
    }

    fn relative(&self, path: &Path) -> String {
        path.strip_prefix(&self.workspace_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }
}

fn run_mechanical_validation(
    workspace_root: &Path,
    mode: VerificationMode,
    runner: &dyn ScriptRunner,
) -> MechanicalValidationReport {
    let steps = mode
        .required_scripts()
        .iter()
        .map(|script| {
            let outcome = runner.run_script(workspace_root, script);
            ValidationStepReport {
                script: (*script).to_owned(),
                status: if outcome.success() {
                    StepStatus::Pass
                } else {
                    StepStatus::Fail
                },
                exit_code: outcome.exit_code,
                detail: outcome.output,
            }
        })
        .collect::<Vec<_>>();

    let status = if steps.iter().any(|step| step.status == StepStatus::Fail) {
        OverallStatus::Fail
    } else {
        OverallStatus::Ok
    };

    MechanicalValidationReport {
        status,
        mode,
        steps,
    }
}

fn evaluate_workspace_structure(inventory: &WorkspaceInventory) -> WorkspaceStructureReport {
    let present_crates = EXPECTED_CRATES
        .iter()
        .filter(|name| inventory.crates.contains(**name))
        .map(|name| (*name).to_owned())
        .collect::<Vec<_>>();
    let missing_critical_crates = EXPECTED_CRATES
        .iter()
        .filter(|name| !inventory.crates.contains(**name))
        .map(|name| (*name).to_owned())
        .collect::<Vec<_>>();

    let present_scripts = CRITICAL_SCRIPTS
        .iter()
        .chain(ADVISORY_SCRIPTS.iter())
        .filter(|name| inventory.scripts.contains(**name))
        .map(|name| (*name).to_owned())
        .collect::<Vec<_>>();
    let missing_critical_scripts = CRITICAL_SCRIPTS
        .iter()
        .filter(|name| !inventory.scripts.contains(**name))
        .map(|name| (*name).to_owned())
        .collect::<Vec<_>>();
    let missing_advisory_scripts = ADVISORY_SCRIPTS
        .iter()
        .filter(|name| !inventory.scripts.contains(**name))
        .map(|name| (*name).to_owned())
        .collect::<Vec<_>>();

    let extra_crates = inventory
        .crates
        .iter()
        .filter(|name| {
            !EXPECTED_CRATES.contains(&name.as_str()) && name.as_str() != "verify_progress"
        })
        .cloned()
        .collect::<Vec<_>>();

    let status = if !missing_critical_crates.is_empty() || !missing_critical_scripts.is_empty() {
        OverallStatus::Fail
    } else if !missing_advisory_scripts.is_empty() {
        OverallStatus::OkWithWarnings
    } else {
        OverallStatus::Ok
    };

    WorkspaceStructureReport {
        status,
        present_crates,
        missing_critical_crates,
        present_scripts,
        missing_critical_scripts,
        missing_advisory_scripts,
        extra_crates,
    }
}

fn evaluate_documentation_checks(inventory: &WorkspaceInventory) -> DocumentationChecksReport {
    let present_docs = EXPECTED_DOCS
        .iter()
        .filter(|name| inventory.files.contains(**name))
        .map(|name| (*name).to_owned())
        .collect::<Vec<_>>();
    let missing_docs = EXPECTED_DOCS
        .iter()
        .filter(|name| !inventory.files.contains(**name))
        .map(|name| (*name).to_owned())
        .collect::<Vec<_>>();

    let status = if missing_docs.is_empty() {
        OverallStatus::Ok
    } else {
        OverallStatus::OkWithWarnings
    };

    DocumentationChecksReport {
        status,
        present_docs,
        missing_docs,
    }
}

fn evaluate_architecture_watchpoints(
    workspace_root: &Path,
    inventory: &WorkspaceInventory,
    snapshot_contents: Option<&str>,
) -> Vec<ArchitectureWatchpoint> {
    let mut watchpoints = Vec::new();

    if !inventory.crates.contains("project_runtime") {
        watchpoints.push(ArchitectureWatchpoint {
            id: "missing_project_runtime".to_owned(),
            severity: FindingSeverity::Fail,
            message: "project_runtime is missing from the workspace".to_owned(),
        });
    }

    if !inventory.crates.contains("app_services") {
        watchpoints.push(ArchitectureWatchpoint {
            id: "missing_app_services".to_owned(),
            severity: FindingSeverity::Fail,
            message: "app_services is missing from the workspace".to_owned(),
        });
    }

    let suspicious_extra_crates = inventory
        .crates
        .iter()
        .filter(|name| {
            !EXPECTED_CRATES.contains(&name.as_str()) && name.as_str() != "verify_progress"
        })
        .filter(|name| is_suspicious_project_crate(name))
        .cloned()
        .collect::<Vec<_>>();
    if !suspicious_extra_crates.is_empty() {
        watchpoints.push(ArchitectureWatchpoint {
            id: "suspicious_extra_project_crates".to_owned(),
            severity: FindingSeverity::Warning,
            message: format!(
                "extra crates may suggest parallel project orchestration: {}",
                suspicious_extra_crates.join(", ")
            ),
        });
    }

    if let Ok(metrics) = source_metrics(&workspace_root.join(APP_SERVICES_FILE)) {
        if metrics.non_test_non_empty_lines > 80 || metrics.public_function_count > 3 {
            watchpoints.push(ArchitectureWatchpoint {
                id: "app_services_density".to_owned(),
                severity: FindingSeverity::Warning,
                message: format!(
                    "app_services looks thicker than expected ({} lines, {} public functions)",
                    metrics.non_test_non_empty_lines, metrics.public_function_count
                ),
            });
        }
    }

    if let Ok(metrics) = source_metrics(&workspace_root.join(PROJECT_RUNTIME_FILE)) {
        if metrics.non_test_non_empty_lines > 1200 {
            watchpoints.push(ArchitectureWatchpoint {
                id: "project_runtime_density".to_owned(),
                severity: FindingSeverity::Warning,
                message: format!(
                    "project_runtime remains dense ({} non-test non-empty lines)",
                    metrics.non_test_non_empty_lines
                ),
            });
        }
    }

    match snapshot_contents {
        Some(snapshot) => {
            if !snapshot.contains("F4A") || !snapshot.contains("app_services") {
                watchpoints.push(ArchitectureWatchpoint {
                    id: "snapshot_basic_mismatch".to_owned(),
                    severity: FindingSeverity::Warning,
                    message: "snapshot does not reflect the current F4A thin app_services front"
                        .to_owned(),
                });
            }
        }
        None => watchpoints.push(ArchitectureWatchpoint {
            id: "missing_snapshot".to_owned(),
            severity: FindingSeverity::Warning,
            message: format!(
                "snapshot not found at {}",
                workspace_root.join(SNAPSHOT_PATH).display()
            ),
        }),
    }

    watchpoints
}

fn derive_overall_status(
    mechanical_validation: &MechanicalValidationReport,
    workspace_structure: &WorkspaceStructureReport,
    documentation_checks: &DocumentationChecksReport,
    watchpoints: &[ArchitectureWatchpoint],
) -> OverallStatus {
    if mechanical_validation.status == OverallStatus::Fail
        || workspace_structure.status == OverallStatus::Fail
        || watchpoints
            .iter()
            .any(|watchpoint| watchpoint.severity == FindingSeverity::Fail)
    {
        OverallStatus::Fail
    } else if workspace_structure.status == OverallStatus::OkWithWarnings
        || documentation_checks.status == OverallStatus::OkWithWarnings
        || watchpoints
            .iter()
            .any(|watchpoint| watchpoint.severity == FindingSeverity::Warning)
    {
        OverallStatus::OkWithWarnings
    } else {
        OverallStatus::Ok
    }
}

fn current_timestamp_text() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_millis();
    format!("unix_ms:{millis}")
}

fn build_notes(
    mode: VerificationMode,
    workspace_structure: &WorkspaceStructureReport,
    documentation_checks: &DocumentationChecksReport,
) -> Vec<String> {
    let mut notes = vec![format!(
        "verify_progress v1 runs Rust mechanical validation through sandbox wrappers in {:?} mode",
        mode
    )];

    if !workspace_structure.extra_crates.is_empty() {
        notes.push(format!(
            "non-expected crates were observed: {}",
            workspace_structure.extra_crates.join(", ")
        ));
    }

    if !documentation_checks.missing_docs.is_empty() {
        notes.push(format!(
            "some expected docs are missing: {}",
            documentation_checks.missing_docs.join(", ")
        ));
    }

    notes
}

fn read_optional_text(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn source_metrics(path: &Path) -> Result<SourceMetrics, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    let production_only = contents.split("#[cfg(test)]").next().unwrap_or_default();
    let non_test_non_empty_lines = production_only
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .count();
    let public_function_count = production_only.matches("pub fn ").count();

    Ok(SourceMetrics {
        non_test_non_empty_lines,
        public_function_count,
    })
}

fn collect_directory_names(path: &Path) -> std::io::Result<BTreeSet<String>> {
    let mut names = BTreeSet::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            names.insert(name.to_owned());
        }
    }
    Ok(names)
}

fn collect_json_files(path: &Path, files: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_json_files(&path, files);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("json") {
            files.push(path);
        }
    }
}

fn string_field<'a>(value: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(serde_json::Value::as_str)
}

fn string_array_field<'a>(value: &'a serde_json::Value, key: &str) -> BTreeSet<&'a str> {
    value
        .get(key)
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(serde_json::Value::as_str)
        .collect()
}

fn nested_string<'a>(value: &'a serde_json::Value, path: &[&str]) -> Option<&'a str> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_str()
}

fn nested_bool(value: &serde_json::Value, path: &[&str]) -> Option<bool> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_bool()
}

fn contains_all_string_array(value: &serde_json::Value, key: &str, expected: &[&str]) -> bool {
    let values = string_array_field(value, key);
    expected.iter().all(|item| values.contains(item))
}

fn nested_contains_all_string_array(
    value: &serde_json::Value,
    path: &[&str],
    expected: &[&str],
) -> bool {
    let mut current = value;
    for key in path {
        let Some(next) = current.get(*key) else {
            return false;
        };
        current = next;
    }
    let values = current
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    expected.iter().all(|item| values.contains(item))
}

fn policy_rules_for_level<'a>(
    value: &'a serde_json::Value,
    level: &str,
) -> Vec<&'a serde_json::Map<String, serde_json::Value>> {
    value
        .get("rules")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(serde_json::Value::as_object)
        .filter(|rule| rule.get("level").and_then(serde_json::Value::as_str) == Some(level))
        .collect()
}

fn summarize_command_output(stdout: &str, stderr: &str) -> String {
    let combined = format!("{stdout}\n{stderr}");
    combined
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_owned())
        .unwrap_or_else(|| "wrapper produced no output".to_owned())
}

fn is_suspicious_project_crate(name: &str) -> bool {
    let lowered = name.to_ascii_lowercase();
    lowered != "project_runtime"
        && lowered.contains("project")
        && ["pipeline", "orchestr", "runtime", "service", "controller"]
            .iter()
            .any(|fragment| lowered.contains(fragment))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SourceMetrics {
    non_test_non_empty_lines: usize,
    public_function_count: usize,
}

#[cfg(test)]
mod tests {
    use super::{
        derive_overall_status, evaluate_documentation_checks, evaluate_workspace_structure,
        report_to_pretty_json, run_mechanical_validation, ArchitectureWatchpoint,
        DocumentationChecksReport, FindingSeverity, MechanicalValidationReport, OverallStatus,
        ProgressVerificationReport, ScriptRunOutcome, ScriptRunner, ValidationStepReport,
        VerificationMode, WorkspaceInventory, WorkspaceStructureReport,
    };
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::Path;

    struct FakeRunner {
        results: BTreeMap<String, ScriptRunOutcome>,
    }

    impl ScriptRunner for FakeRunner {
        fn run_script(&self, _workspace_root: &Path, script_name: &str) -> ScriptRunOutcome {
            self.results
                .get(script_name)
                .cloned()
                .unwrap_or(ScriptRunOutcome {
                    exit_code: 1,
                    output: "missing fake result".to_owned(),
                })
        }
    }

    #[test]
    fn workspace_structure_fails_when_critical_crate_is_missing() {
        let inventory = WorkspaceInventory::new(
            ["app_services", "cli_app"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            ["cargo_all.bat", "cargo_check.bat", "cargo_test.bat"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            BTreeSet::new(),
        );

        let report = evaluate_workspace_structure(&inventory);

        assert_eq!(report.status, OverallStatus::Fail);
        assert!(report
            .missing_critical_crates
            .contains(&"project_runtime".to_owned()));
    }

    #[test]
    fn documentation_checks_warn_when_expected_docs_are_missing() {
        let inventory = WorkspaceInventory::new(BTreeSet::new(), BTreeSet::new(), BTreeSet::new());

        let report = evaluate_documentation_checks(&inventory);

        assert_eq!(report.status, OverallStatus::OkWithWarnings);
        assert!(report.missing_docs.contains(&"README.md".to_owned()));
    }

    #[test]
    fn mechanical_validation_fails_when_any_required_wrapper_fails() {
        let runner = FakeRunner {
            results: BTreeMap::from([
                (
                    "cargo_check.bat".to_owned(),
                    ScriptRunOutcome {
                        exit_code: 0,
                        output: "ok".to_owned(),
                    },
                ),
                (
                    "cargo_test.bat".to_owned(),
                    ScriptRunOutcome {
                        exit_code: 101,
                        output: "failed".to_owned(),
                    },
                ),
            ]),
        };

        let report =
            run_mechanical_validation(Path::new("C:\\workspace"), VerificationMode::Quick, &runner);

        assert_eq!(report.status, OverallStatus::Fail);
        assert_eq!(report.steps[1].script, "cargo_test.bat");
        assert_eq!(report.steps[1].exit_code, 101);
    }

    #[test]
    fn overall_status_maps_warnings_without_failures() {
        let mechanical = MechanicalValidationReport {
            status: OverallStatus::Ok,
            mode: VerificationMode::Quick,
            steps: vec![],
        };
        let structure = WorkspaceStructureReport {
            status: OverallStatus::Ok,
            present_crates: vec![],
            missing_critical_crates: vec![],
            present_scripts: vec![],
            missing_critical_scripts: vec![],
            missing_advisory_scripts: vec![],
            extra_crates: vec![],
        };
        let docs = DocumentationChecksReport {
            status: OverallStatus::Ok,
            present_docs: vec![],
            missing_docs: vec![],
        };
        let watchpoints = vec![ArchitectureWatchpoint {
            id: "density".to_owned(),
            severity: FindingSeverity::Warning,
            message: "warning".to_owned(),
        }];

        let status = derive_overall_status(&mechanical, &structure, &docs, &watchpoints);

        assert_eq!(status, OverallStatus::OkWithWarnings);
    }

    #[test]
    fn report_serializes_to_json() {
        let report = ProgressVerificationReport {
            overall_status: OverallStatus::Ok,
            mode: VerificationMode::Quick,
            workspace_root: "C:/workspace".to_owned(),
            timestamp: "unix_ms:1".to_owned(),
            mechanical_validation: MechanicalValidationReport {
                status: OverallStatus::Ok,
                mode: VerificationMode::Quick,
                steps: vec![ValidationStepReport {
                    script: "cargo_check.bat".to_owned(),
                    status: super::StepStatus::Pass,
                    exit_code: 0,
                    detail: "ok".to_owned(),
                }],
            },
            workspace_structure: WorkspaceStructureReport {
                status: OverallStatus::Ok,
                present_crates: vec!["project_runtime".to_owned()],
                missing_critical_crates: vec![],
                present_scripts: vec!["cargo_check.bat".to_owned()],
                missing_critical_scripts: vec![],
                missing_advisory_scripts: vec![],
                extra_crates: vec![],
            },
            architecture_watchpoints: vec![],
            documentation_checks: DocumentationChecksReport {
                status: OverallStatus::Ok,
                present_docs: vec!["README.md".to_owned()],
                missing_docs: vec![],
            },
            notes: vec!["note".to_owned()],
        };

        let json = report_to_pretty_json(&report).expect("json");

        assert!(json.contains("\"overall_status\": \"OK\""));
        assert!(json.contains("\"mode\": \"quick\""));
    }
}
