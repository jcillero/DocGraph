use serde::{Deserialize, Serialize};

use crate::action_request::RequiredDomain;
use crate::action_request::RiskLevel;

/// AuthorizedExecutionRequest status only. It does not dispatch or execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorizedExecutionStatus {
    AuthorizedPrepared,
    NotAuthorized,
    Blocked,
    Stale,
    Superseded,
    Expired,
}

/// First future execution mode only. It does not activate runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionMode {
    SingleToolLocalDeterministic,
}

/// First future executable tool kinds only. It does not widen execution scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolKind {
    Operational,
    Base,
}

/// Staleness result snapshot only. It does not perform checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StalenessResult {
    Fresh,
    Stale,
    Unknown,
}

/// Execution-scope declaration only. It does not authorize execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionScope {
    pub execution_mode: ExecutionMode,
    pub tool_id: String,
    pub tool_kind: ToolKind,
    pub required_capabilities: Vec<String>,
    pub required_domain: RequiredDomain,
}

/// Safety metadata only. It does not perform checks or authorize execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafetySnapshot {
    pub staleness_result: StalenessResult,
    pub risk_level: RiskLevel,
    pub security_checked: bool,
    pub sandbox_checked: bool,
    pub capability_checked: bool,
    pub policy_checked: bool,
}

/// Future owner and manifest requirements only. It does not create outputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerRequirements {
    pub owner_ref_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_ref: Option<String>,
    pub tool_run_manifest_required: bool,
    pub trace_required: bool,
}

/// Post-confirmation authorization artifact only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizedExecutionRequest {
    pub authorized_execution_request_id: String,
    pub human_confirmation_ref: String,
    pub pending_action_candidate_ref: String,
    pub resolution_candidate_ref: String,
    pub action_request_ref: String,
    pub intent_ref: String,
    pub trace_ref: String,
    pub authorization_status: AuthorizedExecutionStatus,
    pub execution_scope: ExecutionScope,
    pub safety_snapshot: SafetySnapshot,
    pub owner_requirements: OwnerRequirements,
    pub blocking_reasons: Vec<String>,
    pub created_at: String,
}
