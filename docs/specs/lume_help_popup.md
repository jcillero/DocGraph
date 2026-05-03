# lume_help_popup

## Purpose

Define the F9.5 `Lume Help` contextual help popup for DocGraph.

Lume Help = capa de ayuda contextual gobernada basada en fuentes declarativas.

Lume Help is part of Lume, the single governed front-facing assistant identity.

It must not introduce another assistant identity or expose internal agent roles directly.

Lume Help can operate as local declarative help when LLM is unavailable.

Missing LLM is a state to explain, not an app failure.

In the governed Help menu model, `Open Lume Help` is one dedicated Help entry.

## Scope F9.5

The popup is a readonly, ephemeral help surface. It may explain current governance, phases, restrictions, and usage rules already declared in documentation or resources.

It is conceptually separate from onboarding:

- onboarding is initial help
- Lume Help is contextual help available during use

## Allowed responsibilities

- present governed help text
- present declared suggestions
- present an input placeholder as a visual affordance
- present navigation hints toward help topics and documentation surfaces
- route future user intent to governed surfaces only after a later phase opens that behavior

## Forbidden responsibilities

- no tool execution
- no LLM calls
- no file mutation
- no action creation
- no action approval
- no ActionResolution implementation
- no RAG engine
- no document search
- no parser of governance or specs
- no conversational memory
- no popup history persistence
- no project data storage
- no workspace tab behavior
- no filesystem access
- no credential access
- no internet calls in F9
- no prompt editing UI
- no internal agent selector
- no alternative assistant identity

## Declarative sources

Current declared sources:

- `resources/help/lume_help.json`
- `resources/help/gui_objects.json`
- `resources/lume/lume_help_tree.json`
- `resources/i18n/es/lume_messages.ftl`
- `resources/i18n/en/lume_messages.ftl`
- `docs/specs/lume_interaction_model.md`
- `docs/specs/lume_onboarding_modal.md`
- `docs/specs/gui_objects_v1.md`
- `docs/specs/llm_agent_prompts.md`

## GUI Objects section

Lume Help may present a readonly `GUI Objects` / `Objetos de la interfaz` section.

The section uses `GUI_OBJECTS_v1` canonical names so explanations avoid ambiguous phrases like "this", "here", or "on the left" without naming the GUI object.

The section is declarative help only. It does not infer active context, create actions, call LLMs, execute tools, persist history, or mutate files.

## Help tree section

Lume Help may render a readonly help tree from `resources/lume/lume_help_tree.json`.

The tree may organize navigation, explanation, and procedure nodes for Static Mode guidance.

The popup renders prepared selection state only. It does not interpret policy, execute actions, or decide permissions from tree content.

The main menu may open `Lume Help` directly or request focus on the help tree branch, but it does not change the popup into an execution surface.

Lume Help may also be opened from the DocGraph top-left icon.

Help menu and DocGraph icon are navigation intents to the same `Lume Help` surface.

The icon does not change Lume into an execution surface.

`Help` opens `Lume Help` as an informational popup only.

The popup must not interact with `Tools`, `Preferences`, credentials, or runtime actions.

## Agent and prompt boundary

Lume Help may explain that future assisted reasoning can use internal Planner, Specialist, and Synthesizer roles.

It must present those roles as internal conceptual roles only.

It must not:

- expose internal roles as separate user-facing agents
- let the user select or invoke internal roles directly
- define prompts
- edit prompts
- persist prompt changes
- embed prompt text in UI state
- call providers to test prompts

## Invariants

- `INV-LUME-HELP-001`: Lume Help is contextual help, not runtime.
- `INV-LUME-HELP-002`: Lume Help is based on governed declarative sources or i18n/help resources.
- `INV-LUME-HELP-003`: Lume Help does not execute, mutate, call LLMs, or approve actions.
- `INV-LUME-HELP-004`: the help popup is ephemeral and does not create project data.
- `INV-LUME-HELP-005`: visible text must come from i18n/resources, not hardcoded Slint views.
- `INV-LUME-HELP-006`: Lume Help uses `GUI_OBJECTS_v1` canonical names when explaining interface objects.
- `INV-LUME-HELP-007`: Lume Help MUST NOT access filesystem, credentials, or internet in F9.
- `INV-LUME-HELP-008`: Lume Help MUST remain informational and MUST NOT trigger `ActionResolution`.
- `INV-LUME-HELP-009`: Lume Help MUST remain part of Lume and MUST NOT introduce another front-facing assistant.
- `INV-LUME-HELP-010`: Lume Help MUST NOT define, edit, persist, or test agent prompts.

## Future notes

A later phase may connect the visual input to governed help routing. That must not open LLM execution, tool execution, filesystem mutation, or project-pipeline changes unless a phase explicitly authorizes them.

## Related specs

- `docs/specs/help_menu.md`
- `docs/specs/lume_help_tree.md`
- `docs/specs/gui_objects_v1.md`
