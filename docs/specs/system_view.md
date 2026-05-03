# system_view

## Status

DECLARED_ONLY / F10-F11_PREP governance.

This document does not implement runtime, UI code, i18n resources, execution, authorization, provider invocation, tool dispatch, filesystem mutation, policy resolution, or `project_runtime` authority.

## Purpose

Define `System View` as the owner contract for a future readonly workspace tab that explains prepared system state without execution.

`System View` combines:

1. `Why Nothing Happens`
2. `System Trace`
3. `System Effort`
4. `Effective Tool Surface`
5. `Lume Interpretation`

It presents prepared state only.

It does not execute, authorize, approve, mutate, dispatch, resolve policy, or become runtime authority.

## System View definition

`System View` is:

- a single workspace tab
- readonly
- presentation-only
- driven by prepared state
- not a controller
- not a runner
- not a policy engine
- not a diagnostic executor

It may display:

- blocked explanations
- trace summaries
- action-intent summaries
- action-request summaries
- resolution-candidate summaries
- pending-action-candidate summaries
- human-confirmation summaries
- authorized-execution-request summaries
- capability and domain summaries
- effective tool-surface summaries
- Lume explanations

It must not display:

- secrets
- raw payloads
- unsanitized private paths
- hidden execution state

## Why Nothing Happens

`Why Nothing Happens` is the blocked-state explanation model.

Conceptual structure:

```json
{
  "blocked_reason_code": "...",
  "short_user_message_key": "...",
  "technical_detail_key": "...",
  "related_capability": "...",
  "related_domain": "SANDBOX | HOST | EXTERNAL | UNSPECIFIED",
  "related_trace_ref": "trace_xxx",
  "display_level": "simple | technical"
}
```

Controlled `blocked_reason_code` values:

- `missing_capability`
- `disabled_capability`
- `future_only_capability`
- `blocked_intent`
- `no_authorization`
- `domain_forbidden`
- `host_write_forbidden`
- `external_disabled`
- `ambiguous_target`
- `missing_trace`
- `execution_boundary_closed`
- `tool_not_implemented`
- `security_policy_blocked`

Rules:

- explanation is descriptive only
- explanation does not unblock anything
- explanation does not auto-fix anything
- explanation must not suggest bypassing governance
- Lume may explain the state but must not resolve it

## System Trace

`System Trace` is the readonly trace-presentation section.

It may show:

- `trace_ref`
- trace scope
- trace status
- linked proposals
- linked drafts
- linked intents
- linked quad instances
- linked relations
- linked tool-surface summaries
- timestamps, when prepared state provides them

Rules:

- refs only, never raw payloads
- no secrets
- no private absolute host paths
- trace does not imply execution
- trace does not imply approval
- trace does not mutate lifecycle

## System Effort

`System Effort` is a readonly effort summary over prepared state.

Conceptual structure:

```json
{
  "effort_level": "IDLE | LOW | MEDIUM | HIGH | UNKNOWN",
  "proposals_count": 0,
  "drafts_count": 0,
  "intents_count": 0,
  "blocked_count": 0,
  "traces_count": 0,
  "capability_count": 0,
  "restricted_capability_count": 0,
  "non_executable_notice_key": "..."
}
```

Effort factors may include:

- capability count
- domain sensitivity
- blocked state
- confirmation need
- trace breadth
- semantic material count, when available in prepared state

Rules:

- effort is UI interpretation of prepared state
- effort is not scheduling
- effort is not cost accounting
- effort is not telemetry
- effort does not execute anything

## Effective Tool Surface

`Effective Tool Surface` is a readonly visible section for effective tool and capability state.

Entries may be grouped by:

- `declared`
- `visible`
- `allowed`
- `implemented`
- `unavailable`
- `disabled`
- `future_only`
- `non_executable`

Reason hints may include:

- `policy_disabled`
- `project_override`
- `missing_dependency`
- `execution_closed`
- `future_phase`
- `external_dependency_missing`
- `provider_closed`
- `sandbox_blocked`

Rules:

- `System View` must not toggle tools
- `System View` must not select tools
- selection inside the visible summary must not trigger runtime work
- tool identity does not imply authorization
- capability availability does not imply execution
- raw full catalogs must not be exposed as authority
- effective tool-surface summary is consumed from prepared governed state, not from raw catalog inspection in UI

## Lume Interpretation

Within `System View`, Lume is a system-state interpreter only.

Lume may explain:

- why an action is blocked
- what capability is missing
- what domain is closed
- what trace links exist
- what the system is prepared to do
- why execution did not occur

Lume must not:

- execute
- authorize
- approve
- mutate files
- infer policy
- resolve tools
- call providers
- generate hidden state

Allowed explanation modes:

- `short`
- `detailed`
- `technical`

Rules:

- explanation derives from prepared state only
- visible text must use future i18n keys
- no LLM invocation is implied
- Static Mode explanation must be possible

## Future UI model

The following conceptual names are presentation contracts only:

- `SystemViewState`
- `NothingHappensState`
- `BlockedExplanationViewState`
- `TraceViewState`
- `TraceSummaryViewState`
- `EffortViewState`
- `EffectiveToolSurfaceSummaryViewState`
- `ToolSurfaceGroupViewState`
- `LumeInterpretationViewState`

Rules:

- these names are presentation contracts only
- no Rust implementation is created here
- no Slint implementation is created here
- no i18n resources are created here

## Readonly UI state model

All `System View` states are readonly presentation contracts.

Rules:

- all fields are presentation-ready
- no field grants authority
- no field triggers execution
- no state mutates runtime
- no raw payloads
- no secrets
- no private absolute host paths
- refs only where trace or source links are needed
- visible labels must be represented by future i18n keys, not literal strings

### SystemViewState

Conceptual fields:

- `tab_id`
- `title_key`
- `non_executable_notice_key`
- `nothing_happens`
- `trace_view`
- `effort`
- `effective_tool_surface`
- `lume_interpretation`
- `empty_state_key`

Interpretation:

- `tab_id`: governed workspace-tab identity for the `System View` tab
- `title_key`: i18n key for the tab title
- `non_executable_notice_key`: i18n key for the readonly and non-executing notice
- `nothing_happens`: readonly `NothingHappensState`
- `trace_view`: readonly `TraceViewState`
- `effort`: readonly `EffortViewState`
- `effective_tool_surface`: readonly `EffectiveToolSurfaceSummaryViewState`
- `lume_interpretation`: readonly `LumeInterpretationViewState`
- `empty_state_key`: i18n key for a no-data prepared state

### NothingHappensState

Conceptual fields:

- `has_blocked_items`
- `blocked_items`
- `primary_reason_code`
- `primary_message_key`
- `technical_message_key`
- `related_trace_refs`

Interpretation:

- `has_blocked_items`: whether any prepared blocked explanation exists
- `blocked_items`: list of readonly `BlockedExplanationViewState`
- `primary_reason_code`: dominant blocked explanation code, when one exists
- `primary_message_key`: primary user-facing explanation key
- `technical_message_key`: technical-detail explanation key
- `related_trace_refs`: sanitized trace references related to the blocked state

### BlockedExplanationViewState

Conceptual fields:

- `blocked_reason_code`
- `display_level`
- `message_key`
- `technical_detail_key`
- `related_capability`
- `related_domain`
- `related_trace_ref`

Interpretation:

- `blocked_reason_code`: controlled blocked explanation code
- `display_level`: `simple` or `technical`
- `message_key`: short explanation key
- `technical_detail_key`: technical explanation key
- `related_capability`: capability identifier or capability label ref, when applicable
- `related_domain`: `SANDBOX`, `HOST`, `EXTERNAL`, or `UNSPECIFIED`
- `related_trace_ref`: optional sanitized `trace_ref`

### TraceViewState

Conceptual fields:

- `traces`
- `selected_trace_ref`
- `empty_state_key`

Interpretation:

- `traces`: list of readonly `TraceSummaryViewState`
- `selected_trace_ref`: currently focused trace summary ref, if any
- `empty_state_key`: i18n key for no-trace state

### TraceSummaryViewState

Conceptual fields:

- `trace_ref`
- `scope`
- `status`
- `linked_proposals_count`
- `linked_drafts_count`
- `linked_intents_count`
- `linked_quad_instances_count`
- `linked_relations_count`
- `sanitized_label_key`

Interpretation:

- `trace_ref`: governed trace identity
- `scope`: `SINGLE_INTENT`, `MULTI_STEP_FLOW`, or `CROSS_ARTIFACT`
- `status`: `OPEN`, `PARTIAL`, `BLOCKED`, or `COMPLETE`
- count fields: readonly linked-entity counts only
- `sanitized_label_key`: i18n key for a sanitized human-readable label

### EffortViewState

Conceptual fields:

- `effort_level`
- `proposals_count`
- `drafts_count`
- `intents_count`
- `blocked_count`
- `traces_count`
- `capability_count`
- `restricted_capability_count`
- `effort_explanation_key`

Interpretation:

- `effort_level`: `IDLE`, `LOW`, `MEDIUM`, `HIGH`, or `UNKNOWN`
- count fields: readonly prepared-state counts only
- `effort_explanation_key`: i18n key describing current effort interpretation

### EffectiveToolSurfaceSummaryViewState

Conceptual fields:

- `groups`
- `empty_state_key`
- `non_authority_notice_key`

Interpretation:

- `groups`: list of readonly `ToolSurfaceGroupViewState`
- `empty_state_key`: i18n key for no-surface state
- `non_authority_notice_key`: i18n key clarifying that visible tool state is not authority

### ToolSurfaceGroupViewState

Conceptual fields:

- `group_kind`
- `label_key`
- `entries_count`
- `reason_hint_keys`

Interpretation:

- `group_kind`: one of `declared`, `visible`, `allowed`, `implemented`, `unavailable`, `disabled`, `future_only`, or `non_executable`
- `label_key`: i18n key for the group label
- `entries_count`: number of summarized entries in the group
- `reason_hint_keys`: i18n keys explaining why entries appear in the group

### LumeInterpretationViewState

Conceptual fields:

- `mode`
- `explanation_key`
- `detail_keys`
- `related_trace_ref`
- `static_mode_available`

Interpretation:

- `mode`: `short`, `detailed`, or `technical`
- `explanation_key`: primary Lume explanation key
- `detail_keys`: additional explanation keys derived from prepared state
- `related_trace_ref`: optional sanitized `trace_ref`
- `static_mode_available`: whether explanation remains possible without LLM capability

## Boundaries

`System View` must not:

- execute tools
- call providers
- dispatch actions
- mutate storage
- mutate semantic state
- mutate graph state
- mutate action state
- approve semantic material
- infer authority from UI
- become a policy engine

Future readonly presentation may summarize `ActionRequest` status, blocking reasons, capability requirements, domain constraints, `trace_ref`, and risk level without creating execution authority.

Future readonly presentation may also summarize `ResolutionCandidate` state, blocking reasons, capability evaluation, domain evaluation, policy evaluation, candidate tools, `trace_ref`, risk, and staleness without creating execution authority.

Future readonly presentation may also summarize `PendingActionCandidate` readiness, pending status, blocking reasons, staleness reasons, capability summary, domain summary, policy summary, expected-outputs summary, risk, and `trace_ref` without creating execution authority.

Future readonly presentation may also summarize `HumanConfirmation` decision, review scope, staleness-check result, risk-acknowledgement status, and `trace_ref` without creating execution authority.

Future readonly presentation may also summarize `AuthorizedExecutionRequest` authorization status, execution-scope summary, safety-snapshot summary, owner requirements, blocking reasons, and `trace_ref` without creating execution authority.

Future readonly presentation may also summarize `SingleToolExecution` declared-only status, tool-binding summary, execution-scope summary, output-plan summary, safety-snapshot summary, result placeholder, and `trace_ref`.

`System View` must not create `SingleToolExecution`, dispatch tools, select tools, execute tools, create outputs, or persist manifests or traces.

Future readonly presentation may also summarize manifest-required state, trace-required state, missing manifest or trace warnings, `owner_ref` readiness, and declared-only downstream persistence requirements.

`System View` must not create `ToolRunManifest`, persist `TraceRecord`, or treat missing persistence artifacts as runtime authority.

## F12.0 / F11.RUNTIME-0 System View alignment

`System View` may later show:

- runtime proposal state
- selected future minimal tool
- why runtime is still closed
- missing preconditions
- `owner_ref` readiness
- manifest requirement
- trace requirement
- execution boundary notice

`System View` must not:

- run the tool
- confirm
- authorize
- dispatch
- create manifest
- persist trace
- mutate files

Lume may explain this state only.

## F12.1 text.measure gate presentation

`System View` may later show:

- `text.measure` gate state
- missing `owner_ref`
- missing `trace_ref`
- unsafe input
- stale input
- invalid domain
- manifest-required state
- trace-required state
- execution boundary notice

`System View` must not:

- run `text.measure`
- authorize `text.measure`
- dispatch `text.measure`
- create `result.json`
- create `tool_run_manifest.json`
- persist `TraceRecord`
- mutate files

Lume may explain the gate state only.

## F12.2A text.measure implementation-plan presentation

`System View` may later show:

- `F12.2A` plan-only state
- future `F12.2B` crate boundary
- required `text.measure` `tool_id`
- missing `owner_ref`
- missing `trace_ref`
- manifest-required state
- trace-required state
- owner-scoped output requirement
- mandatory audit requirement
- execution boundary notice

`System View` must not:

- trigger `F12.2B`
- run `text.measure`
- authorize `text.measure`
- dispatch `text.measure`
- create `result.json`
- create `tool_run_manifest.json`
- persist `TraceRecord`
- mutate files
- grant `tool_runtime` dispatcher authority
- imply `project_runtime` may be modified

Lume may explain the implementation plan and audit checklist only.

## F12.4 file intake presentation alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

`System View` may present:

- selected files count
- intake batch status
- blocked reasons
- classification summary
- storage readiness
- exposure readiness
- derivation readiness
- `trace_ref`
- missing preconditions
- comment preview
- comment presence indicator
- comment sanitization warnings
- missing comment as optional information

`System View` must not:

- import files
- copy files
- classify by itself
- expose files
- register files
- derive files
- execute tools
- authorize intake
- mutate storage
- edit comments
- persist comments
- generate comments
- infer comments

Lume may explain why a file cannot be ingested, what metadata is missing, why a format is unsupported, why source path is not portable truth, why exposure requires manifest governance, how comments are used, how to improve comments, and why a comment was flagged.

Lume must not import files, authorize intake, mutate manifests, execute tools, infer project exposure, persist comments, modify comment metadata, bypass the promotion flow, or inspect raw payloads unless a later governed runtime provides sanitized summaries.

## F12.5 file intake plan presentation

`F12.5` is plan-only and adds no UI behavior.

System View may later present the planned `F12.6` gate, required audit, missing `owner_ref`, missing `trace_ref`, blocked reasons, metadata readiness, comment sanitization readiness, and execution-boundary notice.

System View must not import, copy, hash, classify, persist metadata, expose files, mutate `project_manifest.json`, generate registries, write graph entries, derive text, or execute tools.

Lume may explain the `F12.5` plan and future audit checklist only.

## F12.7 file intake batch visibility

`F12.7` allows readonly `System View` presentation of already prepared `F12.6` file intake batch state.

System View may present:

- intake batch reference
- item count
- per-item sanitized filename or sanitized source label
- per-item status: `candidate`, `blocked`, or `imported_not_exposed`
- `owner_ref`
- `trace_ref`
- sanitization state
- sanitized `user_comment` preview when present
- blocking reason when present

Rules:

- UI consumes prepared file intake state only
- UI must not classify, validate, reinterpret, import, expose, derive, execute, or persist file intake state
- visible labels must use i18n keys
- source host paths must not be presented or persisted as portable truth
- blocked state remains explanatory and does not authorize retry or runtime work

System View must not import files, expose files to `project_manifest.json`, modify `registry.json`, write graph entries, create derivatives, call `document_text_runtime`, call `tool_runtime`, add execution authority, or persist raw host paths.

## F12.5-F12.7 minimal intake baseline System View closure

Within the minimal governed file intake baseline:

- `F12.5` is complete as the plan/audit slice
- `F12.6` is complete as the minimal governed intake runtime slice
- `F12.7` is complete as readonly `System View` visibility over prepared intake state

`System View` remains presentation-only.

This closure does not authorize UI import, exposure, registry generation, graph writes, derivatives, `document_text_runtime`, `tool_runtime` orchestration, or execution authority.

Allowed future expansion candidates are proposal-only:

1. project exposure gate, defined by `F13.0` as SPEC-only
2. derivatives gate
3. more file formats
4. intake history/index view, defined by `F12.9` as SPEC-only
5. storage dedup hardening, defined by `F12.8` as SPEC-only

## F12.9 intake history/index presentation

`F12.9` defines a future readonly intake history/index presentation contract.

System View may later present:

- intake batch history rows
- item history rows
- batch reference and item count
- item status: `candidate`, `blocked`, or `imported_not_exposed`
- duplicate/reuse indicators from prepared metadata
- blocked item visibility with sanitized blocking reason
- `owner_ref`
- `trace_ref`
- sanitization state
- sanitized `user_comment` preview when present

Rules:

- history/index state is derivable and rebuildable from governed intake metadata
- history/index state is non-authoritative
- history/index state must preserve one row per governed `IntakeItem`
- duplicate/reuse visibility must not merge items or infer exposure
- blocked item visibility must not authorize retry or runtime work
- visible history labels must use i18n keys when implemented

System View must not import files, expose files to `project_manifest.json`, modify `registry.json`, write graph entries, create derivatives, call `document_text_runtime`, call `tool_runtime`, create or repair history indexes, add execution authority, or persist raw host paths.

## F13.0 project exposure gate presentation

`F13.0` defines a future readonly presentation contract for the Project Exposure Gate.

System View may later present:

- exposure request summary
- exposure candidate summary
- source `IntakeItem` or `StoredObject` refs
- current state `imported_not_exposed`
- proposed future state `exposed_to_project`
- human confirmation requirement
- `owner_ref`
- `trace_ref`
- duplicate exposure warning
- blocked or unsupported exposure rejection reason
- project manifest requirement
- registry non-authority notice
- graph future-only notice

Rules:

- System View consumes prepared exposure-gate state only
- System View must not create exposure requests
- System View must not confirm exposure
- System View must not mutate `project_manifest.json`
- System View must not infer exposure from `file_store`, intake history, registry, graph, chat, or filesystem presence
- System View must not expose blocked or unsupported items

System View must not create a manifest writer, registry generator, graph writer, derivative runtime, `document_text_runtime` call, `tool_runtime` call, rollback flow, revocation flow, or execution authority.

## F13.4 manifest exposure contract presentation

`F13.4` further hardens the future readonly exposure presentation contract.

System View may later present:

- request status
- candidate status
- eligibility result
- blocking reasons
- confirmation state
- future manifest exposure status
- duplicate assessment
- `owner_ref`
- `trace_ref`
- sanitized display label

Rules:

- accepted confirmation must be shown as prerequisite state only, not as exposure
- future manifest exposure summaries must remain ref-based and sanitized
- System View must not treat trace presence, duplicate reuse, intake history rows, or blob presence as project membership
- System View must not write `project_manifest.json`, registry rows, graph rows, or derivative state

## F13.5A manifest exposure runtime presentation checklist

`F13.5A` is SPEC-only and adds no UI or presentation runtime.

Future `System View` alignment checklist:

- may present prepared exposure runtime readiness only
- may present typed failure categories only after a later explicit runtime slice exists
- must not trigger manifest writes, retries, confirmation, or duplicate-policy resolution
- must not become evidence of exposure authority without manifest state

## System View invariants

- `INV-SYSTEM-VIEW-001`: `System View` is readonly.
- `INV-SYSTEM-VIEW-002`: `System View` does not execute.
- `INV-SYSTEM-VIEW-003`: `System View` explains prepared state only.
- `INV-SYSTEM-VIEW-004`: UI does not become authority.
- `INV-SYSTEM-VIEW-005`: blocked state is explanatory, not failure.
- `INV-SYSTEM-VIEW-006`: `System View` does not authorize capabilities or tools.
- `INV-SYSTEM-VIEW-007`: `System View` does not mutate storage, semantic, graph, actions, or lifecycle.
- `INV-SYSTEM-VIEW-008`: `System View` must not expose secrets or raw payloads.
- `INV-SYSTEM-VIEW-009`: Lume interpretation is explanatory only.
- `INV-SYSTEM-VIEW-010`: `System View` must remain compatible with Static Mode.
- `INV-SYSTEM-VIEW-011`: `System View` state is presentation-only.
- `INV-SYSTEM-VIEW-012`: `System View` state must not contain raw payloads.
- `INV-SYSTEM-VIEW-013`: `System View` state must not contain secrets.
- `INV-SYSTEM-VIEW-014`: `System View` state must not grant authorization.
- `INV-SYSTEM-VIEW-015`: `System View` state must be derivable from prepared state.
- `INV-SYSTEM-VIEW-016`: UI state must not reinterpret policy.
- `INV-SYSTEM-VIEW-017`: `System View` selection must not trigger runtime work.
- `INV-SYSTEM-VIEW-018`: visible labels must be represented by i18n keys.
- `INV-SYSTEM-VIEW-019`: `F12.7` file intake visibility consumes prepared `F12.6` state only.
- `INV-SYSTEM-VIEW-020`: `F12.7` must not grant UI import, exposure, derivation, tool, or persistence authority.
- `INV-SYSTEM-VIEW-021`: minimal intake baseline closure does not promote `System View` into intake authority.
- `INV-SYSTEM-VIEW-022`: future intake expansions remain proposal-only until separately opened.
- `INV-SYSTEM-VIEW-023`: `F12.9` intake history/index visibility is readonly, derivable, rebuildable, and non-authoritative.
- `INV-SYSTEM-VIEW-024`: `F12.9` history/index presentation must not imply project exposure, registry generation, graph writes, derivatives, or runtime authority.
- `INV-SYSTEM-VIEW-025`: `F12.9` duplicate, blocked, owner, trace, and comment visibility is explanatory only.
- `INV-SYSTEM-VIEW-026`: `F13.0` project exposure gate presentation is readonly and consumes prepared state only.
- `INV-SYSTEM-VIEW-027`: `F13.0` System View presentation must not create requests, confirm exposure, mutate `project_manifest.json`, or expose items.
- `INV-SYSTEM-VIEW-028`: `F13.0` System View presentation must not infer exposure from filesystem, `file_store`, registry, graph, history/index, chat, or UI selection.
- `INV-SYSTEM-VIEW-029`: `F13.1` hardening forbids `System View` from routing legacy import, tree-scan, chat-resource, or derivation helpers as hidden exposure actions.
- `INV-SYSTEM-VIEW-030`: legacy file visibility shown by `System View`, if any, must remain explicitly non-authoritative and must not be presented as manifest-governed exposure.
- `INV-SYSTEM-VIEW-031`: `F13.4` exposure request, candidate, confirmation, and manifest-status visibility is readonly and explanatory only.
- `INV-SYSTEM-VIEW-032`: `System View` must not treat accepted confirmation, trace presence, duplicate reuse, or intake history presence as project exposure authority.
- `INV-SYSTEM-VIEW-033`: future manifest exposure summaries must use sanitized portable refs only and must not expose raw payloads, secrets, or private absolute host paths.

## Related specs

- `docs/specs/system_observability.md`
- `docs/specs/action_contract.md`
- `docs/specs/action_resolution.md`
- `docs/specs/tool_capability_model.md`
- `docs/specs/llm_tool_surface_resolution.md`
- `docs/specs/sandbox_boundary_model.md`
- `docs/specs/security_sanitization_policy.md`
- `docs/specs/lume_interaction_model.md`
- `docs/specs/workspace_tabs.md`
- `docs/specs/ui_core.md`
