# flow_control_policy

## Purpose

Define F9-ready declarative rules for confirmation friction and pending action grouping.

This spec prevents over-confirmation while keeping risky or mutable operations human-governed.

## Scope F9

F9 may declare:

- when confirmation is unnecessary
- when confirmation is required
- how many confirmations a logical flow may request
- when inline confirmation is preferred
- when modal confirmation is allowed

No execution behavior is implemented by this spec.

## Confirmation rules

Do not confirm:

- readonly actions
- low-risk derivations

Confirm only for:

- risk escalation
- `ARTIFACT` mutation
- external execution
- ambiguity
- irreversibility

At most one confirmation should be requested per logical flow.

Pending actions should be grouped when they belong to the same logical flow.

Inline confirmation is preferred.

Modal confirmation is reserved for:

- high risk
- external execution
- irreversible action

## Forbidden responsibilities

The flow-control policy must not:

- execute actions
- decide inside Slint
- call tools
- call LLM providers
- bypass human-in-the-loop policy
- reinterpret project manifests

## Future F10/F11 notes

F10 may connect these rules to a future pending-action model.

F11 should audit that friction control does not silently turn confirmation into implicit approval.
