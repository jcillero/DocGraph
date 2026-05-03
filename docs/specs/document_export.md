# document_export

## Status

PROPOSAL. This document defines future governed export only. No export runtime, external tool execution, or UI-owned logic is implemented.

## Purpose

Define a governed and reproducible export model for `DocumentHolder` packages where Markdown remains canonical source and every export artifact is regenerable.

## Canonical source

`document.md` is the only editable source for document export.

Export must treat the following as inputs to future preparation only:

- `document.md`
- selected `export_profile`
- `template_snapshot.json` and/or `effective_template.json`
- `reference_style`
- bibliography sources

Export must not treat the following as source of truth:

- `derived/latex/document.tex`
- `derived/pdf/document.pdf`
- `derived/docx/document.docx`
- UI state
- viewer state
- hidden chat state
- external binary defaults

## Derived artifacts

Future export may materialize the following regenerable outputs under `DocumentHolder`:

- `derived/latex/document.tex`
- `derived/pdf/document.pdf`
- `derived/docx/document.docx`

Rules:

- Markdown is canonical.
- LaTeX is derived, not authoritative.
- PDF and DOCX are final artifacts only.
- Derived artifacts are never editable source of truth.
- Export artifacts must remain regenerable from governed inputs.

## Conceptual export pipeline

The future conceptual pipeline is:

`document.md`
`+ export_profile`
`+ template_snapshot / effective_template`
`+ reference_style`
`+ bibliography_sources`
`-> LaTeX`
`-> PDF / DOCX`

This pipeline is declarative only in the current phase.

It does not:

- execute Pandoc
- execute Tectonic
- execute LibreOffice
- execute any external binary
- mutate the source document
- create a runtime runner
- move orchestration into `project_runtime`

## Export profile model

Export profiles live as declarative resources under:

`resources/export_profiles/<profile_id>.json`

Each profile must define:

- `id`
- `target_format` (`latex` | `pdf` | `docx`)
- `template_ref`
- `citation_style`
- `include_toc`
- `include_cover`
- `page_geometry`
- `font_config`
- `extra_args`

Rules:

- profiles are declarative resources only
- profiles do not imply that any external tool exists
- profiles do not authorize execution
- `extra_args` is opaque and must not encode hardcoded binary paths
- profile resolution is future governance, not current runtime

## External dependency model

Potential future export toolchains such as:

- Pandoc
- Tectonic
- LibreOffice

must be treated as external tools.

Rules:

- external tools are resolved via configuration, not code
- external tools must not be hardcoded in runtime crates
- external tools are not required for F9 or F10 execution
- declaring an export profile does not require any toolchain to be installed

## Traceability requirement

In a future execution phase, each export should produce:

- `export_manifest.json`

That future manifest should include at least:

- `source_hash`
- `profile_id`
- `timestamp`
- `toolchain_reference`

This traceability requirement is future-ready only. It does not enable export execution now.

## Invariants

- `INV-DOC-EXPORT-001`: Markdown is the only editable source.
- `INV-DOC-EXPORT-002`: LaTeX is derived and not authoritative.
- `INV-DOC-EXPORT-003`: PDF and DOCX are final artifacts only.
- `INV-DOC-EXPORT-004`: export is fully regenerable.
- `INV-DOC-EXPORT-005`: export must not mutate source content.
- `INV-DOC-EXPORT-006`: no UI-driven export logic is allowed.
- `INV-DOC-EXPORT-007`: no hardcoded binary paths are allowed.
- `INV-DOC-EXPORT-008`: export profiles are declarative and non-executing.
- `INV-DOC-EXPORT-009`: LaTeX and PDF are never source of truth.
- `INV-DOC-EXPORT-010`: external tools are future dependencies, not current runtime requirements.

## Forbidden responsibilities

This spec must not:

- implement export runtime
- create new runtime crates
- wire export into `project_runtime`
- assume execution enabled
- place logic in UI
- treat LaTeX as source
- treat PDF as source
- enable RDF, graph analysis, LLM execution, or tool execution

## Future notes

If export is opened in a later phase, implementation must preserve:

- Markdown as canonical source
- reproducible derived artifacts
- configuration-based external tool resolution
- auditable manifest production
- separation between declarative profiles and executable runtime
