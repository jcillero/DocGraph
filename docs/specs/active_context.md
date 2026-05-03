# active_context

## Purpose

Define the future active-context model for referenced document work.

This is a conceptual contract only. It does not implement UI state or runtime behavior.

`active_context` is derived usage context. It is not the canonical selection model.

The canonical cross-family selection model is defined in `docs/specs/active_object_context.md`.

## Core rule

The active workspace tab may contribute default context for user instructions, but it does not replace `ActiveObjectContext`.

Chat orders apply to prepared active context only when a valid selected family and target exist, or when the user provides an explicit structured reference.

## Active targets

Future context may include:

- active document
- active sandbox
- active figure
- active table
- active equation
- active patch preview

The active target is derived from `ActiveObjectContext`, the selected workspace tab, and controlled view state.

## Context by tab

- Document Viewer: document or selection context
- Sandbox View: sandbox context
- Figure Inspector: figure and figure-metadata context
- future Table Inspector: table context
- future Equation Inspector: equation context
- future Patch Preview: proposal-review context

These tabs remain controlled views, not independent domain runtimes.

Opening or activating a tab does not silently erase other-family selections.

## Confirmation rule

If the target is ambiguous, stale, hidden, or conflicts with an explicit chat reference, the system must ask for confirmation before preparing a patch request.

Confirmation is required when:

- no active artifact exists
- no valid focused family exists
- multiple candidate artifacts match
- the active tab is readonly context only
- the request could target a `SOURCE` or `DERIVED` document
- the selected context changed after the request was prepared

## Context changes

Opening another file or view may change the default focused context only when the change is explicit and observable.

The previous context may remain referenceable only through explicit structured references.

Chat must not silently apply a request to an earlier tab after the active tab changes.

Chat must not use the last opened object as implicit fallback.
