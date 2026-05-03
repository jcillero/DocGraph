use serde::{Deserialize, Serialize};

use crate::action_request::{AccessLevel, RequiredDomain, RiskAssessment};
use crate::resolution_candidate::{AvailabilityState, DomainStatus, PolicyStatus, RequirementStatus, StalenessState};

/// PendingActionCandidate status only. It does not confirm or execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PendingActionCandidateStatus {
    PendingReview,
    NotReady,
    Blocked,
    Stale,
    Superseded,
    Expired,
}

/// Confirmation readiness only. It does not authorize or execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConfirmationReadiness {
    Ready,
    NotReady,
    Blocked,
    Stale,
    Unsafe,
}

/// Summary text keys only. It does not contain raw payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingActionSummary {
    pub summary_key: String,
    pub technical_summary_key: String,
}

/// Presentation-ready capability summary only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilitySummary {
    pub capability: String,
    pub status: RequirementStatus,
    pub message_key: String,
}

/// Presentation-ready domain summary only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainSummary {
    pub required_domain: RequiredDomain,
    pub access_level: AccessLevel,
    pub status: DomainStatus,
    pub message_key: String,
}

/// Presentation-ready policy summary only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySummary {
    pub policy_ref: String,
    pub status: PolicyStatus,
    pub message_key: String,
}

/// Presentation-ready candidate tool summary only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateToolSummary {
    pub tool_id: String,
    pub tool_kind: String,
    pub availability_state: AvailabilityState,
    pub message_key: String,
}

/// Presentation-ready expected-output summary only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedOutputSummary {
    pub output_kind: String,
    pub owner_ref_required: bool,
    pub manifest_required: bool,
    pub trace_required: bool,
    pub message_key: String,
}

/// Rich non-executing PendingActionCandidate contract only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingActionCandidate {
    pub pending_action_candidate_id: String,
    pub resolution_candidate_ref: String,
    pub action_request_ref: String,
    pub intent_ref: String,
    pub trace_ref: String,
    pub status: PendingActionCandidateStatus,
    pub confirmation_readiness: ConfirmationReadiness,
    pub summary: PendingActionSummary,
    pub capability_summary: Vec<CapabilitySummary>,
    pub domain_summary: Vec<DomainSummary>,
    pub policy_summary: Vec<PolicySummary>,
    pub candidate_tool_summary: Vec<CandidateToolSummary>,
    pub expected_outputs_summary: Vec<ExpectedOutputSummary>,
    pub risk: RiskAssessment,
    pub blocking_reasons: Vec<String>,
    pub staleness: StalenessState,
    pub created_at: String,
}
