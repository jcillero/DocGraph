# action_policy

## Purpose

Define the F9-ready declarative governance model for action risk, decision outcomes, and human-in-the-loop approval.

This spec does not implement action execution, tool execution, LLM invocation, or filesystem mutation.

## Scope F9

F9 may declare:

- risk levels
- allow/confirm/reject decisions
- human-in-the-loop policy
- document mutation constraints
- blocked action identifiers

The policy is declarative and read-only in F9.

## Risk levels

- `read_only`
- `low`
- `medium`
- `high`
- `forbidden`

`forbidden` cannot be overridden by user confirmation.

## Decision outcomes

- `allow`
- `confirm`
- `reject`

These outcomes describe future policy resolution only. They do not execute anything in F9.

## Document rules

- `SOURCE`: readonly
- `DERIVED`: readonly
- `ARTIFACT`: confirm before mutation

`SOURCE` and `DERIVED` may provide context but must not become mutation targets.

## Blocked actions

- `mutate_source_document`
- `mutate_derived_document`
- `llm_direct_filesystem_write`
- `execute_unregistered_tool`

## Forbidden responsibilities

The action policy must not:

- duplicate the `project_runtime` pipeline
- execute tools
- invoke LLM providers
- mutate files
- authorize unregistered tools
- place policy decisions in UI crates

## Future F10/F11 notes

F10 may introduce policy resolution over concrete action requests.

F11 should audit that forbidden actions remain non-overridable and that human approval is recorded before any future mutable execution.
