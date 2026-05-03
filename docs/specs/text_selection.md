# text_selection

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define governed text selection in `Document Viewer` as a structured reference.

Text selection must not become a new source of truth.

It must remain a governed reference to a document context that may later feed popup capture, chat references, and future transformation workflows.

## Core rule

Text selection references a governed document.

It does not modify the document.
It does not become an independent document copy.
It does not replace the governed source.

## Canonical model

Conceptual selection reference may be named `TextSelectionRef` or `ContextSelectionRef`.

Minimum conceptual fields:

- `selection_ref`
- `document_ref`
- `range_start`
- `range_end`
- `selected_text_hash`
- `selected_text_preview`
- `document_version_or_hash`
- `created_at`
- `status`

Field meaning:

- `selection_ref`: stable governed identifier for the prepared selection reference
- `document_ref`: governed target document reference
- `range_start`: start position in the governed rendered or textual model
- `range_end`: end position in the governed rendered or textual model
- `selected_text_hash`: integrity hash of the selected text snapshot
- `selected_text_preview`: non-authoritative preview text for display only
- `document_version_or_hash`: document version marker used to detect drift or stale selection state
- `created_at`: creation timestamp for trace and ordering
- `status`: prepared selection state

## Rules

- selection does not modify the document
- selection references the document
- selection must not lose `document_ref`
- selection must not use host absolute paths
- if the document changes, the selection becomes `stale`
- if `selection_ref` cannot be resolved, any transformation depending on it must be blocked

Normative form:

- `INV-TSEL-001`: text selection MUST NOT modify the document
- `INV-TSEL-002`: text selection MUST retain `document_ref`
- `INV-TSEL-003`: text selection MUST NOT use host absolute paths as identity or source of truth
- `INV-TSEL-004`: document drift MUST move the selection to `stale`
- `INV-TSEL-005`: unresolved `selection_ref` MUST block transformation preparation that depends on it
- `INV-TSEL-006`: `selected_text_preview` is display aid only and MUST NOT become project source truth

## Relationship to Document Viewer

`Document Viewer` captures visual text selection.

Prepared selection state may be passed to popup or other governed context as structured reference.

The viewer does not decide permissions and does not authorize transformations.

Text selection does not turn the viewer into an editor.

## Relationship to chat

Text selection may be sent to chat as a structured reference.

Chat must not treat selected text as an opaque blob source of truth.

If text content is shown in chat, it remains explanatory only; the governed reference remains authoritative.

## Relationship to transformation

Future transformation requests may use `selection_ref` as optional scoped context.

`selection_ref` must remain linked to:

- `document_ref`
- document version or hash
- selected text integrity hash

This spec does not define popup behavior, patch preview, diff behavior, or transformation execution.

## States

Prepared selection state may include:

- `active`
- `cleared`
- `stale`
- `missing`
- `invalid`

Interpretation:

- `active`: valid selection reference exists and matches current document context
- `cleared`: selection was intentionally removed and no active selection remains
- `stale`: selection reference exists but no longer matches current document version or hash
- `missing`: `selection_ref` no longer resolves
- `invalid`: range or integrity constraints are not valid for current prepared context

## Failure modes

- `empty_selection`
- `selection_out_of_range`
- `document_hash_mismatch`
- `selection_ref_missing`
- `selection_stale`

Interpretation:

- `empty_selection`: no non-empty governed selection was captured
- `selection_out_of_range`: prepared range is outside the valid governed document model
- `document_hash_mismatch`: captured selection no longer matches current document version or hash
- `selection_ref_missing`: `selection_ref` no longer resolves
- `selection_stale`: selection exists but is no longer current enough for safe transformation preparation

## Forbidden responsibilities

Text selection must not:

- create a new durable source document
- detach from `document_ref`
- bypass `Document Viewer`
- authorize transformation by itself
- mutate document content
- replace `ActionResolution`

## Related specs

- `docs/specs/document_viewer.md`
- `docs/specs/active_object_context.md`
- `docs/specs/chat_document_flows.md`
- `docs/specs/transformation_core.md`
- `docs/specs/transform_popup.md`
