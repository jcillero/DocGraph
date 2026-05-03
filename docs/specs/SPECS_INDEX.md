# SPECS_INDEX

## Status

GOVERNED INDEX.

This file is a navigation and responsibility map for modular specs.

It does not replace the authority of each individual spec, does not merge spec content, and does not open F10.

## Purpose

Avoid dispersion across modular specs.

Clarify which spec governs each responsibility.

Reduce concept duplication by pointing to the correct owner instead of copying rules.

Make dependencies, non-responsibilities, phase status, and runtime effect explicit for Codex consumption.

## Rules

- reuse an existing spec when it already governs the responsibility
- do not duplicate responsibilities across sibling specs
- transversal specs must be referenced, not copied
- consolidation candidates are advisory metadata only
- `specs_index.json` is authoritative for navigation, not a normative source
- individual specs remain the normative source for their owned contracts
- `transformation_core.md` is the conceptual source for transformation vocabulary
- F10 remains not opened by this index

## Modular index

| Spec | Domain | Responsibility | Depends On | Must Not Define | Phase / Runtime |
| --- | --- | --- | --- | --- | --- |
| `action_policy.md` | action/policy | Declares F9 action risk, decision outcomes, human-in-the-loop policy, and blocked actions. | `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | Execution, mutation, unregistered tool authorization | `F9 / declarative_only` |
| `action_resolution.md` | action/policy | Defines declarative ActionRequest-to-trace governance before any future execution. | `action_policy.md`, `flow_control_policy.md`, `pending_action_state.md` | Runner, executor, tool execution, LLM invocation, project_runtime mutation | `F9 / declarative_only` |
| `active_context.md` | workspace/layout | Defines future active-context model derived from selected workspace tab and explicit references. | `workspace_tabs.md`, `document_governed_editing.md` | Stale target fallback, implicit target resolution, execution | `future / future_runtime` |
| `active_object_context.md` | workspace/layout | Governs cross-family selected object context and explicit focus references. | `active_context.md`, `chat_document_flows.md`, `document_tree.md`, `local_sandbox_context.md`, `text_selection.md`, `ui_core.md`, `workspace_tabs.md` | Execution, implicit targets, host-path identity, workspace-tab authority | `F9 / declarative_only` |
| `ai_governance_f9_5.md` | LLM/governance | Declares F9.5 AI governance resources, semantic proposal preparation, disabled flags, and readonly/mock UI surface. | `governance/LLM_RUNTIME_POLICY.md`, `semantic_proposal_layer.md`, `document_text_runtime.md` | LLM execution, embeddings, RDF persistence, SPARQL, document mutation | `F9.5 / declarative_only` |
| `app_main_menu.md` | UI/presentation | Defines governed top-level File, Preferences, Tools, and Help menu domains. | `credentials_policy.md`, `help_menu.md`, `lume_help_popup.md`, `lume_help_tree.md`, `preferences_policy.md`, `project_settings_popup.md`, `project_setup_popup.md`, `tools_panel.md`, `ui_preferences_popups.md` | Tool execution, LLM calls, filesystem mutation, runtime authority | `F9 / presentation_only` |
| `app_status_bar.md` | UI/presentation | Defines bottom status-bar presentation of prepared state, capability hints, and navigation intents. | `credentials_policy.md`, `llm_tool_surface_resolution.md`, `local_sandbox_context.md`, `lume_onboarding_model.md`, `project_profiles.md`, `tools_panel.md` | Execution, secret display, policy inference, runtime mutation | `F9 / presentation_only` |
| `artifact_graph.md` | knowledge/semantic | Defines graph folder as traceable artifact relationship layer, not runtime authority. | `project_folder_layout.md`, `project_runtime.md`, `semantic_proposal_layer.md` | Filesystem inference, project_runtime replacement, RDF persistence, semantic approval | `F9 / declarative_only` |
| `chat_document_flows.md` | workspace/layout | Defines tree, clip, chat, workspace tab interactions, chat references, and chat session folder contract. | `workspace_tabs.md`, `tools_panel.md` | Hidden document store, autonomous tool execution, duplicated document storage | `F8/F9 / declarative_only` |
| `chat_input.md` | UI/presentation | Governs keyboard-first multiline chat input and capture-state boundaries. | `chat_document_flows.md`, `ui_core.md` | Command execution, target resolution, STT integration, provider invocation | `F9 / presentation_only` |
| `credentials_policy.md` | sandbox/preferences/profile/privacy | Defines credential references, secret non-exposure, and non-authority boundaries. | `preferences_policy.md`, `project_settings_popup.md`, `ui_preferences_popups.md` | Project-stored secrets, LLM credential exposure, raw secret display, execution authority | `F9 / declarative_only` |
| `diff_view.md` | documents/templates/export | Defines readonly proposal comparison behavior for future document transformations. | `document_viewer.md`, `patch_preview.md`, `transform_popup.md`, `transformation_core.md` | Editing, patch application, tool execution, timeline memory | `F9 / presentation_only` |
| `document_creation_flow.md` | documents/templates/export | Defines future governed flow for creating Markdown ARTIFACT packages from templates. | `document_templates.md`, `document_package.md`, `action_resolution.md` | Package creation in F9, mutation, export execution, LLM calls | `future / future_runtime` |
| `document_export.md` | documents/templates/export | Defines future export governance from Markdown ARTIFACT packages to derived outputs. | `document_package.md`, `document_references.md` | Pandoc/Tectonic/LibreOffice execution, UI state as export source | `future / future_runtime` |
| `document_governed_editing.md` | documents/templates/export | Defines future governed editing model for ARTIFACT documents. | `active_context.md`, `document_patch_runtime.md`, `action_resolution.md` | Source/derived mutation, UI patch logic, LLM direct file writes | `future / future_runtime` |
| `document_package.md` | documents/templates/export | Defines future DocumentHolder package layout and ownership model. | `document_creation_flow.md`, `document_template_ui_contract.md` | Package runtime, root outputs, source/derived mutation | `future / future_runtime` |
| `document_patch_runtime.md` | documents/templates/export | Defines future patch proposal and application model. | `transform_popup.md`, `transformation_core.md` | Current patch runtime, UI-owned validation, direct mutation | `future / future_runtime` |
| `document_references.md` | documents/templates/export | Defines future governed document references, bibliography, and citation/export style contracts. | `document_templates.md` | Parser runtime, export runtime, LLM calls, document mutation | `future / future_runtime` |
| `document_template_designer_tool.md` | tools/catalogs | Defines future TemplateDesignerTool proposal and relationship to Tools Panel, LLM suggestions, and ActionResolution. | `action_resolution.md`, `chat_document_flows.md`, `document_template_resolution.md`, `document_template_ui_contract.md`, `document_template_validation.md`, `document_templates.md`, `tools_panel.md` | Tool runtime, template runtime, LLM execution, filesystem mutation, UI behavior | `future / future_runtime` |
| `document_template_resolution.md` | documents/templates/export | Defines future declarative rules for selecting templates before governed document creation. | `document_creation_flow.md`, `document_package.md`, `document_template_validation.md`, `document_templates.md` | Resolver runtime, document creation, export, LLM calls, tool execution | `future / future_runtime` |
| `document_template_ui_contract.md` | documents/templates/export | Defines future UI capture boundary for template popup flows. | `document_creation_flow.md`, `document_template_designer_tool.md`, `document_template_resolution.md`, `document_template_validation.md`, `document_templates.md` | UI runtime behavior, validation authority, LLM calls, tool execution | `future / presentation_only` |
| `document_template_validation.md` | documents/templates/export | Defines future validation dimensions and findings model for templates, overrides, and packages. | `document_creation_flow.md`, `document_template_resolution.md`, `document_template_ui_contract.md`, `document_templates.md` | Validator runtime, export runtime, LLM calls, tool execution | `future / future_runtime` |
| `document_templates.md` | documents/templates/export | Defines governed Markdown document template model. | `document_creation_flow.md`, `document_references.md`, `document_template_resolution.md`, `document_template_ui_contract.md`, `document_template_validation.md` | Runtime, export, editor behavior, LLM behavior, tool execution | `future / future_runtime` |
| `document_text_runtime.md` | documents/templates/export | Defines current regenerable text derivation contract over primary documents. | None declared | UI text derivation, semantic approval, LLM execution | `F8/F9 / none` |
| `document_tree.md` | workspace/layout | Governs manifest-exposed project object navigation and selection entrypoints. | `active_context.md`, `active_object_context.md`, `gui_objects_v1.md`, `local_sandbox_context.md`, `workspace_tabs.md` | Filesystem scan, import, mutation, viewer logic, clip behavior | `F9 / presentation_only` |
| `document_viewer.md` | UI/presentation | Governs readonly document rendering, selection presence, and prepared overlay consumption. | `active_object_context.md`, `diff_view.md`, `document_tree.md`, `patch_preview.md`, `text_selection.md`, `transform_popup.md`, `transformation_core.md`, `workspace_tabs.md` | Editing, patch runtime, tool execution, popup/diff internals | `F9 / presentation_only` |
| `external_dependency_manager.md` | tools/catalogs | Declares governed handling of portable external dependency metadata. | `tools_catalogs.md`, `tool_implementation_governance.md` | Download, extraction, execution, UI logic, F10 runtime | `F9 / declarative_only` |
| `flow_control_policy.md` | action/policy | Defines confirmation and flow-control rules. | `action_resolution.md` | Execution, mutation, bypassing approval policy | `F9 / declarative_only` |
| `gui_objects_v1.md` | Lume/help/onboarding | Defines canonical GUI object vocabulary for Lume contextual help. | `help_menu.md`, `lume_help_popup.md`, `lume_interaction_model.md`, `ui_core.md` | Runtime behavior, LLM calls, tool execution, action creation, mutation | `F9.5 / declarative_only` |
| `help_menu.md` | Lume/help/onboarding | Defines governed Help menu entries and informational-only Help surfaces. | `app_main_menu.md`, `gui_objects_v1.md`, `lume_help_popup.md`, `lume_help_tree.md` | Actions, tools, LLM execution, credentials, runtime mutation | `F9 / presentation_only` |
| `how_to_add_a_tool.md` | tools/catalogs | Provides procedural checklist for governed tool onboarding. | `action_resolution.md`, `tool_implementation_governance.md`, `tools_catalogs.md`, `tools_panel.md` | Runtime logic, UI logic, execution authority, architecture redefinition | `F9 / declarative_only` |
| `i18n_audit_consistency.md` | audits/meta | Defines advisory-only interpretation rules for i18n consistency auditing. | `governance/I18N_POLICY.md` | Runtime authority, auto-fix behavior, destructive changes, UI/runtime modification | `F9 / none` |
| `knowledge_panel.md` | knowledge/semantic | Defines minimum UI-facing contract for readonly project knowledge navigation. | `project_runtime.md`, `ui_core.md` | Editing, semantic inference, manifest reinterpretation, runtime authority | `F8/F9 / presentation_only` |
| `llm_core.md` | LLM/governance | Defines inherited LLM contract, input analysis, decomposition, and response synthesis policy for future governed LLM integration. | `governance/LLM_RUNTIME_POLICY.md` | Provider execution in F9, tool execution, document mutation, UI orchestration | `F9/F10 / future_runtime` |
| `llm_agent_prompts.md` | LLM/governance | Defines Lume as the single front-facing assistant identity and governs declarative storage, format, versioning, and privacy constraints for agent prompt resources. | `llm_core.md`, `lume_interaction_model.md`, `llm_tool_surface_resolution.md` | Runtime loading, provider calls, tool execution, prompt embedding in code, second front-facing assistant | `F10_PREP / declarative_only` |
| `llm_tool_runtime.md` | LLM/governance | Defines LLM Tools as governed capability requests, not model-executed functions. | `llm_tool_surface_resolution.md`, `action_resolution.md`, `tools_catalogs.md`, `tool_implementation_governance.md` | Provider calls, tool execution, duplicated tool_runtime, F10 opening | `F9 / declarative_only` |
| `llm_tool_surface_resolution.md` | LLM/governance | Defines future EffectiveToolSurface contract, modes, explainability, and bounded LLM context. | `tools_catalogs.md`, `tool_implementation_governance.md`, `action_resolution.md`, `llm_core.md` | Resolver code, execution authority, raw catalog injection, UI authority | `F10_PREP / future_runtime` |
| `local_sandbox_context.md` | sandbox/preferences/profile/privacy | Defines declarative policy for future authorized local sandbox roots. | `project_profiles.md` | Sandbox runtime, write-back, mutable actions, absolute-path authority | `F9 / declarative_only` |
| `lume_help_popup.md` | Lume/help/onboarding | Defines F9.5 contextual help popup as readonly, ephemeral, and non-executing. | `gui_objects_v1.md`, `help_menu.md`, `lume_help_tree.md`, `lume_interaction_model.md`, `lume_onboarding_modal.md` | Execution, mutation, action approval, provider calls, persistent help history | `F9.5 / presentation_only` |
| `lume_help_tree.md` | Lume/help/onboarding | Defines declarative help-tree navigation, explanation, and procedure state. | `gui_objects_v1.md`, `help_menu.md`, `llm_tool_surface_resolution.md`, `lume_help_popup.md`, `lume_onboarding_model.md` | Execution, policy inference, LLM calls, runtime state persistence | `F9 / presentation_only` |
| `lume_interaction_model.md` | Lume/help/onboarding | Defines Lume as declarative interaction and help layer, not executor. | `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | Filesystem execution, tool execution, LLM execution, project pipeline authority | `F9 / declarative_only` |
| `lume_onboarding_modal.md` | Lume/help/onboarding | Defines declarative onboarding/help modal without creating project, chat persistence, or execution. | `lume_interaction_model.md` | Project creation, LLM calls, tool execution, chat persistence | `F9 / presentation_only` |
| `lume_onboarding_model.md` | Lume/help/onboarding | Defines dual-mode Lume onboarding and capability-state explanation. | `credentials_policy.md`, `llm_core.md`, `llm_tool_surface_resolution.md`, `lume_help_popup.md`, `lume_interaction_model.md`, `lume_onboarding_modal.md`, `project_setup_popup.md` | LLM requirement, credential requirement, tool execution, blocking project setup | `F9 / declarative_only` |
| `patch_preview.md` | documents/templates/export | Defines readonly in-place preview overlay semantics for pending proposals. | `diff_view.md`, `document_viewer.md`, `transform_popup.md`, `transformation_core.md` | Patch application, persistence, execution, diff review ownership | `F9 / presentation_only` |
| `pending_action_state.md` | action/policy | Defines conceptual pending-action state for future explicit action execution references. | `action_resolution.md` | Runner, executor, hidden mutation, automatic execution | `F9 / declarative_only` |
| `preferences_policy.md` | sandbox/preferences/profile/privacy | Defines non-secret preference configuration boundaries and non-authority rules. | `credentials_policy.md`, `project_settings_popup.md`, `ui_preferences_popups.md` | Secrets, execution authority, policy bypass, host-path truth | `F9 / declarative_only` |
| `privacy_and_consent_model.md` | sandbox/preferences/profile/privacy | Defines governed privacy, consent, and bounded-context exposure rules for future Lume and LLM-facing interaction. | `preferences_policy.md`, `llm_core.md`, `lume_interaction_model.md`, `user_profile_policy.md` | Runtime storage, implicit consent is forbidden, credential exposure, tool execution, UI authority | `F9/F10_PREP / declarative_only` |
| `project_folder_layout.md` | project/runtime | Defines normative project folder contract, owner-scoped outputs, and manifest-governed exposure. | `project_runtime.md`, `artifact_graph.md`, `chat_document_flows.md`, `document_package.md`, `tools_panel.md` | Root outputs, filesystem exposure authority, project_runtime replacement | `F9 / declarative_only` |
| `project_profiles.md` | sandbox/preferences/profile/privacy | Defines project-profile declarations as intended capabilities and default policies. | None declared | Execution enabling, project-policy override, tool authority, F10 opening | `F9 / declarative_only` |
| `project_runtime.md` | project/runtime | Defines central project pipeline. | `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | UI logic, tool runtime expansion, duplicated pipeline | `3A-F9 / none` |
| `project_settings_popup.md` | sandbox/preferences/profile/privacy | Defines future project-settings intent capture. | `credentials_policy.md`, `preferences_policy.md`, `ui_preferences_popups.md` | Project config write-back, tool execution, secret display, policy authority | `F9 / presentation_only` |
| `project_setup_popup.md` | sandbox/preferences/profile/privacy | Defines future project setup draft capture. | `action_resolution.md`, `credentials_policy.md`, `local_sandbox_context.md`, `lume_interaction_model.md`, `preferences_policy.md`, `project_folder_layout.md`, `project_profiles.md`, `tool_implementation_governance.md`, `tools_panel.md` | Project creation, folder mutation, LLM invocation, tool execution | `F9 / presentation_only` |
| `runtime_layout.md` | project/runtime | Documents declarative runtime folder layout for future external tools, engines, models, cache, and temp data. | `governance/WORKSPACE_RULES.md` | Runtime materialization, execution, PATH mutation, dependency installation | `F9 / declarative_only` |
| `sandbox_viewer.md` | sandbox/preferences/profile/privacy | Governs readonly sandbox-context inspection as a workspace object. | `active_object_context.md`, `document_tree.md`, `local_sandbox_context.md`, `ui_core.md`, `workspace_tabs.md` | Filesystem exploration, write-back UI, tool launch, mutation | `F9 / presentation_only` |
| `semantic_proposal_layer.md` | knowledge/semantic | Defines SemanticProposal as proposal, not approved knowledge. | `action_resolution.md`, `ai_governance_f9_5.md`, `document_text_runtime.md`, `pending_action_state.md` | LLM execution, embeddings, RDF persistence, Oxigraph, SPARQL, approval runtime | `F9.5 / declarative_only` |
| `semantic_quad_lifecycle.md` | knowledge/semantic | Defines lifecycle states for future semantic quads as proposal governance. | `ai_governance_f9_5.md`, `document_text_runtime.md`, `semantic_proposal_layer.md`, `semantic_quad_model.md` | Semantic approval, RDF persistence, SPARQL, LLM execution | `F9/F10_PREP / declarative_only` |
| `semantic_quad_model.md` | knowledge/semantic | Defines future semantic quad representation for proposal-only semantic preparation. | `action_resolution.md`, `ai_governance_f9_5.md`, `document_text_runtime.md`, `semantic_proposal_layer.md` | Fact creation, RDF persistence, semantic store, LLM execution | `F9/F10_PREP / declarative_only` |
| `semantic_source_scope.md` | knowledge/semantic | Defines source-scope boundaries for semantic proposal preparation. | `architecture/ARCHITECTURE.md`, `governance/FUNCTIONAL_SCOPE.md`, `governance/GOVERNANCE.md`, `document_text_runtime.md`, `semantic_proposal_layer.md`, `semantic_quad_lifecycle.md`, `semantic_quad_model.md`, `tools_catalogs.md` | Raw catalog injection, runtime expansion, knowledge approval, tool execution | `F9/F10_PREP / declarative_only` |
| `spec_runtime.md` | project/runtime | Defines declarative loading and normalized lookup responsibilities for specs, configs, registries, and contracts. | `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | Runtime authority beyond declared lookup, UI policy, project pipeline ownership | `3A-F9 / none` |
| `SPECS_INDEX.md` | audits/meta | Human-readable responsibility map for modular specs. | `specs_index.json` | Normative replacement, F10 opening, runtime behavior, spec-content merging | `global / none` |
| `storage_policy.md` | storage/i18n/theme | Defines F9-ready file-based storage policy, checksum deduplication, and regenerable JSONL index rules. | `governance/WORKSPACE_RULES.md` | Database runtime, hidden mutation, root outputs, non-regenerable authority | `F9 / declarative_only` |
| `structured_content_assets.md` | documents/templates/export | Defines future governed treatment of figures, tables, and equations as structured assets. | `document_package.md`, `document_governed_editing.md` | Editing surfaces, export runtime, inspector runtime, source mutation | `future / future_runtime` |
| `text_selection.md` | documents/templates/export | Governs structured text selection references tied to document integrity and trace. | `active_object_context.md`, `chat_document_flows.md`, `document_viewer.md`, `transform_popup.md`, `transformation_core.md` | Popup UX, diff behavior, execution, independent source truth | `F9 / presentation_only` |
| `tool_implementation_governance.md` | tools/catalogs | Defines rules and invariants for tools and implementation discipline. | `how_to_add_a_tool.md`, `tools_catalogs.md` | UI execution, LLM execution, dependency/tool confusion, ActionResolution bypass | `F9 / declarative_only` |
| `tools_catalogs.md` | tools/catalogs | Defines tool catalog structure and declaration boundaries. | `how_to_add_a_tool.md`, `llm_tool_surface_resolution.md`, `tool_implementation_governance.md` | Runtime execution, catalog-as-runtime, dependency installation, F10 opening | `F9 / declarative_only` |
| `tools_panel.md` | tools/catalogs | Defines UI contract for Operational Tools, LLM Tools, and External Dependencies surfaces. | `action_resolution.md`, `app_main_menu.md`, `external_dependency_manager.md`, `how_to_add_a_tool.md`, `tool_implementation_governance.md`, `tools_catalogs.md` | Tool execution policy, LLM execution, catalog mutation, shared Preferences panel | `F8/F9 / presentation_only` |
| `transform_popup.md` | documents/templates/export | Governs contextual transformation intent capture and proposal-state controls. | `diff_view.md`, `document_governed_editing.md`, `document_patch_runtime.md`, `document_viewer.md`, `text_selection.md`, `transformation_core.md` | Patch application, runtime validation, diff semantics, editing | `F9 / presentation_only` |
| `transform_timeline.md` | documents/templates/export | Governs structured memory of transformation attempts, proposals, and user decisions. | `diff_view.md`, `document_governed_editing.md`, `patch_preview.md`, `transform_popup.md`, `transformation_core.md`, `transformation_recipes.md`, `workspace_tabs.md` | Graph authority, tool manifests, execution, patch application | `F9 / declarative_only` |
| `transformation_core.md` | documents/templates/export | Canonical conceptual source for transformation vocabulary, proposal lineage, overlay, preview, timeline, and recipe concepts. | `active_context.md`, `document_governed_editing.md`, `document_patch_runtime.md`, `workspace_tabs.md` | Viewer UX, popup UX, diff UX, runtime execution details | `F9 / declarative_only` |
| `transformation_recipes.md` | documents/templates/export | Governs reusable declarative transformation patterns derived from timeline or future suggestion. | `transformation_core.md`, `transform_timeline.md` | Script logic, macro execution, runtime orchestration, LLM invocation | `F9 / declarative_only` |
| `ui_core.md` | UI/presentation | Defines UI state model and presentation-only boundaries. | `governance/UI_SLINT_POLICY.md`, `governance/WORKSPACE_RULES.md` | Filesystem ownership, domain policy, LLM calls, tool execution, semantic inference | `F5-F9 / presentation_only` |
| `ui_preferences_popups.md` | sandbox/preferences/profile/privacy | Defines governed preference, credential, and settings popup presentation boundaries and i18n keys. | `app_main_menu.md`, `credentials_policy.md`, `preferences_policy.md`, `project_settings_popup.md` | Execution, secret display, ActionResolution bypass, implied tool/LLM availability | `F9 / presentation_only` |
| `user_profile_policy.md` | sandbox/preferences/profile/privacy | Defines optional, opaque, non-sensitive user-profile references for future Lume and LLM-facing flows. | `preferences_policy.md`, `llm_core.md`, `lume_interaction_model.md`, `privacy_and_consent_model.md` | Sensitive data requirements, runtime persistence, credential exposure, tool execution, authentication runtime | `F9/F10_PREP / declarative_only` |
| `workspace_core.md` | workspace/layout | Defines workspace roots and path discipline. | `governance/WORKSPACE_RULES.md` | Project pipeline ownership, UI policy, host-specific persisted truth | `3A-F9 / none` |
| `workspace_layout.md` | workspace/layout | Defines central layout relationships for tree, chat, tabs workspace, viewer, and conditional sandbox presentation. | `ui_core.md`, `workspace_tabs.md` | Execution logic, project_runtime ownership, file mutation, tools/knowledge replacement | `F9 / presentation_only` |
| `workspace_tabs.md` | workspace/layout | Governs tab identity, non-duplication, mouse interaction, and representation-only tab behavior. | `active_object_context.md`, `document_tree.md`, `document_viewer.md`, `gui_objects_v1.md`, `sandbox_viewer.md`, `transform_timeline.md`, `ui_core.md`, `workspace_layout.md` | Viewer internals, diff behavior, popup behavior, execution, permission decisions | `F8/F9 / presentation_only` |

## Related advisory scripts

| Script | Purpose | Must Not Do |
| --- | --- | --- |
| `dev/scripts/audits/audit_tools_compliance.bat` | Advisory audit for tool onboarding compliance across catalogs, specs, and tool-runtime declarations. | Auto-fix, execute tools, mutate catalogs, open F10 |
| `dev/scripts/audits/audit_i18n_consistency.bat` | Advisory audit for i18n key consistency and best-effort hardcoded text detection. | Auto-fix, mutate i18n files, enforce destructive changes |

## Consolidation Candidates

These groups are advisory only. They do not merge, deprecate, delete, or weaken any current spec.

| Candidate target | Sources | Risk | Preservation rule |
| --- | --- | --- | --- |
| `lume_help_and_onboarding.md` | `lume_help_popup.md`, `lume_help_tree.md`, `lume_onboarding_modal.md`, `lume_onboarding_model.md`, `gui_objects_v1.md`, `help_menu.md` | LOW | Lume remains help/interaction only; no execution, mutation, provider calls, or action approval |
| `tool_governance.md` | `how_to_add_a_tool.md`, selected procedural sections of `tool_implementation_governance.md` | LOW | Preserve declared != executable, UI != executor, LLM != executor, tools != dependencies |
| `preferences_surfaces.md` | `preferences_policy.md`, `project_settings_popup.md`, `project_setup_popup.md`, `ui_preferences_popups.md` | MEDIUM | Keep `credentials_policy.md` separate; preferences do not contain secrets or grant authority |
| `document_template_governance.md` | `document_templates.md`, `document_template_resolution.md`, `document_template_validation.md`, `document_template_ui_contract.md`, `document_creation_flow.md` | MEDIUM | Preserve proposal/future status and no document creation/export runtime |
| `semantic_governance.md` | `ai_governance_f9_5.md`, `semantic_proposal_layer.md`, `semantic_quad_model.md`, `semantic_quad_lifecycle.md`, `semantic_source_scope.md` | MEDIUM | Preserve proposal != fact, no RDF/Oxigraph/SPARQL/embeddings/runtime approval |
| `document_transformation_proposals.md` | `transformation_core.md`, `text_selection.md`, `transform_popup.md`, `patch_preview.md`, `diff_view.md`, `transform_timeline.md`, `transformation_recipes.md` | HIGH | Preserve UI-only/proposal boundaries and keep patch application outside UI |
| `workspace_surfaces.md` | `workspace_layout.md`, `workspace_tabs.md`, `document_tree.md`, `sandbox_viewer.md`, `active_object_context.md`, `active_context.md` | HIGH | Preserve UI presentation only; UI must not decide policy or own runtime authority |

## Old To New Candidate Map

This map is advisory only. It is not a deprecation notice and does not authorize deletion.

| Current spec | Candidate target |
| --- | --- |
| `lume_help_popup.md` | `lume_help_and_onboarding.md` |
| `lume_help_tree.md` | `lume_help_and_onboarding.md` |
| `lume_onboarding_modal.md` | `lume_help_and_onboarding.md` |
| `lume_onboarding_model.md` | `lume_help_and_onboarding.md` |
| `gui_objects_v1.md` | `lume_help_and_onboarding.md` |
| `help_menu.md` | `lume_help_and_onboarding.md` |
| `preferences_policy.md` | `preferences_surfaces.md` |
| `project_settings_popup.md` | `preferences_surfaces.md` |
| `project_setup_popup.md` | `preferences_surfaces.md` |
| `ui_preferences_popups.md` | `preferences_surfaces.md` |
| `credentials_policy.md` | `DO_NOT_MERGE_PRIVACY_BOUNDARY` |
| `how_to_add_a_tool.md` | `tool_governance.md` |
| `tool_implementation_governance.md` | `tool_governance.md` |
| `document_templates.md` | `document_template_governance.md` |
| `document_template_resolution.md` | `document_template_governance.md` |
| `document_template_validation.md` | `document_template_governance.md` |
| `document_template_ui_contract.md` | `document_template_governance.md` |
| `document_creation_flow.md` | `document_template_governance.md` |
| `ai_governance_f9_5.md` | `semantic_governance.md` |
| `semantic_proposal_layer.md` | `semantic_governance.md` |
| `semantic_quad_model.md` | `semantic_governance.md` |
| `semantic_quad_lifecycle.md` | `semantic_governance.md` |
| `semantic_source_scope.md` | `semantic_governance.md` |
| `transformation_core.md` | `document_transformation_proposals.md` |
| `text_selection.md` | `document_transformation_proposals.md` |
| `transform_popup.md` | `document_transformation_proposals.md` |
| `patch_preview.md` | `document_transformation_proposals.md` |
| `diff_view.md` | `document_transformation_proposals.md` |
| `transform_timeline.md` | `document_transformation_proposals.md` |
| `transformation_recipes.md` | `document_transformation_proposals.md` |
| `workspace_layout.md` | `workspace_surfaces.md` |
| `workspace_tabs.md` | `workspace_surfaces.md` |
| `document_tree.md` | `workspace_surfaces.md` |
| `sandbox_viewer.md` | `workspace_surfaces.md` |
| `active_object_context.md` | `workspace_surfaces.md` |
| `active_context.md` | `workspace_surfaces.md` |

## Transversal note

For transformation-related responsibilities:

- `transformation_core.md` defines concepts
- `text_selection.md` defines governed selection reference
- `transform_popup.md` defines intent capture
- `patch_preview.md` defines inline overlay preview
- `diff_view.md` defines comparison review
- `transform_timeline.md` defines memory and trace
- `transformation_recipes.md` defines reusable declarative patterns

These specs should reference each other when needed, but they must not copy each other's responsibilities.
