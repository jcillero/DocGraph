# transform_popup

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed contextual transformation popup for DocGraph.

The popup lets the user order a transformation over a valid text selection while keeping selected text readonly and user instruction editable.

DocGraph does not edit documents directly.

## Dependencies

This spec depends on:

- `docs/specs/transformation_core.md`
- `docs/specs/text_selection.md`
- `docs/specs/document_viewer.md`

## Opening rule

The popup may open from right-click on a valid selection in `Document Viewer`.

If no valid `TextSelectionRef` exists:

- the popup must not open, or
- it must open in blocked state with no transformation flow available

The popup is a governed intent-capture surface.

It is not an editor.
It is not a runtime.
It does not apply changes.

## Popup structure

The popup contains two conceptual fields.

### Selection Preview

The upper field is `Selection Preview`.

It:

- shows selected text
- is readonly
- may allow copy or local text selection if presentation requires it
- must not allow editing

The preview is display state only and must remain tied to the underlying `TextSelectionRef`.

### Transform Instruction

The lower field is `Transform Instruction`.

It:

- is editable
- captures the user's transformation order
- changes only the instruction text
- does not modify the document

Editing the instruction does not change the selected text and does not mutate the target document.

## Buttons by state

### Before generation

- `Generar propuesta`
- `Cancelar`

### After generation

- `Aceptar`
- `Regenerar`
- `Cancelar`

### Prohibited buttons

The popup must not introduce:

- separate `Rechazar` button
- separate `Corregir instruccion` button
- document edit button

## Semantic actions

### Generar propuesta

`Generar propuesta` conceptually creates a `TransformProposal`.

It uses:

- current instruction text
- target `document_ref`
- current `selection_ref`

No document application occurs in F9.

### Regenerar

`Regenerar` uses the current instruction text and creates a new `proposal_id`.

It must preserve lineage through `instruction_id`, target, selection, and trace.

### Cancelar

`Cancelar` discards the current popup flow and clears the active overlay representation.

It does not save.
It does not apply.

### Aceptar

`Aceptar` marks the proposal as accepted.

In F9, acceptance does not apply the proposal.

Any future application must pass through `ActionResolution` and later governed runtime.

## States

Prepared popup state may include:

- `selection_ready`
- `instruction_draft`
- `proposal_requested`
- `proposal_ready`
- `accepted`
- `cancelled`
- `regenerate_requested`
- `error`

Interpretation:

- `selection_ready`: valid selection context is present and popup may begin instruction capture
- `instruction_draft`: user is editing or has edited the instruction field
- `proposal_requested`: proposal generation was requested conceptually
- `proposal_ready`: a governed proposal representation is available
- `accepted`: user accepted the proposal, but no application occurred in F9
- `cancelled`: popup flow was abandoned and overlay should be cleared
- `regenerate_requested`: a new proposal attempt was requested using current instruction text
- `error`: popup flow cannot continue safely

## Failure modes

- `missing_selection`
- `empty_instruction`
- `stale_selection`
- `proposal_generation_unavailable`
- `pending_overlay_conflict`

Interpretation:

- `missing_selection`: no valid `TextSelectionRef` is available
- `empty_instruction`: no meaningful transform instruction was provided
- `stale_selection`: selection no longer matches current document version or hash
- `proposal_generation_unavailable`: proposal creation is not available in current phase or prepared state
- `pending_overlay_conflict`: popup flow conflicts with an already active governed overlay state

## Relationship to transformation_core

This popup captures and advances the flow of:

- `TransformInstruction`
- `TransformProposal`
- `PatchPreviewOverlay`

But it must not redefine those models.

The popup is one consumer surface of the transformation-core vocabulary.

## Relationship to Document Viewer

`Document Viewer` supplies readonly selection context.

The popup may be opened from viewer selection, but the viewer remains readonly and separate from popup instruction capture.

The popup must not turn the viewer into an editor.

## Relationship to text_selection

The popup requires valid governed `TextSelectionRef` state.

If `selection_ref` is missing, invalid, or stale, transformation flow must be blocked.

## Relationship to future runtime

The popup does not validate determinism.
The popup does not apply patches.
The popup does not mutate files.

Those concerns remain future runtime responsibilities.

The popup may conceptually expose an optional `Ver cambios` path to governed diff visualization, but side-by-side diff remains external to popup semantics and is defined in `docs/specs/diff_view.md`.

## Invariants

- `INV-TPOP-001`: `Selection Preview` MUST remain readonly
- `INV-TPOP-002`: instruction editing MUST modify only instruction state, not the document
- `INV-TPOP-003`: `Generar propuesta` MUST create conceptual proposal state only
- `INV-TPOP-004`: `Regenerar` MUST create a new `proposal_id`
- `INV-TPOP-005`: `Cancelar` MUST clear active overlay representation
- `INV-TPOP-006`: `Aceptar` MUST NOT apply changes in F9
- `INV-TPOP-007`: popup flow MUST be blocked when no valid `selection_ref` exists

## Related specs

- `docs/specs/transformation_core.md`
- `docs/specs/text_selection.md`
- `docs/specs/document_viewer.md`
- `docs/specs/document_governed_editing.md`
- `docs/specs/document_patch_runtime.md`
- `docs/specs/diff_view.md`
