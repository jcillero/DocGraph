use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::action_request::InputDescriptor;
use crate::authorized_execution_request::ToolKind;

/// ToolRunManifest status only. It does not activate runtime lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ToolRunManifestStatus {
    DeclaredOnly,
    NotCreated,
    FutureRequired,
}

/// Manifest tool identity metadata only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolRunManifestTool {
    pub tool_id: String,
    pub tool_kind: ToolKind,
    pub tool_version: String,
    pub implementation_ref: String,
}

/// Manifest output descriptor only. It does not imply output creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestOutput {
    pub output_ref: String,
    pub output_kind: String,
    pub owner_ref: String,
}

/// Future ToolRunManifest contract only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolRunManifest {
    pub tool_run_manifest_id: String,
    pub single_tool_execution_ref: String,
    pub authorized_execution_request_ref: String,
    pub human_confirmation_ref: String,
    pub action_request_ref: String,
    pub intent_ref: String,
    pub trace_ref: String,
    pub owner_ref: String,
    pub tool: ToolRunManifestTool,
    pub inputs: Vec<InputDescriptor>,
    pub configuration: BTreeMap<String, Value>,
    pub outputs: Vec<ManifestOutput>,
    pub status: ToolRunManifestStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub warnings: Vec<String>,
    pub created_at: String,
}
