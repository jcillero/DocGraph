# app_status_bar

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed bottom status bar for DocGraph.

## Core rule

The status bar displays prepared system state.

It does not compute authority, execute tools, call LLM, mutate files, or change configuration.

## Layout

The F9 status bar contains these blocks:

- `project`
- `project_profile`
- `llm`
- `tools`
- `sandbox`
- `mode`

Visual order:

`Project | Profile | LLM | Tools | Sandbox | Mode`

## StatusBarState conceptual contract

Conceptual JSON shape:

```json
{
  "project": {
    "loaded": false,
    "display_name": null
  },
  "project_profile": {
    "profile_id": null,
    "display_i18n": null
  },
  "llm": {
    "desired_llm_mode": "OFF",
    "effective_llm_mode": "OFF",
    "interaction_mode": "GUIDED",
    "llm_available": false,
    "provider_configured": false,
    "credential_ref_present": false,
    "local_engine_available": false,
    "local_model_available": false,
    "policy_allows_local": false,
    "policy_allows_cloud": false,
    "tool_surface_available": false,
    "execution_enabled": false,
    "reason_codes": [
      "llm_off_by_preference",
      "guided_fallback_active"
    ],
    "display_i18n": "status.llm.off"
  },
  "tools": {
    "declared_count": 0,
    "executable_count": 0,
    "active_filter": "all",
    "display_i18n": "status.tools.summary"
  },
  "sandbox": {
    "state": "off",
    "display_i18n": "status.sandbox.off"
  },
  "mode": {
    "lume_mode": "static",
    "display_i18n": "status.mode.static"
  }
}
```

## Allowed LLM states

The status bar consumes prepared `LLMCapabilityState`.

Allowed `desired_llm_mode` and `effective_llm_mode` values:

- `OFF`
- `LOCAL`
- `CLOUD`

Allowed `interaction_mode` values:

- `GUIDED`
- `ASSISTED`

The status bar may display reason codes such as:

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

Rules:

- status bar MUST NOT display credential values
- status bar MUST NOT display credential secrets
- missing credentials are status, not failure
- LLM absence does not block the app
- `desired_llm_mode` MUST remain distinguishable from `effective_llm_mode`
- `OFF` MUST be displayable as a valid guided mode
- status bar MUST display prepared reason codes rather than infer them

## Tools summary

The tools block shows:

- `executable_count`
- `declared_count`

Format:

`Tools: executable_count/declared_count`

Rules:

- declared vs executable must remain distinguishable
- clicking or selecting tools summary may only navigate to `Tools Panel` in future
- status bar must not execute tools

## Sandbox states

Allowed:

- `off`
- `readonly`
- `working_copy_future`
- `configured_future`

Rules:

- sandbox status is informational
- original external folders are never mutated from the status bar
- write-back is never triggered from the status bar

## Mode states

Allowed:

- `GUIDED`
- `ASSISTED`

Rules:

- `GUIDED` means Lume local or declarative help
- `ASSISTED` means LLM-assisted mode is conceptual or future unless runtime later opens it and capability state is available
- status bar must not activate Assisted mode

## Optional lightweight navigation

Future allowed navigation only:

- LLM block may open Lume Help explanation
- Tools block may open Tools Panel
- Sandbox block may open Lume help topic
- Mode block may open onboarding or help

These are navigation intents only.

They must not execute actions.

## Relationship to Lume

- Lume may explain each status block.
- Status bar may link to Lume Help.
- Lume does not compute status bar state.
- Lume may explain `reason_codes`.
- Status bar and Lume consume the same prepared capability state.

## Relationship to Tool Surface Resolver

- `tools.declared_count` and `tools.executable_count` may come from future effective tool surface summary.
- status bar does not read raw catalogs directly.
- status bar does not infer tool availability.

## Relationship to credentials

- credentials are never shown.
- `credential_ref` is not shown unless future policy explicitly allows non-secret references.
- `credential_ref_present` may influence status text only.

## Invariants

- `INV-STATUSBAR-001`: the status bar MUST display prepared system state only.
- `INV-STATUSBAR-002`: the status bar MUST NOT execute tools, LLM calls, filesystem mutations, or project actions.
- `INV-STATUSBAR-003`: the status bar MUST NOT expose credentials or sensitive data.
- `INV-STATUSBAR-004`: the status bar MUST reflect capability state, not infer or fabricate it.
- `INV-STATUSBAR-005`: the status bar MUST remain usable without LLM.
- `INV-STATUSBAR-006`: the status bar MAY provide navigation intents, but MUST NOT become a control surface.
- `INV-STATUSBAR-007`: displayed text MUST come from i18n resources.
- `INV-STATUSBAR-008`: declared vs executable capability MUST be distinguishable when relevant.
- `INV-STATUSBAR-009`: status bar state MUST be prepared outside UI rendering.
- `INV-STATUSBAR-010`: status bar must not read raw catalogs directly.
- `INV-STATUSBAR-011`: the status bar MUST distinguish desired LLM mode, effective LLM mode, and interaction mode.
- `INV-STATUSBAR-012`: the status bar MUST present `OFF` as a valid guided mode, not an error.
- `INV-STATUSBAR-013`: the status bar MUST NOT decide policy or resolve capability state from UI state.
- `INV-STATUSBAR-014`: status bar navigation intents MUST NOT execute tools, call providers, validate credentials, or approve actions.

## Forbidden responsibilities

The status bar must not:

- execute tools
- activate LLM
- validate credentials
- resolve credentials
- show secrets
- mutate project files
- change preferences
- change project profile
- create sandbox
- trigger write-back
- infer policy from UI state
- duplicate `Tools Panel`
- duplicate `Lume Help`
- open F10

## Related specs

- `docs/specs/lume_onboarding_model.md`
- `docs/specs/llm_tool_surface_resolution.md`
- `docs/specs/credentials_policy.md`
- `docs/specs/tools_panel.md`
- `docs/specs/project_profiles.md`
- `docs/specs/local_sandbox_context.md`
