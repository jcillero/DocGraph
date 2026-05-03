# transform_timeline

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `TransformTimeline` surface for DocGraph.

The timeline is a structured memory of transformation attempts, proposals, and user decisions.

It enables future inspection without executing, editing, or applying changes.

## Dependencies

This spec depends on:

- `docs/specs/transformation_core.md`
- `docs/specs/transform_popup.md`
- `docs/specs/patch_preview.md`
- `docs/specs/diff_view.md`

## Role

`TransformTimeline` is structured transformation memory.

It is not an editor.
It does not execute.
It does not apply changes.

## TransformationEvent

Minimum conceptual fields:

- `transformation_id`
- `document_ref`
- `selection_ref`
- `instruction_id`
- `proposal_id`
- `attempt_index`
- `status`
- `user_decision`
- `input_hash`
- `output_hash`
- `timestamp`
- `trace_ref`

Field meaning:

- `transformation_id`: stable event lineage identifier
- `document_ref`: governed target document reference
- `selection_ref`: governed scoped selection reference when applicable
- `instruction_id`: originating transform instruction
- `proposal_id`: associated proposal identity
- `attempt_index`: ordered attempt number within lineage
- `status`: current lifecycle state for the event
- `user_decision`: governed user review outcome when present
- `input_hash`: integrity hash of relevant input snapshot
- `output_hash`: integrity hash of relevant proposal or output snapshot
- `timestamp`: event ordering timestamp
- `trace_ref`: governed trace reference for further inspection

## States

Prepared timeline event state may include:

- `draft`
- `proposed`
- `accepted`
- `cancelled`
- `superseded`
- `stale`
- `applied_future`
- `error`

Interpretation:

- `draft`: instruction or attempt exists before proposal is ready
- `proposed`: proposal exists and is awaiting user validation
- `accepted`: user accepted the proposal, but no application is implied in F9
- `cancelled`: flow was cancelled
- `superseded`: a newer proposal replaced the prior attempt
- `stale`: event lineage no longer matches current document or selection integrity safely
- `applied_future`: reserved future state for a later execution phase
- `error`: the attempt or trace state failed safely

## Conceptual UX

The timeline may conceptually provide:

- a list of transformations per document or artifact
- click on an event may highlight the associated overlay or diff
- hover may show conceptual preview
- save as recipe remains future-only

This is inspection UX only.

It does not authorize mutation.

## Rules

- timeline does not substitute `graph/`
- timeline is not `tool_run_manifest`
- timeline does not apply changes
- timeline preserves traceability
- timeline may feed future recipes

Future recipe abstraction is governed separately by `docs/specs/transformation_recipes.md`.

Normative form:

- `INV-TTL-001`: `TransformTimeline` MUST preserve transformation traceability
- `INV-TTL-002`: `TransformTimeline` MUST NOT replace `graph/`
- `INV-TTL-003`: `TransformTimeline` MUST NOT be treated as `tool_run_manifest`
- `INV-TTL-004`: `TransformTimeline` MUST NOT apply changes
- `INV-TTL-005`: timeline memory MAY feed future recipe construction without becoming execution authority

## Relationship to transformation_core

`TransformTimeline` consumes `TransformationEvent` and related lineage from `transformation_core.md`.

It must not redefine proposal, preview, acceptance, cancellation, or supersession semantics independently.

## Relationship to popup, preview, and diff

- popup creates or advances proposal lineage
- preview may provide associated in-place overlay state
- diff may provide associated comparative inspection state
- timeline records attempts and decisions across those surfaces

## Failure modes

- `event_without_target`
- `event_without_proposal`
- `stale_event`
- `missing_trace_ref_future`
- `duplicate_transformation_id`

Interpretation:

- `event_without_target`: event exists without valid document or artifact target reference
- `event_without_proposal`: event lineage lacks required proposal identity for the expected state
- `stale_event`: event no longer matches safe current target integrity
- `missing_trace_ref_future`: trace reference required for future deeper inspection is absent
- `duplicate_transformation_id`: two separate events collide on the same transformation identity

## Invariants

- `INV-TTL-006`: timeline interaction MUST remain inspection-only in F9
- `INV-TTL-007`: clicking or hovering a timeline event MAY expose overlay or diff context, but MUST NOT execute or apply anything
- `INV-TTL-008`: accepted timeline state MUST remain distinct from `applied_future`

## Related specs

- `docs/specs/transformation_core.md`
- `docs/specs/transform_popup.md`
- `docs/specs/patch_preview.md`
- `docs/specs/diff_view.md`
- `docs/specs/document_governed_editing.md`
- `docs/specs/workspace_tabs.md`
- `docs/specs/transformation_recipes.md`
