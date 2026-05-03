# document_text_runtime

## Purpose

Define the minimum contract for regenerable text derivation over primary project documents.

## Primary vs derived

- the source file remains the primary document
- `text/` contains derived, regenerable artifacts
- future `semantics/` layers build on derived text, not on raw source ownership changes

## Minimum output

Each derivation produces:

- `text/extracted.txt` when ready
- `text/text_manifest.json`
- optional `text/pages/page_XXX.txt`
- optional `text/chunks/chunk_XXXX.txt`
- optional `text/chunks/chunks_manifest.json`

## Status model

- `ready`
- `unsupported`
- `error`

## Traceability

Every derivation and chunk must preserve at least:

- `document_id`
- `source_filename`
- `source_format`
- `source_hash`

Chunk metadata also keeps:

- `chunk_id`
- `char_start`
- `char_end`
- optional `page_start`
- optional `page_end`

Chunk evidence integrity matters for downstream semantic invalidation:

- `chunk_hash` changes may stale downstream semantic evidence
- missing chunk artifacts may orphan downstream semantic evidence
- document-text runtime does not invalidate semantic or graph material directly; it provides governed derivative evidence only

## Out of scope

- OCR by default
- semantic parsing
- embeddings
- graph/quads
- retrieval orchestration
- notebook or editing workflows

## F12.4 file intake derivation alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

F12.4 may plan future derivation only. It does not extract text, pages, chunks, thumbnails, previews, metadata indexes, or checksum indexes.

`document_text_runtime` may later consume exposed or eligible project-owned objects, but F12.4 does not implement that handoff.

## F12.5 file intake derivation plan

`F12.5` is plan-only and adds no derivation runtime.

Future `F12.6` must not generate text, chunks, pages, thumbnails, previews, metadata indexes, checksum indexes, or any derivative.

`document_text_runtime` must not be modified for `F12.6` unless a later explicit derivation phase opens that scope.

Derivatives remain deterministic, regenerable, and not source truth.

## F13.1 legacy derivation bypass hardening

`F13.1` is SPEC-only and adds no derivation runtime.

`derive_document_text` and any equivalent derivation helper must not be used as:

- an intake step
- an exposure precondition
- a project visibility trigger
- a UI shortcut for importing or promoting external files

Direct UI-triggered derivation from external files or newly copied files is legacy-only and must be blocked before any `F13` runtime implementation begins.
