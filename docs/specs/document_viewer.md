# document_viewer

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Document Viewer` surface for DocGraph.

DocGraph does not edit documents.
DocGraph transforms documents.

This spec fixes the viewer as a readonly rendering surface that may expose text selection and future visual proposal overlays without becoming an editor or patch runtime.

## Role

`Document Viewer`:

- shows governed documents
- is readonly
- does not save
- does not edit
- does not apply changes

The viewer renders prepared governed state only.

## Core rule

The viewer is a readonly surface.

It may present content, selection, and visual proposal context.

It must not become:

- an editor
- a file writer
- a patch runtime
- a tool executor
- an LLM invocation surface

## Allowed behavior

The viewer may:

- render a governed document
- allow text selection
- copy visible text
- copy a structured reference
- show visual overlays received as prepared state
- open a contextual popup from a text selection through UI intent only

These actions do not imply mutation or execution.

When text selection exists, it should be represented as governed `TextSelectionRef` state rather than as a new document copy.

## Forbidden behavior

The viewer must not:

- edit directly
- write to a file
- save changes
- mutate `SOURCE`
- mutate `DERIVED`
- apply a proposal without acceptance
- execute tools
- invoke LLM
- become a disguised editor

## Relationship to transformation specs

This document does not redefine selection, popup, preview, diff, or transformation-core contracts.

It must align with:

- `docs/specs/text_selection.md`
- `docs/specs/transform_popup.md`
- `docs/specs/patch_preview.md`
- `docs/specs/diff_view.md`
- `docs/specs/transformation_core.md`

In this viewer spec:

- text selection is treated as prepared viewer interaction state
- popup opening is treated as UI intent only
- overlays are treated as prepared visual state only
- patch preview and diff remain external contracts

The selection structure itself is defined in `docs/specs/text_selection.md`.

## States

Prepared viewer state may include:

- `readonly`
- `selection_active`
- `overlay_active`
- `missing`
- `stale`
- `error`

Interpretation:

- `readonly`: valid rendered document context with no mutation authority
- `selection_active`: readonly viewer state with active text selection
- `overlay_active`: readonly viewer state with prepared visual overlay present
- `missing`: source reference no longer resolves
- `stale`: rendered context exists but is no longer current enough for intended derived interaction
- `error`: rendering or preparation failed in a governed way

## Relationship to Workspace Tabs

`Document Viewer` is rendered inside `Document Viewer Tab`.

The tab identifies the governed target reference.
The viewer renders the prepared document context for that tab.

The viewer does not own tab identity, permission, or runtime authority.

## Relationship to ActiveObjectContext

The viewer may render the currently selected document context prepared through `ActiveObjectContext`.

Viewer visibility does not create or invent active context by itself.

Selection inside the viewer must not silently authorize any transformation.

Selection captured in the viewer must retain `document_ref` and document-integrity linkage through governed selection reference state.

## Relationship to Document Tree

`Document Tree` prepares or requests opening the governed document target.

The viewer shows the resolved governed document.

The viewer does not import, discover, or expose arbitrary host files.

## Relationship to overlays

Visual overlays may be rendered only when prepared state provides them.

An overlay:

- is visual only
- is not persistence
- is not acceptance
- is not application

The viewer may show overlay presence, but it must not apply the underlying proposal.

Pending proposal highlight semantics are defined in `docs/specs/patch_preview.md`.

## Failure modes

- `source_not_resolvable`
- `unsupported_render_format`
- `selection_invalid`
- `overlay_stale`
- `forbidden_edit_attempt`

Interpretation:

- `source_not_resolvable`: governed document target no longer resolves
- `unsupported_render_format`: target exists but cannot be rendered by the current governed viewer contract
- `selection_invalid`: prepared text selection is not valid for the current rendered context
- `overlay_stale`: overlay state exists but no longer matches current viewer context safely
- `forbidden_edit_attempt`: direct edit behavior was attempted on the readonly viewer

## Invariants

- `INV-DVIEW-001`: `Document Viewer` MUST remain readonly
- `INV-DVIEW-002`: `Document Viewer` MUST NOT save or mutate files
- `INV-DVIEW-003`: `Document Viewer` MUST NOT apply proposals by itself
- `INV-DVIEW-004`: text selection MAY exist without turning the viewer into an editor
- `INV-DVIEW-005`: overlays MAY be displayed as prepared visual state only
- `INV-DVIEW-006`: the viewer MUST remain separate from patch runtime and transformation execution
- `INV-DVIEW-007`: the viewer MUST NOT execute tools or invoke LLM

## Related specs

- `docs/specs/workspace_tabs.md`
- `docs/specs/document_tree.md`
- `docs/specs/active_object_context.md`
- `docs/specs/text_selection.md`
- `docs/specs/transform_popup.md`
- `docs/specs/patch_preview.md`
- `docs/specs/diff_view.md`
- `docs/specs/transformation_core.md`
