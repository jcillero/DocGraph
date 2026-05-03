# document_package

## Status

PROPOSAL. This is a declarative package model only. No package runtime is implemented.

## Purpose

Define a self-contained `DocumentHolder` folder shape for future Markdown `ARTIFACT` documents created from templates.

`DocumentHolder` is the document production unit inside a DocGraph project. It owns the editable Markdown source, governed manifests, template state, references, assets, lifecycle state, owner-scoped tool outputs, and regenerable exports.

## Proposed package layout

```text
document_package/
  document.md
  manifests/
    document_manifest.json
    source_context.json
  template/
    template_snapshot.json
    template_overrides.json
    effective_template.json
  references/
  assets/
  lifecycle/
    status.json
    review_log.jsonl
  tool_outputs/
    <tool_id>/
      <run_id>/
        tool_run_manifest.json
        outputs/
        logs/
  derived/
    latex/
    docx/
    pdf/
```

## File roles

- `document.md`: authoritative editable Markdown source.
- `manifests/document_manifest.json`: local package metadata and document identity.
- `manifests/source_context.json`: governed source and context references used to create or revise the document.
- `template/template_snapshot.json`: immutable local copy of the base template used at creation time.
- `template/template_overrides.json`: typed local changes over the snapshot.
- `template/effective_template.json`: derived, regenerable result of snapshot plus overrides.
- `references/`: structured internal, bibliographic, and cross-reference entries.
- `assets/`: package-local document assets, not application runtime assets.
- `lifecycle/status.json`: document state metadata for the DocumentHolder.
- `lifecycle/review_log.jsonl`: append-oriented review history for the DocumentHolder.
- `tool_outputs/<tool_id>/<run_id>/`: document-owned tool outputs with `tool_run_manifest.json`, `outputs/`, and `logs/`.
- `derived/latex/`: future regenerable LaTeX output.
- `derived/docx/`: future regenerable DOCX output.
- `derived/pdf/`: future regenerable PDF output.

## Rules

- `INV-DOC-HOLDER-001`: `DocumentHolder` is the owner of document production state.
- `INV-DOC-HOLDER-002`: `document.md` is the only editable textual source.
- `INV-DOC-HOLDER-003`: `derived/` contains regenerable exports only.
- `INV-DOC-HOLDER-004`: `tool_outputs/` inside `DocumentHolder` require `tool_run_manifest.json` and `owner_ref`.
- `INV-DOC-HOLDER-005`: `lifecycle/` records state and review only; it is not a workflow runner.
- `INV-TOOL-OUTPUT-003`: document-contributing outputs MUST live under the owning `DocumentHolder`.
- `template_snapshot.json` preserves the original template base.
- `template_overrides.json` records typed local changes.
- `effective_template.json` is derived and regenerable.
- `document.md` is the textual authority.
- `derived/*` contains regenerable outputs only.
- package-local `assets/` are document content assets, not runtime resources.
- references are structured entities, not opaque citation text.
- BibTeX sources are knowledge or bibliography resources, not mutable document bodies.
- tool outputs created for a document must be scoped under the owning `DocumentHolder`.
- document-owned tool output manifests must carry an `owner_ref` pointing to the `DocumentHolder`.
- `lifecycle/` is limited to document status and review data. It is not a workflow runner.
- There is no project-root `outputs/` folder for document exports.

## Forbidden responsibilities

The package model must not imply:

- export execution
- editor implementation
- filesystem mutation outside explicit future package operations
- LLM calls
- external tool execution
- project manifest reinterpretation
- UI-owned document truth

## Future notes

Future package operations should treat the package as user-owned document data and should not move project pipeline responsibility into document export logic.

Future document creation from templates is described in `docs/specs/document_creation_flow.md`.

The UI capture boundary for template drafts is described in `docs/specs/document_template_ui_contract.md`.
