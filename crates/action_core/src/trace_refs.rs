use serde::{Deserialize, Serialize};

/// Trace scope only. It does not imply orchestration or execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TraceScope {
    SingleIntent,
    MultiStepFlow,
    CrossArtifact,
}

/// TraceRecord status only. It does not imply persistence is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TraceRecordStatus {
    DeclaredOnly,
    NotPersisted,
    FutureRequired,
}

/// Trace event kind only. It does not emit runtime events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TraceEventKind {
    SingleToolExecutionDeclared,
}

/// TraceRecord contract only. It is reference-only, sanitized, and non-executing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceRecord {
    pub trace_record_id: String,
    pub trace_ref: String,
    pub single_tool_execution_ref: String,
    pub authorized_execution_request_ref: String,
    pub action_request_ref: String,
    pub event_kind: TraceEventKind,
    pub scope: TraceScope,
    pub status: TraceRecordStatus,
    pub links: Vec<String>,
    pub sanitized_summary_key: String,
    pub created_at: String,
}
