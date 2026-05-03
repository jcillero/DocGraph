# llm_core

## Purpose

Define the inherited LLM contract and policy layer for the Rust successor.

## Responsibilities

- define governed LLM modes: `OFF`, `LOCAL`, `CLOUD`
- define conceptual capability state and effective mode resolution
- define guided fallback behavior when effective LLM mode is `OFF`
- resolve deterministic governed plans
- apply configuration precedence and policy
- govern model catalog validation
- define tool request policy
- define required observability payloads

## Inputs

- explicit runtime arguments
- user preferences when available
- conceptual user profile and consent references, when available
- system LLM configuration
- model catalog
- available tool specifications
- governed user input
- governed context references

## Outputs

- deterministic governed plans
- validated provider and model selections
- effective tool policy
- structured observability metadata requirements
- conceptual request-analysis artifacts
- conceptual response-synthesis artifacts

## Allowed Dependencies

- standard library
- `core_domain`
- `spec_runtime`

## Forbidden Responsibilities

- no direct provider SDK integration
- no direct tool execution
- no UI logic
- no Slint dependency
- no filesystem ownership outside governed config loading

## Initial Phase Scope

- policy and contract definitions
- LLM mode and model catalog rules
- capability-state and guided fallback contracts
- agent role and prompt resource governance
- tool request governance
- observability requirements
- conceptual input-analysis and response-synthesis pipeline
- no real provider invocation yet

## F10 step-1 opening objective

The first explicit F10 opening gate is to allow a minimum governed LLM-assisted runtime slice without opening tool execution or `ActionResolution` execution.

This step may open runtime concern for:

- capability-state resolution
- effective-mode resolution
- governed context filtering
- declarative prompt loading
- assisted-request preparation
- guided fallback behavior

This step does not require provider invocation.

Provider binding remains optional and phase-gated.

## Governed LLM modes

The only governed LLM modes are:

- `OFF`
- `LOCAL`
- `CLOUD`

No `auto` mode is authorized by this spec.

### OFF

`OFF` means:

- no LLM generation
- no provider calls
- no local model usage
- deterministic guided menus may be used
- DocGraph remains usable

`OFF` is not an error state.

### LOCAL

`LOCAL` means local backend intent only.

It requires:

- local engine capability
- local model capability
- policy allowing local LLM use

It does not require cloud credentials.

Availability is derived from `LLMCapabilityState`, not from UI state.

### CLOUD

`CLOUD` means cloud backend intent only.

It requires:

- provider configuration
- non-secret `credential_ref`
- policy allowing cloud LLM use

Credential values must never be exposed to LLM context, logs, status surfaces, prompts, tool manifests, or UI state.

Availability is derived from `LLMCapabilityState`, not from UI state.

## Interaction modes

The governed interaction modes are:

- `GUIDED`
- `ASSISTED`

`GUIDED` works without LLM capability.

`ASSISTED` requires an available effective LLM capability.

Interaction mode does not imply tool execution, file mutation, provider calls, or action authority.

## LLMCapabilityState

Before any future Lume interaction, the system must conceptually resolve:

`desired_llm_mode -> LLMCapabilityState -> effective_llm_mode -> interaction_mode`

Conceptual fields:

- `desired_llm_mode`: `OFF | LOCAL | CLOUD`
- `effective_llm_mode`: `OFF | LOCAL | CLOUD`
- `interaction_mode`: `GUIDED | ASSISTED`
- `llm_available`: bool
- `provider_configured`: bool
- `credential_ref_present`: bool
- `local_engine_available`: bool
- `local_model_available`: bool
- `policy_allows_local`: bool
- `policy_allows_cloud`: bool
- `tool_surface_available`: bool
- `execution_enabled`: bool
- `reason_codes`: string[]

Rules:

- all capability states must be explicit
- missing capability must not fail silently
- credentials and secret values must never be exposed
- `desired_llm_mode` is not the same as `effective_llm_mode`
- `credential_ref` is an identifier, not execution permission
- `execution_enabled=false` remains the current governed default

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

## Effective mode resolution

Resolution is conceptual only. It does not implement runtime logic.

Rules:

- if `desired_llm_mode=OFF`, then `effective_llm_mode=OFF`, `interaction_mode=GUIDED`, and `reason_codes` includes `llm_off_by_preference`
- if `LOCAL` is requested but local engine is missing, local model is missing, or policy blocks local mode, then `effective_llm_mode=OFF`, `interaction_mode=GUIDED`, and `reason_codes` must explain the missing or blocked capability
- if `CLOUD` is requested but provider configuration is missing, `credential_ref` is missing, or policy blocks cloud mode, then `effective_llm_mode=OFF`, `interaction_mode=GUIDED`, and `reason_codes` must explain the missing or blocked capability
- if local engine exists, local model exists, and policy allows local mode, then `effective_llm_mode=LOCAL`, `interaction_mode=ASSISTED`, and `llm_available=true`
- if provider configuration exists, `credential_ref` is present, and policy allows cloud mode, then `effective_llm_mode=CLOUD`, `interaction_mode=ASSISTED`, and `llm_available=true`
- no mode grants tool execution authority
- no mode bypasses `ActionResolution`

When future execution remains closed, `execution_enabled=false` and `reason_codes` must include `execution_not_open` when relevant.

## GuidedMenuEngine contract

`GuidedMenuEngine` is a conceptual deterministic menu contract for `GUIDED` interaction.

It is not an LLM, runtime, executor, provider adapter, or UI authority.

Conceptual structures:

- `GuidedMenu`
- `GuidedMenuOption`
- `GuidedMenuSelection`
- `GuidedMenuResult`

Allowed menu intents:

- `OpenProjectIntent`
- `OpenPreferencesIntent`
- `OpenProjectSettingsIntent`
- `OpenToolsPanelIntent`
- `OpenLumeHelpIntent`
- `ShowCapabilityStateIntent`
- `ExplainMissingCapabilityIntent`
- `ConfigureLocalLlmIntent`
- `ConfigureCloudLlmIntent`

Forbidden menu intents:

- `ExecuteToolIntent`
- `MutateFileIntent`
- `MutateDocumentIntent`
- `CallLlmProviderIntent`
- `ValidateCredentialSecretIntent`
- `RunExternalBinaryIntent`
- `ApproveActionIntent`
- `ExecuteActionResolutionIntent`

OFF-mode menu example:

1. Open project
2. Review documents
3. Open Tools Panel
4. Configure local LLM
5. Configure cloud LLM
6. Show capability state
7. Open Lume Help

The example is informational only. It does not define UI implementation or runtime behavior.

## Agent role model

Lume is the single governed front-facing assistant identity.

Future assisted reasoning may use internal conceptual roles:

- Planner
- Specialists
- Synthesizer

Canonical flow:

```text
User
<-> Lume
-> Planner
-> Specialists
-> Synthesizer
-> Lume
-> User
```

Rules:

- all user interactions enter and exit through Lume
- internal roles do not directly interact with the user
- internal roles are roles, not authorities
- internal roles may propose tool or action candidates only as proposals
- internal roles must not execute tools, mutate files, approve actions, expose credentials, call providers directly, or bypass ActionResolution

Prompt storage, prompt format, versioning, and privacy constraints are governed by `docs/specs/llm_agent_prompts.md`.

## Agent prompt resource contract

All future agent prompts must live under:

`resources/llm/agents/`

Prompt resources are declarative, versioned, and non-executable.

Runtime must not embed prompt text in code.

UI must not define or modify prompts.

Prompt loading in a future phase does not imply provider invocation, agent execution, tool execution, credential resolution, or ActionResolution execution.

Future traces should record prompt version and fingerprint when prompts are used.

## Governed LLM analysis pipeline

Future governed LLM interaction follows this conceptual sequence:

`UserInput`
-> `IntentAnalysis`
-> `RequestDecomposition`
-> `SubRequestPlan[]`
-> `IndependentSubRequestResolution[]`
-> `ResponseSynthesis`
-> `FinalAnswer`

This sequence is declarative only.

It does not implement runtime execution, provider calls, tool execution, or filesystem mutation.

### IntentAnalysis

`IntentAnalysis` must produce:

- `primary_intent`
- `secondary_intents`
- `user_goal`
- `required_context`
- `ambiguity_level`
- `risk_level`
- `needs_clarification`
- `may_require_tools`
- `may_require_action_resolution`

Purpose:

- identify the dominant user objective
- identify secondary asks that must not be lost
- detect missing context
- detect whether any later governed tool or action path may be relevant

### RequestDecomposition

`RequestDecomposition` must split governed user input into atomic subrequests.

Rules:

- preserve original wording references
- preserve ordering when order matters
- do not invent extra user requests
- do not silently discard lower-priority but still relevant asks

Allowed subrequest classes:

- `informational`
- `planning`
- `code_change`
- `document_change`
- `tool_candidate`
- `clarification_needed`
- `blocked`

### SubRequestPlan

Each subrequest must include:

- `subrequest_id`
- `source_span` or `source_summary`
- `intent`
- `required_context`
- `dependencies`
- `can_resolve_independently`
- `requires_tool_surface`
- `requires_action_resolution`
- `risk_level`
- `expected_output_type`

`SubRequestPlan` is planning metadata only.

It does not authorize execution.

### IndependentSubRequestResolution

Independent resolution is logical independence only.

It means a subrequest can be analyzed or answered without waiting for unrelated sibling subrequests.

This spec does not require physical concurrency.

Parallel execution is a future optimization only.

Each partial resolution may therefore produce:

- `resolved`
- `uncertain`
- `blocked`
- `clarification_required`

### ResponseSynthesis

`ResponseSynthesis` must:

- preserve dependency order
- expose blocked or uncertain subresults
- avoid hiding failed subrequests
- avoid contradiction between partial answers
- produce one coherent final response

`FinalAnswer` is the single synthesized answer returned to the user after governed synthesis.

## Governed user profile, privacy, and consent contract

Future governed LLM interaction may conceptually consume:

- optional `UserProfile`
- optional `UserPreferences`
- optional `ConsentState`

These structures influence:

- `desired_llm_mode`
- `interaction_mode`
- context filtering

They do not:

- trigger tool execution
- bypass `ActionResolution`
- grant access to restricted data
- expose credentials

`UserProfile` remains optional and non-blocking.

Consent must be explicit before any future LLM processing.

Preferences remain advisory only.

Privacy level affects exposure filtering only.

It does not grant execution authority.

Future allowed context may include only:

- explicit user input
- explicitly allowed knowledge sources
- `EffectiveToolSurfaceSummary`, when allowed
- non-sensitive system-state summaries

Future forbidden context includes:

- credentials
- secret values
- denied context references
- hidden system prompts
- raw full tool catalogs by default
- raw host-specific filesystem paths

## F10 step-1 minimum runtime slice

If F10 opens its first minimum governed slice, `llm_core` runtime responsibility may be limited to:

- resolve `LLMCapabilityState`
- resolve `effective_llm_mode`
- enforce consent-aware and privacy-bounded context filtering
- load declarative prompt resources and prompt metadata
- prepare a bounded LLM request envelope
- produce deterministic guided fallback when assistance is unavailable

The first F10 slice must not open:

- broad tool execution
- `ActionResolution` runner
- filesystem mutation
- hidden memory
- raw full tool-catalog injection
- UI authority

## Tool surface note

Future LLM context must receive an effective tool surface, not the raw full catalog by default.

A future tool-surface context provider is policy-oriented and read-only. It does not execute tools or filesystem actions.

Tool visibility does not imply execution permission.

Future LLM tool intent becomes proposal only and must be governed before any execution.

## Credential boundary note

Future LLM execution may require `credential_ref` conceptually.

`llm_core` policy may reference credentials conceptually, but LLM context must never receive credential material.

Credential resolution is a future runtime concern.

LLM availability is optional.

No LLM available must resolve to a non-blocking capability state.

LLM absence must not block core DocGraph usage.

## Invariants

- LLM decomposition does not grant execution authority.
- subrequests do not execute tools directly.
- any tool or action candidate becomes a proposal only.
- `ActionResolution` governs future executable actions.
- credentials are never exposed to LLM context.
- raw full tool catalog is not injected by default.
- Lume is the single governed front-facing agent.
- internal agents do not directly interact with the user.
- internal agents are roles, not authorities.
- all agent prompts live under `resources/llm/agents/`.
- prompts are declarative, versioned, and non-executable.
- prompts contain no credentials or secrets.
- prompts do not hardcode tool availability.
- runtime must not embed prompt text in code.
- UI must not define or modify prompts.
- governed LLM modes are limited to `OFF`, `LOCAL`, and `CLOUD`.
- `OFF` is not an error and must not call LLM providers, local models, tools, or runtime actions.
- `GUIDED` works without LLM capability.
- `ASSISTED` requires available LLM capability.
- `desired_llm_mode` does not equal `effective_llm_mode` by default.
- capability state must expose missing, blocked, unavailable, and fallback states explicitly.
- `credential_ref` grants no execution authority.
- changing preferences must not trigger execution.
- UI, status bar, and tools panel display prepared capability state only.
- tool visibility does not imply execution permission.
- user profile is optional and non-blocking.
- consent must be explicit and separate from execution permission.
- preferences are advisory only.
- no hidden context injection is allowed.
- privacy levels affect exposure, not authority.
- future implementation must be deterministic, traceable, and testable without provider calls.

## Non-goals

- no runtime implementation
- no physical parallelism
- no tool execution
- no document mutation
- no provider calls
- no credential resolution
- no ActionResolution runner
- no filesystem mutation
- no external binary validation
- no UI authority
- no prompt editing UI
- no prompt embedding in code
- no hidden memory
- no user database
- no telemetry
- no behavioral tracking
- no project_runtime changes
- no raw tool catalog injection
- no new pipeline outside `llm_core`
- no UI orchestration

For F10 step 1 specifically:

- no provider binding is implied
- no provider invocation is required
- no tool execution is authorized
