# document_template_validation

## Status

PROPOSAL / F9-ready declarative governance. This document defines future validation semantics only. It does not implement a validator, runtime enforcement, export, LLM calls, tool execution, or filesystem mutation.

## Purpose

Define how future document templates, template drafts, overrides, and document packages may be checked before governed document creation or update.

Validation reports findings. It does not decide UI behavior, execute actions, or mutate documents.

## Validation dimensions

- `structure`: required sections, order, heading hierarchy, document kind.
- `style`: typography, heading style, spacing, citation style.
- `export`: declared targets, profile references, renderer hints.
- `metadata`: title, author, language, version, document kind.
- `guidance`: section instructions, tone, quality checklist, internal notes.
- `references`: internal references, cross-references, bibliography entries, `reference_style_id`.

## Conformity modes

- `strict`: incompatible structure or missing required sections are errors.
- `guided`: deviations produce warnings unless forbidden by policy.
- `free_from_template`: template conformance is advisory after creation.

## Drift classification

- `style_drift`
- `metadata_drift`
- `export_drift`
- `structure_drift`
- `semantic_drift`

Severity levels:

- `info`
- `warning`
- `error`
- `incompatible`

## Minimum future checks

- template id is present and stable
- template id is unique within its source layer
- supported conformity mode is selected
- template ownership and source layer are declared consistently
- system templates are readonly
- user template rename changes `display_name`, not `template_id`
- cloned templates preserve `cloned_from` when derived from a system template
- required structural sections are declared consistently
- override types are known
- style overrides do not degrade structural conformance
- selected `reference_style_id` is declarative and resolvable
- export profiles are declarative and do not imply execution
- `document.md` remains the authoritative editable source
- derived outputs remain regenerable
- UI state is not treated as package truth

## Template operation failure classes

Future validation should report:

- `forbidden_template_operation`: editing, deleting, or renaming a system template, or performing any operation disallowed by policy.
- `invalid_template_name`: empty, unsafe, or policy-incompatible `display_name`.
- `duplicate_template_id`: duplicate stable id within a source layer or unresolved duplicate after precedence.
- `missing_cloned_from`: clone metadata is missing for a template derived from a system template.
- `delete_user_template_requires_confirmation`: user template deletion has associated documents or package references and requires explicit confirmation.

These failures are declarative classes only. They do not implement deletion, renaming, cloning, or filesystem mutation.

## UI boundary

UI may display validation findings in a future phase.

UI must not:

- perform template validation as authority
- hardcode validation rules
- decide policy
- mutate documents
- create packages
- run export

## Forbidden responsibilities

Template validation must not:

- execute export tools
- invoke LLMs
- call tools
- write files
- create a runtime
- duplicate `project_runtime`
- reinterpret project manifests

## Related specs

- `docs/specs/document_templates.md`
- `docs/specs/document_template_resolution.md`
- `docs/specs/document_creation_flow.md`
- `docs/specs/document_template_ui_contract.md`
