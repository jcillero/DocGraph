# ui_core

## Purpose

Define UI-facing state and interaction contracts while preserving the inherited rule that UI is representation, not truth.

## Responsibilities

- define view-facing state contracts
- define minimal panel and shell-level presentation models
- consume controller or service-authorized results
- separate representation concerns from runtime and filesystem authority

## Inputs

- controller or service results
- localized text keys
- semantic theme tokens
- manifest-authorized viewer models
- readonly knowledge-listing results from a small technical boundary
- readonly project-document tree results from a small technical boundary
- workspace-tab selection and routing state
- readonly F9.5 pipeline / ontology mock state

## Outputs

- UI-facing state models
- shell scaffold and panel contracts
- workspace-tab, viewer, tree, chat, knowledge, tools, and document-workflow presentation contracts
- pipeline / ontology proposal view contracts
- future readonly `System View` presentation contracts

## Allowed Dependencies

- standard library
- `core_domain`

## Forbidden Responsibilities

- no direct filesystem access
- no direct business logic
- no direct tool execution
- no direct provider invocation
- no direct Slint dependency

## Current Scope

- shell scaffold contracts
- project, tree, tool, viewer, chat, knowledge, workspace-tab, document-profile, and status presentation state
- Lume Help popup presentation state
- Pipeline / Ontology View presentation state for F9.5 semantic proposals
- Lume Help GUI Objects presentation state based on `GUI_OBJECTS_v1`
- separated `Operational Tools` and `LLM Tools` panel state
- prepared LLM capability and interaction-mode state
- no widget implementation
- no domain ownership
- no editing, notebook, semantic inference, semantic approval, or semantic store behavior
- no Lume Help runtime behavior, LLM call, tool call, filesystem mutation, or history persistence

## Active object context

`ui_core` may expose presentation-facing state derived from `ActiveObjectContext`.

Conceptually, UI state may distinguish:

- one selected document or none
- one selected sandbox or none
- one selected knowledge item or none
- one selected artifact or none
- one selected tool output or none
- one focused family or none

This is not pure widget state and not execution authority.

UI may render and route prepared active-object context, but it must not invent fallback targets, grant permissions, or execute actions from that context alone.

## Pipeline / Ontology View

`PipelineOntologyViewState` is a readonly/mock presentation contract for F9.5 AI governance.

It may represent:

- source document identity
- derived chunk identity
- `PromptSpec`
- `SemanticDerivationSpec`
- `SemanticProposal` rows
- human review state
- future RDF/Oxigraph disabled flags
- trace and policy metadata

It must not infer semantics, approve proposals, call LLMs, create embeddings, persist RDF, run SPARQL, execute tools, mutate documents, or reinterpret project manifests.

## Lume Help popup

`LumeHelpPopupState` is a presentation-only contract for the F9.5 Lume Help popup.

It is:

- readonly
- ephemeral
- not a workspace tab
- not project chat
- not a runtime
- not an action approval surface

Slint may present this state. Slint must not derive governance answers, parse specs, call LLMs, execute tools, mutate files, or persist popup history.

`LumeHelpPopupState` may include a readonly GUI Objects section. The canonical source is `resources/help/gui_objects.json`; UI state only presents a minimal view of canonical names and boundaries.

## Chat and document interaction

The UI-facing model now distinguishes:

- existing governed documents referenced from the tree
- external intake and workflow launch from the clip
- structured references and results in chat
- real work happening in workspace tabs and readonly viewer flows
- a common governed workspace workflow even when the entry point differs

This does not imply blob storage inside chat, editing behavior, or notebook growth.

Conceptually, `ui_core` may expose a `ChatInputDraft` plus readonly input-state markers for multiline growth, overflow, microphone availability, recording, and transcription readiness.

These remain presentation contracts only. They must not execute commands, resolve targets implicitly, invoke LLMs, or bind directly to STT providers.

Conceptually, `ui_core` may also expose readonly LLM-tool request state as proposal or intent representation only. It must not execute tools, call providers, or bypass governed tool-surface and action-resolution policies.

## LLM capability presentation

Conceptually, `ui_core` may expose a readonly `LLMCapabilityStateView`.

It presents prepared state only:

- `desired_llm_mode`
- `effective_llm_mode`
- `interaction_mode`
- `llm_available`
- `provider_configured`
- `credential_ref_present`
- `local_engine_available`
- `local_model_available`
- `policy_allows_local`
- `policy_allows_cloud`
- `tool_surface_available`
- `execution_enabled`
- `reason_codes`

Allowed mode values:

- `OFF`
- `LOCAL`
- `CLOUD`

Allowed interaction values:

- `GUIDED`
- `ASSISTED`

UI must not:

- compute capability state
- decide policy
- resolve credentials
- display secrets
- call providers
- load local models
- execute tools
- execute ActionResolution
- mutate files

When `effective_llm_mode=OFF`, UI may present prepared guided menu state, but that menu may emit navigation, explanation, or configuration intent only.

## Workspace tabs

The main content area is a simple workspace tab container.

Minimum conceptual model:

- `WorkspaceTabsState`
- `WorkspaceTabKind`
- `WorkspaceTabRef`
- `WorkspaceTabTargetRef`

Current concrete kinds may include:

- `viewer`
- `sandbox_view`
- `knowledge_detail`
- `system_view`
- `tool_result`
- `home`
- `pipeline_ontology`

This does not imply notebook behavior, editing, docking, or layout orchestration.

The readonly viewer remains one concrete tab kind inside the workspace container.

It verifies resolved content inside the workspace model; it does not become a document owner, editor, or intake surface depending on where the workflow started.

Conceptually, `ui_core` may expose a `DocumentViewerState` as a readonly rendering contract with optional selection state and prepared visual overlays.

That state must not imply editing, persistence, patch application, tool execution, or LLM execution.

Conceptually, prepared overlay rendering may be represented through a readonly `PatchPreviewOverlayState` using semantic highlight tokens rather than hardcoded success/error colors.

Conceptually, readonly comparison may also be represented through a `DiffViewState` that supports inline-by-default review and optional side-by-side comparison on demand.

Conceptually, `ui_core` may expose a `SandboxViewerState` as readonly governed sandbox-context presentation with capability state, descriptive source metadata, and future working-copy placeholders only.

Conceptually, `ui_core` may expose readonly `SystemViewState`, `NothingHappensState`, `BlockedExplanationViewState`, `TraceViewState`, `TraceSummaryViewState`, `EffortViewState`, `EffectiveToolSurfaceSummaryViewState`, `ToolSurfaceGroupViewState`, and `LumeInterpretationViewState` as presentation contracts only, governed by `docs/specs/system_view.md`.

Those states must remain presentation-ready and sanitized:

- no raw payloads
- no secrets
- no private absolute host paths
- no policy inference
- no authority grants
- refs only where governed linking is needed
- visible labels represented through future i18n keys rather than literal strings

Conceptually, `ui_core` may expose one reusable tab per governed target reference.

Opening the same governed target should activate the existing tab state rather than duplicate it.

Mouse interaction over tab headers may be represented as navigation or close intents only. It must not execute tools, mutate files, or decide permissions.

## Future governed editing concepts

The following names are conceptual contracts only. They do not imply implementation in `ui_core` yet.

- `ActiveContextState`: current tab-derived context for document, figure, table, equation, or patch preview work
- `ReferencedPatchRequestState`: structured user request derived from a referenced selection or active inspector context
- `PatchPreviewState`: readonly preview of a validated patch proposal before acceptance
- `StructuredAssetInspectorState`: readonly inspection state for future figure, table, and equation assets

These future states must remain presentation contracts. Patch validation, patch application, filesystem mutation, LLM execution, and tool execution stay outside UI crates.

## Invariants

- `INV-UI-LLM-001`: UI MUST display prepared LLM capability state only.
- `INV-UI-LLM-002`: UI MUST NOT compute effective LLM mode, interaction mode, policy, or reason codes.
- `INV-UI-LLM-003`: UI MUST NOT expose credentials or credential secrets.
- `INV-UI-LLM-004`: UI MUST NOT call providers, load local models, execute tools, execute ActionResolution, or mutate files.
- `INV-UI-LLM-005`: guided menu presentation MUST NOT emit executing intents.
- `INV-UI-LLM-006`: tool visibility in UI MUST NOT imply execution permission.
- `INV-UI-LLM-007`: raw tool catalogs MUST NOT be injected into LLM context by UI state.
