# llm_tool_surface_resolution

## Status

PROPOSAL / F10_PREP declarative governance.

No runtime implementation.

## Purpose

Define the governed `EffectiveToolSurface` contract for future LLM-facing tool context without injecting the full raw tool catalog by default.

This spec is preparation only.

It does not implement a resolver, bind runtime, modify `tool_runtime`, or open execution.

## Core rule

The LLM must not receive the raw full tool catalog by default.

`EffectiveToolSurface` is not the raw master catalog.

It is a governed, filtered, stateful surface derived from declarations plus policy and capability constraints.

Instead, a future governed resolver provides:

1. minimal tool surface summary
2. relevant effective tools when intent requires it
3. optional expanded surface through explicit context request

## Conceptual pipeline

`user intent`
-> `intent/context classification`
-> `EffectiveToolSurfaceResolver`
-> `EffectiveToolSurface`
-> `LLM context`
-> `optional expansion request`

## Resolution inputs

Future resolution is declarative only in this phase.

The future resolver will consume:

- `resources/tools/tools_master_catalog.json` as declarative master capability source
- `resources/tool_runtime/meta_catalog.json` as current operative runtime declaration source
- `resources/tool_runtime/llm_tool_policy.json` as current operative global LLM/tool policy source
- project-level policy or override declarations, when present
- execution capability flags derived from declared implementation and availability state

These inputs are contracts only.

They do not create runtime coupling in F9/F10_PREP.

`project_runtime` is not a resolution input.

## F10 step-1 boundary

The first explicit F10 opening gate may allow runtime production of a bounded `EffectiveToolSurfaceSummary`.

That minimum slice is limited to:

- deriving governed summary data
- deriving bounded per-tool explanation state
- serving summary or `active_only` context to assisted LLM flows

That minimum slice does not open:

- tool execution
- raw full catalog injection
- UI authority
- `ActionResolution` runner
- provider authority

## EffectiveToolSurface model

### Summary object

Minimum conceptual fields:

- `surface_kind`
- `phase`
- `mode`
- `project_profile`
- `generated_by`
- `active_tool_count`
- `declared_tool_count`
- `visible_tool_count`
- `enabled_tool_count`
- `implemented_tool_count`
- `executable_tool_count`
- `non_executable_relevant_count`
- `can_query_more`
- `default_scope`

### Entry object

Minimum conceptual fields:

- `capability_id`
- `tool_id`
- `tool_class`
- `category`
- `status`
- `execution_available`
- `visible`
- `allowed_in_project`
- `execution_enabled`
- `execution_implemented`
- `requires_confirmation`
- `requires_owner_ref`
- `policy_flags`
- `reason`
- `limitations`

### Required distinctions

`EffectiveToolSurface` must preserve these distinctions:

- declared: present in a governed declaration source
- visible: allowed to appear in a governed surface
- enabled: policy and capability flags allow the tool to be considered active
- implemented: the corresponding governed runtime boundary exists conceptually
- executable: future runtime may invoke it through governed execution path

These terms must not be collapsed into one field.

In `F9` and `F10_PREP`, declared tools remain non-executable unless a later phase explicitly opens execution.

`effective surface != raw catalog`

## Tool state model

Each `ToolSurfaceEntry` must expose:

- `capability_id`
- `tool_id`
- `tool_class`
- `status`
- `execution_available`
- `visibility`
- `policy_flags`

Allowed `tool_class` values:

- `operational`
- `llm`
- `external_dependency`
- `base_utility`, when hidden support is required by higher governance

Canonical meanings:

- `operational`: deterministic system action that may become executable only in a later governed phase
- `llm`: capability available to LLM reasoning surfaces; not manually executable in F9
- `external_dependency`: external binary or runtime dependency; not a tool by itself
- `base_utility`: internal support capability; not user-facing

`tool_class` defines classification only.

It does not define execution.

Tool visibility does not imply execution permission.

LLM tools are not agent roles.

Internal agent roles are not `tool_class` values.

Allowed `status` values:

- `declared_only`
- `not_implemented`
- `disabled`
- `enabled`

Interpretation:

- `declared_only`: governed declaration exists, but the tool is not active for execution
- `not_implemented`: semantic capability exists, but no governed implementation boundary is available
- `disabled`: implementation may exist conceptually, but policy or capability state blocks use
- `enabled`: the tool is active in the effective governed surface, but not necessarily executable in the current phase

`execution_available` is a boolean capability summary.

It must remain `false` whenever execution is not opened by phase or policy.

`visibility` is governed exposure only.

It is not permission and not execution authority.

`policy_flags` may include:

- `allowed_in_project`
- `allowed_by_global_policy`
- `allowed_by_phase`
- `requires_confirmation`
- `requires_owner_ref`
- `requires_dependency`
- `requires_credentials`
- `hidden_support`

## Resolution modes

Supported modes:

- `active_only`
- `all`
- `explain`

Definitions:

`active_only`:
returns only tools that are:

- `visible=true`
- `allowed_in_project=true`
- `execution_enabled=true`
- `execution_implemented=true`

and, in future phases, aligned with current capability flags

Invariants:

- `active_only` = visible + allowed + implemented + enabled
- `active_only` is not a raw dump

`all`:
returns the governed effective surface with state, flags, and limitations.

It must not return the raw master catalog or unfiltered runtime declarations.

`explain`:
returns the governed effective surface enriched for explanation.

It may include:

- visible but disabled tools
- declared but not implemented tools
- relevant hidden-support capabilities
- reasons and limitation records

`explain` exists to help Lume and future chat UX explain absence, disabled state, or blocked usability.

## LLM context contract

Default behavior:

- inject summary only by default
- inject `active_only` or `explain` entries only when tool/capability context requires it
- do not inject the full raw catalog by default

### Minimal default LLM payload

The minimum default representation may include:

- `surface_kind`
- `phase`
- `mode`
- `project_profile`
- `active_tool_count`
- `declared_tool_count`
- `non_executable_relevant_count`
- `can_query_more`
- small `tools` list with only the most relevant entries

Per-tool minimal fields for LLM context:

- `capability_id`
- `tool_id`
- `tool_class`
- `status`
- `execution_available`
- `reason`, when relevant
- `limitations`, when relevant

### Size discipline

The LLM-facing payload must remain bounded.

Rules:

- do not inject the full master catalog by default
- do not inject full schemas by default
- do not inject hidden support tools unless required for explanation
- prefer summary plus a small relevant subset over broad catalog dumps
- expanded context is future opt-in behavior only

## Agent prompt relationship

Future Lume and internal agent prompts may reference tool awareness only through `EffectiveToolSurfaceSummary`.

Rules:

- prompts must not embed raw tool catalogs
- prompts must not hardcode tool availability
- prompts must not declare execution permission
- prompt text must not override `EffectiveToolSurface`
- agent roles may propose tool usage only as proposal-level intent
- tool/action candidates still require future ActionResolution before execution

Prompt storage and versioning are governed by `docs/specs/llm_agent_prompts.md`.

## LLM context provider

Conceptual future context provider:

`read_effective_tool_surface`

Kind:

- `llm_context_provider`
- not operational tool
- not executable filesystem action

Arguments:

- `mode`: `enum(active_only, all, explain)`, default `active_only`
- `tool_class_filter`: `enum(operational, llm, external_dependency, base_utility, any)`, default `any`
- `category_filter`: optional text
- `project_profile`: optional text
- `include_reasons`: boolean, default `true`
- `include_schemas`: boolean, default `false`
- `max_results`: integer, default `20`

Output:

- `surface_kind`
- `phase`
- `project_profile`
- `mode`
- `generated_by`
- `tools`

No provider API is implemented in this phase.

This is a conceptual interface only.

If F10 step 1 opens, the future context provider remains read-only and non-executing.

## Explainability rules

`EffectiveToolSurface` may explain:

- why a capability is missing from `active_only`
- why a visible tool is disabled
- why a declared tool is not implemented
- why a tool is not usable in the current project or phase

Explanation records should prefer stable structured reasons over free-form prose.

Minimum explanation dimensions:

- `reason_code`
- `summary`
- `policy_source`, when applicable
- `capability_blocker`, when applicable

Recommended reason codes:

- `not_declared_for_surface`
- `disabled_by_global_policy`
- `disabled_by_project_policy`
- `not_implemented`
- `execution_not_open_in_phase`
- `missing_dependency`
- `missing_credentials`
- `hidden_support_only`

Explainability is presentation-safe context only.

It does not authorize execution, mutate policy, or change tool state.

## Invariants

- `INV-LLM-TOOL-SURFACE-001`: the system SHOULD inject a minimal effective tool surface summary into LLM context.
- `INV-LLM-TOOL-SURFACE-002`: the raw full tool catalog MUST NOT be injected into LLM context by default.
- `INV-LLM-TOOL-SURFACE-003`: the LLM MAY request expanded tool context through a governed context provider.
- `INV-LLM-TOOL-SURFACE-004`: disabled or declared-only tools MAY be surfaced when relevant to explain capability limits.
- `INV-LLM-TOOL-SURFACE-005`: effective tool surface MUST be generated by governed policy, not by UI state or LLM guessing.
- `INV-LLM-TOOL-SURFACE-006`: `active_only` MUST mean `visible`, allowed in project, execution enabled, and execution implemented.
- `INV-LLM-TOOL-SURFACE-007`: `all` returns a governed surface with state and limits, not raw unfiltered catalog data.
- `INV-LLM-TOOL-SURFACE-008`: `EffectiveToolSurface` MUST distinguish declared, visible, enabled, implemented, and executable state.
- `INV-LLM-TOOL-SURFACE-009`: explanation fields MUST remain advisory context, not execution authority.
- `INV-LLM-TOOL-SURFACE-010`: `project_runtime` MUST NOT participate in effective tool surface derivation.
- `INV-LLM-TOOL-SURFACE-011`: agent prompts MUST reference tools only through `EffectiveToolSurfaceSummary`.
- `INV-LLM-TOOL-SURFACE-012`: prompt text MUST NOT hardcode tool availability or execution permission.
- `INV-LLM-TOOL-SURFACE-013`: only the defined `tool_class` values are valid: `operational`, `llm`, `external_dependency`, `base_utility`.
- `INV-LLM-TOOL-SURFACE-014`: `tool_class` MUST be explicit and MUST NOT be inferred implicitly.
- `INV-LLM-TOOL-SURFACE-015`: agent roles are not `tool_class` values.
- `INV-LLM-TOOL-SURFACE-016`: no new `tool_class` value may be introduced without an explicit governance update.
- `INV-LLM-TOOL-SURFACE-017`: the first F10 opening gate may expose bounded summary/runtime resolution only; it MUST NOT open tool execution.
- `INV-LLM-TOOL-SURFACE-018`: the first F10 opening gate MUST NOT inject the raw full catalog or authorize provider/tool execution by implication.

## Forbidden responsibilities

The Tool Surface Resolver must not:

- execute tools
- call LLM providers
- authorize actions
- mutate files
- bypass `ActionResolution`
- reinterpret `project_manifest`
- replace tool catalogs
- expose raw catalog by default
- treat UI as deciding effective permissions, policy, or capability state
- open F10
- define prompts
- edit prompts

This spec must not:

- implement a resolver
- bind runtime
- mutate catalogs
- define UI authority
- define execution API
- embed prompt text

## Relationship to tools catalogs

- tool catalogs declare possible tools
- project policy restricts effective availability
- Tool Surface Resolver computes the LLM-facing effective surface
- it does not create, modify, or execute tools

## Relationship to Project Setup / Settings

- project configuration may restrict allowed tools
- `project_profile` may influence relevant tools
- changing project configuration does not enable execution by itself

## Relationship to status bar

- `EffectiveToolSurface` summary may feed the tools block in a future status bar
- the status bar must not consume raw catalog data directly

## Relationship to System View

- future readonly `System View` may consume prepared `EffectiveToolSurfaceSummaryViewState`
- `System View` must consume only prepared summary state, not raw catalog data
- `System View` must not toggle tools, resolve permissions, or create execution authority

## Integration boundaries

- `project_runtime`: not involved
- `tool_runtime`: remains the current operative source in F9/F10_PREP; not replaced by this spec
- UI: consumer only
- LLM: future consumer only
- `app_services`: future thin adapter only

No boundary in this spec gains execution authority.

## Future resolver hook

Future crate boundary only:

- `app_services` may host a thin adapter that requests a resolved surface
- the future resolver should live below UI and outside `project_runtime`
- the resolver must consume governed declarations and policy inputs only

Conceptual future interface:

- input: master catalog + operative runtime declarations + project policy + global policy + capability flags + mode
- output: `EffectiveToolSurface`

No crate code, public API, or runtime binding is defined here.

## Related specs

- `docs/specs/llm_core.md`
- `docs/specs/tools_catalogs.md`
- `docs/specs/tool_implementation_governance.md`
- `docs/specs/project_setup_popup.md`
- `docs/specs/action_resolution.md`
- `docs/specs/llm_agent_prompts.md`
- `docs/specs/system_view.md`
