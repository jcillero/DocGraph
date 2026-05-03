# credentials_policy

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define how DocGraph must treat future credentials without exposing secrets or breaking portability.

This spec governs credential handling boundaries.
Menu and popup surfaces are governed separately.

## Core rule

Credentials are secrets.
Credentials are not preferences.
Credentials are not project configuration.
Credentials MUST NOT be stored inside project files.

## Credential references

Project configuration may store only references, never secret values.

Allowed example:

```json
{
  "credential_ref": "cred_openai_default"
}
```

Forbidden example:

```json
{
  "api_key": "secret_value"
}
```

## Credential storage model

F9 declares only the conceptual model:

- credentials live outside project files
- credentials may be resolved by a future secure local store
- credentials may be resolved by an OS-level secret manager
- project files contain only `credential_ref`
- `credential_ref` is not a secret by itself

## LLM boundary

- LLM context MUST NOT include credentials.
- LLM context MUST NOT include raw secret values.
- LLM may receive capability state, never secret material.
- Tool calls must receive credentials only through future authorized runtime resolution.

## Logging boundary

- credentials MUST NOT be written to logs
- credentials MUST NOT appear in `tool_run_manifest.json`
- credentials MUST NOT appear in `graph/`
- credentials MUST NOT appear in chat history
- credentials MUST NOT appear in exported documents

## Project Setup / Settings relationship

Project Setup / Settings may capture intent to configure credentials.

In F9:

- no credential is stored
- no credential is validated
- no credential is resolved
- only future conceptual `credential_ref` may be recorded

In the governed Preferences menu model:

- `Credentials management` is a dedicated popup surface
- the popup manages references and status only
- the popup MUST NOT display raw secret values
- the popup emits `ConfigurationIntent`, not `ExecutionIntent`
- changing credentials MUST NOT enable execution by itself

## Invariants

- `INV-CRED-001`: credentials MUST NOT be stored in project files.
- `INV-CRED-002`: project files MAY store credential references, not secret values.
- `INV-CRED-003`: credentials MUST NOT be exposed to LLM context.
- `INV-CRED-004`: credentials MUST NOT be written to logs, chats, manifests, graph, `tool_run_manifest`, or exports.
- `INV-CRED-005`: credential resolution is future runtime behavior and is not implemented in F9.
- `INV-CRED-006`: UI may capture credential intent but must not become credential authority.
- `INV-CRED-007`: `credential_ref` is an identifier, not execution permission.
- `INV-CRED-008`: changing `credential_ref` MUST NOT enable execution by itself.
- `INV-CRED-009`: `Credentials management` MUST remain separate from `Preferences` content and `Tools` capability flow even when routed from the main menu.
- `INV-CRED-010`: UI MUST NOT display raw secrets in credentials-management surfaces.
- `INV-CRED-011`: status such as `configured` or `missing` MUST NOT imply execution authority.

## Forbidden responsibilities

This spec must not:

- implement secret storage
- select OS credential manager
- validate real API keys
- call providers
- expose credentials to LLM
- write secrets to project files
- create cloud execution
- open F10

## Related specs

- `docs/specs/ui_preferences_popups.md`
- `docs/specs/project_settings_popup.md`
- `docs/specs/preferences_policy.md`
