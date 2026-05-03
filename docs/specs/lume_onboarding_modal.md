# lume_onboarding_modal

## Purpose

Define the declarative onboarding and initial help modal for Lume inside `DocGraph`.

The onboarding modal is not a project, not a persistent chat, and not an execution surface.

Declared does not mean implemented or executed.

`DocGraph` is the visible application identity. `Lume` is the assistant/help identity. The onboarding modal uses these names declaratively and does not create runtime, tool, LLM, filesystem, or project-pipeline authority.

## Scope F9

F9 declares:

- modal id `lume_onboarding_help`
- ephemeral help chat
- startup preference `show_onboarding_modal`
- manual opening from the help menu
- optional intro video placeholder
- i18n keys for greeting, input placeholder, checkbox, and common labels

Onboarding must work in `Static Mode` without LLM.

Onboarding may show capability state.

Onboarding must not imply LLM availability.

The modal may be hidden at startup by user preference, but it must remain manually openable from the help menu.

## UX constraints

The onboarding modal should remain:

- simple
- optional
- skippable
- context-help oriented
- limited to one primary action

The modal is help, not workspace work.

## Forbidden responsibilities

The onboarding modal must not:

- persist chat history
- create project data
- generate file outputs
- execute tools
- invoke real LLM execution
- mutate filesystem
- hardcode visible strings in Slint
- belong to workspace tabs
- save artifacts
- interact with `project_runtime`

## Chat boundary

The onboarding chat is ephemeral help context.

It is not:

- project chat
- document storage
- a workspace tab
- a tool invocation surface
- an LLM execution surface

## Future F10/F11 notes

F10 may route onboarding questions through governed LLM policy only if real execution is explicitly opened.

F11 should verify that disabling startup display does not remove manual help access.
