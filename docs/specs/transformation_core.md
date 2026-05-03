# transformation_core

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Principle

DocGraph does not edit documents.

DocGraph transforms documents.

## Purpose

Centralize the transversal document-transformation model.

This spec avoids duplicating concepts across viewer, popup, diff, timeline, and future recipes.

It defines canonical names for instruction, proposal, preview, overlay, event, timeline, and future recipe concepts.

## Canonical concepts

- `TransformInstruction`
- `TransformProposal`
- `PatchPreviewOverlay`
- `PatchPreview`
- `TransformationEvent`
- `TransformTimeline`
- `TransformationRecipe`, future

## TransformInstruction

Conceptual fields:

- `instruction_id`
- `source`
- `raw_instruction`
- `target_ref`
- `selection_ref`, optional
- `created_at`

## TransformProposal

Conceptual fields:

- `proposal_id`
- `instruction_id`
- `target_ref`
- `selection_ref`
- `original_text_hash`
- `proposed_text`
- `attempt_index`
- `status`
- `created_at`

## PatchPreviewOverlay

Conceptual fields:

- `overlay_id`
- `proposal_id`
- `target_ref`
- `selection_ref`
- `overlay_kind`
- `overlay_state`
- `highlight_token`

## PatchPreview

`PatchPreview` is the governed preview representation associated with a transformation proposal.

It is the preview contract consumed by review surfaces.

It is not persistence, not acceptance, and not application.

Minimum conceptual relationship:

- one `TransformProposal` may produce one preview representation
- `PatchPreviewOverlay` is the in-view rendering layer for that preview
- preview may be represented in popup, diff, or future dedicated patch-preview surfaces

## TransformationEvent

Conceptual fields:

- `transformation_id`
- `proposal_id`
- `instruction_id`
- `target_ref`
- `selection_ref`
- `user_decision`
- `status`
- `trace_ref`
- `timestamp`

## TransformTimeline

`TransformTimeline` is the ordered trace of transformation attempts and decisions for a target or instruction lineage.

It may include:

- original `TransformInstruction`
- one or more `TransformProposal` attempts
- associated preview lifecycle
- user decisions
- supersession or cancellation events

It is a traceability surface, not an execution engine.

## TransformationRecipe

Future only.

`TransformationRecipe` is a future higher-level declarative structure for reusable transformation patterns.

It must build on canonical transformation concepts rather than redefining them.

## Canonical states

- `draft`
- `proposed`
- `pending_user_validation`
- `accepted`
- `cancelled`
- `superseded`
- `stale`
- `applied_future`
- `error`

## Invariants

- proposal does not modify document
- overlay is not persistence
- acceptance does not apply in F9
- regenerate creates a new `proposal_id`
- cancel clears overlay
- every attempt preserves traceability

Normative form:

- `INV-TRANSFORM-001`: a `TransformProposal` MUST NOT modify the document by itself.
- `INV-TRANSFORM-002`: `PatchPreviewOverlay` MUST NOT be treated as persistence.
- `INV-TRANSFORM-003`: accepting a proposal MUST NOT apply changes in F9.
- `INV-TRANSFORM-004`: regeneration MUST create a new `proposal_id`.
- `INV-TRANSFORM-005`: cancellation MUST clear the active overlay representation.
- `INV-TRANSFORM-006`: every attempt MUST preserve traceability to instruction, target, selection, and proposal lineage.

## Failure modes

- `invalid_target_ref`
- `stale_selection`
- `proposal_superseded`
- `proposal_without_instruction`
- `missing_original_hash`

## Relationship to existing specs

- `document_governed_editing.md` describes future governed editing flow; `transformation_core.md` defines the shared transformation vocabulary.
- `document_patch_runtime.md` may later validate, preview, and apply accepted patches; it should reuse `TransformProposal`, `PatchPreview`, and `TransformationEvent` terminology instead of redefining them.
- `active_context.md` provides target and selection context; it should remain the source of active-context interpretation.
- `workspace_tabs.md` may host future `patch_preview` views, but view hosting must not redefine transformation concepts.

## Specs that should reference this spec

- `docs/specs/document_governed_editing.md`
- `docs/specs/document_patch_runtime.md`
- `docs/specs/active_context.md`
- `docs/specs/workspace_tabs.md`

Potential future references:

- viewer selection popup spec
- diff or patch-preview spec
- transformation timeline spec
- transformation recipe spec

## Risks

- if future specs redefine proposal, preview, or event concepts locally, terminology drift will reappear
- if preview and overlay are conflated, UI state may be mistaken for persistence
- if accepted and applied are not kept distinct, F9 documentation may overstate execution

## Validation

Recommended:

```bat
dev\scripts\generate_llm_context_bundle.bat
dev\scripts\generate_status_snapshot.bat
dev\scripts\validate_f9_declarations.bat
```
