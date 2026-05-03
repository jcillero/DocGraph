# pending_action_state

## Status

DECLARED_ONLY / F11.3-F11.4_PREP governance.

This document does not implement `PendingAction` runtime, confirmation runtime, `AuthorizedExecutionRequest`, `SingleToolExecution`, runner behavior, dispatcher behavior, executor behavior, filesystem mutation, tool execution, provider invocation, or `project_runtime` authority.

## Purpose

Define the conceptual F9-ready contract for a future `PendingActionState`.

This is documentation only. It does not implement `ActionRequest`, execution, cancellation, editing, or runtime state.

This document also defines `PendingActionCandidate` as the rich `F11.3` confirmation-preparation artifact derived from `ResolutionCandidate`.

`PendingActionCandidate` is reviewable and non-executing.

This document also defines `HumanConfirmation` as the explicit `F11.4` human decision event over a `PendingActionCandidate`.

`HumanConfirmation` is traceable and non-executing.

## Conceptual contract

`PendingActionState` represents one concrete `ActionRequest` that is pending execution, cancellation, or editing.

The Execute button never means "execute the latest thing from chat".

Execute always references a specific `action_id`.

The UI may show:

- `Execute`
- `Cancel`
- `Edit`

### Legacy UI vocabulary clarification

The `PendingActionState`, `Execute`, `Cancel`, and `Edit` wording above is future UI vocabulary only.

It is not part of `System View` authority.

`PendingActionCandidate` is confirmation-preparation only.

`System View` may present pending or confirmation summaries, but it must not confirm, authorize, execute, cancel, edit, or mutate anything.

Any future `Execute` action belongs only to a later explicit execution slice.

The UI does not decide policy.

Policy is resolved by governed runtime layers in a future phase.

## F9 limits

`PendingActionState` in F9 is a documented concept only.

It does not imply:

- real execution
- tool calling
- LLM invocation
- filesystem mutation
- sandbox action execution
- `project_runtime` changes

## Future F10/F11 notes

F10 may introduce concrete pending action requests if tool or LLM execution is explicitly opened.

F11 should audit that every execution request references an explicit `action_id` and cannot fall back to implicit chat context.

## F10.9 pending-candidate bridge

`F10.9` opens resolution preparation only.

It may introduce an inert pending candidate produced from governed `ResolutionCandidate` preparation.

In F10.9:

- pending candidate is not execution
- pending candidate is not approval to execute
- pending candidate is not an `ActionResolution` runner
- pending candidate may preserve origin, target refs, context refs, policy refs, and trace refs
- ambiguous targets must become `BLOCKED` or `CONFIRM_REQUIRED`
- stale drafts must not become confirmable pending candidates
- forbidden actions must remain `BLOCKED`
- user confirmation may only approve a pending candidate, not execute it
- execution remains future and separately gated

## F11.0 pending-state note

`F11.0` is DECLARED / NOT ACTIVE.

Pending state may model a future confirmation lifecycle.

Confirmation does not execute in `F11.0`.

Stale or expired candidates cannot be confirmed.

## F11.3 PendingActionCandidate opening note

`F11.3` opens only the rich `PendingActionCandidate` contract.

It does not open confirmation runtime.

It does not open authorization.

It does not open execution.

It does not implement transition runtime.

## F11.4 HumanConfirmation opening note

`F11.4` opens only the `HumanConfirmation` contract.

It does not open confirmation runtime.

It does not open authorization runtime.

It does not open execution.

It does not implement transition runtime.

## F11.3 non-executing chain

The governed non-executing chain is:

`ActionRequest -> ResolutionCandidate -> PendingActionCandidate -> HumanConfirmation -> AuthorizedExecutionRequest -> SingleToolExecution`

Rules:

- `F11.3` opens only the `PendingActionCandidate` contract
- `F11.4` opens only the `HumanConfirmation` contract
- `F11.5` may define `AuthorizedExecutionRequest` as a contract only
- `F11.6` may define `SingleToolExecution` as a declared-only contract only
- `SingleToolExecution` remains non-running in `F11.6`
- execution boundary remains `CLOSED`
- no transition runtime is implemented

## PendingActionCandidate definition

`PendingActionCandidate` is a rich non-executing confirmation-preparation artifact.

It represents a reviewed, non-stale, non-blocked `ResolutionCandidate` that may be shown for future human confirmation.

It does not confirm anything.

It does not authorize anything.

It does not execute anything.

Conceptual structure:

```json
{
  "pending_action_candidate_id": "pac_xxx",
  "resolution_candidate_ref": "rc_xxx",
  "action_request_ref": "ar_xxx",
  "intent_ref": "intent_xxx",
  "trace_ref": "trace_xxx",
  "status": "PENDING_REVIEW",
  "confirmation_readiness": "READY | NOT_READY | BLOCKED | STALE | UNSAFE",
  "summary": {
    "summary_key": "...",
    "technical_summary_key": "..."
  },
  "capability_summary": [],
  "domain_summary": [],
  "policy_summary": [],
  "candidate_tool_summary": [],
  "expected_outputs_summary": [],
  "risk": {
    "risk_level": "LOW | MEDIUM | HIGH | UNKNOWN",
    "risk_reasons": []
  },
  "blocking_reasons": [],
  "staleness": {
    "is_stale": false,
    "stale_reasons": []
  },
  "created_at": "..."
}
```

## Status and confirmation-readiness model

`PendingActionCandidate` status values:

- `PENDING_REVIEW`
- `NOT_READY`
- `BLOCKED`
- `STALE`
- `SUPERSEDED`
- `EXPIRED`

`confirmation_readiness` values:

- `READY`
- `NOT_READY`
- `BLOCKED`
- `STALE`
- `UNSAFE`

Rules:

- `READY` means reviewable for future human confirmation only
- `READY` does not mean confirmed
- `READY` does not mean authorized
- `READY` does not mean executable
- there is no `CONFIRMED` state in `F11.3`
- there is no `AUTHORIZED` state
- there is no `EXECUTING` state
- there is no `EXECUTED` state
- `BLOCKED` and `STALE` do not auto-fix

## Eligibility rules

A `PendingActionCandidate` may only be represented as `READY` when:

- `ResolutionCandidate` is `RESOLVABLE`
- `ResolutionCandidate` is not `BLOCKED`
- `ResolutionCandidate` is not `STALE`
- `ActionRequest` is not `BLOCKED`
- `trace_ref` exists
- required capabilities are satisfied by contract evaluation
- domain evaluation has no forbidden access
- policy evaluation has no blocked or unknown-critical policy
- expected outputs preserve future `owner_ref`, manifest, and trace requirements
- risk is not `UNKNOWN` when policy requires reviewable risk classification
- source material is not stale

Rules:

- eligibility is declarative only
- no runtime transition occurs
- no resolver is invoked
- no tool is selected for execution

## Summary models

Summary arrays are presentation-ready and non-authoritative.

Capability summary fields:

- `capability`
- `status`
- `message_key`

Domain summary fields:

- `required_domain`
- `access_level`
- `status`
- `message_key`

Policy summary fields:

- `policy_ref`
- `status`
- `message_key`

Candidate-tool summary fields:

- `tool_id`
- `tool_kind`
- `availability_state`
- `message_key`

Expected-outputs summary fields:

- `output_kind`
- `owner_ref_required`
- `manifest_required`
- `trace_required`
- `message_key`

Rules:

- summaries do not grant authority
- candidate-tool summary is explanatory only
- expected-outputs summary does not materialize files
- message fields must be future i18n keys in UI

## Relationship to HumanConfirmation

- `PendingActionCandidate` = prepared candidate for future confirmation display
- `HumanConfirmation` = future explicit user approval event
- `AuthorizedExecutionRequest` = downstream post-confirmation authorization artifact

Rules:

- `F11.3` does not create `HumanConfirmation`
- `F11.3` does not accept confirmation
- `F11.3` does not authorize execution
- `F11.3` does not create `AuthorizedExecutionRequest`
- future confirmation must verify that the candidate is not stale at confirmation time

## HumanConfirmation definition

`HumanConfirmation` is an explicit, traceable human decision event over a `PendingActionCandidate`.

It records whether a human reviewer accepted, rejected, deferred, or requested changes.

It does not execute anything.

It does not call tools.

It does not mutate files.

It does not by itself create execution.

Conceptual structure:

```json
{
  "human_confirmation_id": "hc_xxx",
  "pending_action_candidate_ref": "pac_xxx",
  "resolution_candidate_ref": "rc_xxx",
  "action_request_ref": "ar_xxx",
  "intent_ref": "intent_xxx",
  "trace_ref": "trace_xxx",
  "reviewer_ref": "user_xxx",
  "decision": "ACCEPTED | REJECTED | DEFERRED | CHANGES_REQUESTED",
  "decision_reason_code": "...",
  "review_scope": "SUMMARY_ONLY | TECHNICAL_DETAILS | FULL_TRACE",
  "staleness_check": {
    "performed": true,
    "result": "FRESH | STALE | UNKNOWN",
    "checked_at": "..."
  },
  "risk_acknowledgement": {
    "required": false,
    "acknowledged": false
  },
  "created_at": "..."
}
```

## HumanConfirmation decision model

Allowed `decision` values:

- `ACCEPTED`
- `REJECTED`
- `DEFERRED`
- `CHANGES_REQUESTED`

Rules:

- `ACCEPTED` does not mean executed
- `ACCEPTED` does not mean authorized for execution by itself
- `REJECTED` does not delete the candidate
- `DEFERRED` preserves state for later review
- `CHANGES_REQUESTED` does not auto-modify the request
- any accepted confirmation must still pass a future authorization gate

## HumanConfirmation review-scope model

Allowed `review_scope` values:

- `SUMMARY_ONLY`
- `TECHNICAL_DETAILS`
- `FULL_TRACE`

Rules:

- `review_scope` describes what was presented to the human
- `review_scope` does not grant authority
- `FULL_TRACE` must still avoid secrets and raw payloads
- presentation must remain sanitized

## HumanConfirmation staleness check

`HumanConfirmation` must record staleness-check metadata.

Rules:

- future confirmation must re-check staleness
- `ACCEPTED` is invalid if `staleness_check.result` is `STALE`
- `UNKNOWN` staleness must block progression to future authorization unless explicitly governed later
- staleness-check metadata is descriptive in `F11.4`
- no runtime check is implemented now

## HumanConfirmation risk acknowledgement

Risk acknowledgement is explicit metadata.

Rules:

- `HIGH` or `UNKNOWN` risk may require acknowledgement in the future
- `risk_acknowledgement` does not authorize execution
- acknowledgement does not bypass sandbox, policy, security, or capability rules

## Relationship to authorization

- `HumanConfirmation` = human decision event
- `AuthorizedExecutionRequest` = post-confirmation authorization artifact
- `SingleToolExecution` = future execution step

Rules:

- `F11.4` does not create `AuthorizedExecutionRequest`
- `F11.4` does not create authorization runtime
- `F11.4` does not execute
- future authorization must revalidate candidate freshness, capability status, domain constraints, security and sanitization policy, and expected-output `owner_ref`, manifest, and trace requirements
- `F11.5` may define `AuthorizedExecutionRequest` as a post-confirmation artifact only
- `F11.5` does not create authorization runtime or execution runtime
- `F11.6` may define `SingleToolExecution` as a downstream declared-only artifact only
- `F11.6` does not create execution runtime, runner, dispatcher, executor, `ToolRunManifest`, `TraceRecord`, output persistence, provider invocation, or filesystem mutation

## HumanConfirmation traceability

`HumanConfirmation` must preserve:

- `pending_action_candidate_ref`
- `resolution_candidate_ref`
- `action_request_ref`
- `intent_ref`
- `trace_ref`
- `reviewer_ref`
- decision
- `review_scope`
- `staleness_check`
- `risk_acknowledgement`

Rules:

- refs only
- no raw payloads
- no secrets
- no private absolute host paths
- `reviewer_ref` is an identifier only, not credential authority

## Relationship to System View for HumanConfirmation

`System View` may present:

- confirmation decision summary
- review scope
- staleness-check result
- risk-acknowledgement status
- `trace_ref`

Rules:

- `System View` does not confirm
- `System View` does not authorize
- `System View` does not execute
- Lume may explain the confirmation state but must not decide or confirm

## Traceability

`PendingActionCandidate` must preserve:

- `resolution_candidate_ref`
- `action_request_ref`
- `intent_ref`
- `trace_ref`
- capability summary
- domain summary
- policy summary
- candidate-tool summary
- expected-output summary
- risk metadata
- staleness metadata

Rules:

- refs only
- no raw payloads
- no secrets
- no private absolute host paths
- trace does not authorize execution

## Relationship to System View

`System View` may present:

- `confirmation_readiness`
- pending status
- summary
- blocking reasons
- staleness reasons
- capability summary
- domain summary
- policy summary
- expected-outputs summary
- risk level
- `trace_ref`

Rules:

- `System View` does not create `PendingActionCandidate`
- `System View` does not confirm
- `System View` does not authorize
- `System View` does not execute
- Lume may explain readiness but must not resolve or confirm

## Relationship to ActionRequest

`ActionRequest` is the richer formal reviewable request that may exist before pending state.

`ResolutionCandidate` is the inert evaluation artifact that may exist between `ActionRequest` and future pending state.

`PendingActionCandidate` remains future confirmation-preparation only.

Rules:

- `ActionRequest` is not pending state
- `ResolutionCandidate` is not pending state
- pending state remains downstream from inert resolution preparation
- only a non-stale, non-blocked, reviewable `ResolutionCandidate` may become `PendingActionCandidate` in a future slice
- `F11.2` does not perform that transition
- this spec does not create confirmation, authorization, or execution

## Pending-action invariants

- `INV-PENDING-CAND-001`: `PendingActionCandidate` is confirmation-preparation only.
- `INV-PENDING-CAND-002`: `PendingActionCandidate` is non-executing.
- `INV-PENDING-CAND-003`: `READY` does not mean confirmed, authorized, or executable.
- `INV-PENDING-CAND-004`: `PendingActionCandidate` must derive from a non-blocked, non-stale `ResolutionCandidate`.
- `INV-PENDING-CAND-005`: `PendingActionCandidate` must preserve `resolution_candidate_ref`, `action_request_ref`, `intent_ref`, and `trace_ref`.
- `INV-PENDING-CAND-006`: candidate-tool summaries are explanatory, not selected tools.
- `INV-PENDING-CAND-007`: expected-output summaries do not create files.
- `INV-PENDING-CAND-008`: `HumanConfirmation` remains explicit and non-executing; confirmation runtime remains future.
- `INV-PENDING-CAND-009`: future confirmation must re-check staleness.
- `INV-PENDING-CAND-010`: `System View` may present `PendingActionCandidate` but must not confirm or control it.
- `INV-PENDING-CAND-011`: `F11.3` does not create runtime transition, runner, dispatcher, executor, or confirmation runtime.
- `INV-PENDING-CAND-012`: `project_runtime` remains unchanged.
- `INV-PENDING-CAND-013`: execution boundary remains `CLOSED`.
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
