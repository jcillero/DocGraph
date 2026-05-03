use serde::{Deserialize, Serialize};

/// Origin classification for action-governance contracts only.
/// It does not authorize execution or provider calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    User,
    Llm,
    System,
}

/// Traceable source reference for non-executing action contracts only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionSource {
    pub source_kind: SourceKind,
    pub source_ref: String,
}

/// Target classification for non-executing action contracts only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKind {
    StoredObject,
    FileRef,
    Artifact,
    Chat,
    ToolOutput,
    Project,
    SemanticQuadset,
    Unknown,
}

/// Action target reference only. It does not imply authority or execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionTarget {
    pub target_kind: TargetKind,
    pub target_ref: String,
}

/// Controlled action-request status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionRequestStatus {
    Drafted,
    NeedsContext,
    Blocked,
    ReadyForResolution,
    Superseded,
    Expired,
}

/// Controlled action-request kind values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestKind {
    ToolUse,
    DocumentOperation,
    SemanticOperation,
    FilesystemOperation,
    ReviewOperation,
    Unknown,
}

/// Governed domain declaration only. It does not authorize access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequiredDomain {
    Sandbox,
    Host,
    External,
    Unspecified,
}

/// Declared access level only. It does not grant permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessLevel {
    Read,
    Write,
    Create,
    Delete,
    Network,
    None,
}

/// Sandbox scope declaration only. It does not mutate any filesystem scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxScope {
    ProjectOnly,
    FileStore,
    UserOutput,
    Unspecified,
}

/// Host access declaration only. It does not authorize host access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostAccess {
    None,
    ReadOnly,
    Forbidden,
}

/// Sanitization state metadata only. It does not perform sanitization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SanitizationState {
    Sanitized,
    Pending,
    Blocked,
    Unknown,
}

/// Action-request domain constraints only. They do not authorize access.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainConstraints {
    pub required_domain: RequiredDomain,
    pub access_level: AccessLevel,
    pub sandbox_scope: SandboxScope,
    pub host_access: HostAccess,
    pub network_access: bool,
}

/// Policy references only. They do not resolve or enforce policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PolicyContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_policy_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_policy_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_policy_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_policy_ref: Option<String>,
}

/// Reference-only input descriptor. It does not carry raw payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputDescriptor {
    pub input_ref: String,
    pub input_kind: String,
    pub source_ref: String,
    pub required: bool,
    pub order_index: u32,
    pub sanitization_state: SanitizationState,
}

/// Declarative expected-output descriptor only. It does not create files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedOutputDescriptor {
    pub output_kind: String,
    pub owner_ref_required: bool,
    pub expected_location_policy: String,
    pub manifest_required: bool,
    pub trace_required: bool,
}

/// Risk metadata only. It does not schedule, authorize, or execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Unknown,
}

/// Risk preparation metadata only. It does not trigger runtime work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub risk_level: RiskLevel,
    pub risk_reasons: Vec<String>,
}

/// Rich non-executing ActionRequest contract only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionRequest {
    pub action_request_id: String,
    pub intent_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_ref: Option<String>,
    pub source: ActionSource,
    pub request_kind: RequestKind,
    pub target: ActionTarget,
    pub capability_requirements: Vec<String>,
    pub domain_constraints: DomainConstraints,
    pub policy_context: PolicyContext,
    pub inputs: Vec<InputDescriptor>,
    pub expected_outputs: Vec<ExpectedOutputDescriptor>,
    pub risk: RiskAssessment,
    pub status: ActionRequestStatus,
    pub blocking_reasons: Vec<String>,
    pub trace_ref: String,
    pub created_at: String,
}
