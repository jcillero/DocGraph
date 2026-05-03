# patch_preview

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `PatchPreviewOverlay` model and semantic highlight behavior for DocGraph.

The goal is to show an in-place proposal over a valid selection inside `Document Viewer` while making it clear that the underlying document has not changed.

## Dependencies

This spec depends on:

- `docs/specs/transformation_core.md`
- `docs/specs/document_viewer.md`
- `docs/specs/transform_popup.md`

## Canonical model

`PatchPreviewOverlay` uses the transformation-core vocabulary and may include:

- `overlay_id`
- `proposal_id`
- `selection_ref`
- `target_ref`
- `original_text_hash`
- `proposed_text`
- `overlay_kind`
- `overlay_state`
- `highlight_token`

Field meaning:

- `overlay_id`: stable overlay identity for the prepared visual layer
- `proposal_id`: governed proposal identity associated with the preview
- `selection_ref`: governed selection scope for the preview
- `target_ref`: governed document target
- `original_text_hash`: integrity marker tying preview to the original selected content
- `proposed_text`: proposed replacement or transformed text snapshot
- `overlay_kind`: governed overlay rendering kind
- `overlay_state`: prepared visual state of the overlay
- `highlight_token`: semantic theme token used for the pending proposal highlight

## Main rule

`Document source + PatchPreviewOverlay = viewer rendering`

The real document does not change.

The overlay is visual composition only.

It is not:

- persistence
- acceptance
- application
- document mutation

## Overlay states

Prepared overlay state may include:

- `pending_proposal`
- `accepted_visual_feedback_future`
- `cancelled`
- `stale`
- `superseded`
- `error`

Interpretation:

- `pending_proposal`: proposal exists and is awaiting governed user validation
- `accepted_visual_feedback_future`: future-only visual state after acceptance feedback
- `cancelled`: popup flow was cancelled and overlay should disappear
- `stale`: overlay no longer matches current document or selection integrity
- `superseded`: a newer proposal replaced the previous one
- `error`: overlay could not be rendered safely as prepared state

## Highlight semantics

The preview should use semantic color tokens:

- `proposal.pending.background`
- `proposal.pending.border`, optional

Semantic meaning:

- blue = proposal pending validation
- normal text remains normal text
- the overlay must not look like saved text

## Highlight rules

- blue indicates a pending proposal awaiting validation
- the highlight must not resemble committed or saved document state
- the highlight must not fatigue reading
- the highlight must not block legibility
- the highlight should guide attention, not hijack it
- strong red or green should not be the default pending-preview language

## Animation

Subtle fade-in is optional.

No animation is required in F9.

Animation must not be necessary to understand that the overlay is pending preview only.

## Relationship to Document Viewer

`Document Viewer` may render the overlay when prepared state provides it.

The viewer still renders a readonly document surface.

The overlay does not make the viewer editable and does not apply the proposal.

## Relationship to Transform Popup

The popup may generate or cancel the preview flow.

`Cancelar` should clear the overlay representation.

`Aceptar` may leave future visual feedback state, but no application occurs in F9.

For deeper comparison under user demand, side-by-side review is governed separately in `docs/specs/diff_view.md`.

## Relationship to transformation_core

This spec consumes `PatchPreviewOverlay` from `transformation_core.md`.

It must not redefine proposal lineage, acceptance, cancellation, or supersession semantics outside that shared vocabulary.

## Failure modes

- `overlay_without_proposal`
- `stale_overlay`
- `overlay_target_missing`
- `overlay_conflict`
- `unsupported_overlay_kind`

Interpretation:

- `overlay_without_proposal`: prepared overlay exists without valid proposal lineage
- `stale_overlay`: overlay no longer matches current document or selection integrity
- `overlay_target_missing`: target reference no longer resolves
- `overlay_conflict`: two incompatible overlay states compete for the same governed target region
- `unsupported_overlay_kind`: prepared overlay kind cannot be rendered by the current governed viewer contract

## Invariants

- `INV-PPREV-001`: `PatchPreviewOverlay` MUST remain visual-only rendering state
- `INV-PPREV-002`: rendering an overlay MUST NOT mutate the document
- `INV-PPREV-003`: pending preview highlight MUST communicate unvalidated proposal state
- `INV-PPREV-004`: default pending-preview highlight MUST avoid strong red/green commit semantics
- `INV-PPREV-005`: overlay legibility MUST remain subordinate to document readability
- `INV-PPREV-006`: cancellation MUST clear overlay rendering state

## Related specs

- `docs/specs/transformation_core.md`
- `docs/specs/document_viewer.md`
- `docs/specs/transform_popup.md`
- `docs/specs/diff_view.md`
