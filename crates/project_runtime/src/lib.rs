//! First contract layer for project-scoped ref-driven runtime behavior.

use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use core_domain::{ErrorDomain, ManifestId, ManifestRefId, PortableAppError, ProjectId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spec_runtime::{
    load_document_from_disk, DocumentCategory, DocumentSource, LoadedDocument, SourceAuthority,
};
use workspace_core::{ProjectRoot, WorkspacePath, WorkspaceRoot};

/// Result alias for project contract validation.
pub type ProjectRuntimeResult<T> = Result<T, ProjectRuntimeError>;

/// Closed set of runtime operations observed by the current project flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeOperation {
    OpenProject,
    LoadManifest,
    BuildManifestContract,
    ValidateManifest,
    BuildSurfaceModel,
    ResolveViewerTargets,
}

impl RuntimeOperation {
    /// Return the stable runtime operation name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OpenProject => "open_project",
            Self::LoadManifest => "load_manifest",
            Self::BuildManifestContract => "build_manifest_contract",
            Self::ValidateManifest => "validate_manifest",
            Self::BuildSurfaceModel => "build_surface_model",
            Self::ResolveViewerTargets => "resolve_viewer_targets",
        }
    }
}

/// Closed status set for the minimal runtime event stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeStatus {
    Start,
    Success,
    Failure,
}

impl RuntimeStatus {
    /// Return the stable runtime status name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Success => "success",
            Self::Failure => "failure",
        }
    }
}

/// Minimal structured runtime event emitted by the project flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEvent {
    operation: RuntimeOperation,
    status: RuntimeStatus,
    metadata: Option<String>,
}

impl RuntimeEvent {
    /// Build a runtime event with explicit metadata.
    pub fn new(
        operation: RuntimeOperation,
        status: RuntimeStatus,
        metadata: Option<impl Into<String>>,
    ) -> Self {
        Self {
            operation,
            status,
            metadata: metadata.map(Into::into),
        }
    }

    /// Return the event operation.
    pub fn operation(&self) -> RuntimeOperation {
        self.operation
    }

    /// Return the event status.
    pub fn status(&self) -> RuntimeStatus {
        self.status
    }

    /// Return the optional event metadata.
    pub fn metadata(&self) -> Option<&str> {
        self.metadata.as_deref()
    }
}

/// Value plus accumulated runtime events for one observed operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeResult<T> {
    pub value: T,
    pub events: Vec<RuntimeEvent>,
}

impl<T> RuntimeResult<T> {
    /// Build a runtime result from a value and its observed events.
    pub fn new(value: T, events: Vec<RuntimeEvent>) -> Self {
        Self { value, events }
    }
}

/// Portable app error plus emitted runtime events for a failed observed operation.
#[derive(Debug, Clone)]
pub struct RuntimeFailure {
    error: PortableAppError,
    events: Vec<RuntimeEvent>,
}

impl RuntimeFailure {
    /// Build a runtime failure from the portable error and accumulated events.
    pub fn new(error: PortableAppError, events: Vec<RuntimeEvent>) -> Self {
        Self { error, events }
    }

    /// Return the portable error for the failed operation.
    pub fn error(&self) -> &PortableAppError {
        &self.error
    }

    /// Return the accumulated events for the failed operation.
    pub fn events(&self) -> &[RuntimeEvent] {
        &self.events
    }

    /// Split the failure into portable error and accumulated events.
    pub fn into_parts(self) -> (PortableAppError, Vec<RuntimeEvent>) {
        (self.error, self.events)
    }
}

/// Error type for conservative project contract construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectRuntimeError {
    ManifestSourceMustBeManifest,
    EmptyLabel,
    ViewerTargetPathMissing {
        target_ref: String,
    },
    ViewerTargetOutsideProject {
        target_ref: String,
        project_root: PathBuf,
        resolved_path: PathBuf,
    },
    SnapshotMissingField {
        field: String,
    },
    SnapshotMalformed {
        reason: String,
    },
    UnsupportedTargetKind {
        value: String,
    },
}

impl ProjectRuntimeError {
    fn code(&self) -> &'static str {
        match self {
            Self::ManifestSourceMustBeManifest => "manifest_source_must_be_manifest",
            Self::EmptyLabel => "empty_label",
            Self::ViewerTargetPathMissing { .. } => "viewer_target_path_missing",
            Self::ViewerTargetOutsideProject { .. } => "viewer_target_outside_project",
            Self::SnapshotMissingField { .. } => "snapshot_missing_field",
            Self::SnapshotMalformed { .. } => "snapshot_malformed",
            Self::UnsupportedTargetKind { .. } => "unsupported_target_kind",
        }
    }
}

impl fmt::Display for ProjectRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ManifestSourceMustBeManifest => {
                f.write_str("project manifest source must use the manifest document category")
            }
            Self::EmptyLabel => f.write_str("label must not be empty when provided"),
            Self::ViewerTargetPathMissing { target_ref } => {
                write!(f, "viewer target '{target_ref}' is missing a relative path")
            }
            Self::ViewerTargetOutsideProject {
                target_ref,
                project_root,
                resolved_path,
            } => write!(
                f,
                "viewer target '{}' resolved outside project root '{}': {}",
                target_ref,
                project_root.display(),
                resolved_path.display()
            ),
            Self::SnapshotMissingField { field } => {
                write!(f, "manifest snapshot is missing required field '{field}'")
            }
            Self::SnapshotMalformed { reason } => {
                write!(f, "manifest snapshot is malformed: {reason}")
            }
            Self::UnsupportedTargetKind { value } => {
                write!(
                    f,
                    "manifest snapshot uses unsupported target kind '{value}'"
                )
            }
        }
    }
}

impl Error for ProjectRuntimeError {}

impl From<ProjectRuntimeError> for PortableAppError {
    fn from(value: ProjectRuntimeError) -> Self {
        PortableAppError::new(ErrorDomain::ProjectRuntime, value.code(), value.to_string())
    }
}

/// Allowed exposure surfaces inherited from the governed ref-driven project model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceName {
    Tree,
    Viewer,
    ChatContext,
}

impl SurfaceName {
    /// Return the stable surface name used by lightweight consumers.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Tree => "tree",
            Self::Viewer => "viewer",
            Self::ChatContext => "chat_context",
        }
    }
}

/// Allowed target kinds for ref-driven project visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetKind {
    Document,
    Output,
    Artifact,
    Run,
    Chat,
    Project,
}

impl TargetKind {
    /// Return the stable target kind name used by lightweight consumers.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::Output => "output",
            Self::Artifact => "artifact",
            Self::Run => "run",
            Self::Chat => "chat",
            Self::Project => "project",
        }
    }
}

/// Typed project descriptor for project-scoped runtime operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDescriptor {
    project_id: ProjectId,
    project_root: ProjectRoot,
    manifest_source: DocumentSource,
}

impl ProjectDescriptor {
    /// Build a typed project descriptor tied to a manifest source.
    pub fn new(
        project_id: ProjectId,
        project_root: ProjectRoot,
        manifest_source: DocumentSource,
    ) -> ProjectRuntimeResult<Self> {
        if manifest_source.category() != DocumentCategory::Manifest {
            return Err(ProjectRuntimeError::ManifestSourceMustBeManifest);
        }

        Ok(Self {
            project_id,
            project_root,
            manifest_source,
        })
    }

    /// Return the stable project identifier.
    pub fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Return the canonical project root.
    pub fn project_root(&self) -> &ProjectRoot {
        &self.project_root
    }

    /// Return the typed manifest source.
    pub fn manifest_source(&self) -> &DocumentSource {
        &self.manifest_source
    }
}

/// Typed manifest reference descriptor used by the first ref-driven contract layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestRef {
    target_ref: ManifestRefId,
    target_kind: TargetKind,
    relative_path: Option<WorkspacePath>,
    label: Option<String>,
}

impl ManifestRef {
    /// Build a manifest reference descriptor.
    pub fn new(
        target_ref: ManifestRefId,
        target_kind: TargetKind,
        relative_path: Option<WorkspacePath>,
        label: Option<impl AsRef<str>>,
    ) -> ProjectRuntimeResult<Self> {
        let label = match label {
            Some(value) => {
                let trimmed = value.as_ref().trim();
                if trimmed.is_empty() {
                    return Err(ProjectRuntimeError::EmptyLabel);
                }
                Some(trimmed.to_owned())
            }
            None => None,
        };

        Ok(Self {
            target_ref,
            target_kind,
            relative_path,
            label,
        })
    }

    /// Return the stable manifest reference id.
    pub fn target_ref(&self) -> &ManifestRefId {
        &self.target_ref
    }

    /// Return the target kind.
    pub fn target_kind(&self) -> TargetKind {
        self.target_kind
    }

    /// Return the optional project-scoped relative path.
    pub fn relative_path(&self) -> Option<&WorkspacePath> {
        self.relative_path.as_ref()
    }

    /// Return the optional UI-facing label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Narrow DTO for the first manifest snapshot loaded from disk.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ProjectManifestSnapshot {
    manifest_id: String,
    viewer_target_ref: String,
    viewer_target_kind: String,
    #[serde(default)]
    viewer_relative_path: Option<String>,
    #[serde(default)]
    viewer_label: Option<String>,
}

impl ProjectManifestSnapshot {
    /// Return the manifest identity declared by the snapshot.
    pub fn manifest_id(&self) -> &str {
        &self.manifest_id
    }

    /// Convert the narrow snapshot into the first typed manifest contract.
    pub fn into_contract(
        self,
        project_id: &ProjectId,
    ) -> ProjectRuntimeResult<ProjectManifestContract> {
        let viewer_ref = ManifestRef::new(
            ManifestRefId::new(self.viewer_target_ref).map_err(snapshot_malformed_error)?,
            parse_target_kind(self.viewer_target_kind)?,
            self.viewer_relative_path
                .map(WorkspacePath::new)
                .transpose()
                .map_err(snapshot_malformed_error)?,
            self.viewer_label,
        )?;

        Ok(ProjectManifestContract::new(
            ManifestId::new(self.manifest_id).map_err(snapshot_malformed_error)?,
            project_id.clone(),
            Vec::new(),
            vec![viewer_ref],
            Vec::new(),
        ))
    }
}

/// Small manifest contract placeholder for the initial Rust project layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectManifestContract {
    pub manifest_id: ManifestId,
    pub project_id: ProjectId,
    pub tree_refs: Vec<ManifestRef>,
    pub viewer_refs: Vec<ManifestRef>,
    pub chat_context_refs: Vec<ManifestRef>,
}

impl ProjectManifestContract {
    /// Build a minimal project manifest contract snapshot.
    pub fn new(
        manifest_id: ManifestId,
        project_id: ProjectId,
        tree_refs: Vec<ManifestRef>,
        viewer_refs: Vec<ManifestRef>,
        chat_context_refs: Vec<ManifestRef>,
    ) -> Self {
        Self {
            manifest_id,
            project_id,
            tree_refs,
            viewer_refs,
            chat_context_refs,
        }
    }
}

/// Neutral surface entry for controller-facing or UI-facing consumers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceEntry {
    pub target_ref: ManifestRefId,
    pub target_kind: TargetKind,
    pub relative_path: Option<WorkspacePath>,
    pub label: Option<String>,
}

/// Neutral surface model preserving authorized ref order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceModel {
    pub surface: SurfaceName,
    pub entries: Vec<SurfaceEntry>,
}

impl SurfaceModel {
    /// Build a neutral surface model from already-authorized refs.
    pub fn from_refs(surface: SurfaceName, refs: &[ManifestRef]) -> Self {
        let entries = refs
            .iter()
            .map(|reference| SurfaceEntry {
                target_ref: reference.target_ref.clone(),
                target_kind: reference.target_kind,
                relative_path: reference.relative_path.clone(),
                label: reference.label.clone(),
            })
            .collect();

        Self { surface, entries }
    }

    /// Return the surface represented by this model.
    pub fn surface(&self) -> SurfaceName {
        self.surface
    }

    /// Return the ordered surface entries.
    pub fn entries(&self) -> &[SurfaceEntry] {
        &self.entries
    }
}

impl SurfaceEntry {
    /// Return the stable manifest ref id for this entry.
    pub fn target_ref(&self) -> &ManifestRefId {
        &self.target_ref
    }

    /// Return the target kind for this entry.
    pub fn target_kind(&self) -> TargetKind {
        self.target_kind
    }

    /// Return the optional relative path for this entry.
    pub fn relative_path(&self) -> Option<&WorkspacePath> {
        self.relative_path.as_ref()
    }

    /// Return the optional display label for this entry.
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Machine-facing validation issue for conservative manifest checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestValidationIssue {
    pub code: &'static str,
    pub message: String,
}

/// High-level validation state for a minimal manifest contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestValidationStatus {
    Valid,
    Invalid,
}

/// Validation result for a minimal ref-driven manifest contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestValidationResult {
    pub status: ManifestValidationStatus,
    pub issues: Vec<ManifestValidationIssue>,
}

/// Small typed identity card for a validated project flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectIdentity {
    project_id: ProjectId,
    project_root: ProjectRoot,
    manifest_source: DocumentSource,
}

impl ProjectIdentity {
    /// Build a minimal project identity from the stable project descriptor.
    pub fn new(descriptor: &ProjectDescriptor) -> Self {
        Self {
            project_id: descriptor.project_id().clone(),
            project_root: descriptor.project_root().clone(),
            manifest_source: descriptor.manifest_source().clone(),
        }
    }

    /// Return the stable project identifier.
    pub fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Return the canonical project root.
    pub fn project_root(&self) -> &ProjectRoot {
        &self.project_root
    }

    /// Return the typed manifest source.
    pub fn manifest_source(&self) -> &DocumentSource {
        &self.manifest_source
    }
}

/// Unified runtime output for the current project vertical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectRuntimeOutput {
    identity: ProjectIdentity,
    contract: ProjectManifestContract,
    validation: ManifestValidationResult,
    surface: SurfaceModel,
    resolved_viewer_targets: Vec<ResolvedViewerTarget>,
}

impl ProjectRuntimeOutput {
    /// Build a unified output from the current project runtime flow.
    pub fn new(
        identity: ProjectIdentity,
        contract: ProjectManifestContract,
        validation: ManifestValidationResult,
        surface: SurfaceModel,
        resolved_viewer_targets: Vec<ResolvedViewerTarget>,
    ) -> Self {
        Self {
            identity,
            contract,
            validation,
            surface,
            resolved_viewer_targets,
        }
    }

    /// Return the project identity.
    pub fn identity(&self) -> &ProjectIdentity {
        &self.identity
    }

    /// Return the manifest contract produced by the pipeline.
    pub fn contract(&self) -> &ProjectManifestContract {
        &self.contract
    }

    /// Return the manifest validation result.
    pub fn validation(&self) -> &ManifestValidationResult {
        &self.validation
    }

    /// Return the built surface model.
    pub fn surface(&self) -> &SurfaceModel {
        &self.surface
    }

    /// Return the resolved viewer targets.
    pub fn resolved_viewer_targets(&self) -> &[ResolvedViewerTarget] {
        &self.resolved_viewer_targets
    }
}

/// Small runtime-safe resolved viewer target for later consumers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedViewerTarget {
    target_ref: ManifestRefId,
    target_kind: TargetKind,
    resolved_path: PathBuf,
    label: Option<String>,
}

impl ResolvedViewerTarget {
    /// Return the stable target reference id.
    pub fn target_ref(&self) -> &ManifestRefId {
        &self.target_ref
    }

    /// Return the resolved target kind.
    pub fn target_kind(&self) -> TargetKind {
        self.target_kind
    }

    /// Return the runtime-safe resolved path under the project root.
    pub fn resolved_path(&self) -> &std::path::Path {
        &self.resolved_path
    }

    /// Return the optional display label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Typed exposure request required before any future manifest exposure write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExposureRequest {
    pub exposure_request_id: String,
    pub intake_item_ref: Option<String>,
    pub stored_object_candidate_ref: Option<String>,
    pub requested_by_ref: String,
    pub owner_ref: String,
    pub trace_ref: String,
    pub requested_at: String,
    pub requested_exposure_kind: String,
    pub user_intent_comment: Option<String>,
    pub expected_manifest_target_kind: String,
    pub duplicate_handling_preference: Option<String>,
    pub request_status: String,
}

/// Narrow candidate status set for the minimal governed manifest exposure runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExposureCandidateStatus {
    Candidate,
    Blocked,
    Unsupported,
    Stale,
}

/// Eligibility outcomes accepted by the minimal governed manifest exposure runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExposureEligibilityResult {
    EligibleImportedNotExposed,
    SourceNotImported,
    BlockedIntakeItem,
    UnsupportedIntakeItem,
    DuplicatePolicyBlock,
    UnsafeMetadata,
}

/// Human-readable stale check result required before the manifest write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StaleCheckResult {
    Fresh,
    Stale,
    Unknown,
}

/// Inert candidate artifact consumed by the minimal governed manifest exposure runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExposureCandidate {
    pub exposure_candidate_id: String,
    pub exposure_request_ref: String,
    pub candidate_status: ExposureCandidateStatus,
    pub eligibility_result: ExposureEligibilityResult,
    pub source_intake_item_refs: Vec<String>,
    pub file_ref: String,
    pub content_hash: Option<String>,
    pub stored_object_candidate_ref: String,
    pub sanitized_display_label: String,
    pub blocking_reasons: Vec<String>,
    pub duplicate_assessment: Option<String>,
    pub owner_ref: String,
    pub trace_ref: String,
}

/// Minimal reviewer decision set accepted by the governed exposure runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumanConfirmationDecision {
    Accepted,
    Rejected,
}

/// Explicit reviewer decision artifact required before any manifest exposure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HumanConfirmation {
    pub confirmation_id: String,
    pub exposure_candidate_ref: String,
    pub reviewer_ref: String,
    pub decision: HumanConfirmationDecision,
    pub confirmed_at: String,
    pub trace_ref: String,
    pub stale_check_result: StaleCheckResult,
    pub risk_acknowledgement: Option<String>,
}

/// Minimal manifest entry written by the governed manifest exposure runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestExposureEntry {
    pub manifest_entry_id: String,
    pub exposed_object_ref: String,
    pub file_ref: String,
    pub content_hash: Option<String>,
    pub stored_object_candidate_ref: String,
    pub source_intake_item_ref: String,
    pub exposure_request_ref: String,
    pub exposure_candidate_ref: String,
    pub confirmation_ref: String,
    pub owner_ref: String,
    pub trace_ref: String,
    pub exposed_to_project_at: String,
    pub artifact_kind: String,
    pub display_label: String,
    pub content_kind: String,
    #[serde(default)]
    pub metadata_refs: Vec<String>,
    pub exposure_state: String,
}

/// Structured success result for the minimal governed manifest exposure runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestExposureResult {
    manifest_path: String,
    manifest_entry_count: usize,
    manifest_entry: ManifestExposureEntry,
}

impl ManifestExposureResult {
    pub fn manifest_path(&self) -> &str {
        &self.manifest_path
    }

    pub fn manifest_entry_count(&self) -> usize {
        self.manifest_entry_count
    }

    pub fn manifest_entry(&self) -> &ManifestExposureEntry {
        &self.manifest_entry
    }
}

/// Readonly tree row derived only from manifest-governed exposure entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestDocumentTreeRow {
    manifest_entry_id: String,
    exposed_object_ref: String,
    file_ref: String,
    owner_ref: String,
    trace_ref: String,
    display_label: String,
    artifact_kind: String,
    content_kind: String,
}

impl ManifestDocumentTreeRow {
    pub fn manifest_entry_id(&self) -> &str {
        &self.manifest_entry_id
    }

    pub fn exposed_object_ref(&self) -> &str {
        &self.exposed_object_ref
    }

    pub fn file_ref(&self) -> &str {
        &self.file_ref
    }

    pub fn owner_ref(&self) -> &str {
        &self.owner_ref
    }

    pub fn trace_ref(&self) -> &str {
        &self.trace_ref
    }

    pub fn display_label(&self) -> &str {
        &self.display_label
    }

    pub fn artifact_kind(&self) -> &str {
        &self.artifact_kind
    }

    pub fn content_kind(&self) -> &str {
        &self.content_kind
    }
}

/// Typed failures returned by the minimal governed manifest exposure runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestExposureError {
    MissingExposureRequest,
    MissingExposureCandidate,
    MissingAcceptedHumanConfirmation,
    RejectedConfirmation,
    StaleCandidate,
    MissingOwnerRef,
    MissingTraceRef,
    BlockedIntakeItem,
    UnsupportedIntakeItem,
    SourceNotImported,
    DuplicatePolicyBlock,
    ManifestConflict { reason: String },
    UnsafeMetadata { field: String },
    RawHostPath { field: String },
    UnauthorizedTransition { reason: String },
}

impl ManifestExposureError {
    fn code(&self) -> &'static str {
        match self {
            Self::MissingExposureRequest => "missing_exposure_request",
            Self::MissingExposureCandidate => "missing_exposure_candidate",
            Self::MissingAcceptedHumanConfirmation => "missing_accepted_human_confirmation",
            Self::RejectedConfirmation => "rejected_confirmation",
            Self::StaleCandidate => "stale_candidate",
            Self::MissingOwnerRef => "missing_owner_ref",
            Self::MissingTraceRef => "missing_trace_ref",
            Self::BlockedIntakeItem => "blocked_intake_item",
            Self::UnsupportedIntakeItem => "unsupported_intake_item",
            Self::SourceNotImported => "source_not_imported",
            Self::DuplicatePolicyBlock => "duplicate_policy_block",
            Self::ManifestConflict { .. } => "manifest_conflict",
            Self::UnsafeMetadata { .. } => "unsafe_metadata",
            Self::RawHostPath { .. } => "raw_host_path",
            Self::UnauthorizedTransition { .. } => "unauthorized_transition",
        }
    }
}

impl fmt::Display for ManifestExposureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingExposureRequest => f.write_str("missing ExposureRequest"),
            Self::MissingExposureCandidate => f.write_str("missing ExposureCandidate"),
            Self::MissingAcceptedHumanConfirmation => {
                f.write_str("missing accepted HumanConfirmation")
            }
            Self::RejectedConfirmation => f.write_str("rejected HumanConfirmation blocks exposure"),
            Self::StaleCandidate => f.write_str("stale candidate blocks manifest exposure"),
            Self::MissingOwnerRef => f.write_str("owner_ref is required"),
            Self::MissingTraceRef => f.write_str("trace_ref is required"),
            Self::BlockedIntakeItem => f.write_str("blocked intake item cannot expose"),
            Self::UnsupportedIntakeItem => f.write_str("unsupported intake item cannot expose"),
            Self::SourceNotImported => f.write_str("source item is not imported_not_exposed"),
            Self::DuplicatePolicyBlock => {
                f.write_str("duplicate policy blocks manifest exposure")
            }
            Self::ManifestConflict { reason } => {
                write!(f, "manifest conflict prevents exposure: {reason}")
            }
            Self::UnsafeMetadata { field } => {
                write!(f, "unsafe metadata in field '{field}'")
            }
            Self::RawHostPath { field } => {
                write!(f, "raw host path is not allowed in field '{field}'")
            }
            Self::UnauthorizedTransition { reason } => {
                write!(f, "unauthorized exposure transition: {reason}")
            }
        }
    }
}

impl Error for ManifestExposureError {}

impl From<ManifestExposureError> for PortableAppError {
    fn from(value: ManifestExposureError) -> Self {
        PortableAppError::new(ErrorDomain::ProjectRuntime, value.code(), value.to_string())
    }
}

/// Open a project descriptor and emit runtime events for the operation.
pub fn open_project(
    project_id: ProjectId,
    project_root: ProjectRoot,
    manifest_source: DocumentSource,
) -> Result<RuntimeResult<ProjectDescriptor>, RuntimeFailure> {
    let mut events = vec![RuntimeEvent::new(
        RuntimeOperation::OpenProject,
        RuntimeStatus::Start,
        Some(format!("project_id={project_id}")),
    )];

    match ProjectDescriptor::new(project_id, project_root, manifest_source) {
        Ok(descriptor) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::OpenProject,
                RuntimeStatus::Success,
                Some(format!("project_id={}", descriptor.project_id())),
            ));
            Ok(RuntimeResult::new(descriptor, events))
        }
        Err(error) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::OpenProject,
                RuntimeStatus::Failure,
                Some(error.to_string()),
            ));
            Err(RuntimeFailure::new(error.into(), events))
        }
    }
}

/// Open a project descriptor from the current project-root convention and emit runtime events.
pub fn open_project_from_root(
    workspace_root: &WorkspaceRoot,
    project_root: ProjectRoot,
) -> Result<RuntimeResult<ProjectDescriptor>, RuntimeFailure> {
    let mut events = vec![RuntimeEvent::new(
        RuntimeOperation::OpenProject,
        RuntimeStatus::Start,
        Some(format!("project_root={}", project_root.as_path().display())),
    )];

    let project_id = match project_id_from_root(&project_root) {
        Ok(project_id) => project_id,
        Err(error) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::OpenProject,
                RuntimeStatus::Failure,
                Some(error.to_string()),
            ));
            return Err(RuntimeFailure::new(error, events));
        }
    };

    let manifest_source = match manifest_source_from_project_root(workspace_root, &project_root) {
        Ok(manifest_source) => manifest_source,
        Err(error) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::OpenProject,
                RuntimeStatus::Failure,
                Some(error.to_string()),
            ));
            return Err(RuntimeFailure::new(error, events));
        }
    };

    match ProjectDescriptor::new(project_id, project_root, manifest_source) {
        Ok(descriptor) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::OpenProject,
                RuntimeStatus::Success,
                Some(format!("project_id={}", descriptor.project_id())),
            ));
            Ok(RuntimeResult::new(descriptor, events))
        }
        Err(error) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::OpenProject,
                RuntimeStatus::Failure,
                Some(error.to_string()),
            ));
            Err(RuntimeFailure::new(error.into(), events))
        }
    }
}

/// Load the project manifest from disk and emit runtime events for the operation.
pub fn load_manifest(
    workspace_root: &WorkspaceRoot,
    manifest_source: DocumentSource,
    authority: SourceAuthority,
) -> Result<RuntimeResult<LoadedDocument>, RuntimeFailure> {
    let mut events = vec![RuntimeEvent::new(
        RuntimeOperation::LoadManifest,
        RuntimeStatus::Start,
        Some(format!(
            "path={}",
            manifest_source.path().as_path().display()
        )),
    )];

    match load_document_from_disk(workspace_root, manifest_source, authority) {
        Ok(document) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::LoadManifest,
                RuntimeStatus::Success,
                Some(format!(
                    "path={}",
                    document.declared_path().as_path().display()
                )),
            ));
            Ok(RuntimeResult::new(document, events))
        }
        Err(error) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::LoadManifest,
                RuntimeStatus::Failure,
                Some(error.to_string()),
            ));
            Err(RuntimeFailure::new(error.into(), events))
        }
    }
}

/// Deserialize the narrow manifest snapshot used by the current contract path.
pub fn deserialize_manifest_snapshot(
    contents: &str,
) -> ProjectRuntimeResult<ProjectManifestSnapshot> {
    serde_json::from_str::<ProjectManifestSnapshot>(contents).map_err(map_snapshot_parse_error)
}

/// Build a minimal manifest contract from a narrow on-disk snapshot representation.
pub fn load_manifest_snapshot_contract(
    document: &LoadedDocument,
    project_id: &ProjectId,
) -> ProjectRuntimeResult<ProjectManifestContract> {
    deserialize_manifest_snapshot(document.contents())?.into_contract(project_id)
}

/// Build the manifest contract from a loaded document and emit runtime events for the operation.
pub fn build_manifest_contract(
    document: &LoadedDocument,
    project_id: &ProjectId,
) -> Result<RuntimeResult<ProjectManifestContract>, RuntimeFailure> {
    let mut events = vec![RuntimeEvent::new(
        RuntimeOperation::BuildManifestContract,
        RuntimeStatus::Start,
        Some(format!(
            "path={}",
            document.declared_path().as_path().display()
        )),
    )];

    match load_manifest_snapshot_contract(document, project_id) {
        Ok(contract) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::BuildManifestContract,
                RuntimeStatus::Success,
                Some(format!("manifest_id={}", contract.manifest_id)),
            ));
            Ok(RuntimeResult::new(contract, events))
        }
        Err(error) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::BuildManifestContract,
                RuntimeStatus::Failure,
                Some(error.to_string()),
            ));
            Err(RuntimeFailure::new(error.into(), events))
        }
    }
}

/// Validate the manifest contract and emit runtime events for the operation.
pub fn validate_manifest(
    contract: &ProjectManifestContract,
) -> RuntimeResult<ManifestValidationResult> {
    let mut events = vec![RuntimeEvent::new(
        RuntimeOperation::ValidateManifest,
        RuntimeStatus::Start,
        Some(format!("manifest_id={}", contract.manifest_id)),
    )];
    let result = validate_manifest_contract(contract);

    events.push(RuntimeEvent::new(
        RuntimeOperation::ValidateManifest,
        RuntimeStatus::Success,
        Some(format!(
            "status={} issues={}",
            match result.status {
                ManifestValidationStatus::Valid => "valid",
                ManifestValidationStatus::Invalid => "invalid",
            },
            result.issues.len()
        )),
    ));

    RuntimeResult::new(result, events)
}

/// Build a surface model from already-authorized refs and emit runtime events.
pub fn build_surface_model(
    surface: SurfaceName,
    refs: &[ManifestRef],
) -> RuntimeResult<SurfaceModel> {
    let mut events = vec![RuntimeEvent::new(
        RuntimeOperation::BuildSurfaceModel,
        RuntimeStatus::Start,
        Some(format!("surface={}", surface.as_str())),
    )];
    let surface_model = SurfaceModel::from_refs(surface, refs);

    events.push(RuntimeEvent::new(
        RuntimeOperation::BuildSurfaceModel,
        RuntimeStatus::Success,
        Some(format!(
            "surface={} entries={}",
            surface_model.surface().as_str(),
            surface_model.entries().len()
        )),
    ));

    RuntimeResult::new(surface_model, events)
}

/// Resolve already-validated viewer refs into runtime-safe absolute targets under the project root.
pub fn resolve_viewer_targets(
    project_root: &ProjectRoot,
    viewer_refs: &[ManifestRef],
) -> ProjectRuntimeResult<Vec<ResolvedViewerTarget>> {
    viewer_refs
        .iter()
        .map(|reference| {
            let relative_path = reference.relative_path().ok_or_else(|| {
                ProjectRuntimeError::ViewerTargetPathMissing {
                    target_ref: reference.target_ref().as_str().to_owned(),
                }
            })?;

            let resolved_path = project_root.join(relative_path);
            if !resolved_path.starts_with(project_root.as_path()) {
                return Err(ProjectRuntimeError::ViewerTargetOutsideProject {
                    target_ref: reference.target_ref().as_str().to_owned(),
                    project_root: project_root.as_path().to_path_buf(),
                    resolved_path,
                });
            }

            Ok(ResolvedViewerTarget {
                target_ref: reference.target_ref().clone(),
                target_kind: reference.target_kind(),
                resolved_path,
                label: reference.label().map(str::to_owned),
            })
        })
        .collect()
}

/// Resolve viewer refs into runtime-safe targets and emit runtime events.
pub fn resolve_viewer_targets_observed(
    project_root: &ProjectRoot,
    viewer_refs: &[ManifestRef],
) -> Result<RuntimeResult<Vec<ResolvedViewerTarget>>, RuntimeFailure> {
    let mut events = vec![RuntimeEvent::new(
        RuntimeOperation::ResolveViewerTargets,
        RuntimeStatus::Start,
        Some(format!("viewer_refs={}", viewer_refs.len())),
    )];

    match resolve_viewer_targets(project_root, viewer_refs) {
        Ok(targets) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::ResolveViewerTargets,
                RuntimeStatus::Success,
                Some(format!("resolved_targets={}", targets.len())),
            ));
            Ok(RuntimeResult::new(targets, events))
        }
        Err(error) => {
            events.push(RuntimeEvent::new(
                RuntimeOperation::ResolveViewerTargets,
                RuntimeStatus::Failure,
                Some(error.to_string()),
            ));
            Err(RuntimeFailure::new(error.into(), events))
        }
    }
}

/// Run the current project runtime pipeline and return a unified reusable output.
pub fn run_project_pipeline(
    workspace_root: &WorkspaceRoot,
    project_root: ProjectRoot,
) -> Result<RuntimeResult<ProjectRuntimeOutput>, RuntimeFailure> {
    let mut events = Vec::new();

    let opened_project = match open_project_from_root(workspace_root, project_root) {
        Ok(result) => result,
        Err(failure) => return Err(merge_runtime_failure(events, failure)),
    };
    events.extend(opened_project.events);
    let descriptor = opened_project.value;

    let loaded_manifest = match load_manifest(
        workspace_root,
        descriptor.manifest_source().clone(),
        SourceAuthority::Project,
    ) {
        Ok(result) => result,
        Err(failure) => return Err(merge_runtime_failure(events, failure)),
    };
    events.extend(loaded_manifest.events);

    let manifest_contract =
        match build_manifest_contract(&loaded_manifest.value, descriptor.project_id()) {
            Ok(result) => result,
            Err(failure) => return Err(merge_runtime_failure(events, failure)),
        };
    events.extend(manifest_contract.events);

    let validation = validate_manifest(&manifest_contract.value);
    events.extend(validation.events);

    let identity = ProjectIdentity::new(&descriptor);
    let surface = build_surface_model(SurfaceName::Viewer, &manifest_contract.value.viewer_refs);
    events.extend(surface.events);

    let resolved_viewer_targets = match resolve_viewer_targets_observed(
        identity.project_root(),
        &manifest_contract.value.viewer_refs,
    ) {
        Ok(result) => result,
        Err(failure) => return Err(merge_runtime_failure(events, failure)),
    };
    events.extend(resolved_viewer_targets.events);

    Ok(RuntimeResult::new(
        ProjectRuntimeOutput::new(
            identity,
            manifest_contract.value,
            validation.value,
            surface.value,
            resolved_viewer_targets.value,
        ),
        events,
    ))
}

const MANIFEST_EXPOSURE_ENTRIES_FIELD: &str = "manifest_exposure_entries";
const EXPOSED_TO_PROJECT_STATE: &str = "exposed_to_project";

/// Write exactly one governed manifest exposure entry into project_manifest.json.
pub fn expose_manifest_entry(
    workspace_root: &WorkspaceRoot,
    project_root: &ProjectRoot,
    exposure_request: Option<ExposureRequest>,
    exposure_candidate: Option<ExposureCandidate>,
    human_confirmation: Option<HumanConfirmation>,
) -> Result<ManifestExposureResult, ManifestExposureError> {
    let exposure_request = exposure_request.ok_or(ManifestExposureError::MissingExposureRequest)?;
    let exposure_candidate =
        exposure_candidate.ok_or(ManifestExposureError::MissingExposureCandidate)?;
    let human_confirmation =
        human_confirmation.ok_or(ManifestExposureError::MissingAcceptedHumanConfirmation)?;

    validate_owner_ref("request.owner_ref", &exposure_request.owner_ref)?;
    validate_owner_ref("candidate.owner_ref", &exposure_candidate.owner_ref)?;
    validate_trace_ref("request.trace_ref", &exposure_request.trace_ref)?;
    validate_trace_ref("candidate.trace_ref", &exposure_candidate.trace_ref)?;
    validate_trace_ref("confirmation.trace_ref", &human_confirmation.trace_ref)?;

    validate_sanitized_field(
        "request.exposure_request_id",
        &exposure_request.exposure_request_id,
    )?;
    validate_optional_sanitized_field(
        "request.intake_item_ref",
        exposure_request.intake_item_ref.as_deref(),
    )?;
    validate_optional_sanitized_field(
        "request.stored_object_candidate_ref",
        exposure_request.stored_object_candidate_ref.as_deref(),
    )?;
    validate_sanitized_field("request.requested_by_ref", &exposure_request.requested_by_ref)?;
    validate_sanitized_field("request.requested_at", &exposure_request.requested_at)?;
    validate_sanitized_field(
        "request.requested_exposure_kind",
        &exposure_request.requested_exposure_kind,
    )?;
    validate_optional_sanitized_field(
        "request.user_intent_comment",
        exposure_request.user_intent_comment.as_deref(),
    )?;
    validate_sanitized_field(
        "request.expected_manifest_target_kind",
        &exposure_request.expected_manifest_target_kind,
    )?;
    validate_optional_sanitized_field(
        "request.duplicate_handling_preference",
        exposure_request.duplicate_handling_preference.as_deref(),
    )?;
    validate_sanitized_field("request.request_status", &exposure_request.request_status)?;

    validate_sanitized_field(
        "candidate.exposure_candidate_id",
        &exposure_candidate.exposure_candidate_id,
    )?;
    validate_sanitized_field(
        "candidate.exposure_request_ref",
        &exposure_candidate.exposure_request_ref,
    )?;
    validate_sanitized_field("candidate.file_ref", &exposure_candidate.file_ref)?;
    validate_optional_sanitized_field(
        "candidate.content_hash",
        exposure_candidate.content_hash.as_deref(),
    )?;
    validate_sanitized_field(
        "candidate.stored_object_candidate_ref",
        &exposure_candidate.stored_object_candidate_ref,
    )?;
    validate_sanitized_field(
        "candidate.sanitized_display_label",
        &exposure_candidate.sanitized_display_label,
    )?;
    if exposure_candidate.sanitized_display_label.trim().is_empty() {
        return Err(ManifestExposureError::UnsafeMetadata {
            field: "candidate.sanitized_display_label".to_owned(),
        });
    }
    for source_ref in &exposure_candidate.source_intake_item_refs {
        validate_sanitized_field("candidate.source_intake_item_refs", source_ref)?;
    }
    for reason in &exposure_candidate.blocking_reasons {
        validate_sanitized_field("candidate.blocking_reasons", reason)?;
    }
    validate_optional_sanitized_field(
        "candidate.duplicate_assessment",
        exposure_candidate.duplicate_assessment.as_deref(),
    )?;

    validate_sanitized_field(
        "confirmation.confirmation_id",
        &human_confirmation.confirmation_id,
    )?;
    validate_sanitized_field(
        "confirmation.exposure_candidate_ref",
        &human_confirmation.exposure_candidate_ref,
    )?;
    validate_sanitized_field("confirmation.reviewer_ref", &human_confirmation.reviewer_ref)?;
    validate_sanitized_field("confirmation.confirmed_at", &human_confirmation.confirmed_at)?;
    validate_optional_sanitized_field(
        "confirmation.risk_acknowledgement",
        human_confirmation.risk_acknowledgement.as_deref(),
    )?;

    if exposure_request.owner_ref != exposure_candidate.owner_ref {
        return Err(ManifestExposureError::UnauthorizedTransition {
            reason: "request and candidate owner_ref must match".to_owned(),
        });
    }
    if exposure_request.trace_ref != exposure_candidate.trace_ref
        || exposure_request.trace_ref != human_confirmation.trace_ref
    {
        return Err(ManifestExposureError::UnauthorizedTransition {
            reason: "request, candidate, and confirmation trace_ref must match".to_owned(),
        });
    }
    if exposure_candidate.exposure_request_ref != exposure_request.exposure_request_id {
        return Err(ManifestExposureError::UnauthorizedTransition {
            reason: "candidate must reference the supplied ExposureRequest".to_owned(),
        });
    }
    if human_confirmation.exposure_candidate_ref != exposure_candidate.exposure_candidate_id {
        return Err(ManifestExposureError::UnauthorizedTransition {
            reason: "confirmation must reference the supplied ExposureCandidate".to_owned(),
        });
    }

    match exposure_candidate.candidate_status {
        ExposureCandidateStatus::Candidate => {}
        ExposureCandidateStatus::Blocked => return Err(ManifestExposureError::BlockedIntakeItem),
        ExposureCandidateStatus::Unsupported => {
            return Err(ManifestExposureError::UnsupportedIntakeItem)
        }
        ExposureCandidateStatus::Stale => return Err(ManifestExposureError::StaleCandidate),
    }

    match exposure_candidate.eligibility_result {
        ExposureEligibilityResult::EligibleImportedNotExposed => {}
        ExposureEligibilityResult::SourceNotImported => {
            return Err(ManifestExposureError::SourceNotImported)
        }
        ExposureEligibilityResult::BlockedIntakeItem => {
            return Err(ManifestExposureError::BlockedIntakeItem)
        }
        ExposureEligibilityResult::UnsupportedIntakeItem => {
            return Err(ManifestExposureError::UnsupportedIntakeItem)
        }
        ExposureEligibilityResult::DuplicatePolicyBlock => {
            return Err(ManifestExposureError::DuplicatePolicyBlock)
        }
        ExposureEligibilityResult::UnsafeMetadata => {
            return Err(ManifestExposureError::UnsafeMetadata {
                field: "candidate.eligibility_result".to_owned(),
            })
        }
    }

    match human_confirmation.decision {
        HumanConfirmationDecision::Accepted => {}
        HumanConfirmationDecision::Rejected => {
            return Err(ManifestExposureError::RejectedConfirmation)
        }
    }

    match human_confirmation.stale_check_result {
        StaleCheckResult::Fresh => {}
        StaleCheckResult::Stale | StaleCheckResult::Unknown => {
            return Err(ManifestExposureError::StaleCandidate)
        }
    }

    let source_intake_item_ref = exposure_request
        .intake_item_ref
        .clone()
        .or_else(|| exposure_candidate.source_intake_item_refs.first().cloned())
        .ok_or(ManifestExposureError::SourceNotImported)?;

    let manifest_entry = ManifestExposureEntry {
        manifest_entry_id: format!("manifest_entry.{}", exposure_request.exposure_request_id),
        exposed_object_ref: exposure_candidate.stored_object_candidate_ref.clone(),
        file_ref: exposure_candidate.file_ref.clone(),
        content_hash: exposure_candidate.content_hash.clone(),
        stored_object_candidate_ref: exposure_candidate.stored_object_candidate_ref.clone(),
        source_intake_item_ref,
        exposure_request_ref: exposure_request.exposure_request_id.clone(),
        exposure_candidate_ref: exposure_candidate.exposure_candidate_id.clone(),
        confirmation_ref: human_confirmation.confirmation_id.clone(),
        owner_ref: exposure_request.owner_ref.clone(),
        trace_ref: exposure_request.trace_ref.clone(),
        exposed_to_project_at: human_confirmation.confirmed_at.clone(),
        artifact_kind: exposure_request.expected_manifest_target_kind.clone(),
        display_label: exposure_candidate.sanitized_display_label.clone(),
        content_kind: exposure_request.requested_exposure_kind.clone(),
        metadata_refs: vec![exposure_request.requested_by_ref.clone()],
        exposure_state: EXPOSED_TO_PROJECT_STATE.to_owned(),
    };
    validate_manifest_exposure_entry(&manifest_entry)?;

    let descriptor = open_project_from_root(workspace_root, project_root.clone())
        .map_err(|failure| ManifestExposureError::ManifestConflict {
            reason: failure.error().to_string(),
        })?
        .value;
    let manifest_path = workspace_root.join(descriptor.manifest_source().path());
    let loaded_manifest = load_manifest(
        workspace_root,
        descriptor.manifest_source().clone(),
        SourceAuthority::Project,
    )
    .map_err(|failure| ManifestExposureError::ManifestConflict {
        reason: failure.error().to_string(),
    })?
    .value;

    let mut manifest_value = serde_json::from_str::<Value>(loaded_manifest.contents()).map_err(|err| {
        ManifestExposureError::ManifestConflict {
            reason: format!("manifest is not valid JSON: {err}"),
        }
    })?;
    let manifest_object = manifest_value
        .as_object_mut()
        .ok_or_else(|| ManifestExposureError::ManifestConflict {
            reason: "project_manifest.json must contain a JSON object".to_owned(),
        })?;

    let entries_value = manifest_object
        .entry(MANIFEST_EXPOSURE_ENTRIES_FIELD.to_owned())
        .or_insert_with(|| Value::Array(Vec::new()));
    let entries_array = entries_value
        .as_array_mut()
        .ok_or_else(|| ManifestExposureError::ManifestConflict {
            reason: format!("{MANIFEST_EXPOSURE_ENTRIES_FIELD} must be an array"),
        })?;

    let manifest_entry_count = {
        let existing_entries = parse_manifest_exposure_entries(entries_array)?;
        enforce_duplicate_policy(&existing_entries, &manifest_entry)?;

        entries_array.push(
            serde_json::to_value(&manifest_entry).map_err(|err| ManifestExposureError::ManifestConflict {
                reason: format!("failed to serialize manifest exposure entry: {err}"),
            })?,
        );
        entries_array.len()
    };

    let serialized =
        serde_json::to_string_pretty(&manifest_value).map_err(|err| ManifestExposureError::ManifestConflict {
            reason: format!("failed to render updated manifest JSON: {err}"),
        })?;
    fs::write(&manifest_path, serialized).map_err(|err| ManifestExposureError::ManifestConflict {
        reason: format!("failed to write project manifest: {err}"),
    })?;

    Ok(ManifestExposureResult {
        manifest_path: portable_path_text(descriptor.manifest_source().path().as_path()),
        manifest_entry_count,
        manifest_entry,
    })
}

/// Load readonly document-tree rows from manifest-governed exposure entries only.
pub fn load_manifest_document_tree_rows(
    workspace_root: &WorkspaceRoot,
    project_root: &ProjectRoot,
) -> Result<Vec<ManifestDocumentTreeRow>, ManifestExposureError> {
    let descriptor = open_project_from_root(workspace_root, project_root.clone())
        .map_err(|failure| ManifestExposureError::ManifestConflict {
            reason: failure.error().to_string(),
        })?
        .value;
    let loaded_manifest = load_manifest(
        workspace_root,
        descriptor.manifest_source().clone(),
        SourceAuthority::Project,
    )
    .map_err(|failure| ManifestExposureError::ManifestConflict {
        reason: failure.error().to_string(),
    })?
    .value;

    let manifest_value =
        serde_json::from_str::<Value>(loaded_manifest.contents()).map_err(|err| {
            ManifestExposureError::ManifestConflict {
                reason: format!("manifest is not valid JSON: {err}"),
            }
        })?;

    let entries = manifest_value
        .as_object()
        .and_then(|object| object.get(MANIFEST_EXPOSURE_ENTRIES_FIELD))
        .map(|value| {
            value.as_array()
                .ok_or_else(|| ManifestExposureError::ManifestConflict {
                    reason: format!("{MANIFEST_EXPOSURE_ENTRIES_FIELD} must be an array"),
                })
        })
        .transpose()?
        .map(|values| parse_manifest_exposure_entries(values))
        .transpose()?
        .unwrap_or_default();

    entries
        .into_iter()
        .filter(|entry| entry.exposure_state == EXPOSED_TO_PROJECT_STATE)
        .map(build_manifest_document_tree_row)
        .collect()
}

/// Validate a minimal ref-driven project manifest contract.
pub fn validate_manifest_contract(contract: &ProjectManifestContract) -> ManifestValidationResult {
    let mut issues = Vec::new();

    validate_surface_refs(SurfaceName::Tree, &contract.tree_refs, &mut issues);
    validate_surface_refs(SurfaceName::Viewer, &contract.viewer_refs, &mut issues);
    validate_surface_refs(
        SurfaceName::ChatContext,
        &contract.chat_context_refs,
        &mut issues,
    );

    let status = if issues.is_empty() {
        ManifestValidationStatus::Valid
    } else {
        ManifestValidationStatus::Invalid
    };

    ManifestValidationResult { status, issues }
}

fn validate_surface_refs(
    surface: SurfaceName,
    refs: &[ManifestRef],
    issues: &mut Vec<ManifestValidationIssue>,
) {
    let mut seen = HashSet::new();

    for reference in refs {
        let target_ref = reference.target_ref().as_str();
        if !seen.insert(target_ref.to_owned()) {
            issues.push(ManifestValidationIssue {
                code: "duplicate_target_ref",
                message: format!(
                    "surface {:?} contains duplicate target_ref '{}'",
                    surface, target_ref
                ),
            });
        }

        if surface == SurfaceName::Viewer && !viewer_target_kind_allowed(reference.target_kind()) {
            issues.push(ManifestValidationIssue {
                code: "viewer_target_kind_not_allowed",
                message: format!(
                    "viewer surface does not allow target kind {:?} for '{}'",
                    reference.target_kind(),
                    target_ref
                ),
            });
        }
    }
}

fn viewer_target_kind_allowed(kind: TargetKind) -> bool {
    matches!(
        kind,
        TargetKind::Document | TargetKind::Output | TargetKind::Artifact | TargetKind::Run
    )
}

fn parse_target_kind(value: String) -> ProjectRuntimeResult<TargetKind> {
    match value.as_str() {
        "document" => Ok(TargetKind::Document),
        "output" => Ok(TargetKind::Output),
        "artifact" => Ok(TargetKind::Artifact),
        "run" => Ok(TargetKind::Run),
        "chat" => Ok(TargetKind::Chat),
        "project" => Ok(TargetKind::Project),
        _ => Err(ProjectRuntimeError::UnsupportedTargetKind { value }),
    }
}

fn snapshot_malformed_error(error: impl fmt::Display) -> ProjectRuntimeError {
    ProjectRuntimeError::SnapshotMalformed {
        reason: error.to_string(),
    }
}

fn merge_runtime_failure(
    mut existing_events: Vec<RuntimeEvent>,
    failure: RuntimeFailure,
) -> RuntimeFailure {
    let (error, failure_events) = failure.into_parts();
    existing_events.extend(failure_events);
    RuntimeFailure::new(error, existing_events)
}

fn project_id_from_root(project_root: &ProjectRoot) -> Result<ProjectId, PortableAppError> {
    let project_name = project_root
        .as_path()
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            PortableAppError::new(
                ErrorDomain::ProjectRuntime,
                "invalid_project_name",
                "project path must end with a valid directory name",
            )
        })?;

    ProjectId::new(project_name)
}

fn manifest_source_from_project_root(
    workspace_root: &WorkspaceRoot,
    project_root: &ProjectRoot,
) -> Result<DocumentSource, PortableAppError> {
    let relative_project_root = project_root
        .as_path()
        .strip_prefix(workspace_root.as_path())
        .map_err(|_| {
            PortableAppError::new(
                ErrorDomain::ProjectRuntime,
                "project_outside_workspace",
                "project root could not be expressed relative to workspace root",
            )
        })?;

    let manifest_relative = relative_project_root
        .join("config")
        .join("project_manifest.json");
    let manifest_relative = WorkspacePath::new(manifest_relative.to_string_lossy())?;

    Ok(DocumentSource::new(
        DocumentCategory::Manifest,
        "project_manifest",
        manifest_relative,
    )?)
}

fn validate_owner_ref(field: &str, value: &str) -> Result<(), ManifestExposureError> {
    if value.trim().is_empty() {
        return Err(ManifestExposureError::MissingOwnerRef);
    }
    validate_no_raw_host_path(field, value)
}

fn validate_trace_ref(field: &str, value: &str) -> Result<(), ManifestExposureError> {
    if value.trim().is_empty() {
        return Err(ManifestExposureError::MissingTraceRef);
    }
    validate_no_raw_host_path(field, value)
}

fn validate_sanitized_field(field: &str, value: &str) -> Result<(), ManifestExposureError> {
    if value.trim().is_empty() {
        return Err(ManifestExposureError::UnsafeMetadata {
            field: field.to_owned(),
        });
    }
    validate_no_raw_host_path(field, value)
}

fn validate_optional_sanitized_field(
    field: &str,
    value: Option<&str>,
) -> Result<(), ManifestExposureError> {
    if let Some(value) = value {
        validate_no_raw_host_path(field, value)?;
    }
    Ok(())
}

fn validate_no_raw_host_path(field: &str, value: &str) -> Result<(), ManifestExposureError> {
    if contains_raw_host_path(value) {
        return Err(ManifestExposureError::RawHostPath {
            field: field.to_owned(),
        });
    }
    Ok(())
}

fn contains_raw_host_path(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.len() >= 3 {
        let bytes = trimmed.as_bytes();
        if bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && (bytes[2] == b'\\' || bytes[2] == b'/') {
            return true;
        }
    }

    let unix_users_prefix = format!("/{}/", "Users");
    let unix_home_prefix = format!("/{}/", "home");
    let unix_private_var_prefix = format!("/{}/{}/", "private", "var");

    trimmed.starts_with(r"\\")
        || trimmed.starts_with(&unix_users_prefix)
        || trimmed.starts_with(&unix_home_prefix)
        || trimmed.starts_with(&unix_private_var_prefix)
}

fn portable_path_text(path: &std::path::Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

fn validate_manifest_exposure_entry(
    entry: &ManifestExposureEntry,
) -> Result<(), ManifestExposureError> {
    validate_sanitized_field("entry.manifest_entry_id", &entry.manifest_entry_id)?;
    validate_sanitized_field("entry.exposed_object_ref", &entry.exposed_object_ref)?;
    validate_sanitized_field("entry.file_ref", &entry.file_ref)?;
    validate_optional_sanitized_field("entry.content_hash", entry.content_hash.as_deref())?;
    validate_sanitized_field(
        "entry.stored_object_candidate_ref",
        &entry.stored_object_candidate_ref,
    )?;
    validate_sanitized_field("entry.source_intake_item_ref", &entry.source_intake_item_ref)?;
    validate_sanitized_field("entry.exposure_request_ref", &entry.exposure_request_ref)?;
    validate_sanitized_field("entry.exposure_candidate_ref", &entry.exposure_candidate_ref)?;
    validate_sanitized_field("entry.confirmation_ref", &entry.confirmation_ref)?;
    validate_owner_ref("entry.owner_ref", &entry.owner_ref)?;
    validate_trace_ref("entry.trace_ref", &entry.trace_ref)?;
    validate_sanitized_field("entry.exposed_to_project_at", &entry.exposed_to_project_at)?;
    validate_sanitized_field("entry.artifact_kind", &entry.artifact_kind)?;
    validate_sanitized_field("entry.display_label", &entry.display_label)?;
    validate_sanitized_field("entry.content_kind", &entry.content_kind)?;
    if entry.exposure_state != EXPOSED_TO_PROJECT_STATE {
        return Err(ManifestExposureError::UnauthorizedTransition {
            reason: "manifest exposure entry must persist exposed_to_project state".to_owned(),
        });
    }
    for metadata_ref in &entry.metadata_refs {
        validate_sanitized_field("entry.metadata_refs", metadata_ref)?;
    }
    Ok(())
}

fn build_manifest_document_tree_row(
    entry: ManifestExposureEntry,
) -> Result<ManifestDocumentTreeRow, ManifestExposureError> {
    validate_manifest_exposure_entry(&entry)?;

    let display_label = if entry.display_label.trim().is_empty() {
        format!("Project-visible artifact {}", entry.manifest_entry_id)
    } else {
        entry.display_label.trim().to_owned()
    };

    Ok(ManifestDocumentTreeRow {
        manifest_entry_id: entry.manifest_entry_id,
        exposed_object_ref: entry.exposed_object_ref,
        file_ref: entry.file_ref,
        owner_ref: entry.owner_ref,
        trace_ref: entry.trace_ref,
        display_label,
        artifact_kind: entry.artifact_kind,
        content_kind: entry.content_kind,
    })
}

fn parse_manifest_exposure_entries(
    values: &[Value],
) -> Result<Vec<ManifestExposureEntry>, ManifestExposureError> {
    values
        .iter()
        .map(|value| {
            serde_json::from_value::<ManifestExposureEntry>(value.clone()).map_err(|err| {
                ManifestExposureError::ManifestConflict {
                    reason: format!("existing manifest exposure entry is malformed: {err}"),
                }
            })
        })
        .collect()
}

fn enforce_duplicate_policy(
    existing_entries: &[ManifestExposureEntry],
    new_entry: &ManifestExposureEntry,
) -> Result<(), ManifestExposureError> {
    for existing in existing_entries {
        if existing.manifest_entry_id == new_entry.manifest_entry_id {
            return Err(ManifestExposureError::ManifestConflict {
                reason: format!(
                    "manifest entry id '{}' already exists",
                    new_entry.manifest_entry_id
                ),
            });
        }
        if existing.exposure_request_ref == new_entry.exposure_request_ref
            || existing.exposure_candidate_ref == new_entry.exposure_candidate_ref
            || existing.confirmation_ref == new_entry.confirmation_ref
        {
            return Err(ManifestExposureError::DuplicatePolicyBlock);
        }
    }
    Ok(())
}

fn map_snapshot_parse_error(error: serde_json::Error) -> ProjectRuntimeError {
    let message = error.to_string();

    if let Some(field) = parse_missing_field(&message) {
        return ProjectRuntimeError::SnapshotMissingField { field };
    }

    ProjectRuntimeError::SnapshotMalformed { reason: message }
}

fn parse_missing_field(message: &str) -> Option<String> {
    let field_start = message.strip_prefix("missing field `")?;
    let field_end = field_start.find('`')?;
    Some(field_start[..field_end].to_owned())
}

#[cfg(test)]
mod tests {
    use super::{
        build_manifest_contract, build_surface_model, deserialize_manifest_snapshot, load_manifest,
        expose_manifest_entry, load_manifest_document_tree_rows, load_manifest_snapshot_contract,
        open_project, open_project_from_root, resolve_viewer_targets,
        resolve_viewer_targets_observed, run_project_pipeline, validate_manifest,
        validate_manifest_contract, ExposureCandidate, ExposureCandidateStatus,
        ExposureEligibilityResult, ExposureRequest, HumanConfirmation,
        HumanConfirmationDecision, ManifestExposureError, ManifestExposureEntry, ManifestRef,
        ManifestValidationStatus, ProjectDescriptor,
        ProjectIdentity, ProjectManifestContract, ProjectManifestSnapshot, ProjectRuntimeError,
        ProjectRuntimeOutput, RuntimeEvent, RuntimeOperation, RuntimeResult, RuntimeStatus,
        StaleCheckResult, SurfaceModel, SurfaceName, TargetKind,
    };
    use core_domain::{ManifestId, ManifestRefId, ProjectId};
    use spec_runtime::{
        DocumentCategory, DocumentSource, LoadedDocument, SourceAuthority, SourceProvenance,
    };
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use workspace_core::{ProjectRoot, WorkspacePath, WorkspaceRoot};

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before epoch")
            .as_nanos();
        path.push(format!("{prefix}_{}_{}", std::process::id(), stamp));
        path
    }

    fn stable_relative_path(path: &Path) -> String {
        path.components()
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/")
    }

    fn normalize_absolute_path(path: &Path) -> PathBuf {
        let rendered = path.display().to_string();
        if let Some(trimmed) = rendered.strip_prefix(r"\\?\") {
            PathBuf::from(trimmed)
        } else {
            path.to_path_buf()
        }
    }

    fn build_valid_project_runtime_output(prefix: &str) -> (ProjectRuntimeOutput, PathBuf) {
        let workspace_dir = unique_temp_dir(prefix);
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let config_dir = project_dir.join("config");
        fs::create_dir_all(&config_dir).expect("create config dir");
        fs::write(
            config_dir.join("project_manifest.json"),
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        )
        .expect("write manifest");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let result = run_project_pipeline(&workspace_root, project_root).expect("pipeline result");

        (result.value, workspace_dir)
    }

    fn run_valid_project_pipeline(prefix: &str) -> (RuntimeResult<ProjectRuntimeOutput>, PathBuf) {
        let workspace_dir = unique_temp_dir(prefix);
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let config_dir = project_dir.join("config");
        fs::create_dir_all(&config_dir).expect("create config dir");
        fs::write(
            config_dir.join("project_manifest.json"),
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        )
        .expect("write manifest");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let result = run_project_pipeline(&workspace_root, project_root).expect("pipeline result");

        (result, workspace_dir)
    }

    fn build_manifest_exposure_workspace(prefix: &str) -> (WorkspaceRoot, ProjectRoot, PathBuf) {
        let workspace_dir = unique_temp_dir(prefix);
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let config_dir = project_dir.join("config");
        fs::create_dir_all(&config_dir).expect("create config dir");
        fs::write(
            config_dir.join("project_manifest.json"),
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        )
        .expect("write manifest");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root =
            ProjectRoot::new(&workspace_root, project_dir.clone()).expect("project root");
        (workspace_root, project_root, workspace_dir)
    }

    fn valid_exposure_request() -> ExposureRequest {
        ExposureRequest {
            exposure_request_id: "exposure_request.alpha".to_owned(),
            intake_item_ref: Some("intake_item.alpha".to_owned()),
            stored_object_candidate_ref: Some("stored_object_candidate.alpha".to_owned()),
            requested_by_ref: "user.alice".to_owned(),
            owner_ref: "owner/demo".to_owned(),
            trace_ref: "trace_demo".to_owned(),
            requested_at: "2026-05-02T10:00:00Z".to_owned(),
            requested_exposure_kind: "text".to_owned(),
            user_intent_comment: Some("promote imported artifact".to_owned()),
            expected_manifest_target_kind: "document".to_owned(),
            duplicate_handling_preference: Some("allow_distinct_logical_entry".to_owned()),
            request_status: "requested".to_owned(),
        }
    }

    fn valid_exposure_candidate() -> ExposureCandidate {
        ExposureCandidate {
            exposure_candidate_id: "exposure_candidate.alpha".to_owned(),
            exposure_request_ref: "exposure_request.alpha".to_owned(),
            candidate_status: ExposureCandidateStatus::Candidate,
            eligibility_result: ExposureEligibilityResult::EligibleImportedNotExposed,
            source_intake_item_refs: vec!["intake_item.alpha".to_owned()],
            file_ref: "file_ref.alpha".to_owned(),
            content_hash: Some("sha256_alpha".to_owned()),
            stored_object_candidate_ref: "stored_object_candidate.alpha".to_owned(),
            sanitized_display_label: "Alpha README".to_owned(),
            blocking_reasons: Vec::new(),
            duplicate_assessment: Some("distinct_logical_entry".to_owned()),
            owner_ref: "owner/demo".to_owned(),
            trace_ref: "trace_demo".to_owned(),
        }
    }

    fn valid_human_confirmation() -> HumanConfirmation {
        HumanConfirmation {
            confirmation_id: "confirmation.alpha".to_owned(),
            exposure_candidate_ref: "exposure_candidate.alpha".to_owned(),
            reviewer_ref: "reviewer.alice".to_owned(),
            decision: HumanConfirmationDecision::Accepted,
            confirmed_at: "2026-05-02T10:05:00Z".to_owned(),
            trace_ref: "trace_demo".to_owned(),
            stale_check_result: StaleCheckResult::Fresh,
            risk_acknowledgement: Some("approved_for_manifest_exposure".to_owned()),
        }
    }

    fn valid_manifest_exposure_entry() -> ManifestExposureEntry {
        ManifestExposureEntry {
            manifest_entry_id: "manifest_entry.alpha".to_owned(),
            exposed_object_ref: "exposed_object.alpha".to_owned(),
            file_ref: "file_ref.alpha".to_owned(),
            content_hash: Some("sha256_alpha".to_owned()),
            stored_object_candidate_ref: "stored_object_candidate.alpha".to_owned(),
            source_intake_item_ref: "intake_item.alpha".to_owned(),
            exposure_request_ref: "exposure_request.alpha".to_owned(),
            exposure_candidate_ref: "exposure_candidate.alpha".to_owned(),
            confirmation_ref: "confirmation.alpha".to_owned(),
            owner_ref: "owner/demo".to_owned(),
            trace_ref: "trace_demo".to_owned(),
            exposed_to_project_at: "2026-05-02T10:05:00Z".to_owned(),
            artifact_kind: "document".to_owned(),
            display_label: "Alpha README".to_owned(),
            content_kind: "text/markdown".to_owned(),
            metadata_refs: vec!["metadata.alpha".to_owned()],
            exposure_state: "exposed_to_project".to_owned(),
        }
    }

    fn read_manifest_exposure_entries(workspace_dir: &Path) -> Vec<ManifestExposureEntry> {
        let manifest_path = workspace_dir
            .join("user")
            .join("projects")
            .join("alpha")
            .join("config")
            .join("project_manifest.json");
        let contents = fs::read_to_string(manifest_path).expect("read manifest");
        let value: serde_json::Value = serde_json::from_str(&contents).expect("parse manifest");
        value["manifest_exposure_entries"]
            .as_array()
            .expect("exposure entries array")
            .iter()
            .map(|entry| serde_json::from_value(entry.clone()).expect("typed entry"))
            .collect()
    }

    fn write_manifest_exposure_entries(
        workspace_dir: &Path,
        entries: &[ManifestExposureEntry],
    ) {
        let manifest_path = workspace_dir
            .join("user")
            .join("projects")
            .join("alpha")
            .join("config")
            .join("project_manifest.json");
        let mut value: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&manifest_path).expect("read manifest"))
                .expect("parse manifest");
        let object = value.as_object_mut().expect("manifest object");
        object.insert(
            "manifest_exposure_entries".to_owned(),
            serde_json::to_value(entries).expect("entries value"),
        );
        fs::write(
            manifest_path,
            serde_json::to_string_pretty(&value).expect("render manifest"),
        )
        .expect("write manifest");
    }

    #[test]
    fn creates_runtime_event_with_metadata() {
        let event = RuntimeEvent::new(
            RuntimeOperation::OpenProject,
            RuntimeStatus::Start,
            Some("project_id=alpha"),
        );

        assert_eq!(event.operation(), RuntimeOperation::OpenProject);
        assert_eq!(event.status(), RuntimeStatus::Start);
        assert_eq!(event.metadata(), Some("project_id=alpha"));
    }

    #[test]
    fn builds_project_descriptor_and_surface_model() {
        let workspace_dir = unique_temp_dir("project_runtime_workspace");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        fs::create_dir_all(&project_dir).expect("create project dir");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let manifest_path = WorkspacePath::new("user/projects/alpha/config/project_manifest.json")
            .expect("manifest path");
        let manifest_source = DocumentSource::new(
            DocumentCategory::Manifest,
            "project_manifest",
            manifest_path,
        )
        .expect("manifest source");

        let descriptor = ProjectDescriptor::new(
            ProjectId::new("alpha").expect("project id"),
            project_root,
            manifest_source,
        )
        .expect("descriptor");

        let manifest_ref = ManifestRef::new(
            ManifestRefId::new("doc.main").expect("ref id"),
            TargetKind::Document,
            Some(WorkspacePath::new("docs/main.md").expect("relative path")),
            Some("Main Document"),
        )
        .expect("manifest ref");

        let surface = SurfaceModel::from_refs(SurfaceName::Viewer, &[manifest_ref]);

        assert_eq!(descriptor.project_id().as_str(), "alpha");
        assert_eq!(surface.entries().len(), 1);
        assert_eq!(surface.surface().as_str(), "viewer");
        assert_eq!(surface.entries()[0].target_ref().as_str(), "doc.main");
        assert_eq!(surface.entries()[0].target_kind().as_str(), "document");
        assert_eq!(
            surface.entries()[0]
                .relative_path()
                .expect("relative path")
                .as_path(),
            Path::new("docs/main.md")
        );
        assert_eq!(
            stable_relative_path(
                surface.entries()[0]
                    .relative_path()
                    .expect("relative path")
                    .as_path()
            ),
            "docs/main.md"
        );
        assert_eq!(surface.entries()[0].label(), Some("Main Document"));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn rejects_duplicate_manifest_refs() {
        let first = ManifestRef::new(
            ManifestRefId::new("dup.ref").expect("ref id"),
            TargetKind::Document,
            None,
            None::<&str>,
        )
        .expect("first ref");
        let second = ManifestRef::new(
            ManifestRefId::new("dup.ref").expect("ref id"),
            TargetKind::Document,
            None,
            None::<&str>,
        )
        .expect("second ref");

        let contract = ProjectManifestContract::new(
            ManifestId::new("manifest.alpha").expect("manifest id"),
            ProjectId::new("alpha").expect("project id"),
            vec![],
            vec![first, second],
            vec![],
        );

        let result = validate_manifest_contract(&contract);

        assert_eq!(result.status, ManifestValidationStatus::Invalid);
        assert_eq!(result.issues[0].code, "duplicate_target_ref");
    }

    #[test]
    fn rejects_chat_target_kind_in_viewer_surface() {
        let viewer_ref = ManifestRef::new(
            ManifestRefId::new("chat.ref").expect("ref id"),
            TargetKind::Chat,
            None,
            Some("Chat ref"),
        )
        .expect("viewer ref");

        let contract = ProjectManifestContract::new(
            ManifestId::new("manifest.alpha").expect("manifest id"),
            ProjectId::new("alpha").expect("project id"),
            vec![],
            vec![viewer_ref],
            vec![],
        );

        let result = validate_manifest_contract(&contract);

        assert_eq!(result.status, ManifestValidationStatus::Invalid);
        assert_eq!(result.issues[0].code, "viewer_target_kind_not_allowed");
    }

    #[test]
    fn loads_valid_manifest_snapshot_contract() {
        let path =
            WorkspacePath::new("user/projects/alpha/config/project_manifest.json").expect("path");
        let source =
            DocumentSource::new(DocumentCategory::Manifest, "project_manifest", path.clone())
                .expect("source");
        let provenance = SourceProvenance::new(SourceAuthority::Project, path, None::<&str>)
            .expect("provenance");
        let loaded = LoadedDocument::new(
            source,
            provenance,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        )
        .expect("loaded document");

        let contract =
            load_manifest_snapshot_contract(&loaded, &ProjectId::new("alpha").expect("project id"))
                .expect("contract");

        assert_eq!(contract.manifest_id.as_str(), "manifest.alpha");
        assert_eq!(contract.viewer_refs.len(), 1);
    }

    #[test]
    fn builds_project_identity_from_valid_flow() {
        let workspace_dir = unique_temp_dir("project_runtime_identity");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        fs::create_dir_all(&project_dir).expect("create project dir");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let manifest_path = WorkspacePath::new("user/projects/alpha/config/project_manifest.json")
            .expect("manifest path");
        let manifest_source = DocumentSource::new(
            DocumentCategory::Manifest,
            "project_manifest",
            manifest_path.clone(),
        )
        .expect("manifest source");
        let descriptor = ProjectDescriptor::new(
            ProjectId::new("alpha").expect("project id"),
            project_root,
            manifest_source,
        )
        .expect("descriptor");
        let provenance =
            SourceProvenance::new(SourceAuthority::Project, manifest_path, None::<&str>)
                .expect("provenance");
        let loaded = LoadedDocument::new(
            descriptor.manifest_source().clone(),
            provenance,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document"
}"#,
        )
        .expect("loaded document");
        let contract =
            load_manifest_snapshot_contract(&loaded, descriptor.project_id()).expect("contract");
        let identity = ProjectIdentity::new(&descriptor);

        assert_eq!(identity.project_id().as_str(), "alpha");
        assert_eq!(contract.manifest_id.as_str(), "manifest.alpha");
        assert_eq!(
            validate_manifest_contract(&contract).status,
            ManifestValidationStatus::Valid
        );
        assert_eq!(
            identity.manifest_source().path().as_path(),
            Path::new("user/projects/alpha/config/project_manifest.json")
        );
        assert_eq!(
            stable_relative_path(identity.manifest_source().path().as_path()),
            "user/projects/alpha/config/project_manifest.json"
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn deserializes_valid_manifest_snapshot() {
        let snapshot = deserialize_manifest_snapshot(
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        )
        .expect("snapshot");

        assert_eq!(snapshot.manifest_id(), "manifest.alpha");
    }

    #[test]
    fn missing_required_snapshot_field_fails_cleanly() {
        let error = deserialize_manifest_snapshot(
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme"
}"#,
        )
        .expect_err("missing field error");

        assert_eq!(
            error,
            ProjectRuntimeError::SnapshotMissingField {
                field: "viewer_target_kind".to_owned(),
            }
        );
    }

    #[test]
    fn invalid_viewer_target_kind_fails_cleanly() {
        let snapshot = deserialize_manifest_snapshot(
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "folder"
}"#,
        )
        .expect("snapshot");

        let error = snapshot
            .into_contract(&ProjectId::new("alpha").expect("project id"))
            .expect_err("invalid target kind");

        assert_eq!(
            error,
            ProjectRuntimeError::UnsupportedTargetKind {
                value: "folder".to_owned(),
            }
        );
    }

    #[test]
    fn converts_snapshot_into_manifest_contract() {
        let snapshot: ProjectManifestSnapshot = deserialize_manifest_snapshot(
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        )
        .expect("snapshot");

        let contract = snapshot
            .into_contract(&ProjectId::new("alpha").expect("project id"))
            .expect("contract");

        assert_eq!(contract.manifest_id.as_str(), "manifest.alpha");
        assert_eq!(contract.viewer_refs.len(), 1);
        assert_eq!(
            contract.viewer_refs[0]
                .relative_path()
                .expect("relative path")
                .as_path(),
            Path::new("docs/README.md")
        );
        assert_eq!(
            stable_relative_path(
                contract.viewer_refs[0]
                    .relative_path()
                    .expect("relative path")
                    .as_path()
            ),
            "docs/README.md"
        );
    }

    #[test]
    fn malformed_manifest_snapshot_fails_cleanly() {
        let error = deserialize_manifest_snapshot(
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": document
}"#,
        )
        .expect_err("malformed snapshot error");

        match error {
            ProjectRuntimeError::SnapshotMalformed { .. } => {}
            other => panic!("expected malformed snapshot error, got {other:?}"),
        }
    }

    #[test]
    fn resolves_viewer_targets_inside_project_root() {
        let workspace_dir = unique_temp_dir("project_runtime_resolve_viewer");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        fs::create_dir_all(&project_dir).expect("create project dir");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root =
            ProjectRoot::new(&workspace_root, project_dir.clone()).expect("project root");
        let viewer_ref = ManifestRef::new(
            ManifestRefId::new("document.readme").expect("ref id"),
            TargetKind::Document,
            Some(WorkspacePath::new("docs/README.md").expect("relative path")),
            Some("Project README"),
        )
        .expect("viewer ref");

        let resolved =
            resolve_viewer_targets(&project_root, &[viewer_ref]).expect("resolved targets");

        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].target_ref().as_str(), "document.readme");
        assert_eq!(resolved[0].target_kind().as_str(), "document");
        assert_eq!(
            normalize_absolute_path(resolved[0].resolved_path()),
            normalize_absolute_path(project_dir.join("docs").join("README.md").as_path())
        );
        assert_eq!(resolved[0].label(), Some("Project README"));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn fails_cleanly_when_viewer_target_has_no_relative_path() {
        let workspace_dir = unique_temp_dir("project_runtime_resolve_missing_path");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        fs::create_dir_all(&project_dir).expect("create project dir");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let viewer_ref = ManifestRef::new(
            ManifestRefId::new("document.readme").expect("ref id"),
            TargetKind::Document,
            None,
            Some("Project README"),
        )
        .expect("viewer ref");

        let error = resolve_viewer_targets(&project_root, &[viewer_ref]).expect_err("missing path");

        assert!(matches!(
            error,
            ProjectRuntimeError::ViewerTargetPathMissing { .. }
        ));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn open_project_emits_start_then_success_events() {
        let workspace_dir = unique_temp_dir("project_runtime_open_project");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        fs::create_dir_all(&project_dir).expect("create project dir");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let manifest_source = DocumentSource::new(
            DocumentCategory::Manifest,
            "project_manifest",
            WorkspacePath::new("user/projects/alpha/config/project_manifest.json")
                .expect("manifest path"),
        )
        .expect("manifest source");

        let result = open_project(
            ProjectId::new("alpha").expect("project id"),
            project_root,
            manifest_source,
        )
        .expect("open project result");

        assert_eq!(result.events.len(), 2);
        assert_eq!(result.events[0].operation(), RuntimeOperation::OpenProject);
        assert_eq!(result.events[0].status(), RuntimeStatus::Start);
        assert_eq!(result.events[1].operation(), RuntimeOperation::OpenProject);
        assert_eq!(result.events[1].status(), RuntimeStatus::Success);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn open_project_emits_start_then_failure_events() {
        let workspace_dir = unique_temp_dir("project_runtime_open_project_failure");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        fs::create_dir_all(&project_dir).expect("create project dir");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let manifest_source = DocumentSource::new(
            DocumentCategory::Spec,
            "project_manifest",
            WorkspacePath::new("user/projects/alpha/config/project_manifest.json")
                .expect("manifest path"),
        )
        .expect("manifest source");

        let failure = open_project(
            ProjectId::new("alpha").expect("project id"),
            project_root,
            manifest_source,
        )
        .expect_err("open project failure");

        assert_eq!(failure.events().len(), 2);
        assert_eq!(
            failure.events()[0].operation(),
            RuntimeOperation::OpenProject
        );
        assert_eq!(failure.events()[0].status(), RuntimeStatus::Start);
        assert_eq!(
            failure.events()[1].operation(),
            RuntimeOperation::OpenProject
        );
        assert_eq!(failure.events()[1].status(), RuntimeStatus::Failure);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn load_validate_build_and_resolve_emit_ordered_events() {
        let workspace_dir = unique_temp_dir("project_runtime_observed_flow");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let config_dir = project_dir.join("config");
        fs::create_dir_all(&config_dir).expect("create config dir");
        fs::write(
            config_dir.join("project_manifest.json"),
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        )
        .expect("write manifest");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let descriptor = open_project_from_root(&workspace_root, project_root)
            .expect("open project")
            .value;

        let loaded = load_manifest(
            &workspace_root,
            descriptor.manifest_source().clone(),
            SourceAuthority::Project,
        )
        .expect("load manifest");
        assert_eq!(loaded.events[0].operation(), RuntimeOperation::LoadManifest);
        assert_eq!(loaded.events[0].status(), RuntimeStatus::Start);
        assert_eq!(loaded.events[1].status(), RuntimeStatus::Success);

        let contract =
            build_manifest_contract(&loaded.value, descriptor.project_id()).expect("contract");
        assert_eq!(
            contract.events[0].operation(),
            RuntimeOperation::BuildManifestContract
        );
        assert_eq!(contract.events[0].status(), RuntimeStatus::Start);
        assert_eq!(contract.events[1].status(), RuntimeStatus::Success);

        let validation = validate_manifest(&contract.value);
        assert_eq!(
            validation.events[0].operation(),
            RuntimeOperation::ValidateManifest
        );
        assert_eq!(validation.events[0].status(), RuntimeStatus::Start);
        assert_eq!(validation.events[1].status(), RuntimeStatus::Success);

        let surface = build_surface_model(SurfaceName::Viewer, &contract.value.viewer_refs);
        assert_eq!(
            surface.events[0].operation(),
            RuntimeOperation::BuildSurfaceModel
        );
        assert_eq!(surface.events[0].status(), RuntimeStatus::Start);
        assert_eq!(surface.events[1].status(), RuntimeStatus::Success);

        let resolved =
            resolve_viewer_targets_observed(descriptor.project_root(), &contract.value.viewer_refs)
                .expect("resolve targets");
        assert_eq!(
            resolved.events[0].operation(),
            RuntimeOperation::ResolveViewerTargets
        );
        assert_eq!(resolved.events[0].status(), RuntimeStatus::Start);
        assert_eq!(resolved.events[1].status(), RuntimeStatus::Success);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn open_project_from_root_uses_project_opening_convention() {
        let workspace_dir = unique_temp_dir("project_runtime_open_from_root");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        fs::create_dir_all(&project_dir).expect("create project dir");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");

        let result =
            open_project_from_root(&workspace_root, project_root).expect("open project from root");

        assert_eq!(result.value.project_id().as_str(), "alpha");
        assert_eq!(
            result.value.manifest_source().path().as_path(),
            Path::new("user/projects/alpha/config/project_manifest.json")
        );
        assert_eq!(
            stable_relative_path(result.value.manifest_source().path().as_path()),
            "user/projects/alpha/config/project_manifest.json"
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn build_manifest_contract_emits_start_then_failure_events() {
        let path =
            WorkspacePath::new("user/projects/alpha/config/project_manifest.json").expect("path");
        let source =
            DocumentSource::new(DocumentCategory::Manifest, "project_manifest", path.clone())
                .expect("source");
        let provenance = SourceProvenance::new(SourceAuthority::Project, path, None::<&str>)
            .expect("provenance");
        let loaded = LoadedDocument::new(
            source,
            provenance,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme"
}"#,
        )
        .expect("loaded document");

        let failure =
            build_manifest_contract(&loaded, &ProjectId::new("alpha").expect("project id"))
                .expect_err("contract failure");

        assert_eq!(failure.events().len(), 2);
        assert_eq!(
            failure.events()[0].operation(),
            RuntimeOperation::BuildManifestContract
        );
        assert_eq!(failure.events()[0].status(), RuntimeStatus::Start);
        assert_eq!(failure.events()[1].status(), RuntimeStatus::Failure);
    }

    #[test]
    fn builds_project_runtime_output_from_valid_flow() {
        let (output, workspace_dir) = build_valid_project_runtime_output("project_runtime_output");

        assert_eq!(output.identity().project_id().as_str(), "alpha");
        assert_eq!(output.contract().manifest_id.as_str(), "manifest.alpha");
        assert_eq!(output.validation().status, ManifestValidationStatus::Valid);
        assert_eq!(output.surface().surface().as_str(), "viewer");
        assert_eq!(output.surface().entries().len(), 1);
        assert_eq!(output.resolved_viewer_targets().len(), 1);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn project_runtime_output_shape_is_intentionally_narrow() {
        let (output, workspace_dir) =
            build_valid_project_runtime_output("project_runtime_output_shape");

        // This exact destructuring is intentional: if the unified output gains or loses fields,
        // this test must be revisited as part of an explicit contract decision.
        let ProjectRuntimeOutput {
            identity,
            contract,
            validation,
            surface,
            resolved_viewer_targets,
        } = output;

        assert_eq!(identity.project_id().as_str(), "alpha");
        assert_eq!(contract.manifest_id.as_str(), "manifest.alpha");
        assert_eq!(validation.status, ManifestValidationStatus::Valid);
        assert_eq!(surface.surface().as_str(), "viewer");
        assert_eq!(resolved_viewer_targets.len(), 1);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn project_runtime_output_preserves_project_vertical_semantics() {
        let (output, workspace_dir) =
            build_valid_project_runtime_output("project_runtime_output_semantics");

        let identity = output.identity();
        let contract = output.contract();
        let surface = output.surface();
        let resolved_targets = output.resolved_viewer_targets();
        let normalized_project_root = normalize_absolute_path(identity.project_root().as_path());

        assert_eq!(identity.project_id(), &contract.project_id);
        assert_eq!(
            stable_relative_path(identity.manifest_source().path().as_path()),
            "user/projects/alpha/config/project_manifest.json"
        );
        assert_eq!(surface.entries().len(), contract.viewer_refs.len());
        assert_eq!(resolved_targets.len(), contract.viewer_refs.len());

        let viewer_ref = &contract.viewer_refs[0];
        let surface_entry = &surface.entries()[0];
        let resolved_target = &resolved_targets[0];

        assert_eq!(surface_entry.target_ref(), viewer_ref.target_ref());
        assert_eq!(surface_entry.target_kind(), viewer_ref.target_kind());
        assert_eq!(surface_entry.label(), viewer_ref.label());
        assert_eq!(
            stable_relative_path(
                surface_entry
                    .relative_path()
                    .expect("surface relative path")
                    .as_path()
            ),
            "docs/README.md"
        );

        assert_eq!(resolved_target.target_ref(), viewer_ref.target_ref());
        assert_eq!(resolved_target.target_kind(), viewer_ref.target_kind());
        assert_eq!(resolved_target.label(), viewer_ref.label());
        assert!(normalize_absolute_path(resolved_target.resolved_path())
            .starts_with(normalized_project_root.as_path()));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn project_runtime_output_excludes_decorative_or_cross_domain_state() {
        let (output, workspace_dir) =
            build_valid_project_runtime_output("project_runtime_output_boundaries");

        let ProjectRuntimeOutput {
            identity,
            contract,
            validation,
            surface,
            resolved_viewer_targets,
        } = output;

        assert_eq!(identity.project_id().as_str(), "alpha");
        assert_eq!(contract.project_id.as_str(), "alpha");
        assert_eq!(validation.issues.len(), 0);
        assert_eq!(surface.entries().len(), 1);
        assert_eq!(resolved_viewer_targets.len(), 1);

        // The output contract is intentionally limited to project-vertical results.
        // It must not grow into summaries, presentation payloads, or cross-domain aggregates
        // without an explicit contract decision that updates this test.

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn valid_minimal_exposure_writes_one_manifest_exposure_entry() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_exposure_valid");

        let result = expose_manifest_entry(
            &workspace_root,
            &project_root,
            Some(valid_exposure_request()),
            Some(valid_exposure_candidate()),
            Some(valid_human_confirmation()),
        )
        .expect("manifest exposure result");

        let entries = read_manifest_exposure_entries(&workspace_dir);
        assert_eq!(result.manifest_entry_count(), 1);
        assert_eq!(entries.len(), 1);
        assert_eq!(result.manifest_entry().exposure_state, "exposed_to_project");
        assert_eq!(entries[0].source_intake_item_ref, "intake_item.alpha");
        assert_eq!(
            result.manifest_path(),
            "user/projects/alpha/config/project_manifest.json"
        );

        assert!(
            !workspace_dir
                .join("user")
                .join("projects")
                .join("alpha")
                .join("registry.json")
                .exists()
        );
        assert!(
            !workspace_dir
                .join("user")
                .join("projects")
                .join("alpha")
                .join("graph")
                .exists()
        );
        assert!(
            !workspace_dir
                .join("user")
                .join("projects")
                .join("alpha")
                .join("knowledge")
                .join("derived")
                .exists()
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn manifest_with_one_exposure_entry_produces_one_tree_row() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_tree_one");
        write_manifest_exposure_entries(&workspace_dir, &[valid_manifest_exposure_entry()]);

        let rows =
            load_manifest_document_tree_rows(&workspace_root, &project_root).expect("tree rows");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].manifest_entry_id(), "manifest_entry.alpha");
        assert_eq!(rows[0].display_label(), "Alpha README");
        assert_eq!(rows[0].owner_ref(), "owner/demo");
        assert_eq!(rows[0].trace_ref(), "trace_demo");

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn manifest_with_multiple_exposure_entries_produces_multiple_tree_rows() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_tree_multiple");
        let mut first = valid_manifest_exposure_entry();
        let mut second = valid_manifest_exposure_entry();
        second.manifest_entry_id = "manifest_entry.beta".to_owned();
        second.exposed_object_ref = "exposed_object.beta".to_owned();
        second.exposure_request_ref = "exposure_request.beta".to_owned();
        second.exposure_candidate_ref = "exposure_candidate.beta".to_owned();
        second.confirmation_ref = "confirmation.beta".to_owned();
        second.source_intake_item_ref = "intake_item.beta".to_owned();
        second.display_label = "Beta README".to_owned();
        second.metadata_refs = vec!["metadata.beta".to_owned()];
        first.metadata_refs = vec!["metadata.alpha".to_owned()];
        write_manifest_exposure_entries(&workspace_dir, &[first, second]);

        let rows =
            load_manifest_document_tree_rows(&workspace_root, &project_root).expect("tree rows");

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].manifest_entry_id(), "manifest_entry.alpha");
        assert_eq!(rows[1].manifest_entry_id(), "manifest_entry.beta");

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn empty_manifest_exposure_entries_produces_empty_tree_rows() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_tree_empty");
        write_manifest_exposure_entries(&workspace_dir, &[]);

        let rows =
            load_manifest_document_tree_rows(&workspace_root, &project_root).expect("tree rows");

        assert!(rows.is_empty());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn imported_not_exposed_without_manifest_entry_does_not_appear_in_tree_rows() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_tree_missing");

        let rows =
            load_manifest_document_tree_rows(&workspace_root, &project_root).expect("tree rows");

        assert!(rows.is_empty());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn duplicate_file_ref_with_distinct_manifest_entry_id_produces_distinct_tree_rows() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_tree_duplicate_refs");
        let first = valid_manifest_exposure_entry();
        let mut second = valid_manifest_exposure_entry();
        second.manifest_entry_id = "manifest_entry.beta".to_owned();
        second.exposed_object_ref = "exposed_object.beta".to_owned();
        second.exposure_request_ref = "exposure_request.beta".to_owned();
        second.exposure_candidate_ref = "exposure_candidate.beta".to_owned();
        second.confirmation_ref = "confirmation.beta".to_owned();
        second.source_intake_item_ref = "intake_item.beta".to_owned();
        second.display_label = "Alpha README Copy".to_owned();
        write_manifest_exposure_entries(&workspace_dir, &[first, second]);

        let rows =
            load_manifest_document_tree_rows(&workspace_root, &project_root).expect("tree rows");

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].file_ref(), "file_ref.alpha");
        assert_eq!(rows[1].file_ref(), "file_ref.alpha");
        assert_ne!(rows[0].manifest_entry_id(), rows[1].manifest_entry_id());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn manifest_tree_rows_use_portable_refs_only() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_tree_portable");
        let mut entry = valid_manifest_exposure_entry();
        entry.display_label = "C:/Users/demo/private.txt".to_owned();
        write_manifest_exposure_entries(&workspace_dir, &[entry]);

        let error = load_manifest_document_tree_rows(&workspace_root, &project_root)
            .expect_err("raw host path error");

        assert!(matches!(error, ManifestExposureError::RawHostPath { .. }));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn blocked_item_cannot_expose() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_exposure_blocked");
        let mut candidate = valid_exposure_candidate();
        candidate.candidate_status = ExposureCandidateStatus::Blocked;
        candidate.eligibility_result = ExposureEligibilityResult::BlockedIntakeItem;

        let error = expose_manifest_entry(
            &workspace_root,
            &project_root,
            Some(valid_exposure_request()),
            Some(candidate),
            Some(valid_human_confirmation()),
        )
        .expect_err("blocked exposure error");

        assert_eq!(error, ManifestExposureError::BlockedIntakeItem);
        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn unsupported_item_cannot_expose() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_exposure_unsupported");
        let mut candidate = valid_exposure_candidate();
        candidate.candidate_status = ExposureCandidateStatus::Unsupported;
        candidate.eligibility_result = ExposureEligibilityResult::UnsupportedIntakeItem;

        let error = expose_manifest_entry(
            &workspace_root,
            &project_root,
            Some(valid_exposure_request()),
            Some(candidate),
            Some(valid_human_confirmation()),
        )
        .expect_err("unsupported exposure error");

        assert_eq!(error, ManifestExposureError::UnsupportedIntakeItem);
        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn rejected_confirmation_blocks() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_exposure_rejected");
        let mut confirmation = valid_human_confirmation();
        confirmation.decision = HumanConfirmationDecision::Rejected;

        let error = expose_manifest_entry(
            &workspace_root,
            &project_root,
            Some(valid_exposure_request()),
            Some(valid_exposure_candidate()),
            Some(confirmation),
        )
        .expect_err("rejected confirmation error");

        assert_eq!(error, ManifestExposureError::RejectedConfirmation);
        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn stale_candidate_blocks() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_exposure_stale");
        let mut confirmation = valid_human_confirmation();
        confirmation.stale_check_result = StaleCheckResult::Stale;

        let error = expose_manifest_entry(
            &workspace_root,
            &project_root,
            Some(valid_exposure_request()),
            Some(valid_exposure_candidate()),
            Some(confirmation),
        )
        .expect_err("stale candidate error");

        assert_eq!(error, ManifestExposureError::StaleCandidate);
        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn missing_owner_ref_blocks() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_exposure_missing_owner");
        let mut request = valid_exposure_request();
        request.owner_ref.clear();

        let error = expose_manifest_entry(
            &workspace_root,
            &project_root,
            Some(request),
            Some(valid_exposure_candidate()),
            Some(valid_human_confirmation()),
        )
        .expect_err("missing owner error");

        assert_eq!(error, ManifestExposureError::MissingOwnerRef);
        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn missing_trace_ref_blocks() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_exposure_missing_trace");
        let mut request = valid_exposure_request();
        request.trace_ref.clear();

        let error = expose_manifest_entry(
            &workspace_root,
            &project_root,
            Some(request),
            Some(valid_exposure_candidate()),
            Some(valid_human_confirmation()),
        )
        .expect_err("missing trace error");

        assert_eq!(error, ManifestExposureError::MissingTraceRef);
        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn duplicate_hash_does_not_collapse_logical_exposure_identity() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_exposure_duplicate_hash");

        expose_manifest_entry(
            &workspace_root,
            &project_root,
            Some(valid_exposure_request()),
            Some(valid_exposure_candidate()),
            Some(valid_human_confirmation()),
        )
        .expect("first exposure");

        let mut second_request = valid_exposure_request();
        second_request.exposure_request_id = "exposure_request.beta".to_owned();
        second_request.intake_item_ref = Some("intake_item.beta".to_owned());

        let mut second_candidate = valid_exposure_candidate();
        second_candidate.exposure_candidate_id = "exposure_candidate.beta".to_owned();
        second_candidate.exposure_request_ref = "exposure_request.beta".to_owned();
        second_candidate.source_intake_item_refs = vec!["intake_item.beta".to_owned()];
        second_candidate.stored_object_candidate_ref = "stored_object_candidate.beta".to_owned();
        second_candidate.sanitized_display_label = "Alpha README Duplicate".to_owned();

        let mut second_confirmation = valid_human_confirmation();
        second_confirmation.confirmation_id = "confirmation.beta".to_owned();
        second_confirmation.exposure_candidate_ref = "exposure_candidate.beta".to_owned();

        let second_result = expose_manifest_entry(
            &workspace_root,
            &project_root,
            Some(second_request),
            Some(second_candidate),
            Some(second_confirmation),
        )
        .expect("second exposure");

        let entries = read_manifest_exposure_entries(&workspace_dir);
        assert_eq!(entries.len(), 2);
        assert_eq!(second_result.manifest_entry_count(), 2);
        assert_eq!(entries[0].file_ref, entries[1].file_ref);
        assert_ne!(entries[0].manifest_entry_id, entries[1].manifest_entry_id);
        assert_ne!(entries[0].source_intake_item_ref, entries[1].source_intake_item_ref);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn manifest_entry_uses_portable_refs_only() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_exposure_portable_refs");

        let result = expose_manifest_entry(
            &workspace_root,
            &project_root,
            Some(valid_exposure_request()),
            Some(valid_exposure_candidate()),
            Some(valid_human_confirmation()),
        )
        .expect("portable result");

        let entry = result.manifest_entry();
        assert!(!entry.file_ref.contains(":\\"));
        assert!(!entry.source_intake_item_ref.contains(":\\"));
        assert!(!entry.display_label.contains(":\\"));
        assert_eq!(entry.exposure_state, "exposed_to_project");

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn raw_host_path_is_rejected() {
        let (workspace_root, project_root, workspace_dir) =
            build_manifest_exposure_workspace("manifest_exposure_raw_host_path");
        let mut candidate = valid_exposure_candidate();
        candidate.sanitized_display_label = r"C:\Users\Alice\secret.txt".to_owned();

        let error = expose_manifest_entry(
            &workspace_root,
            &project_root,
            Some(valid_exposure_request()),
            Some(candidate),
            Some(valid_human_confirmation()),
        )
        .expect_err("raw host path error");

        assert!(matches!(error, ManifestExposureError::RawHostPath { .. }));
        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn pipeline_observability_emits_stable_ordered_start_success_pairs() {
        let (result, workspace_dir) = run_valid_project_pipeline("project_runtime_order_contract");
        let expected_operations = [
            RuntimeOperation::OpenProject,
            RuntimeOperation::LoadManifest,
            RuntimeOperation::BuildManifestContract,
            RuntimeOperation::ValidateManifest,
            RuntimeOperation::BuildSurfaceModel,
            RuntimeOperation::ResolveViewerTargets,
        ];

        assert_eq!(result.events.len(), expected_operations.len() * 2);

        for (pair, expected_operation) in result.events.chunks_exact(2).zip(expected_operations) {
            assert_eq!(pair[0].operation(), expected_operation);
            assert_eq!(pair[0].status(), RuntimeStatus::Start);
            assert_eq!(pair[1].operation(), expected_operation);
            assert_eq!(pair[1].status(), RuntimeStatus::Success);
        }

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn pipeline_observability_covers_each_governed_step_exactly_once() {
        let (result, workspace_dir) = run_valid_project_pipeline("project_runtime_scope_contract");
        let expected_operations = [
            RuntimeOperation::OpenProject,
            RuntimeOperation::LoadManifest,
            RuntimeOperation::BuildManifestContract,
            RuntimeOperation::ValidateManifest,
            RuntimeOperation::BuildSurfaceModel,
            RuntimeOperation::ResolveViewerTargets,
        ];

        for expected_operation in expected_operations {
            let start_count = result
                .events
                .iter()
                .filter(|event| {
                    event.operation() == expected_operation
                        && event.status() == RuntimeStatus::Start
                })
                .count();
            let success_count = result
                .events
                .iter()
                .filter(|event| {
                    event.operation() == expected_operation
                        && event.status() == RuntimeStatus::Success
                })
                .count();
            let failure_count = result
                .events
                .iter()
                .filter(|event| {
                    event.operation() == expected_operation
                        && event.status() == RuntimeStatus::Failure
                })
                .count();

            assert_eq!(start_count, 1);
            assert_eq!(success_count, 1);
            assert_eq!(failure_count, 0);
        }

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn pipeline_failure_keeps_prior_trace_and_stops_after_contract_build() {
        let workspace_dir = unique_temp_dir("project_runtime_failure_boundary");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let config_dir = project_dir.join("config");
        fs::create_dir_all(&config_dir).expect("create config dir");
        fs::write(
            config_dir.join("project_manifest.json"),
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme"
}"#,
        )
        .expect("write invalid manifest");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let failure =
            run_project_pipeline(&workspace_root, project_root).expect_err("pipeline failure");

        let observed: Vec<(RuntimeOperation, RuntimeStatus)> = failure
            .events()
            .iter()
            .map(|event| (event.operation(), event.status()))
            .collect();

        assert_eq!(
            observed,
            vec![
                (RuntimeOperation::OpenProject, RuntimeStatus::Start),
                (RuntimeOperation::OpenProject, RuntimeStatus::Success),
                (RuntimeOperation::LoadManifest, RuntimeStatus::Start),
                (RuntimeOperation::LoadManifest, RuntimeStatus::Success),
                (
                    RuntimeOperation::BuildManifestContract,
                    RuntimeStatus::Start,
                ),
                (
                    RuntimeOperation::BuildManifestContract,
                    RuntimeStatus::Failure,
                ),
            ]
        );

        assert!(!failure.events().iter().any(|event| matches!(
            event.operation(),
            RuntimeOperation::ValidateManifest
                | RuntimeOperation::BuildSurfaceModel
                | RuntimeOperation::ResolveViewerTargets
        )));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn run_project_pipeline_emits_expected_observed_order() {
        let workspace_dir = unique_temp_dir("project_runtime_pipeline_order");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let config_dir = project_dir.join("config");
        fs::create_dir_all(&config_dir).expect("create config dir");
        fs::write(
            config_dir.join("project_manifest.json"),
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        )
        .expect("write manifest");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let result = run_project_pipeline(&workspace_root, project_root).expect("pipeline result");

        let operations: Vec<(RuntimeOperation, RuntimeStatus)> = result
            .events
            .iter()
            .map(|event| (event.operation(), event.status()))
            .collect();

        assert_eq!(
            operations,
            vec![
                (RuntimeOperation::OpenProject, RuntimeStatus::Start),
                (RuntimeOperation::OpenProject, RuntimeStatus::Success),
                (RuntimeOperation::LoadManifest, RuntimeStatus::Start),
                (RuntimeOperation::LoadManifest, RuntimeStatus::Success),
                (
                    RuntimeOperation::BuildManifestContract,
                    RuntimeStatus::Start
                ),
                (
                    RuntimeOperation::BuildManifestContract,
                    RuntimeStatus::Success
                ),
                (RuntimeOperation::ValidateManifest, RuntimeStatus::Start),
                (RuntimeOperation::ValidateManifest, RuntimeStatus::Success),
                (RuntimeOperation::BuildSurfaceModel, RuntimeStatus::Start),
                (RuntimeOperation::BuildSurfaceModel, RuntimeStatus::Success),
                (RuntimeOperation::ResolveViewerTargets, RuntimeStatus::Start),
                (
                    RuntimeOperation::ResolveViewerTargets,
                    RuntimeStatus::Success
                ),
            ]
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }
}
