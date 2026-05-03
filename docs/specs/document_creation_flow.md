# document_creation_flow

## Status

PROPOSAL / F9-ready declarative governance. This document defines a future flow only. It does not implement package creation, runtime mutation, export, LLM calls, tool execution, or UI behavior.

## Purpose

Define the future governed flow for creating a Markdown `ARTIFACT` document package from a declarative template.

## Future conceptual flow

1. The user opens a governed template creation popup.
2. UI captures a `TemplateDraft`.
3. UI emits a structured draft request.
4. Future resolution selects a template using `project > user > resources`.
5. Future validation checks the draft against template rules.
6. Future policy/action resolution decides whether confirmation is required.
7. Only an authorized future runtime may materialize the package after acceptance.
8. The created package contains `document.md` as the authoritative editable source.

In F9 this flow is documentation only.

## TemplateDraft boundary

`TemplateDraft` is user intent captured by UI. It is not a document package and is not execution.

It may conceptually include:

- draft id
- selected `template_id`
- selected conformity mode
- document title
- document kind
- language
- metadata overrides
- selected export profile ids
- selected `reference_style_id`
- guidance overrides

## Future package result

A future created package should follow `docs/specs/document_package.md` and include:

- `document.md`
- `document_manifest.json`
- `template_snapshot.json`
- `template_overrides.json`
- `effective_template.json`
- `export_profiles.json`
- `reference_entries.json`
- `reference_style.json`
- `bibliography_sources.json`
- `assets/`
- `derived/latex/`
- `derived/pdf/`
- `derived/docx/`

## Non-execution guarantee

This flow does not allow:

- package creation in F9
- filesystem mutation in F9
- export execution
- LLM calls
- tool execution
- document mutation
- project manifest reinterpretation
- duplication of `project_runtime`

## Related specs

- `docs/specs/document_template_ui_contract.md`
- `docs/specs/document_template_resolution.md`
- `docs/specs/document_template_validation.md`
- `docs/specs/document_package.md`
- `docs/specs/action_resolution.md`

