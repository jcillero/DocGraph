//! Non-executing action-governance contracts for the Rust sandbox.
//!
//! These types are serialization-ready contracts only.
//! They do not execute, resolve, dispatch, authorize, persist, or mutate.

mod action_request;
mod authorized_execution_request;
mod human_confirmation;
mod pending_action_candidate;
mod resolution_candidate;
mod single_tool_execution;
mod tool_run_manifest;
mod trace_refs;

pub use action_request::{
    AccessLevel, ActionRequest, ActionRequestStatus, ActionSource, ActionTarget, DomainConstraints,
    ExpectedOutputDescriptor, HostAccess, InputDescriptor, PolicyContext, RequestKind,
    RequiredDomain, RiskAssessment, RiskLevel, SandboxScope, SanitizationState, SourceKind,
    TargetKind,
};
pub use authorized_execution_request::{
    AuthorizedExecutionRequest, AuthorizedExecutionStatus, ExecutionMode, ExecutionScope,
    OwnerRequirements, SafetySnapshot, StalenessResult, ToolKind,
};
pub use human_confirmation::{
    HumanConfirmation, HumanConfirmationDecision, ReviewScope, RiskAcknowledgement,
    StalenessCheck, StalenessCheckResult,
};
pub use pending_action_candidate::{
    CandidateToolSummary, CapabilitySummary, ConfirmationReadiness, DomainSummary,
    ExpectedOutputSummary, PendingActionCandidate, PendingActionCandidateStatus,
    PendingActionSummary, PolicySummary,
};
pub use resolution_candidate::{
    AvailabilityState, CandidateTool, CapabilityEvaluation, DomainEvaluation, DomainStatus,
    PolicyEvaluation, PolicyStatus, RequirementStatus, ResolutionCandidate,
    ResolutionCandidateStatus, ResolutionState, StalenessState,
};
pub use single_tool_execution::{
    OutputPlan, OutputPolicy, ResultPlaceholder, ResultStatus, SingleToolExecution,
    SingleToolExecutionScope, SingleToolExecutionStatus, ToolBinding, ToolDeterminism,
};
pub use tool_run_manifest::{
    ManifestOutput, ToolRunManifest, ToolRunManifestStatus, ToolRunManifestTool,
};
pub use trace_refs::{TraceEventKind, TraceRecord, TraceRecordStatus, TraceScope};

use serde::{Deserialize, Serialize};

/// Readonly status for an inert action request draft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionRequestDraftStatus {
    Draft,
    Blocked,
}

/// Minimal kind classification for future governed action request drafts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionRequestDraftKind {
    ToolUse,
    DocumentMutation,
    ConfigurationChange,
    Other,
}

/// Non-executing future action request draft model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionRequestDraft {
    pub action_id: String,
    pub kind: ActionRequestDraftKind,
    pub status: ActionRequestDraftStatus,
    pub origin_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<String>,
    pub rationale: String,
    pub reason_codes: Vec<String>,
}

/// Build an inert draft action request without validation, approval, or execution.
pub fn create_action_request_draft(
    action_id: String,
    kind: ActionRequestDraftKind,
    origin_ref: String,
    target_ref: Option<String>,
    rationale: String,
    reason_codes: Vec<String>,
) -> ActionRequestDraft {
    ActionRequestDraft {
        action_id,
        kind,
        status: ActionRequestDraftStatus::Draft,
        origin_ref,
        target_ref,
        rationale,
        reason_codes,
    }
}

/// Build an inert blocked action request without resolution or execution.
pub fn create_blocked_action_request_draft(
    action_id: String,
    kind: ActionRequestDraftKind,
    origin_ref: String,
    target_ref: Option<String>,
    rationale: String,
    reason_codes: Vec<String>,
) -> ActionRequestDraft {
    ActionRequestDraft {
        action_id,
        kind,
        status: ActionRequestDraftStatus::Blocked,
        origin_ref,
        target_ref,
        rationale,
        reason_codes,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::{json, Value};

    use super::*;

    #[test]
    fn action_request_roundtrip_json() {
        let request = ActionRequest {
            action_request_id: "ar_001".to_owned(),
            intent_ref: "intent_001".to_owned(),
            proposal_ref: Some("proposal_001".to_owned()),
            draft_ref: Some("draft_001".to_owned()),
            source: ActionSource {
                source_kind: SourceKind::User,
                source_ref: "user_001".to_owned(),
            },
            request_kind: RequestKind::ToolUse,
            target: ActionTarget {
                target_kind: TargetKind::Project,
                target_ref: "project_alpha".to_owned(),
            },
            capability_requirements: vec!["read_file".to_owned(), "write_file".to_owned()],
            domain_constraints: DomainConstraints {
                required_domain: RequiredDomain::Sandbox,
                access_level: AccessLevel::Write,
                sandbox_scope: SandboxScope::ProjectOnly,
                host_access: HostAccess::Forbidden,
                network_access: false,
            },
            policy_context: PolicyContext {
                security_policy_ref: Some("policy:security".to_owned()),
                sandbox_policy_ref: Some("policy:sandbox".to_owned()),
                tool_policy_ref: Some("policy:tools".to_owned()),
                llm_policy_ref: None,
                project_policy_ref: Some("policy:project".to_owned()),
            },
            inputs: vec![InputDescriptor {
                input_ref: "input_001".to_owned(),
                input_kind: "stored_object".to_owned(),
                source_ref: "obj_001".to_owned(),
                required: true,
                order_index: 0,
                sanitization_state: SanitizationState::Sanitized,
            }],
            expected_outputs: vec![ExpectedOutputDescriptor {
                output_kind: "report".to_owned(),
                owner_ref_required: true,
                expected_location_policy: "OWNER_SCOPED".to_owned(),
                manifest_required: true,
                trace_required: true,
            }],
            risk: RiskAssessment {
                risk_level: RiskLevel::Medium,
                risk_reasons: vec!["human_review_required".to_owned()],
            },
            status: ActionRequestStatus::Drafted,
            blocking_reasons: vec!["missing_authorization".to_owned()],
            trace_ref: "trace_001".to_owned(),
            created_at: "2026-05-01T10:00:00Z".to_owned(),
        };

        let serialized = serde_json::to_value(&request).expect("serialize action request");
        assert_eq!(serialized["status"], Value::String("DRAFTED".to_owned()));
        assert_eq!(serialized["request_kind"], Value::String("tool_use".to_owned()));

        let roundtrip: ActionRequest =
            serde_json::from_value(serialized).expect("deserialize action request");
        assert_eq!(roundtrip, request);
    }

    #[test]
    fn resolution_candidate_roundtrip_json() {
        let candidate = ResolutionCandidate {
            resolution_candidate_id: "rc_001".to_owned(),
            action_request_ref: "ar_001".to_owned(),
            intent_ref: "intent_001".to_owned(),
            trace_ref: "trace_001".to_owned(),
            status: ResolutionCandidateStatus::Candidate,
            resolution_state: ResolutionState::Resolvable,
            capability_evaluation: vec![CapabilityEvaluation {
                capability: "read_file".to_owned(),
                requirement_status: RequirementStatus::Satisfied,
                reason_code: "capability_declared".to_owned(),
                notes_key: "system.capability.ok".to_owned(),
            }],
            domain_evaluation: vec![DomainEvaluation {
                required_domain: RequiredDomain::Sandbox,
                access_level: AccessLevel::Read,
                domain_status: DomainStatus::AllowedByContract,
                reason_code: "sandbox_only".to_owned(),
            }],
            policy_evaluation: vec![PolicyEvaluation {
                policy_ref: "policy:security".to_owned(),
                policy_status: PolicyStatus::Satisfied,
                reason_code: "sanitized".to_owned(),
            }],
            candidate_tools: vec![CandidateTool {
                tool_id: "text_measure".to_owned(),
                tool_kind: "operational".to_owned(),
                capability_match: vec!["read_file".to_owned()],
                availability_state: AvailabilityState::Implemented,
                reason_code: "candidate_available".to_owned(),
            }],
            blocking_reasons: Vec::new(),
            risk: RiskAssessment {
                risk_level: RiskLevel::Low,
                risk_reasons: vec!["local_deterministic".to_owned()],
            },
            staleness: StalenessState {
                is_stale: false,
                stale_reasons: Vec::new(),
            },
            created_at: "2026-05-01T10:05:00Z".to_owned(),
        };

        let serialized =
            serde_json::to_value(&candidate).expect("serialize resolution candidate");
        assert_eq!(
            serialized["resolution_state"],
            Value::String("RESOLVABLE".to_owned())
        );

        let roundtrip: ResolutionCandidate =
            serde_json::from_value(serialized).expect("deserialize resolution candidate");
        assert_eq!(roundtrip, candidate);
    }

    #[test]
    fn pending_action_candidate_roundtrip_json() {
        let candidate = PendingActionCandidate {
            pending_action_candidate_id: "pac_001".to_owned(),
            resolution_candidate_ref: "rc_001".to_owned(),
            action_request_ref: "ar_001".to_owned(),
            intent_ref: "intent_001".to_owned(),
            trace_ref: "trace_001".to_owned(),
            status: PendingActionCandidateStatus::PendingReview,
            confirmation_readiness: ConfirmationReadiness::Ready,
            summary: PendingActionSummary {
                summary_key: "pending.summary".to_owned(),
                technical_summary_key: "pending.summary.technical".to_owned(),
            },
            capability_summary: vec![CapabilitySummary {
                capability: "read_file".to_owned(),
                status: RequirementStatus::Satisfied,
                message_key: "capability.satisfied".to_owned(),
            }],
            domain_summary: vec![DomainSummary {
                required_domain: RequiredDomain::Sandbox,
                access_level: AccessLevel::Read,
                status: DomainStatus::AllowedByContract,
                message_key: "domain.allowed".to_owned(),
            }],
            policy_summary: vec![PolicySummary {
                policy_ref: "policy:security".to_owned(),
                status: PolicyStatus::Satisfied,
                message_key: "policy.satisfied".to_owned(),
            }],
            candidate_tool_summary: vec![CandidateToolSummary {
                tool_id: "text_measure".to_owned(),
                tool_kind: "operational".to_owned(),
                availability_state: AvailabilityState::Implemented,
                message_key: "tool.implemented".to_owned(),
            }],
            expected_outputs_summary: vec![ExpectedOutputSummary {
                output_kind: "report".to_owned(),
                owner_ref_required: true,
                manifest_required: true,
                trace_required: true,
                message_key: "output.owner_scoped".to_owned(),
            }],
            risk: RiskAssessment {
                risk_level: RiskLevel::Low,
                risk_reasons: vec!["reviewable".to_owned()],
            },
            blocking_reasons: Vec::new(),
            staleness: StalenessState {
                is_stale: false,
                stale_reasons: Vec::new(),
            },
            created_at: "2026-05-01T10:10:00Z".to_owned(),
        };

        let serialized =
            serde_json::to_value(&candidate).expect("serialize pending action candidate");
        assert_eq!(
            serialized["confirmation_readiness"],
            Value::String("READY".to_owned())
        );

        let roundtrip: PendingActionCandidate =
            serde_json::from_value(serialized).expect("deserialize pending action candidate");
        assert_eq!(roundtrip, candidate);
    }

    #[test]
    fn human_confirmation_roundtrip_json() {
        let confirmation = HumanConfirmation {
            human_confirmation_id: "hc_001".to_owned(),
            pending_action_candidate_ref: "pac_001".to_owned(),
            resolution_candidate_ref: "rc_001".to_owned(),
            action_request_ref: "ar_001".to_owned(),
            intent_ref: "intent_001".to_owned(),
            trace_ref: "trace_001".to_owned(),
            reviewer_ref: "user_001".to_owned(),
            decision: HumanConfirmationDecision::Accepted,
            decision_reason_code: "review_accepted".to_owned(),
            review_scope: ReviewScope::TechnicalDetails,
            staleness_check: StalenessCheck {
                performed: true,
                result: StalenessCheckResult::Fresh,
                checked_at: "2026-05-01T10:12:00Z".to_owned(),
            },
            risk_acknowledgement: RiskAcknowledgement {
                required: false,
                acknowledged: false,
            },
            created_at: "2026-05-01T10:13:00Z".to_owned(),
        };

        let serialized = serde_json::to_value(&confirmation).expect("serialize confirmation");
        assert_eq!(serialized["decision"], Value::String("ACCEPTED".to_owned()));

        let roundtrip: HumanConfirmation =
            serde_json::from_value(serialized).expect("deserialize confirmation");
        assert_eq!(roundtrip, confirmation);
    }

    #[test]
    fn authorized_execution_request_roundtrip_json() {
        let request = AuthorizedExecutionRequest {
            authorized_execution_request_id: "aer_001".to_owned(),
            human_confirmation_ref: "hc_001".to_owned(),
            pending_action_candidate_ref: "pac_001".to_owned(),
            resolution_candidate_ref: "rc_001".to_owned(),
            action_request_ref: "ar_001".to_owned(),
            intent_ref: "intent_001".to_owned(),
            trace_ref: "trace_001".to_owned(),
            authorization_status: AuthorizedExecutionStatus::AuthorizedPrepared,
            execution_scope: ExecutionScope {
                execution_mode: ExecutionMode::SingleToolLocalDeterministic,
                tool_id: "text_measure".to_owned(),
                tool_kind: ToolKind::Operational,
                required_capabilities: vec!["read_file".to_owned()],
                required_domain: RequiredDomain::Sandbox,
            },
            safety_snapshot: SafetySnapshot {
                staleness_result: StalenessResult::Fresh,
                risk_level: RiskLevel::Low,
                security_checked: true,
                sandbox_checked: true,
                capability_checked: true,
                policy_checked: true,
            },
            owner_requirements: OwnerRequirements {
                owner_ref_required: true,
                owner_ref: Some("chat_001".to_owned()),
                tool_run_manifest_required: true,
                trace_required: true,
            },
            blocking_reasons: Vec::new(),
            created_at: "2026-05-01T10:15:00Z".to_owned(),
        };

        let serialized =
            serde_json::to_value(&request).expect("serialize authorized execution request");
        assert_eq!(
            serialized["authorization_status"],
            Value::String("AUTHORIZED_PREPARED".to_owned())
        );

        let roundtrip: AuthorizedExecutionRequest =
            serde_json::from_value(serialized).expect("deserialize authorized execution request");
        assert_eq!(roundtrip, request);
    }

    #[test]
    fn single_tool_execution_roundtrip_json() {
        let execution = SingleToolExecution {
            single_tool_execution_id: "ste_001".to_owned(),
            authorized_execution_request_ref: "aer_001".to_owned(),
            human_confirmation_ref: "hc_001".to_owned(),
            pending_action_candidate_ref: "pac_001".to_owned(),
            resolution_candidate_ref: "rc_001".to_owned(),
            action_request_ref: "ar_001".to_owned(),
            intent_ref: "intent_001".to_owned(),
            trace_ref: "trace_001".to_owned(),
            execution_status: SingleToolExecutionStatus::DeclaredOnly,
            tool_binding: ToolBinding {
                tool_id: "text_measure".to_owned(),
                tool_kind: ToolKind::Operational,
                capabilities: vec!["read_file".to_owned()],
                implementation_ref: "tool:text_measure:v1".to_owned(),
                determinism: ToolDeterminism::Deterministic,
            },
            execution_scope: SingleToolExecutionScope {
                execution_mode: ExecutionMode::SingleToolLocalDeterministic,
                required_domain: RequiredDomain::Sandbox,
                network_access: false,
                host_write: false,
                external_binary: false,
                provider_invocation: false,
            },
            input_refs: vec![InputDescriptor {
                input_ref: "input_001".to_owned(),
                input_kind: "stored_object".to_owned(),
                source_ref: "obj_001".to_owned(),
                required: true,
                order_index: 0,
                sanitization_state: SanitizationState::Sanitized,
            }],
            output_plan: OutputPlan {
                owner_ref: "chat_001".to_owned(),
                output_policy: OutputPolicy::OwnerScoped,
                expected_output_refs: vec!["output_001".to_owned()],
                tool_run_manifest_required: true,
                trace_required: true,
            },
            safety_snapshot: SafetySnapshot {
                staleness_result: StalenessResult::Fresh,
                risk_level: RiskLevel::Low,
                security_checked: true,
                sandbox_checked: true,
                capability_checked: true,
                policy_checked: true,
            },
            result_placeholder: ResultPlaceholder {
                result_status: ResultStatus::NotRun,
                manifest_ref: None,
                trace_record_ref: None,
            },
            created_at: "2026-05-01T10:20:00Z".to_owned(),
        };

        let serialized =
            serde_json::to_value(&execution).expect("serialize single tool execution");
        assert_eq!(
            serialized["execution_status"],
            Value::String("DECLARED_ONLY".to_owned())
        );

        let roundtrip: SingleToolExecution =
            serde_json::from_value(serialized).expect("deserialize single tool execution");
        assert_eq!(roundtrip, execution);
    }

    #[test]
    fn tool_run_manifest_roundtrip_json() {
        let mut configuration = BTreeMap::new();
        configuration.insert("mode".to_owned(), json!("strict"));

        let manifest = ToolRunManifest {
            tool_run_manifest_id: "trm_001".to_owned(),
            single_tool_execution_ref: "ste_001".to_owned(),
            authorized_execution_request_ref: "aer_001".to_owned(),
            human_confirmation_ref: "hc_001".to_owned(),
            action_request_ref: "ar_001".to_owned(),
            intent_ref: "intent_001".to_owned(),
            trace_ref: "trace_001".to_owned(),
            owner_ref: "chat_001".to_owned(),
            tool: ToolRunManifestTool {
                tool_id: "text_measure".to_owned(),
                tool_kind: ToolKind::Operational,
                tool_version: "1.0.0".to_owned(),
                implementation_ref: "tool:text_measure:v1".to_owned(),
            },
            inputs: vec![InputDescriptor {
                input_ref: "input_001".to_owned(),
                input_kind: "stored_object".to_owned(),
                source_ref: "obj_001".to_owned(),
                required: true,
                order_index: 0,
                sanitization_state: SanitizationState::Sanitized,
            }],
            configuration,
            outputs: vec![ManifestOutput {
                output_ref: "output_001".to_owned(),
                output_kind: "report".to_owned(),
                owner_ref: "chat_001".to_owned(),
            }],
            status: ToolRunManifestStatus::FutureRequired,
            started_at: None,
            finished_at: None,
            error: None,
            warnings: vec!["manifest_pending".to_owned()],
            created_at: "2026-05-01T10:25:00Z".to_owned(),
        };

        let serialized = serde_json::to_value(&manifest).expect("serialize manifest");
        assert_eq!(
            serialized["status"],
            Value::String("FUTURE_REQUIRED".to_owned())
        );

        let roundtrip: ToolRunManifest =
            serde_json::from_value(serialized).expect("deserialize manifest");
        assert_eq!(roundtrip, manifest);
    }

    #[test]
    fn trace_record_roundtrip_json() {
        let record = TraceRecord {
            trace_record_id: "trrec_001".to_owned(),
            trace_ref: "trace_001".to_owned(),
            single_tool_execution_ref: "ste_001".to_owned(),
            authorized_execution_request_ref: "aer_001".to_owned(),
            action_request_ref: "ar_001".to_owned(),
            event_kind: TraceEventKind::SingleToolExecutionDeclared,
            scope: TraceScope::SingleIntent,
            status: TraceRecordStatus::DeclaredOnly,
            links: vec!["aer_001".to_owned(), "ste_001".to_owned()],
            sanitized_summary_key: "trace.execution.declared".to_owned(),
            created_at: "2026-05-01T10:30:00Z".to_owned(),
        };

        let serialized = serde_json::to_value(&record).expect("serialize trace record");
        assert_eq!(
            serialized["status"],
            Value::String("DECLARED_ONLY".to_owned())
        );
        assert_eq!(
            serialized["event_kind"],
            Value::String("SINGLE_TOOL_EXECUTION_DECLARED".to_owned())
        );

        let roundtrip: TraceRecord =
            serde_json::from_value(serialized).expect("deserialize trace record");
        assert_eq!(roundtrip, record);
    }

    #[test]
    fn disallowed_runtime_statuses_fail_to_deserialize() {
        assert!(serde_json::from_str::<ActionRequestStatus>("\"AUTHORIZED\"").is_err());
        assert!(serde_json::from_str::<ActionRequestStatus>("\"EXECUTING\"").is_err());
        assert!(serde_json::from_str::<ResolutionCandidateStatus>("\"RUNNING\"").is_err());
        assert!(serde_json::from_str::<PendingActionCandidateStatus>("\"READY\"").is_err());
        assert!(serde_json::from_str::<ConfirmationReadiness>("\"EXECUTED\"").is_err());
        assert!(serde_json::from_str::<SingleToolExecutionStatus>("\"RUNNING\"").is_err());
        assert!(serde_json::from_str::<ToolRunManifestStatus>("\"FAILED\"").is_err());
        assert!(serde_json::from_str::<TraceRecordStatus>("\"EXECUTED\"").is_err());
        assert!(serde_json::from_str::<AuthorizedExecutionStatus>("\"DISPATCHED\"").is_err());
    }

    #[test]
    fn draft_action_request_constructors_remain_inert() {
        let draft = create_action_request_draft(
            "action-001".to_owned(),
            ActionRequestDraftKind::ToolUse,
            "llm:chat".to_owned(),
            Some("tool:text.measure".to_owned()),
            "This request is still only a draft.".to_owned(),
            vec!["tool_may_be_useful".to_owned()],
        );

        let blocked = create_blocked_action_request_draft(
            "action-002".to_owned(),
            ActionRequestDraftKind::ConfigurationChange,
            "system".to_owned(),
            None,
            "Blocked request.".to_owned(),
            vec!["execution_not_open".to_owned()],
        );

        assert_eq!(draft.status, ActionRequestDraftStatus::Draft);
        assert_eq!(blocked.status, ActionRequestDraftStatus::Blocked);
    }
}
