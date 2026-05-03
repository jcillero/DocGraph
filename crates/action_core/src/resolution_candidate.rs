use serde::{Deserialize, Serialize};

use crate::action_request::{AccessLevel, RequiredDomain, RiskAssessment};

/// Inert resolution-candidate status only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResolutionCandidateStatus {
    Candidate,
    Blocked,
    Superseded,
    Expired,
}

/// Inert resolution-state only. It does not authorize or execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResolutionState {
    Resolvable,
    Blocked,
    Ambiguous,
    Unsupported,
    Unsafe,
    Stale,
}

/// Requirement evaluation only. It does not grant permissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirementStatus {
    Satisfied,
    Missing,
    Disabled,
    FutureOnly,
    Unknown,
}

/// Domain evaluation only. It does not authorize access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainStatus {
    AllowedByContract,
    Forbidden,
    FutureGated,
    Unspecified,
    Unknown,
}

/// Policy evaluation only. It does not enforce runtime behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyStatus {
    Satisfied,
    Blocked,
    Warning,
    Unknown,
}

/// Candidate tool availability only. It does not select or execute a tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityState {
    Declared,
    Visible,
    Allowed,
    Implemented,
    Unavailable,
    Disabled,
    FutureOnly,
    NonExecutable,
}

/// Capability evaluation metadata only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityEvaluation {
    pub capability: String,
    pub requirement_status: RequirementStatus,
    pub reason_code: String,
    pub notes_key: String,
}

/// Domain evaluation metadata only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainEvaluation {
    pub required_domain: RequiredDomain,
    pub access_level: AccessLevel,
    pub domain_status: DomainStatus,
    pub reason_code: String,
}

/// Policy evaluation metadata only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyEvaluation {
    pub policy_ref: String,
    pub policy_status: PolicyStatus,
    pub reason_code: String,
}

/// Candidate tool metadata only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateTool {
    pub tool_id: String,
    pub tool_kind: String,
    pub capability_match: Vec<String>,
    pub availability_state: AvailabilityState,
    pub reason_code: String,
}

/// Staleness metadata only. It does not trigger invalidation logic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StalenessState {
    pub is_stale: bool,
    pub stale_reasons: Vec<String>,
}

/// Rich inert ResolutionCandidate contract only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionCandidate {
    pub resolution_candidate_id: String,
    pub action_request_ref: String,
    pub intent_ref: String,
    pub trace_ref: String,
    pub status: ResolutionCandidateStatus,
    pub resolution_state: ResolutionState,
    pub capability_evaluation: Vec<CapabilityEvaluation>,
    pub domain_evaluation: Vec<DomainEvaluation>,
    pub policy_evaluation: Vec<PolicyEvaluation>,
    pub candidate_tools: Vec<CandidateTool>,
    pub blocking_reasons: Vec<String>,
    pub risk: RiskAssessment,
    pub staleness: StalenessState,
    pub created_at: String,
}
