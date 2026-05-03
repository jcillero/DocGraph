//! Slint presentation integration crate for the Rust sandbox.
//!
//! This crate hosts the structural shell plus thin controllers over existing
//! lower-layer boundaries.

use std::{rc::Rc, time::Duration};

use app_services::{open_project_app_service, ProjectRuntimeOutput, RuntimeFailure};
use document_text_runtime::DocumentTextDerivationError;
use io_runtime::{
    list_project_knowledge_documents, register_chat_resource, resolve_project_document,
    resolve_project_knowledge_document, IoRuntimeError, KnowledgeDocumentRef,
    ProjectDocumentArea, ProjectDocumentRef,
};
use project_runtime::{
    load_manifest_document_tree_rows, ManifestDocumentTreeRow, ResolvedViewerTarget,
};
use spec_runtime::{load_document_from_disk, DocumentCategory, DocumentSource, SourceAuthority};
use tool_runtime::{
    load_effective_meta_catalog, resolve_catalog_request, resolve_catalog_selection,
    resolve_tool_selection, CatalogEntryKind, CatalogSelectionRequest, ToolExecutionOutcome,
    ToolRuntimeError, ToolRuntimeResult,
};
use ui_core::{
    ChatBlockActionIntent, ChatDocumentOriginView, ChatMessageKind, ChatMessageViewState,
    ChatPanelState, ClipIntentKind, ClipIntentOptionViewState, ClipIntentSelectorState,
    DocumentProfilePanelState, DocumentProfileStatusView, DocumentTreeActionView,
    DocumentTreeEntryViewState, DocumentTreePanelState, KnowledgeEntryViewState,
    KnowledgePanelState, KnowledgePanelStatus, LumeHelpPopupState, OperationalToolRowState,
    build_file_intake_system_view_state, FileIntakeItemStatusView, FileIntakeSystemViewState,
    PipelineOntologyViewState, ProjectDocumentAreaView, ProjectPanelState, ShellViewModel,
    StatusLineState, ToolEnabledStateView, ToolEntryKindView, ToolLaunchModalState,
    ToolLaunchModalStatus, ToolPanelState, ToolPanelTab, ToolProjectOverrideView, UiShellScaffold,
    ViewerPanelState, ViewerPanelStatus, WorkspaceTabKind, WorkspaceTabViewState,
    WorkspaceTabsState,
};
use workspace_core::{ProjectRoot, WorkspaceError, WorkspaceRoot};
use slint::Model;

slint::slint! {
    import { Button, HorizontalBox, ScrollView, VerticalBox } from "std-widgets.slint";

    export struct ChatBlockRow {
        block-id: string,
        title: string,
        body: string,
        body-preview: string,
        copy-payload: string,
        copy-feedback-visible: bool,
        fork-feedback-visible: bool,
        is-collapsed: bool,
    }

    export component RustPortableAppShell inherits Window {
        in property <string> window-title;
        in property <string> control-bar-text;
        in property <string> open-project-label;
        in property <string> clip-label;
        in property <string> accept-tool-label;
        in property <string> lume-help-label;
        in property <string> project-panel-title;
        in property <string> project-panel-body;
        in property <string> tree-panel-title;
        in property <string> tree-panel-body;
        in property <string> tool-panel-title;
        in property <string> tool-panel-body;
        in property <string> operational-tools-label;
        in property <string> llm-tools-label;
        in property <string> tool-modal-title;
        in property <string> tool-modal-body;
        in property <string> knowledge-panel-title;
        in property <string> knowledge-panel-body;
        in property <string> chat-panel-title;
        in property <string> chat-panel-body;
        in property <[ChatBlockRow]> chat-blocks;
        in property <string> chat-action-copy-label;
        in property <string> chat-action-copied-label;
        in property <string> chat-action-fork-label;
        in property <string> chat-action-fork-pending-label;
        in property <string> chat-action-collapse-label;
        in property <string> chat-action-expand-label;
        in property <string> workspace-panel-title;
        in property <string> workspace-panel-body;
        in property <string> pipeline-view-title;
        in property <string> pipeline-view-body;
        in property <string> file-intake-view-title;
        in property <string> file-intake-view-body;
        in property <string> viewer-panel-title;
        in property <string> viewer-panel-body;
        in property <bool> lume-help-visible;
        in property <string> lume-help-title;
        in property <string> lume-help-subtitle;
        in property <string> lume-help-body;
        in property <string> lume-help-gui-objects-title;
        in property <string> lume-help-gui-objects;
        in property <string> lume-help-suggestions;
        in property <string> lume-help-input-placeholder;
        in property <string> lume-help-close-label;
        in property <string> status-text;

        callback request-open-project();
        callback request-open-clip();
        callback request-accept-tool();
        callback request-open-lume-help();
        callback request-close-lume-help();
        callback request-copy-block(string);
        callback request-fork-block(string);
        callback request-toggle-collapse-block(string);

        title: root.window-title;
        width: 960px;
        height: 640px;

        VerticalBox {
            spacing: 12px;

            HorizontalBox {
                spacing: 12px;
                Text {
                    text: root.control-bar-text;
                }
                Button {
                    text: root.open-project-label;
                    clicked => { root.request-open-project(); }
                }
                Button {
                    text: root.clip-label;
                    clicked => { root.request-open-clip(); }
                }
                Button {
                    text: root.accept-tool-label;
                    clicked => { root.request-accept-tool(); }
                }
                Button {
                    text: root.lume-help-label;
                    clicked => { root.request-open-lume-help(); }
                }
            }

            HorizontalBox {
                spacing: 12px;

                VerticalBox {
                    spacing: 4px;
                    Text {
                        text: root.project-panel-title;
                    }
                    Text {
                        text: root.project-panel-body;
                    }
                }

                VerticalBox {
                    spacing: 4px;
                    Text {
                        text: root.tree-panel-title;
                    }
                    Text {
                        text: root.tree-panel-body;
                    }
                }

                VerticalBox {
                    spacing: 4px;
                    Text {
                        text: root.tool-panel-title;
                    }
                    HorizontalBox {
                        spacing: 8px;
                        Text {
                            text: root.operational-tools-label;
                        }
                        Text {
                            text: root.llm-tools-label;
                        }
                    }
                    Text {
                        text: root.tool-panel-body;
                    }
                    Text {
                        text: root.tool-modal-title;
                    }
                    Text {
                        text: root.tool-modal-body;
                    }
                }

                VerticalBox {
                    spacing: 4px;
                    Text {
                        text: root.knowledge-panel-title;
                    }
                    Text {
                        text: root.knowledge-panel-body;
                    }
                }
            }

            HorizontalBox {
                spacing: 12px;

                VerticalBox {
                    spacing: 4px;
                    Text {
                        text: root.chat-panel-title;
                    }
                    Text {
                        text: root.chat-panel-body;
                    }
                    ScrollView {
                        min-width: 240px;
                        min-height: 180px;
                        VerticalBox {
                            spacing: 8px;
                            for chat-block in root.chat-blocks : Rectangle {
                                min-width: 240px;
                                min-height: 96px;
                                border-width: 1px;
                                VerticalBox {
                                    padding: 8px;
                                    spacing: 6px;

                                    Text {
                                        text: chat-block.title;
                                        wrap: word-wrap;
                                    }

                                    Text {
                                        text: chat-block.is-collapsed ? chat-block.body-preview : chat-block.body;
                                        wrap: word-wrap;
                                    }

                                    HorizontalBox {
                                        min-height: 24px;
                                        spacing: 6px;

                                        Button {
                                            text: "⧉";
                                            accessible-label: root.chat-action-copy-label;
                                            clicked => {
                                                root.request-copy-block(chat-block.block-id);
                                            }
                                        }

                                        Button {
                                            text: "⑂";
                                            accessible-label: root.chat-action-fork-label;
                                            clicked => {
                                                root.request-fork-block(chat-block.block-id);
                                            }
                                        }

                                        Button {
                                            text: chat-block.is-collapsed ? "▸" : "▾";
                                            accessible-label: chat-block.is-collapsed ? root.chat-action-expand-label : root.chat-action-collapse-label;
                                            clicked => {
                                                root.request-toggle-collapse-block(chat-block.block-id);
                                            }
                                        }

                                        Text {
                                            visible: chat-block.copy-feedback-visible;
                                            text: root.chat-action-copied-label;
                                        }

                                        Text {
                                            visible: chat-block.fork-feedback-visible;
                                            text: root.chat-action-fork-pending-label;
                                        }

                                        Rectangle {
                                            horizontal-stretch: 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                VerticalBox {
                    spacing: 4px;
                    Text {
                        text: root.workspace-panel-title;
                    }
                    Text {
                        text: root.workspace-panel-body;
                    }
                    Text {
                        text: root.pipeline-view-title;
                    }
                    Text {
                        text: root.pipeline-view-body;
                    }
                    Text {
                        text: root.file-intake-view-title;
                    }
                    Text {
                        text: root.file-intake-view-body;
                    }
                }

                VerticalBox {
                    spacing: 4px;
                    Text {
                        text: root.viewer-panel-title;
                    }
                    Text {
                        text: root.viewer-panel-body;
                    }
                }
            }

            Text {
                text: root.status-text;
            }

            VerticalBox {
                visible: root.lume-help-visible;
                spacing: 4px;
                Text {
                    text: root.lume-help-title;
                }
                Text {
                    text: root.lume-help-subtitle;
                }
                Text {
                    text: root.lume-help-body;
                }
                Text {
                    text: root.lume-help-gui-objects-title;
                }
                Text {
                    text: root.lume-help-gui-objects;
                }
                Text {
                    text: root.lume-help-suggestions;
                }
                Text {
                    text: root.lume-help-input-placeholder;
                }
                Button {
                    text: root.lume-help-close-label;
                    clicked => { root.request-close-lume-help(); }
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProjectController;

impl ProjectController {
    pub fn new() -> Self {
        Self
    }

    pub fn open_project(
        &self,
        workspace_root: &WorkspaceRoot,
        project_root: ProjectRoot,
    ) -> Result<ProjectViewUpdate, RuntimeFailure> {
        let result = open_project_app_service(workspace_root, project_root)?;

        Ok(ProjectViewUpdate {
            project_panel: map_project_runtime_output_to_project_panel_state(&result.value),
            status_line: StatusLineState {
                text: "Project flow loaded through app_services.".to_owned(),
            },
        })
    }
}

#[derive(Debug)]
pub enum DocumentWorkflowError {
    Io(IoRuntimeError),
    TextDerivation(DocumentTextDerivationError),
    ToolRuntime(ToolRuntimeError),
    Workspace(WorkspaceError),
    LegacyQuarantined(&'static str),
}

impl std::fmt::Display for DocumentWorkflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "{error}"),
            Self::TextDerivation(error) => write!(f, "{error}"),
            Self::ToolRuntime(error) => write!(f, "{error}"),
            Self::Workspace(error) => write!(f, "{error}"),
            Self::LegacyQuarantined(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for DocumentWorkflowError {}

impl From<IoRuntimeError> for DocumentWorkflowError {
    fn from(value: IoRuntimeError) -> Self {
        Self::Io(value)
    }
}

impl From<DocumentTextDerivationError> for DocumentWorkflowError {
    fn from(value: DocumentTextDerivationError) -> Self {
        Self::TextDerivation(value)
    }
}

impl From<ToolRuntimeError> for DocumentWorkflowError {
    fn from(value: ToolRuntimeError) -> Self {
        Self::ToolRuntime(value)
    }
}

impl From<WorkspaceError> for DocumentWorkflowError {
    fn from(value: WorkspaceError) -> Self {
        Self::Workspace(value)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ToolController;

impl ToolController {
    pub fn new() -> Self {
        Self
    }

    pub fn load_operational_tools(
        &self,
        project_root: &ProjectRoot,
    ) -> Result<ToolViewUpdate, ToolRuntimeError> {
        let effective_meta_catalog = load_effective_meta_catalog(project_root)?;
        let tool_panel = map_operational_tools_to_panel_state(&effective_meta_catalog);

        Ok(ToolViewUpdate {
            tool_panel,
            status_line: StatusLineState {
                text: "Operational tools loaded through tool_runtime.".to_owned(),
            },
        })
    }

    pub fn open_operational_tool_launch(
        &self,
        project_root: &ProjectRoot,
        catalog_id: &str,
        tool_id: &str,
    ) -> Result<ToolViewUpdate, ToolRuntimeError> {
        let effective_meta_catalog = load_effective_meta_catalog(project_root)?;
        let catalog = resolve_catalog_selection(&effective_meta_catalog, catalog_id)?;
        let tool = resolve_tool_selection(&catalog, tool_id)?;
        let mut tool_panel = map_operational_tools_to_panel_state(&effective_meta_catalog);
        tool_panel.launch_modal = ToolLaunchModalState {
            status: ToolLaunchModalStatus::Ready,
            title: "Operational tool launch".to_owned(),
            tool_label: tool.label().to_owned(),
            tool_id: tool.id().to_owned(),
            catalog_id: catalog.id().to_owned(),
            description: format!("Controlled manual launch for {}.", tool.label()),
            inputs_summary: "No inputs supported in this iteration.".to_owned(),
            result_text: String::new(),
            artifact_path: None,
        };
        tool_panel.message = "Launch modal opened for operational tool.".to_owned();

        Ok(ToolViewUpdate {
            tool_panel,
            status_line: StatusLineState {
                text: "Operational tool modal opened.".to_owned(),
            },
        })
    }

    pub fn open_operational_tool_launch_for_documents(
        &self,
        project_root: &ProjectRoot,
        catalog_id: &str,
        tool_id: &str,
        selected_documents: &[ProjectDocumentRef],
    ) -> Result<ToolViewUpdate, ToolRuntimeError> {
        let mut update = self.open_operational_tool_launch(project_root, catalog_id, tool_id)?;
        if !selected_documents.is_empty() {
            update.tool_panel.launch_modal.inputs_summary =
                format_selected_documents(selected_documents);
            update.tool_panel.message =
                "Launch modal opened for operational tool with document selection.".to_owned();
            update.status_line = StatusLineState {
                text: "Operational tool modal opened with selected documents.".to_owned(),
            };
        }
        Ok(update)
    }

    pub fn run_operational_tool(
        &self,
        project_root: &ProjectRoot,
        catalog_id: &str,
        entry_id: &str,
    ) -> Result<ToolViewUpdate, ToolRuntimeError> {
        let effective_meta_catalog = load_effective_meta_catalog(project_root)?;
        let request = CatalogSelectionRequest::new(catalog_id, entry_id);
        let resolved_entry = resolve_catalog_request(&effective_meta_catalog, &request)?;
        let mut tool_panel = map_operational_tools_to_panel_state(&effective_meta_catalog);
        tool_panel.launch_modal = ToolLaunchModalState {
            status: ToolLaunchModalStatus::Error,
            title: "Operational tool launch".to_owned(),
            tool_label: resolved_entry.entry_id().to_owned(),
            tool_id: resolved_entry.entry_id().to_owned(),
            catalog_id: resolved_entry.catalog_id().to_owned(),
            description: "F9 operational UI emits intent only; runtime execution remains blocked at this boundary."
                .to_owned(),
            inputs_summary: "Execution request captured as UI intent only.".to_owned(),
            result_text: "execution_disabled_in_f9_ui_boundary".to_owned(),
            artifact_path: None,
        };
        tool_panel.message = "Operational tool execution is blocked in the UI boundary during F9."
            .to_owned();

        Ok(ToolViewUpdate {
            tool_panel,
            status_line: StatusLineState {
                text: "Operational tool intent captured; execution remains blocked in F9."
                    .to_owned(),
            },
        })
    }

    pub fn load_llm_tools(
        &self,
        _project_root: Option<&ProjectRoot>,
    ) -> Result<ToolViewUpdate, ToolRuntimeError> {
        Ok(ToolViewUpdate {
            tool_panel: build_phase_blocked_llm_tool_panel_state(),
            status_line: StatusLineState {
                text: "LLM tools remain declarative and blocked in F9.".to_owned(),
            },
        })
    }

    pub fn set_global_llm_catalog_state(
        &self,
        project_root: Option<&ProjectRoot>,
        _catalog_id: &str,
        _enabled: bool,
    ) -> Result<ToolViewUpdate, ToolRuntimeError> {
        self.load_llm_tools(project_root)
    }

    pub fn set_global_llm_entry_state(
        &self,
        project_root: Option<&ProjectRoot>,
        _catalog_id: &str,
        _entry_id: &str,
        _enabled: bool,
    ) -> Result<ToolViewUpdate, ToolRuntimeError> {
        self.load_llm_tools(project_root)
    }

    pub fn set_project_llm_catalog_state(
        &self,
        project_root: &ProjectRoot,
        _catalog_id: &str,
        _enabled: bool,
    ) -> Result<ToolViewUpdate, ToolRuntimeError> {
        self.load_llm_tools(Some(project_root))
    }

    pub fn set_project_llm_entry_state(
        &self,
        project_root: &ProjectRoot,
        _catalog_id: &str,
        _entry_id: &str,
        _enabled: bool,
    ) -> Result<ToolViewUpdate, ToolRuntimeError> {
        self.load_llm_tools(Some(project_root))
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DocumentTreeController;

impl DocumentTreeController {
    pub fn new() -> Self {
        Self
    }

    pub fn load_tree(
        &self,
        workspace_root: &WorkspaceRoot,
        project_root: &ProjectRoot,
    ) -> DocumentTreeViewUpdate {
        match load_manifest_document_tree_rows(workspace_root, project_root) {
            Ok(rows) => manifest_tree_view_update(&rows),
            Err(error) => DocumentTreeViewUpdate {
                tree_panel: manifest_tree_error_panel_state(&error.to_string()),
                status_line: StatusLineState {
                    text: "Document tree manifest visibility could not be prepared.".to_owned(),
                },
            },
        }
    }

    pub fn select_document(
        &self,
        project_root: &ProjectRoot,
        document_id: &str,
    ) -> Result<DocumentTreeSelectionUpdate, IoRuntimeError> {
        let document = resolve_project_document(project_root, document_id)?;

        Ok(DocumentTreeSelectionUpdate {
            tree_panel: map_project_documents_to_tree_panel_state(
                std::slice::from_ref(&document),
                Some(document_id),
            ),
            document,
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ChatController;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChatBlockActionUpdate {
    next_panel: ChatPanelState,
    intent: ChatBlockActionIntent,
    copied_text: Option<String>,
}

impl ChatController {
    pub fn new() -> Self {
        Self
    }

    pub fn open_clip_selector(&self, current: &ChatPanelState) -> ChatPanelState {
        ChatPanelState {
            title: current.title.clone(),
            messages: current.messages.clone(),
            clip_selector: default_clip_selector_state(),
            message: "Clip opened for explicit document intake and workflows.".to_owned(),
        }
    }

    pub fn append_document_reference_from_tree(
        &self,
        current: &ChatPanelState,
        document: &ProjectDocumentRef,
    ) -> ChatPanelState {
        self.push_message(
            current,
            ChatMessageKind::DocumentRef,
            format!("Reference: {}", document.display_name()),
            format!(
                "document_id={}\npath={}\norigin=tree_reference",
                document.document_id(),
                document.logical_path().portable_text()
            ),
            Some(document.document_id().to_owned()),
            Some(document.logical_path().portable_text()),
            Some(ChatDocumentOriginView::TreeReference),
            "Structured document reference inserted from the tree.",
        )
    }

    pub fn append_chat_context_resources(
        &self,
        current: &ChatPanelState,
        resources: &[io_runtime::ChatResource],
    ) -> ChatPanelState {
        let mut next = ChatPanelState {
            title: current.title.clone(),
            messages: current.messages.clone(),
            clip_selector: ClipIntentSelectorState::default(),
            message: if resources.is_empty() {
                "No chat context resource imported.".to_owned()
            } else {
                "External files imported as chat context references.".to_owned()
            },
        };

        for resource in resources {
            let next_index = next.messages.len() + 1;
            next.messages.push(ChatMessageViewState {
                message_id: format!("msg_{next_index:04}"),
                kind: ChatMessageKind::DocumentRef,
                title: format!("Context: {}", resource.original_filename()),
                body: format!(
                    "resource_id={}\nrelative_path={}\norigin=chat_context",
                    resource.resource_id(),
                    resource.relative_path()
                ),
                document_id: Some(resource.resource_id().to_owned()),
                logical_path: Some(resource.relative_path().to_owned()),
                document_origin: Some(ChatDocumentOriginView::ChatContext),
                copy_feedback_visible: false,
                fork_feedback_visible: false,
                is_collapsed: false,
            });
        }

        next
    }

    pub fn append_project_resource_reference(
        &self,
        current: &ChatPanelState,
        document: &ProjectDocumentRef,
        origin: ChatDocumentOriginView,
        message: &str,
    ) -> ChatPanelState {
        self.push_message(
            current,
            ChatMessageKind::DocumentRef,
            format!("Reference: {}", document.display_name()),
            format!(
                "document_id={}\npath={}\norigin={}",
                document.document_id(),
                document.logical_path().portable_text(),
                chat_origin_text(origin)
            ),
            Some(document.document_id().to_owned()),
            Some(document.logical_path().portable_text()),
            Some(origin),
            message,
        )
    }

    pub fn append_tool_result(
        &self,
        current: &ChatPanelState,
        result: &ToolRuntimeResult,
    ) -> ChatPanelState {
        self.push_message(
            current,
            ChatMessageKind::ToolResult,
            format!("Tool result: {}", result.entry_id()),
            format!(
                "catalog_id={}\nentry_id={}\nentry_kind={}\noutcome={}",
                result.catalog_id(),
                result.entry_id(),
                entry_kind_text(result.entry_kind()),
                execution_outcome_text(result.outcome())
            ),
            None,
            None,
            None,
            "Structured tool result appended to chat.",
        )
    }

    pub fn append_system_state(
        &self,
        current: &ChatPanelState,
        title: &str,
        body: &str,
        message: &str,
    ) -> ChatPanelState {
        self.push_message(
            current,
            ChatMessageKind::SystemState,
            title.to_owned(),
            body.to_owned(),
            None,
            None,
            None,
            message,
        )
    }

    fn request_copy_block(
        &self,
        current: &ChatPanelState,
        block_id: &str,
    ) -> Option<ChatBlockActionUpdate> {
        let copied_text = current
            .messages
            .iter()
            .find(|message| message.message_id == block_id)
            .map(chat_message_copy_payload)?;

        Some(ChatBlockActionUpdate {
            next_panel: chat_panel_with_action_feedback(current, block_id, true, false),
            intent: ChatBlockActionIntent::CopyBlockRequested {
                block_id: block_id.to_owned(),
            },
            copied_text: Some(copied_text),
        })
    }

    fn request_fork_block(
        &self,
        current: &ChatPanelState,
        block_id: &str,
    ) -> Option<ChatBlockActionUpdate> {
        current
            .messages
            .iter()
            .find(|message| message.message_id == block_id)?;

        Some(ChatBlockActionUpdate {
            next_panel: chat_panel_with_action_feedback(current, block_id, false, true),
            intent: ChatBlockActionIntent::ForkBlockRequested {
                block_id: block_id.to_owned(),
            },
            copied_text: None,
        })
    }

    fn request_toggle_collapse_block(
        &self,
        current: &ChatPanelState,
        block_id: &str,
    ) -> Option<ChatBlockActionUpdate> {
        let target = current
            .messages
            .iter()
            .find(|message| message.message_id == block_id)?;

        Some(ChatBlockActionUpdate {
            next_panel: chat_panel_with_collapsed_toggle(current, block_id, !target.is_collapsed),
            intent: ChatBlockActionIntent::ToggleCollapseRequested {
                block_id: block_id.to_owned(),
            },
            copied_text: None,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn push_message(
        &self,
        current: &ChatPanelState,
        kind: ChatMessageKind,
        title: String,
        body: String,
        document_id: Option<String>,
        logical_path: Option<String>,
        document_origin: Option<ChatDocumentOriginView>,
        message: &str,
    ) -> ChatPanelState {
        let mut messages = current.messages.clone();
        let next_index = messages.len() + 1;
        messages.push(ChatMessageViewState {
            message_id: format!("msg_{next_index:04}"),
            kind,
            title,
            body,
            document_id,
            logical_path,
            document_origin,
            copy_feedback_visible: false,
            fork_feedback_visible: false,
            is_collapsed: false,
        });

        ChatPanelState {
            title: current.title.clone(),
            messages,
            clip_selector: ClipIntentSelectorState::default(),
            message: message.to_owned(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DocumentWorkflowController;

impl DocumentWorkflowController {
    pub fn new() -> Self {
        Self
    }

    pub fn profile_existing_document(
        &self,
        document: &ProjectDocumentRef,
    ) -> DocumentProfileViewUpdate {
        DocumentProfileViewUpdate {
            profile_panel: DocumentProfilePanelState {
                title: "Document profile".to_owned(),
                document_id: Some(document.document_id().to_owned()),
                display_name: Some(document.display_name().to_owned()),
                logical_path: Some(document.logical_path().portable_text()),
                source_filename: Some(document.display_name().to_owned()),
                derivation_status: DocumentProfileStatusView::Pending,
                extracted_text_path: None,
                pages_count: 0,
                chunks_count: 0,
                message:
                    "Profile workflow opened over an existing source document without reimporting it."
                        .to_owned(),
            },
            workspace_tabs: build_workspace_tabs_for_document_profile(
                document.document_id(),
                document.display_name(),
            ),
            status_line: StatusLineState {
                text: "Document profile workflow opened from the tree.".to_owned(),
            },
        }
    }

    pub fn import_and_profile_document(
        &self,
        _project_root: &ProjectRoot,
        _source_file: &std::path::Path,
    ) -> Result<ImportedProfileDocument, DocumentWorkflowError> {
        Err(legacy_quarantined_error(
            "Legacy clip-profile import is quarantined until F12 intake and the F13 exposure gate replace direct UI import and derivation paths.",
        ))
    }

    pub fn promote_external_document(
        &self,
        _project_root: &ProjectRoot,
        _source_file: &std::path::Path,
    ) -> Result<ProjectDocumentRef, DocumentWorkflowError> {
        Err(legacy_quarantined_error(
            "Legacy external knowledge promotion is quarantined until F12 intake and the F13 exposure gate replace direct UI promotion paths.",
        ))
    }

    pub fn add_external_project_resource(
        &self,
        _project_root: &ProjectRoot,
        _source_file: &std::path::Path,
    ) -> Result<ProjectDocumentRef, DocumentWorkflowError> {
        Err(legacy_quarantined_error(
            "Legacy project-resource import is quarantined until F12 intake and the F13 exposure gate replace direct UI import paths.",
        ))
    }

    pub fn promote_existing_document(
        &self,
        _project_root: &ProjectRoot,
        _document: &ProjectDocumentRef,
    ) -> Result<ProjectDocumentRef, DocumentWorkflowError> {
        Err(legacy_quarantined_error(
            "Legacy in-project promotion is quarantined until the governed exposure path is implemented.",
        ))
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct KnowledgeController;

impl KnowledgeController {
    pub fn new() -> Self {
        Self
    }

    pub fn load_knowledge(&self, project_root: &ProjectRoot) -> KnowledgeViewUpdate {
        match list_project_knowledge_documents(project_root) {
            Ok(documents) => KnowledgeViewUpdate {
                knowledge_panel: map_knowledge_documents_to_panel_state(&documents, None),
                status_line: StatusLineState {
                    text: if documents.is_empty() {
                        "Knowledge panel is empty.".to_owned()
                    } else {
                        "Knowledge panel loaded from project knowledge.".to_owned()
                    },
                },
            },
            Err(error) => KnowledgeViewUpdate {
                knowledge_panel: map_knowledge_error_to_panel_state(&error),
                status_line: StatusLineState {
                    text: "Knowledge panel failed to load.".to_owned(),
                },
            },
        }
    }

    pub fn open_knowledge_entry(
        &self,
        workspace_root: &WorkspaceRoot,
        project_root: &ProjectRoot,
        document_id: &str,
    ) -> KnowledgeSelectionUpdate {
        match list_project_knowledge_documents(project_root) {
            Ok(documents) => match resolve_project_knowledge_document(project_root, document_id) {
                Ok(document) => {
                    let source =
                        match knowledge_document_source(workspace_root, project_root, &document) {
                            Ok(source) => source,
                            Err(error) => {
                                return KnowledgeSelectionUpdate {
                                    knowledge_panel: map_knowledge_documents_to_panel_state(
                                        &documents, None,
                                    ),
                                    viewer_selection: None,
                                    status_line: StatusLineState {
                                        text: format!(
                                            "Knowledge document could not be prepared: {error}"
                                        ),
                                    },
                                };
                            }
                        };

                    KnowledgeSelectionUpdate {
                        knowledge_panel: map_knowledge_documents_to_panel_state(
                            &documents,
                            Some(document.document_id()),
                        ),
                        viewer_selection: Some(ViewerDocumentSelection {
                            title: document.display_name().to_owned(),
                            target_ref: document.document_id().to_owned(),
                            source,
                            authority: SourceAuthority::Project,
                        }),
                        status_line: StatusLineState {
                            text: "Knowledge document loaded in readonly mode.".to_owned(),
                        },
                    }
                }
                Err(error) => KnowledgeSelectionUpdate {
                    knowledge_panel: map_knowledge_error_to_panel_state(&error),
                    viewer_selection: None,
                    status_line: StatusLineState {
                        text: "Knowledge document could not be selected.".to_owned(),
                    },
                },
            },
            Err(error) => KnowledgeSelectionUpdate {
                knowledge_panel: map_knowledge_error_to_panel_state(&error),
                viewer_selection: None,
                status_line: StatusLineState {
                    text: "Knowledge panel failed to load.".to_owned(),
                },
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ViewerController;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewerDocumentSelection {
    title: String,
    target_ref: String,
    source: DocumentSource,
    authority: SourceAuthority,
}

impl ViewerController {
    pub fn new() -> Self {
        Self
    }

    pub fn open_resolved_target(
        &self,
        workspace_root: &WorkspaceRoot,
        resolved_target: &ResolvedViewerTarget,
    ) -> ViewerPanelState {
        let relative_path = match workspace_root.relative_path_for(resolved_target.resolved_path())
        {
            Ok(path) => path,
            Err(error) => {
                return ViewerPanelState {
                    title: "Viewer panel".to_owned(),
                    target_ref: Some(resolved_target.target_ref().as_str().to_owned()),
                    target_path: None,
                    content_text: format!("viewer error: {error}"),
                    status: ViewerPanelStatus::Error,
                }
            }
        };
        let target_path = relative_path.to_string();

        let source = match DocumentSource::new(
            DocumentCategory::Resource,
            resolved_target.target_ref().as_str(),
            relative_path,
        ) {
            Ok(source) => source,
            Err(error) => {
                return ViewerPanelState {
                    title: "Viewer panel".to_owned(),
                    target_ref: Some(resolved_target.target_ref().as_str().to_owned()),
                    target_path: Some(target_path),
                    content_text: format!("viewer error: {error}"),
                    status: ViewerPanelStatus::Error,
                }
            }
        };

        self.open_document_source(
            workspace_root,
            &ViewerDocumentSelection {
                title: resolved_target.label().unwrap_or("Viewer panel").to_owned(),
                target_ref: resolved_target.target_ref().as_str().to_owned(),
                source,
                authority: SourceAuthority::Project,
            },
        )
    }

    pub fn open_document_source(
        &self,
        workspace_root: &WorkspaceRoot,
        selection: &ViewerDocumentSelection,
    ) -> ViewerPanelState {
        let target_path = selection.source.path_text();
        if !supports_readonly_text_view(&target_path) {
            return ViewerPanelState {
                title: selection.title.clone(),
                target_ref: Some(selection.target_ref.clone()),
                target_path: Some(target_path),
                content_text: "viewer target is not supported for readonly text view".to_owned(),
                status: ViewerPanelStatus::Unsupported,
            };
        }

        match load_document_from_disk(
            workspace_root,
            selection.source.clone(),
            selection.authority,
        ) {
            Ok(document) => ViewerPanelState {
                title: selection.title.clone(),
                target_ref: Some(selection.target_ref.clone()),
                target_path: Some(target_path),
                content_text: document.contents().to_owned(),
                status: ViewerPanelStatus::Ready,
            },
            Err(error) => ViewerPanelState {
                title: selection.title.clone(),
                target_ref: Some(selection.target_ref.clone()),
                target_path: Some(target_path),
                content_text: format!("viewer error: {error}"),
                status: ViewerPanelStatus::Error,
            },
        }
    }
}

/// Thin shell controller that routes UI events to existing boundaries.
#[derive(Debug, Default, Clone, Copy)]
pub struct ShellController {
    project_controller: ProjectController,
    document_tree_controller: DocumentTreeController,
    tool_controller: ToolController,
    knowledge_controller: KnowledgeController,
    chat_controller: ChatController,
    document_workflow_controller: DocumentWorkflowController,
    viewer_controller: ViewerController,
}

impl ShellController {
    pub fn new() -> Self {
        Self {
            project_controller: ProjectController::new(),
            document_tree_controller: DocumentTreeController::new(),
            tool_controller: ToolController::new(),
            knowledge_controller: KnowledgeController::new(),
            chat_controller: ChatController::new(),
            document_workflow_controller: DocumentWorkflowController::new(),
            viewer_controller: ViewerController::new(),
        }
    }

    pub fn scaffold(&self) -> UiShellScaffold {
        UiShellScaffold::default()
    }

    pub fn initial_view_model(&self) -> ShellViewModel {
        ShellViewModel::default()
    }

    pub fn present_file_intake_batch(
        &self,
        current: &ShellViewModel,
        batch: &io_runtime::FileIntakeBatch,
    ) -> ShellViewModel {
        let labels = load_file_intake_system_labels();

        ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: build_workspace_tabs_for_file_intake(&batch.intake_batch_ref, &labels),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: build_file_intake_system_view_state(batch),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: StatusLineState {
                text: labels.notice,
            },
        }
    }

    pub fn open_project(
        &self,
        current: &ShellViewModel,
        workspace_root: &WorkspaceRoot,
        project_root: ProjectRoot,
    ) -> Result<ShellViewModel, RuntimeFailure> {
        let update = self
            .project_controller
            .open_project(workspace_root, project_root)?;

        Ok(ShellViewModel {
            project_panel: update.project_panel,
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: update.status_line,
        })
    }

    pub fn load_document_tree(
        &self,
        current: &ShellViewModel,
        workspace_root: &WorkspaceRoot,
        project_root: &ProjectRoot,
    ) -> ShellViewModel {
        let update = self
            .document_tree_controller
            .load_tree(workspace_root, project_root);

        ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: update.tree_panel,
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: update.status_line,
        }
    }

    pub fn load_operational_tools(
        &self,
        current: &ShellViewModel,
        project_root: &ProjectRoot,
    ) -> Result<ShellViewModel, ToolRuntimeError> {
        let update = self.tool_controller.load_operational_tools(project_root)?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: update.tool_panel,
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: update.status_line,
        })
    }

    pub fn open_operational_tool_launch(
        &self,
        current: &ShellViewModel,
        project_root: &ProjectRoot,
        catalog_id: &str,
        tool_id: &str,
    ) -> Result<ShellViewModel, ToolRuntimeError> {
        let update =
            self.tool_controller
                .open_operational_tool_launch(project_root, catalog_id, tool_id)?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: update.tool_panel,
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: update.status_line,
        })
    }

    pub fn run_operational_tool(
        &self,
        current: &ShellViewModel,
        project_root: &ProjectRoot,
        catalog_id: &str,
        entry_id: &str,
    ) -> Result<ShellViewModel, ToolRuntimeError> {
        let update =
            self.tool_controller
                .run_operational_tool(project_root, catalog_id, entry_id)?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: update.tool_panel,
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: build_workspace_tabs_for_tool_result(entry_id),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: update.status_line,
        })
    }

    pub fn load_llm_tools(
        &self,
        current: &ShellViewModel,
        project_root: Option<&ProjectRoot>,
    ) -> Result<ShellViewModel, ToolRuntimeError> {
        let update = self.tool_controller.load_llm_tools(project_root)?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: update.tool_panel,
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: update.status_line,
        })
    }

    pub fn set_global_llm_catalog_state(
        &self,
        current: &ShellViewModel,
        project_root: Option<&ProjectRoot>,
        catalog_id: &str,
        enabled: bool,
    ) -> Result<ShellViewModel, ToolRuntimeError> {
        let update =
            self.tool_controller
                .set_global_llm_catalog_state(project_root, catalog_id, enabled)?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: update.tool_panel,
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: update.status_line,
        })
    }

    pub fn set_global_llm_entry_state(
        &self,
        current: &ShellViewModel,
        project_root: Option<&ProjectRoot>,
        catalog_id: &str,
        entry_id: &str,
        enabled: bool,
    ) -> Result<ShellViewModel, ToolRuntimeError> {
        let update = self.tool_controller.set_global_llm_entry_state(
            project_root,
            catalog_id,
            entry_id,
            enabled,
        )?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: update.tool_panel,
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: update.status_line,
        })
    }

    pub fn set_project_llm_catalog_state(
        &self,
        current: &ShellViewModel,
        project_root: &ProjectRoot,
        catalog_id: &str,
        enabled: bool,
    ) -> Result<ShellViewModel, ToolRuntimeError> {
        let update = self.tool_controller.set_project_llm_catalog_state(
            project_root,
            catalog_id,
            enabled,
        )?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: update.tool_panel,
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: update.status_line,
        })
    }

    pub fn set_project_llm_entry_state(
        &self,
        current: &ShellViewModel,
        project_root: &ProjectRoot,
        catalog_id: &str,
        entry_id: &str,
        enabled: bool,
    ) -> Result<ShellViewModel, ToolRuntimeError> {
        let update = self.tool_controller.set_project_llm_entry_state(
            project_root,
            catalog_id,
            entry_id,
            enabled,
        )?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: update.tool_panel,
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: update.status_line,
        })
    }

    pub fn load_knowledge(
        &self,
        current: &ShellViewModel,
        project_root: &ProjectRoot,
    ) -> ShellViewModel {
        let update = self.knowledge_controller.load_knowledge(project_root);

        ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: update.knowledge_panel,
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: update.status_line,
        }
    }

    pub fn open_knowledge_entry(
        &self,
        current: &ShellViewModel,
        workspace_root: &WorkspaceRoot,
        project_root: &ProjectRoot,
        document_id: &str,
    ) -> ShellViewModel {
        let update = self.knowledge_controller.open_knowledge_entry(
            workspace_root,
            project_root,
            document_id,
        );
        let viewer_panel = update
            .viewer_selection
            .as_ref()
            .map(|selection| {
                self.viewer_controller
                    .open_document_source(workspace_root, selection)
            })
            .unwrap_or_else(|| current.viewer_panel.clone());

        ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: update.knowledge_panel,
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: build_workspace_tabs_for_viewer(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel,
            status_line: update.status_line,
        }
    }

    pub fn open_viewer_target(
        &self,
        current: &ShellViewModel,
        workspace_root: &WorkspaceRoot,
        resolved_target: &ResolvedViewerTarget,
    ) -> ShellViewModel {
        let viewer_panel = self
            .viewer_controller
            .open_resolved_target(workspace_root, resolved_target);

        ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: build_workspace_tabs_for_viewer(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel,
            status_line: StatusLineState {
                text: "Viewer target loaded in readonly mode.".to_owned(),
            },
        }
    }

    pub fn open_clip_selector(&self, current: &ShellViewModel) -> ShellViewModel {
        ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: self.chat_controller.open_clip_selector(&current.chat_panel),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: StatusLineState {
                text: "Clip opened with explicit document intentions.".to_owned(),
            },
        }
    }

    pub fn reference_tree_document_in_chat(
        &self,
        current: &ShellViewModel,
        project_root: &ProjectRoot,
        document_id: &str,
    ) -> Result<ShellViewModel, IoRuntimeError> {
        let update = self
            .document_tree_controller
            .select_document(project_root, document_id)?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: update.tree_panel,
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: self
                .chat_controller
                .append_document_reference_from_tree(&current.chat_panel, &update.document),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: StatusLineState {
                text: "Existing workspace document referenced in chat.".to_owned(),
            },
        })
    }

    pub fn open_tree_document_in_viewer(
        &self,
        current: &ShellViewModel,
        workspace_root: &WorkspaceRoot,
        project_root: &ProjectRoot,
        document_id: &str,
    ) -> Result<ShellViewModel, DocumentWorkflowError> {
        let update = self
            .document_tree_controller
            .select_document(project_root, document_id)?;
        let selection =
            viewer_selection_for_project_document(workspace_root, project_root, &update.document)?;
        let viewer_panel = self
            .viewer_controller
            .open_document_source(workspace_root, &selection);

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: update.tree_panel,
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: build_workspace_tabs_for_viewer(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel,
            status_line: StatusLineState {
                text: "Existing workspace document opened in readonly viewer.".to_owned(),
            },
        })
    }

    pub fn profile_tree_document(
        &self,
        current: &ShellViewModel,
        workspace_root: &WorkspaceRoot,
        project_root: &ProjectRoot,
        document_id: &str,
    ) -> Result<ShellViewModel, DocumentWorkflowError> {
        let update = self
            .document_tree_controller
            .select_document(project_root, document_id)?;
        let profile = self
            .document_workflow_controller
            .profile_existing_document(&update.document);
        let selection =
            viewer_selection_for_project_document(workspace_root, project_root, &update.document)?;
        let viewer_panel = self
            .viewer_controller
            .open_document_source(workspace_root, &selection);

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: update.tree_panel,
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: profile.workspace_tabs,
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: profile.profile_panel,
            viewer_panel,
            status_line: StatusLineState {
                text: "Document profile workflow opened from the tree with readonly viewer."
                    .to_owned(),
            },
        })
    }

    pub fn run_operational_tool_on_tree_document(
        &self,
        current: &ShellViewModel,
        project_root: &ProjectRoot,
        document_id: &str,
        catalog_id: &str,
        tool_id: &str,
    ) -> Result<ShellViewModel, DocumentWorkflowError> {
        let tree_update = self
            .document_tree_controller
            .select_document(project_root, document_id)?;
        let tool_update = self
            .tool_controller
            .open_operational_tool_launch_for_documents(
                project_root,
                catalog_id,
                tool_id,
                &[tree_update.document],
            )?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: tree_update.tree_panel,
            tool_panel: tool_update.tool_panel,
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: tool_update.status_line,
        })
    }

    pub fn promote_tree_document_to_knowledge(
        &self,
        current: &ShellViewModel,
        project_root: &ProjectRoot,
        document_id: &str,
    ) -> Result<ShellViewModel, DocumentWorkflowError> {
        let tree_update = self
            .document_tree_controller
            .select_document(project_root, document_id)?;
        let promoted = self
            .document_workflow_controller
            .promote_existing_document(project_root, &tree_update.document)?;
        let knowledge_refresh = self.knowledge_controller.load_knowledge(project_root);

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: knowledge_refresh.knowledge_panel,
            chat_panel: self.chat_controller.append_project_resource_reference(
                &current.chat_panel,
                &promoted,
                ChatDocumentOriginView::KnowledgePromotion,
                "Document promoted to governed knowledge and referenced in chat.",
            ),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: StatusLineState {
                text: "Existing workspace document promoted to knowledge.".to_owned(),
            },
        })
    }

    pub fn add_files_as_chat_context(
        &self,
        current: &ShellViewModel,
        chat_root: &std::path::Path,
        source_files: &[std::path::PathBuf],
    ) -> Result<ShellViewModel, IoRuntimeError> {
        let mut resources = Vec::new();
        for source_file in source_files {
            resources.push(register_chat_resource(chat_root, source_file)?);
        }

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: self
                .chat_controller
                .append_chat_context_resources(&current.chat_panel, &resources),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: StatusLineState {
                text: "External files imported as chat context references.".to_owned(),
            },
        })
    }

    pub fn clip_profile_document(
        &self,
        current: &ShellViewModel,
        workspace_root: &WorkspaceRoot,
        project_root: &ProjectRoot,
        source_file: &std::path::Path,
    ) -> Result<ShellViewModel, DocumentWorkflowError> {
        let imported = self
            .document_workflow_controller
            .import_and_profile_document(project_root, source_file)?;
        let selection = viewer_selection_for_project_document(
            workspace_root,
            project_root,
            &imported.document,
        )?;
        let viewer_panel = self
            .viewer_controller
            .open_document_source(workspace_root, &selection);

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: build_workspace_tabs_for_document_profile(
                imported.document.document_id(),
                imported.document.display_name(),
            ),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: imported.profile_panel,
            viewer_panel,
            status_line: StatusLineState {
                text: "Document profiling workflow opened from clip intake.".to_owned(),
            },
        })
    }

    pub fn clip_promote_to_knowledge(
        &self,
        current: &ShellViewModel,
        workspace_root: &WorkspaceRoot,
        project_root: &ProjectRoot,
        source_file: &std::path::Path,
    ) -> Result<ShellViewModel, DocumentWorkflowError> {
        let promoted = self
            .document_workflow_controller
            .promote_external_document(project_root, source_file)?;
        let knowledge_refresh = self.knowledge_controller.load_knowledge(project_root);
        let selection =
            viewer_selection_for_project_document(workspace_root, project_root, &promoted)?;
        let viewer_panel = self
            .viewer_controller
            .open_document_source(workspace_root, &selection);

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: knowledge_refresh.knowledge_panel,
            chat_panel: self.chat_controller.append_project_resource_reference(
                &current.chat_panel,
                &promoted,
                ChatDocumentOriginView::KnowledgePromotion,
                "External document promoted to knowledge and referenced in chat.",
            ),
            workspace_tabs: build_workspace_tabs_for_viewer(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel,
            status_line: StatusLineState {
                text: "External document promoted to knowledge.".to_owned(),
            },
        })
    }

    pub fn clip_add_as_project_resource(
        &self,
        current: &ShellViewModel,
        project_root: &ProjectRoot,
        source_file: &std::path::Path,
    ) -> Result<ShellViewModel, DocumentWorkflowError> {
        let resource = self
            .document_workflow_controller
            .add_external_project_resource(project_root, source_file)?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: current.tool_panel.clone(),
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: self.chat_controller.append_project_resource_reference(
                &current.chat_panel,
                &resource,
                ChatDocumentOriginView::ProjectResource,
                "External document imported as a governed project resource.",
            ),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: StatusLineState {
                text: "External document added as project resource.".to_owned(),
            },
        })
    }

    pub fn clip_run_operational_tool_on_documents(
        &self,
        current: &ShellViewModel,
        project_root: &ProjectRoot,
        source_files: &[std::path::PathBuf],
        catalog_id: &str,
        tool_id: &str,
    ) -> Result<ShellViewModel, DocumentWorkflowError> {
        let mut imported_documents = Vec::new();
        for source_file in source_files {
            imported_documents.push(
                self.document_workflow_controller
                    .add_external_project_resource(project_root, source_file)?,
            );
        }
        let tool_update = self
            .tool_controller
            .open_operational_tool_launch_for_documents(
                project_root,
                catalog_id,
                tool_id,
                &imported_documents,
            )?;

        Ok(ShellViewModel {
            project_panel: current.project_panel.clone(),
            document_tree_panel: current.document_tree_panel.clone(),
            tool_panel: tool_update.tool_panel,
            knowledge_panel: current.knowledge_panel.clone(),
            chat_panel: current.chat_panel.clone(),
            workspace_tabs: current.workspace_tabs.clone(),
            pipeline_ontology_view: current.pipeline_ontology_view.clone(),
            file_intake_system_view: current.file_intake_system_view.clone(),
            document_profile_panel: current.document_profile_panel.clone(),
            viewer_panel: current.viewer_panel.clone(),
            status_line: tool_update.status_line,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectViewUpdate {
    project_panel: ProjectPanelState,
    status_line: StatusLineState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTreeViewUpdate {
    tree_panel: DocumentTreePanelState,
    status_line: StatusLineState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTreeSelectionUpdate {
    tree_panel: DocumentTreePanelState,
    document: ProjectDocumentRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolViewUpdate {
    tool_panel: ToolPanelState,
    status_line: StatusLineState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeViewUpdate {
    knowledge_panel: KnowledgePanelState,
    status_line: StatusLineState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeSelectionUpdate {
    knowledge_panel: KnowledgePanelState,
    viewer_selection: Option<ViewerDocumentSelection>,
    status_line: StatusLineState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentProfileViewUpdate {
    profile_panel: DocumentProfilePanelState,
    workspace_tabs: WorkspaceTabsState,
    status_line: StatusLineState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedProfileDocument {
    document: ProjectDocumentRef,
    profile_panel: DocumentProfilePanelState,
}

fn legacy_quarantined_error(message: &'static str) -> DocumentWorkflowError {
    DocumentWorkflowError::LegacyQuarantined(message)
}

fn manifest_tree_empty_panel_state() -> DocumentTreePanelState {
    DocumentTreePanelState {
        title: "Document tree".to_owned(),
        entries: Vec::new(),
        message: "No manifest-governed project-visible artifacts are currently declared."
            .to_owned(),
    }
}

fn manifest_tree_error_panel_state(message: &str) -> DocumentTreePanelState {
    DocumentTreePanelState {
        title: "Document tree".to_owned(),
        entries: Vec::new(),
        message: format!("manifest tree error: {message}"),
    }
}

fn manifest_tree_view_update(rows: &[ManifestDocumentTreeRow]) -> DocumentTreeViewUpdate {
    let tree_panel = if rows.is_empty() {
        manifest_tree_empty_panel_state()
    } else {
        map_manifest_document_tree_rows_to_panel_state(rows, None)
    };

    DocumentTreeViewUpdate {
        tree_panel,
        status_line: StatusLineState {
            text: "Document tree is derived from manifest-governed exposure.".to_owned(),
        },
    }
}

/// Build a new shell component populated with scaffold and default view state.
pub fn create_shell() -> Result<RustPortableAppShell, slint::PlatformError> {
    let shell = RustPortableAppShell::new()?;
    let copy_feedback_timer = Rc::new(slint::Timer::default());

    apply_scaffold(&shell, &UiShellScaffold::default());
    apply_shell_view_model(&shell, &ShellViewModel::default());

    {
        let shell_handle = shell.as_weak();
        let copy_feedback_timer = Rc::clone(&copy_feedback_timer);
        shell.on_request_copy_block(move |block_id| {
            let Some(shell) = shell_handle.upgrade() else {
                return;
            };
            let current = chat_panel_state_from_shell(&shell);
            let controller = ChatController::new();
            let Some(update) = controller.request_copy_block(&current, block_id.as_str()) else {
                return;
            };
            match &update.intent {
                ChatBlockActionIntent::CopyBlockRequested { .. } => {}
                ChatBlockActionIntent::ForkBlockRequested { .. } => {}
                ChatBlockActionIntent::ToggleCollapseRequested { .. } => {}
            }

            if let Some(copied_text) = update.copied_text.as_deref() {
                let _ = clipboard_win::set_clipboard_string(copied_text);
            }

            apply_chat_panel_state(&shell, &update.next_panel);

            let shell_handle = shell.as_weak();
            copy_feedback_timer.start(
                slint::TimerMode::SingleShot,
                Duration::from_millis(1200),
                move || {
                    let Some(shell) = shell_handle.upgrade() else {
                        return;
                    };
                    clear_copy_feedback_in_shell(&shell);
                },
            );
        });
    }

    {
        let shell_handle = shell.as_weak();
        shell.on_request_fork_block(move |block_id| {
            let Some(shell) = shell_handle.upgrade() else {
                return;
            };
            let current = chat_panel_state_from_shell(&shell);
            let controller = ChatController::new();
            let Some(update) = controller.request_fork_block(&current, block_id.as_str()) else {
                return;
            };
            match &update.intent {
                ChatBlockActionIntent::ForkBlockRequested { .. } => {}
                ChatBlockActionIntent::CopyBlockRequested { .. } => {}
                ChatBlockActionIntent::ToggleCollapseRequested { .. } => {}
            }
            apply_chat_panel_state(&shell, &update.next_panel);
        });
    }

    {
        let shell_handle = shell.as_weak();
        shell.on_request_toggle_collapse_block(move |block_id| {
            let Some(shell) = shell_handle.upgrade() else {
                return;
            };
            let current = chat_panel_state_from_shell(&shell);
            let controller = ChatController::new();
            let Some(update) =
                controller.request_toggle_collapse_block(&current, block_id.as_str())
            else {
                return;
            };
            match &update.intent {
                ChatBlockActionIntent::ToggleCollapseRequested { .. } => {}
                ChatBlockActionIntent::CopyBlockRequested { .. } => {}
                ChatBlockActionIntent::ForkBlockRequested { .. } => {}
            }
            apply_chat_panel_state(&shell, &update.next_panel);
        });
    }

    Ok(shell)
}

/// Apply the static shell scaffold to the current Slint component.
pub fn apply_scaffold(shell: &RustPortableAppShell, scaffold: &UiShellScaffold) {
    let chat_action_labels = load_chat_action_labels();
    let file_intake_labels = load_file_intake_system_labels();

    shell.set_window_title(slint::SharedString::from(scaffold.window_title.as_str()));
    shell.set_control_bar_text(slint::SharedString::from(
        scaffold.control_bar_text.as_str(),
    ));
    shell.set_open_project_label(slint::SharedString::from(
        scaffold.open_project_label.as_str(),
    ));
    shell.set_clip_label(slint::SharedString::from(scaffold.clip_label.as_str()));
    shell.set_accept_tool_label(slint::SharedString::from(
        scaffold.accept_tool_label.as_str(),
    ));
    shell.set_lume_help_label(slint::SharedString::from(
        scaffold.lume_help_label.as_str(),
    ));
    shell.set_project_panel_title(slint::SharedString::from(
        scaffold.project_panel_title.as_str(),
    ));
    shell.set_project_panel_body(slint::SharedString::from(
        scaffold.project_panel_body.as_str(),
    ));
    shell.set_tree_panel_title(slint::SharedString::from(
        scaffold.tree_panel_title.as_str(),
    ));
    shell.set_tree_panel_body(slint::SharedString::from(scaffold.tree_panel_body.as_str()));
    shell.set_tool_panel_title(slint::SharedString::from(
        scaffold.tool_panel_title.as_str(),
    ));
    shell.set_operational_tools_label(slint::SharedString::from("Operational Tools"));
    shell.set_llm_tools_label(slint::SharedString::from("LLM Tools"));
    shell.set_tool_modal_title(slint::SharedString::from("Operational tool launch"));
    shell.set_tool_modal_body(slint::SharedString::from("No launch modal open."));
    shell.set_knowledge_panel_title(slint::SharedString::from(
        scaffold.knowledge_panel_title.as_str(),
    ));
    shell.set_knowledge_panel_body(slint::SharedString::from(
        scaffold.knowledge_panel_body.as_str(),
    ));
    shell.set_chat_panel_title(slint::SharedString::from(
        scaffold.chat_panel_title.as_str(),
    ));
    shell.set_chat_panel_body(slint::SharedString::from(scaffold.chat_panel_body.as_str()));
    shell.set_chat_blocks(slint::ModelRc::new(slint::VecModel::from(Vec::<ChatBlockRow>::new())));
    shell.set_chat_action_copy_label(slint::SharedString::from(chat_action_labels.copy.as_str()));
    shell.set_chat_action_copied_label(slint::SharedString::from(
        chat_action_labels.copied.as_str(),
    ));
    shell.set_chat_action_fork_label(slint::SharedString::from(chat_action_labels.fork.as_str()));
    shell.set_chat_action_fork_pending_label(slint::SharedString::from(
        chat_action_labels.fork_pending.as_str(),
    ));
    shell.set_chat_action_collapse_label(slint::SharedString::from(
        chat_action_labels.collapse.as_str(),
    ));
    shell.set_chat_action_expand_label(slint::SharedString::from(
        chat_action_labels.expand.as_str(),
    ));
    shell.set_workspace_panel_title(slint::SharedString::from(
        scaffold.workspace_panel_title.as_str(),
    ));
    shell.set_workspace_panel_body(slint::SharedString::from(
        scaffold.workspace_panel_body.as_str(),
    ));
    shell.set_pipeline_view_title(slint::SharedString::from(
        scaffold.pipeline_view_title.as_str(),
    ));
    shell.set_pipeline_view_body(slint::SharedString::from(
        scaffold.pipeline_view_body.as_str(),
    ));
    shell.set_file_intake_view_title(slint::SharedString::from(
        file_intake_labels.title.as_str(),
    ));
    shell.set_file_intake_view_body(slint::SharedString::from(
        file_intake_labels.empty.as_str(),
    ));
    shell.set_viewer_panel_title(slint::SharedString::from(
        scaffold.viewer_panel_title.as_str(),
    ));
    shell.set_viewer_panel_body(slint::SharedString::from(
        scaffold.viewer_panel_body.as_str(),
    ));
    shell.set_status_text(slint::SharedString::from(scaffold.status_text.as_str()));
    apply_lume_help_popup(shell, &LumeHelpPopupState::default());
}

/// Apply the current shell view model to the Slint shell.
pub fn apply_shell_view_model(shell: &RustPortableAppShell, model: &ShellViewModel) {
    let file_intake_labels = load_file_intake_system_labels();
    shell.set_project_panel_title(slint::SharedString::from(
        model.project_panel.title.as_str(),
    ));
    shell.set_project_panel_body(slint::SharedString::from(model.project_panel.body.as_str()));
    shell.set_tree_panel_title(slint::SharedString::from(
        model.document_tree_panel.title.as_str(),
    ));
    shell.set_tree_panel_body(slint::SharedString::from(tree_panel_body(
        &model.document_tree_panel,
    )));
    shell.set_tool_panel_title(slint::SharedString::from(model.tool_panel.title.as_str()));
    shell.set_operational_tools_label(slint::SharedString::from(tool_tab_label(
        model.tool_panel.active_tab,
        ToolPanelTab::OperationalTools,
    )));
    shell.set_llm_tools_label(slint::SharedString::from(tool_tab_label(
        model.tool_panel.active_tab,
        ToolPanelTab::LlmTools,
    )));
    shell.set_tool_panel_body(slint::SharedString::from(tool_panel_body(
        &model.tool_panel,
    )));
    shell.set_tool_modal_title(slint::SharedString::from(
        model.tool_panel.launch_modal.title.as_str(),
    ));
    shell.set_tool_modal_body(slint::SharedString::from(tool_modal_body(
        &model.tool_panel.launch_modal,
    )));
    shell.set_knowledge_panel_title(slint::SharedString::from(
        model.knowledge_panel.title.as_str(),
    ));
    shell.set_knowledge_panel_body(slint::SharedString::from(knowledge_panel_body(
        &model.knowledge_panel,
    )));
    apply_chat_panel_state(shell, &model.chat_panel);
    shell.set_workspace_panel_title(slint::SharedString::from("Workspace tabs"));
    shell.set_workspace_panel_body(slint::SharedString::from(workspace_panel_body(
        &model.workspace_tabs,
        &model.document_profile_panel,
    )));
    shell.set_pipeline_view_title(slint::SharedString::from(
        model.pipeline_ontology_view.title.as_str(),
    ));
    shell.set_pipeline_view_body(slint::SharedString::from(pipeline_ontology_view_body(
        &model.pipeline_ontology_view,
    )));
    shell.set_file_intake_view_title(slint::SharedString::from(
        file_intake_labels.title.as_str(),
    ));
    shell.set_file_intake_view_body(slint::SharedString::from(file_intake_system_view_body(
        &model.file_intake_system_view,
        &file_intake_labels,
    )));
    shell.set_viewer_panel_title(slint::SharedString::from(model.viewer_panel.title.as_str()));
    shell.set_viewer_panel_body(slint::SharedString::from(viewer_panel_body(
        &model.viewer_panel,
    )));
    shell.set_status_text(slint::SharedString::from(model.status_line.text.as_str()));
}

/// Apply the Lume Help popup state without granting execution authority.
pub fn apply_lume_help_popup(shell: &RustPortableAppShell, state: &LumeHelpPopupState) {
    shell.set_lume_help_visible(state.is_visible);
    shell.set_lume_help_title(slint::SharedString::from(state.title.as_str()));
    shell.set_lume_help_subtitle(slint::SharedString::from(state.subtitle.as_str()));
    shell.set_lume_help_body(slint::SharedString::from(state.body.as_str()));
    shell.set_lume_help_gui_objects_title(slint::SharedString::from(
        state.gui_objects_title.as_str(),
    ));
    shell.set_lume_help_gui_objects(slint::SharedString::from(
        format_lume_help_gui_objects(&state.gui_objects).as_str(),
    ));
    shell.set_lume_help_suggestions(slint::SharedString::from(
        format_lume_help_suggestions(&state.suggestions).as_str(),
    ));
    shell.set_lume_help_input_placeholder(slint::SharedString::from(
        state.input_placeholder.as_str(),
    ));
    shell.set_lume_help_close_label(slint::SharedString::from(state.close_label.as_str()));
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChatActionLabels {
    copy: String,
    copied: String,
    fork: String,
    fork_pending: String,
    collapse: String,
    expand: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileIntakeSystemLabels {
    title: String,
    empty: String,
    notice: String,
    batch_ref: String,
    item_count: String,
    owner_ref: String,
    trace_ref: String,
    status: String,
    sanitization: String,
    comment: String,
    blocking_reason: String,
    source_label: String,
    status_candidate: String,
    status_blocked: String,
    status_imported_not_exposed: String,
}

fn apply_chat_panel_state(shell: &RustPortableAppShell, state: &ChatPanelState) {
    shell.set_chat_panel_title(slint::SharedString::from(state.title.as_str()));
    shell.set_chat_panel_body(slint::SharedString::from(chat_panel_body(state)));
    shell.set_chat_blocks(slint::ModelRc::new(slint::VecModel::from(
        chat_block_rows(state),
    )));
}

fn clear_copy_feedback_in_shell(shell: &RustPortableAppShell) {
    let blocks = collect_chat_block_rows(shell);
    shell.set_chat_blocks(slint::ModelRc::new(slint::VecModel::from(
        blocks
            .into_iter()
            .map(|mut block| {
                block.copy_feedback_visible = false;
                block
            })
            .collect::<Vec<_>>(),
    )));
}

fn chat_panel_state_from_shell(shell: &RustPortableAppShell) -> ChatPanelState {
    ChatPanelState {
        title: shell.get_chat_panel_title().to_string(),
        messages: collect_chat_block_rows(shell)
            .into_iter()
            .map(|block| ChatMessageViewState {
                message_id: block.block_id.to_string(),
                kind: ChatMessageKind::Text,
                title: block.title.to_string(),
                body: block.body.to_string(),
                document_id: None,
                logical_path: None,
                document_origin: None,
                copy_feedback_visible: block.copy_feedback_visible,
                fork_feedback_visible: block.fork_feedback_visible,
                is_collapsed: block.is_collapsed,
            })
            .collect(),
        clip_selector: ClipIntentSelectorState::default(),
        message: shell.get_chat_panel_body().to_string(),
    }
}

fn collect_chat_block_rows(shell: &RustPortableAppShell) -> Vec<ChatBlockRow> {
    let model = shell.get_chat_blocks();
    let mut rows = Vec::new();
    for index in 0..model.row_count() {
        if let Some(row) = model.row_data(index) {
            rows.push(row);
        }
    }
    rows
}

fn load_chat_action_labels() -> ChatActionLabels {
    let preferred = match preferred_ui_language() {
        UiLanguage::Spanish => include_str!("../../../resources/i18n/es/chat.ftl"),
        UiLanguage::English => include_str!("../../../resources/i18n/en/chat.ftl"),
    };
    let fallback = include_str!("../../../resources/i18n/en/chat.ftl");

    ChatActionLabels {
        copy: ftl_value(preferred, fallback, "chat.action.copy"),
        copied: ftl_value(preferred, fallback, "chat.action.copied"),
        fork: ftl_value(preferred, fallback, "chat.action.fork"),
        fork_pending: ftl_value(preferred, fallback, "chat.action.fork_pending"),
        collapse: ftl_value(preferred, fallback, "chat.action.collapse"),
        expand: ftl_value(preferred, fallback, "chat.action.expand"),
    }
}

fn load_file_intake_system_labels() -> FileIntakeSystemLabels {
    let preferred = match preferred_ui_language() {
        UiLanguage::Spanish => include_str!("../../../resources/i18n/es/common.ftl"),
        UiLanguage::English => include_str!("../../../resources/i18n/en/common.ftl"),
    };
    let fallback = include_str!("../../../resources/i18n/en/common.ftl");

    FileIntakeSystemLabels {
        title: ftl_value(preferred, fallback, "system_view.file_intake.title"),
        empty: ftl_value(preferred, fallback, "system_view.file_intake.empty"),
        notice: ftl_value(preferred, fallback, "system_view.file_intake.notice"),
        batch_ref: ftl_value(preferred, fallback, "system_view.file_intake.batch_ref"),
        item_count: ftl_value(preferred, fallback, "system_view.file_intake.item_count"),
        owner_ref: ftl_value(preferred, fallback, "system_view.file_intake.owner_ref"),
        trace_ref: ftl_value(preferred, fallback, "system_view.file_intake.trace_ref"),
        status: ftl_value(preferred, fallback, "system_view.file_intake.status"),
        sanitization: ftl_value(preferred, fallback, "system_view.file_intake.sanitization"),
        comment: ftl_value(preferred, fallback, "system_view.file_intake.comment"),
        blocking_reason: ftl_value(
            preferred,
            fallback,
            "system_view.file_intake.blocking_reason",
        ),
        source_label: ftl_value(preferred, fallback, "system_view.file_intake.source_label"),
        status_candidate: ftl_value(
            preferred,
            fallback,
            "system_view.file_intake.status.candidate",
        ),
        status_blocked: ftl_value(
            preferred,
            fallback,
            "system_view.file_intake.status.blocked",
        ),
        status_imported_not_exposed: ftl_value(
            preferred,
            fallback,
            "system_view.file_intake.status.imported_not_exposed",
        ),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiLanguage {
    English,
    Spanish,
}

fn preferred_ui_language() -> UiLanguage {
    for key in ["LC_ALL", "LANG", "LANGUAGE"] {
        if let Ok(value) = std::env::var(key) {
            if value.to_ascii_lowercase().starts_with("es") {
                return UiLanguage::Spanish;
            }
        }
    }

    UiLanguage::English
}

fn ftl_value(preferred: &str, fallback: &str, key: &str) -> String {
    lookup_ftl_value(preferred, key)
        .or_else(|| lookup_ftl_value(fallback, key))
        .unwrap_or_else(|| key.to_owned())
}

fn lookup_ftl_value(contents: &str, key: &str) -> Option<String> {
    contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .find_map(|line| {
            let (candidate_key, candidate_value) = line.split_once('=')?;
            (candidate_key.trim() == key).then(|| candidate_value.trim().to_owned())
        })
}

fn map_project_runtime_output_to_project_panel_state(
    output: &ProjectRuntimeOutput,
) -> ProjectPanelState {
    ProjectPanelState {
        title: "Project panel".to_owned(),
        body: format!(
            "project_id={}\nmanifest_id={}\nvalidation={}\nsurface_entries={}\nresolved_viewer_targets={}",
            output.identity().project_id(),
            output.contract().manifest_id,
            validation_status_text(output.validation().status),
            output.surface().entries().len(),
            output.resolved_viewer_targets().len()
        ),
    }
}

fn map_project_documents_to_tree_panel_state(
    documents: &[ProjectDocumentRef],
    selected_document_id: Option<&str>,
) -> DocumentTreePanelState {
    if documents.is_empty() {
        return DocumentTreePanelState::default();
    }

    DocumentTreePanelState {
        title: "Document tree".to_owned(),
        entries: documents
            .iter()
            .map(|document| DocumentTreeEntryViewState {
                document_id: document.document_id().to_owned(),
                display_name: document.display_name().to_owned(),
                logical_path: document.logical_path().portable_text(),
                area: map_project_document_area(document.area()),
                exposed_object_ref: None,
                file_ref: None,
                owner_ref: None,
                trace_ref: None,
                is_selected: selected_document_id == Some(document.document_id()),
                available_actions: tree_actions_for_document(document.area()),
            })
            .collect(),
        message: "Existing documents are referenced from the tree.".to_owned(),
    }
}

fn map_manifest_document_tree_rows_to_panel_state(
    rows: &[ManifestDocumentTreeRow],
    selected_document_id: Option<&str>,
) -> DocumentTreePanelState {
    DocumentTreePanelState {
        title: "Document tree".to_owned(),
        entries: rows
            .iter()
            .map(|row| DocumentTreeEntryViewState {
                document_id: row.manifest_entry_id().to_owned(),
                display_name: safe_manifest_display_label(row),
                logical_path: manifest_logical_path(row),
                area: map_manifest_document_tree_area(row),
                exposed_object_ref: Some(row.exposed_object_ref().to_owned()),
                file_ref: Some(row.file_ref().to_owned()),
                owner_ref: Some(row.owner_ref().to_owned()),
                trace_ref: Some(row.trace_ref().to_owned()),
                is_selected: selected_document_id == Some(row.manifest_entry_id()),
                available_actions: Vec::new(),
            })
            .collect(),
        message: "Project-visible artifacts are derived from project_manifest authority."
            .to_owned(),
    }
}

fn map_operational_tools_to_panel_state(
    effective_meta_catalog: &tool_runtime::EffectiveMetaCatalog,
) -> ToolPanelState {
    ToolPanelState {
        title: "Tool panel".to_owned(),
        active_tab: ToolPanelTab::OperationalTools,
        operational_tools: effective_meta_catalog
            .domains()
            .iter()
            .flat_map(|catalog| {
                catalog
                    .tools()
                    .iter()
                    .map(|tool| OperationalToolRowState {
                        label: tool.label().to_owned(),
                        tool_id: tool.id().to_owned(),
                        catalog_id: catalog.id().to_owned(),
                        kind: ToolEntryKindView::Tool,
                        enabled: ToolEnabledStateView::Enabled,
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
        llm_catalogs: Vec::new(),
        llm_tools: Vec::new(),
        launch_modal: ToolLaunchModalState::default(),
        message: "Operational tools are ready for controlled manual launch.".to_owned(),
    }
}

fn build_phase_blocked_llm_tool_panel_state() -> ToolPanelState {
    ToolPanelState {
        title: "Tool panel".to_owned(),
        active_tab: ToolPanelTab::LlmTools,
        operational_tools: Vec::new(),
        llm_catalogs: Vec::new(),
        llm_tools: Vec::new(),
        launch_modal: ToolLaunchModalState::default(),
        message:
            "LLM tools remain declarative in F9; UI does not resolve policy or execution."
                .to_owned(),
    }
}

fn map_knowledge_documents_to_panel_state(
    documents: &[KnowledgeDocumentRef],
    selected_document_id: Option<&str>,
) -> KnowledgePanelState {
    if documents.is_empty() {
        return KnowledgePanelState::default();
    }

    KnowledgePanelState {
        title: "Knowledge panel".to_owned(),
        entries: documents
            .iter()
            .map(|document| KnowledgeEntryViewState {
                document_id: document.document_id().to_owned(),
                display_name: document.display_name().to_owned(),
                logical_path: document.logical_path().portable_text(),
                is_viewable: supports_readonly_text_view(&document.logical_path().portable_text()),
                is_selected: selected_document_id == Some(document.document_id()),
            })
            .collect(),
        status: KnowledgePanelStatus::Ready,
        message: "Project knowledge documents are available.".to_owned(),
    }
}

fn map_knowledge_error_to_panel_state(error: &IoRuntimeError) -> KnowledgePanelState {
    KnowledgePanelState {
        title: "Knowledge panel".to_owned(),
        entries: Vec::new(),
        status: KnowledgePanelStatus::Error,
        message: format!("knowledge error: {error}"),
    }
}

fn tool_panel_body(state: &ToolPanelState) -> String {
    match state.active_tab {
        ToolPanelTab::OperationalTools => {
            let mut body = format!(
                "tab=operational\nitems={}\n{}\n",
                state.operational_tools.len(),
                state.message
            );

            for tool in &state.operational_tools {
                body.push_str(&format!(
                    "\n- {} [{}]\n  id={}\n  catalog={}\n  enabled={}",
                    tool.label,
                    tool_entry_kind_view_text(tool.kind),
                    tool.tool_id,
                    tool.catalog_id,
                    tool_enabled_state_text(tool.enabled)
                ));
            }

            body
        }
        ToolPanelTab::LlmTools => {
            let mut body = format!(
                "tab=llm\ncatalogs={}\nentries={}\n{}\n",
                state.llm_catalogs.len(),
                state.llm_tools.len(),
                state.message
            );

            for catalog in &state.llm_catalogs {
                body.push_str(&format!(
                    "\n- catalog {} [{}]\n  global={}\n  project={}\n  effective={}",
                    catalog.label,
                    catalog.catalog_id,
                    tool_enabled_state_text(catalog.global_state),
                    tool_project_override_text(catalog.project_state),
                    tool_enabled_state_text(catalog.effective_state)
                ));
            }

            for tool in &state.llm_tools {
                body.push_str(&format!(
                    "\n- {} [{}]\n  id={}\n  catalog={}\n  global={}\n  project={}\n  effective={}",
                    tool.label,
                    tool_entry_kind_view_text(tool.kind),
                    tool.tool_id,
                    tool.catalog_id,
                    tool_enabled_state_text(tool.global_state),
                    tool_project_override_text(tool.project_state),
                    tool_enabled_state_text(tool.effective_state)
                ));
            }

            body
        }
    }
}

fn tree_panel_body(state: &DocumentTreePanelState) -> String {
    let mut body = format!("items={}\n{}", state.entries.len(), state.message);
    for entry in &state.entries {
        body.push_str(&format!(
            "\n\n- {} [{}]\n  path={}\n  area={}\n  selected={}\n  actions={}",
            entry.display_name,
            entry.document_id,
            entry.logical_path,
            project_document_area_text(entry.area),
            entry.is_selected,
            entry
                .available_actions
                .iter()
                .map(|action| tree_action_text(*action))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        if let Some(exposed_object_ref) = entry.exposed_object_ref.as_deref() {
            body.push_str(&format!("\n  exposed_object_ref={exposed_object_ref}"));
        }
        if let Some(file_ref) = entry.file_ref.as_deref() {
            body.push_str(&format!("\n  file_ref={file_ref}"));
        }
        if let Some(owner_ref) = entry.owner_ref.as_deref() {
            body.push_str(&format!("\n  owner_ref={owner_ref}"));
        }
        if let Some(trace_ref) = entry.trace_ref.as_deref() {
            body.push_str(&format!("\n  trace_ref={trace_ref}"));
        }
    }
    body
}

fn chat_panel_body(state: &ChatPanelState) -> String {
    let mut body = format!("messages={}\n{}", state.messages.len(), state.message);
    if state.clip_selector.is_open {
        body.push_str(&format!(
            "\n\nclip=open\n{}\n{}",
            state.clip_selector.message,
            state
                .clip_selector
                .options
                .iter()
                .map(|option| format!(
                    "- {} [{}] multi={}",
                    option.label,
                    clip_intent_text(option.kind),
                    option.allows_multi_select
                ))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }

    for message in &state.messages {
        body.push_str(&format!(
            "\n\n- {} [{}]\n  {}",
            message.title,
            chat_message_kind_text(message.kind),
            message.body
        ));
    }

    body
}

fn chat_block_rows(state: &ChatPanelState) -> Vec<ChatBlockRow> {
    state
        .messages
        .iter()
            .map(|message| ChatBlockRow {
                block_id: slint::SharedString::from(message.message_id.as_str()),
                title: slint::SharedString::from(message.title.as_str()),
                body: slint::SharedString::from(message.body.as_str()),
                body_preview: slint::SharedString::from(
                    chat_message_collapsed_body(message.body.as_str()).as_str(),
                ),
                copy_payload: slint::SharedString::from(chat_message_copy_payload(message).as_str()),
                copy_feedback_visible: message.copy_feedback_visible,
                fork_feedback_visible: message.fork_feedback_visible,
                is_collapsed: message.is_collapsed,
            })
            .collect()
}

fn chat_panel_with_action_feedback(
    current: &ChatPanelState,
    block_id: &str,
    copy_feedback_visible: bool,
    fork_feedback_visible: bool,
) -> ChatPanelState {
    ChatPanelState {
        title: current.title.clone(),
        messages: current
            .messages
            .iter()
            .cloned()
            .map(|mut message| {
                let is_target = message.message_id == block_id;
                message.copy_feedback_visible = is_target && copy_feedback_visible;
                message.fork_feedback_visible = is_target && fork_feedback_visible;
                message
            })
            .collect(),
        clip_selector: current.clip_selector.clone(),
        message: current.message.clone(),
    }
}

fn chat_panel_with_collapsed_toggle(
    current: &ChatPanelState,
    block_id: &str,
    is_collapsed: bool,
) -> ChatPanelState {
    ChatPanelState {
        title: current.title.clone(),
        messages: current
            .messages
            .iter()
            .cloned()
            .map(|mut message| {
                if message.message_id == block_id {
                    message.is_collapsed = is_collapsed;
                }
                message
            })
            .collect(),
        clip_selector: current.clip_selector.clone(),
        message: current.message.clone(),
    }
}

fn chat_message_copy_payload(message: &ChatMessageViewState) -> String {
    if message.title.is_empty() {
        return message.body.clone();
    }

    if message.body.is_empty() {
        return message.title.clone();
    }

    format!("{}\n{}", message.title, message.body)
}

fn chat_message_collapsed_body(body: &str) -> String {
    const MAX_LINES: usize = 3;

    let mut lines = body.lines();
    let preview = lines
        .by_ref()
        .take(MAX_LINES)
        .collect::<Vec<_>>()
        .join("\n");

    if lines.next().is_some() {
        format!("{preview}\n...")
    } else {
        preview
    }
}

fn workspace_panel_body(tabs: &WorkspaceTabsState, profile: &DocumentProfilePanelState) -> String {
    let mut body = format!(
        "active={}\n{}\n",
        tabs.active_tab_id.as_deref().unwrap_or("none"),
        tabs.message
    );

    for tab in &tabs.tabs {
        body.push_str(&format!(
            "\n- {} [{}] active={}",
            tab.title,
            workspace_tab_kind_text(tab.kind),
            tab.is_active
        ));
    }

    if profile.derivation_status != DocumentProfileStatusView::Empty {
        body.push_str(&format!(
            "\n\nprofile=document_id={}\nstatus={}\nlogical_path={}\ntext_path={}\npages={}\nchunks={}\n{}",
            profile.document_id.as_deref().unwrap_or("none"),
            profile_status_text(profile.derivation_status),
            profile.logical_path.as_deref().unwrap_or("none"),
            profile.extracted_text_path.as_deref().unwrap_or("none"),
            profile.pages_count,
            profile.chunks_count,
            profile.message
        ));
    }

    body
}

fn pipeline_ontology_view_body(state: &PipelineOntologyViewState) -> String {
    let mut body = format!(
        "[Document]\ndocument_id={}\ndocument_name={}\ndocument_class={}\n\n[Chunks]\nchunk_id={}\npreview={}\nsource_hash={}\n\n[PromptSpec]\nprompt_id={}\nstrategy={}\nexecution_enabled={}\n\n[SemanticSpec]\nsemantic_spec_id={}\nsource_layer={}\nsource_kind={}\nexecution_enabled={}\n\n[Semantic Proposals]",
        state.document_id,
        state.document_name,
        state.document_class,
        state.chunk_id,
        state.chunk_preview,
        state.source_hash,
        state.prompt_id,
        state.prompt_strategy,
        state.prompt_execution_enabled,
        state.semantic_spec_id,
        state.semantic_source_layer,
        state.semantic_source_kind,
        state.semantic_execution_enabled
    );

    for proposal in &state.proposals {
        body.push_str(&format!(
            "\nproposal_id: {}\nsubject: {}\npredicate: {}\nobject: {}\ngraph: {}\nstatus: {}\nconfidence: {}\n",
            proposal.proposal_id,
            proposal.subject,
            proposal.predicate,
            proposal.object,
            proposal.graph,
            proposal.status,
            proposal.confidence
        ));
    }

    body.push_str(&format!(
        "\n[Human Review]\nstate={}\nallowed_states={}\n\n[Future RDF/Oxigraph]\nfuture_format={}\nstore={}\noxigraph_enabled={}\nrdf_export_enabled={}\nsemantic_store_enabled={}\n\n[Trace / Governance]\ntrace_id={}\npolicy={}\nllm_mode={}\nexecuted={}\nmutation={}",
        state.human_review_state,
        state.human_review_allowed_states.join(" / "),
        state.future_format,
        state.future_store,
        state.oxigraph_enabled,
        state.rdf_export_enabled,
        state.semantic_store_enabled,
        state.trace_id,
        state.policy,
        state.llm_mode,
        state.executed,
        state.mutation
    ));

    body
}

fn file_intake_system_view_body(
    state: &FileIntakeSystemViewState,
    labels: &FileIntakeSystemLabels,
) -> String {
    let Some(batch_ref) = state.intake_batch_ref.as_deref() else {
        return labels.empty.clone();
    };

    let mut body = format!(
        "{}\n{}={}\n{}={}\n{}={}\n{}={}",
        labels.notice,
        labels.batch_ref,
        batch_ref,
        labels.item_count,
        state.item_count,
        labels.owner_ref,
        state.owner_ref.as_deref().unwrap_or("none"),
        labels.trace_ref,
        state.trace_ref.as_deref().unwrap_or("none")
    );

    for item in &state.items {
        body.push_str(&format!(
            "\n\n- {}\n  {}={}\n  {}={}\n  {}={}\n  {}={}\n  {}={}",
            item.sanitized_filename,
            labels.source_label,
            item.source_label,
            labels.status,
            file_intake_item_status_text(item.status, labels),
            labels.owner_ref,
            item.owner_ref,
            labels.trace_ref,
            item.trace_ref,
            labels.sanitization,
            item.sanitization_state
        ));

        if let Some(comment) = item.user_comment_preview.as_deref() {
            body.push_str(&format!("\n  {}={}", labels.comment, comment));
        }

        if let Some(reason) = item.blocking_reason.as_deref() {
            body.push_str(&format!("\n  {}={}", labels.blocking_reason, reason));
        }
    }

    body
}

fn file_intake_item_status_text<'a>(
    status: FileIntakeItemStatusView,
    labels: &'a FileIntakeSystemLabels,
) -> &'a str {
    match status {
        FileIntakeItemStatusView::Candidate => labels.status_candidate.as_str(),
        FileIntakeItemStatusView::Blocked => labels.status_blocked.as_str(),
        FileIntakeItemStatusView::ImportedNotExposed => {
            labels.status_imported_not_exposed.as_str()
        }
    }
}

fn tool_modal_body(state: &ToolLaunchModalState) -> String {
    match state.status {
        ToolLaunchModalStatus::Hidden => "No launch modal open.".to_owned(),
        _ => format!(
            "status={}\nlabel={}\nid={}\ncatalog={}\n{}\ninputs={}\n{}\nartifact={}",
            tool_modal_status_text(state.status),
            state.tool_label,
            state.tool_id,
            state.catalog_id,
            state.description,
            state.inputs_summary,
            state.result_text,
            state.artifact_path.as_deref().unwrap_or("<none>")
        ),
    }
}

fn format_lume_help_suggestions(suggestions: &[String]) -> String {
    if suggestions.is_empty() {
        return "No contextual suggestions declared.".to_owned();
    }

    suggestions
        .iter()
        .map(|suggestion| format!("- {suggestion}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_lume_help_gui_objects(objects: &[ui_core::GuiObjectHelpViewState]) -> String {
    if objects.is_empty() {
        return "No GUI objects declared.".to_owned();
    }

    objects
        .iter()
        .map(|object| {
            format!(
                "- {}: {} {}",
                object.canonical_name, object.short_description, object.not_description
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn knowledge_panel_body(state: &KnowledgePanelState) -> String {
    let mut body = format!(
        "state={}\nentries={}\n{}\n",
        knowledge_status_text(state.status),
        state.entries.len(),
        state.message
    );

    for entry in &state.entries {
        let selection = if entry.is_selected {
            "selected"
        } else {
            "idle"
        };
        let viewable = if entry.is_viewable {
            "viewable"
        } else {
            "unsupported"
        };
        body.push_str(&format!(
            "\n- {} [{} | {}]\n  id={}\n  path={}",
            entry.display_name, selection, viewable, entry.document_id, entry.logical_path
        ));
    }

    body
}

fn tool_tab_label(active: ToolPanelTab, target: ToolPanelTab) -> &'static str {
    match (active, target) {
        (ToolPanelTab::OperationalTools, ToolPanelTab::OperationalTools) => "[Operational Tools]",
        (ToolPanelTab::LlmTools, ToolPanelTab::LlmTools) => "[LLM Tools]",
        (_, ToolPanelTab::OperationalTools) => "Operational Tools",
        (_, ToolPanelTab::LlmTools) => "LLM Tools",
    }
}

fn viewer_panel_body(state: &ViewerPanelState) -> String {
    let target_ref = state.target_ref.as_deref().unwrap_or("<none>");
    let target_path = state.target_path.as_deref().unwrap_or("<none>");

    format!(
        "state={}\ntarget_ref={}\ntarget_path={}\n\n{}",
        viewer_status_text(state.status),
        target_ref,
        target_path,
        state.content_text
    )
}

fn knowledge_document_source(
    workspace_root: &WorkspaceRoot,
    project_root: &ProjectRoot,
    document: &KnowledgeDocumentRef,
) -> Result<DocumentSource, workspace_core::WorkspaceError> {
    let absolute_path = project_root.join(document.logical_path());
    let workspace_path = workspace_root.relative_path_for(absolute_path)?;

    DocumentSource::new(
        DocumentCategory::Resource,
        document.document_id(),
        workspace_path,
    )
    .map_err(|_| workspace_core::WorkspaceError::InvalidRelativePath {
        input: document.logical_path().portable_text(),
        reason: "knowledge document could not be converted into a workspace source",
    })
}

fn viewer_selection_for_project_document(
    workspace_root: &WorkspaceRoot,
    project_root: &ProjectRoot,
    document: &ProjectDocumentRef,
) -> Result<ViewerDocumentSelection, WorkspaceError> {
    let absolute_path = project_root.join(document.logical_path());
    let workspace_path = workspace_root.relative_path_for(absolute_path)?;
    let source = DocumentSource::new(
        DocumentCategory::Resource,
        document.document_id(),
        workspace_path,
    )
    .map_err(|_| WorkspaceError::InvalidRelativePath {
        input: document.logical_path().portable_text(),
        reason: "project document could not be converted into a workspace source",
    })?;

    Ok(ViewerDocumentSelection {
        title: document.display_name().to_owned(),
        target_ref: document.document_id().to_owned(),
        source,
        authority: SourceAuthority::Project,
    })
}

fn build_workspace_tabs_for_viewer() -> WorkspaceTabsState {
    WorkspaceTabsState {
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
                is_active: true,
            },
        ],
        active_tab_id: Some("viewer".to_owned()),
        message: "Viewer is the active workspace tab.".to_owned(),
    }
}

fn build_workspace_tabs_for_document_profile(
    document_id: &str,
    display_name: &str,
) -> WorkspaceTabsState {
    WorkspaceTabsState {
        tabs: vec![
            WorkspaceTabViewState {
                tab_id: "viewer".to_owned(),
                title: "Viewer".to_owned(),
                kind: WorkspaceTabKind::Viewer,
                is_active: false,
            },
            WorkspaceTabViewState {
                tab_id: format!("profile:{document_id}"),
                title: format!("Profile: {display_name}"),
                kind: WorkspaceTabKind::DocumentProfile,
                is_active: true,
            },
        ],
        active_tab_id: Some(format!("profile:{document_id}")),
        message: "Document profiling is active in the workspace.".to_owned(),
    }
}

fn build_workspace_tabs_for_tool_result(entry_id: &str) -> WorkspaceTabsState {
    WorkspaceTabsState {
        tabs: vec![
            WorkspaceTabViewState {
                tab_id: "viewer".to_owned(),
                title: "Viewer".to_owned(),
                kind: WorkspaceTabKind::Viewer,
                is_active: false,
            },
            WorkspaceTabViewState {
                tab_id: format!("tool:{entry_id}"),
                title: format!("Tool result: {entry_id}"),
                kind: WorkspaceTabKind::ToolResult,
                is_active: true,
            },
        ],
        active_tab_id: Some(format!("tool:{entry_id}")),
        message: "Operational tool result is active in the workspace.".to_owned(),
    }
}

fn build_workspace_tabs_for_file_intake(
    batch_ref: &str,
    labels: &FileIntakeSystemLabels,
) -> WorkspaceTabsState {
    WorkspaceTabsState {
        tabs: vec![
            WorkspaceTabViewState {
                tab_id: "viewer".to_owned(),
                title: "Viewer".to_owned(),
                kind: WorkspaceTabKind::Viewer,
                is_active: false,
            },
            WorkspaceTabViewState {
                tab_id: format!("file_intake:{batch_ref}"),
                title: format!("{}: {batch_ref}", labels.title),
                kind: WorkspaceTabKind::FileIntakeSystem,
                is_active: true,
            },
        ],
        active_tab_id: Some(format!("file_intake:{batch_ref}")),
        message: labels.notice.clone(),
    }
}

fn default_clip_selector_state() -> ClipIntentSelectorState {
    ClipIntentSelectorState {
        is_open: true,
        options: vec![
            ClipIntentOptionViewState {
                kind: ClipIntentKind::AddAsChatContext,
                label: "Add as chat context".to_owned(),
                allows_multi_select: true,
            },
            ClipIntentOptionViewState {
                kind: ClipIntentKind::ProfileDocument,
                label: "Profile document".to_owned(),
                allows_multi_select: false,
            },
            ClipIntentOptionViewState {
                kind: ClipIntentKind::PromoteToKnowledge,
                label: "Promote to knowledge".to_owned(),
                allows_multi_select: false,
            },
            ClipIntentOptionViewState {
                kind: ClipIntentKind::RunOperationalToolOnDocuments,
                label: "Run operational tool on document(s)".to_owned(),
                allows_multi_select: true,
            },
            ClipIntentOptionViewState {
                kind: ClipIntentKind::AddAsProjectResource,
                label: "Add as project resource".to_owned(),
                allows_multi_select: false,
            },
        ],
        message: "Clip is explicit intake and workflow launch, not tree reference.".to_owned(),
    }
}

fn tree_actions_for_document(area: ProjectDocumentArea) -> Vec<DocumentTreeActionView> {
    let mut actions = vec![
        DocumentTreeActionView::OpenInViewer,
        DocumentTreeActionView::ReferenceInChat,
        DocumentTreeActionView::ProfileDocument,
        DocumentTreeActionView::RunOperationalTool,
    ];
    if area != ProjectDocumentArea::Knowledge {
        actions.push(DocumentTreeActionView::PromoteToKnowledge);
    }
    actions
}

fn safe_manifest_display_label(row: &ManifestDocumentTreeRow) -> String {
    let label = row.display_label().trim();
    if label.is_empty() {
        return format!("Project-visible artifact {}", row.manifest_entry_id());
    }

    if label.len() >= 3 {
        let bytes = label.as_bytes();
        if bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && (bytes[2] == b'\\' || bytes[2] == b'/')
        {
            return format!("Project-visible artifact {}", row.manifest_entry_id());
        }
    }

    label.to_owned()
}

fn manifest_logical_path(row: &ManifestDocumentTreeRow) -> String {
    format!("manifest_exposure/{}", row.manifest_entry_id())
}

fn map_manifest_document_tree_area(row: &ManifestDocumentTreeRow) -> ProjectDocumentAreaView {
    let artifact_kind = row.artifact_kind().to_ascii_lowercase();
    let content_kind = row.content_kind().to_ascii_lowercase();

    if artifact_kind.contains("knowledge") || content_kind.contains("knowledge") {
        ProjectDocumentAreaView::Knowledge
    } else if artifact_kind.contains("output") || content_kind.contains("output") {
        ProjectDocumentAreaView::Output
    } else {
        ProjectDocumentAreaView::Resource
    }
}

fn format_selected_documents(selected_documents: &[ProjectDocumentRef]) -> String {
    if selected_documents.is_empty() {
        return "No inputs.".to_owned();
    }

    selected_documents
        .iter()
        .map(|document| format!("{} ({})", document.display_name(), document.logical_path()))
        .collect::<Vec<_>>()
        .join(", ")
}

fn validation_status_text(status: app_services::ManifestValidationStatus) -> &'static str {
    match status {
        app_services::ManifestValidationStatus::Valid => "valid",
        app_services::ManifestValidationStatus::Invalid => "invalid",
    }
}

fn entry_kind_text(kind: CatalogEntryKind) -> &'static str {
    match kind {
        CatalogEntryKind::Tool => "tool",
        CatalogEntryKind::Recipe => "recipe",
    }
}

fn execution_outcome_text(outcome: ToolExecutionOutcome) -> &'static str {
    match outcome {
        ToolExecutionOutcome::Accepted => "accepted",
    }
}

fn map_project_document_area(area: ProjectDocumentArea) -> ProjectDocumentAreaView {
    match area {
        ProjectDocumentArea::Knowledge => ProjectDocumentAreaView::Knowledge,
        ProjectDocumentArea::Resource => ProjectDocumentAreaView::Resource,
        ProjectDocumentArea::Output => ProjectDocumentAreaView::Output,
    }
}

fn tool_entry_kind_view_text(kind: ToolEntryKindView) -> &'static str {
    match kind {
        ToolEntryKindView::Tool => "tool",
        ToolEntryKindView::Recipe => "recipe",
    }
}

fn tool_enabled_state_text(state: ToolEnabledStateView) -> &'static str {
    match state {
        ToolEnabledStateView::Enabled => "enabled",
        ToolEnabledStateView::Disabled => "disabled",
    }
}

fn tool_project_override_text(state: ToolProjectOverrideView) -> &'static str {
    match state {
        ToolProjectOverrideView::Inherit => "inherit",
        ToolProjectOverrideView::Enabled => "enabled",
        ToolProjectOverrideView::Disabled => "disabled",
    }
}

fn project_document_area_text(area: ProjectDocumentAreaView) -> &'static str {
    match area {
        ProjectDocumentAreaView::Knowledge => "knowledge",
        ProjectDocumentAreaView::Resource => "resource",
        ProjectDocumentAreaView::Output => "output",
    }
}

fn tree_action_text(action: DocumentTreeActionView) -> &'static str {
    match action {
        DocumentTreeActionView::OpenInViewer => "open_in_viewer",
        DocumentTreeActionView::ReferenceInChat => "reference_in_chat",
        DocumentTreeActionView::ProfileDocument => "profile_document",
        DocumentTreeActionView::RunOperationalTool => "run_operational_tool",
        DocumentTreeActionView::PromoteToKnowledge => "promote_to_knowledge",
    }
}

fn clip_intent_text(kind: ClipIntentKind) -> &'static str {
    match kind {
        ClipIntentKind::AddAsChatContext => "add_as_chat_context",
        ClipIntentKind::ProfileDocument => "profile_document",
        ClipIntentKind::PromoteToKnowledge => "promote_to_knowledge",
        ClipIntentKind::RunOperationalToolOnDocuments => "run_operational_tool_on_documents",
        ClipIntentKind::AddAsProjectResource => "add_as_project_resource",
    }
}

fn chat_message_kind_text(kind: ChatMessageKind) -> &'static str {
    match kind {
        ChatMessageKind::Text => "text",
        ChatMessageKind::DocumentRef => "document_ref",
        ChatMessageKind::ToolResult => "tool_result",
        ChatMessageKind::SystemState => "system_state",
    }
}

fn chat_origin_text(origin: ChatDocumentOriginView) -> &'static str {
    match origin {
        ChatDocumentOriginView::TreeReference => "tree_reference",
        ChatDocumentOriginView::ChatContext => "chat_context",
        ChatDocumentOriginView::KnowledgePromotion => "knowledge_promotion",
        ChatDocumentOriginView::ProjectResource => "project_resource",
    }
}

fn workspace_tab_kind_text(kind: WorkspaceTabKind) -> &'static str {
    match kind {
        WorkspaceTabKind::Home => "home",
        WorkspaceTabKind::Viewer => "viewer",
        WorkspaceTabKind::KnowledgeDetail => "knowledge_detail",
        WorkspaceTabKind::ToolResult => "tool_result",
        WorkspaceTabKind::DocumentProfile => "document_profile",
        WorkspaceTabKind::PipelineOntology => "pipeline_ontology",
        WorkspaceTabKind::FileIntakeSystem => "file_intake_system",
    }
}

fn profile_status_text(status: DocumentProfileStatusView) -> &'static str {
    match status {
        DocumentProfileStatusView::Empty => "empty",
        DocumentProfileStatusView::Pending => "pending",
        DocumentProfileStatusView::Ready => "ready",
        DocumentProfileStatusView::Unsupported => "unsupported",
        DocumentProfileStatusView::Error => "error",
    }
}

fn tool_modal_status_text(status: ToolLaunchModalStatus) -> &'static str {
    match status {
        ToolLaunchModalStatus::Hidden => "hidden",
        ToolLaunchModalStatus::Ready => "ready",
        ToolLaunchModalStatus::Ok => "ok",
        ToolLaunchModalStatus::Error => "error",
    }
}

fn viewer_status_text(status: ViewerPanelStatus) -> &'static str {
    match status {
        ViewerPanelStatus::Empty => "empty",
        ViewerPanelStatus::Ready => "ready",
        ViewerPanelStatus::Unsupported => "unsupported",
        ViewerPanelStatus::Error => "error",
    }
}

fn knowledge_status_text(status: KnowledgePanelStatus) -> &'static str {
    match status {
        KnowledgePanelStatus::Empty => "empty",
        KnowledgePanelStatus::Ready => "ready",
        KnowledgePanelStatus::Error => "error",
    }
}

fn supports_readonly_text_view(path: &str) -> bool {
    let lowercase = path.to_ascii_lowercase();

    lowercase.ends_with(".md")
        || lowercase.ends_with(".txt")
        || lowercase.ends_with(".json")
        || lowercase.ends_with(".toml")
        || lowercase.ends_with(".yaml")
        || lowercase.ends_with(".yml")
        || lowercase.ends_with(".log")
}

#[cfg(test)]
mod tests {
    use super::{
        format_lume_help_suggestions, ChatController, DocumentTreeController,
        KnowledgeController, ProjectController, ShellController, ToolController, UiShellScaffold,
        ViewerController,
    };
    use app_services::open_project_app_service;
    use project_runtime::ManifestExposureEntry;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use ui_core::{
        ChatBlockActionIntent, ChatDocumentOriginView, ChatMessageKind, ChatMessageViewState,
        ChatPanelState, ClipIntentSelectorState, KnowledgePanelStatus, LumeHelpPopupState,
        ToolEnabledStateView, ToolLaunchModalStatus, ToolPanelTab, ViewerPanelStatus,
    };
    use workspace_core::{ProjectRoot, WorkspaceRoot};

    fn write_manifest_exposure_entries(
        workspace_dir: &std::path::Path,
        entries: &[ManifestExposureEntry],
    ) {
        let entries_json = entries
            .iter()
            .map(|entry| {
                format!(
                    concat!(
                        "{{\n",
                        "    \"manifest_entry_id\": \"{manifest_entry_id}\",\n",
                        "    \"exposed_object_ref\": \"{exposed_object_ref}\",\n",
                        "    \"file_ref\": \"{file_ref}\",\n",
                        "    \"content_hash\": \"{content_hash}\",\n",
                        "    \"stored_object_candidate_ref\": \"{stored_object_candidate_ref}\",\n",
                        "    \"source_intake_item_ref\": \"{source_intake_item_ref}\",\n",
                        "    \"exposure_request_ref\": \"{exposure_request_ref}\",\n",
                        "    \"exposure_candidate_ref\": \"{exposure_candidate_ref}\",\n",
                        "    \"confirmation_ref\": \"{confirmation_ref}\",\n",
                        "    \"owner_ref\": \"{owner_ref}\",\n",
                        "    \"trace_ref\": \"{trace_ref}\",\n",
                        "    \"exposed_to_project_at\": \"{exposed_to_project_at}\",\n",
                        "    \"artifact_kind\": \"{artifact_kind}\",\n",
                        "    \"display_label\": \"{display_label}\",\n",
                        "    \"content_kind\": \"{content_kind}\",\n",
                        "    \"metadata_refs\": [\"{metadata_ref}\"],\n",
                        "    \"exposure_state\": \"{exposure_state}\"\n",
                        "  }}"
                    ),
                    manifest_entry_id = entry.manifest_entry_id,
                    exposed_object_ref = entry.exposed_object_ref,
                    file_ref = entry.file_ref,
                    content_hash = entry.content_hash.clone().unwrap_or_default(),
                    stored_object_candidate_ref = entry.stored_object_candidate_ref,
                    source_intake_item_ref = entry.source_intake_item_ref,
                    exposure_request_ref = entry.exposure_request_ref,
                    exposure_candidate_ref = entry.exposure_candidate_ref,
                    confirmation_ref = entry.confirmation_ref,
                    owner_ref = entry.owner_ref,
                    trace_ref = entry.trace_ref,
                    exposed_to_project_at = entry.exposed_to_project_at,
                    artifact_kind = entry.artifact_kind,
                    display_label = entry.display_label,
                    content_kind = entry.content_kind,
                    metadata_ref = entry.metadata_refs.first().cloned().unwrap_or_default(),
                    exposure_state = entry.exposure_state,
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");
        let manifest_contents = format!(
            concat!(
                "{{\n",
                "  \"manifest_id\": \"manifest.alpha\",\n",
                "  \"viewer_target_ref\": \"document.readme\",\n",
                "  \"viewer_target_kind\": \"document\",\n",
                "  \"viewer_relative_path\": \"docs/README.md\",\n",
                "  \"viewer_label\": \"Project README\",\n",
                "  \"manifest_exposure_entries\": [\n",
                "{entries_json}\n",
                "  ]\n",
                "}}"
            ),
            entries_json = entries_json
        );
        create_project_workspace(&workspace_dir.to_path_buf(), &manifest_contents);
    }

    fn sample_manifest_exposure_entry(
        manifest_entry_id: &str,
        file_ref: &str,
        display_label: &str,
    ) -> ManifestExposureEntry {
        ManifestExposureEntry {
            manifest_entry_id: manifest_entry_id.to_owned(),
            exposed_object_ref: format!("exposed_object.{manifest_entry_id}"),
            file_ref: file_ref.to_owned(),
            content_hash: Some(format!("sha256.{manifest_entry_id}")),
            stored_object_candidate_ref: format!("stored_object_candidate.{manifest_entry_id}"),
            source_intake_item_ref: format!("intake_item.{manifest_entry_id}"),
            exposure_request_ref: format!("exposure_request.{manifest_entry_id}"),
            exposure_candidate_ref: format!("exposure_candidate.{manifest_entry_id}"),
            confirmation_ref: format!("confirmation.{manifest_entry_id}"),
            owner_ref: "owner/demo".to_owned(),
            trace_ref: "trace_demo".to_owned(),
            exposed_to_project_at: "2026-05-02T10:05:00Z".to_owned(),
            artifact_kind: "document".to_owned(),
            display_label: display_label.to_owned(),
            content_kind: "text/markdown".to_owned(),
            metadata_refs: vec![format!("metadata.{manifest_entry_id}")],
            exposure_state: "exposed_to_project".to_owned(),
        }
    }

    #[test]
    fn chat_controller_copy_block_marks_only_target_and_returns_payload() {
        let controller = ChatController::new();
        let current = sample_chat_panel_state();

        let update = controller
            .request_copy_block(&current, "msg_0002")
            .expect("copy action update");

        assert_eq!(
            update.intent,
            ChatBlockActionIntent::CopyBlockRequested {
                block_id: String::from("msg_0002"),
            }
        );
        assert_eq!(
            update.copied_text.as_deref(),
            Some("Snippet\nfn main() {\n    println!(\"hi\");\n}")
        );
        assert!(!update.next_panel.messages[0].copy_feedback_visible);
        assert!(update.next_panel.messages[1].copy_feedback_visible);
        assert!(!update.next_panel.messages[1].fork_feedback_visible);
    }

    #[test]
    fn chat_controller_fork_block_marks_only_target_and_emits_intent() {
        let controller = ChatController::new();
        let current = sample_chat_panel_state();

        let update = controller
            .request_fork_block(&current, "msg_0001")
            .expect("fork action update");

        assert_eq!(
            update.intent,
            ChatBlockActionIntent::ForkBlockRequested {
                block_id: String::from("msg_0001"),
            }
        );
        assert!(update.copied_text.is_none());
        assert!(update.next_panel.messages[0].fork_feedback_visible);
        assert!(!update.next_panel.messages[0].copy_feedback_visible);
        assert!(!update.next_panel.messages[1].fork_feedback_visible);
    }

    #[test]
    fn chat_controller_toggle_collapse_updates_only_target_block() {
        let controller = ChatController::new();
        let current = sample_chat_panel_state();

        let update = controller
            .request_toggle_collapse_block(&current, "msg_0002")
            .expect("collapse action update");

        assert_eq!(
            update.intent,
            ChatBlockActionIntent::ToggleCollapseRequested {
                block_id: String::from("msg_0002"),
            }
        );
        assert!(update.copied_text.is_none());
        assert!(!update.next_panel.messages[0].is_collapsed);
        assert!(update.next_panel.messages[1].is_collapsed);
    }

    #[test]
    fn project_controller_consumes_existing_app_service_boundary() {
        let workspace_dir = unique_temp_dir("ui_slint_project_controller");
        let project_dir = create_project_workspace(
            &workspace_dir,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        );
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root =
            ProjectRoot::new(&workspace_root, project_dir.clone()).expect("project root");
        let controller = ProjectController::new();

        let update = controller
            .open_project(&workspace_root, project_root)
            .expect("project update");

        assert!(update.project_panel.body.contains("project_id=alpha"));
        assert!(update
            .project_panel
            .body
            .contains("manifest_id=manifest.alpha"));
        assert!(update
            .project_panel
            .body
            .contains("resolved_viewer_targets=1"));
        assert_eq!(
            update.status_line.text,
            "Project flow loaded through app_services."
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn operational_tools_tab_is_open_by_default_and_lists_tools() {
        let workspace_dir = unique_temp_dir("ui_slint_tool_controller");
        let project_root = create_project_root(&workspace_dir);
        let controller = ToolController::new();

        let update = controller
            .load_operational_tools(&project_root)
            .expect("tool update");

        assert_eq!(update.tool_panel.active_tab, ToolPanelTab::OperationalTools);
        assert_eq!(update.tool_panel.operational_tools.len(), 4);
        assert_eq!(update.tool_panel.operational_tools[0].tool_id, "manifest");
        assert_eq!(
            update.tool_panel.operational_tools[0].enabled,
            ToolEnabledStateView::Enabled
        );
        assert_eq!(
            update.status_line.text,
            "Operational tools loaded through tool_runtime."
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn operational_launch_opens_modal_and_blocks_execution_in_f9_ui_boundary() {
        let workspace_dir = unique_temp_dir("ui_slint_tool_modal");
        let project_root = create_project_root(&workspace_dir);
        let controller = ToolController::new();

        let opened = controller
            .open_operational_tool_launch(&project_root, "project", "manifest")
            .expect("open modal");
        assert_eq!(
            opened.tool_panel.launch_modal.status,
            ToolLaunchModalStatus::Ready
        );
        assert_eq!(opened.tool_panel.launch_modal.tool_id, "manifest");

        let executed = controller
            .run_operational_tool(&project_root, "project", "manifest")
            .expect("run tool");
        assert_eq!(
            executed.tool_panel.launch_modal.status,
            ToolLaunchModalStatus::Error
        );
        assert!(executed
            .tool_panel
            .launch_modal
            .result_text
            .contains("execution_disabled_in_f9_ui_boundary"));
        assert_eq!(
            executed.status_line.text,
            "Operational tool intent captured; execution remains blocked in F9."
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn llm_tools_tab_is_blocked_as_declarative_only_in_f9() {
        let workspace_dir = unique_temp_dir("ui_slint_llm_tools");
        let project_root = create_project_root(&workspace_dir);
        let controller = ToolController::new();

        let initial = controller
            .load_llm_tools(Some(&project_root))
            .expect("load llm tools");
        assert_eq!(initial.tool_panel.active_tab, ToolPanelTab::LlmTools);
        assert!(initial.tool_panel.llm_catalogs.is_empty());
        assert!(initial.tool_panel.llm_tools.is_empty());
        assert!(initial
            .tool_panel
            .message
            .contains("LLM tools remain declarative in F9"));

        let updated = controller
            .set_project_llm_catalog_state(&project_root, "knowledge", false)
            .expect("disable knowledge catalog");
        assert_eq!(
            updated.status_line.text,
            "LLM tools remain declarative and blocked in F9."
        );
        assert!(updated.tool_panel.llm_catalogs.is_empty());
        assert!(updated.tool_panel.llm_tools.is_empty());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn shell_controller_keeps_llm_panel_presentation_separate_from_project_and_viewer_flows() {
        let workspace_dir = unique_temp_dir("ui_slint_llm_shell");
        let project_root = create_project_root(&workspace_dir);
        let controller = ShellController::new();
        let current = controller.initial_view_model();

        let next = controller
            .load_llm_tools(&current, Some(&project_root))
            .expect("load llm tools");

        assert_eq!(next.project_panel.body, current.project_panel.body);
        assert_eq!(next.knowledge_panel.status, current.knowledge_panel.status);
        assert_eq!(next.viewer_panel.status, current.viewer_panel.status);
        assert_eq!(next.tool_panel.active_tab, ToolPanelTab::LlmTools);
        assert!(next.tool_panel.llm_catalogs.is_empty());
        assert!(next
            .tool_panel
            .message
            .contains("LLM tools remain declarative in F9"));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn document_tree_uses_manifest_entries_as_its_only_source() {
        let workspace_dir = unique_temp_dir("ui_slint_tree_manifest");
        let project_root = create_project_root(&workspace_dir);
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        write_manifest_exposure_entries(
            &workspace_dir,
            &[sample_manifest_exposure_entry(
                "manifest_entry.alpha",
                "file_ref.alpha",
                "Alpha README",
            )],
        );
        let controller = DocumentTreeController::new();

        let update = controller.load_tree(&workspace_root, &project_root);

        assert_eq!(update.tree_panel.entries.len(), 1);
        assert_eq!(update.tree_panel.entries[0].document_id, "manifest_entry.alpha");
        assert_eq!(update.tree_panel.entries[0].display_name, "Alpha README");
        assert_eq!(
            update.tree_panel.entries[0].logical_path,
            "manifest_exposure/manifest_entry.alpha"
        );
        assert!(update.tree_panel.entries[0].available_actions.is_empty());
        assert_eq!(
            update.status_line.text,
            "Document tree is derived from manifest-governed exposure."
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn document_tree_empty_manifest_produces_safe_empty_state() {
        let workspace_dir = unique_temp_dir("ui_slint_tree_empty");
        let project_root = create_project_root(&workspace_dir);
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let controller = ShellController::new();

        let next = controller.load_document_tree(
            &controller.initial_view_model(),
            &workspace_root,
            &project_root,
        );

        assert_eq!(
            next.document_tree_panel.message,
            "No manifest-governed project-visible artifacts are currently declared."
        );
        assert!(next.document_tree_panel.entries.is_empty());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn document_tree_keeps_distinct_rows_for_duplicate_file_refs() {
        let workspace_dir = unique_temp_dir("ui_slint_tree_duplicate_rows");
        let project_root = create_project_root(&workspace_dir);
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let controller = ShellController::new();
        write_manifest_exposure_entries(
            &workspace_dir,
            &[
                sample_manifest_exposure_entry(
                    "manifest_entry.alpha",
                    "file_ref.shared",
                    "Alpha README",
                ),
                sample_manifest_exposure_entry(
                    "manifest_entry.beta",
                    "file_ref.shared",
                    "Beta README",
                ),
            ],
        );

        let next = controller.load_document_tree(
            &controller.initial_view_model(),
            &workspace_root,
            &project_root,
        );

        assert_eq!(next.document_tree_panel.entries.len(), 2);
        assert_eq!(
            next.document_tree_panel.entries[0].file_ref.as_deref(),
            Some("file_ref.shared")
        );
        assert_eq!(
            next.document_tree_panel.entries[1].file_ref.as_deref(),
            Some("file_ref.shared")
        );
        assert_ne!(
            next.document_tree_panel.entries[0].document_id,
            next.document_tree_panel.entries[1].document_id
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn document_tree_never_displays_raw_absolute_host_paths() {
        let workspace_dir = unique_temp_dir("ui_slint_tree_sanitized");
        let project_root = create_project_root(&workspace_dir);
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let controller = ShellController::new();
        write_manifest_exposure_entries(
            &workspace_dir,
            &[sample_manifest_exposure_entry(
                "manifest_entry.alpha",
                "file_ref.alpha",
                "C:/Users/demo/private.txt",
            )],
        );

        let next = controller.load_document_tree(
            &controller.initial_view_model(),
            &workspace_root,
            &project_root,
        );

        assert!(next.document_tree_panel.entries.is_empty());
        assert!(next
            .document_tree_panel
            .message
            .contains("manifest tree error"));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn clip_opens_with_five_explicit_intents_and_no_tree_reference_option() {
        let controller = ShellController::new();
        let current = controller.initial_view_model();

        let next = controller.open_clip_selector(&current);

        assert!(next.chat_panel.clip_selector.is_open);
        assert_eq!(next.chat_panel.clip_selector.options.len(), 5);
        assert!(!next
            .chat_panel
            .clip_selector
            .options
            .iter()
            .any(|option| option.label.contains("Reference existing document")));
    }

    #[test]
    fn add_as_chat_context_imports_files_and_keeps_chat_structured() {
        let controller = ShellController::new();
        let current = controller.initial_view_model();
        let (chat_root, upload_root, source_files) =
            create_chat_root_with_uploads("ui_slint_chat_context", 2);

        let next = controller
            .add_files_as_chat_context(&current, &chat_root, &source_files)
            .expect("chat context");

        assert_eq!(next.chat_panel.messages.len(), 2);
        assert!(next
            .chat_panel
            .messages
            .iter()
            .all(|message| message.kind == ChatMessageKind::DocumentRef));
        assert!(chat_root.join("chat.jsonl").exists());

        fs::remove_dir_all(chat_root).expect("cleanup chat root");
        fs::remove_dir_all(upload_root).expect("cleanup uploads root");
    }

    #[test]
    fn clip_profile_document_opens_profile_workflow_and_viewer() {
        let workspace_dir = unique_temp_dir("ui_slint_clip_profile");
        let project_root = create_project_root(&workspace_dir);
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let controller = ShellController::new();
        let current = controller.initial_view_model();
        let external_root = unique_temp_dir("ui_slint_clip_profile_upload");
        let source_file = external_root.join("incoming.md");
        fs::create_dir_all(&external_root).expect("create external root");
        fs::write(&source_file, "# Incoming\nprofile").expect("write source");

        let error = controller
            .clip_profile_document(&current, &workspace_root, &project_root, &source_file)
            .expect_err("clip profile quarantined");
        assert!(error.to_string().contains("quarantined"));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
        fs::remove_dir_all(external_root).expect("cleanup external root");
    }

    #[test]
    fn clip_promote_to_knowledge_creates_governed_knowledge_and_chat_reference() {
        let workspace_dir = unique_temp_dir("ui_slint_clip_knowledge");
        let project_root = create_project_root(&workspace_dir);
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let controller = ShellController::new();
        let current = controller.initial_view_model();
        let external_root = unique_temp_dir("ui_slint_clip_knowledge_upload");
        let source_file = external_root.join("knowledge.md");
        fs::create_dir_all(&external_root).expect("create external root");
        fs::write(&source_file, "# Knowledge").expect("write source");

        let error = controller
            .clip_promote_to_knowledge(&current, &workspace_root, &project_root, &source_file)
            .expect_err("promote knowledge quarantined");
        assert!(error.to_string().contains("quarantined"));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
        fs::remove_dir_all(external_root).expect("cleanup external root");
    }

    #[test]
    fn clip_run_operational_tool_supports_multi_selection() {
        let workspace_dir = unique_temp_dir("ui_slint_clip_tool_multi");
        let project_root = create_project_root(&workspace_dir);
        let controller = ShellController::new();
        let current = controller.initial_view_model();
        let external_root = unique_temp_dir("ui_slint_clip_tool_upload");
        fs::create_dir_all(&external_root).expect("create external root");
        let source_a = external_root.join("a.md");
        let source_b = external_root.join("b.md");
        fs::write(&source_a, "# A").expect("write a");
        fs::write(&source_b, "# B").expect("write b");

        let error = controller
            .clip_run_operational_tool_on_documents(
                &current,
                &project_root,
                &[source_a.clone(), source_b.clone()],
                "project",
                "manifest",
            )
            .expect_err("clip tool multi quarantined");
        assert!(error.to_string().contains("quarantined"));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
        fs::remove_dir_all(external_root).expect("cleanup external root");
    }

    #[test]
    fn viewer_controller_reads_supported_resolved_target_in_readonly_mode() {
        let workspace_dir = unique_temp_dir("ui_slint_viewer_ready");
        let project_dir = create_project_workspace(
            &workspace_dir,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        );
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root =
            ProjectRoot::new(&workspace_root, project_dir.clone()).expect("project root");
        let runtime_result =
            open_project_app_service(&workspace_root, project_root).expect("runtime result");
        let target = &runtime_result.value.resolved_viewer_targets()[0];
        let controller = ViewerController::new();

        let state = controller.open_resolved_target(&workspace_root, target);

        assert_eq!(state.status, ViewerPanelStatus::Ready);
        assert_eq!(state.target_ref.as_deref(), Some("document.readme"));
        assert_eq!(
            state.target_path.as_deref(),
            Some("user/projects/alpha/docs/README.md")
        );
        assert!(state.content_text.contains("# Alpha README"));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn viewer_controller_marks_unsupported_target_without_reading_arbitrary_content() {
        let workspace_dir = unique_temp_dir("ui_slint_viewer_unsupported");
        let project_dir = create_project_workspace(
            &workspace_dir,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.binary",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/manual.pdf",
  "viewer_label": "Manual PDF"
}"#,
        );
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root =
            ProjectRoot::new(&workspace_root, project_dir.clone()).expect("project root");
        let runtime_result =
            open_project_app_service(&workspace_root, project_root).expect("runtime result");
        let target = &runtime_result.value.resolved_viewer_targets()[0];
        let controller = ViewerController::new();

        let state = controller.open_resolved_target(&workspace_root, target);

        assert_eq!(state.status, ViewerPanelStatus::Unsupported);
        assert_eq!(state.target_ref.as_deref(), Some("document.binary"));
        assert!(state.content_text.contains("not supported"));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn viewer_controller_maps_missing_file_to_error_state() {
        let workspace_dir = unique_temp_dir("ui_slint_viewer_error");
        let project_dir = create_project_workspace(
            &workspace_dir,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.missing",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/missing.md",
  "viewer_label": "Missing README"
}"#,
        );
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root =
            ProjectRoot::new(&workspace_root, project_dir.clone()).expect("project root");
        let runtime_result =
            open_project_app_service(&workspace_root, project_root).expect("runtime result");
        let target = &runtime_result.value.resolved_viewer_targets()[0];
        let controller = ViewerController::new();

        let state = controller.open_resolved_target(&workspace_root, target);

        assert_eq!(state.status, ViewerPanelStatus::Error);
        assert_eq!(state.target_ref.as_deref(), Some("document.missing"));
        assert!(state.content_text.contains("viewer error:"));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn knowledge_controller_returns_empty_state_when_project_has_no_knowledge_root() {
        let workspace_dir = unique_temp_dir("ui_slint_knowledge_empty");
        let project_root = create_project_root(&workspace_dir);
        let controller = KnowledgeController::new();

        let update = controller.load_knowledge(&project_root);

        assert_eq!(update.knowledge_panel.status, KnowledgePanelStatus::Empty);
        assert!(update.knowledge_panel.entries.is_empty());
        assert_eq!(update.status_line.text, "Knowledge panel is empty.");

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn knowledge_controller_loads_project_knowledge_through_io_boundary() {
        let workspace_dir = unique_temp_dir("ui_slint_knowledge_ready");
        let project_dir = create_project_workspace(
            &workspace_dir,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        );
        write_knowledge_docs(&project_dir);
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let controller = KnowledgeController::new();

        let update = controller.load_knowledge(&project_root);

        assert_eq!(update.knowledge_panel.status, KnowledgePanelStatus::Ready);
        assert_eq!(update.knowledge_panel.entries.len(), 2);
        assert_eq!(
            update.knowledge_panel.entries[0].document_id,
            "knowledge/guides/setup.md"
        );
        assert!(update.knowledge_panel.entries[0].is_viewable);
        assert_eq!(
            update.status_line.text,
            "Knowledge panel loaded from project knowledge."
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn shell_controller_routes_knowledge_selection_into_existing_viewer() {
        let workspace_dir = unique_temp_dir("ui_slint_knowledge_viewer");
        let project_dir = create_project_workspace(
            &workspace_dir,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        );
        write_knowledge_docs(&project_dir);
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root =
            ProjectRoot::new(&workspace_root, project_dir.clone()).expect("project root");
        let controller = ShellController::new();
        let current = controller.load_knowledge(&controller.initial_view_model(), &project_root);

        let next = controller.open_knowledge_entry(
            &current,
            &workspace_root,
            &project_root,
            "knowledge/overview.md",
        );

        assert_eq!(next.tool_panel.active_tab, current.tool_panel.active_tab);
        assert_eq!(next.project_panel.body, current.project_panel.body);
        assert_eq!(next.knowledge_panel.status, KnowledgePanelStatus::Ready);
        assert!(next
            .knowledge_panel
            .entries
            .iter()
            .any(|entry| entry.document_id == "knowledge/overview.md" && entry.is_selected));
        assert_eq!(next.viewer_panel.status, ViewerPanelStatus::Ready);
        assert_eq!(
            next.viewer_panel.target_path.as_deref(),
            Some("user/projects/alpha/knowledge/overview.md")
        );
        assert!(next.viewer_panel.content_text.contains("# Overview"));
        assert_eq!(
            next.status_line.text,
            "Knowledge document loaded in readonly mode."
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn shell_controller_maps_unknown_knowledge_document_to_error_without_touching_tools() {
        let workspace_dir = unique_temp_dir("ui_slint_knowledge_error");
        let project_dir = create_project_workspace(
            &workspace_dir,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        );
        write_knowledge_docs(&project_dir);
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root =
            ProjectRoot::new(&workspace_root, project_dir.clone()).expect("project root");
        let controller = ShellController::new();
        let current = controller.load_knowledge(&controller.initial_view_model(), &project_root);

        let next = controller.open_knowledge_entry(
            &current,
            &workspace_root,
            &project_root,
            "knowledge/missing.md",
        );

        assert_eq!(next.tool_panel.active_tab, current.tool_panel.active_tab);
        assert_eq!(next.viewer_panel.status, ViewerPanelStatus::Empty);
        assert_eq!(next.knowledge_panel.status, KnowledgePanelStatus::Error);
        assert!(next.knowledge_panel.message.contains("knowledge error:"));
        assert_eq!(
            next.status_line.text,
            "Knowledge document could not be selected."
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn shell_controller_routes_viewer_updates_without_touching_tool_flow() {
        let workspace_dir = unique_temp_dir("ui_slint_shell_viewer");
        let project_dir = create_project_workspace(
            &workspace_dir,
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        );
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root =
            ProjectRoot::new(&workspace_root, project_dir.clone()).expect("project root");
        let runtime_result =
            open_project_app_service(&workspace_root, project_root).expect("runtime result");
        let target = &runtime_result.value.resolved_viewer_targets()[0];
        let controller = ShellController::new();
        let current = controller.initial_view_model();

        let next = controller.open_viewer_target(&current, &workspace_root, target);

        assert_eq!(next.project_panel.body, current.project_panel.body);
        assert_eq!(next.tool_panel.active_tab, current.tool_panel.active_tab);
        assert_eq!(next.viewer_panel.status, ViewerPanelStatus::Ready);
        assert!(next.viewer_panel.content_text.contains("# Alpha README"));
        assert_eq!(
            next.status_line.text,
            "Viewer target loaded in readonly mode."
        );

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn scaffold_exposes_shell_structure_without_runtime_logic() {
        let scaffold = UiShellScaffold::default();

        assert_eq!(scaffold.project_panel_title, "Project panel");
        assert_eq!(scaffold.tree_panel_title, "Document tree");
        assert_eq!(scaffold.tool_panel_title, "Tool panel");
        assert_eq!(scaffold.knowledge_panel_title, "Knowledge panel");
        assert_eq!(scaffold.chat_panel_title, "Chat panel");
        assert_eq!(scaffold.workspace_panel_title, "Workspace tabs");
        assert_eq!(scaffold.viewer_panel_title, "Viewer panel");
        assert_eq!(scaffold.lume_help_label, "Lume Help");
        assert_eq!(
            scaffold.control_bar_text,
            "Phased shell: project, tree, tools, chat, knowledge, and workspace tabs"
        );
    }

    #[test]
    fn lume_help_popup_state_is_ephemeral_and_readonly() {
        let popup = LumeHelpPopupState::visible();

        assert!(popup.is_visible);
        assert_eq!(popup.title, "Lume Help");
        assert!(popup.body.contains("governed contextual help"));
        assert!(popup
            .suggestions
            .iter()
            .any(|suggestion| suggestion.contains("LLM execution")));
        assert!(format_lume_help_suggestions(&popup.suggestions).contains("- Explain this screen"));
    }

    fn sample_chat_panel_state() -> ChatPanelState {
        ChatPanelState {
            title: String::from("Chat panel"),
            messages: vec![
                ChatMessageViewState {
                    message_id: String::from("msg_0001"),
                    kind: ChatMessageKind::Text,
                    title: String::from("Summary"),
                    body: String::from("First block body"),
                    document_id: None,
                    logical_path: None,
                    document_origin: Some(ChatDocumentOriginView::ChatContext),
                    copy_feedback_visible: false,
                    fork_feedback_visible: false,
                    is_collapsed: false,
                },
                ChatMessageViewState {
                    message_id: String::from("msg_0002"),
                    kind: ChatMessageKind::ToolResult,
                    title: String::from("Snippet"),
                    body: String::from("fn main() {\n    println!(\"hi\");\n}"),
                    document_id: None,
                    logical_path: None,
                    document_origin: None,
                    copy_feedback_visible: false,
                    fork_feedback_visible: false,
                    is_collapsed: false,
                },
            ],
            clip_selector: ClipIntentSelectorState::default(),
            message: String::from("Chat stores text, references, and structured results."),
        }
    }

    fn create_project_workspace(workspace_dir: &PathBuf, manifest_contents: &str) -> PathBuf {
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let config_dir = project_dir.join("config");
        let docs_dir = project_dir.join("docs");
        fs::create_dir_all(&config_dir).expect("create config dir");
        fs::create_dir_all(&docs_dir).expect("create docs dir");
        fs::write(config_dir.join("project_manifest.json"), manifest_contents)
            .expect("write manifest");

        let manifest_text = manifest_contents.to_ascii_lowercase();
        if manifest_text.contains("docs/readme.md") {
            fs::write(
                docs_dir.join("README.md"),
                "# Alpha README\ntechnical readonly content",
            )
            .expect("write readme");
        }
        if manifest_text.contains("docs/manual.pdf") {
            fs::write(docs_dir.join("manual.pdf"), "%PDF-1.4").expect("write fake pdf");
        }

        project_dir
    }

    fn write_knowledge_docs(project_dir: &PathBuf) {
        let knowledge_dir = project_dir.join("knowledge");
        let guides_dir = knowledge_dir.join("guides");

        fs::create_dir_all(&guides_dir).expect("create guides dir");
        fs::write(
            knowledge_dir.join("overview.md"),
            "# Overview\nreadonly knowledge",
        )
        .expect("write overview");
        fs::write(guides_dir.join("setup.md"), "# Setup\nreadonly guide").expect("write setup");
    }

    fn create_chat_root_with_uploads(
        prefix: &str,
        count: usize,
    ) -> (PathBuf, PathBuf, Vec<PathBuf>) {
        let chat_root = unique_temp_dir(&format!("{prefix}_chat"));
        let upload_root = unique_temp_dir(&format!("{prefix}_uploads"));
        fs::create_dir_all(&chat_root).expect("create chat root");
        fs::create_dir_all(&upload_root).expect("create upload root");

        let mut source_files = Vec::new();
        for index in 0..count {
            let source_file = upload_root.join(format!("context_{}.md", index + 1));
            fs::write(&source_file, format!("# Context {}\nchat", index + 1))
                .expect("write upload");
            source_files.push(fs::canonicalize(source_file).expect("canonical source"));
        }

        (
            fs::canonicalize(chat_root).expect("canonical chat root"),
            fs::canonicalize(upload_root).expect("canonical upload root"),
            source_files,
        )
    }

    fn create_project_root(workspace_dir: &PathBuf) -> ProjectRoot {
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let config_dir = project_dir.join("config");
        fs::create_dir_all(&config_dir).expect("create project dir");
        fs::write(
            config_dir.join("project_manifest.json"),
            r#"{
  "manifest_id": "manifest.alpha",
  "viewer_target_ref": "document.readme",
  "viewer_target_kind": "document",
  "viewer_relative_path": "docs/README.md",
  "viewer_label": "Project README"
}"#,
        )
        .expect("write manifest");
        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        ProjectRoot::new(&workspace_root, project_dir).expect("project root")
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after unix epoch")
            .as_nanos();
        path.push(format!("rust_portable_app_{prefix}_{nanos}"));
        path
    }
}
