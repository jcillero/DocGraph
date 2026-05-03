# system_observability

## Status

DECLARED_ONLY / F10-F11_PREP governance.

This document does not implement runtime logging, trace persistence, execution, provider invocation, tool dispatch, background processes, or telemetry.

## Purpose

Define the governed observability and trace model for DocGraph around `trace_ref`.

`trace_ref` is the primary declarative trace identifier used to:

- link major governed system artifacts
- reconstruct intent and proposal flow
- preserve future auditability
- remain compatible with future execution artifacts without opening execution now

## TraceRef

`trace_ref` is the primary trace identifier.

Format:

- `trace_<id>`

Rules:

- `trace_ref` is unique per logical operation or governed flow
- `trace_ref` is not tied to execution
- `trace_ref` may span:
  - `ToolUseProposal`
- `ActionRequestDraft`
- `ActionIntent`
- `ActionRequest`
- `ResolutionCandidate`
- `PendingActionCandidate`
- `HumanConfirmation`
- `AuthorizedExecutionRequest`
- `SingleToolExecution`
- future `ActionPlan`
- future `ActionResolution`
  - semantic generation context
  - future `ToolRunManifest`
- `trace_ref` does not imply authority
- `trace_ref` does not imply execution
- `trace_ref` does not imply semantic approval

## Trace scope

Allowed descriptive trace scopes:

- `SINGLE_INTENT`
- `MULTI_STEP_FLOW`
- `CROSS_ARTIFACT`

Rules:

- scope is descriptive only
- scope does not imply orchestration
- scope does not trigger execution

## Trace structure

Conceptual structure:

```json
{
  "trace_ref": "trace_xxx",
  "created_at": "...",
  "source": {
    "source_kind": "user | llm | system",
    "source_ref": "..."
  },
  "linked_entities": {
    "proposals": [],
    "drafts": [],
    "intents": [],
    "action_requests": [],
    "resolution_candidates": [],
    "pending_action_candidates": [],
    "human_confirmations": [],
    "authorized_execution_requests": [],
    "single_tool_executions": [],
    "semantic_quads": [],
    "quad_instances": [],
    "relations": []
  },
  "capability_context": [],
  "domain_context": [],
  "status": "OPEN | PARTIAL | COMPLETE | BLOCKED",
  "trace_metadata": {}
}
```

Rules:

- the structure is declarative only
- it is not persisted in this phase
- it does not create runtime logging
- `trace_metadata` must respect `docs/specs/security_sanitization_policy.md`
- no secrets may appear in trace metadata

## F13.4 manifest exposure trace alignment

`F13.4` is SPEC-only and does not create or persist new trace artifacts.

Future manifest-governed exposure must remain trace-linked but trace-non-authoritative.

Rules:

- `ExposureRequest`, `ExposureCandidate`, `HumanConfirmation`, and future `ManifestExposureEntry` must preserve `trace_ref`
- `trace_ref` may explain exposure history, duplicate handling, and blocking reasons, but must not authorize exposure
- trace linkage must remain ref-only and sanitized
- trace linkage must not contain raw payloads, secrets, or private absolute host paths
- trace presence, trace status, or future trace persistence must not imply project exposure without manifest authority

## Linking model

`trace_ref` should link governed artifacts by reference only.

Expected linkage targets include:

- `ToolUseProposal`
- `ActionRequestDraft`
- `ActionIntent`
- `ActionRequest`
- `ResolutionCandidate`
- `PendingActionCandidate`
- `HumanConfirmation`
- `AuthorizedExecutionRequest`
- `SingleToolExecution`
- `QuadInstance` generation metadata
- `QuadRelation` creation or review metadata
- future `ActionPlan`
- future `ActionResolution`
- future `ToolRunManifest`

Rules:

- linking is reference-only
- no raw data embedding
- no duplication of source content
- links should use governed IDs such as:
  - `file_ref`
  - `object_ref`
  - `intent_id`
  - `quad_id`
  - `quad_instance_id`
  - `relation_id`

## Trace status

Allowed descriptive status values:

- `OPEN`
- `PARTIAL`
- `BLOCKED`
- `COMPLETE`

Rules:

- status is descriptive only
- this generic status applies to flow-level `trace_ref` observability only
- this generic status is distinct from `TraceRecord.status`
- generic trace status must not be used to infer runtime execution
- generic trace status must not be used to infer `ToolRunManifest` creation
- generic trace status must not be used to infer `TraceRecord` persistence
- generic trace status must not be used to infer any persistence side effect
- status does not trigger execution
- `BLOCKED` does not auto-resolve
- `COMPLETE` does not imply execution occurred

`TraceRecord` semantics remain owned by this document.

For `F11.7`, `F11.8`, and `F11.10`, canonical `TraceRecord.status` values remain:

- `DECLARED_ONLY`
- `NOT_PERSISTED`
- `FUTURE_REQUIRED`

## Security and sanitization alignment

Traceability must comply with `docs/specs/security_sanitization_policy.md`.

Rules:

- no secrets in trace
- no raw absolute host paths in trace
- no unsanitized personal data leakage
- sanitized references only
- trace must not re-expand sanitized or redacted content

## Semantic alignment

`QuadInstance` generation metadata may include `trace_ref`.

Rules:

- semantic generation context should link to trace
- trace does not imply semantic approval
- trace does not modify lifecycle
- trace does not promote proposal-only semantic material into approved knowledge

## Capability and action alignment

`ActionIntent`, `ActionRequest`, `ResolutionCandidate`, `PendingActionCandidate`, `HumanConfirmation`, `AuthorizedExecutionRequest`, `SingleToolExecution`, and future action artifacts may include capability and domain context in trace.

Rules:

- capability context is observability metadata only
- domain context is observability metadata only
- trace does not authorize capabilities
- trace does not authorize sandbox, host, or external access

## Future compatibility

This trace model must remain compatible with:

- `ActionPlan`
- `ActionResolution`
- `AuthorizedExecutionRequest`
- `SingleToolExecution`
- `ToolRunManifest`
- `TraceRecord`

But in the current phase:

- no execution is enabled
- no persistence is required
- no runtime tracing is active
- no `TraceRecord` persistence is active

## F11.7 TraceRecord alignment

This document owns `TraceRecord` semantics.

References from `docs/specs/action_resolution.md` or other specs must not redefine `TraceRecord` semantics.

Future `TraceRecord` values may align to `SingleToolExecution` with:

- `trace_record_id`
- `trace_ref`
- `single_tool_execution_ref`
- `authorized_execution_request_ref`
- `action_request_ref`
- `event_kind`
- `scope`
- `status`
- `links`
- `sanitized_summary_key`
- `created_at`

Rules:

- `TraceRecord` remains a future runtime artifact
- `TraceRecord` remains future-required and is not persisted in `F11.7` or `F11.8`
- `TraceRecord` links by governed references only
- `TraceRecord` must remain compatible with `trace_ref` ownership in this spec
- `TraceRecord` must not duplicate raw payloads or secrets
- `TraceRecord` remains sanitized, non-authorizing, and non-executing
- `TraceRecord` status remains non-operational in `F11.7`

## F12.0 / F11.RUNTIME-0 trace proposal note

`F12.0 / F11.RUNTIME-0` is proposal-only.

The future first runtime opening must preserve `trace_ref` and must require future `TraceRecord` production when runtime is explicitly implemented later.

This proposal does not persist `TraceRecord`.

This proposal does not create runtime tracing.

This proposal does not create `ToolRunManifest`.

Trace metadata for any later implementation must remain sanitized, reference-only, and free of raw payloads, secrets, and private absolute host paths.

## F12.1 text.measure trace gate

`F12.1` is gate-only.

For a later `text.measure` implementation slice:

- `trace_ref` is mandatory
- `TraceRecord` is mandatory for future execution
- `TraceRecord` is not persisted in `F12.1`
- `ToolRunManifest` is not created in `F12.1`
- trace metadata must be sanitized
- trace metadata must be reference-only
- trace metadata must not contain raw payloads, secrets, or private absolute host paths

Future-only trace error codes include:

- `missing_trace_ref`
- `trace_required`

## F12.2A text.measure observability plan

`F12.2A` is plan-only and does not persist `TraceRecord`.

Future `F12.2B` must preserve the `F12.1` trace gate:

- `trace_ref` is mandatory
- missing `trace_ref` blocks execution
- `ToolRunManifest` is mandatory
- `TraceRecord`-compatible metadata is mandatory only as already declared by `F12.1` and `F11.7`
- trace metadata must be sanitized
- trace metadata must be reference-only
- trace metadata must not contain raw payloads, secrets, or private absolute host paths

Future `F12.2B` must not treat trace metadata as authorization, tool selection, dispatcher authority, or proof that a broad runtime is open.

Mandatory observability audit:

- `dev/scripts/audits/audit_f12_minimal_runtime_boundary.bat`

## Observability invariants

- `INV-TRACE-001`: `trace_ref` is declarative and non-executing.
- `INV-TRACE-002`: `trace_ref` does not authorize execution.
- `INV-TRACE-003`: `trace_ref` must not contain secrets or raw sensitive data.
- `INV-TRACE-004`: trace links artifacts by reference only.
- `INV-TRACE-005`: trace does not imply semantic approval.
- `INV-TRACE-006`: trace does not modify lifecycle, graph, storage, or actions.

## Related specs

- `docs/specs/identity_system.md`
- `docs/specs/action_contract.md`
- `docs/specs/action_resolution.md`
- `docs/specs/semantic_quad_model.md`
- `docs/specs/tool_capability_model.md`
- `docs/specs/security_sanitization_policy.md`
- `docs/specs/system_view.md`

## F12.4 file intake observability alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

Future file intake requires `trace_ref` before durable import.

Intake trace summaries must be sanitized, reference-only, and non-authorizing.

Trace metadata must not contain raw payloads, secrets, credentials, or private absolute host paths as portable truth.

F12.4 does not persist TraceRecord files.

## F12.5 file intake observability plan

`F12.5` is plan-only and persists no trace records.

Future `F12.6` intake metadata must preserve `trace_ref` and provide sanitized trace linkage for each `IntakeItem`.

Future `F12.6` must not persist raw host absolute paths, secrets, raw payloads, or unsafe comment text in trace metadata.

Trace linkage does not authorize project exposure, derivation, tool execution, or manifest mutation.
