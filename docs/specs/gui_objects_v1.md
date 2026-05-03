# gui_objects_v1

## Status

PROPOSAL / F9.5 declarative governance. This spec defines canonical GUI object names for contextual help. It does not implement runtime behavior, LLM calls, tool execution, action creation, document mutation, or persistent help history.

## Purpose

Fix a stable GUI vocabulary so Lume can explain DocGraph without ambiguous spatial language.

Without stable names, contextual help cannot be precise.

## Declarative source

Canonical objects are declared in:

- `resources/help/gui_objects.json`

GUI objects may have stable technical ids for references and localized or canonical display names for presentation.

The help tree must reference technical ids, not localized labels.

Lume Help may reference this glossary through:

- `resources/help/lume_help.json`

## Canonical objects

- `Document Tree`: navigation of existing governed project objects exposed by `project_manifest`. It is not a free filesystem explorer, import surface, or execution surface.
- `Document Tree` prepares governed selection context by family. It does not collapse all selections into one global active object.
- `Clip Panel`: external intake and workflow launch. It is not project storage.
- `Workspace Tabs`: container for active controlled views. It is not a notebook, free docking system, or miniapp surface.
- `Workspace Tabs` reuses one governed tab per governed target reference. It is not a duplicate-tab surface or execution surface.
- `Readonly Viewer`: non-editable governed document viewing. It must be labeled `READONLY`. It is not an editor, patch runtime, or proposal applier.
- `Chat Panel`: coordination, context, and structured references. It is not document storage or an execution engine.
- `Tools Panel`: access to controlled tools. `Operational Tools` are controlled manual launch; `LLM Tools` are declarative policy in F9.5, not execution.
- `Knowledge Panel`: access to knowledge documents. It is not a semantic graph.
- `Pipeline View`: readonly/mock representation of the F9.5 flow `Document -> Chunk -> PromptSpec -> SemanticSpec -> Proposal -> Trace`. It is not real execution.
- `Ontology Proposal View`: representation of semantic proposals. It is proposal, not fact; it is not approval or semantic persistence.
- `Help Menu`: top-level informational menu domain for `Open Lume Help`, `Help Topics`, and `About DocGraph`. It is not a tool surface, settings surface, credential surface, or execution surface.
- `Lume Help`: declarative contextual help. It is not runtime, persistent chat, active LLM, tool runner, or action approver.
- `Help Topics`: declarative hierarchical help navigation. It is not dynamic runtime logic, diagnostics, or execution.
- `About Popup`: product information, version, credits, and license. It is not system access or action surface.
- `Status Bar`: prepared capability and system state summary. It is not an execution surface or control panel.
- `DocGraph Icon`: branding icon in top app chrome. It may open `Lume Help` as navigation intent. It is not an execution control, LLM activator, settings control, or tool runner.
- `Sandbox View`: conditional workspace tab for governed sandbox status and context. It is declarative in F9 and is not a filesystem executor, write-back control, or tool runner.
- `Sandbox node`: governed tree reference to sandbox context. It is not a host-folder browser or mutation control.

The app shell may also expose a minimal top main menu for `File`, `Preferences`, `Tools`, and `Help`. That menu is navigation chrome, not a new functional panel.

## Lume GUI rules

- `RULE-LUME-GUI-001`: Lume must use names defined in `GUI_OBJECTS_v1`.
- `RULE-LUME-GUI-002`: explanations should follow `[GUI Object] + [action] + [state]`.
- `RULE-LUME-GUI-003`: Lume must not introduce undefined alternative names.
- `RULE-LUME-GUI-004`: Lume should adapt the explanation to the active GUI object when context exists.
- `RULE-LUME-GUI-005`: every object should be explainable through what it is, what it allows, what it does not allow, and current state.
- `RULE-LUME-GUI-006`: when a surface is declarative/mock in F9.5, Lume must state no execution, no mutation, no automatic approval, and no semantic runtime.
- `RULE-LUME-GUI-007`: Lume should avoid vague spatial expressions such as "this", "here", "above", or "on the left" unless the canonical GUI object is named.
- `RULE-LUME-GUI-008`: Lume may provide brief definitions and suggestions, but it must not execute actions.

## UI boundary

Lume Help may show a readonly "GUI Objects" / "Objetos de la interfaz" section.

The UI may present canonical names and short descriptions, but must not infer context, create actions, call LLMs, execute tools, mutate documents, or persist help history.

Visible strings should come from i18n/resources, not hardcoded Slint text.

## Related specs

- `docs/specs/lume_help_popup.md`
- `docs/specs/help_menu.md`
- `docs/specs/lume_interaction_model.md`
- `docs/specs/ui_core.md`
