use serde::{Deserialize, Serialize};

/// HumanConfirmation decision only. It does not authorize execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HumanConfirmationDecision {
    Accepted,
    Rejected,
    Deferred,
    ChangesRequested,
}

/// Review-scope description only. It does not grant authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewScope {
    SummaryOnly,
    TechnicalDetails,
    FullTrace,
}

/// Staleness-check result only. It does not perform checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StalenessCheckResult {
    Fresh,
    Stale,
    Unknown,
}

/// Staleness-check metadata only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StalenessCheck {
    pub performed: bool,
    pub result: StalenessCheckResult,
    pub checked_at: String,
}

/// Risk acknowledgement metadata only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskAcknowledgement {
    pub required: bool,
    pub acknowledged: bool,
}

/// Explicit human decision event only. It does not execute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HumanConfirmation {
    pub human_confirmation_id: String,
    pub pending_action_candidate_ref: String,
    pub resolution_candidate_ref: String,
    pub action_request_ref: String,
    pub intent_ref: String,
    pub trace_ref: String,
    pub reviewer_ref: String,
    pub decision: HumanConfirmationDecision,
    pub decision_reason_code: String,
    pub review_scope: ReviewScope,
    pub staleness_check: StalenessCheck,
    pub risk_acknowledgement: RiskAcknowledgement,
    pub created_at: String,
}
