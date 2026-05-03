use serde::{Deserialize, Serialize};

use crate::action_request::{InputDescriptor, RequiredDomain};
use crate::authorized_execution_request::{ExecutionMode, SafetySnapshot, ToolKind};

/// SingleToolExecution status only. It does not run anything in F11.6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SingleToolExecutionStatus {
    DeclaredOnly,
    NotRun,
    Blocked,
    Stale,
    Superseded,
    Expired,
}

/// Determinism declaration only. It does not execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ToolDeterminism {
    Deterministic,
}

/// Output policy only. It does not create files or folders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OutputPolicy {
    OwnerScoped,
}

/// Result placeholder status only. It does not reflect real execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResultStatus {
    NotRun,
    Blocked,
    Stale,
}

/// Tool binding declaration only. It does not invoke tools.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolBinding {
    pub tool_id: String,
    pub tool_kind: ToolKind,
    pub capabilities: Vec<String>,
    pub implementation_ref: String,
    pub determinism: ToolDeterminism,
}

/// First future execution scope only. It does not open runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SingleToolExecutionScope {
    pub execution_mode: ExecutionMode,
    pub required_domain: RequiredDomain,
    pub network_access: bool,
    pub host_write: bool,
    pub external_binary: bool,
    pub provider_invocation: bool,
}

/// Declarative output plan only. It does not materialize outputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputPlan {
    pub owner_ref: String,
    pub output_policy: OutputPolicy,
    pub expected_output_refs: Vec<String>,
    pub tool_run_manifest_required: bool,
    pub trace_required: bool,
}

/// Result placeholder only. It does not contain a real result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultPlaceholder {
    pub result_status: ResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_record_ref: Option<String>,
}

/// Declared-only SingleToolExecution contract only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SingleToolExecution {
    pub single_tool_execution_id: String,
    pub authorized_execution_request_ref: String,
    pub human_confirmation_ref: String,
    pub pending_action_candidate_ref: String,
    pub resolution_candidate_ref: String,
    pub action_request_ref: String,
    pub intent_ref: String,
    pub trace_ref: String,
    pub execution_status: SingleToolExecutionStatus,
    pub tool_binding: ToolBinding,
    pub execution_scope: SingleToolExecutionScope,
    pub input_refs: Vec<InputDescriptor>,
    pub output_plan: OutputPlan,
    pub safety_snapshot: SafetySnapshot,
    pub result_placeholder: ResultPlaceholder,
    pub created_at: String,
}
