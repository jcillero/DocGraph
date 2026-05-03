# document_governed_editing

## Purpose

Document the future governed document editing model without implementing runtime, UI behavior, or new crates.

This is a proposal for a later phase above the CLOSED F8 workspace model.

## Document classes

- `SOURCE`: original source document, readonly.
- `DERIVED`: text, pages, chunks, technical derivatives, or future semantic derivatives; readonly and regenerable.
- `ARTIFACT`: user or LLM-assisted work product; mutable only through governed patch acceptance.

`SOURCE` and `DERIVED` may be used as context, but they are not mutation targets.

## Active artifact and active tab

The active workspace tab defines the default working context.

- active Document Viewer: document context
- active Figure Inspector: figure or figure-metadata context
- future Table Inspector: table context
- future Equation Inspector: equation context

If the active target is ambiguous, the system must request confirmation before building an edit request.

## Readonly viewer selection

The viewer remains readonly.

Future behavior may allow the user to select text in the viewer and open a contextual action, for example through right click or a controlled action surface.

The contextual popup contract is defined in `docs/specs/transform_popup.md`.

The popup input captures the user's instruction and converts it into a structured governed request containing:

- document reference
- selection reference
- active context
- user instruction
- optional word-count target
- optional length constraint

The selection does not make the viewer an editor.

## Governed modification flow

The user does not directly edit documents.

The future workflow is:

1. user selects, comments, or gives an instruction
2. LLM proposes a change
3. system validates the proposal
4. user accepts, rejects, or requests regeneration
5. runtime applies the accepted patch only after acceptance
6. accepted modification creates a version or `patch_log`

The canonical proposal and regeneration vocabulary must follow `docs/specs/transformation_core.md` and `docs/specs/transform_popup.md`.

## Preview requirement

Every mutation proposal must produce a preview before application.

Preview is mandatory for:

- text replacement
- paragraph replacement
- section rewrite
- structural movement
- table metadata update
- figure metadata update
- equation update

## Versioning and patch log

Every accepted modification must be traceable.

The future runtime must create either:

- a new artifact version
- a `patch_log` entry
- both, when required by the artifact policy

The log must preserve enough information to audit, reverse, or replay the accepted change deterministically.

Future structured memory of attempts, proposals, and decisions is governed separately by `docs/specs/transform_timeline.md` and must not be collapsed into the same role as `patch_log`.

## Phase limits

This spec does not implement:

- document mutation runtime
- editor UI
- rich drafting surface
- renderer or export pipeline
- LLM-driven filesystem mutation
- new crates

It only records the governed editing proposal for future F10/F11 alignment.
