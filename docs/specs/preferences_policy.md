# preferences_policy

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define user and project preferences as non-sensitive configuration.

This spec governs preference content.
Popup surfaces and menu routing are defined separately.

## Core rule

Preferences are non-secret behavior configuration.
Preferences are not credentials.
Preferences are not execution authority.

## Preference scopes

- `app_preferences`
- `user_preferences`
- `project_preferences`

Popup mapping:

- `App preferences` edits global non-sensitive user-scoped configuration
- `Project settings` edits project-scoped portable configuration
- `Credentials management` is not a preference surface and is governed separately

## Allowed preference examples

- `language`
- `theme`
- `default_agent_profile`
- `lume_help_level`
- `show_onboarding_modal`
- `desired_llm_mode`
- `default_tool_surface_scope`
- `preferred_project_profile`
- `document_default_language`
- `confirmation_friction_level`

## LLM preference contract

Preferences may store a desired LLM mode only as non-secret user intent.

Conceptual `UserPreferences` fields may include:

- `desired_llm_mode`
- `interaction_mode_preference`
- `privacy_level`
- `tool_visibility_preference`
- `explanation_level`

Allowed values:

- `OFF`
- `LOCAL`
- `CLOUD`

`interaction_mode_preference` may use:

- `GUIDED`
- `ASSISTED`

`privacy_level` may use:

- `STRICT`
- `BALANCED`
- `PERMISSIVE`

`explanation_level` may use:

- `LOW`
- `MEDIUM`
- `HIGH`

Rules:

- `desired_llm_mode` is not `effective_llm_mode`
- `interaction_mode_preference` is advisory only
- `privacy_level` is advisory only
- `tool_visibility_preference` is advisory only
- `explanation_level` is advisory only
- changing `desired_llm_mode` does not call providers
- changing `desired_llm_mode` does not load local models
- changing `desired_llm_mode` does not validate credentials
- changing `desired_llm_mode` does not execute tools
- changing `desired_llm_mode` does not enable ActionResolution execution
- future mode availability must be derived from prepared `LLMCapabilityState`

If `desired_llm_mode` cannot be satisfied, the effective mode must fall back to `OFF` and `GUIDED` interaction with reason codes.

Preferences influence future prepared state only.

They do not override policy, grant authority, or authorize restricted context exposure.

## Forbidden preference contents

Preferences MUST NOT contain:

- API keys
- tokens
- passwords
- private keys
- provider secrets
- raw credentials
- sensitive file contents
- hidden user profiling data

## Project Setup / Settings relationship

Project Setup may capture initial preferences.
Project Settings may edit preferences later.

UI captures preference drafts.
Future governed runtime validates and persists.
Changing preferences MUST NOT enable execution by itself.

In the governed Preferences menu model:

- `App preferences` MUST remain user-scoped
- `App preferences` MUST live outside project structure
- `Project settings` MAY persist project-scoped configuration inside project structure
- `Preferences` popups emit `ConfigurationIntent`, not `ExecutionIntent`

## Invariants

- `INV-PREF-001`: preferences MUST NOT contain secrets.
- `INV-PREF-002`: preferences MAY configure behavior, language, theme, defaults, and help level.
- `INV-PREF-003`: preferences MUST NOT grant execution authority.
- `INV-PREF-004`: preferences MUST NOT bypass project policy, tool policy, LLM policy, or `ActionResolution`.
- `INV-PREF-005`: project preferences MUST remain portable and avoid host-specific absolute paths.
- `INV-PREF-006`: preferences may be edited through Project Settings in a future governed flow.
- `INV-PREF-007`: preference changes affecting risk or policy require confirmation.
- `INV-PREF-008`: preferences must be distinguishable from credentials in schema and storage.
- `INV-PREF-009`: `App preferences` MUST NOT contain filesystem paths, execution toggles, tool activation flags, or LLM execution authority.
- `INV-PREF-010`: `Preferences` surfaces MUST remain separate from `Tools`.
- `INV-PREF-011`: `desired_llm_mode` MAY store `OFF`, `LOCAL`, or `CLOUD` intent only.
- `INV-PREF-012`: changing LLM preferences MUST NOT trigger provider calls, local model usage, tool execution, credential validation, or filesystem mutation.
- `INV-PREF-013`: preferences MUST NOT decide effective LLM availability.
- `INV-PREF-014`: preference values are advisory and MUST NOT override policy or consent.
- `INV-PREF-015`: privacy-related preferences may affect future context filtering but MUST NOT grant execution authority.

## Forbidden responsibilities

This spec must not:

- implement preferences runtime
- create UI
- write project files
- validate live credentials
- enable LLM
- call LLM providers
- resolve effective LLM mode
- enable tools
- execute tools
- open F10

## Related specs

- `docs/specs/ui_preferences_popups.md`
- `docs/specs/project_settings_popup.md`
- `docs/specs/credentials_policy.md`
- `docs/specs/user_profile_policy.md`
- `docs/specs/privacy_and_consent_model.md`
