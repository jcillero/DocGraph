# lume_onboarding_model

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define Lume as governed onboarding that works without LLM and can be expanded with future LLM assistance.

## Core rule

DocGraph MUST remain usable when no LLM engine, model, provider, credential, or executable tool is available.

Lume onboarding MUST distinguish:

- `Static Mode`: local declarative help, no LLM.
- `Assisted Mode`: future LLM-assisted help, only when explicitly available.

For LLM capability governance, these map to interaction modes:

- `Static Mode` corresponds to `GUIDED`.
- `Assisted Mode` corresponds to `ASSISTED`.

`GUIDED` must remain available when `desired_llm_mode`, `effective_llm_mode`, or capability state resolves to `OFF`.

## Lume modes

### Static Mode

- default when no LLM is available
- uses declarative help, i18n, and resources
- may use `lume_help_tree` as a navigable static help structure
- may explain UI objects, phases, limitations, project setup, and available surfaces
- may guide the user through project creation and configuration
- does not interpret free-form intent as executable action
- does not call providers
- does not execute tools

### Assisted Mode

Future only.

- may use governed LLM execution when F10+ opens it
- must still respect `ActionResolution`, tool policy, credentials policy, and Tool Surface Resolver
- must never expose credentials to LLM context

## StartupCapabilityState conceptual fields

- `llm_available`
- `local_model_available`
- `credential_ref_present`
- `operational_tools_available`
- `executable_tools_count`
- `declared_tools_count`
- `project_loaded`
- `declarative_help_available`
- `current_lume_mode`

`StartupCapabilityState` may also feed a future bottom status bar.

The status bar may expose compact capability state, while Lume explains it.

## LLMCapabilityState onboarding contract

Before onboarding chooses guided or assisted presentation, the prepared capability state must include:

- `desired_llm_mode`: `OFF | LOCAL | CLOUD`
- `effective_llm_mode`: `OFF | LOCAL | CLOUD`
- `interaction_mode`: `GUIDED | ASSISTED`
- `llm_available`
- `provider_configured`
- `credential_ref_present`
- `local_engine_available`
- `local_model_available`
- `policy_allows_local`
- `policy_allows_cloud`
- `tool_surface_available`
- `execution_enabled`
- `reason_codes`

Rules:

- `desired_llm_mode=OFF` resolves to `effective_llm_mode=OFF` and `interaction_mode=GUIDED`
- missing local engine, missing local model, or local policy block resolves requested `LOCAL` to `OFF/GUIDED` with reason codes
- missing provider, missing `credential_ref`, or cloud policy block resolves requested `CLOUD` to `OFF/GUIDED` with reason codes
- available and policy-allowed `LOCAL` resolves to `LOCAL/ASSISTED`
- available and policy-allowed `CLOUD` resolves to `CLOUD/ASSISTED`
- no mode grants tool execution authority

Required reason codes include:

- `llm_off_by_preference`
- `local_engine_missing`
- `local_model_missing`
- `local_policy_blocked`
- `cloud_provider_missing`
- `cloud_credential_ref_missing`
- `cloud_policy_blocked`
- `execution_not_open`
- `assisted_mode_unavailable`
- `guided_fallback_active`

## User-facing onboarding goals

At first launch, Lume should explain:

- DocGraph can be used without AI
- LLM assistance is optional and may be activated later
- current system capability state
- what the user can do now
- what is declared but not executable
- next recommended setup step

## Minimal onboarding flow

1. welcome
2. show capability state
3. explain current mode
4. guide project creation or opening
5. guide project profile selection
6. guide preferences
7. optionally explain credentials as future references, not secrets
8. finish with "ready for configuration" or future equivalent state

## Required user message in Static Mode

Include this conceptual message:

"Lume is running in local help mode. No LLM is active. You can still create and configure projects, inspect documents, use available non-LLM surfaces, and activate assisted capabilities later."

## OFF-mode guided menu

When `effective_llm_mode=OFF`, onboarding may present a deterministic menu such as:

1. Open project
2. Review documents
3. Open Tools Panel
4. Configure local LLM
5. Configure cloud LLM
6. Show capability state
7. Open Lume Help

This menu is conceptual only.

It may emit navigation, explanation, or configuration intent, but it must not execute tools, call providers, validate credential secrets, mutate files, approve actions, or execute ActionResolution.

## Relationship to Project Setup / Settings

- Lume guides.
- Project Setup / Settings captures drafts.
- Future runtime validates and persists.
- Lume does not write project files.
- Lume does not decide profile, tools, credentials, or policies.

## Relationship to tools

- Lume may display effective tool surface summary.
- Lume must not inject raw full catalog by default.
- Lume must not execute tools.
- Lume may explain `declared`, `available`, `not executable`, and `future`.

## Relationship to credentials

- Lume must not require credentials for Static Mode.
- Lume must not see credential values.
- Missing credentials must not block core app usage.

## Invariants

- `INV-LUME-ONBOARD-001`: DocGraph MUST remain usable without any LLM engine, model, provider, credential, or executable tool.
- `INV-LUME-ONBOARD-002`: Lume MUST support Static Mode based on declarative local help.
- `INV-LUME-ONBOARD-003`: Lume MUST distinguish Static Mode from Assisted Mode.
- `INV-LUME-ONBOARD-004`: Lume MUST expose capability state instead of failing silently when LLM is unavailable.
- `INV-LUME-ONBOARD-005`: Missing LLM capability MUST NOT block project creation, project setup, preferences, readonly navigation, or declarative help.
- `INV-LUME-ONBOARD-006`: Lume MUST NOT require credentials in Static Mode.
- `INV-LUME-ONBOARD-007`: Lume MUST NOT execute tools, mutate files, or approve actions in either mode.
- `INV-LUME-ONBOARD-008`: Assisted Mode MUST remain governed by LLM policy, credentials policy, Tool Surface Resolver, and ActionResolution.
- `INV-LUME-ONBOARD-009`: `OFF` mode MUST resolve to guided onboarding and MUST NOT be treated as an error.
- `INV-LUME-ONBOARD-010`: `desired_llm_mode` MUST NOT be treated as `effective_llm_mode`.
- `INV-LUME-ONBOARD-011`: capability state MUST expose missing, blocked, unavailable, and guided fallback reason codes.
- `INV-LUME-ONBOARD-012`: `credential_ref` MUST NOT grant execution authority.
- `INV-LUME-ONBOARD-013`: guided menu fallback MUST NOT emit executing intents.

## Forbidden responsibilities

Lume onboarding must not:

- call LLM providers
- execute tools
- validate credentials
- store secrets
- write project files
- mutate `project_manifest`
- decide project policy
- become Project Setup runtime
- bypass `ActionResolution`
- open F10

## Related specs

- `docs/specs/lume_onboarding_modal.md`
- `docs/specs/lume_help_popup.md`
- `docs/specs/lume_interaction_model.md`
- `docs/specs/project_setup_popup.md`
- `docs/specs/llm_core.md`
- `docs/specs/credentials_policy.md`
- `docs/specs/llm_tool_surface_resolution.md`
