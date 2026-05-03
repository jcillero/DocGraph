# ui_preferences_popups

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed Preferences menu and popup system for DocGraph.

## Menu model

`Preferences` is a top-level menu entry.

It is separate from `Tools`.

It exposes exactly:

- `App preferences`
- `Project settings`
- `Credentials management`

Each entry opens a distinct governed popup surface.

There is no shared Preferences panel.
There are no Preferences tabs.

## Core rule

The Preferences menu emits intent only.

It does not execute.
It does not bypass policy.
It does not mutate runtime.

## App Preferences popup

Purpose:

Global non-sensitive configuration.

Fields may include:

- `ui_language`
- `theme_mode`
- `lume_assistance_level`
- `ui_behavior_flags`

Rules:

- no secrets
- no filesystem paths
- no execution toggles
- no tool activation
- no LLM execution

Persistence:

- user-scoped
- outside project structure

## Project Settings popup

Purpose:

Project-scoped configuration.

Fields may include:

- `project_profile`
- declarative sandbox references
- optional tool defaults
- metadata

Rules:

- no credentials
- no execution
- no binary references
- portable across machines

Persistence:

- inside project structure

## Credentials Management popup

Purpose:

Secure management of credential references.

Fields may include:

- `provider_id`
- `credential_ref`
- `status`

Status remains conceptual, for example:

- `configured`
- `missing`

Rules:

- NEVER store raw secrets in `project/`
- NEVER store raw secrets in `resources/`
- UI MUST NOT display secrets
- credentials MUST NOT be exposed to LLM
- credentials MUST NOT trigger execution
- credentials MUST NOT grant execution authority

Behavior:

- manages references only
- integrates with `docs/specs/credentials_policy.md`

## Popup system rules

Each popup MUST:

- be independent
- capture configuration only
- require explicit confirmation
- not execute any action
- not mutate runtime
- not trigger tools

Popup emits:

- `ConfigurationIntent`

Popup MUST NOT emit:

- `ExecutionIntent`

## Governed popup i18n keys

Each governed Preferences popup must use i18n-managed title and description keys.

Required keys:

- `popup.preferences.app.title`
- `popup.preferences.app.description`
- `popup.preferences.project.title`
- `popup.preferences.project.description`
- `popup.credentials.title`
- `popup.credentials.description`

Popup titles must remain short.

Descriptions must remain informational only.

Descriptions must not imply execution, tool availability, LLM availability, or secret disclosure.

## Domain separation

Strict boundaries:

- `Preferences` -> configuration only
- `Tools` -> capabilities and execution preparation
- `External Dependencies` -> runtime requirements
- `Credentials` -> secure access references

Never:

- `Preferences` triggering tools
- `Credentials` enabling execution
- `Project settings` storing secrets
- `App preferences` altering runtime behavior directly

## Invariants

- `INV-PREF-001`: Preferences MUST NOT contain secrets.
- `INV-PREF-002`: Preferences MUST NOT execute actions.
- `INV-PREF-003`: Preferences MUST NOT bypass `ActionResolution`.
- `INV-PREF-004`: Project settings MUST remain portable.
- `INV-CRED-001`: Credentials MUST NOT be stored in project files.
- `INV-CRED-002`: Credentials MUST NOT be exposed to LLM.
- `INV-CRED-003`: Credentials MUST NOT imply execution authority.
- `INV-UI-POPUP-001`: each Preferences entry MUST open a dedicated popup.
- `INV-UI-POPUP-002`: no shared Preferences panel with tabs is allowed.
- `INV-UI-POPUP-003`: governed Preferences popup titles and descriptions MUST come from i18n resources.

## Non-goals

Do not implement:

- runtime logic
- tool execution
- credential storage engine
- encryption
- API calls
- filesystem mutation
- sandbox interaction
- dependency installation

## Related specs

- `docs/specs/app_main_menu.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/credentials_policy.md`
- `docs/specs/project_settings_popup.md`
