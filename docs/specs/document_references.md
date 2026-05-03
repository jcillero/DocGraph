# document_references

## Status

PROPOSAL. This document defines future governed document references, bibliography, and citation/export style contracts. It does not implement runtime, BibTeX parsing, export, LLM calls, tool execution, or document mutation.

## Purpose

Define references as structured document entities rather than opaque plain text.

The document template may define:

- structure
- layout
- editorial guidance
- visual style
- export behavior
- references
- bibliography

Markdown keeps symbolic or structured reference intent. Future LaTeX, PDF, and DOCX export materializes final citation formatting from structured entries plus declarative style.

## Conceptual source set

Future export may use:

```text
document.md
+ document_manifest.json
+ effective_template.json
+ reference_entries.json
+ reference_style.json
+ bibliography_sources.json
+ export_profile.json
-> future LaTeX / PDF / DOCX
```

UI state is not part of the source set.

## References vs governed storage

Structured references may point to governed content identifiers or logical stored objects, but they do not duplicate physical content.

- refs declare usage
- refs do not define storage authority
- refs do not imply project exposure
- schema validity does not imply project exposure
- `project_manifest` remains the exposure authority
- future reference structures must remain path-independent when a governed `file_ref` is available

## Reference kinds

### Internal active-project references

- `SOURCE`
- `DERIVED`
- `ARTIFACT`
- `knowledge/*`
- future figures, tables, and equations

Internal references may be used as sources, evidence, or appendices without mutating the referenced object.

### External bibliographic references

- BibTeX
- DOI
- ISBN
- URL
- manual metadata

BibTeX files may be knowledge resources when declared or placed under governed knowledge surfaces. Future import creates structured bibliography entries; it does not convert the BibTeX file into a mutable document.

### Internal cross-references

- sections
- tables
- figures
- appendices

## ReferenceEntry examples

Project document reference:

```json
{
  "ref_id": "proj_src_001",
  "kind": "project_document",
  "target": "source://requirements/original.pdf",
  "label": "Documento de requisitos original",
  "citation_role": "evidence",
  "export_behavior": "footnote"
}
```

BibTeX reference:

```json
{
  "ref_id": "smith2024",
  "kind": "bibtex_entry",
  "source": "knowledge/references/main.bib",
  "bibtex_key": "smith2024",
  "citation_role": "bibliographic",
  "export_behavior": "bibliography"
}
```

## Reference style kinds

- `standard`: established styles such as APA, IEEE, or Chicago.
- `corporate`: official organization-owned styles.
- `custom_project`: project-local styles.

Standard and corporate styles may coexist, but each style must be explicitly identified.

## Corporate reference style example

```json
{
  "style_id": "corp_official_letter_refs_v1",
  "style_kind": "corporate",
  "display_name": "Corporate official letter references",
  "labeling": {
    "mode": "alphabetic",
    "format": "lowercase_letter",
    "entry_pattern": "{label}) {title}",
    "citation_pattern": "Referencia {label})"
  },
  "placement": {
    "block": "references_internal",
    "position": "before_sections"
  },
  "export_targets": ["latex", "pdf", "docx"]
}
```

The future system generates labels such as `a)`, `b)`, and `c)`. Users should not manually maintain those labels.

## Layout blocks

A template may define export layout order independently from editing order:

- `cover_page`
- `table_of_contents`
- `references_internal`
- `sections`
- `list_of_figures`
- `list_of_tables`
- `bibliography`
- `appendices`

## Forbidden responsibilities

This proposal must not:

- implement a BibTeX parser
- import real bibliographic data
- execute Pandoc, Tectonic, LibreOffice, or external tools
- create `document_reference_runtime`
- create `document_export_runtime`
- call LLMs
- mutate documents
- make BibTeX files editable documents
- hardcode corporate styles
- place reference logic in UI
- touch `project_runtime`

## Invariants

- `INV-DOC-REF-001`: document references must be structured entities, not opaque plain text.
- `INV-DOC-REF-002`: templates may declare citation and reference style.
- `INV-DOC-REF-003`: original active-project documents may be referenced as internal sources, evidence, or appendices without mutation.
- `INV-DOC-REF-004`: BibTeX files may be bibliographic knowledge resources when declared or located in governed surfaces.
- `INV-DOC-REF-005`: future BibTeX import creates structured bibliography entries; it does not convert BibTeX into a mutable document.
- `INV-DOC-REF-006`: future export resolves references from structured entries plus template style, not UI heuristics.
- `INV-DOC-REF-007`: Markdown keeps symbolic or structured references; LaTeX and DOCX materialize final format.
- `INV-DOC-REF-008`: referenced `SOURCE` and `DERIVED` remain readonly.
- `INV-DOC-REF-009`: templates may define numeric, alphabetic, symbolic, or standard bibliographic labeling.
- `INV-DOC-REF-010`: templates may define the position of the references block in layout.
- `INV-DOC-REF-011`: citation patterns must be declarative and derivable from `reference_style`.
- `INV-DOC-REF-012`: non-academic reference styles must be supported.
- `INV-DOC-REF-013`: label generation such as `a)`, `b)`, `c)` is future system responsibility, not user responsibility.
- `INV-DOC-REF-014`: standard and corporate styles must be supported as declarative resources.
- `INV-DOC-REF-015`: corporate styles must not be hardcoded in code or UI.
- `INV-DOC-REF-016`: templates may select a corporate or standard `reference_style_id`.
- `INV-DOC-REF-017`: company styles may define labeling, placement, citation patterns, and listing rules.
- `INV-DOC-REF-018`: standard and corporate styles may coexist but must be explicitly identified.

## Future notes

A later phase may add loaders, validators, importers, or exporters. Those systems must preserve structured references as the source of citation truth and keep corporate style definitions in resources.

## F12.4 file intake reference alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

Future intake items may be referenced by governed refs such as `intake_batch_ref`, `intake_item_id`, `file_ref`, `object_ref`, `owner_ref`, and `trace_ref`.

Source paths and filenames must not become document identity.

Chat may reference intake items but must not become a hidden blob store.

F12.4 intake comments are referenced as metadata on an `IntakeItem`, not as standalone documents or document references.

Comment references may use the governed `intake_item_id`, `owner_ref`, and `trace_ref`.

Comments must not become citation authority, classification authority, exposure authority, derivation authority, or hidden chat-owned project metadata.

## F12.5 file intake reference plan

`F12.5` is plan-only and adds no reference runtime.

Future `F12.6` metadata may reference intake candidates by `intake_item_id`, `intake_batch_ref`, `object_ref` or `stored_object_candidate_ref`, `file_ref` or `content_hash`, `owner_ref`, and `trace_ref`.

Future `F12.6` must not use source path, filename, or chat comment text as durable identity.
