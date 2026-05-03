# user_profile_policy

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define a governed, privacy-safe conceptual `UserProfile` contract for future Lume and LLM-facing flows.

This spec does not create persistence, authentication, telemetry, identity resolution, or real user data handling.

## Core rule

User profile data is optional, minimal, and non-sensitive by default.

Core DocGraph operation must not depend on a populated user profile.

## UserProfile model

Conceptual structure:

- `user_id`
- `profile_version`
- `preferences_ref`
- `consent_state_ref`
- `allowed_context_refs`
- `denied_context_refs`

Field meaning:

- `user_id`: opaque, non-sensitive identifier only
- `profile_version`: declarative schema or contract version
- `preferences_ref`: conceptual reference to governed non-secret preferences
- `consent_state_ref`: conceptual reference to explicit consent state
- `allowed_context_refs`: optional bounded context references explicitly allowed for future governed use
- `denied_context_refs`: optional bounded context references explicitly denied for future governed use

## UserProfile rules

- no personal sensitive data is required
- no implicit data collection is allowed
- user identity remains opaque
- profile presence is optional, not required for operation
- profile references do not imply persistence by this spec
- profile references do not imply authorization
- profile references do not imply execution permission

## Relationship to preferences and consent

`UserProfile` is a lightweight coordination surface only.

It may conceptually point to:

- `UserPreferences`
- `ConsentState`

It must not absorb or duplicate:

- credential material
- execution policy
- tool authorization
- provider configuration

## Relationship to Lume and future LLM interaction

Lume may explain the existence or absence of governed profile information.

Lume may explain:

- whether a profile is present
- whether preferences are available
- whether consent is present
- whether context references are allowed or denied conceptually

Lume must not:

- infer missing profile values
- create hidden profile data
- persist profile data implicitly
- expose denied context references

## Allowed profile exposure

Future governed LLM-facing exposure may use only:

- opaque `user_id`, when required for traceability
- non-sensitive preference summaries
- explicit consent-state summaries
- allowed context-reference summaries

The profile must not expose:

- credentials
- secrets
- denied context references
- raw host-specific filesystem paths
- hidden prompts

## Invariants

- `INV-USER-PROFILE-001`: user profile is optional and non-blocking.
- `INV-USER-PROFILE-002`: no sensitive data is required for normal operation.
- `INV-USER-PROFILE-003`: `user_id` MUST remain opaque and non-sensitive.
- `INV-USER-PROFILE-004`: no implicit user-data collection is allowed.
- `INV-USER-PROFILE-005`: profile references MUST NOT grant execution authority.
- `INV-USER-PROFILE-006`: profile references MUST NOT bypass policy, consent, or ActionResolution.
- `INV-USER-PROFILE-007`: credentials MUST NOT appear in the profile model.
- `INV-USER-PROFILE-008`: this spec defines no runtime persistence in F9/F10_PREP.

## Non-goals

- no runtime storage implementation
- no user database
- no authentication system
- no encryption-layer definition
- no cloud sync
- no telemetry
- no behavioral tracking
- no profile mutation flows
- no UI editing forms
- no real data handling

## Related specs

- `docs/specs/privacy_and_consent_model.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/lume_interaction_model.md`
- `docs/specs/llm_core.md`
