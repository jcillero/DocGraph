# sandbox_viewer

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Sandbox Viewer` surface for DocGraph.

The sandbox is represented as an inspectable governed object that may be selected from `Document Tree` and opened in `Workspace Tabs`.

## Role

`Sandbox Viewer` is inspection of sandbox context.

It is not:

- an executor
- a filesystem explorer
- a write-back UI

It does not execute tools.
It does not mutate files.
It does not perform write-back.

## Core rule

`Sandbox Viewer` renders prepared governed sandbox state only.

It does not discover arbitrary host filesystem state by itself.
It does not authorize mutation.
It does not replace runtime policy.

## Conceptual content

Prepared sandbox-view state may include:

- `sandbox_ref`
- `inputs`
- `scope`
- selected source-folder metadata, without absolute path as portable truth
- `working_copy_future`
- proposed operations, future
- capability state
- readonly status

Interpretation:

- `sandbox_ref`: governed sandbox identity
- `inputs`: declared or prepared sandbox-related inputs
- `scope`: governed sandbox scope boundary
- selected source-folder metadata: descriptive context about the selected source folder without persisting host absolute path as portable authority
- `working_copy_future`: future working-copy representation only
- proposed operations, future: non-executable descriptions of possible later governed actions
- capability state: readonly summary of currently available or unavailable sandbox capabilities
- readonly status: explicit declaration that the current viewer is non-mutating in F9

## Allowed actions

The viewer may allow:

- inspect
- copy a structured reference
- send sandbox context to chat
- view status

These are inspection and reference actions only.

## Forbidden actions

The viewer must not allow:

- execute filesystem operations
- mutate folders
- write to the original source
- launch tools directly
- apply changes
- perform write-back

## Relationship to ActiveObjectContext

- `selected_sandbox` may hold the currently selected sandbox reference
- `focused_family = Sandbox` when sandbox activation is explicit and observable
- chat may use sandbox context when `Sandbox` is the focused family and a valid selection exists

The viewer must not invent sandbox context when no governed `selected_sandbox` exists.

## Relationship to Document Tree

`Document Tree` may expose sandbox context as governed node.

Selecting that node may prepare `selected_sandbox` and request opening or activation of `Sandbox Viewer`.

The tree does not turn sandbox context into a free folder browser.

## Relationship to Workspace Tabs

`Sandbox Viewer` is hosted inside `Sandbox Viewer Tab`.

Opening the same `sandbox_ref` must reuse the existing governed tab rather than duplicate it.

The tab hosts the view; it does not execute sandbox behavior.

## Relationship to local sandbox context

This viewer consumes the policy and context declared in `docs/specs/local_sandbox_context.md`.

The original selected source folder remains readonly.

Any future mutation must remain limited to future governed working-copy scope.

## States

Prepared viewer state may include:

- `declared`
- `readonly`
- `missing`
- `invalid`
- `future_actionable`
- `stale`

Interpretation:

- `declared`: sandbox context is declared and available as governed inspection state
- `readonly`: sandbox context is visible but non-mutable in current phase
- `missing`: `sandbox_ref` no longer resolves
- `invalid`: prepared sandbox state is not valid for governed rendering
- `future_actionable`: sandbox may become actionable only in a future governed phase
- `stale`: prepared sandbox context no longer safely matches current declared state

## Failure modes

- `sandbox_ref_missing`
- `source_folder_unavailable`
- `absolute_path_persistence_detected`
- `mutation_requested_in_f9`
- `writeback_blocked`

Interpretation:

- `sandbox_ref_missing`: the selected governed sandbox reference no longer resolves
- `source_folder_unavailable`: descriptive source-folder context cannot be prepared
- `absolute_path_persistence_detected`: host absolute path is being treated as portable project truth
- `mutation_requested_in_f9`: a mutation attempt was requested through a non-mutating F9 viewer
- `writeback_blocked`: a write-back attempt was requested even though write-back is not allowed here

## Invariants

- `INV-SVIEW-001`: `Sandbox Viewer` MUST remain inspection-only in F9
- `INV-SVIEW-002`: `Sandbox Viewer` MUST NOT become a filesystem explorer
- `INV-SVIEW-003`: `Sandbox Viewer` MUST NOT execute filesystem operations
- `INV-SVIEW-004`: selected source-folder metadata MUST NOT persist host absolute paths as portable truth
- `INV-SVIEW-005`: sandbox chat context MAY be referenced only from valid governed sandbox selection
- `INV-SVIEW-006`: opening the same `sandbox_ref` MUST reuse the same governed tab identity

## Related specs

- `docs/specs/local_sandbox_context.md`
- `docs/specs/workspace_tabs.md`
- `docs/specs/active_object_context.md`
- `docs/specs/document_tree.md`
- `docs/specs/ui_core.md`
