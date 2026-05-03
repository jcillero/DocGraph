# action_resolution

## Purpose

Define the F9-ready declarative model for resolving proposed actions before any future execution phase.

`ActionResolution` is the governed process between:

`ActionRequest -> ContextResolution -> RiskClassification -> PolicyEvaluation -> ResolutionDecision -> Interaction -> Trace`

This is a specification only. In F9, `execution_enabled=false`.

## F9 limits

In F9:

- no runner exists
- no real execution exists
- no full runtime enforcement exists
- an inert `ActionRequest` contract type may exist, but no runner, dispatcher, executor, or mutation path exists
- no ActionResolution runner exists
- no tool output persistence is authorized by this spec
- no `tool_run_manifest.json` generation is implemented by this spec
- F10 is not opened
- LLMs do not execute actions
- tools do not autoexecute
- chat does not execute actions
- UI does not decide or execute actions
- filesystem mutation is not introduced

The model prepares governance, not operational behavior.

## F10 step-1 closure

The first explicit F10 opening gate does not open `ActionResolution` execution.

In F10 step 1:

- no `ActionResolution` runner exists
- no dispatcher exists
- no authorized executor is introduced by this spec
- no tool execution is authorized through `ActionResolution`
- no provider invocation is authorized through `ActionResolution`

`ActionResolution` remains declarative governance only until a later explicit phase opens executable action handling.

## F10.9 resolution-preparation slice

`F10.9` opens resolution preparation only.

It may define how proposal-level and draft-level artifacts become inert governed resolution candidates before any future execution phase.

`F10.9` does not authorize execution.

`F10.9` does not create an `ActionResolution` runner.

`F10.9` does not invoke providers.

`F10.9` does not execute tools.

`F10.9` does not mutate files.

`F10.9` keeps UI presentation-only.

`F10.9` keeps `project_runtime` unchanged.

## F11.0 execution gate note

`F11.0` is DECLARED / NOT ACTIVE.

`ActionResolution` remains non-running in `F11.0`.

The runner remains future and separately gated.

`ResolutionCandidate` is evaluated but not executable.

`PendingActionCandidate` is confirmable but not executable.

`HumanConfirmation` is approval signal only, not execution.

`AuthorizedExecutionRequest` is future-only and is not active in `F11.0`.

`SingleToolExecution` is future-only and is not active in `F11.0`.

## F11.2 ResolutionCandidate opening note

`F11.2` opens only the rich `ResolutionCandidate` contract.

It does not open resolver runtime.

It does not open execution.

It does not authorize anything.

It does not select tools for execution.

## F11.3 PendingActionCandidate note

`F11.3` opens only the rich `PendingActionCandidate` contract downstream from `ResolutionCandidate`.

`PendingActionCandidate` remains confirmation-preparation only.

No transition runtime, confirmation runtime, authorization runtime, or execution runtime is opened by `F11.3`.

## F11.4 HumanConfirmation note

`F11.4` opens only the `HumanConfirmation` contract downstream from `PendingActionCandidate`.

`HumanConfirmation` is an explicit human decision event.

It is not authorization.

It is not execution.

No transition runtime, authorization runtime, runner, dispatcher, executor, or tool execution is opened by `F11.4`.

## F11.5 AuthorizedExecutionRequest note

`F11.5` opens only the `AuthorizedExecutionRequest` contract downstream from `HumanConfirmation`.

`AuthorizedExecutionRequest` is a governed post-confirmation authorization artifact.

It prepares a future execution request.

It is not execution.

No authorization runtime, runner, dispatcher, executor, `SingleToolExecution` runtime, tool execution, provider invocation, filesystem mutation, or output persistence is opened by `F11.5`.

## F11.6 SingleToolExecution note

`F11.6` opens only the `SingleToolExecution` contract downstream from `AuthorizedExecutionRequest`.

`SingleToolExecution` is the future minimum governed execution artifact for one local deterministic tool execution event.

It is declared-only in `F11.6`.

No runtime execution, runner, dispatcher, executor, tool invocation, provider invocation, external binary invocation, filesystem mutation, real output creation, `ToolRunManifest` production, or `TraceRecord` persistence is opened by `F11.6`.

## F11.7 ToolRunManifest and TraceRecord alignment note

`F11.7` opens only future `ToolRunManifest` and `TraceRecord` contract alignment downstream from `SingleToolExecution`.

`ToolRunManifest` remains a future runtime artifact.

`TraceRecord` remains a future runtime artifact.

No persistence, filesystem mutation, real output creation, registry update, graph update, or runtime execution is opened by `F11.7`.

## F12.0 / F11.RUNTIME-0 minimal governed runtime proposal

Status: PROPOSAL / NOT IMPLEMENTED.

`F12.0 / F11.RUNTIME-0` defines the first possible minimal runtime opening after the F11 action-governance contracts are ready.

Runtime remains closed until a later explicit implementation slice.

The proposed first runtime scope is:

- one tool only
- local only
- deterministic only
- `SANDBOX` only
- no `HOST` write
- no `EXTERNAL`
- no network
- no provider invocation
- no external binary
- no agent tool
- no LLM tool
- no multi-tool orchestration
- no autonomous execution
- no UI authority

The eligible first tool class is `operational` or `base` only.

Recommended first tool: `text.measure`.

Future execution may only occur if these artifacts and refs exist:

- `ActionRequest`
- `ResolutionCandidate`
- `PendingActionCandidate`
- `HumanConfirmation`
- `AuthorizedExecutionRequest`
- `SingleToolExecution`
- `owner_ref`
- `trace_ref`
- declared `ToolRunManifest` requirement
- declared `TraceRecord` requirement
- `FRESH` staleness result
- `SANDBOX` domain
- allowed `tool_kind`
- satisfied capability requirement
- sanitized inputs
- owner-scoped output plan

No precondition may be inferred by UI.

A later implementation slice must name the crate touched, exact tool id, exact input contract, exact output location policy, exact manifest path policy, exact trace path policy, exact validation command, and exact audit command.

Until then, `SingleToolExecution` remains declared-only, `ToolRunManifest` remains future-required, `TraceRecord` remains future-required, and execution boundary remains CLOSED.

## F12.1 Minimal Runtime Gate SPEC

Status: GATE-ONLY / NOT IMPLEMENTED.

`F12.1` defines the exact preconditions for a later implementation slice to propose the first minimal runtime execution.

Runtime remains CLOSED after `F12.1`.

First eligible tool:

- exact `tool_id`: `text.measure`

Accepted input contract:

- governed text input ref
- no raw payloads
- no secrets
- no private absolute host paths
- ordered input is not required unless explicitly declared by a later implementation slice

Output contract:

- future output is `result.json`
- owner-scoped location only
- no project-root `outputs/`
- `owner_ref` mandatory

Manifest and trace requirements:

- `ToolRunManifest` is mandatory for future execution
- manifest path policy is declared only
- `ToolRunManifest` is not created in `F12.1`
- `TraceRecord` is mandatory for future execution
- `trace_ref` is mandatory
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

Mandatory audit before implementation:

- audit minimal runtime gate
- verify no `project_runtime` expansion
- verify no broad dispatcher
- verify `owner_ref`, `trace_ref`, and manifest requirements

Crate boundary:

- `tool_runtime` may be future implementation owner
- `tool_runtime` must remain a single-tool adapter only
- `project_runtime` must not be modified
- `action_core` remains contracts only
- UI remains presentation only

## F12.2A implementation plan and audit checklist

Status: PLAN-ONLY / AUDIT-CHECKLIST-ONLY / NOT IMPLEMENTED.

`F12.2A` prepares a later `F12.2B` implementation slice for the first minimal governed `text.measure` runtime.

Runtime remains CLOSED after `F12.2A`.

Future `F12.2B` may touch only `crates/tool_runtime` unless explicitly re-audited.

Explicitly forbidden crates:

- `project_runtime`
- `app_services`
- `ui_core`
- `ui_slint`
- `llm_core`
- `llm_cloud`
- `llm_local`

`action_core` may be consumed only for existing contract references if needed and must not execute.

Future `F12.2B` may implement only:

- `text.measure` execution
- a single local deterministic tool adapter
- `owner_ref` enforcement
- `trace_ref` requirement
- owner-scoped output directory
- `result.json` creation
- `tool_run_manifest.json` creation
- `TraceRecord`-compatible metadata creation only if already declared by `F12.1` and `F11.7`
- success and blocked-case tests

Future `F12.2B` must not implement a general dispatcher, multi-tool runner, provider call, network access, external binary invocation, `HOST` write, LLM tool, agent tool, `merge_pdfs`, graph/RDF/semantic runtime, document mutation, or UI-triggered execution.

Acceptance checklist:

- `tool_id` is exactly `text.measure`
- `tool_kind` is `operational` or `base` as declared
- input is governed text input only
- no raw payloads are persisted
- `owner_ref` is mandatory
- `trace_ref` is mandatory
- missing `owner_ref` blocks execution
- missing `trace_ref` blocks execution
- unsafe input blocks execution
- stale input blocks execution if staleness is present
- output path is owner-scoped
- no project-root outputs
- `result.json` exists only after successful execution
- `tool_run_manifest.json` exists only after successful execution or governed failed run if explicitly allowed
- manifest contains inputs, configuration, `owner_ref`, outputs, status, and trace data
- manifest contains no private absolute host paths
- result, manifest, and trace metadata contain no secrets
- `project_runtime` remains unchanged
- `tool_runtime` does not become a general dispatcher

Mandatory audit before closure:

- `dev/scripts/audits/audit_f12_minimal_runtime_boundary.bat`

## F11.2 evaluation bridge contract

The governed non-executing chain is:

`ActionRequest -> ResolutionCandidate -> PendingActionCandidate -> HumanConfirmation -> AuthorizedExecutionRequest -> SingleToolExecution -> future ToolRunManifest -> future TraceRecord`

Rules:

- `F11.2` opens only the `ResolutionCandidate` contract
- `PendingActionCandidate` remains preparation-only
- `HumanConfirmation` is a non-executing decision event
- `AuthorizedExecutionRequest` is a post-confirmation authorization artifact
- `F11.6` opens only the `SingleToolExecution` contract
- `SingleToolExecution` remains declared-only and non-running in `F11.6`
- `F11.7` opens only manifest and trace contract alignment
- `ToolRunManifest` production remains future runtime behavior
- `TraceRecord` persistence remains future runtime behavior
- execution boundary remains `CLOSED`

`ActionIntent` is the declarative action contract that may sit between proposal or draft artifacts and future `ActionRequest` handling.

`ActionRequest` is the richer reviewable request that may feed inert `ResolutionCandidate` preparation.

In `F11.2`:

- `ToolUseProposal` remains proposal-only
- `ActionRequestDraft` remains inert
- `ActionIntent` remains non-executing
- `ActionRequest` remains non-executing
- `ResolutionCandidate` is inert evaluation only
- `PendingActionCandidate` is inert confirmation-preparation state only
- execution remains future and separately gated

### ResolutionCandidate

`ResolutionCandidate` is a rich inert evaluation artifact derived from `ActionRequest`.

It represents whether an `ActionRequest` appears:

- resolvable
- blocked
- ambiguous
- unsupported
- unsafe
- stale

It does not compute resolution at runtime.

It does not authorize anything.

It does not select execution.

It does not execute.

Conceptual structure:

```json
{
  "resolution_candidate_id": "rc_xxx",
  "action_request_ref": "ar_xxx",
  "intent_ref": "intent_xxx",
  "trace_ref": "trace_xxx",
  "status": "CANDIDATE",
  "resolution_state": "RESOLVABLE | BLOCKED | AMBIGUOUS | UNSUPPORTED | UNSAFE | STALE",
  "capability_evaluation": [],
  "domain_evaluation": [],
  "policy_evaluation": [],
  "candidate_tools": [],
  "blocking_reasons": [],
  "risk": {
    "risk_level": "LOW | MEDIUM | HIGH | UNKNOWN",
    "risk_reasons": []
  },
  "staleness": {
    "is_stale": false,
    "stale_reasons": []
  },
  "created_at": "..."
}
```

### ResolutionCandidate status and state model

`ResolutionCandidate` status values:

- `CANDIDATE`
- `BLOCKED`
- `SUPERSEDED`
- `EXPIRED`

`ResolutionCandidate` resolution-state values:

- `RESOLVABLE`
- `BLOCKED`
- `AMBIGUOUS`
- `UNSUPPORTED`
- `UNSAFE`
- `STALE`

Rules:

- `RESOLVABLE` does not mean authorized
- `RESOLVABLE` does not mean executable
- `CANDIDATE` does not mean selected
- `BLOCKED` does not auto-fix
- `STALE` must not progress silently
- there is no `AUTHORIZED`, `EXECUTING`, or `EXECUTED` state here

### Capability evaluation

Capability evaluation entries may include:

- `capability`
- `requirement_status`
- `reason_code`
- `notes_key`

Allowed `requirement_status` values:

- `satisfied`
- `missing`
- `disabled`
- `future_only`
- `unknown`

Rules:

- capability evaluation is descriptive
- satisfied capability does not grant execution
- `missing`, `disabled`, `future_only`, or `unknown` should block progression
- no tool execution is selected here

### Domain evaluation

Domain evaluation entries may include:

- `required_domain`
- `access_level`
- `domain_status`
- `reason_code`

Allowed `domain_status` values:

- `allowed_by_contract`
- `forbidden`
- `future_gated`
- `unspecified`
- `unknown`

Rules:

- `allowed_by_contract` does not authorize runtime access
- `HOST` write remains forbidden
- `EXTERNAL` remains closed unless a future gate opens it
- `UNSPECIFIED` blocks when access is required

### Policy evaluation

Policy evaluation entries may include:

- `policy_ref`
- `policy_status`
- `reason_code`

Allowed `policy_status` values:

- `satisfied`
- `blocked`
- `warning`
- `unknown`

Policies may include:

- security and sanitization
- sandbox
- tool capability
- LLM or provider
- project
- future cost
- future credentials

Rules:

- policy evaluation does not enforce runtime behavior
- `satisfied` does not authorize execution
- unknown critical policy blocks progression

### Candidate tools

`candidate_tools` are explanatory only.

Candidate-tool fields may include:

- `tool_id`
- `tool_kind`
- `capability_match`
- `availability_state`
- `reason_code`

Allowed `availability_state` values:

- `declared`
- `visible`
- `implemented`
- `unavailable`
- `disabled`
- `future_only`
- `non_executable`

Rules:

- candidate tool does not mean selected tool
- candidate tool does not mean authorized tool
- `implemented` does not mean executable
- `System View` may display candidate information readonly
- UI must not choose candidate tools

### Blocking and staleness rules

`ResolutionCandidate` must be `BLOCKED` or non-resolvable when:

- `ActionRequest` is `BLOCKED`
- `trace_ref` is missing
- required capability is `missing`, `disabled`, `future_only`, or `unknown`
- required domain is `forbidden` or `UNSPECIFIED` where access is needed
- `HOST` write is requested
- `EXTERNAL` is requested while closed
- `policy_status` is `blocked` or unknown-critical
- the request target is ambiguous
- inputs are missing or unsafe
- expected outputs violate `owner_ref`, manifest, or trace requirements
- a candidate tool is `unavailable` or `non_executable`
- source material is stale

Rules:

- blocking is descriptive
- no auto-fix occurs
- no execution occurs
- Lume may explain blocking
- `System View` may present blocking

Staleness metadata may include:

- `is_stale`
- `stale_reasons`

Rules:

- stale source, context, target, or policy dependencies must be reflected explicitly
- stale candidates must not progress silently

### Relationship to PendingActionCandidate

- `ResolutionCandidate` is evaluation
- `PendingActionCandidate` is future confirmation-preparation
- only a non-stale, non-blocked, reviewable `ResolutionCandidate` may become `PendingActionCandidate` in a future slice

Rules:

- `F11.2` does not create `PendingAction` runtime
- `F11.2` does not perform transition
- `F11.2` does not confirm
- `F11.2` does not authorize
- `F11.3` may define the `PendingActionCandidate` contract only
- `F11.3` still does not create transition runtime, confirmation runtime, authorization runtime, or execution runtime
- `F11.4` may define the `HumanConfirmation` contract only
- `F11.4` still does not create authorization runtime, runner, dispatcher, executor, or tool execution
- `F11.5` may define the `AuthorizedExecutionRequest` contract only
- `F11.5` still does not create runner, dispatcher, executor, `SingleToolExecution` runtime, `ToolRunManifest`, output persistence, provider invocation, or filesystem mutation
- `F11.6` may define the `SingleToolExecution` contract only
- `F11.6` still does not create runner, dispatcher, executor, tool invocation, `ToolRunManifest`, `TraceRecord`, output persistence, provider invocation, external binary invocation, or filesystem mutation
- `F11.7` may define `ToolRunManifest` and `TraceRecord` alignment only
- `F11.7` still does not create files, folders, outputs, manifests, traces, registry entries, graph entries, or `project_manifest` updates

### AuthorizedExecutionRequest

`AuthorizedExecutionRequest` is a governed post-confirmation authorization artifact.

It may only be derived from an `ACCEPTED` `HumanConfirmation` over a non-stale `PendingActionCandidate`.

It prepares a future execution request.

It does not execute anything.

Conceptual structure:

```json
{
  "authorized_execution_request_id": "aer_xxx",
  "human_confirmation_ref": "hc_xxx",
  "pending_action_candidate_ref": "pac_xxx",
  "resolution_candidate_ref": "rc_xxx",
  "action_request_ref": "ar_xxx",
  "intent_ref": "intent_xxx",
  "trace_ref": "trace_xxx",
  "authorization_status": "AUTHORIZED_PREPARED",
  "execution_scope": {
    "execution_mode": "SINGLE_TOOL_LOCAL_DETERMINISTIC",
    "tool_id": "...",
    "tool_kind": "operational | base",
    "required_capabilities": [],
    "required_domain": "SANDBOX"
  },
  "safety_snapshot": {
    "staleness_result": "FRESH",
    "risk_level": "LOW | MEDIUM | HIGH | UNKNOWN",
    "security_checked": true,
    "sandbox_checked": true,
    "capability_checked": true,
    "policy_checked": true
  },
  "owner_requirements": {
    "owner_ref_required": true,
    "owner_ref": null,
    "tool_run_manifest_required": true,
    "trace_required": true
  },
  "blocking_reasons": [],
  "created_at": "..."
}
```

### AuthorizedExecutionRequest status model

Allowed `authorization_status` values:

- `AUTHORIZED_PREPARED`
- `NOT_AUTHORIZED`
- `BLOCKED`
- `STALE`
- `SUPERSEDED`
- `EXPIRED`

Rules:

- `AUTHORIZED_PREPARED` does not execute
- `AUTHORIZED_PREPARED` does not dispatch
- `AUTHORIZED_PREPARED` does not create files
- `AUTHORIZED_PREPARED` only means the request passed declared pre-execution checks
- there is no `EXECUTING` state
- there is no `EXECUTED` state

### AuthorizedExecutionRequest eligibility rules

`AuthorizedExecutionRequest` may only be represented as `AUTHORIZED_PREPARED` when:

- `HumanConfirmation.decision` is `ACCEPTED`
- `HumanConfirmation.staleness_check.result` is `FRESH`
- `PendingActionCandidate.confirmation_readiness` is `READY`
- `PendingActionCandidate` is not stale
- `ResolutionCandidate` is `RESOLVABLE`
- `ActionRequest` is not `BLOCKED`
- `trace_ref` exists
- required capabilities are satisfied
- selected future execution scope is single-tool only
- required domain is `SANDBOX`
- `HOST` access is none or read-only where explicitly allowed by future policy
- `HOST` write is forbidden
- `EXTERNAL` access is false
- security and sanitization checks are satisfied
- `owner_ref` is required before future execution
- `tool_run_manifest` is required before future execution
- trace is required before future execution

Rules:

- eligibility is declarative only
- no runtime authorization engine is implemented
- no tool is executed
- no output is created

### Execution scope rules

Future first execution scope is:

- `SINGLE_TOOL_LOCAL_DETERMINISTIC`

Rules:

- no multi-tool orchestration
- no agent tool execution
- no LLM autonomous execution
- no provider invocation
- no external binary execution unless separately gated later
- no network access
- no `HOST` write
- first executable domain must be `SANDBOX`
- execution scope is a contract, not runtime execution

### Owner and manifest requirements

Owner requirements include:

- `owner_ref_required`
- `owner_ref`
- `tool_run_manifest_required`
- `trace_required`

Rules:

- `owner_ref` must exist before future execution
- missing `owner_ref` must block future execution
- `tool_run_manifest` is mandatory for any future execution
- trace is mandatory for any future execution
- declaring requirements does not create a manifest
- declaring requirements does not create output folders

### Relationship to SingleToolExecution

- `AuthorizedExecutionRequest` = authorized pre-execution artifact
- `SingleToolExecution` = future execution event or slice
- `ToolRunManifest` = future persisted execution artifact

Rules:

- `F11.5` does not create `SingleToolExecution`
- `F11.5` does not execute tools
- `F11.5` does not create `ToolRunManifest`
- `F11.5` does not persist outputs
- `F11.5` does not invoke providers
- `F11.5` does not mutate filesystem

### AuthorizedExecutionRequest traceability

`AuthorizedExecutionRequest` must preserve:

- `human_confirmation_ref`
- `pending_action_candidate_ref`
- `resolution_candidate_ref`
- `action_request_ref`
- `intent_ref`
- `trace_ref`
- `execution_scope`
- `safety_snapshot`
- `owner_requirements`
- `blocking_reasons`

Rules:

- refs only
- no raw payloads
- no secrets
- no private absolute host paths
- trace does not execute anything

### System View alignment for AuthorizedExecutionRequest

`System View` may present:

- `authorization_status`
- execution-scope summary
- safety-snapshot summary
- owner requirements
- blocking reasons
- `trace_ref`

Rules:

- `System View` does not create `AuthorizedExecutionRequest`
- `System View` does not authorize
- `System View` does not execute
- `System View` does not select tools
- Lume may explain authorization state but must not authorize or execute

### SingleToolExecution

`SingleToolExecution` is the future minimum governed execution artifact.

It may only follow an `AuthorizedExecutionRequest`.

It represents a single local deterministic tool execution event in a future explicit runtime slice.

In `F11.6`, it is a contract only.

It does not implement execution.

Conceptual structure:

```json
{
  "single_tool_execution_id": "ste_xxx",
  "authorized_execution_request_ref": "aer_xxx",
  "human_confirmation_ref": "hc_xxx",
  "pending_action_candidate_ref": "pac_xxx",
  "resolution_candidate_ref": "rc_xxx",
  "action_request_ref": "ar_xxx",
  "intent_ref": "intent_xxx",
  "trace_ref": "trace_xxx",
  "execution_status": "DECLARED_ONLY",
  "tool_binding": {
    "tool_id": "...",
    "tool_kind": "operational | base",
    "capabilities": [],
    "implementation_ref": "...",
    "determinism": "DETERMINISTIC"
  },
  "execution_scope": {
    "execution_mode": "SINGLE_TOOL_LOCAL_DETERMINISTIC",
    "required_domain": "SANDBOX",
    "network_access": false,
    "host_write": false,
    "external_binary": false,
    "provider_invocation": false
  },
  "input_refs": [],
  "output_plan": {
    "owner_ref": "...",
    "output_policy": "OWNER_SCOPED",
    "expected_output_refs": [],
    "tool_run_manifest_required": true,
    "trace_required": true
  },
  "safety_snapshot": {
    "staleness_result": "FRESH",
    "security_checked": true,
    "sandbox_checked": true,
    "capability_checked": true,
    "policy_checked": true
  },
  "result_placeholder": {
    "result_status": "NOT_RUN",
    "manifest_ref": null,
    "trace_record_ref": null
  },
  "created_at": "..."
}
```

### SingleToolExecution execution status model

Allowed `execution_status` values:

- `DECLARED_ONLY`
- `NOT_RUN`
- `BLOCKED`
- `STALE`
- `SUPERSEDED`
- `EXPIRED`

Rules:

- there is no `RUNNING` state in this SPEC-only slice
- there is no `SUCCEEDED` state
- there is no `FAILED` state
- there is no `PARTIAL` state
- there is no `RETRYING` state
- `NOT_RUN` is the only non-error operational placeholder
- `DECLARED_ONLY` means the contract exists but runtime is not active

### SingleToolExecution tool binding rules

Tool binding is contractual and not runtime selection.

Rules:

- `tool_id` must come from governed tool identity
- `tool_kind` is limited to:
  - `operational`
  - `base`
- agent tools are excluded
- LLM tools are excluded
- external tools are excluded unless a later explicit gate opens them
- `implementation_ref` is a declaration only
- binding does not invoke the tool
- binding does not load binaries
- binding does not inspect filesystem
- binding does not grant authority

### SingleToolExecution execution scope rules

The first future execution scope is:

- `SINGLE_TOOL_LOCAL_DETERMINISTIC`
- `SANDBOX` only
- no network
- no `HOST` write
- no provider invocation
- no external binary invocation
- no multi-tool orchestration
- no autonomous LLM tool calling
- no agent-tool cascade

Rules:

- scope is contract only
- scope does not activate runtime
- scope does not create output directories
- scope does not mutate filesystem

### SingleToolExecution input rules

`input_refs` are governed references only.

Each input ref may include:

- `input_ref`
- `input_kind`
- `source_ref`
- `order_index`
- `required`
- `sanitization_state`

Rules:

- no raw payloads
- no secrets
- no private absolute host paths
- ordered inputs must be explicit
- missing required input blocks future execution
- unsafe sanitization blocks future execution

### SingleToolExecution output plan rules

The output plan is declarative only.

Fields:

- `owner_ref`
- `output_policy`
- `expected_output_refs`
- `tool_run_manifest_required`
- `trace_required`

Allowed `output_policy` values:

- `OWNER_SCOPED`

Rules:

- `owner_ref` is mandatory before future execution
- outputs must be owner-scoped
- project-root `outputs/` is forbidden
- output plan does not create files
- output plan does not create folders
- output plan does not create manifest
- output plan does not persist trace
- `ToolRunManifest` remains a future artifact unless separately opened

### SingleToolExecution safety snapshot rules

`SingleToolExecution` must preserve the safety snapshot from `AuthorizedExecutionRequest`.

Required checks:

- `staleness_result`
- `security_checked`
- `sandbox_checked`
- `capability_checked`
- `policy_checked`

Rules:

- staleness must be `FRESH` before any future runtime execution
- security must be satisfied
- sandbox must be satisfied
- capability must be satisfied
- policy must be satisfied
- SPEC-only declaration does not perform these checks

### SingleToolExecution result placeholder

Result placeholder fields:

- `result_status`
- `manifest_ref`
- `trace_record_ref`

Allowed `result_status` values in this SPEC-only slice:

- `NOT_RUN`
- `BLOCKED`
- `STALE`

Rules:

- no real result is produced
- `manifest_ref` must remain `null` unless future manifest persistence is opened
- `trace_record_ref` must remain `null` unless future trace persistence is opened
- no outputs are materialized

### Relationship to ToolRunManifest and TraceRecord

- `SingleToolExecution` = future execution event contract
- `ToolRunManifest` = future persisted run artifact
- `TraceRecord` = future persisted trace artifact

Rules:

- `F11.6` does not create `ToolRunManifest`
- `F11.6` does not create `TraceRecord`
- `F11.6` does not persist outputs
- any future runtime execution must produce `ToolRunManifest` and `TraceRecord`
- manifest and trace requirements are mandatory but inactive here

### ToolRunManifest contract alignment

Future `ToolRunManifest` fields may include:

```json
{
  "tool_run_manifest_id": "trm_xxx",
  "single_tool_execution_ref": "ste_xxx",
  "authorized_execution_request_ref": "aer_xxx",
  "human_confirmation_ref": "hc_xxx",
  "action_request_ref": "ar_xxx",
  "intent_ref": "intent_xxx",
  "trace_ref": "trace_xxx",
  "owner_ref": "...",
  "tool": {
    "tool_id": "...",
    "tool_kind": "operational | base",
    "tool_version": "...",
    "implementation_ref": "..."
  },
  "inputs": [],
  "configuration": {},
  "outputs": [],
  "status": "DECLARED_ONLY",
  "started_at": null,
  "finished_at": null,
  "error": null,
  "warnings": [],
  "created_at": "..."
}
```

Rules:

- `ToolRunManifest` is mandatory for future runtime execution
- `F11.7` does not create `tool_run_manifest.json`
- `owner_ref` is mandatory before future execution
- inputs must be refs, not raw payloads
- outputs must be owner-scoped
- configuration must be separate from inputs
- no secrets
- no private absolute host paths
- no project-root `outputs/`

### ToolRunManifest status model

Allowed `ToolRunManifest.status` values in `F11.7`:

- `DECLARED_ONLY`
- `NOT_CREATED`
- `FUTURE_REQUIRED`

Rules:

- there is no `RUNNING` status
- there is no `SUCCEEDED` status
- there is no `FAILED` status
- there is no `PARTIAL` status
- there is no `RETRYING` status
- runtime lifecycle statuses belong only to future runtime slices

### TraceRecord contract alignment

`docs/specs/system_observability.md` is the owner of `TraceRecord` semantics.

`action_resolution.md` references `TraceRecord` only as a downstream required future artifact of `SingleToolExecution`.

Any `TraceRecord` shape shown here is alignment-only and non-authoritative if it conflicts with `docs/specs/system_observability.md`.

Future `TraceRecord` fields may include:

```json
{
  "trace_record_id": "trrec_xxx",
  "trace_ref": "trace_xxx",
  "single_tool_execution_ref": "ste_xxx",
  "authorized_execution_request_ref": "aer_xxx",
  "action_request_ref": "ar_xxx",
  "event_kind": "SINGLE_TOOL_EXECUTION_DECLARED",
  "scope": "SINGLE_INTENT | MULTI_STEP_FLOW | CROSS_ARTIFACT",
  "status": "DECLARED_ONLY",
  "links": [],
  "sanitized_summary_key": "...",
  "created_at": "..."
}
```

Rules:

- `TraceRecord` is mandatory for future runtime execution
- `F11.7` does not persist `TraceRecord`
- trace links by reference only
- no raw payloads
- no secrets
- no private absolute host paths
- trace does not authorize execution
- `TraceRecord` remains declared-only, non-running, non-executing, and non-persisted in this phase

### TraceRecord status model

Allowed `TraceRecord.status` values in `F11.7`:

- `DECLARED_ONLY`
- `NOT_PERSISTED`
- `FUTURE_REQUIRED`

Rules:

- no runtime event emission is activated
- no execution result is activated
- no automatic trace persistence is activated
- operational trace statuses belong only to future runtime slices

### Manifest and trace linkage rules

Required linkage:

- `ToolRunManifest.single_tool_execution_ref -> SingleToolExecution`
- `ToolRunManifest.trace_ref -> trace_ref`
- `TraceRecord.single_tool_execution_ref -> SingleToolExecution`
- `TraceRecord.authorized_execution_request_ref -> AuthorizedExecutionRequest`
- `TraceRecord.trace_ref -> trace_ref`

Rules:

- manifest and trace must agree on `trace_ref`
- manifest and trace must refer to the same `SingleToolExecution`
- missing `owner_ref` blocks future execution
- missing `trace_ref` blocks future execution
- missing manifest blocks future execution
- linkage rules are declarative only in `F11.7`

### System View alignment for SingleToolExecution

`System View` may present:

- `SingleToolExecution` declared-only status
- tool-binding summary
- execution-scope summary
- output-plan summary
- safety-snapshot summary
- result placeholder
- `trace_ref`

Rules:

- `System View` does not create `SingleToolExecution`
- `System View` does not execute
- `System View` does not dispatch
- `System View` does not select tools
- `System View` does not create outputs
- Lume may explain why execution is still not active

### System View alignment for ToolRunManifest and TraceRecord

`System View` may present:

- manifest required state
- trace required state
- missing manifest or trace warnings
- `owner_ref` readiness
- declared-only status

Rules:

- `System View` does not create manifests
- `System View` does not persist traces
- `System View` does not execute
- `System View` does not authorize
- Lume may explain missing manifest or trace requirements only

### Traceability

`ResolutionCandidate` must preserve:

- `action_request_ref`
- `intent_ref`
- `trace_ref`
- capability evaluations
- domain evaluations
- policy evaluations
- candidate-tool explanations
- blocking reasons
- risk metadata
- staleness metadata

Rules:

- refs only
- no raw payloads
- no secrets
- no private absolute host paths

### System View alignment

`System View` may present:

- `resolution_state`
- `blocking_reasons`
- capability-evaluation summary
- domain-evaluation summary
- policy-evaluation summary
- candidate tools as non-authoritative
- risk and staleness summary

Rules:

- `System View` does not compute `ResolutionCandidate`
- `System View` does not select tools
- `System View` does not authorize
- `System View` does not execute

## Base model

The conceptual chain is:

`action -> risk -> policy -> resolution -> confirmation -> authorized executor -> trace`

The authorized executor is future F10+ only.

## Minimal components

### ActionRequest

A conceptual `ActionRequest` includes:

- `action_id`
- `origin`
- `action_class`
- `targets`
- `parameters`
- `context_refs`

Future action shapes may also include capability requirements before candidate tool selection.

Future action shapes may also include declarative domain constraints such as required domain, host access, and network access before any candidate tool selection.

Those future fields align with `ActionIntent` and `ActionRequest` and remain declarative only in the current phase.

Allowed origins:

- `user`: explicit user intent
- `llm`: proposal only
- `system`: internal derivation

Forbidden origins:

- UI directly
- chat directly
- autonomous tools

Future template management actions may be represented as `ActionRequest` values:

- create template
- rename template
- edit template
- delete template
- clone template

These actions are declarative in F9. They do not imply a runner, filesystem mutation, UI authority, or template runtime.

They may originate from explicit user intent, an LLM proposal accepted by the user, or a future Tools Panel entrypoint such as `TemplateDesignerTool`. They must not originate from UI state alone, chat state alone, or autonomous tools.

Deleting a user template requires confirmation.

Editing, renaming, or deleting a system template must produce `BLOCKED`.

Cloning a system template may be proposed as an alternative governed action, but it still requires normal policy evaluation.

Future governed LLM decomposition may classify a subrequest as `tool_candidate`.

That classification does not execute anything.

It may only produce a proposal-level action candidate that later becomes an `ActionRequest` through governed system policy.

### ContextResolution

Context resolution determines whether targets and references are specific, current, and safe to evaluate.

Ambiguous targets must produce confirmation or blocking. They must not silently resolve.

### RiskClassification

Risk classification maps the request to the risk levels declared in `action_policy`.

### PolicyEvaluation

Policy evaluation must happen before any future execution.

It produces a policy decision compatible with:

- `allow`
- `confirm`
- `reject`

Forbidden policy outcomes become `BLOCKED`, not ordinary `REJECTED`.

### ResolutionDecision

Resolution combines context, risk, and policy into one result.

### Interaction

Interaction determines whether the user sees an inline confirmation, modal confirmation, rejection, blocked state, stale state, or expired state.

### Trace

Every resolution produces minimum trace data.

## Resolution results

### Legacy vocabulary clarification

The following `ALLOW`, `CONFIRM_REQUIRED`, `REJECTED`, `BLOCKED`, `STALE`, `EXPIRED`, and conceptual-state terms are legacy/base conceptual vocabulary.

This block is legacy/base conceptual vocabulary only.

It is non-authoritative for:

- the `F11.1` to `F11.8` artifact chain
- `action_core` Rust type design
- future runtime state design

They do not override the F11.1-F11.5 artifact chain:

`ActionRequest -> ResolutionCandidate -> PendingActionCandidate -> HumanConfirmation -> AuthorizedExecutionRequest`

Canonical downstream chain for current F11 governance is:

`ActionRequest -> ResolutionCandidate -> PendingActionCandidate -> HumanConfirmation -> AuthorizedExecutionRequest -> SingleToolExecution -> future ToolRunManifest -> future TraceRecord`

Any execution-related words in this older section, including `Approved`, `Executed`, `Failed`, executor, or execution references, are future-only and non-authoritative.

Canonical pre-execution terms for the F11.1-F11.5 chain are:

- `READY_FOR_RESOLUTION`
- `RESOLVABLE`
- `READY`
- `ACCEPTED`
- `AUTHORIZED_PREPARED`
- `DECLARED_ONLY`
- `NOT_RUN`
- `NOT_CREATED`
- `NOT_PERSISTED`
- `FUTURE_REQUIRED`

`AUTHORIZED_PREPARED` does not mean dispatched, executing, executed, or output-producing.

Legacy/base terms in this section must not be used to introduce:

- runner
- dispatcher
- executor
- runtime transition
- execution status
- manifest persistence
- trace persistence

### ALLOW

`ALLOW` means the action is permitted by policy and does not require confirmation.

`ALLOW` does not mean invisible execution.

It must produce visible representation and trace.

The word `silent`, if ever used in future implementation notes, may only mean "without interruption". It must never mean "without representation or trace".

### CONFIRM_REQUIRED

`CONFIRM_REQUIRED` means the action is allowed only after human confirmation.

In F10.9 it may only generate an inert pending candidate linked to `action_id`.

### REJECTED

`REJECTED` means the action is not allowed by ordinary policy while that policy applies.

It is not executable under the current policy.

### BLOCKED

`BLOCKED` means the action cannot be resolved because of system state or safety constraints.

Examples:

- invalid target
- inconsistent context
- broken context reference
- forbidden action
- missing resource
- incomplete system capability

`BLOCKED` must not be confused with ordinary policy rejection.

### STALE

`STALE` means a pending action is no longer safe because something relevant changed.

Examples:

- context changed
- target changed
- active document changed
- artifact version changed
- applicable policy changed

### EXPIRED

`EXPIRED` means a pending action expired due to time, flow lifecycle, phase transition, or operational context change.

## Conceptual states

- `Draft`
- `Resolved`
- `PendingConfirmation`
- `Approved`
- `Rejected`
- `Blocked`
- `Stale`
- `Expired`
- `Executed` future F10+ only
- `Failed` future F10+ only

## Invariants

- `INV-RES-001`: every `ActionRequest` must have controlled origin.
- `INV-RES-002`: no action resolves over an ambiguous target without confirmation or blocking.
- `INV-RES-003`: policy evaluation precedes any future execution.
- `INV-RES-004`: `ALLOW` requires visible representation and trace; it does not require confirmation.
- `INV-RES-005`: forbidden produces `BLOCKED`, not ordinary `REJECTED`.
- `INV-RES-006`: `PendingConfirmation` must expire or become stale if context, target, artifact, version, or policy changes.
- `INV-RES-007`: human confirmation does not revive `Stale` or `Expired` actions; a new `ActionRequest` must be regenerated.
- `INV-RES-008`: every resolution must produce minimum trace.
- `INV-RES-009`: the first F10 opening gate MUST NOT be interpreted as opening `ActionResolution` runner or execution authority.
- `INV-RES-010`: every proposal-to-resolution transition MUST preserve origin, target refs, context refs, policy refs, and trace refs.
- `INV-RES-011`: ambiguous targets MUST produce `BLOCKED` or `CONFIRM_REQUIRED`, never silent resolution.
- `INV-RES-012`: forbidden actions MUST produce `BLOCKED`.
- `INV-RES-013`: stale drafts MUST NOT be confirmed.
- `INV-RES-014`: user confirmation in F10.9 may approve only a pending candidate; it must not execute it.
- `INV-RES-015`: execution remains future and separately gated.
- `INV-RES-016`: `ResolutionCandidate` is evaluated but not executable.
- `INV-RES-017`: `PendingActionCandidate` is confirmable but not executable.
- `INV-RES-018`: `HumanConfirmation` is approval signal only and does not execute in `F11.0`.
- `INV-RES-019`: `AuthorizedExecutionRequest` is future-only and not active in `F11.0`.
- `INV-RES-020`: `SingleToolExecution` is future-only and not active in `F11.0`.
- `INV-RESCAND-001`: `ResolutionCandidate` is inert and non-executing.
- `INV-RESCAND-002`: `ResolutionCandidate` does not authorize execution.
- `INV-RESCAND-003`: `RESOLVABLE` does not mean executable.
- `INV-RESCAND-004`: `candidate_tools` are explanatory and not selected.
- `INV-RESCAND-005`: capability evaluation does not grant permissions.
- `INV-RESCAND-006`: domain evaluation does not grant access.
- `INV-RESCAND-007`: blocked or stale candidates must not progress silently.
- `INV-RESCAND-008`: `ResolutionCandidate` must preserve `action_request_ref` and `trace_ref`.
- `INV-RESCAND-009`: `System View` may present `ResolutionCandidate` but must not control it.
- `INV-RESCAND-010`: `F11.2` does not create runner, dispatcher, executor, or `PendingAction` runtime.
- `INV-RESCAND-011`: `project_runtime` remains unchanged.
- `INV-RESCAND-012`: execution boundary remains `CLOSED`.
- `INV-HCONF-001`: `HumanConfirmation` is an explicit human decision event.
- `INV-HCONF-002`: `HumanConfirmation` is non-executing.
- `INV-HCONF-003`: `ACCEPTED` does not mean authorized or executed.
- `INV-HCONF-004`: `HumanConfirmation` must preserve `pending_action_candidate_ref` and `trace_ref`.
- `INV-HCONF-005`: accepted confirmation requires non-stale candidate evidence.
- `INV-HCONF-006`: `UNKNOWN` staleness must not silently progress to authorization.
- `INV-HCONF-007`: risk acknowledgement does not bypass policy.
- `INV-HCONF-008`: `reviewer_ref` is not credential authority.
- `INV-HCONF-009`: `System View` may present `HumanConfirmation` but must not create or control it.
- `INV-HCONF-010`: `F11.4` does not create authorization runtime, runner, dispatcher, executor, or tool execution.
- `INV-HCONF-011`: `project_runtime` remains unchanged.
- `INV-HCONF-012`: execution boundary remains `CLOSED`.
- `INV-AER-001`: `AuthorizedExecutionRequest` is a post-confirmation authorization artifact, not execution.
- `INV-AER-002`: `AUTHORIZED_PREPARED` does not mean executing or executed.
- `INV-AER-003`: `AuthorizedExecutionRequest` requires `ACCEPTED` `HumanConfirmation`.
- `INV-AER-004`: `AuthorizedExecutionRequest` requires `FRESH` staleness result.
- `INV-AER-005`: first future execution scope is single-tool, local, deterministic, and `SANDBOX`-scoped.
- `INV-AER-006`: `owner_ref`, `tool_run_manifest`, and trace are mandatory before future execution.
- `INV-AER-007`: `AuthorizedExecutionRequest` must not create files, folders, manifests, or outputs.
- `INV-AER-008`: `AuthorizedExecutionRequest` must not call providers or tools.
- `INV-AER-009`: `HOST` write and `EXTERNAL` access remain closed.
- `INV-AER-010`: `System View` may present `AuthorizedExecutionRequest` but must not control it.
- `INV-AER-011`: `F11.5` does not create runner, dispatcher, executor, or `SingleToolExecution` runtime.
- `INV-AER-012`: `project_runtime` remains unchanged.
- `INV-AER-013`: execution boundary remains `CLOSED` until a later explicit execution slice.
- `INV-STE-001`: `SingleToolExecution` contract is declared-only in `F11.6`.
- `INV-STE-002`: `SingleToolExecution` does not implement runner, dispatcher, executor, or tool invocation.
- `INV-STE-003`: `SingleToolExecution` must derive from `AuthorizedExecutionRequest`.
- `INV-STE-004`: `SingleToolExecution` is limited to one local deterministic tool.
- `INV-STE-005`: first future execution scope is `SANDBOX`-only.
- `INV-STE-006`: `HOST` write, `EXTERNAL` access, provider invocation, external binary invocation, agent execution, and multi-tool orchestration remain closed.
- `INV-STE-007`: input refs must be governed refs, not raw payloads.
- `INV-STE-008`: output plan must be owner-scoped and must not create files in this slice.
- `INV-STE-009`: `ToolRunManifest` and `TraceRecord` are mandatory for future runtime execution but are not produced in `F11.6`.
- `INV-STE-010`: `result_status` remains `NOT_RUN` unless a later explicit runtime slice opens execution.
- `INV-STE-011`: `System View` may present `SingleToolExecution` but must not control or execute it.
- `INV-STE-012`: `project_runtime` remains unchanged.
- `INV-STE-013`: execution boundary remains `CLOSED` after this SPEC-only slice.
- `INV-TRM-001`: `ToolRunManifest` is mandatory for future `SingleToolExecution` runtime.
- `INV-TRM-002`: `F11.7` does not create `tool_run_manifest.json`.
- `INV-TRM-003`: `ToolRunManifest` must link to `SingleToolExecution`, `AuthorizedExecutionRequest`, `ActionRequest`, `intent`, `owner_ref`, and `trace_ref`.
- `INV-TRM-004`: `ToolRunManifest` inputs are governed refs, not raw payloads.
- `INV-TRM-005`: `ToolRunManifest` outputs must be owner-scoped.
- `INV-TRM-006`: `ToolRunManifest` must not contain secrets or private absolute host paths.
- `INV-TRM-007`: `ToolRunManifest` configuration is separate from inputs.
- `INV-TRM-008`: `ToolRunManifest` status remains non-operational in `F11.7`.
- `INV-TRREC-001`: `TraceRecord` is mandatory for future `SingleToolExecution` runtime.
- `INV-TRREC-002`: `F11.7` does not persist `TraceRecord`.
- `INV-TRREC-003`: `TraceRecord` links artifacts by reference only.
- `INV-TRREC-004`: `TraceRecord` must not contain secrets, raw payloads, or private absolute host paths.
- `INV-TRREC-005`: `TraceRecord` does not authorize execution.
- `INV-TRREC-006`: `TraceRecord` status remains non-operational in `F11.7`.
- `INV-MANIFEST-TRACE-001`: `ToolRunManifest` and `TraceRecord` must agree on `trace_ref`.
- `INV-MANIFEST-TRACE-002`: `ToolRunManifest` and `TraceRecord` must reference the same `SingleToolExecution`.
- `INV-MANIFEST-TRACE-003`: missing `owner_ref`, `trace_ref`, or manifest blocks future execution.
- `INV-MANIFEST-TRACE-004`: `F11.7` creates no files, folders, outputs, manifests, traces, registry entries, graph entries, or `project_manifest` updates.
- `INV-RUNTIME-OPEN-001`: `F12.0 / F11.RUNTIME-0` is proposal-only.
- `INV-RUNTIME-OPEN-002`: minimal runtime remains closed until a later explicit implementation slice.
- `INV-RUNTIME-OPEN-003`: first future runtime scope is one local deterministic `SANDBOX`-only tool.
- `INV-RUNTIME-OPEN-004`: first eligible tool kind is `operational` or `base` only.
- `INV-RUNTIME-OPEN-005`: LLM, agent, external, provider, network, external binary, and multi-tool execution remain closed.
- `INV-RUNTIME-OPEN-006`: `text.measure` is the recommended first future tool because it is deterministic and low risk.
- `INV-RUNTIME-OPEN-007`: `owner_ref`, `trace_ref`, `ToolRunManifest`, and `TraceRecord` are mandatory for future execution.
- `INV-RUNTIME-OPEN-008`: UI and `System View` must not become execution authority.
- `INV-RUNTIME-OPEN-009`: `project_runtime` must not be expanded by convenience.
- `INV-RUNTIME-OPEN-010`: this proposal creates no runner, dispatcher, executor, outputs, manifests, or traces.
- `INV-RUNTIME-GATE-001`: `F12.1` is gate-only.
- `INV-RUNTIME-GATE-002`: runtime remains closed after `F12.1`.
- `INV-RUNTIME-GATE-003`: only `text.measure` may be considered for first implementation.
- `INV-RUNTIME-GATE-004`: `owner_ref` and `trace_ref` are mandatory.
- `INV-RUNTIME-GATE-005`: `ToolRunManifest` and `TraceRecord` are mandatory future artifacts but not created in `F12.1`.
- `INV-RUNTIME-GATE-006`: `project_runtime` must not be modified for the first implementation unless a later explicit audit justifies it.
- `INV-RUNTIME-GATE-007`: `tool_runtime` must not become a general dispatcher.
- `INV-RUNTIME-GATE-008`: UI and `System View` must not execute, authorize, or dispatch.
- `INV-RUNTIME-IMPL-PLAN-001`: `F12.2A` is plan-only.
- `INV-RUNTIME-IMPL-PLAN-002`: `F12.2A` introduces no runtime.
- `INV-RUNTIME-IMPL-PLAN-003`: `F12.2B` may touch only `tool_runtime` unless explicitly re-audited.
- `INV-RUNTIME-IMPL-PLAN-004`: `F12.2B` must implement only `text.measure`.
- `INV-RUNTIME-IMPL-PLAN-005`: `F12.2B` must not create a general dispatcher.
- `INV-RUNTIME-IMPL-PLAN-006`: `owner_ref` and `trace_ref` are mandatory.
- `INV-RUNTIME-IMPL-PLAN-007`: outputs must be owner-scoped.
- `INV-RUNTIME-IMPL-PLAN-008`: `audit_f12_minimal_runtime_boundary` is mandatory before closure.

## Minimum trace payload

- `action_id`
- `origin`
- `action_class`
- `target_refs`
- `risk_level`
- `policy_decision`
- `resolution_result`
- `confirmation_required`
- `user_confirmation_ref`, when applicable
- `executor_authority`, future
- `timestamp`
- `context_version` or `context_fingerprint`, conceptual
- `policy_version` or `policy_ref`

## Integration points

Operational tools may only prepare future `ActionRequest` values through governed UI intent capture.

Future action matching may reference `CapabilityRequirement` before resolving candidate tools, but that matching remains non-executing and non-authorizing in the current phase.

Future action matching may also carry sandbox, host, or external-domain constraints, but those constraints remain declarative and non-enforcing in the current phase.

Future readonly presentation may summarize `ActionRequest`, resolution, and blocking states through `docs/specs/system_view.md` without creating execution authority.

LLM tools may only be requested by future governed LLM policy flow and must still resolve through system action governance.

External dependencies are never directly executed from their inspection panel.

Tools panels and popups emit intent only.

Future `IntentAnalysis`, `RequestDecomposition`, and `SubRequestPlan[]` artifacts may mark:

- `may_require_action_resolution`
- `requires_action_resolution`

Those markers are declarative only.

They do not create a runner, dispatcher, executor, or mutation path.

In F10.9 they may contribute only to inert resolution-candidate preparation.

This spec integrates with:

- `docs/specs/action_contract.md`
- `docs/specs/action_policy.md`
- `docs/specs/flow_control_policy.md`
- `docs/specs/pending_action_state.md`
- `docs/specs/how_to_add_a_tool.md`
- `docs/specs/tool_capability_model.md`
- `docs/specs/sandbox_boundary_model.md`
- `docs/specs/system_observability.md`
- `docs/specs/system_view.md`
- `governance/GOVERNANCE.md`
- `governance/WORKSPACE_RULES.md`
- `governance/LLM_RUNTIME_POLICY.md`

## Forbidden responsibilities

This spec must not:

- create a runner
- create `document_patch_runtime`
- implement execution
- implement operational `ActionRequest`
- touch `project_runtime`
- widen `ProjectRuntimeOutput`
- place logic in `ui_slint`
- execute tools
- connect LLM providers
- download binaries
- mutate external runtime folders
- move or merge tool catalogs

This spec must not treat decomposition, planning, or synthesized responses as execution authority.

It is declarative F9 governance only.
