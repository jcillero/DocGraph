# document_template_ui_contract

## Status

PROPOSAL / F9-ready declarative governance. This document defines a future UI capture contract only. It does not implement UI behavior, runtime mutation, validation authority, LLM calls, tool execution, or document creation.

## Purpose

Define the boundary for a future popup used to create or edit a template draft.

The popup captures intent. It does not write, validate, decide, execute, or create packages.

The popup may be opened only from a confirmed governed action, such as a Tools Panel action accepted by the user or an LLM suggestion that has been explicitly confirmed. The LLM may suggest opening or using the popup, but it must not open it directly or execute template operations.

See `docs/specs/document_template_designer_tool.md` for the future `TemplateDesignerTool` proposal.

## Normative rule

Template popup = `TemplateDraft` capture surface.

It is not a document editor, validator, resolver, runtime, or project pipeline.

## TemplateDraft conceptual fields

- `draft_id`
- `template_id`
- `conformity_mode`
- `document_title`
- `document_kind`
- `language`
- `metadata_overrides`
- `selected_export_profile_ids`
- `reference_style_id`
- `guidance_overrides`
- optional target package hint

The target package hint is not permission to write files.

## UI responsibilities

UI may:

- list declarative templates provided by a controller or future service
- group templates by `system`, `user`, and future `project` ownership
- display template title, kind, summary, and guidance
- display permitted operations supplied by future policy or controller state
- capture user choices
- capture user intent to create, view, rename, edit, delete, or clone a template
- show readonly warnings or future validation findings
- emit a structured draft request

## Template operation capture

System templates may expose these operations:

- `view`
- `clone`

User templates may expose these operations, when permitted by policy:

- `view`
- `edit`
- `rename`
- `delete`
- `clone`

Project templates are future scoped resources and must follow the same policy-driven operation model.

UI presents allowed operations. UI does not decide permissions.

Template creation, rename, edit, delete, and clone are future governed requests. They are not filesystem operations in F9.

When the request originates from a future `TemplateDesignerTool`, the popup still remains a capture surface only. It does not become the tool runner or execution authority.

## UI forbidden responsibilities

UI must not:

- write files
- delete files
- create document packages
- create template resources
- rename template resources
- clone template resources
- validate templates as authority
- resolve template precedence
- decide whether a template is editable, renameable, deletable, or cloneable
- mutate `document.md`
- generate `template_snapshot.json`
- generate `effective_template.json`
- run export
- invoke LLMs
- execute tools
- act as `TemplateDesignerTool`
- approve actions
- hardcode template rules
- use UI state as document truth

## Text and resources

Visible strings should come from i18n or declarative resources.

Template rules should come from declarative template resources, not from Slint code.

## Relationship to workspace tabs

The popup is a capture surface. It is not a workspace tab, miniapp, editor, or runtime surface.

Future previews or package editors, if introduced, must remain governed by their own specs.

## Related specs

- `docs/specs/document_creation_flow.md`
- `docs/specs/document_template_resolution.md`
- `docs/specs/document_template_validation.md`
- `docs/specs/document_templates.md`
