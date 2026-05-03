# document_templates

## Status

PROPOSAL. This document defines future governed Markdown document templates. It does not implement runtime, export, editor behavior, LLM behavior, or tool execution.

## Purpose

Define a declarative model for creating `ARTIFACT` documents from governed templates.

A document template is a layered contract:

`structure + style + export + references + bibliography + editorial guidance`

Templates are resources. They are not execution logic.

## Core model

Each assisted document creation starts from either:

- an explicit declarative template
- a governed default template

The instantiated document is an `ARTIFACT` whose textual source is Markdown.

`document.md` is the authoritative editable source.

Future LaTeX, PDF, and DOCX outputs are derived and regenerable.

## Template identity and ownership

Every template should declare stable identity and governance metadata:

- `template_id`: stable technical identifier; it must not change when a visible name changes.
- `display_name`: user-facing name; user-owned templates may rename this value through a governed future operation.
- `source_layer`: `resources`, `user`, or `project`.
- `ownership`: `system`, `user`, or `project`.
- `editable`: whether the template body may be changed through future governed operations.
- `deletable`: whether the template may be deleted through future governed operations.
- `renameable`: whether `display_name` may be changed through future governed operations.
- `cloneable`: whether a copy may be created from this template.
- `cloned_from`: optional source template reference for cloned templates.

System templates are readonly and may not be edited, renamed, or deleted. They may be cloneable.

User templates may be created, edited, renamed, deleted, or cloned only when future policy permits it.

Project templates are future scoped resources and must remain governed by project-local policy.

Changing `display_name` must not change `template_id`.

Cloning a system template creates a user or project template copy. The clone must preserve `cloned_from`.

## Conformity modes

- `strict`: only template-compatible changes are accepted.
- `guided`: deviations are allowed with warnings.
- `free_from_template`: the template is used as a starting point, with no strict conformance after creation.

## Template layers

- `structure`: document type, sections, required sections, ordering, headings, references.
- `style`: typography, spacing, heading style, citation style, page geometry.
- `export`: future target outputs, export profile references, renderer hints.
- `references`: internal references, cross-references, bibliographic references, and `reference_style_id`.
- `bibliography`: future bibliography sources and listing behavior.
- `metadata`: title, author, language, document kind, version, dates.
- `guidance`: editorial instructions for human or future assisted drafting.

## Typed overrides

Local changes must be recorded as typed overrides:

- `structural_overrides`: section additions, section removals, heading changes, ordering changes.
- `style_overrides`: font, heading style, spacing, citation style.
- `export_overrides`: target format, profile, renderer hints, output naming.
- `reference_overrides`: reference style, reference block placement, citation behavior.
- `metadata_overrides`: title, author, language, document kind, project-specific metadata.
- `guidance_overrides`: section instructions, tone, quality checklist, internal notes.

Examples:

- changing font: `style_override`
- adding a section: `structural_override`
- changing DOCX target behavior: `export_override`
- changing APA to a corporate references style: `reference_override`
- changing title or author: `metadata_override`
- modifying section instructions: `guidance_override`

## Editorial guidance

Templates may declare per-section guidance:

- purpose
- drafting instructions
- recommended tone
- recommended length
- expected content
- forbidden content
- quality checklist
- internal observations
- export notes

Guidance orients future drafting. It does not execute, mutate, approve, block by itself, or replace policy.

Internal observations are not exported unless an export profile explicitly allows them.

## Drift classification

Template deviation must be classified by dimension:

- `style_drift`
- `metadata_drift`
- `export_drift`
- `structure_drift`
- `semantic_drift`

Example severity guidance:

- changing font: `style_drift`, minor
- adding section: `structure_drift`, warning
- removing required section: `structure_drift`, error in `strict`
- changing document type: `semantic_drift`, incompatible

Style changes do not degrade structural conformance by themselves.

## Forbidden responsibilities

Document templates must not:

- implement export
- create `document_template_runtime`
- create `document_export_runtime`
- execute Pandoc, Tectonic, LibreOffice, or external tools
- invoke LLMs
- mutate existing documents
- create an editor
- place logic in UI crates
- reinterpret project manifests
- duplicate `project_runtime`
- use `assets/` as runtime
- hardcode template rules in code
- allow direct mutation of system templates
- treat renaming as a change to `template_id`

## References and bibliography

Templates may select a `reference_style_id` from declarative resources such as:

- `resources/reference_styles/standard/`
- `resources/reference_styles/corporate/`

Reference styles are declarative. They must not be hardcoded in code or UI.

See `docs/specs/document_references.md`.

## Resolution, validation, and UI capture

Template resolution, validation, creation flow, and popup capture are specified separately:

- `docs/specs/document_template_resolution.md`
- `docs/specs/document_template_validation.md`
- `docs/specs/document_creation_flow.md`
- `docs/specs/document_template_ui_contract.md`

The built-in declarative catalog is `resources/document_templates/templates_catalog.json`.

These contracts are proposals only. They do not create runtime, package creation, export, LLM execution, tool execution, or UI authority.

## Invariants

- `INV-DOC-TEMPLATE-001`: every assisted document creation starts from a declarative template or governed default template.
- `INV-DOC-TEMPLATE-002`: templates instantiate Markdown `ARTIFACT` documents.
- `INV-DOC-TEMPLATE-003`: Markdown is the authoritative editable source.
- `INV-DOC-TEMPLATE-004`: LaTeX, PDF, and DOCX are derived and regenerable outputs.
- `INV-DOC-TEMPLATE-005`: every document created from a template preserves `template_id` and `template_snapshot`.
- `INV-DOC-TEMPLATE-006`: the local template snapshot is not destructively modified.
- `INV-DOC-TEMPLATE-007`: local modifications are recorded as typed overrides.
- `INV-DOC-TEMPLATE-008`: `effective_template` is calculated from snapshot plus overrides.
- `INV-DOC-TEMPLATE-009`: the user may choose `strict`, `guided`, or `free_from_template`.
- `INV-DOC-TEMPLATE-010`: style changes do not degrade structural conformance.
- `INV-DOC-TEMPLATE-011`: drift is classified by structure, style, metadata, export, and semantics.
- `INV-DOC-TEMPLATE-012`: templates may include editorial guidance per section.
- `INV-DOC-TEMPLATE-013`: guidance is declarative; it does not execute, mutate, or decide policy.
- `INV-DOC-TEMPLATE-014`: template observations are not exported unless explicitly declared.
- `INV-DOC-TEMPLATE-015`: future export uses `document.md`, `effective_template`, reference data, and export profile, not UI state.

## Future notes

A later phase may introduce a validator, resolver, or export runtime. That phase must keep templates declarative, preserve Markdown as source, and avoid coupling export behavior to UI state.
