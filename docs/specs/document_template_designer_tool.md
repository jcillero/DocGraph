# document_template_designer_tool

## Status

PROPOSAL / F9-ready declarative governance. This document defines a future `TemplateDesignerTool` concept only. It does not implement `tool_runtime`, template runtime, LLM execution, filesystem mutation, or UI behavior.

## Purpose

Define how a future governed template design surface may relate to the Tools Panel, LLM suggestions, template popup, and `ActionResolution`.

`TemplateDesignerTool` is a governed entrypoint proposal, not an executable tool in F9.

## Normative rule

TemplateDesignerTool = governed template-management proposal surface.

It does not execute, write files, validate as authority, open itself from LLM output, or bypass `ActionResolution`.

## Future entrypoints

The Tools Panel may expose a future explicit user action to open the template popup.

An LLM may suggest creating or editing a template, but it must not open the popup directly.

LLM output may produce:

- `TemplateProposal`
- `TemplateDraft`
- structured rationale
- suggested operation type

LLM output must not:

- write template files
- mutate `resources/`
- mutate `user/`
- run export
- execute tools
- approve actions

## Future operations

The following operations are future `ActionRequest` classes:

- create template
- clone template
- rename template
- edit template
- delete template

Every operation must pass through `ActionResolution`.

Deleting a user template requires confirmation.

Editing, renaming, or deleting a system template must be blocked.

System templates are readonly and only cloneable.

User templates are editable, renameable, cloneable, and deletable only when future policy allows it.

## Relationship with template popup

The template popup captures user intent and draft data.

The popup may present operations supplied by future policy/controller state, but it must not decide permissions.

The popup must not write files, validate as authority, create resources, clone resources, delete resources, or approve actions.

## Relationship with Tools Panel

Tools Panel may present `TemplateDesignerTool` as a future explicit user entrypoint.

In F9 this is documentation only. No `tool_runtime` entry, runner, catalog mutation, or executable integration is introduced.

## Relationship with ActionResolution

Template design operations are future actions.

`ActionResolution` must evaluate:

- controlled origin
- target template
- source layer
- ownership
- operation type
- risk level
- confirmation need
- blocked system-template mutations

No action may be executed merely because it appears in chat, popup state, or Tools Panel state.

## Forbidden responsibilities

`TemplateDesignerTool` must not:

- implement runtime
- create or modify templates in F9
- touch `project_runtime`
- touch `tool_runtime`
- invoke LLMs
- execute tools
- mutate filesystem
- run export
- hardcode UI permissions
- turn the popup into an editor
- bypass user confirmation

## Related specs

- `docs/specs/document_template_ui_contract.md`
- `docs/specs/document_templates.md`
- `docs/specs/document_template_resolution.md`
- `docs/specs/document_template_validation.md`
- `docs/specs/tools_panel.md`
- `docs/specs/action_resolution.md`
- `docs/specs/chat_document_flows.md`

