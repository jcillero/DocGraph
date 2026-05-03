# project_settings_popup

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Project settings` popup as the project-scoped configuration surface in DocGraph.

## Core rule

`Project settings` is configuration only.

It does not execute.
It does not resolve policy.
It does not trigger tools.
It does not mutate runtime directly.

## Scope

`Project settings` may capture project-scoped configuration such as:

- `project_profile`
- declarative sandbox references
- optional tool defaults
- project metadata

## Forbidden contents

`Project settings` MUST NOT contain:

- raw credentials
- secrets
- binary references
- execution toggles
- host-specific absolute paths

## Portability rule

Project-scoped settings must remain portable across machines.

They may live inside project structure only when represented as declarative portable configuration.

## Popup contract

The popup is independent.

It is not a tab inside a shared Preferences panel.

The popup captures:

- project-scoped configuration draft
- user confirmation intent

The popup emits:

- `ConfigurationIntent`

The popup MUST NOT emit:

- `ExecutionIntent`

## Relationship to credentials

`Project settings` is not a credentials surface.

Credential references may be connected conceptually in future governed flows, but credential management remains external to this popup.

## Invariants

- `INV-PROJ-SETTINGS-001`: `Project settings` MUST remain a dedicated popup surface.
- `INV-PROJ-SETTINGS-002`: `Project settings` MUST remain project-scoped configuration only.
- `INV-PROJ-SETTINGS-003`: `Project settings` MUST NOT contain secrets or raw credentials.
- `INV-PROJ-SETTINGS-004`: `Project settings` MUST remain portable across machines.
- `INV-PROJ-SETTINGS-005`: `Project settings` MUST NOT trigger tools, LLM execution, external dependencies, or runtime mutation.

## Forbidden responsibilities

This spec must not:

- implement popup runtime
- define secret storage
- enable tool execution
- enable LLM execution
- mutate filesystem
- open F10

## Related specs

- `docs/specs/ui_preferences_popups.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/credentials_policy.md`
