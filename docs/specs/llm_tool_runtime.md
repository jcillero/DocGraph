# llm_tool_runtime

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the initial governed specification for `LLM Tools` in DocGraph.

This spec prepares LLM tools as declarative capability requests without opening F10, executing tools, invoking providers, or duplicating existing runtime authority.

## Operational Tools vs LLM Tools

`Operational Tools`:

- are controlled technical actions
- may be manually launched in a future permitted phase
- remain explicitly user-driven when execution is later opened

`LLM Tools`:

- are model-requested capability proposals
- are not directly executed by the model
- require governed system resolution before any future execution phase

## Core rule

An `LLM Tool` is not a function executed by the model.

It is a declared capability that the model may request and that the system must later resolve through governed policy.

In F9:

- no real LLM tool execution exists
- no provider call is created by this spec
- no tool is executed by the model

## Relationship to Tool Surface Resolver

`LLM Tools` participate in `EffectiveToolSurface`.

The effective LLM-facing surface is determined by:

- global tool catalog declarations
- project policy
- `llm_tool_policy`
- governed tool-surface resolution

The raw full catalog must not be injected into LLM context by default.

## Minimum states

Prepared LLM tool state may include:

- `declared`
- `visible`
- `allowed_in_project`
- `execution_enabled`
- `execution_implemented`
- `blocked`

Interpretation:

- `declared`: tool exists in declarative catalog only
- `visible`: tool may appear in governed surfaces
- `allowed_in_project`: project policy permits it conceptually
- `execution_enabled`: execution is allowed by declaration if a future phase opens it
- `execution_implemented`: actual runtime execution exists
- `blocked`: current phase, policy, or context prevents execution

## Rules

- `ActionResolution` is mandatory before any future execution
- human confirmation is required when applicable by policy or risk
- `owner_ref` is mandatory for future persisted outputs
- `tool_run_manifest.json` is mandatory for future persisted executions
- raw full catalog must not be injected by default into LLM context
- secrets must never enter LLM context

Normative form:

- `INV-LLMTOOL-001`: `LLM Tools` MUST remain requestable capability declarations in F9
- `INV-LLMTOOL-002`: any future LLM tool execution MUST pass through `ActionResolution`
- `INV-LLMTOOL-003`: persisted future outputs MUST require `owner_ref`
- `INV-LLMTOOL-004`: persisted future executions MUST require `tool_run_manifest.json`
- `INV-LLMTOOL-005`: raw full tool catalog MUST NOT be injected into LLM context by default
- `INV-LLMTOOL-006`: secrets MUST NOT appear in LLM tool context

## Relationship to surfaces

`Chat Panel`:

- captures intention
- may carry LLM tool request context
- does not execute tools

`DocumentHolder`:

- may be future `owner_ref` for persisted outputs

`Knowledge`:

- may be future `owner_ref` for persisted outputs

`ToolRuntime`:

- is the future execution boundary
- is not opened by this spec in F9

## Relationship to tool_runtime resources

Current operative runtime declarations still live under:

- `resources/tool_runtime/meta_catalog.json`
- `resources/tool_runtime/llm_tool_policy.json`

This spec must not duplicate `tool_runtime` or `project_runtime`.

`resources/tools/internal/llm/*` remains declarative F9-ready layout unless a future controlled transition explicitly opens it.

## Failure modes

- `tool_declared_but_not_executable`
- `tool_not_allowed_in_project`
- `missing_owner_ref`
- `action_resolution_required`
- `llm_context_surface_too_broad`
- `secret_exposure_blocked`

Interpretation:

- `tool_declared_but_not_executable`: tool exists in declaration but lacks execution authority or implementation
- `tool_not_allowed_in_project`: project policy blocks the tool
- `missing_owner_ref`: future persisted output target is missing governed ownership
- `action_resolution_required`: execution was attempted without governed resolution
- `llm_context_surface_too_broad`: LLM context received more tool-surface detail than allowed
- `secret_exposure_blocked`: request or context path attempted to expose secret material

## F9 / F9.5 audits

Future or existing declarative audit expectations include:

- raw catalog not injected
- disabled tools not executable
- no secret fields in context
- every LLM tool declares status and limits

These audits remain governance checks, not runtime execution.

## Catalog note

`resources/tools/internal/llm/tools_llm_document_catalog.json` may remain empty in F9.

An empty catalog is valid when no LLM tool entries are yet declared.

## Forbidden responsibilities

This spec must not:

- call providers
- execute tools
- widen `project_runtime`
- duplicate `tool_runtime`
- move policy into UI
- open F10

## Related specs

- `docs/specs/llm_tool_surface_resolution.md`
- `docs/specs/action_resolution.md`
- `docs/specs/tools_catalogs.md`
- `docs/specs/tool_implementation_governance.md`
