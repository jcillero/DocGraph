# tool_implementation_governance

## Status

NORMATIVE DOCUMENTATION.

No runtime implementation.

## Purpose

Define how a new DocGraph tool is declared, classified, documented, and implemented without breaking the architecture.

This spec governs tool contracts before implementation. It does not create executable tools, open F10, call LLMs, create graph runtime, or modify `project_runtime`.

## Tool kinds

- `operational`: deterministic system action that may become executable only in a later governed phase.
- `llm`: capability available to LLM reasoning surfaces; not manually executable in F9.
- `external`: external binary or runtime dependency; not a tool by itself.
- `agent`: composite tool-like provider shape; future-only unless explicitly opened.
- `base`: internal support capability; not user-facing.

`tool_kind` is the canonical classification term.

Legacy `tool_class` is a deprecated alias only when retained for historical compatibility.

`tool_kind` defines classification only.

It does not define execution.

Tool visibility does not imply execution permission.

Future tool capabilities are governed separately by `docs/specs/tool_capability_model.md`.

LLM tools are not agent roles.

Agent roles are governed through `tool_kind=agent`, not through legacy `tool_class`.

## Required tool declaration fields

Every tool declaration MUST include or intentionally mark as not applicable:

- `tool_id`
- `display_name_i18n`
- `description_i18n`
- `tool_kind`
- `category`
- `catalog_ref`
- `status`
- `execution_enabled`
- `requires_confirmation`
- `requires_owner_ref`
- `input_schema`
- `output_schema`
- `persistence_policy`
- `owner_policy`
- `risk_level`
- `allowed_contexts`
- `forbidden_contexts`
- `trace_requirements`

## Input argument contract

Every tool MUST have an explicit `input_schema`.

Recommended argument fields:

- `name`
- `type`
- `required`
- `multiplicity`
- `allowed_extensions`
- `source_policy`
- `validation_rules`
- `description_i18n`

Initial allowed input types:

- `file_ref`
- `file_ref_list`
- `path_ref`
- `text`
- `boolean`
- `enum`
- `integer`
- `object`

Input rules:

- Host-specific absolute paths MUST NOT be persisted as truth.
- Inputs SHOULD resolve as governed references when possible.
- External inputs require explicit intake or explicit user selection.
- A tool MUST NOT reinterpret `project_manifest.json`.
- Input lists whose order affects the result MUST declare `ordered=true`.

## Input source policy

Allowed `source_policy` values:

- `governed_reference`
- `explicit_user_selection`
- `external_intake`
- `governed_reference_or_explicit_user_selection`

Rules:

- Every input MUST declare `source_policy`.
- `external_intake` requires explicit user intent.
- Host-specific absolute paths MUST NOT be persisted as source truth.

## Input ordering

- If order matters, input schema MUST declare `ordered=true`.
- Ordered input order MUST be persisted in `tool_run_manifest.json`.

## Input vs configuration

- Inputs are source data.
- Configuration controls behavior.
- `output_filename` belongs to configuration, not source input, unless catalog migration keeps legacy shape temporarily.

## Output contract

Every tool MUST declare an explicit `output_schema`.

Recommended output fields:

- `name`
- `type`
- `required`
- `persistence`
- `target_policy`
- `allowed_extensions`
- `description_i18n`

Initial allowed output types:

- `file`
- `file_list`
- `json`
- `text`
- `table`
- `log`
- `manifest`
- `none`

Output rules:

- If `persistence = persisted`, `owner_ref` is mandatory.
- Every persisted output MUST live under the functional owner.
- Project-root `outputs/` MUST NOT be used.
- Every persisted run MUST generate `tool_run_manifest.json`.

## tool_run_manifest.json minimum contract

Every persisted tool run MUST record:

- `tool_id`
- `tool_kind`
- `run_id`
- `owner_ref`
- `inputs`
- `configuration`
- `outputs`
- `status`
- `started_at` / `finished_at` or `created_at`
- `trace`
- `errors` / `warnings`, when applicable

## Tool run lifecycle

Allowed future lifecycle states:

- `created`
- `running`
- `completed`
- `failed`
- `cancelled`

No runtime implementation is created by this declaration.

## Structured errors

Minimum future error kinds:

- `validation_error`
- `execution_error`
- `system_error`
- `policy_error`

Errors and warnings MUST be structured records, not only free text.

## Reproducibility

Persisted runs SHOULD be reproducible from:

- `inputs`
- `configuration`
- `tool_id`
- `tool_kind`
- `owner_ref`
- trace data

## Catalog governance

Every tool MUST be declared in the correct catalog.

Operational document tools:

- `resources/tools/internal/operational/tools_operational_document_catalog.json`

LLM document tools:

- `resources/tools/internal/llm/tools_llm_document_catalog.json`

External dependencies:

- `resources/tools/external/tools_external_catalog.json`

Metacatalog:

- `resources/tools/tools_metacatalog.json`

Catalog rules:

- `tools_metacatalog` indexes catalogs, not implementations.
- The catalog declares tools, not code.
- For declared but non-executable tools, `enabled=true` MAY mean visible or declared in the catalog.
- `execution_enabled=false` means the tool is not executable.
- `status=declared_only` MUST be used for non-executable F9-ready declarations to avoid ambiguity.
- Visible strings MUST come from i18n keys or declarative fields, not hardcoded UI.
- `resources/tool_runtime/*` remains the current operative canonical source consumed by runtime in the current phase when declared by `docs/specs/tools_catalogs.md`.
- `resources/tools/*` remains F9-ready declarative layout unless a future controlled transition explicitly changes that.

## Tools menu governance

`Tools` is a top-level menu domain separate from `Preferences`.

It exposes governed surfaces for:

- `Operational Tools`
- `LLM Tools`
- `External Dependencies`

These menu surfaces do not execute.

They do not authorize.

They do not replace `ActionResolution`.

There is no single combined tools panel with tabs in this governed model.

## Operational tool preparation boundary

Operational tools may be manually prepared by the user through a governed popup surface.

That popup may capture:

- inputs
- configuration
- ordered input policy, when relevant
- `owner_ref`
- output destination
- risk summary
- confirmation intent

The popup emits intent only.

It must not execute.

Any future execution must pass through:

`ActionRequest -> ActionResolution -> authorized executor -> trace`

Future matching should resolve required capabilities before selecting candidate tools.

## LLM tool boundary

LLM tools are not manually executable.

They expose governed policy or surface only.

They must not be run from UI.

They must not bypass `EffectiveToolSurfaceResolver`.

The raw full tool catalog must not be injected into LLM context by default.

## External dependency boundary

External dependencies are not tools by themselves.

They are binaries or software that may be required by internal tools.

User-managed external dependencies belong under:

- `user/runtime/external_dependencies/`

They must not be stored under:

- `resources/`
- `assets/`
- `crates/`
- project folders

Install or configure surfaces may capture metadata, local reference, and intended availability only.

They must not:

- download automatically
- execute installers
- modify `PATH`
- run binaries
- treat validation as execution authorization

Custom external dependencies remain user-declared only.

They do not create new internal tools.

They do not enable execution.

## Effective tool surface boundary

`MasterCatalog` is declarative capability source only.

It must not:

- execute
- authorize
- resolve availability
- replace `resources/tool_runtime/*` in F9

Any future operative consumption of master-catalog declarations requires a mandatory `EffectiveToolSurfaceResolver`.

`EffectiveToolSurfaceResolver` must derive effective tool state from:

- master-catalog declarations
- phase policy
- project policy
- global tool policy
- implementation availability
- external dependency availability
- credentials or capability state, when applicable

`EffectiveToolSurfaceResolver` must not:

- live in UI
- be the LLM
- bypass `ActionResolution`

`EffectiveToolSurface` is derived state only.

It may distinguish:

- `declared`
- `visible`
- `allowed`
- `implemented`
- `available`
- `executable`

Rules:

- `visible` is not permission
- `surfaces` is not execution availability
- `executable` must remain false in F9 and may become true only in a later explicitly opened phase
- capability declaration must not be treated as execution authority
- `tool_runtime` MUST NOT consume `resources/tools/tools_master_catalog.json` directly in F9
- any future connection between master catalog and `tool_runtime` requires the explicit phase `F10_PREP_EFFECTIVE_TOOL_SURFACE`
- until that phase opens, `resources/tool_runtime/*` remains the operative source

## F9 minimum governed execution slice

F9 may include one minimal governed operational execution slice without opening F10.

The first minimal slice is `text.measure`.

Rules:

- output must be written under `user/output/tool_runs/<owner-kind>/<owner-id>/<run_id>/`
- `owner_ref` is mandatory
- `tool_run_manifest.json` is mandatory
- no LLM execution
- no external binary execution
- no `EffectiveToolSurfaceResolver`
- no F10 opening

## F9 persisted output discipline

Persisted F9 operational outputs must follow a minimal governed discipline.

Rules:

- `owner_ref` is mandatory
- `owner_ref` is ownership and traceability only; it is not permission
- `tool_run_manifest.json` is mandatory
- every completed run must provide `result.json`
- failed runs must remain traceable
- outputs must stay under owner-scoped paths inside `user/output/tool_runs/`
- root `outputs/` is forbidden
- manifest metadata is traceability only and is not execution authority

## F9 Operational Boundary Closure

F9 currently includes one minimum governed execution slice: `text.measure`.

Its persisted outputs, `owner_ref`, and manifests must be validated by scripts.

The following do not exist in this phase:

- `ActionResolution` runner
- `EffectiveToolSurfaceResolver`
- LLM execution
- external tool execution
- sandbox mutation

An inert `ActionRequest` contract type may exist.

Credential or provider-readiness contracts may exist, but they MUST NOT grant invocation authority or enable cloud execution in the default build.

`ui_slint` may emit intent or render prepared state, but it must not dispatch operational tools, resolve LLM tool policy, mutate LLM tool policy, or decide permissions.

`tools_master_catalog.json` remains declarative only.

F10 remains blocked.

Any introduction of dynamic resolution, tool selection, or external execution implies transition to a future phase and must be governed explicitly.

## F11.6 first execution-contract boundary

`SingleToolExecution` may be declared in `F11.6` as a contract only.

The first future execution-contract scope is constrained to one local deterministic tool and must remain:

- `SINGLE_TOOL_LOCAL_DETERMINISTIC`
- `SANDBOX` scoped
- limited to `operational` or `base` tool kinds
- owner-scoped for any future outputs
- trace and `ToolRunManifest` required before any future runtime execution

This declaration does not implement a runner, dispatcher, executor, tool invocation, provider invocation, external binary invocation, output persistence, `ToolRunManifest` production, or `TraceRecord` persistence.

## F11.7 ToolRunManifest alignment

Future `SingleToolExecution` runtime requires a `ToolRunManifest`.

Future `ToolRunManifest` discipline must preserve:

- `single_tool_execution_ref`
- `authorized_execution_request_ref`
- `human_confirmation_ref`
- `action_request_ref`
- `intent_ref`
- `trace_ref`
- `owner_ref`
- tool identity and implementation metadata
- governed input refs
- separate configuration
- owner-scoped outputs

Rules:

- `ToolRunManifest` is mandatory for future runtime execution
- `F11.7` does not create `tool_run_manifest.json`
- inputs remain refs, not raw payloads
- configuration must remain separate from inputs
- outputs must remain owner-scoped
- no project-root `outputs/`
- no secrets
- no private absolute host paths
- `ToolRunManifest` status remains non-operational in `F11.7`

## F10 step-1 closure

The first explicit F10 opening gate does not open broad tool execution.

In F10 step 1:

- operational tools remain closed except for the existing minimum F9 operational boundary
- LLM tools remain non-manually-executable
- external dependencies remain non-executing declarations
- `EffectiveToolSurface` may become a bounded runtime concern without becoming execution authority
- UI remains intent-only and presentation-only

## Visible text requirements

Each tool MUST declare or reference:

- `display_name_i18n`
- `short_description_i18n`
- `long_description_i18n`
- `input_labels_i18n`
- `output_labels_i18n`
- `confirmation_message_i18n`
- `success_message_i18n`
- `error_message_i18n`

Visible text MUST NOT be hardcoded in UI.

## Implementation checklist

Before implementing a new Operational Tool:

1. Classify `tool_kind`.
2. Choose the correct catalog.
3. Add or verify metacatalog entry.
4. Declare `input_schema`.
5. Declare `output_schema`.
6. Declare `owner_policy`.
7. Declare `persistence_policy`.
8. Declare visible texts or i18n keys.
9. Define `tool_run_manifest.json`.
10. Define minimum errors.
11. Implement in `tool_runtime` only if a phase permits it.
12. Add tests.
13. Validate with wrappers.

## Forbidden responsibilities

A tool MUST NOT:

- create root `outputs/`
- execute without `owner_ref` if it persists outputs
- call LLM directly
- act as UI
- interpret the manifest by itself
- update `graph/` by scanning
- execute an `ActionResolution` runner
- mutate `SOURCE` or `DERIVED`
- use `assets/` as runtime
- hardcode routes, models, providers, or visible text

## Project profile note

Tools may declare `allowed_project_profiles`.

`allowed_project_profiles` is policy metadata, not execution authority.

Folder organization tools must treat the original source folder as readonly and mutate only the sandbox working copy.

Tools may also expose metadata useful for effective tool surface resolution, including:

- `category`
- `status`
- `execution_enabled`
- `execution_implemented`
- `allowed_project_profiles`
- `risk_level`
- `reason`
- `limitations`

## Example: merge_pdfs

This is a non-executable declaration example.

- `tool_id`: `merge_pdfs`
- `tool_kind`: `operational`
- `category`: `document_processing`
- input:
  - `pdf_files`: `file_ref_list`
  - `required`: `true`
  - `min_items`: `2`
  - `allowed_extensions`: [`.pdf`]
- config:
  - `output_filename`: `text`
  - `default`: `merged.pdf`
- output:
  - `merged_pdf`: `file`
  - `extension`: `.pdf`
  - `persistence`: `persisted`
- `owner_ref`:
  - `required`: `true`
  - allowed owners: `chat`, `DocumentHolder`
- catalog:
  - `resources/tools/internal/operational/tools_operational_document_catalog.json`
- graph:
  - no direct graph update in F9
  - future governed action may record `produced_tool_output`

`merge_pdfs` is an Operational Tool, not an LLM Tool.

## Tool kind invariants

- `tool_kind` is the canonical governed classification term.
- Only the defined `tool_kind` values are valid: `operational`, `llm`, `external`, `agent`, `base`.
- `tool_kind` MUST be explicit for every governed tool declaration.
- No tool declaration MAY rely on implicit or inferred `tool_kind`.
- Legacy `tool_class` may remain only as a deprecated alias in historical declarations and must not override `tool_kind`.
- No new `tool_kind` value may be introduced without an explicit governance update.
- The first F10 opening gate MUST NOT be interpreted as authorizing broad tool execution, dependency execution, or UI-driven execution.

## F11.0 future execution gate

`F11.0` is DECLARED / NOT ACTIVE.

The first future implementation slice may open only one registered deterministic local tool.

That future slice requires:

- mandatory `owner_ref`
- mandatory `tool_run_manifest`
- mandatory trace
- explicit lifecycle states
- explicit failure states

It must not open:

- external binaries unless separately gated
- provider invocation
- LLM autonomous tool calling
- broad tool execution
- UI execution authority

## F12.0 / F11.RUNTIME-0 minimal governed runtime proposal

Status: PROPOSAL / NOT IMPLEMENTED.

The first possible runtime opening may later target one registered deterministic local tool only.

The recommended first tool is `text.measure` because it is already minimal, deterministic, local, provider-free, external-binary-free, low risk, and aligned with existing governed output discipline.

The eligible first tool class is limited to `operational` or `base`.

This proposal does not open:

- `merge_pdfs`
- external tools
- LLM tools
- agent tools
- graph analysis runtime
- RDF/Oxigraph/SPARQL
- embeddings
- document mutation
- filesystem writes outside owner-scoped sandbox output

Any future implementation slice must name:

- crate touched
- exact tool id
- exact input contract
- exact output location policy
- exact manifest path policy
- exact trace path policy
- exact validation command
- exact audit command

Until such a slice is explicitly opened, this spec creates no runner, dispatcher, executor, output, `tool_run_manifest.json`, or `TraceRecord`.

## F12.1 Minimal Runtime Gate SPEC for text.measure

Status: GATE-ONLY / NOT IMPLEMENTED.

First eligible tool:

- exact `tool_id`: `text.measure`

Accepted input contract:

- governed text input ref
- no raw payloads
- no secrets
- no private absolute host paths
- ordered input is not required unless explicitly stated later

Output contract:

- future `result.json`
- owner-scoped location only
- no project-root `outputs/`
- `owner_ref` mandatory

Manifest and trace:

- `ToolRunManifest` is mandatory for future execution
- manifest path policy is declared only
- `tool_run_manifest.json` is not created in `F12.1`
- `TraceRecord` is mandatory for future execution
- `trace_ref` mandatory
- `TraceRecord` is not persisted in `F12.1`

Allowed lifecycle placeholders:

- `DECLARED_ONLY`
- `NOT_RUN`
- `BLOCKED`
- `STALE`

Excluded runtime statuses:

- `RUNNING`
- `SUCCEEDED`
- `FAILED`
- `RETRYING`
- `DISPATCHED`
- `EXECUTED`

Future-only error model:

- `missing_owner_ref`
- `missing_trace_ref`
- `unsafe_input`
- `stale_input`
- `invalid_domain`
- `manifest_required`
- `trace_required`

Crate boundary:

- `tool_runtime` may be future implementation owner
- `tool_runtime` must remain a single-tool adapter only
- `tool_runtime` must not become a general dispatcher
- `project_runtime` must not be modified
- `action_core` remains contracts only
- UI remains presentation only

Mandatory audit before implementation:

- audit minimal runtime gate
- verify no `project_runtime` expansion
- verify no broad dispatcher
- verify `owner_ref`, `trace_ref`, and manifest requirements

## F12.2A implementation plan for text.measure

Status: PLAN-ONLY / AUDIT-CHECKLIST-ONLY / NOT IMPLEMENTED.

`F12.2A` does not modify `crates/tool_runtime` and does not open runtime.

Future `F12.2B` may touch only:

- `crates/tool_runtime`

Future `F12.2B` must not touch:

- `project_runtime`
- `app_services`
- `ui_core`
- `ui_slint`
- `llm_core`
- `llm_cloud`
- `llm_local`

`action_core` may be consumed only for existing contract references if needed. It must not execute.

Allowed future implementation scope:

- exact `tool_id`: `text.measure`
- single local deterministic adapter
- `owner_ref` enforcement
- `trace_ref` requirement
- owner-scoped output directory
- `result.json` creation
- `tool_run_manifest.json` creation
- `TraceRecord`-compatible metadata creation only if already declared by `F12.1` and `F11.7`
- tests for success and blocked cases

Forbidden future implementation scope:

- general dispatcher
- multi-tool runner
- provider calls
- network
- external binaries
- `HOST` write
- LLM tools
- agent tools
- `merge_pdfs`
- graph, RDF, or semantic runtime
- document mutation
- UI-triggered execution

Required `F12.2B` tests:

- successful `text.measure` run creates owner-scoped `result.json`
- successful `text.measure` run creates `tool_run_manifest.json`
- missing `owner_ref` is rejected
- missing `trace_ref` is rejected
- output path cannot escape owner-scoped sandbox
- manifest contains no private absolute host path
- runtime accepts only `text.measure`
- runtime rejects non-`text.measure` `tool_id`
- runtime does not call providers
- runtime does not touch `project_runtime`

Mandatory audit:

- `dev/scripts/audits/audit_f12_minimal_runtime_boundary.bat`

The audit must verify no `project_runtime` changes, no `app_services` or UI execution authority, no general dispatcher, no provider calls, no external binary invocation, no network access, no `HOST` write, only `text.measure` executable, required `owner_ref`, required `trace_ref`, required manifest, owner-scoped outputs, and no project-root outputs.

## F12.4 file intake tool relationship

`docs/specs/file_intake.md` owns governed file intake semantics.

File intake is a governed pipeline, not merely a tool.

Future intake may use small `operational` or `base` tools, but `tool_runtime` must not become the intake orchestrator and must not become a broad dispatcher.

Candidate future intake tools are declared only:

- `classify_file_format`
- `compute_file_hash`
- `copy_into_file_store`
- `create_stored_object_metadata`
- `plan_derivatives`
- `validate_intake_manifest`

These tools are not implemented or activated in `F12.4`.

## F12.5 file intake implementation plan

`F12.5` is plan-only and audit-checklist-only.

Future `F12.6` must not use `tool_runtime` as the file intake orchestrator.

Future intake may use small tools only in later explicitly opened phases. `F12.6` must keep the first minimal intake implementation inside the declared crate boundary and must not create a general dispatcher, multi-tool runner, provider call path, network path, external binary path, LLM tool path, or agent tool path.

The future audit `dev/scripts/audits/audit_f12_file_intake_boundary.bat` must verify that no broad `tool_runtime` orchestrator was introduced.

## Validation

Recommended after declarative changes:

```bat
dev\scripts\generate_llm_context_bundle.bat
dev\scripts\generate_status_snapshot.bat
dev\scripts\audits\audit_tools_compliance.bat
dev\scripts\validate_f9_declarations.bat
dev\scripts\validate_ai_specs.bat
```

If Rust is touched in a future phase:

```bat
dev\scripts\cargo_check.bat
dev\scripts\cargo_test.bat
```

## Procedural guide

For the practical checklist-driven procedure to add a new tool, use:

- `docs/specs/how_to_add_a_tool.md`
