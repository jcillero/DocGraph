# llm_agent_prompts

## Status

PROPOSAL / F10_PREP declarative governance.

No runtime implementation.

No prompt files are created by this spec.

## Purpose

Define Lume as the single governed front-facing agent and define declarative storage, format, versioning, and governance rules for all future LLM agent prompts.

This spec is storage and contract only.

It does not implement prompt loading, provider calls, tool execution, prompt editing UI, hidden memory, or ActionResolution execution.

## Canonical agent flow

All future user-facing LLM interaction follows this conceptual flow:

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

- all user interaction enters through Lume
- all user-facing output exits through Lume
- internal agents are not user-facing
- internal agents are roles, not execution authorities
- internal roles may prepare reasoning artifacts only in a future governed runtime
- no role may execute tools, mutate files, call providers directly, approve actions, expose credentials, or bypass ActionResolution

## Lume front-facing role

Lume is the only governed front-facing assistant identity.

Lume acts as:

- intent detection agent
- guidance/help agent
- orchestration entry and exit point
- capability explainer
- tool-aware but non-executing interface

Lume must not:

- execute tools
- mutate files
- approve actions
- expose credentials
- bypass ActionResolution
- call providers directly
- create hidden persistence
- create or delegate to another front-facing assistant identity

No alternative front-facing assistant may be introduced without a governed proposal, phase decision, i18n/resource review, and explicit update to Lume identity governance.

## Internal agent roles

Internal agents are conceptual roles only.

They are not UI identities, user-facing assistants, runtime authorities, or autonomous workers.

### Planner

Planner performs future conceptual decomposition and routing.

Responsibilities:

- analyze user intent received through Lume
- decompose requests into subrequests
- identify needed context
- route subrequests to scoped specialist roles
- mark tool or action candidates as proposals only

Planner must not execute tools, call providers directly, mutate files, approve actions, or communicate directly with the user.

### Specialists

Specialists perform scoped domain reasoning.

Initial conceptual specialist roles:

- `engineering`
- `mathematics`
- `software`
- `legal`
- `medical`
- `financial`
- `documentation`
- `research`
- `general_reasoning`

Specialists may produce scoped partial reasoning artifacts in a future governed runtime.

Specialist output is not user-facing until integrated by Synthesizer and presented by Lume.

Specialists must not execute tools, call providers directly, mutate files, approve actions, or communicate directly with the user.

Domain-sensitive specialists such as legal, medical, and financial must remain informational and must not present professional advice as authority.

### Synthesizer

Synthesizer integrates partial results.

Responsibilities:

- preserve dependency order
- expose blocked, uncertain, or failed subresults
- avoid contradictions between partial results
- produce one coherent `FinalAnswer` candidate for Lume

Synthesizer must not execute tools, call providers directly, mutate files, approve actions, or communicate directly with the user.

## Declarative prompt storage model

All future agent prompts live under:

```text
resources/llm/agents/
  lume/
    lume_agent.prompt.json
  planner/
    planner_agent.prompt.json
  synthesizer/
    synthesizer_agent.prompt.json
  specialists/
    engineering.prompt.json
    mathematics.prompt.json
    software.prompt.json
    legal.prompt.json
    medical.prompt.json
    financial.prompt.json
    documentation.prompt.json
    research.prompt.json
    general_reasoning.prompt.json
```

This is a declarative storage contract only.

The presence of a prompt file must not imply provider invocation, agent execution, tool execution, prompt activation, UI availability, or F10 opening.

Prompt files must be environment-independent and portable.

Runtime code must not embed prompt text.

UI must not define, edit, or mutate prompt files.

## Prompt format

Each prompt resource must declare:

- `agent_id`
- `version`
- `role`
- `system_prompt`
- `constraints`
- `input_contract`
- `output_contract`
- `allowed_context_refs`
- `forbidden_context`
- `capabilities`
- `tool_surface_policy`
- `privacy_constraints`

Rules:

- prompts are declarative, versioned, and non-executable
- prompts contain no credentials, secrets, tokens, passwords, private keys, or provider material
- prompts do not hardcode tool availability
- prompts reference tools only through `EffectiveToolSurfaceSummary`
- prompts must not include raw full tool catalogs by default
- prompts are environment-independent
- prompts must not contain host-specific absolute paths
- prompts must not grant execution authority
- prompts must not override governance, policy, ActionResolution, credentials policy, tool policy, or project_runtime boundaries

## Prompt versioning and traceability

Prompt resources must be versioned.

Future traces should record:

- `agent_id`
- prompt `version`
- prompt fingerprint or checksum
- prompt resource path
- effective LLM mode
- capability state reference
- tool surface summary reference, when used

Traceability does not imply provider invocation or execution authority.

## Conceptual prompt loading

In a future governed phase, `llm_core` may load prompt resources from `resources/llm/agents/`.

Loading prompts:

- does not imply provider invocation
- does not imply agent execution
- does not imply tool execution
- does not imply ActionResolution execution
- does not expose credentials
- does not mutate catalogs
- does not create hidden memory

Credentials must never enter prompts.

## Tool awareness

Lume and internal roles may receive `EffectiveToolSurfaceSummary`.

Rules:

- no agent receives raw full catalogs by default
- agents may propose tool usage
- agents must not execute tools
- tool intent becomes proposal only
- future execution requires governed ActionResolution before any execution path

## OFF / LOCAL / CLOUD behavior

### OFF

When effective LLM mode is `OFF`:

- Lume uses guided menus and static help
- no LLM calls occur
- internal agent roles are inactive or explanatory only
- prompt loading is not required
- no provider, local model, tool, or runtime action is invoked

### LOCAL

When effective LLM mode is `LOCAL`:

- assisted reasoning may be available only if capability state allows
- internal roles may be used only in a future governed runtime
- local model use remains non-executing with respect to tools/actions
- credentials are not required for local mode

### CLOUD

When effective LLM mode is `CLOUD`:

- assisted reasoning may be available only if capability state allows
- internal roles may be used only in a future governed runtime
- provider configuration and non-secret `credential_ref` are required
- credential values must never enter prompts or LLM context

## Invariants

- `INV-AGENT-001`: Lume is the single governed front-facing agent.
- `INV-AGENT-002`: no alternative front-facing assistant may be introduced without governed proposal and phase decision.
- `INV-AGENT-003`: all user interactions enter and exit through Lume.
- `INV-AGENT-004`: internal agents do not directly interact with the user.
- `INV-AGENT-005`: internal agents are roles, not authorities.
- `INV-AGENT-006`: all agent prompts live under `resources/llm/agents/`.
- `INV-AGENT-007`: prompts are declarative, versioned, and non-executable.
- `INV-AGENT-008`: prompts contain no credentials or secrets.
- `INV-AGENT-009`: prompts do not hardcode tool availability.
- `INV-AGENT-010`: runtime must not embed prompt text in code.
- `INV-AGENT-011`: UI must not define or modify prompts.
- `INV-AGENT-012`: prompts are loadable independently of runtime execution.
- `INV-AGENT-013`: `EffectiveToolSurface` governs tool awareness.
- `INV-AGENT-014`: agents may propose but not execute tools or actions.
- `INV-AGENT-015`: prompt loading must be deterministic, traceable, and testable without provider calls.

## Non-goals

- no runtime implementation
- no provider calls
- no tool execution
- no ActionResolution runner
- no UI authority
- no prompt editing UI
- no catalog mutation
- no credential handling
- no hidden memory
- no prompt resource creation in this phase
- no new front-facing agent

## Related specs

- `docs/specs/lume_interaction_model.md`
- `docs/specs/lume_help_popup.md`
- `docs/specs/lume_help_tree.md`
- `docs/specs/chat_document_flows.md`
- `docs/specs/llm_core.md`
- `docs/specs/llm_tool_surface_resolution.md`
