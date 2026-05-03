//! UI-facing state and presentation contracts for the Rust sandbox.
//!
//! This crate should remain free of toolkit-specific view code.

/// Static shell scaffold text for the phased sandbox UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiShellScaffold {
    pub window_title: String,
    pub control_bar_text: String,
    pub open_project_label: String,
    pub clip_label: String,
    pub accept_tool_label: String,
    pub lume_help_label: String,
    pub project_panel_title: String,
    pub project_panel_body: String,
    pub tree_panel_title: String,
    pub tree_panel_body: String,
    pub tool_panel_title: String,
    pub tool_panel_body: String,
    pub knowledge_panel_title: String,
    pub knowledge_panel_body: String,
    pub chat_panel_title: String,
    pub chat_panel_body: String,
    pub workspace_panel_title: String,
    pub workspace_panel_body: String,
    pub pipeline_view_title: String,
    pub pipeline_view_body: String,
    pub viewer_panel_title: String,
    pub viewer_panel_body: String,
    pub status_text: String,
}

impl Default for UiShellScaffold {
    fn default() -> Self {
        Self {
            window_title: "Rust Portable App Sandbox".to_owned(),
            control_bar_text:
                "Phased shell: project, tree, tools, chat, knowledge, and workspace tabs".to_owned(),
            open_project_label: "Open project".to_owned(),
            clip_label: "📎".to_owned(),
            accept_tool_label: "Open tools".to_owned(),
            lume_help_label: "Lume Help".to_owned(),
            project_panel_title: "Project panel".to_owned(),
            project_panel_body: "No project loaded.".to_owned(),
            tree_panel_title: "Document tree".to_owned(),
            tree_panel_body: "No project document listed.".to_owned(),
            tool_panel_title: "Tool panel".to_owned(),
            tool_panel_body: "Operational tools are selected by default.".to_owned(),
            knowledge_panel_title: "Knowledge panel".to_owned(),
            knowledge_panel_body: "No knowledge document listed.".to_owned(),
            chat_panel_title: "Chat panel".to_owned(),
            chat_panel_body: "Chat stores references, not blobs.".to_owned(),
            workspace_panel_title: "Workspace tabs".to_owned(),
            workspace_panel_body: "Viewer is the active workspace tab.".to_owned(),
            pipeline_view_title: "Pipeline / Ontology View".to_owned(),
            pipeline_view_body: "Semantic proposals are readonly mock governance data.".to_owned(),
            viewer_panel_title: "Viewer panel".to_owned(),
            viewer_panel_body: "No viewer target selected.".to_owned(),
            status_text: "Shell ready.".to_owned(),
        }
    }
}

/// Ephemeral, readonly contextual help popup state for Lume.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LumeHelpPopupState {
    pub is_visible: bool,
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub gui_objects_title: String,
    pub gui_objects: Vec<GuiObjectHelpViewState>,
    pub input_placeholder: String,
    pub close_label: String,
    pub suggestions: Vec<String>,
}

/// Readonly GUI glossary entry shown by Lume Help.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiObjectHelpViewState {
    pub canonical_name: String,
    pub short_description: String,
    pub not_description: String,
}

impl Default for LumeHelpPopupState {
    fn default() -> Self {
        Self {
            is_visible: false,
            title: "Lume Help".to_owned(),
            subtitle:
                "Contextual governed help. Lume Help explains; it does not execute.".to_owned(),
            body: "Lume Help = governed contextual help layer based on declarative sources."
                .to_owned(),
            gui_objects_title: "GUI Objects".to_owned(),
            gui_objects: vec![
                GuiObjectHelpViewState {
                    canonical_name: "Document Tree".to_owned(),
                    short_description: "Navigation of existing governed documents.".to_owned(),
                    not_description: "Not a free filesystem explorer.".to_owned(),
                },
                GuiObjectHelpViewState {
                    canonical_name: "Readonly Viewer".to_owned(),
                    short_description: "READONLY non-editable content viewing.".to_owned(),
                    not_description: "Not an editor.".to_owned(),
                },
                GuiObjectHelpViewState {
                    canonical_name: "Pipeline View".to_owned(),
                    short_description: "Readonly/mock F9.5 flow view.".to_owned(),
                    not_description: "Does not execute AI.".to_owned(),
                },
            ],
            input_placeholder: "Ask about DocGraph governance, phases, or restrictions.".to_owned(),
            close_label: "Close".to_owned(),
            suggestions: vec![
                "Explain this screen".to_owned(),
                "What can I do in this phase?".to_owned(),
                "Why is LLM execution disabled?".to_owned(),
                "What does Proposal mean?".to_owned(),
                "Why are SOURCE and DERIVED readonly?".to_owned(),
            ],
        }
    }
}

impl LumeHelpPopupState {
    pub fn visible() -> Self {
        Self {
            is_visible: true,
            ..Self::default()
        }
    }

    pub fn toggle_visibility(mut self) -> Self {
        self.is_visible = !self.is_visible;
        self
    }
}

/// Minimal project-facing panel state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectPanelState {
    pub title: String,
    pub body: String,
}

impl Default for ProjectPanelState {
    fn default() -> Self {
        Self {
            title: "Project panel".to_owned(),
            body: "No project loaded.".to_owned(),
        }
    }
}

/// Minimal tree action shown for an existing workspace document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentTreeActionView {
    OpenInViewer,
    ReferenceInChat,
    ProfileDocument,
    RunOperationalTool,
    PromoteToKnowledge,
}

/// Minimal area discriminator for project-owned primary documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectDocumentAreaView {
    Knowledge,
    Resource,
    Output,
}

/// Minimal tree entry shown by the workspace document tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTreeEntryViewState {
    pub document_id: String,
    pub display_name: String,
    pub logical_path: String,
    pub area: ProjectDocumentAreaView,
    pub exposed_object_ref: Option<String>,
    pub file_ref: Option<String>,
    pub owner_ref: Option<String>,
    pub trace_ref: Option<String>,
    pub is_selected: bool,
    pub available_actions: Vec<DocumentTreeActionView>,
}

/// Minimal existing-document tree panel state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTreePanelState {
    pub title: String,
    pub entries: Vec<DocumentTreeEntryViewState>,
    pub message: String,
}

impl Default for DocumentTreePanelState {
    fn default() -> Self {
        Self {
            title: "Document tree".to_owned(),
            entries: Vec::new(),
            message: "No project document listed.".to_owned(),
        }
    }
}

/// Minimal tool-facing panel state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolPanelState {
    pub title: String,
    pub active_tab: ToolPanelTab,
    pub operational_tools: Vec<OperationalToolRowState>,
    pub llm_catalogs: Vec<LlmCatalogRowState>,
    pub llm_tools: Vec<LlmToolRowState>,
    pub launch_modal: ToolLaunchModalState,
    pub message: String,
}

impl Default for ToolPanelState {
    fn default() -> Self {
        Self {
            title: "Tool panel".to_owned(),
            active_tab: ToolPanelTab::OperationalTools,
            operational_tools: Vec::new(),
            llm_catalogs: Vec::new(),
            llm_tools: Vec::new(),
            launch_modal: ToolLaunchModalState::default(),
            message: "No operational tool loaded.".to_owned(),
        }
    }
}

/// Minimal tab discriminator for the tool surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolPanelTab {
    OperationalTools,
    LlmTools,
}

/// UI-facing entry kind shown in tool rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolEntryKindView {
    Tool,
    Recipe,
}

/// Minimal enabled state shown by the tool surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolEnabledStateView {
    Enabled,
    Disabled,
}

/// Project-level override state shown in the LLM tools tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolProjectOverrideView {
    Inherit,
    Enabled,
    Disabled,
}

/// Minimal operational tool row shown in the manual-launch tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalToolRowState {
    pub label: String,
    pub tool_id: String,
    pub catalog_id: String,
    pub kind: ToolEntryKindView,
    pub enabled: ToolEnabledStateView,
}

/// Minimal catalog-level policy row shown in the LLM tools tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmCatalogRowState {
    pub label: String,
    pub catalog_id: String,
    pub global_state: ToolEnabledStateView,
    pub project_state: ToolProjectOverrideView,
    pub effective_state: ToolEnabledStateView,
}

/// Minimal entry-level policy row shown in the LLM tools tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmToolRowState {
    pub label: String,
    pub tool_id: String,
    pub catalog_id: String,
    pub kind: ToolEntryKindView,
    pub global_state: ToolEnabledStateView,
    pub project_state: ToolProjectOverrideView,
    pub effective_state: ToolEnabledStateView,
}

/// Minimal modal execution status for manual operational tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolLaunchModalStatus {
    Hidden,
    Ready,
    Ok,
    Error,
}

/// Minimal modal state for controlled manual tool launch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolLaunchModalState {
    pub status: ToolLaunchModalStatus,
    pub title: String,
    pub tool_label: String,
    pub tool_id: String,
    pub catalog_id: String,
    pub description: String,
    pub inputs_summary: String,
    pub result_text: String,
    pub artifact_path: Option<String>,
}

impl Default for ToolLaunchModalState {
    fn default() -> Self {
        Self {
            status: ToolLaunchModalStatus::Hidden,
            title: "Operational tool launch".to_owned(),
            tool_label: String::new(),
            tool_id: String::new(),
            catalog_id: String::new(),
            description: String::new(),
            inputs_summary: "No inputs.".to_owned(),
            result_text: String::new(),
            artifact_path: None,
        }
    }
}

/// Minimal readonly knowledge entry exposed to the shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeEntryViewState {
    pub document_id: String,
    pub display_name: String,
    pub logical_path: String,
    pub is_viewable: bool,
    pub is_selected: bool,
}

/// Small state machine for the knowledge panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgePanelStatus {
    Empty,
    Ready,
    Error,
}

/// Minimal knowledge-facing panel state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgePanelState {
    pub title: String,
    pub entries: Vec<KnowledgeEntryViewState>,
    pub status: KnowledgePanelStatus,
    pub message: String,
}

impl Default for KnowledgePanelState {
    fn default() -> Self {
        Self {
            title: "Knowledge panel".to_owned(),
            entries: Vec::new(),
            status: KnowledgePanelStatus::Empty,
            message: "No knowledge document listed.".to_owned(),
        }
    }
}

/// Minimal clip intent discriminator for external intake and document workflows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipIntentKind {
    AddAsChatContext,
    ProfileDocument,
    PromoteToKnowledge,
    RunOperationalToolOnDocuments,
    AddAsProjectResource,
}

/// Minimal clip option shown when the attachment button is opened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipIntentOptionViewState {
    pub kind: ClipIntentKind,
    pub label: String,
    pub allows_multi_select: bool,
}

/// Minimal clip selector state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipIntentSelectorState {
    pub is_open: bool,
    pub options: Vec<ClipIntentOptionViewState>,
    pub message: String,
}

impl Default for ClipIntentSelectorState {
    fn default() -> Self {
        Self {
            is_open: false,
            options: Vec::new(),
            message: "Clip is closed.".to_owned(),
        }
    }
}

/// Minimal chat message discriminator used by the structural shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatMessageKind {
    Text,
    DocumentRef,
    ToolResult,
    SystemState,
}

/// Minimal origin for structured document references shown in chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatDocumentOriginView {
    TreeReference,
    ChatContext,
    KnowledgePromotion,
    ProjectResource,
}

/// Minimal structured chat message shown by the shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessageViewState {
    pub message_id: String,
    pub kind: ChatMessageKind,
    pub title: String,
    pub body: String,
    pub document_id: Option<String>,
    pub logical_path: Option<String>,
    pub document_origin: Option<ChatDocumentOriginView>,
    pub copy_feedback_visible: bool,
    pub fork_feedback_visible: bool,
    pub is_collapsed: bool,
}

/// UI-routed chat block action intent.
///
/// This is presentation and controller vocabulary only.
/// It does not imply persistence, project mutation, or tool execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatBlockActionIntent {
    CopyBlockRequested { block_id: String },
    ForkBlockRequested { block_id: String },
    ToggleCollapseRequested { block_id: String },
}

/// Minimal chat-facing panel state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatPanelState {
    pub title: String,
    pub messages: Vec<ChatMessageViewState>,
    pub clip_selector: ClipIntentSelectorState,
    pub message: String,
}

impl Default for ChatPanelState {
    fn default() -> Self {
        Self {
            title: "Chat panel".to_owned(),
            messages: Vec::new(),
            clip_selector: ClipIntentSelectorState::default(),
            message: "Chat stores text, references, and structured results.".to_owned(),
        }
    }
}

/// Small state machine for the technical readonly viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerPanelStatus {
    Empty,
    Ready,
    Unsupported,
    Error,
}

/// Minimal readonly viewer panel state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewerPanelState {
    pub title: String,
    pub target_ref: Option<String>,
    pub target_path: Option<String>,
    pub content_text: String,
    pub status: ViewerPanelStatus,
}

impl Default for ViewerPanelState {
    fn default() -> Self {
        Self {
            title: "Viewer panel".to_owned(),
            target_ref: None,
            target_path: None,
            content_text: "No viewer target selected.".to_owned(),
            status: ViewerPanelStatus::Empty,
        }
    }
}

/// Minimal workspace tab discriminator for the central content area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceTabKind {
    Home,
    Viewer,
    KnowledgeDetail,
    ToolResult,
    DocumentProfile,
    PipelineOntology,
    FileIntakeSystem,
}

/// Minimal workspace tab state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceTabViewState {
    pub tab_id: String,
    pub title: String,
    pub kind: WorkspaceTabKind,
    pub is_active: bool,
}

/// Minimal central workspace tabs state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceTabsState {
    pub tabs: Vec<WorkspaceTabViewState>,
    pub active_tab_id: Option<String>,
    pub message: String,
}

impl Default for WorkspaceTabsState {
    fn default() -> Self {
        Self {
            tabs: vec![
                WorkspaceTabViewState {
                    tab_id: "home".to_owned(),
                    title: "Home".to_owned(),
                    kind: WorkspaceTabKind::Home,
                    is_active: false,
                },
                WorkspaceTabViewState {
                    tab_id: "viewer".to_owned(),
                    title: "Viewer".to_owned(),
                    kind: WorkspaceTabKind::Viewer,
                    is_active: false,
                },
                WorkspaceTabViewState {
                    tab_id: "pipeline_ontology".to_owned(),
                    title: "Pipeline View".to_owned(),
                    kind: WorkspaceTabKind::PipelineOntology,
                    is_active: true,
                },
            ],
            active_tab_id: Some("pipeline_ontology".to_owned()),
            message: "Pipeline / Ontology View is active in readonly mock mode.".to_owned(),
        }
    }
}

/// Readonly semantic proposal row for the F9.5 pipeline / ontology view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticProposalViewState {
    pub proposal_id: String,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub graph: String,
    pub status: String,
    pub confidence: String,
}

/// Readonly mock pipeline / ontology state. It represents declarations only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineOntologyViewState {
    pub title: String,
    pub document_id: String,
    pub document_name: String,
    pub document_class: String,
    pub chunk_id: String,
    pub chunk_preview: String,
    pub source_hash: String,
    pub prompt_id: String,
    pub prompt_strategy: String,
    pub prompt_execution_enabled: bool,
    pub semantic_spec_id: String,
    pub semantic_source_layer: String,
    pub semantic_source_kind: String,
    pub semantic_execution_enabled: bool,
    pub proposals: Vec<SemanticProposalViewState>,
    pub human_review_state: String,
    pub human_review_allowed_states: Vec<String>,
    pub future_format: String,
    pub future_store: String,
    pub oxigraph_enabled: bool,
    pub rdf_export_enabled: bool,
    pub semantic_store_enabled: bool,
    pub trace_id: String,
    pub policy: String,
    pub llm_mode: String,
    pub executed: bool,
    pub mutation: bool,
}

impl Default for PipelineOntologyViewState {
    fn default() -> Self {
        Self {
            title: "Pipeline / Ontology View".to_owned(),
            document_id: "doc_mock_source_001".to_owned(),
            document_name: "Mock source document".to_owned(),
            document_class: "SOURCE".to_owned(),
            chunk_id: "chunk_001".to_owned(),
            chunk_preview: "Readonly chunk preview for semantic governance.".to_owned(),
            source_hash: "mock_source_hash_001".to_owned(),
            prompt_id: "extract_concepts_v1".to_owned(),
            prompt_strategy: "structured_prompting".to_owned(),
            prompt_execution_enabled: false,
            semantic_spec_id: "concept_extraction_v1".to_owned(),
            semantic_source_layer: "document_text_runtime".to_owned(),
            semantic_source_kind: "document_text_chunks".to_owned(),
            semantic_execution_enabled: false,
            proposals: vec![
                SemanticProposalViewState {
                    proposal_id: "semprop_001".to_owned(),
                    subject: "chunk_001".to_owned(),
                    predicate: "mentions".to_owned(),
                    object: "concept_neural_networks".to_owned(),
                    graph: "domain_knowledge".to_owned(),
                    status: "NeedsReview".to_owned(),
                    confidence: "mock_only".to_owned(),
                },
                SemanticProposalViewState {
                    proposal_id: "semprop_002".to_owned(),
                    subject: "concept_neural_networks".to_owned(),
                    predicate: "related_to".to_owned(),
                    object: "concept_differential_equations".to_owned(),
                    graph: "domain_knowledge".to_owned(),
                    status: "NeedsReview".to_owned(),
                    confidence: "mock_only".to_owned(),
                },
            ],
            human_review_state: "NeedsReview".to_owned(),
            human_review_allowed_states: vec![
                "Generated".to_owned(),
                "NeedsReview".to_owned(),
                "Approved".to_owned(),
                "Rejected".to_owned(),
                "Stale".to_owned(),
                "Expired".to_owned(),
            ],
            future_format: "N-Quads".to_owned(),
            future_store: "Oxigraph".to_owned(),
            oxigraph_enabled: false,
            rdf_export_enabled: false,
            semantic_store_enabled: false,
            trace_id: "mock_trace_semantic_001".to_owned(),
            policy: "data_sensitivity_policy_v1".to_owned(),
            llm_mode: "disabled".to_owned(),
            executed: false,
            mutation: false,
        }
    }
}

/// Small state machine for the document profiling workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentProfileStatusView {
    Empty,
    Pending,
    Ready,
    Unsupported,
    Error,
}

/// Minimal document profiling panel state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentProfilePanelState {
    pub title: String,
    pub document_id: Option<String>,
    pub display_name: Option<String>,
    pub logical_path: Option<String>,
    pub source_filename: Option<String>,
    pub derivation_status: DocumentProfileStatusView,
    pub extracted_text_path: Option<String>,
    pub pages_count: usize,
    pub chunks_count: usize,
    pub message: String,
}

impl Default for DocumentProfilePanelState {
    fn default() -> Self {
        Self {
            title: "Document profile".to_owned(),
            document_id: None,
            display_name: None,
            logical_path: None,
            source_filename: None,
            derivation_status: DocumentProfileStatusView::Empty,
            extracted_text_path: None,
            pages_count: 0,
            chunks_count: 0,
            message: "No document profile open.".to_owned(),
        }
    }
}

/// Minimal status-line state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusLineState {
    pub text: String,
}

impl Default for StatusLineState {
    fn default() -> Self {
        Self {
            text: "Shell ready.".to_owned(),
        }
    }
}

/// Prepared readonly LLM-facing UI state passed into presentation layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedLlmUiState {
    pub effective_llm_mode: String,
    pub interaction_mode: String,
    pub llm_available: bool,
    pub can_attempt_assisted_response: bool,
    pub tool_surface_summary_present: bool,
    pub tool_use_proposal_present: bool,
    pub reason_codes: Vec<String>,
}

/// UI-safe presentation model for governed LLM interaction status.
///
/// Invariants:
/// - UI view model is presentation-only
/// - UI view model is not policy
/// - UI view model is not permission
/// - UI view model is not execution authority
/// - UI consumes prepared state only
/// - UI must not resolve capability
/// - UI must not validate credentials
/// - UI must not execute tools
/// - UI must not approve actions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmStatusViewModel {
    pub effective_llm_mode: String,
    pub interaction_mode: String,
    pub llm_available: bool,
    pub can_attempt_assisted_response: bool,
    pub tool_surface_summary_present: bool,
    pub tool_use_proposal_present: bool,
    pub reason_codes: Vec<String>,
}

/// Build a readonly UI-facing LLM status view model from prepared state only.
pub fn build_llm_status_view_model(input: PreparedLlmUiState) -> LlmStatusViewModel {
    LlmStatusViewModel {
        effective_llm_mode: input.effective_llm_mode,
        interaction_mode: input.interaction_mode,
        llm_available: input.llm_available,
        can_attempt_assisted_response: input.can_attempt_assisted_response,
        tool_surface_summary_present: input.tool_surface_summary_present,
        tool_use_proposal_present: input.tool_use_proposal_present,
        reason_codes: input.reason_codes,
    }
}

/// Readonly status projection for a governed file intake item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileIntakeItemStatusView {
    Candidate,
    Blocked,
    ImportedNotExposed,
}

/// UI-safe row for an F12.6 file intake item.
///
/// This state is derived from prepared intake metadata only. It does not carry
/// host source paths, raw payloads, or policy decisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIntakeItemSystemViewState {
    pub intake_item_id: String,
    pub sanitized_filename: String,
    pub source_label: String,
    pub status: FileIntakeItemStatusView,
    pub owner_ref: String,
    pub trace_ref: String,
    pub sanitization_state: String,
    pub user_comment_preview: Option<String>,
    pub blocking_reason: Option<String>,
}

/// Readonly System View projection for a prepared F12.6 intake batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIntakeSystemViewState {
    pub title_i18n_key: String,
    pub notice_i18n_key: String,
    pub intake_batch_ref: Option<String>,
    pub item_count: usize,
    pub owner_ref: Option<String>,
    pub trace_ref: Option<String>,
    pub items: Vec<FileIntakeItemSystemViewState>,
}

impl Default for FileIntakeSystemViewState {
    fn default() -> Self {
        Self {
            title_i18n_key: "system_view.file_intake.title".to_owned(),
            notice_i18n_key: "system_view.file_intake.notice".to_owned(),
            intake_batch_ref: None,
            item_count: 0,
            owner_ref: None,
            trace_ref: None,
            items: Vec::new(),
        }
    }
}

/// Build a readonly UI projection from a prepared F12.6 intake batch.
pub fn build_file_intake_system_view_state(
    batch: &io_runtime::FileIntakeBatch,
) -> FileIntakeSystemViewState {
    FileIntakeSystemViewState {
        title_i18n_key: "system_view.file_intake.title".to_owned(),
        notice_i18n_key: "system_view.file_intake.notice".to_owned(),
        intake_batch_ref: Some(batch.intake_batch_ref.clone()),
        item_count: batch.items.len(),
        owner_ref: Some(batch.owner_ref.clone()),
        trace_ref: Some(batch.trace_ref.clone()),
        items: batch
            .items
            .iter()
            .map(|item| FileIntakeItemSystemViewState {
                intake_item_id: item.intake_item_id.clone(),
                sanitized_filename: item.metadata.original_filename_sanitized.clone(),
                source_label: item.metadata.source_display_label.clone(),
                status: map_file_intake_status(item.status),
                owner_ref: item.metadata.owner_ref.clone(),
                trace_ref: item.metadata.trace_ref.clone(),
                sanitization_state: intake_sanitization_state_text(
                    item.metadata.security_sanitization_state,
                )
                .to_owned(),
                user_comment_preview: item
                    .metadata
                    .user_comment
                    .as_ref()
                    .map(|comment| comment.text.clone()),
                blocking_reason: item
                    .blocking_reasons
                    .first()
                    .map(|reason| file_intake_error_code(*reason).to_owned()),
            })
            .collect(),
    }
}

fn map_file_intake_status(status: io_runtime::FileIntakeStatus) -> FileIntakeItemStatusView {
    match status {
        io_runtime::FileIntakeStatus::Planned => FileIntakeItemStatusView::Candidate,
        io_runtime::FileIntakeStatus::Blocked => FileIntakeItemStatusView::Blocked,
        io_runtime::FileIntakeStatus::ImportedNotExposed => {
            FileIntakeItemStatusView::ImportedNotExposed
        }
    }
}

fn intake_sanitization_state_text(
    state: io_runtime::IntakeSecuritySanitizationState,
) -> &'static str {
    match state {
        io_runtime::IntakeSecuritySanitizationState::Safe => "safe",
        io_runtime::IntakeSecuritySanitizationState::Flagged => "flagged",
        io_runtime::IntakeSecuritySanitizationState::Rejected => "rejected",
    }
}

fn file_intake_error_code(reason: io_runtime::FileIntakeError) -> &'static str {
    match reason {
        io_runtime::FileIntakeError::MissingSource => "missing_source",
        io_runtime::FileIntakeError::UnreadableSource => "unreadable_source",
        io_runtime::FileIntakeError::SourceIsDirectory => "source_is_directory",
        io_runtime::FileIntakeError::UnsupportedFormat => "unsupported_format",
        io_runtime::FileIntakeError::UnsafeFilename => "unsafe_filename",
        io_runtime::FileIntakeError::UnsafePath => "unsafe_path",
        io_runtime::FileIntakeError::PrivateAbsolutePath => "private_absolute_path",
        io_runtime::FileIntakeError::MissingOwnerRef => "missing_owner_ref",
        io_runtime::FileIntakeError::MissingTraceRef => "missing_trace_ref",
        io_runtime::FileIntakeError::DuplicateConflict => "duplicate_conflict",
        io_runtime::FileIntakeError::SizeLimitExceeded => "size_limit_exceeded",
        io_runtime::FileIntakeError::SanitizationFailed => "sanitization_failed",
        io_runtime::FileIntakeError::CommentContainsSecrets => "comment_contains_secrets",
        io_runtime::FileIntakeError::CommentContainsPrivatePath => {
            "comment_contains_private_path"
        }
        io_runtime::FileIntakeError::CommentTooLarge => "comment_too_large",
        io_runtime::FileIntakeError::CommentSanitizationFailed => {
            "comment_sanitization_failed"
        }
        io_runtime::FileIntakeError::StoragePathEscape => "storage_path_escape",
        io_runtime::FileIntakeError::RuntimeNotOpened => "runtime_not_opened",
        io_runtime::FileIntakeError::PolicyBlocked => "policy_blocked",
        io_runtime::FileIntakeError::IoError => "io_error",
    }
}

/// Minimal shell-level view model used by the UI controller layer.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ShellViewModel {
    pub project_panel: ProjectPanelState,
    pub document_tree_panel: DocumentTreePanelState,
    pub tool_panel: ToolPanelState,
    pub knowledge_panel: KnowledgePanelState,
    pub chat_panel: ChatPanelState,
    pub workspace_tabs: WorkspaceTabsState,
    pub pipeline_ontology_view: PipelineOntologyViewState,
    pub file_intake_system_view: FileIntakeSystemViewState,
    pub document_profile_panel: DocumentProfilePanelState,
    pub viewer_panel: ViewerPanelState,
    pub status_line: StatusLineState,
}

#[cfg(test)]
mod tests {
    use super::{
        build_file_intake_system_view_state, build_llm_status_view_model,
        FileIntakeItemStatusView, LlmStatusViewModel, PreparedLlmUiState,
    };

    #[test]
    fn llm_status_view_model_preserves_modes() {
        let view_model = build_llm_status_view_model(PreparedLlmUiState {
            effective_llm_mode: String::from("LOCAL"),
            interaction_mode: String::from("ASSISTED"),
            llm_available: true,
            can_attempt_assisted_response: true,
            tool_surface_summary_present: false,
            tool_use_proposal_present: false,
            reason_codes: Vec::new(),
        });

        assert_eq!(view_model.effective_llm_mode, "LOCAL");
        assert_eq!(view_model.interaction_mode, "ASSISTED");
    }

    #[test]
    fn llm_status_view_model_preserves_flags() {
        let view_model = build_llm_status_view_model(PreparedLlmUiState {
            effective_llm_mode: String::from("OFF"),
            interaction_mode: String::from("GUIDED"),
            llm_available: false,
            can_attempt_assisted_response: false,
            tool_surface_summary_present: true,
            tool_use_proposal_present: true,
            reason_codes: Vec::new(),
        });

        assert!(!view_model.llm_available);
        assert!(!view_model.can_attempt_assisted_response);
        assert!(view_model.tool_surface_summary_present);
        assert!(view_model.tool_use_proposal_present);
    }

    #[test]
    fn llm_status_view_model_preserves_reason_codes() {
        let expected_reason_codes = vec![
            String::from("cloud_provider_missing"),
            String::from("assisted_mode_unavailable"),
        ];

        let view_model = build_llm_status_view_model(PreparedLlmUiState {
            effective_llm_mode: String::from("OFF"),
            interaction_mode: String::from("GUIDED"),
            llm_available: false,
            can_attempt_assisted_response: false,
            tool_surface_summary_present: false,
            tool_use_proposal_present: false,
            reason_codes: expected_reason_codes.clone(),
        });

        assert_eq!(view_model.reason_codes, expected_reason_codes);
    }

    #[test]
    fn llm_status_view_model_allows_empty_reason_codes() {
        let view_model = build_llm_status_view_model(PreparedLlmUiState {
            effective_llm_mode: String::from("LOCAL"),
            interaction_mode: String::from("ASSISTED"),
            llm_available: true,
            can_attempt_assisted_response: true,
            tool_surface_summary_present: false,
            tool_use_proposal_present: false,
            reason_codes: Vec::new(),
        });

        assert!(view_model.reason_codes.is_empty());
    }

    #[test]
    fn llm_status_view_model_does_not_modify_values() {
        let input = PreparedLlmUiState {
            effective_llm_mode: String::from("CLOUD"),
            interaction_mode: String::from("ASSISTED"),
            llm_available: true,
            can_attempt_assisted_response: true,
            tool_surface_summary_present: true,
            tool_use_proposal_present: false,
            reason_codes: vec![String::from("tool_visible")],
        };

        let view_model = build_llm_status_view_model(input);

        assert_eq!(
            view_model,
            LlmStatusViewModel {
                effective_llm_mode: String::from("CLOUD"),
                interaction_mode: String::from("ASSISTED"),
                llm_available: true,
                can_attempt_assisted_response: true,
                tool_surface_summary_present: true,
                tool_use_proposal_present: false,
                reason_codes: vec![String::from("tool_visible")],
            }
        );
    }

    #[test]
    fn file_intake_system_view_maps_prepared_batch_without_host_paths() {
        let batch = io_runtime::FileIntakeBatch {
            intake_batch_ref: "intake_batch_alpha".to_owned(),
            owner_ref: "owner.project.alpha".to_owned(),
            trace_ref: "trace_file_intake_alpha".to_owned(),
            status: io_runtime::FileIntakeStatus::ImportedNotExposed,
            batch_comment: None,
            items: vec![io_runtime::FileIntakeItem {
                intake_item_id: "intake_item_001".to_owned(),
                intake_batch_ref: "intake_batch_alpha".to_owned(),
                status: io_runtime::FileIntakeStatus::ImportedNotExposed,
                metadata: intake_metadata(
                    "intake_item_001",
                    "notes.md",
                    io_runtime::IntakeSecuritySanitizationState::Safe,
                    Some("safe project-scoped note"),
                ),
                blocking_reasons: Vec::new(),
                stored_object_candidate_ref: Some("stored_object_candidate_001".to_owned()),
                stored_relative_path: Some("blobs/hash_001/content".to_owned()),
                metadata_relative_path: Some(
                    "intake_batches/intake_batch_alpha/intake_item_001/metadata.json".to_owned(),
                ),
            }],
            created_at: "2026-05-02T00:00:00Z".to_owned(),
        };

        let view = build_file_intake_system_view_state(&batch);

        assert_eq!(view.intake_batch_ref.as_deref(), Some("intake_batch_alpha"));
        assert_eq!(view.item_count, 1);
        assert_eq!(view.owner_ref.as_deref(), Some("owner.project.alpha"));
        assert_eq!(view.trace_ref.as_deref(), Some("trace_file_intake_alpha"));
        assert_eq!(
            view.items[0].status,
            FileIntakeItemStatusView::ImportedNotExposed
        );
        assert_eq!(view.items[0].sanitized_filename, "notes.md");
        assert_eq!(
            view.items[0].user_comment_preview.as_deref(),
            Some("safe project-scoped note")
        );
        assert!(!format!("{view:?}").contains("C:\\"));
    }

    #[test]
    fn file_intake_system_view_maps_blocked_item_reason() {
        let batch = io_runtime::FileIntakeBatch {
            intake_batch_ref: "intake_batch_blocked".to_owned(),
            owner_ref: "owner.project.alpha".to_owned(),
            trace_ref: "trace_file_intake_alpha".to_owned(),
            status: io_runtime::FileIntakeStatus::Blocked,
            batch_comment: None,
            items: vec![io_runtime::FileIntakeItem {
                intake_item_id: "intake_item_unsupported".to_owned(),
                intake_batch_ref: "intake_batch_blocked".to_owned(),
                status: io_runtime::FileIntakeStatus::Blocked,
                metadata: intake_metadata(
                    "intake_item_unsupported",
                    "manual.pdf",
                    io_runtime::IntakeSecuritySanitizationState::Flagged,
                    None,
                ),
                blocking_reasons: vec![io_runtime::FileIntakeError::UnsupportedFormat],
                stored_object_candidate_ref: None,
                stored_relative_path: None,
                metadata_relative_path: None,
            }],
            created_at: "2026-05-02T00:00:00Z".to_owned(),
        };

        let view = build_file_intake_system_view_state(&batch);

        assert_eq!(view.items[0].status, FileIntakeItemStatusView::Blocked);
        assert_eq!(
            view.items[0].blocking_reason.as_deref(),
            Some("unsupported_format")
        );
        assert_eq!(view.items[0].sanitization_state, "flagged");
    }

    fn intake_metadata(
        intake_item_id: &str,
        filename: &str,
        sanitization_state: io_runtime::IntakeSecuritySanitizationState,
        user_comment: Option<&str>,
    ) -> io_runtime::IntakeMetadata {
        io_runtime::IntakeMetadata {
            intake_item_id: intake_item_id.to_owned(),
            intake_batch_ref: "intake_batch_alpha".to_owned(),
            stored_object_candidate_ref: Some("stored_object_candidate_001".to_owned()),
            file_ref: None,
            content_hash: Some("hash_001".to_owned()),
            owner_ref: "owner.project.alpha".to_owned(),
            trace_ref: "trace_file_intake_alpha".to_owned(),
            original_filename_sanitized: filename.to_owned(),
            detected_kind: io_runtime::IntakeDetectedKind::Markdown,
            size_bytes: 42,
            source_kind: "local_file_selection".to_owned(),
            source_display_label: filename.to_owned(),
            classification: io_runtime::IntakeClassification {
                extension: "md".to_owned(),
                media_type_hint: "text/markdown".to_owned(),
                confidence: "basic_extension".to_owned(),
                classification_source: "extension_hint".to_owned(),
                supported_state: "supported".to_owned(),
            },
            security_sanitization_state: sanitization_state,
            exposure_state: io_runtime::IntakeExposureState::ImportedNotExposed,
            derivation_state: "not_available".to_owned(),
            user_comment: user_comment.map(|text| io_runtime::IntakeCommentMetadata {
                text: text.to_owned(),
                comment_author_ref: "user.local".to_owned(),
                comment_created_at: "2026-05-02T00:00:00Z".to_owned(),
                comment_sanitization_state: io_runtime::CommentSanitizationState::Safe,
                comment_visibility: "project".to_owned(),
            }),
        }
    }
}
