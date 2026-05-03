# action_contract

## Status

DECLARED_ONLY / F10-F11_PREP governance.

This document does not implement runtime execution, authorization, tool dispatch, provider invocation, filesystem mutation, `ActionResolution` runner behavior, or `project_runtime` authority.

## Purpose

Define `ActionIntent` as the first governed action contract connecting:

- `ToolUseProposal`
- `ActionRequestDraft`
- user, system, or LLM source context

`ActionIntent` is declarative, traceable, and non-executing.

It expresses intended work, required capabilities, sandbox or domain constraints, and traceability.

It does not execute anything.

This document also defines `ActionRequest` as the first governed post-`F11.0` rich action-request contract.

`ActionRequest` is richer than `ActionIntent`.

It remains declarative, traceable, reviewable, and non-executing.

`ResolutionCandidate` remains the next inert evaluation artifact after `ActionRequest` and is owned by `docs/specs/action_resolution.md`.

## ActionIntent definition

Conceptual structure:

```json
{
  "intent_id": "intent_xxx",
  "source": {
    "source_kind": "user | llm | system",
    "source_ref": "..."
  },
  "proposal_ref": "...",
  "draft_ref": "...",
  "target": {
    "target_kind": "stored_object | file_ref | artifact | chat | tool_output | project | unknown",
    "target_ref": "..."
  },
  "capabilities_required": [],
  "constraints": {
    "required_domain": "SANDBOX | HOST | EXTERNAL | UNSPECIFIED",
    "access_level": "read | write | create | delete | network | execute | none",
    "sandbox_scope": "project_only | file_store | user_output | unspecified",
    "host_access": "none | read_only | forbidden",
    "network_access": false
  },
  "status": "CREATED",
  "trace_ref": "trace_xxx"
}
```

Rules:

- `ActionIntent` is declarative only
- `ActionIntent` does not execute
- `ActionIntent` does not authorize
- `ActionIntent` does not select a tool for execution
- `ActionIntent` does not mutate state

## Relationship model

The relationship is:

- `ToolUseProposal` suggests possible tool or capability use
- `ActionRequestDraft` shapes a possible action request
- `ActionIntent` formalizes intended action
- `ActionRequest` formalizes a richer reviewable request

Rules:

- `ToolUseProposal` remains proposal-only
- `ActionRequestDraft` remains inert
- `ActionIntent` is a governed bridge, not execution
- `ActionIntent` is not `ActionResolution`
- `ActionIntent` is not `PendingAction`
- `ActionIntent` is not `AuthorizedExecutionRequest`

## F11.1 ActionRequest opening note

`F11.1` opens only the rich `ActionRequest` contract.

It does not open execution.

The non-executing chain is:

`ToolUseProposal / ActionRequestDraft -> ActionIntent -> ActionRequest -> ResolutionCandidate -> PendingActionCandidate -> HumanConfirmation -> AuthorizedExecutionRequest -> SingleToolExecution`

Rules:

- `F11.1` opens only the rich `ActionRequest` contract
- `HumanConfirmation` is a downstream non-executing decision event
- `AuthorizedExecutionRequest` is a downstream post-confirmation artifact
- `SingleToolExecution` is a downstream declared-only artifact in `F11.6`
- `ActionRequest` is never executable by itself
- execution boundary remains `CLOSED`

## ActionRequest definition

Conceptual structure:

```json
{
  "action_request_id": "ar_xxx",
  "intent_ref": "intent_xxx",
  "source": {
    "source_kind": "user | llm | system",
    "source_ref": "..."
  },
  "request_kind": "tool_use | document_operation | semantic_operation | filesystem_operation | review_operation | unknown",
  "target": {
    "target_kind": "stored_object | file_ref | artifact | chat | tool_output | project | semantic_quadset | unknown",
    "target_ref": "..."
  },
  "capability_requirements": [],
  "domain_constraints": {
    "required_domain": "SANDBOX | HOST | EXTERNAL | UNSPECIFIED",
    "access_level": "read | write | create | delete | network | none",
    "sandbox_scope": "project_only | file_store | user_output | unspecified",
    "host_access": "none | read_only | forbidden",
    "network_access": false
  },
  "policy_context": {
    "security_policy_ref": "...",
    "sandbox_policy_ref": "...",
    "tool_policy_ref": "...",
    "llm_policy_ref": "...",
    "project_policy_ref": "..."
  },
  "inputs": [],
  "expected_outputs": [],
  "risk": {
    "risk_level": "LOW | MEDIUM | HIGH | UNKNOWN",
    "risk_reasons": []
  },
  "status": "DRAFTED",
  "blocking_reasons": [],
  "trace_ref": "trace_xxx",
  "created_at": "..."
}
```

Rules:

- `ActionRequest` is declarative only
- `ActionRequest` does not execute
- `ActionRequest` does not authorize
- `ActionRequest` does not select a tool for execution
- `ActionRequest` does not mutate state
- `ActionRequest` is richer than `ActionIntent` but remains non-executable

## Relationship to ActionIntent

- `ActionIntent` = intended work
- `ActionRequest` = formal reviewable request
- `ResolutionCandidate` = inert evaluation candidate
- `PendingActionCandidate` = future confirmation-preparation state

Rules:

- `ActionRequest` may derive from `ActionIntent`
- `ActionRequest` must preserve `intent_ref`
- `ActionRequest` must preserve `trace_ref`
- `ActionRequest` may feed `ResolutionCandidate` preparation only
- `PendingActionCandidate` remains downstream from `ResolutionCandidate` only
- `HumanConfirmation` remains downstream from `PendingActionCandidate` only
- `AuthorizedExecutionRequest` remains downstream from `HumanConfirmation` only
- `ActionRequest` is not `ActionResolution`
- `ActionRequest` is not `PendingActionCandidate`
- `ActionRequest` is not `AuthorizedExecutionRequest`

## ActionRequest status model

Allowed `ActionRequest` status values:

- `DRAFTED`
- `NEEDS_CONTEXT`
- `BLOCKED`
- `READY_FOR_RESOLUTION`
- `SUPERSEDED`
- `EXPIRED`

Rules:

- there is no `AUTHORIZED` status in `F11.1`
- there is no `EXECUTING` status
- there is no `EXECUTED` status
- `READY_FOR_RESOLUTION` means eligible for inert resolution preparation only
- `BLOCKED` is explanatory, not failure
- status does not trigger runtime work

## ActionRequest request-kind model

Controlled `request_kind` values:

- `tool_use`
- `document_operation`
- `semantic_operation`
- `filesystem_operation`
- `review_operation`
- `unknown`

Rules:

- `request_kind` classifies intent
- `request_kind` does not select a tool
- `request_kind` does not authorize execution
- `filesystem_operation` remains non-executing
- `semantic_operation` does not generate or approve quads
- `document_operation` does not mutate documents

## ActionRequest capability requirements

`ActionRequest` must use capability requirements before any future tool candidate selection.

Rules:

- `capability_requirements` align with `docs/specs/tool_capability_model.md`
- capabilities are requirements, not grants
- capability matching does not authorize
- tools remain future candidate providers only
- disabled or future capabilities block request progression

## ActionRequest domain constraints

`ActionRequest` must carry domain constraints aligned with `docs/specs/sandbox_boundary_model.md`.

Rules:

- `required_domain` uses canonical uppercase enum:
  - `SANDBOX`
  - `HOST`
  - `EXTERNAL`
  - `UNSPECIFIED`
- `UNSPECIFIED` must block progression when access is required
- `HOST` write remains forbidden
- `EXTERNAL` remains disabled unless a future explicit gate opens it
- `SANDBOX` write remains future-governed and not active here

## ActionRequest inputs and expected outputs

`inputs` are reference-only input descriptors.

Conceptual input fields:

- `input_ref`
- `input_kind`
- `source_ref`
- `required`
- `order_index`
- `sanitization_state`

`expected_outputs` are non-materialized expectations.

Conceptual expected output fields:

- `output_kind`
- `owner_ref_required`
- `expected_location_policy`
- `manifest_required`
- `trace_required`

Rules:

- inputs are refs, not raw payloads
- no private absolute host paths
- expected outputs do not create files
- `owner_ref` may be required for future execution but is not enforced here
- `manifest_required` is declarative only

## ActionRequest risk model

`ActionRequest` may include readonly risk preparation.

Conceptual risk fields:

- `risk_level`
- `risk_reasons`

Allowed `risk_level` values:

- `LOW`
- `MEDIUM`
- `HIGH`
- `UNKNOWN`

Rules:

- risk is descriptive only
- risk is not authorization
- risk does not trigger runtime work

## ActionRequest blocking rules

`ActionRequest` must be `BLOCKED` when:

- `intent_ref` is missing
- `trace_ref` is missing
- source is ambiguous
- target is ambiguous
- required capability is unknown
- required capability is disabled or `future_only`
- required domain is `UNSPECIFIED` and access is required
- `HOST` write is requested
- `EXTERNAL` access is requested without future gate
- security or sanitization policy would be violated
- expected output lacks future `owner_ref` requirement
- `request_kind` is `unknown` and cannot be reviewed safely

Rules:

- `BLOCKED` does not auto-fix
- `BLOCKED` does not execute
- `BLOCKED` may be shown in `System View`
- Lume may explain but not resolve

## ActionRequest traceability

`ActionRequest` must link to:

- source
- `intent_ref`
- `proposal_ref`, if available through intent
- `draft_ref`, if available through intent
- target refs
- capability requirements
- domain constraints
- policy refs
- `trace_ref`

Rules:

- trace links by reference only
- no raw payloads
- no secrets
- trace does not authorize execution

## Relationship to System View

`System View` may present summaries of `ActionRequest`:

- status
- blocking reasons
- capabilities required
- domain constraints
- `trace_ref`
- risk level

Rules:

- `System View` is readonly
- `System View` does not create or update `ActionRequest`
- `System View` does not authorize or execute `ActionRequest`
- visible labels must use future i18n keys

## Capability requirements

`ActionIntent` must reference capabilities, not direct execution grants.

Rules:

- `capabilities_required` must align with `docs/specs/tool_capability_model.md`
- tools are only future candidate providers
- capability requirement does not authorize execution
- capability requirement does not imply tool selection
- disabled or future capabilities remain unavailable for execution

## Sandbox and domain constraints

`ActionIntent` must declare domain and access constraints aligned with `docs/specs/sandbox_boundary_model.md`.

Rules:

- domain declaration does not authorize access
- `UNSPECIFIED` domain must not imply permission
- `HOST` access remains read-only future and controlled at most
- `HOST` write is forbidden by default
- `EXTERNAL` remains disabled unless separately gated in the future
- `SANDBOX` write remains future-governed and not active now

## Status model

`ActionIntent` status values:

- `CREATED`
- `NEEDS_CONTEXT`
- `BLOCKED`
- `READY_FOR_REVIEW`
- `EXPIRED`
- `SUPERSEDED`

Rules:

- only `CREATED`, `NEEDS_CONTEXT`, and `BLOCKED` should be considered active descriptive states in this phase
- there is no `AUTHORIZED` state in this phase
- there is no `EXECUTING` state
- there is no `EXECUTED` state
- `READY_FOR_REVIEW` is review preparation only, not authorization

## Blocking rules

`ActionIntent` must become or remain `BLOCKED` when:

- target is ambiguous
- required capability is unknown
- required capability is disabled or future-only
- required domain is unspecified where access is required
- `HOST` write is requested
- `EXTERNAL` access is requested without a future gate
- source, proposal, draft, or trace is missing
- security or sanitization policy would be violated

Rules:

- `BLOCKED` does not execute
- `BLOCKED` does not auto-fix
- `BLOCKED` requires future human or governed review

## Traceability

`ActionIntent` must preserve references to:

- source
- `ToolUseProposal`, when present
- `ActionRequestDraft`, when present
- target refs
- capability requirements
- sandbox constraints
- `trace_ref`

Rules:

- traceability must not duplicate secrets
- trace refs do not authorize execution
- missing traceability should block future progression
- trace structure and status ownership are governed by `docs/specs/system_observability.md`

## Future compatibility

`ActionIntent` may feed:

- future `ActionPlan`
- future `ActionRequest`
- future `ActionResolution`
- future capability, sandbox, security, or cost validation
- future UI presentation as reviewable intent

But:

- no runner is created
- no execution is enabled
- no authorization is performed
- no provider invocation is enabled
- no tool dispatch is enabled

## Action invariants

- `INV-ACTION-001`: `ActionIntent` is declarative and non-executing.
- `INV-ACTION-002`: `ActionIntent` does not authorize tools, providers, filesystem mutation, or network access.
- `INV-ACTION-003`: `ToolUseProposal` remains proposal-only.
- `INV-ACTION-004`: `ActionRequestDraft` remains inert.
- `INV-ACTION-005`: `capabilities_required` are requirements, not grants.
- `INV-ACTION-006`: domain constraints are declarations, not permissions.
- `INV-ACTION-007`: ambiguous or unsafe intent must be `BLOCKED`.
- `INV-ACTION-008`: `ActionIntent` must preserve traceability.
- `INV-ACTION-009`: `ActionIntent` is not `ActionResolution`, `PendingAction`, or `AuthorizedExecutionRequest`.
- `INV-ACTION-010`: `project_runtime` remains unchanged.
- `INV-ACTION-011`: UI may present future `ActionIntent` but must not authorize or execute it.
- `INV-ACTION-012`: no execution boundary is opened by this contract.
- `INV-ACTIONREQ-001`: `ActionRequest` is declarative and non-executing.
- `INV-ACTIONREQ-002`: `ActionRequest` does not authorize tools, providers, filesystem mutation, network access, or document mutation.
- `INV-ACTIONREQ-003`: `ActionRequest` is richer than `ActionIntent` but remains non-executable.
- `INV-ACTIONREQ-004`: `ActionRequest` must preserve `intent_ref` and `trace_ref`.
- `INV-ACTIONREQ-005`: capability requirements are requirements, not grants.
- `INV-ACTIONREQ-006`: domain constraints are declarations, not permissions.
- `INV-ACTIONREQ-007`: blocked `ActionRequest` must not auto-fix or execute.
- `INV-ACTIONREQ-008`: `READY_FOR_RESOLUTION` does not mean authorized.
- `INV-ACTIONREQ-009`: `ActionRequest` is not `ActionResolution`, `PendingActionCandidate`, `AuthorizedExecutionRequest`, or `SingleToolExecution`.
- `INV-ACTIONREQ-010`: `ActionRequest` expected outputs do not create files.
- `INV-ACTIONREQ-011`: `ActionRequest` inputs must use governed refs, not raw payloads.
- `INV-ACTIONREQ-012`: `System View` may present `ActionRequest` summaries but must not control them.
- `INV-ACTIONREQ-013`: `project_runtime` remains unchanged.
- `INV-ACTIONREQ-014`: execution boundary remains `CLOSED`.

## Related specs

- `docs/specs/action_resolution.md`
- `docs/specs/tool_capability_model.md`
- `docs/specs/sandbox_boundary_model.md`
- `docs/specs/security_sanitization_policy.md`
- `docs/specs/system_observability.md`
- `docs/specs/system_view.md`
