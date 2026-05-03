# privacy_and_consent_model

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed privacy and consent contract for future Lume and LLM-facing interaction.

This spec defines exposure boundaries only.

It does not implement storage, consent UI, provider invocation, or data processing.

## Core rule

Context exposure must be explicit, bounded, and privacy-safe.

Consent is required before any future LLM processing.

Consent does not imply execution permission.

## ConsentState model

Conceptual structure:

- `consent_version`
- `consent_given`
- `consent_scope`
- `consent_timestamp`

`consent_scope` may include:

- `allow_llm_processing`
- `allow_context_usage`
- `allow_tool_suggestions`

Rules:

- no implicit consent
- no hidden consent escalation
- consent is explicit or absent
- consent must remain separate from execution permission
- consent must remain separate from credential handling

## Context exposure contract

Allowed future context may include:

- explicit user input
- explicitly allowed knowledge sources
- `EffectiveToolSurfaceSummary`, when allowed
- non-sensitive system-state summaries

Forbidden future context includes:

- credentials
- secret values
- raw host-specific filesystem paths
- hidden system prompts
- full raw tool catalogs by default
- denied context references

Rules:

- context exposure must be explicit and bounded
- hidden context injection is forbidden
- exposure must respect consent and privacy level
- bounded context does not imply execution authority

## Privacy levels

The governed privacy levels are:

- `STRICT`
- `BALANCED`
- `PERMISSIVE`

### STRICT

- minimal context only
- no tool-surface exposure
- no inferred context
- guided interaction preferred

### BALANCED

- bounded context
- limited tool awareness
- explicit sources only

### PERMISSIVE

- broader governed context
- more explainability
- still no credentials or secrets

Rules:

- privacy level never bypasses governance
- privacy level does not grant execution authority
- privacy level does not override denied context references

## Lume privacy boundary

Lume may explain:

- current capability state
- missing requirements
- privacy settings
- consent status

Lume must not:

- infer consent
- store hidden data
- escalate permissions
- expose restricted context

## Relationship to future LLM interaction

Privacy and consent may influence:

- future context filtering
- desired interaction presentation
- whether LLM processing is allowed conceptually

Privacy and consent must not:

- trigger tool execution
- bypass ActionResolution
- grant access to restricted data
- expose credentials

## Invariants

- `INV-PRIV-001`: consent must be explicit.
- `INV-PRIV-002`: consent is separate from execution permission.
- `INV-PRIV-003`: hidden context injection is forbidden.
- `INV-PRIV-004`: credentials must never enter LLM context.
- `INV-PRIV-005`: privacy levels affect exposure, not authority.
- `INV-PRIV-006`: denied context references must remain excluded.
- `INV-PRIV-007`: no runtime persistence is defined in F9/F10_PREP.
- `INV-PRIV-008`: Lume must not manage or store private data.

## Non-goals

- no runtime storage implementation
- no consent database
- no authentication system
- no encryption-layer definition
- no cloud sync
- no telemetry
- no behavioral tracking
- no UI consent forms
- no real data handling

## Related specs

- `docs/specs/user_profile_policy.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/lume_interaction_model.md`
- `docs/specs/llm_core.md`
- `docs/specs/llm_tool_surface_resolution.md`
