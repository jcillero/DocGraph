# project_setup_popup

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the initial project setup popup and the later project settings popup for DocGraph.

The same governed popup may eventually operate in two modes:

- `mode = create`
- `mode = edit`

`Project Setup Popup` is used when creating a new project.  
`Project Settings Popup` is used when editing an already opened project.

## Core rule

The popup captures intent and configuration.

The popup does not validate as authority.
The popup does not write directly.
The popup does not execute tools.
The popup does not enable LLM.
The popup does not activate a real sandbox.

Project Setup may be guided by Static Mode Lume.

No LLM is required to create or configure a project.

It emits either a conceptual `ProjectSetupDraft` or a conceptual `ProjectSettingsDraft`.

## ProjectSetupDraft conceptual fields

- `project_name`
- `project_profile`
- `default_agent_profile`
- `allowed_operational_tools`
- `allowed_llm_tools_policy`
- `sandbox_policy`
- `llm_policy`
- `preferences`
- `credential_ref`
- `language`
- `help_level`
- `trace_metadata`

## ProjectSettingsDraft conceptual fields

The `ProjectSettingsDraft` includes the same conceptual fields, but targets an already opened project and future governed update flow.

`preferences` may be included as non-secret configuration.

`credential_ref` may be included only as a conceptual identifier. Secret values are not included.

`llm_policy` remains optional and future-facing.

## Editable fields

Low risk:

- visible project name
- language
- theme
- Lume help level
- declarative default agent profile

Medium risk:

- allowed tools
- declarative LLM policy
- sandbox readonly / dry-run policy
- project preferences

High risk / future:

- credentials
- cloud LLM
- mutable sandbox
- write-back to external folder
- tool execution

## Confirmation rules

- Low-risk changes may be saved through a future governed flow.
- Medium-risk changes require confirmation.
- High-risk changes are future proposal or `BLOCKED` in F9.
- Credentials must not be stored in `project_config.json`.
- No credential validation is performed in F9.
- Changing configuration does not enable real execution by itself.

## Forbidden responsibilities

The popup must not:

- write files directly
- mutate `project_manifest.json` directly
- execute tools
- execute LLM
- decide permissions
- validate as authority
- create a real sandbox
- mutate user folders
- reinterpret the manifest
- duplicate `project_runtime`
- store secrets in plain text
- use absolute paths as portable truth

## Related specs

- `docs/specs/credentials_policy.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/project_profiles.md`
- `docs/specs/project_folder_layout.md`
- `docs/specs/local_sandbox_context.md`
- `docs/specs/tools_panel.md`
- `docs/specs/tool_implementation_governance.md`
- `docs/specs/action_resolution.md`
- `docs/specs/lume_interaction_model.md`
