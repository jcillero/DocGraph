# workspace_tabs

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Workspace Tabs` model for DocGraph.

Tabs represent governed objects opened in the workspace.

This spec defines tab kinds, tab identity, mouse interaction, relation to `ActiveObjectContext`, and the rule that tabs are representation only.

## Core rule

`Workspace Tabs` is a controlled representation surface.

Tabs do not execute actions.
Tabs do not mutate documents.
Tabs do not decide permissions.
Tabs do not replace runtime authority.

## Relationship to the central layout

The central layout remains:

`[ Tree ] [ Chat ] [ Tabs Workspace ]`

The tree selects governed objects.
The chat captures intention.
The tabs workspace hosts controlled views.
The viewers render context.

## Supported tab kinds

Current or declared kinds may include:

- `Document Viewer Tab`
- `Sandbox Viewer Tab`
- `Knowledge Viewer Tab`, when applicable
- `System View Tab`, future readonly inspection tab governed by `docs/specs/system_view.md`
- `Tool Output Viewer Tab`, future
- `Timeline Tab`, future inspection tab governed by `docs/specs/transform_timeline.md`

Concrete workspace identifiers may still use governed internal kinds such as:

- `viewer`
- `sandbox_view`
- `knowledge_detail`
- `system_view`
- `tool_result`

This document does not define text selection, popup, overlay, diff, or timeline behavior beyond tab-hosting boundaries.

## Tab identity

Tabs are identified by governed references, not by filesystem paths or visible titles.

Examples:

- `viewer::document::doc://f110`
- `viewer::sandbox::sandbox://org_main`
- `viewer::knowledge::knowledge://main/ref_001`
- `viewer::tool_output::tool_output://run_001`

Rule:

- one governed object = one reusable tab
- opening the same governed object activates the existing tab
- the system must not duplicate tabs for the same governed reference

## Conceptual model

Minimum conceptual fields may include:

- `tab_id`
- `tab_kind`
- `target_ref`
- `family`
- `state`
- `is_active`
- `is_closable`
- `is_pinned_future`
- `has_pending_visual_proposal`, when future integration requires it

## Mouse interaction

### Left-click on tab header

- activates the tab
- may update `focused_family` when the family is explicit and observable
- must not duplicate the tab
- must not execute anything

### Middle-click on tab header

- closes the tab when it is closable
- if a pending proposal is associated, close is conceptually blocked or requires confirmation
- does not save
- does not execute

### Right-click on tab header

Context menu may include:

- close tab
- close others
- close all
- copy reference
- pin tab, future optional

The context menu must not:

- execute a tool
- apply a proposal
- mutate a document
- open the filesystem

### Mouse interaction from Document Tree

Left-click from `Document Tree`:

- selects the governed object
- activates or creates the corresponding tab
- updates `ActiveObjectContext`

Middle-click from `Document Tree`:

- opens or activates the corresponding tab without duplication
- does not execute

## Relationship to ActiveObjectContext

- activating a `Document Viewer Tab` may focus family `Document`
- activating a `Sandbox Viewer Tab` may focus family `Sandbox`
- activating a tab does not clear selections of other families
- `focused_family` must be observable in the prepared state
- the system must not use hidden fallback to the last opened tab

Tab activation must not silently manufacture a new target reference when no governed selection or ref exists.

## Tab states

Prepared tab state may include:

- `active`
- `inactive`
- `readonly`
- `missing`
- `stale`
- `invalid`
- `closable`
- `pinned_future`
- `has_pending_visual_proposal`

Interpretation:

- `active`: current visible workspace tab
- `inactive`: open but not currently active
- `readonly`: valid and non-mutable in current phase or policy
- `missing`: target reference no longer resolves
- `stale`: target exists but prepared view state is no longer current enough
- `invalid`: tab kind or target is not valid for governed rendering
- `closable`: tab may be closed
- `pinned_future`: future optional pinned state only
- `has_pending_visual_proposal`: future readonly flag for proposal-related visual integration

## Relationship to viewers

`Document Viewer Tab` renders governed document context.

Its internal rendered surface is defined separately in `docs/specs/document_viewer.md`.

`Sandbox Viewer Tab` renders governed sandbox context.

Its internal rendered surface is defined separately in `docs/specs/sandbox_viewer.md`.

`Knowledge Viewer Tab` may render governed knowledge context when applicable.

Tabs host these views, but they do not replace the viewers and do not own filesystem or runtime authority.

## Relationship to Document Tree

The tree prepares selection and may request opening or activation of a governed tab.

Tabs do not replace tree navigation, and the tree does not create duplicate tabs for the same governed reference.

## Failure modes

- `duplicate_tab_ref`
- `stale_tab_ref`
- `missing_tab_target`
- `close_blocked_pending_proposal`
- `invalid_tab_kind`

Interpretation:

- `duplicate_tab_ref`: a second tab was attempted for the same governed reference
- `stale_tab_ref`: the tab target resolves, but view state is no longer current enough
- `missing_tab_target`: the governed reference no longer resolves
- `close_blocked_pending_proposal`: a close action was attempted while governed pending visual proposal rules block it
- `invalid_tab_kind`: tab state references an unsupported kind

## Invariants

- `INV-WTAB-001`: no duplicate tab may exist for the same governed reference
- `INV-WTAB-002`: tab identity MUST be based on governed reference, not path or title
- `INV-WTAB-003`: tabs are representation only
- `INV-WTAB-004`: tabs MUST NOT execute actions
- `INV-WTAB-005`: tabs MUST NOT mutate documents or filesystem state
- `INV-WTAB-006`: tabs MUST NOT decide permissions
- `INV-WTAB-007`: activating a tab MUST NOT erase other-family selections
- `INV-WTAB-008`: tabs host views; tabs do not execute logic

## Out of scope

- text selection
- popup behavior
- patch overlay
- diff rendering
- timeline behavior
- editing
- notebook behavior
- docking
- multi-window
- semantic orchestration
- patch application

## Related specs

- `docs/specs/workspace_layout.md`
- `docs/specs/document_tree.md`
- `docs/specs/active_object_context.md`
- `docs/specs/gui_objects_v1.md`
- `docs/specs/ui_core.md`
- `docs/specs/system_view.md`
