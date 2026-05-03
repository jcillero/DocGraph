# diff_view

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Diff View` surface for transformation proposals in DocGraph.

The goal is to support visual comparison without introducing editing, persistence, or application behavior.

## Dependencies

This spec depends on:

- `docs/specs/transformation_core.md`
- `docs/specs/patch_preview.md`
- `docs/specs/document_viewer.md`

## Core rule

`Diff View` is visualization only.

It does not edit.
It does not apply changes.
It does not mutate the document.

## Inline Preview

Inline preview is the default review mode.

Characteristics:

- the proposal is shown in context of the document
- it uses the pending proposal overlay model
- it uses soft blue highlight
- it is oriented to novice users

Inline preview prioritizes contextual readability over formal comparison density.

## Side-by-side Diff

Side-by-side diff is on demand.

Conceptual entry:

- `Ver cambios`

Characteristics:

- shows original vs proposed content
- supports more rigorous review
- is not the default view

Side-by-side review is a comparison surface, not a second editor.

## Review rules

- diff is visualization
- diff does not edit
- diff does not apply changes
- diff does not mutate the document
- side-by-side is not default

## Color semantics

- original = neutral
- proposal = soft blue
- optional internal changes = slightly stronger but still soft blue

Default presentation must avoid aggressive red/green commit semantics.

The visual language should communicate:

- original text remains neutral baseline
- proposed text is pending and uncommitted
- comparison remains readable for long-form document review

## States

Prepared diff-view state may include:

- `available`
- `unavailable`
- `stale`
- `superseded`
- `error`

Interpretation:

- `available`: governed diff data is ready for visual presentation
- `unavailable`: diff rendering is not currently available in prepared state
- `stale`: proposal or source context no longer matches current document integrity
- `superseded`: a newer proposal replaced the diff currently being viewed
- `error`: diff rendering failed safely

## Failure modes

- `missing_original_text`
- `missing_proposed_text`
- `stale_proposal`
- `diff_generation_unavailable`

Interpretation:

- `missing_original_text`: original comparison text cannot be prepared
- `missing_proposed_text`: proposed comparison text cannot be prepared
- `stale_proposal`: proposal is no longer current enough for safe review
- `diff_generation_unavailable`: diff comparison state is unavailable in current governed phase or prepared state

## Relationship to PatchPreview

Inline preview remains the default path and should align with `PatchPreviewOverlay`.

`Diff View` is the optional deeper inspection surface.

It must not replace the inline overlay as the default novice path.

## Relationship to Document Viewer

Inline preview may be rendered in `Document Viewer`.

Side-by-side diff may be shown in a separate readonly review surface when requested.

Neither surface may edit or apply the proposal.

## Invariants

- `INV-DIFF-001`: `Diff View` MUST remain visualization-only
- `INV-DIFF-002`: inline preview MUST remain the default review path
- `INV-DIFF-003`: side-by-side diff MUST remain optional and on demand
- `INV-DIFF-004`: default proposal highlighting MUST avoid aggressive red/green semantics
- `INV-DIFF-005`: diff rendering MUST NOT mutate source or proposal state

## Related specs

- `docs/specs/transformation_core.md`
- `docs/specs/patch_preview.md`
- `docs/specs/document_viewer.md`
- `docs/specs/transform_popup.md`
