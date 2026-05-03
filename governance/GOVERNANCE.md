# Governance

This sandbox evolves under contract-first, spec-driven, crate-driven governance.

It inherits doctrine from the historical Python platform while making Rust the active operational and engineering center of gravity for the living sandbox.

## Normative inheritance

The Rust sandbox does not supersede the live Python knowledge base.

It inherits and adapts, in this order:

1. `OPERATIONAL_DEFINITION_Rev09.txt`
2. `INVARIANTS_GUIDE.md`
3. active governance indexing and policy documents
4. active specs, contracts, and registries
5. stable Python behavior when aligned with the above
6. sandbox-local Rust documentation in this workspace

## Governing principles

- preserve system separation between runtime, tooling, and user-owned data
- prefer declarative contracts over hardcoded behavior
- grow by explicit crate boundaries, not by convenience
- keep higher layers consuming governed verticals rather than replacing them
- avoid speculative APIs, future-proof abstractions, and duplicate pipelines
- keep documentation aligned with implementation before expanding scope

## Current evolution posture

- Python remains an inherited doctrinal and migration reference
- Rust is the active implementation and operational target
- the sandbox remains in `dev/` because it is still an engineering workspace
- the visible application identity is `DocGraph`

Normative Python boundary:

- Python is doctrinal reference only
- Python must not be invoked, embedded, required, or treated as runtime authority in the active Rust sandbox
- Rust remains the active implementation and operational runtime
- any future dependency on Python would require an explicit governed phase and contract
- the assistant/help identity is `Lume`
- `3A`, `3B`, `F4A`, `F4B`, `F5`, `F6`, `F7`, and `F8` are CLOSED
- F9 preparation may add declarative preferences, credentials, runtime locations, Lume help resources, tool catalogs, sandbox policy, and storage policy without opening F10 execution
- F9 tools/catalog governance is declaratively closed and does not implement runtime execution
- `resources/tools/tools_master_catalog.json` remains declarative only
- `resources/tool_runtime/*` remains the operative source in the current phase
- `EffectiveToolSurfaceResolver` is future-only and is not implemented in F9

## Rules for system growth

- `project_runtime` owns the central project pipeline
- `app_services` consumes governed verticals and remains thin
- no second implementation of the project pipeline may appear in `app_services`, `cli_app`, `ui_*`, or elsewhere
- `tool_runtime` remains separate from project and knowledge concerns unless a later phase explicitly opens a justified integration
- the knowledge layer consumes existing boundaries or a very small technical boundary; it does not govern the project pipeline
- the knowledge layer must not push knowledge concerns into `project_runtime`
- document source files remain primary; derived text, pages, chunks, and future semantics stay regenerable
- text derivation stays outside UI, viewer, `project_runtime`, `app_services`, and `tool_runtime`
- the central UI content area is a simple workspace tab container, not a fixed viewer pane and not a new domain layer
- workspace tabs are controlled views; they do not govern runtime, manifest interpretation, or project structure
- the `tree -> clip -> chat -> workspace -> viewer` interaction model is the CLOSED F8 operating contract
- `Operational Tools` and `LLM Tools` are governed as different surfaces:
  - operational tools may expose controlled manual launch
  - LLM tools expose policy, not manual execution
- UI is not the source of truth for tool policy; declarative global and project policy remain canonical
- a project override may restrict or select within the globally allowed LLM tool surface, but it does not redefine catalogs or create new tools
- UI must not reinterpret the manifest locally
- the readonly viewer is reused as a concrete workspace tab; parallel viewer flows are not introduced casually
- moving from a viewer-centric shell description to workspace tabs does not reopen F5, F6, or F7; it clarifies the shell model above the same boundaries
- existing governed documents are referenced from the tree; they are not re-imported through chat convenience flows
- the clip is reserved for external document intake and explicit workflow launch
- chat stores text, structured document references, tool results, and system-state messages; it does not become a hidden document store
- document profiling opens a workspace workflow; it does not execute “inside” the chat input
- operational tool launch from document workflows remains operational only; `LLM Tools` are not manually executed from clip/chat flows
- phase growth is sequential; do not open multiple large fronts at once

## F9-ready declarative governance

The following declarations are allowed as F9 preparation only:

- nominal product identity for `DocGraph` and assistant identity for `Lume`
- Lume interaction, tone, symbols, onboarding, and help resources
- Lume Help contextual popup resources and presentation contracts without execution
- `GUI_OBJECTS_v1` canonical GUI vocabulary for Lume Help contextual explanations
- governed Markdown document template proposals without export runtime
- governed document reference and bibliography style proposals without parser or export runtime
- user preferences and transparent user profile policy
- internal, external, and LLM tool catalogs without new execution
- runtime locations and missing external runtime placeholders
- LLM engine and model catalogs with real execution disabled
- authorized local sandbox policy with mutable actions disabled
- file-based storage policy with checksum deduplication and regenerable JSONL indexes
- governed file-store and `StoredObject` JSON schemas as declarative contracts only
- action, flow-control, and pending-action-state policies without execution
- ActionResolution specification without runner, execution, LLM invocation, tool invocation, or filesystem mutation
- F9.5 AI governance declarations, semantic proposal schema, and readonly/mock Pipeline / Ontology View without LLM execution, embeddings, RDF persistence, Oxigraph, or semantic store
- semantic quad, relation, lifecycle, quadset, and graph-analysis schemas as declarative contracts only
- semantic classification tag catalog and lifecycle policy as declarative contracts only
- semantic tag catalog schema as a declarative contract only
- semantic layer boundary policy as a declarative contract only
- semantic invalidation policy as a declarative contract only
- security and sanitization policy as a declarative contract only
- quad generation policy as a declarative contract only
- semantic storage limits policy as a declarative contract only
- caching and reuse policy as a declarative contract only
- tool capability model as a declarative contract only
- sandbox boundary model as a declarative contract only
- action contract as a declarative contract only
- system observability and trace model as a declarative contract only
- System View as a declarative readonly presentation contract only
- rich `ActionRequest` contract as a declarative non-executing contract only
- rich `ResolutionCandidate` contract as a declarative inert evaluation contract only
- rich `PendingActionCandidate` contract as a declarative confirmation-preparation contract only
- `HumanConfirmation` contract as a declarative non-executing human decision event only
- `AuthorizedExecutionRequest` contract as a declarative post-confirmation authorization artifact only
- `SingleToolExecution` contract as a declarative future minimum execution artifact only

These declarations must not open F10, enable broad tool execution, install binaries, invoke real LLM workflows, mutate files through Lume, hardcode visible UI strings, or duplicate the `project_runtime` pipeline.

F9 closure does not authorize broad tool execution, LLM execution, external binary execution, or governed document editing runtime.

Governed storage schemas are declarative only:

- schema presence does not imply runtime validation
- schema validity does not imply project exposure
- schema validity does not imply execution authority
- `project_manifest` remains the exposure authority

Governed semantic schemas are declarative only:

- schema presence does not imply approval
- schema validity does not imply RDF projection
- schema validity does not imply graph analysis runtime
- schema validity does not imply execution authority

The only allowed operational exception is the minimum governed `text.measure` slice, which may persist output only under an owner-scoped `user/output/tool_runs/<owner-kind>/<owner-id>/<run_id>/` path with mandatory `owner_ref` and `tool_run_manifest.json`.

## F10 step-1 opening gate

The first explicit F10 opening gate is limited to a minimum governed LLM-assisted runtime slice.

Current implemented state:

- F10 minimal governed LLM runtime: CONDITIONALLY CLOSED
- `F10.1` is minimally opened for pure `llm_core` capability-state resolution
- `F10.2` is minimally opened for pure `llm_core` assisted-chat envelope preparation
- `F10.3` is minimally opened for readonly `EffectiveToolSurfaceSummary` awareness
- `F10.4` is minimally opened for precomputed tool-surface summary injection into the assisted-chat envelope
- `F10.5` is minimally opened for proposal-only `ToolUseProposal` modeling
- `F10.6` is minimally opened for inert `ActionRequestDraft` modeling
- `F10.7` is minimally opened for `LlmInteractionTrace` observability metadata
- `F10.8` is minimally opened for a UI-safe LLM status view model
- `F10.9` opens resolution preparation only between proposal/draft artifacts and inert `ActionResolution` results
- broad F10 runtime remains closed
- provider invocation remains closed
- tool execution remains closed
- full `ActionRequest` runtime remains not implemented
- `ActionResolution` runner remains closed
- UI remains presentation-only
- `project_runtime` remains unchanged

This conditional closure applies only to the completed minimum governed runtime slices above.

It does not authorize provider invocation, tool execution, full `ActionRequest` runtime, `ActionResolution` runtime, UI authority, or `project_runtime` expansion.

`F10.9` opens resolution preparation only.

`F10.9` does not authorize execution, create an `ActionResolution` runner, invoke providers, execute tools, mutate files, widen UI authority, or change `project_runtime`.

Objective:

- allow LLM assistance to become a governed runtime concern
- keep tool execution closed
- keep `ActionResolution` runner closed
- keep `EffectiveToolSurface` bounded and non-executing
- keep provider binding optional and phase-gated
- keep UI presentation-only

Allowed minimum runtime slice:

- runtime resolution of `desired_llm_mode -> LLMCapabilityState -> effective_llm_mode -> interaction_mode`
- runtime loading of declarative prompt resources
- runtime filtering of governed context under privacy and consent policy
- runtime construction of bounded `EffectiveToolSurfaceSummary`
- runtime production of assisted-request envelopes, observability metadata, and guided fallback state

This gate does not open:

- broad tool execution
- `ActionResolution` runner
- raw catalog injection
- UI execution authority
- `project_runtime` changes

Provider invocation is not implied by this gate.

The current step-1/step-2 implementation is limited to pure `llm_core` state and envelope preparation only.

It does not introduce provider invocation, tool execution, `ActionResolution` runner behavior, UI authority, or `project_runtime` changes.

Any future provider binding must be phase-gated explicitly and remain optional.

## F10 status after F10.9

F10 minimal runtime: CONDITIONALLY CLOSED.

- `F10.1` to `F10.8`: DONE
- `F10.9`: DONE ^(resolution preparation bridge, non-executing^)

F10 provides:

- capability resolution
- assisted envelope preparation
- tool awareness ^(readonly^)
- proposal generation
- draft action representation
- resolution candidate preparation
- pending action representation
- traceability
- UI-safe state exposure

F10 does not provide:

- provider invocation
- tool execution
- `ActionResolution` runner
- filesystem mutation
- UI-triggered side effects
- `project_runtime` authority changes

Execution boundary remains CLOSED and must only be opened by a dedicated post-F10 phase.

## F11.0 - Execution Opening Gate

Status: DECLARED / NOT ACTIVE.

F10 is closed as a governed non-executing runtime after `F10.9`.

`F11.0` is the only valid post-F10 gate for future governed execution opening.

No execution is authorized by this documentation alone.

Objective:

- define the future conditions under which governed execution may be opened later
- preserve the closed execution boundary until a later explicit F11 implementation slice activates it

Allowed future minimum slice:

- manual human-confirmed execution
- local deterministic single-tool execution
- mandatory `owner_ref`
- mandatory `tool_run_manifest`
- mandatory trace
- explicit lifecycle states
- explicit failure states
- visible UI representation without UI authority

Future minimum execution chain:

`ToolUseProposal / ActionRequestDraft -> ResolutionCandidate -> PendingActionCandidate -> HumanConfirmation -> AuthorizedExecutionRequest -> SingleToolExecution -> ToolRunManifest -> TraceRecord`

Everything from `HumanConfirmation` onward is FUTURE / NOT ACTIVE in `F11.0`.

`F11.0` does not open:

- provider invocation
- LLM autonomous tool calling
- broad tool execution
- multi-tool orchestration
- `ActionResolution` runner
- external binary execution
- document mutation
- `project_runtime` expansion
- UI execution authority

Invariants:

- execution requires explicit human confirmation
- execution requires a non-stale pending candidate
- execution requires a registered and allowed single tool
- execution requires deterministic local execution first
- execution requires `owner_ref` before persistence
- every execution must produce `tool_run_manifest`
- every execution must produce trace metadata
- UI may present execution state but must not execute
- LLM may propose but must not execute
- providers remain closed
- external binaries remain closed unless separately gated
- `project_runtime` remains unchanged unless explicitly opened later

Execution boundary remains CLOSED until a later explicit F11 implementation slice activates it.

## F11.0 audit stabilization note

`F11.0` is declared but not active.

Execution boundary remains CLOSED.

Execution-related terminology in `F11.0` is FUTURE-only.

No runtime capability was introduced by the `F11.0` declaration.

Documentation of future execution paths does not authorize execution.

Advisory-warning increases are expected after `F11.0` declaration.

Those warnings are caused by future execution terminology and do not indicate execution boundary drift.

## F11.1-F11.5 audit stabilization note

The cross-spec audit of the F11.1-F11.5 action governance contracts found no critical findings.

Legacy/base vocabulary in `action_resolution.md` and future UI vocabulary in `pending_action_state.md` are clarified as non-authoritative for the F11.1-F11.5 artifact chain.

Execution boundary remains CLOSED.

`F11.6` may proceed as SPEC-only after this cleanup.

## F11.6 SingleToolExecution stabilization note

`F11.6` declares `SingleToolExecution` as the future minimum governed execution artifact.

This declaration is SPEC-only.

It does not create runner, dispatcher, executor, tool invocation, provider invocation, external binary invocation, filesystem mutation, real outputs, `ToolRunManifest` files, or `TraceRecord` files.

Execution boundary remains CLOSED.

## F11.7 ToolRunManifest and TraceRecord stabilization note

`F11.7` declares future `ToolRunManifest` and `TraceRecord` contract alignment downstream from `SingleToolExecution`.

These artifacts are mandatory for future runtime execution but remain non-created and non-persisted in `F11.7`.

`F11.7` does not create files, folders, outputs, manifests, traces, registry entries, graph entries, or `project_manifest` updates.

Execution boundary remains CLOSED.

## F11.8 TraceRecord ownership cleanup note

`TraceRecord` ownership is clarified: `docs/specs/system_observability.md` owns `TraceRecord` semantics, and `docs/specs/action_resolution.md` references it for downstream execution-chain alignment only.

This cleanup changes no contracts and introduces no runtime behavior.

Execution boundary remains CLOSED.

The system is ready for `action_core` Rust type design after validation.

## F11.10 pre-runtime vocabulary cleanup note

Generic trace status and `TraceRecord.status` are distinct.

Legacy action-resolution vocabulary is non-authoritative for F11 runtime and type design.

No contract semantics changed.

No crates changed.

Execution boundary remains CLOSED.

After validation, the workspace is READY FOR MINIMAL RUNTIME PROPOSAL.

## F12.0 / F11.RUNTIME-0 minimal governed runtime proposal

Status: PROPOSAL / NOT IMPLEMENTED.

`F11` contracts are ready for a future runtime proposal.

`F12.0 / F11.RUNTIME-0` defines the first possible minimal governed runtime opening, but it does not implement that opening.

The proposed first runtime scope is limited to:

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

The only eligible first tool class is an `operational` or `base` deterministic tool.

Recommended first tool: `text.measure`.

Rationale:

- already minimal
- deterministic
- local
- no provider
- no external binary
- low risk
- already aligned with existing governed tool-runtime output discipline

This proposal does not open `merge_pdfs`, external tools, LLM tools, agent tools, graph analysis runtime, RDF/Oxigraph/SPARQL, embeddings, document mutation, or filesystem writes outside owner-scoped sandbox output.

Future execution may occur only if all of the following exist:

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

A future implementation slice must explicitly name:

- crate touched
- exact tool id
- exact input contract
- exact output location policy
- exact manifest path policy
- exact trace path policy
- exact validation command
- exact audit command

Until such a slice is explicitly opened, `SingleToolExecution` remains declared-only, `ToolRunManifest` remains future-required, `TraceRecord` remains future-required, and execution boundary remains CLOSED.

Crate boundary proposal:

- `action_core` owns non-executing contract types only and must not execute
- `tool_runtime` is the likely owner of a future single-tool execution adapter and must remain narrow
- `tool_runtime` must not become a general dispatcher by convenience
- `project_runtime` must not be modified for the first runtime opening unless explicitly justified later
- `project_runtime` must not become execution authority by convenience
- `app_services` may expose prepared summaries later but must not govern runtime
- `ui_*` remains presentation-only and has no execute authority

## F12.0 / F11.RUNTIME-0 invariants

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

## F12.1 Minimal Runtime Gate SPEC

Status: GATE-ONLY / NOT IMPLEMENTED.

`F12.1` defines the exact gate that must be satisfied before a later implementation slice may open the first minimal runtime execution.

Runtime remains CLOSED after `F12.1`.

First eligible tool:

- exact `tool_id`: `text.measure`

Accepted input contract:

- governed text input ref
- no raw payloads
- no secrets
- no private absolute host paths
- ordered input is not required unless a later implementation slice explicitly declares it

Output contract:

- future output file: `result.json`
- owner-scoped location only
- no project-root `outputs/`
- `owner_ref` mandatory

Future `ToolRunManifest` requirements:

- mandatory for future execution
- path policy declared only in `F12.1`
- not created in `F12.1`

Future `TraceRecord` requirements:

- mandatory for future execution
- `trace_ref` mandatory
- not persisted in `F12.1`

Allowed lifecycle placeholders for the gate:

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

## F12.1 Minimal Runtime Gate invariants

- `INV-RUNTIME-GATE-001`: `F12.1` is gate-only.
- `INV-RUNTIME-GATE-002`: runtime remains closed after `F12.1`.
- `INV-RUNTIME-GATE-003`: only `text.measure` may be considered for first implementation.
- `INV-RUNTIME-GATE-004`: `owner_ref` and `trace_ref` are mandatory.
- `INV-RUNTIME-GATE-005`: `ToolRunManifest` and `TraceRecord` are mandatory future artifacts but not created in `F12.1`.
- `INV-RUNTIME-GATE-006`: `project_runtime` must not be modified for the first implementation unless a later explicit audit justifies it.
- `INV-RUNTIME-GATE-007`: `tool_runtime` must not become a general dispatcher.
- `INV-RUNTIME-GATE-008`: UI and `System View` must not execute, authorize, or dispatch.

## F12.2A Minimal Runtime Implementation Plan and Audit Checklist

Status: PLAN-ONLY / AUDIT-CHECKLIST-ONLY / NOT IMPLEMENTED.

`F12.2A` prepares a later `F12.2B` implementation slice for the first minimal governed `text.measure` runtime.

Runtime remains CLOSED after `F12.2A`.

Future crate touched:

- `crates/tool_runtime` only

Explicitly forbidden crates for `F12.2B` unless a later explicit re-audit changes the scope:

- `project_runtime`
- `app_services`
- `ui_core`
- `ui_slint`
- `llm_core`
- `llm_cloud`
- `llm_local`

`action_core` may be consumed only for existing contract references if needed. It must not execute, dispatch, persist, or authorize.

Future `F12.2B` may implement only:

- `text.measure` execution
- a single local deterministic tool adapter
- `owner_ref` enforcement
- `trace_ref` requirement
- owner-scoped output directory
- `result.json` creation
- `tool_run_manifest.json` creation
- `TraceRecord`-compatible metadata creation only if already declared by `F12.1` and `F11.7`
- tests for success and blocked cases

`F12.2B` must not introduce:

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

`F12.2B` acceptance checklist:

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

Mandatory `F12.2B` tests:

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

Mandatory `F12.2B` audit:

- `dev/scripts/audits/audit_f12_minimal_runtime_boundary.bat`

The audit must check:

- no `project_runtime` changes
- no `app_services` or UI execution authority
- no general dispatcher
- no provider calls
- no external binary invocation
- no network access
- no `HOST` write
- only `text.measure` is executable
- `owner_ref` required
- `trace_ref` required
- manifest required
- outputs owner-scoped
- no project-root outputs

## F12.2A implementation plan invariants

- `INV-RUNTIME-IMPL-PLAN-001`: `F12.2A` is plan-only.
- `INV-RUNTIME-IMPL-PLAN-002`: `F12.2A` introduces no runtime.
- `INV-RUNTIME-IMPL-PLAN-003`: `F12.2B` may touch only `tool_runtime` unless explicitly re-audited.
- `INV-RUNTIME-IMPL-PLAN-004`: `F12.2B` must implement only `text.measure`.
- `INV-RUNTIME-IMPL-PLAN-005`: `F12.2B` must not create a general dispatcher.
- `INV-RUNTIME-IMPL-PLAN-006`: `owner_ref` and `trace_ref` are mandatory.
- `INV-RUNTIME-IMPL-PLAN-007`: outputs must be owner-scoped.
- `INV-RUNTIME-IMPL-PLAN-008`: `audit_f12_minimal_runtime_boundary` is mandatory before closure.

## F12.2B Minimal text.measure Runtime Implementation

Status: IMPLEMENTED / MINIMAL RUNTIME OPENING.

`F12.2B` opens only the first governed `text.measure` runtime adapter in `crates/tool_runtime`.

The implementation is limited to:

- exact `tool_id`: `text.measure`
- governed text input ref
- mandatory `owner_ref`
- mandatory `trace_ref`
- local deterministic measurement
- owner-scoped output under `user/output`
- `result.json`
- `tool_run_manifest.json`
- sanitized TraceRecord-compatible metadata inside the manifest

The implementation does not open `project_runtime`, `app_services`, UI execution authority, LLM tool calling, provider calls, network access, external binaries, `merge_pdfs`, agent tools, graph/RDF/semantic runtime, or document mutation.

`F12.2B` remains a single-tool adapter and must not be generalized into a dispatcher without a later governed phase.

## F12.3 Minimal Runtime Closure Audit

Status: AUDITED / HARDENED / CLOSED FOR FIRST SLICE.

`F12.3` audited and hardened the first minimal governed `text.measure` runtime slice.

Closure findings:

- `text.measure` remains the only opened executable tool path.
- `tool_runtime` remains narrow and does not become a broad dispatcher.
- `project_runtime` remains unchanged and does not become execution authority.
- `app_services` and UI layers do not execute, authorize, or dispatch.
- broad runtime, multi-tool orchestration, providers, network access, external binaries, LLM tool calling, agent tools, `merge_pdfs`, graph/RDF/semantic runtime, and document mutation remain closed.
- future file intake remains separate and is not opened by `F12.3`.

## F12.4 Governed File Intake SPEC

Status: SPEC-ONLY / NOT IMPLEMENTED.

`F12.4` defines governed file intake as the formal process by which explicit user-selected external files may later become portable, traceable, project-owned inputs.

`docs/specs/file_intake.md` owns intake semantics.

F12.4 does not copy files, scan host folders, mutate `project_manifest.json`, create registry entries, create graph entries, derive text or chunks, execute tools, add tools, modify runtime behavior, call providers, use network, invoke external binaries, open LLM tool calling, or open semantic/RDF/Oxigraph runtime.

File intake remains governed, not a raw copy operation. Source paths are not portable identity, original host files remain readonly, filesystem presence does not imply project exposure, and `project_manifest.json` remains exposure authority.

Future file intake may use narrow operational/base tools, but `tool_runtime` must not become a general intake orchestrator and `project_runtime` must not be bypassed.

F12.4 permits optional user comments per intake item as governed metadata.

Intake comments are descriptive only. They do not classify files, grant capability, authorize exposure, authorize derivation, or create project visibility.

UI may capture comment text as intent only. Chat may propose comments only through `ChatCommentProposal`; explicit governed promotion is required before any future metadata update.

Comments must be sanitized, trace-linked, project-scoped, and free of secrets, credentials, tokens, raw sensitive data, and private absolute host paths.

## F12.5 file intake implementation plan

Status: PLAN-ONLY / AUDIT-CHECKLIST-ONLY / NOT IMPLEMENTED.

`F12.5` prepares the future `F12.6` minimal governed file intake runtime.

Runtime remains CLOSED after `F12.5`.

Future `F12.6` may only implement explicit user-selected file intake with governed `IntakeItem` metadata, mandatory `owner_ref`, mandatory `trace_ref`, sanitized source labels, optional sanitized `user_comment`, and governed owner/project-scoped storage.

Future `F12.6` must not implement project exposure unless separately opened, generate derivatives, use `tool_runtime` as intake orchestrator, mutate `project_manifest.json`, generate registries, write graph entries, call providers, use network, invoke external binaries, or add UI/app_services authority.

The future audit `dev/scripts/audits/audit_f12_file_intake_boundary.bat` is mandatory before closing `F12.6`.

## F12.5-F12.7 minimal governed file intake baseline closure

Closure state:

- `F12.5`: PLAN-ONLY COMPLETE
- `F12.6`: MINIMAL GOVERNED FILE INTAKE RUNTIME COMPLETE
- `F12.7`: READ-ONLY SYSTEM VIEW VISIBILITY COMPLETE

Baseline preserved:

- no project exposure through `project_manifest.json`
- no registry generation
- no graph writes
- no derivatives
- no `document_text_runtime` invocation
- no `tool_runtime` orchestration
- no UI execution authority
- no LLM/provider/network/external-binary execution

Next allowed expansion candidates remain proposal-only:

1. project exposure gate, defined by `F13.0` as SPEC-only
2. derivatives gate
3. more file formats
4. intake history/index view
5. storage dedup hardening

## F12.8 storage dedup hardening / intake identity strategy

Status: SPEC-ONLY / HARDENING-STRATEGY-ONLY.

`F12.8` defines when two selected files are the same physical artifact and how duplicate imports relate to `content_hash`, `file_ref`, `stored_object_candidate_ref`, and `intake_item_id`.

Two files are the same physical artifact only when governed byte-level content hash matches.

`file_ref` identifies content. `stored_object_candidate_ref` identifies logical intake context. `intake_item_id` identifies the explicit selection event in an intake batch.

Duplicate user selections remain distinct intake items. Physical blob reuse is preferred when hash and policy allow, but logical metadata, comments, owners, traces, and blocking reasons must not be merged silently.

F12.8 does not modify crates, mutate `project_manifest.json`, generate registries, write graph entries, create derivatives, invoke `document_text_runtime`, use `tool_runtime` orchestration, add UI authority, call providers, use network, invoke external binaries, or open LLM execution.

Future tests and audits must prove duplicate selections preserve distinct `IntakeItem` identity, reuse physical blobs only when safe, keep `file_ref` content-based and path-independent, and maintain zero project exposure impact.

## F12.9 intake history/index view

Status: SPEC-ONLY / HISTORY-INDEX-ONLY.

`F12.9` defines a readonly, derivable, rebuildable, non-authoritative intake history/index view over governed file intake metadata.

The history/index may present batch rows, item rows, duplicate/reuse indicators, blocked items, `owner_ref`, `trace_ref`, sanitization state, and sanitized comment previews.

F12.9 does not modify crates, expose files through `project_manifest.json`, generate `registry.json`, write graph entries, create derivatives, invoke `document_text_runtime`, use `tool_runtime`, add UI execution authority, call providers, use network, invoke external binaries, or open LLM execution.

Future tests and audits must prove the history/index can be rebuilt, does not become source truth, preserves distinct `IntakeItem` rows, keeps blocked items visible, and never grants project exposure or runtime authority.

## F13.0 Project Exposure Gate

Status: SPEC-ONLY / GATE-ONLY / NOT IMPLEMENTED.

`F13.0` defines the governed gate for promoting an already `imported_not_exposed` `IntakeItem` or `StoredObject` candidate into project visibility.

The gate requires an exposure request, inert exposure candidate, and explicit human confirmation before any future `project_manifest.json` mutation.

`project_manifest.json` remains the only project exposure authority. Filesystem presence, `file_store` presence, intake metadata, registry entries, graph entries, history/index rows, UI selection, and chat references do not create exposure.

F13.0 does not modify crates, implement runtime, create a manifest writer, generate `registry.json`, write graph entries, create derivatives, call `document_text_runtime`, call `tool_runtime`, add UI authority, expose blocked or unsupported items, infer exposure from storage, or open rollback/revocation.

Future tests and audits must prove the transition `imported_not_exposed -> exposed_to_project` is explicit, confirmed, manifest-governed, duplicate-safe, owner/trace preserving, and document-tree visible only after manifest exposure.

## F13.1 Legacy Exposure/Import Bypass Hardening

`F13.1` is SPEC-only and defines the hardening layer that prevents pre-`F12` or pre-`F13` helpers from becoming an implicit exposure pipeline.

The canonical exposure path remains `F12 intake -> F13 exposure gate -> project_manifest visibility -> document_tree visibility`.

Legacy `import_project_document`, `list_project_documents`, UI direct file promotion, UI direct derivation, and any chat-resource-to-project shortcut are deprecated as production exposure paths and must be blocked before any `F13` runtime implementation starts.

`register_chat_resource` may remain tolerated only as chat-local staging without project exposure authority. `derive_document_text` remains a separate derivative concern and must not run as part of import or exposure.

Future `audit_f13_exposure_boundary.bat` is mandatory before `F13` runtime closure. It must prove there is no implicit exposure via filesystem scanning, copied files, chat-resource registration, direct UI derivation, or `app_services` shadow authority.

## F13.4 Manifest Exposure Contract Hardening

`F13.4` is SPEC-only and closes the future manifest exposure contract without opening runtime.

Governance decisions:

- `exposed_to_project` is the canonical exposure state
- `project_manifest.json` remains the only project exposure authority
- future exposure requires `ExposureRequest`, `ExposureCandidate`, accepted `HumanConfirmation`, and a future `ManifestExposureEntry`
- accepted confirmation is not execution and is not a manifest write
- duplicate blob reuse must remain visible and traceable and must not collapse logical exposure identity
- `registry.json`, graph state, derivatives, document tree presence, and `System View` presence remain non-authoritative for project exposure

Closure note:

- no crates changed by governance requirement
- no runtime introduced
- no manifest writer introduced
- no project exposure implemented
- readiness advances only to future `F13.5` audit or runtime-preparation work

## Minimal contract runtime support (post-F11.0)

Rust contract types now exist for:

- `file_ref`
- `BlobRecord`
- `StoredObject`
- `UsageRef`
- `DerivationManifest`
- `SemanticQuad`
- `QuadInstance`
- `QuadSet`
- `QuadRelation`
- `GraphAnalysis`

These types provide:

- serialization/deserialization
- schema-aligned structure
- deterministic identifiers for `file_ref`
- contract-level validation via tests only

These types do not provide:

- execution
- ingestion
- scanning
- project exposure
- semantic quad generation
- lifecycle transitions
- RDF projection
- graph analysis runtime
- tool execution
- provider invocation

Minimal contract runtime support is CONDITIONALLY CLOSED as non-executing infrastructure.

Execution boundary remains CLOSED.

## F9.5 semantic proposal invariants

The following invariants govern F9.5 semantic proposals. They do not implement LLM execution, embeddings, RDF persistence, Oxigraph, SPARQL, semantic store, or document mutation.

- `INV-SEM-001`: proposal is not fact; every F9.5 semantic output is a `SemanticProposal`, not approved knowledge
- `INV-SEM-002`: no proposal may become `Approved` without explicit human decision
- `INV-SEM-003`: every semantic node must have a stable reference
- `INV-SEM-004`: every proposed relation preserves document/chunk origin, `prompt_ref`, `semantic_spec_ref`, and `trace_ref`
- `INV-SEM-005`: future graph references or N-Quads do not imply RDF persistence
- `INV-SEM-006`: `oxigraph_enabled`, `rdf_export_enabled`, and `semantic_store_enabled` remain false in F9.5
- `INV-SEM-007`: the graph does not decide, approve, execute, or mutate actions
- `INV-SEM-008`: ontology proposes context, not authority; policy, `ActionResolution`, human review, and runtime remain governing boundaries
- `INV-SEM-009`: named graph hints distinguish `domain_knowledge`, `system_governance`, `ai_governance`, `document_governance`, and `lume_policy`
- `INV-SEM-010`: governance may be modeled as knowledge but remains declarative
- `INV-SEM-011`: Lume is `InteractionLayer`, not executor, runtime, tool runner, LLM provider, or filesystem owner
- `INV-SEM-012`: every `SemanticProposal` has lifecycle state
- `INV-SEM-013`: UI represents proposals, relations, and traces; it does not infer, approve, or create facts
- `INV-SEM-014`: the semantic layer consumes governed documents/chunks/refs and does not reinterpret manifests or routes
- `INV-SEM-015`: RDF/Oxigraph preparation must not introduce Oxigraph dependency, store, SPARQL, embeddings, or LLM execution
- `INV-SEM-016`: semantic identity and generation-instance identity must remain separate
- `INV-SEM-017`: semantic quads are proposals by default and are not approved facts
- `INV-SEM-018`: semantic quad evidence must remain traceable to governed sources
- `INV-SEM-019`: semantic quad material must remain separate from deterministic derivatives
- `INV-SEM-020`: future graph, RDF, or analysis consumers may rely only on explicitly approved semantic material
- `INV-SEM-021`: approved semantic knowledge depends on approved quad instances, not semantic identity alone
- `INV-SEM-022`: invalidated evidence must not remain silently approved
- `INV-SEM-023`: semantic relations must remain explicit graph edges rather than embedded hierarchy inside quad structure
- `INV-SEM-024`: approved semantic graph consumption depends on approved relation edges as well as approved quad instances
- `INV-TAG-001`: controlled tags must exist in the catalog
- `INV-TAG-002`: `tag_id` is stable and never reused
- `INV-TAG-003`: tag families must not be mixed
- `INV-TAG-004`: LLM may propose but not activate tags
- `INV-TAG-005`: `ACTIVE` tag transition requires governance approval
- `INV-TAG-006`: `DEPRECATED` tags remain readable
- `INV-TAG-007`: `REMOVED` tags require migration note
- `INV-TAG-008`: documents and objects reference `tag_id`, not local meanings
- `INV-TAG-009`: free-form `system_tags` and `usage_tags` are forbidden
- `INV-TAG-010`: tag-based quads remain proposal-only unless lifecycle-approved semantic flow allows them
- `INV-RDF-001`: RDF is projection, not authority
- `INV-RDF-002`: RDF projection may consume only explicitly approved semantic material
- `INV-RDF-003`: RDF projection must not bypass lifecycle filtering, source traceability, or project exposure governance
- `INV-ANALYSIS-001`: semantic graph analysis may consume only explicitly approved semantic nodes and relations
- `INV-ANALYSIS-002`: semantic graph analysis must not mutate lifecycle, approval, graph authority, or execution authority
- `INV-LAYER-001`: storage is root authority across storage, derivatives, semantic, graph, and analysis
- `INV-LAYER-002`: no semantic writes into derivatives
- `INV-LAYER-003`: graph cannot modify quads
- `INV-LAYER-004`: analysis cannot modify graph
- `INV-LAYER-005`: no reverse mutation is allowed across semantic-layer boundaries
- `INV-LAYER-006`: invalidation flows only from storage to derivatives to semantic to graph
- `INV-LAYER-007`: analysis is read-only evaluation only
- `INV-LAYER-008`: semantic and graph writes remain bounded to their own layers
- `INV-INVALIDATION-001`: approved quads must not remain valid if evidence breaks
- `INV-INVALIDATION-002`: invalidation must be traceable
- `INV-INVALIDATION-003`: invalidation propagates downstream only
- `INV-INVALIDATION-004`: `chunk_hash` change leads to `STALE`
- `INV-INVALIDATION-005`: missing chunk evidence leads to `ORPHANED`
- `INV-INVALIDATION-006`: graph and relations react to upstream invalidation and must not mutate upstream layers
- `INV-INVALIDATION-007`: lazy invalidation is the current declared mode
- `INV-INVALIDATION-008`: eager invalidation is future-only and not active
- `INV-SEC-001`: no secrets may appear in metadata
- `INV-SEC-002`: no secrets may appear in deterministic derivatives
- `INV-SEC-003`: no secrets may appear in semantic quads or relations
- `INV-SEC-004`: no secrets may appear in RDF projections
- `INV-SEC-005`: no secrets may appear in trace records or storage indexes
- `INV-SEC-006`: private absolute host paths must not become portable truth
- `INV-SEC-007`: sanitization must preserve traceability without exposing secret values
- `INV-SEC-008`: hashing, masking, and redaction do not imply authority, approval, or execution permission
- `INV-SEC-009`: semantic and RDF layers must not re-expand previously sanitized material
- `INV-SEC-010`: personal data must be minimized and sanitized when not strictly required for governed traceability
- `INV-GEN-001`: quad generation must not occur without an explicit governed trigger
- `INV-GEN-002`: quad generation is optional and must not be treated as mandatory for all sources
- `INV-GEN-003`: generation scope must remain bounded per governed object, source kind, session, or request
- `INV-GEN-004`: uncontrolled or silent whole-project quad generation is forbidden
- `INV-GEN-005`: generation must be traceable
- `INV-GEN-006`: generation must respect storage-layer and derivative-layer boundaries
- `INV-GEN-007`: generation must respect security and sanitization policy
- `INV-GEN-008`: generated quads remain proposals by default and do not become approved knowledge automatically
- `INV-GEN-009`: `system_suggested` generation remains future-only and non-active unless a later governed runtime opens it
- `INV-GEN-010`: exhaustive generation remains future-only and restricted
- `INV-LIMIT-001`: the semantic layer must remain bounded
- `INV-LIMIT-002`: semantic storage must not grow unbounded by default
- `INV-LIMIT-003`: semantic limits must remain configurable in a future governed runtime
- `INV-LIMIT-004`: soft limits are advisory only in principle and do not imply deletion
- `INV-LIMIT-005`: hard limits remain future-only until explicitly opened
- `INV-LIMIT-006`: pruning strategies are conceptual only in this phase
- `INV-LIMIT-007`: storage-layer authority must not be mutated by semantic limit policy
- `INV-LIMIT-008`: semantic limits must remain aligned with lifecycle, invalidation, and security policy
- `INV-REUSE-001`: duplicate semantic meaning should reuse the same `quad_id` rather than creating a new semantic identity
- `INV-REUSE-002`: reuse is preferred over regeneration when governed identity and context match
- `INV-REUSE-003`: multiple `QuadInstance` records for the same `quad_id` are allowed but must remain explicitly linked
- `INV-REUSE-004`: caching and reuse must not bypass lifecycle state or approval boundaries
- `INV-REUSE-005`: caching and reuse must not bypass invalidation policy
- `INV-REUSE-006`: caching and reuse must not bypass security and sanitization policy
- `INV-REUSE-007`: derivative reuse must remain content-based and deterministic
- `INV-REUSE-008`: governed cache concepts remain declarative only until a later runtime phase explicitly opens them
- `INV-CAP-001`: tools do not imply authorization
- `INV-CAP-002`: capabilities are declarative
- `INV-CAP-003`: capability surface is readonly
- `INV-CAP-004`: future actions reference capabilities before tools
- `INV-CAP-005`: capability declaration does not grant execution
- `INV-CAP-006`: external tools require explicit future dependency governance
- `INV-CAP-007`: agent tools are composite and non-executing unless separately opened
- `INV-CAP-008`: disabled or future capabilities cannot be selected for execution
- `INV-CAP-009`: UI must not authorize capabilities
- `INV-CAP-010`: `project_runtime` remains unchanged unless a future phase explicitly opens authority
- `INV-CAP-011`: `tool_kind` is the canonical governed classification term; `tool_class` is a deprecated alias only if retained in historical references
- `INV-CAP-012`: the canonical domain enum is uppercase across action, capability, and sandbox contracts
- `INV-SANDBOX-001`: `SANDBOX` is the only future writable domain by default
- `INV-SANDBOX-002`: there is no implicit `HOST` access
- `INV-SANDBOX-003`: there is no implicit `EXTERNAL` or network access
- `INV-SANDBOX-004`: `HOST` write is forbidden by default
- `INV-SANDBOX-005`: `EXTERNAL` access requires explicit future capability and gate
- `INV-SANDBOX-006`: capability declaration does not expand domain access
- `INV-SANDBOX-007`: future action constraints must declare domain needs
- `INV-SANDBOX-008`: `project_manifest` remains exposure authority
- `INV-SANDBOX-009`: import and export are future governed operations and are not active behavior
- `INV-SANDBOX-010`: sandbox policy does not activate enforcement runtime
- `INV-SANDBOX-011`: the domain enum is canonical uppercase across governed action, capability, and sandbox contracts
- `INV-ACTION-001`: `ActionIntent` is declarative and non-executing
- `INV-ACTION-002`: `ActionIntent` does not authorize tools, providers, filesystem mutation, or network access
- `INV-ACTION-003`: `ToolUseProposal` remains proposal-only
- `INV-ACTION-004`: `ActionRequestDraft` remains inert
- `INV-ACTION-005`: `capabilities_required` are requirements, not grants
- `INV-ACTION-006`: domain constraints are declarations, not permissions
- `INV-ACTION-007`: ambiguous or unsafe intent must be `BLOCKED`
- `INV-ACTION-008`: `ActionIntent` must preserve traceability
- `INV-ACTION-009`: `ActionIntent` is not `ActionResolution`, `PendingAction`, or `AuthorizedExecutionRequest`
- `INV-ACTION-010`: `project_runtime` remains unchanged
- `INV-ACTION-011`: UI may present future `ActionIntent` but must not authorize or execute it
- `INV-ACTION-012`: no execution boundary is opened by this contract
- `INV-ACTIONREQ-001`: `ActionRequest` is declarative and non-executing
- `INV-ACTIONREQ-002`: `ActionRequest` does not authorize tools, providers, filesystem mutation, network access, or document mutation
- `INV-ACTIONREQ-003`: `ActionRequest` is richer than `ActionIntent` but remains non-executable
- `INV-ACTIONREQ-004`: `ActionRequest` must preserve `intent_ref` and `trace_ref`
- `INV-ACTIONREQ-005`: capability requirements are requirements, not grants
- `INV-ACTIONREQ-006`: domain constraints are declarations, not permissions
- `INV-ACTIONREQ-007`: blocked `ActionRequest` must not auto-fix or execute
- `INV-ACTIONREQ-008`: `READY_FOR_RESOLUTION` does not mean authorized
- `INV-ACTIONREQ-009`: `ActionRequest` is not `ActionResolution`, `PendingActionCandidate`, `AuthorizedExecutionRequest`, or `SingleToolExecution`
- `INV-ACTIONREQ-010`: `ActionRequest` expected outputs do not create files
- `INV-ACTIONREQ-011`: `ActionRequest` inputs must use governed refs, not raw payloads
- `INV-ACTIONREQ-012`: `System View` may present `ActionRequest` summaries but must not control them
- `INV-ACTIONREQ-013`: `project_runtime` remains unchanged
- `INV-ACTIONREQ-014`: execution boundary remains `CLOSED`
- `INV-RESCAND-001`: `ResolutionCandidate` is inert and non-executing
- `INV-RESCAND-002`: `ResolutionCandidate` does not authorize execution
- `INV-RESCAND-003`: `RESOLVABLE` does not mean executable
- `INV-RESCAND-004`: `candidate_tools` are explanatory and not selected
- `INV-RESCAND-005`: capability evaluation does not grant permissions
- `INV-RESCAND-006`: domain evaluation does not grant access
- `INV-RESCAND-007`: blocked or stale candidates must not progress silently
- `INV-RESCAND-008`: `ResolutionCandidate` must preserve `action_request_ref` and `trace_ref`
- `INV-RESCAND-009`: `System View` may present `ResolutionCandidate` but must not control it
- `INV-RESCAND-010`: `F11.2` does not create runner, dispatcher, executor, or `PendingAction` runtime
- `INV-RESCAND-011`: `project_runtime` remains unchanged
- `INV-RESCAND-012`: execution boundary remains `CLOSED`
- `INV-PENDING-CAND-001`: `PendingActionCandidate` is confirmation-preparation only
- `INV-PENDING-CAND-002`: `PendingActionCandidate` is non-executing
- `INV-PENDING-CAND-003`: `READY` does not mean confirmed, authorized, or executable
- `INV-PENDING-CAND-004`: `PendingActionCandidate` must derive from a non-blocked, non-stale `ResolutionCandidate`
- `INV-PENDING-CAND-005`: `PendingActionCandidate` must preserve `resolution_candidate_ref`, `action_request_ref`, `intent_ref`, and `trace_ref`
- `INV-PENDING-CAND-006`: candidate-tool summaries are explanatory and not selected tools
- `INV-PENDING-CAND-007`: expected-output summaries do not create files
- `INV-PENDING-CAND-008`: `HumanConfirmation` remains explicit and non-executing; confirmation runtime remains future
- `INV-PENDING-CAND-009`: future confirmation must re-check staleness
- `INV-PENDING-CAND-010`: `System View` may present `PendingActionCandidate` but must not confirm or control it
- `INV-PENDING-CAND-011`: `F11.3` does not create runtime transition, runner, dispatcher, executor, or confirmation runtime
- `INV-PENDING-CAND-012`: `project_runtime` remains unchanged
- `INV-PENDING-CAND-013`: execution boundary remains `CLOSED`
- `INV-HCONF-001`: `HumanConfirmation` is an explicit human decision event
- `INV-HCONF-002`: `HumanConfirmation` is non-executing
- `INV-HCONF-003`: `ACCEPTED` does not mean authorized or executed
- `INV-HCONF-004`: `HumanConfirmation` must preserve `pending_action_candidate_ref` and `trace_ref`
- `INV-HCONF-005`: accepted confirmation requires non-stale candidate evidence
- `INV-HCONF-006`: `UNKNOWN` staleness must not silently progress to authorization
- `INV-HCONF-007`: risk acknowledgement does not bypass policy
- `INV-HCONF-008`: `reviewer_ref` is not credential authority
- `INV-HCONF-009`: `System View` may present `HumanConfirmation` but must not create or control it
- `INV-HCONF-010`: `F11.4` does not create authorization runtime, runner, dispatcher, executor, or tool execution
- `INV-HCONF-011`: `project_runtime` remains unchanged
- `INV-HCONF-012`: execution boundary remains `CLOSED`
- `INV-AER-001`: `AuthorizedExecutionRequest` is a post-confirmation authorization artifact, not execution
- `INV-AER-002`: `AUTHORIZED_PREPARED` does not mean executing or executed
- `INV-AER-003`: `AuthorizedExecutionRequest` requires `ACCEPTED` `HumanConfirmation`
- `INV-AER-004`: `AuthorizedExecutionRequest` requires `FRESH` staleness result
- `INV-AER-005`: first future execution scope is single-tool, local, deterministic, and `SANDBOX`-scoped
- `INV-AER-006`: `owner_ref`, `tool_run_manifest`, and trace are mandatory before future execution
- `INV-AER-007`: `AuthorizedExecutionRequest` must not create files, folders, manifests, or outputs
- `INV-AER-008`: `AuthorizedExecutionRequest` must not call providers or tools
- `INV-AER-009`: `HOST` write and `EXTERNAL` access remain closed
- `INV-AER-010`: `System View` may present `AuthorizedExecutionRequest` but must not control it
- `INV-AER-011`: `F11.5` does not create runner, dispatcher, executor, or `SingleToolExecution` runtime
- `INV-AER-012`: `project_runtime` remains unchanged
- `INV-AER-013`: execution boundary remains `CLOSED` until a later explicit execution slice
- `INV-STE-001`: `SingleToolExecution` contract is declared-only in `F11.6`
- `INV-STE-002`: `SingleToolExecution` does not implement runner, dispatcher, executor, or tool invocation
- `INV-STE-003`: `SingleToolExecution` must derive from `AuthorizedExecutionRequest`
- `INV-STE-004`: `SingleToolExecution` is limited to one local deterministic tool
- `INV-STE-005`: first future execution scope is `SANDBOX`-only
- `INV-STE-006`: `HOST` write, `EXTERNAL` access, provider invocation, external binary invocation, agent execution, and multi-tool orchestration remain closed
- `INV-STE-007`: input refs must be governed refs, not raw payloads
- `INV-STE-008`: output plan must be owner-scoped and must not create files in this slice
- `INV-STE-009`: `ToolRunManifest` and `TraceRecord` are mandatory for future runtime execution but are not produced in `F11.6`
- `INV-STE-010`: `result_status` remains `NOT_RUN` unless a later explicit runtime slice opens execution
- `INV-STE-011`: `System View` may present `SingleToolExecution` but must not control or execute it
- `INV-STE-012`: `project_runtime` remains unchanged
- `INV-STE-013`: execution boundary remains `CLOSED` after this SPEC-only slice
- `INV-TRM-001`: `ToolRunManifest` is mandatory for future `SingleToolExecution` runtime
- `INV-TRM-002`: `F11.7` does not create `tool_run_manifest.json`
- `INV-TRM-003`: `ToolRunManifest` must link to `SingleToolExecution`, `AuthorizedExecutionRequest`, `ActionRequest`, `intent`, `owner_ref`, and `trace_ref`
- `INV-TRM-004`: `ToolRunManifest` inputs are governed refs, not raw payloads
- `INV-TRM-005`: `ToolRunManifest` outputs must be owner-scoped
- `INV-TRM-006`: `ToolRunManifest` must not contain secrets or private absolute host paths
- `INV-TRM-007`: `ToolRunManifest` configuration is separate from inputs
- `INV-TRM-008`: `ToolRunManifest` status remains non-operational in `F11.7`
- `INV-TRREC-001`: `TraceRecord` is mandatory for future `SingleToolExecution` runtime
- `INV-TRREC-002`: `F11.7` does not persist `TraceRecord`
- `INV-TRREC-003`: `TraceRecord` links artifacts by reference only
- `INV-TRREC-004`: `TraceRecord` must not contain secrets, raw payloads, or private absolute host paths
- `INV-TRREC-005`: `TraceRecord` does not authorize execution
- `INV-TRREC-006`: `TraceRecord` status remains non-operational in `F11.7`
- `INV-MANIFEST-TRACE-001`: `ToolRunManifest` and `TraceRecord` must agree on `trace_ref`
- `INV-MANIFEST-TRACE-002`: `ToolRunManifest` and `TraceRecord` must reference the same `SingleToolExecution`
- `INV-MANIFEST-TRACE-003`: missing `owner_ref`, `trace_ref`, or manifest blocks future execution
- `INV-MANIFEST-TRACE-004`: `F11.7` creates no files, folders, outputs, manifests, traces, registry entries, graph entries, or `project_manifest` updates
- `INV-ID-SYS-008`: `intent_id` and `trace_ref` are governed namespaces and must remain distinct from storage and semantic IDs
- `INV-ID-SYS-009`: `intent_id` and `trace_ref` do not imply authorization, approval, or execution
- `INV-TRACE-001`: `trace_ref` is declarative and non-executing
- `INV-TRACE-002`: `trace_ref` does not authorize execution
- `INV-TRACE-003`: `trace_ref` must not contain secrets or raw sensitive data
- `INV-TRACE-004`: trace links artifacts by reference only
- `INV-TRACE-005`: trace does not imply semantic approval
- `INV-TRACE-006`: trace does not modify lifecycle, graph, storage, or actions
- `INV-SYSTEM-VIEW-001`: `System View` is readonly
- `INV-SYSTEM-VIEW-002`: `System View` does not execute
- `INV-SYSTEM-VIEW-003`: `System View` explains prepared state only
- `INV-SYSTEM-VIEW-004`: UI does not become authority
- `INV-SYSTEM-VIEW-005`: blocked state is explanatory, not failure
- `INV-SYSTEM-VIEW-006`: `System View` does not authorize capabilities or tools
- `INV-SYSTEM-VIEW-007`: `System View` does not mutate storage, semantic, graph, actions, or lifecycle
- `INV-SYSTEM-VIEW-008`: `System View` must not expose secrets or raw payloads
- `INV-SYSTEM-VIEW-009`: Lume interpretation is explanatory only
- `INV-SYSTEM-VIEW-010`: `System View` must remain compatible with Static Mode
- `INV-SYSTEM-VIEW-011`: `System View` state is presentation-only
- `INV-SYSTEM-VIEW-012`: `System View` state must not contain raw payloads
- `INV-SYSTEM-VIEW-013`: `System View` state must not contain secrets
- `INV-SYSTEM-VIEW-014`: `System View` state must not grant authorization
- `INV-SYSTEM-VIEW-015`: `System View` state must be derivable from prepared state
- `INV-SYSTEM-VIEW-016`: UI state must not reinterpret policy
- `INV-SYSTEM-VIEW-017`: `System View` selection must not trigger runtime work
- `INV-SYSTEM-VIEW-018`: visible labels must be represented by i18n keys
- `INV-SYSTEM-VIEW-016`: UI state must not reinterpret policy

## Project folder and artifact graph invariants

The following invariants govern the DocGraph project folder contract. They do not implement runtime behavior, tool execution, graph runtime, RDF, Oxigraph, SPARQL, embeddings, or semantic store behavior.

- `INV-PROJ-LAYOUT-001`: `project_root` MUST NOT contain a root `outputs/` directory
- `INV-PROJ-LAYOUT-004`: filesystem presence MUST NOT imply project exposure
- `INV-PROJ-LAYOUT-005`: project exposure MUST be governed by `project_manifest.json`, not folder scanning
- `INV-TOOL-OUTPUT-001`: every tool output MUST declare `owner_ref`
- `INV-TOOL-OUTPUT-003`: document-contributing outputs MUST live under the owning `DocumentHolder`
- `INV-CHAT-014`: chat artifacts become durable knowledge or document inputs only through explicit promotion
- `INV-DOC-HOLDER-001`: `DocumentHolder` is the owner of document production state
- `INV-GRAPH-001`: `graph/` explains artifact relations; it is not runtime authority
- `INV-GRAPH-004`: graph MUST NOT infer relations from passive filesystem scanning

## Final F9 consistency invariants

The following invariants close the current F9 documentary contract. They do not implement runtime behavior, tools, LLM execution, graph runtime, export runtime, or semantic storage.

- `INV-CONSISTENCY-001`: any artifact referenced by another artifact MUST exist, be resolvable, and be exposed by `project_manifest.json`
- `INV-ID-001`: artifact IDs MUST be stable across moves and renames and MUST NOT depend on filesystem path
- `INV-STORAGE-001`: the same physical file SHOULD NOT be duplicated across `knowledge/files`, chats, or documents; stable references SHOULD be preferred
- `INV-TOOL-DET-001`: every persisted tool run MUST declare inputs, configuration, `owner_ref`, outputs, status, and trace data in `tool_run_manifest.json`

## Future tool execution invariants

The following invariants define future execution discipline for tools. They do not implement tool execution, enable runtime behavior, open F10, or create a runner in the current sandbox.

- `INV-TOOL-INPUT-ORDER-001`: if input order affects the result, the input schema MUST declare `ordered=true`
- `INV-TOOL-INPUT-SOURCE-001`: every tool input MUST declare its source policy: governed reference, explicit user selection, or external intake
- `INV-TOOL-CONFIG-001`: tool configuration MUST be explicitly separated from input data
- `INV-TOOL-ERROR-001`: tool errors MUST be typed and structured, not free text
- `INV-TOOL-RUN-LIFECYCLE-001`: every persisted tool run MUST follow a declared lifecycle state
- `INV-TOOL-RUN-RETRY-001`: persisted tool runs SHOULD be reproducible from recorded inputs and configuration
- `INV-TOOL-DETERMINISM-001`: deterministic tools SHOULD produce the same outputs from the same inputs and configuration

## Project profile and folder sandbox invariants

The following invariants govern declarative project setup and project settings customization. They do not implement popup runtime, sandbox runtime, tool execution, LLM execution, or write-back.

- `INV-PROJECT-PROFILE-001`: every project MUST declare a `project_profile`
- `INV-PROJECT-PROFILE-002`: `project_profile` declares intended capabilities and default policies, not runtime authority
- `INV-PROJECT-PROFILE-003`: changing `project_profile` MUST NOT enable execution by itself
- `INV-FOLDER-SANDBOX-001`: the user-selected source folder MUST be treated as readonly unless a future explicit export or write-back phase is opened
- `INV-FOLDER-SANDBOX-002`: folder organization mutations MUST operate only on the sandbox working copy, never on the original selected source folder
- `INV-FOLDER-SANDBOX-003`: host-specific absolute source paths MUST NOT become portable project truth
- `INV-FOLDER-SANDBOX-004`: LLM output may propose an organization plan, but only governed tools may materialize changes inside the sandbox
- `INV-FOLDER-SANDBOX-005`: any future write-back from sandbox to source folder MUST require explicit user confirmation and trace
- `INV-FOLDER-SANDBOX-006`: sandbox working copy MUST be distinguishable from original source folder in manifests and trace

## LLM tool surface invariants

The following invariants govern future LLM-facing tool surface preparation. They do not implement tool execution, LLM execution, runtime resolution, or F10 behavior.

- `INV-LLM-TOOL-SURFACE-001`: the system SHOULD inject a minimal effective tool surface summary into LLM context
- `INV-LLM-TOOL-SURFACE-002`: the raw full tool catalog MUST NOT be injected into LLM context by default
- `INV-LLM-TOOL-SURFACE-003`: the LLM MAY request expanded tool context through a governed context provider
- `INV-LLM-TOOL-SURFACE-004`: disabled or declared-only tools MAY be surfaced when relevant to explain capability limits
- `INV-LLM-TOOL-SURFACE-005`: effective tool surface MUST be generated by governed policy, not by UI state or LLM guessing
- `INV-LLM-TOOL-SURFACE-006`: `active_only` MUST mean `visible`, allowed in project, execution enabled, and execution implemented
- `INV-LLM-TOOL-SURFACE-007`: `all` returns a governed surface with state and limits, not raw unfiltered catalog data

## Credentials and preferences invariants

The following invariants govern declarative treatment of credentials and preferences. They do not implement secret storage, credential resolution, preferences runtime, LLM execution, or F10 behavior.

- `INV-CRED-001`: credentials MUST NOT be stored in project files
- `INV-CRED-002`: project files MAY store credential references, not secret values
- `INV-CRED-003`: credentials MUST NOT be exposed to LLM context
- `INV-CRED-004`: credentials MUST NOT be written to logs, chats, manifests, graph, `tool_run_manifest`, or exports
- `INV-CRED-005`: credential resolution is future runtime behavior and is not implemented in F9
- `INV-CRED-006`: UI may capture credential intent but must not become credential authority
- `INV-CRED-007`: `credential_ref` is an identifier, not execution permission
- `INV-CRED-008`: changing `credential_ref` MUST NOT enable execution by itself
- `INV-PREF-001`: preferences MUST NOT contain secrets
- `INV-PREF-002`: preferences MAY configure behavior, language, theme, defaults, and help level
- `INV-PREF-003`: preferences MUST NOT grant execution authority
- `INV-PREF-004`: preferences MUST NOT bypass project policy, tool policy, LLM policy, or `ActionResolution`
- `INV-PREF-005`: project preferences MUST remain portable and avoid host-specific absolute paths
- `INV-PREF-006`: preferences may be edited through Project Settings in a future governed flow
- `INV-PREF-007`: preference changes affecting risk or policy require confirmation
- `INV-PREF-008`: preferences must be distinguishable from credentials in schema and storage

## Lume onboarding and optional LLM invariants

The following invariants govern dual-mode Lume onboarding. They do not implement LLM runtime, onboarding runtime, capability-state resolver runtime, or F10 behavior.

- `INV-LUME-ONBOARD-001`: DocGraph MUST remain usable without any LLM engine, model, provider, credential, or executable tool
- `INV-LUME-ONBOARD-002`: Lume MUST support Static Mode based on declarative local help
- `INV-LUME-ONBOARD-003`: Lume MUST distinguish Static Mode from Assisted Mode
- `INV-LUME-ONBOARD-004`: Lume MUST expose capability state instead of failing silently when LLM is unavailable
- `INV-LUME-ONBOARD-005`: missing LLM capability MUST NOT block project creation, project setup, preferences, readonly navigation, or declarative help
- `INV-LUME-ONBOARD-006`: Lume MUST NOT require credentials in Static Mode
- `INV-LUME-ONBOARD-007`: Lume MUST NOT execute tools, mutate files, or approve actions in either mode
- `INV-LUME-ONBOARD-008`: Assisted Mode MUST remain governed by LLM policy, credentials policy, Tool Surface Resolver, and `ActionResolution`

## Lume help tree invariants

The following invariants govern the declarative help tree for Static Mode. They do not implement runtime help routing, tool execution, UI policy interpretation, or F10 behavior.

- `INV-LUME-TREE-001`: `lume_help_tree` MUST support Static Mode help without LLM
- `INV-LUME-TREE-002`: visible help-tree text MUST be referenced through i18n keys, not embedded visible strings
- `INV-LUME-TREE-003`: help-tree nodes MAY describe procedures but MUST NOT execute actions
- `INV-LUME-TREE-004`: help-tree nodes MAY reference GUI objects and tools but MUST NOT grant authority
- `INV-LUME-TREE-005`: UI renders prepared tree state only; it does not interpret policy, execute tools, or resolve permissions
- `INV-LUME-TREE-006`: nodes with `requires_llm=true` MUST remain explanatory when LLM is unavailable
- `INV-LUME-TREE-007`: `executes_action` MUST remain `false` for F9 declarative help-tree nodes

## Future document template proposal invariants

The following invariants are proposals for future governed Markdown document templates. They do not implement runtime, export, editing, LLM calls, tools, or filesystem mutation in the current sandbox.

- `INV-DOC-TEMPLATE-001`: every assisted document creation starts from a declarative template or governed default template
- `INV-DOC-TEMPLATE-002`: templates instantiate Markdown `ARTIFACT` documents
- `INV-DOC-TEMPLATE-003`: Markdown is the authoritative editable source
- `INV-DOC-TEMPLATE-004`: LaTeX, PDF, and DOCX are derived and regenerable outputs
- `INV-DOC-TEMPLATE-005`: every document created from a template preserves `template_id` and `template_snapshot`
- `INV-DOC-TEMPLATE-006`: the local template snapshot is not destructively modified
- `INV-DOC-TEMPLATE-007`: local modifications are recorded as typed overrides
- `INV-DOC-TEMPLATE-008`: `effective_template` is calculated from snapshot plus overrides
- `INV-DOC-TEMPLATE-009`: the user may choose `strict`, `guided`, or `free_from_template`
- `INV-DOC-TEMPLATE-010`: style changes do not degrade structural conformance
- `INV-DOC-TEMPLATE-011`: drift is classified by structure, style, metadata, export, and semantics
- `INV-DOC-TEMPLATE-012`: templates may include editorial guidance per section
- `INV-DOC-TEMPLATE-013`: guidance is declarative; it does not execute, mutate, or decide policy
- `INV-DOC-TEMPLATE-014`: template observations are not exported unless explicitly declared
- `INV-DOC-TEMPLATE-015`: future export uses `document.md`, `effective_template`, reference data, and export profile, not UI state

## Future document reference proposal invariants

The following invariants are proposals for future governed document references and bibliography. They do not implement parsers, importers, export, LLM calls, tools, runtime behavior, or document mutation.

- `INV-DOC-REF-001`: document references must be structured entities, not opaque plain text
- `INV-DOC-REF-002`: templates may declare citation and reference style
- `INV-DOC-REF-003`: original active-project documents may be referenced as internal sources, evidence, or appendices without mutation
- `INV-DOC-REF-004`: BibTeX files may be bibliographic knowledge resources when declared or located in governed surfaces
- `INV-DOC-REF-005`: future BibTeX import creates structured bibliography entries; it does not convert BibTeX into a mutable document
- `INV-DOC-REF-006`: future export resolves references from structured entries plus template style, not UI heuristics
- `INV-DOC-REF-007`: Markdown keeps symbolic or structured references; LaTeX and DOCX materialize final format
- `INV-DOC-REF-008`: referenced `SOURCE` and `DERIVED` remain readonly
- `INV-DOC-REF-009`: templates may define numeric, alphabetic, symbolic, or standard bibliographic labeling
- `INV-DOC-REF-010`: templates may define the position of the references block in layout
- `INV-DOC-REF-011`: citation patterns must be declarative and derivable from `reference_style`
- `INV-DOC-REF-012`: non-academic reference styles must be supported
- `INV-DOC-REF-013`: label generation such as `a)`, `b)`, `c)` is future system responsibility, not user responsibility
- `INV-DOC-REF-014`: standard and corporate styles must be supported as declarative resources
- `INV-DOC-REF-015`: corporate styles must not be hardcoded in code or UI
- `INV-DOC-REF-016`: templates may select a corporate or standard `reference_style_id`
- `INV-DOC-REF-017`: company styles may define labeling, placement, citation patterns, and listing rules
- `INV-DOC-REF-018`: standard and corporate styles may coexist but must be explicitly identified

## Lume Help invariants

- `INV-LUME-HELP-001`: Lume Help is contextual help, not runtime
- `INV-LUME-HELP-002`: Lume Help is based on governed declarative sources or i18n/help resources
- `INV-LUME-HELP-003`: Lume Help does not execute, mutate, call LLMs, or approve actions
- `INV-LUME-HELP-004`: the help popup is ephemeral and does not create project data
- `INV-LUME-HELP-005`: visible text must come from i18n/resources, not hardcoded Slint views
- `INV-LUME-HELP-006`: Lume Help must use `GUI_OBJECTS_v1` canonical names for interface explanations

## Interaction invariants

- `INV-CHAT-001`: chat references, not blobs
- `INV-CHAT-002`: existing documents are referenced from the tree
- `INV-CHAT-003`: clip is for external intake and workflow initiation
- `INV-CHAT-004`: profiling is a workspace workflow
- `INV-CHAT-005`: viewer remains readonly
- `INV-CHAT-006`: no duplicate source truth is hidden inside chat
- `INV-CHAT-007`: attachment intent must be explicit
- `INV-CHAT-008`: clip-driven tool launch is operational only
- `INV-CHAT-009`: multi-file selection is controlled and intention-scoped
- `INV-CHAT-010`: project and runtime boundaries remain intact

## Future governed document editing proposal

The following invariants are proposals for a future governed editing phase. They are not implemented in the current CLOSED F8 sandbox.

- `INV-DOC-EDIT-001`: only `ARTIFACT` documents may be modified through governed patches
- `INV-DOC-EDIT-002`: `SOURCE` and `DERIVED` documents are readonly
- `INV-DOC-EDIT-003`: the user does not directly edit documents
- `INV-DOC-EDIT-004`: the LLM proposes; the user validates; the runtime applies
- `INV-DOC-EDIT-005`: every accepted modification generates a version or `patch_log`
- `INV-DOC-EDIT-006`: the active tab defines the default context
- `INV-DOC-EDIT-007`: ambiguous active targets require confirmation
- `INV-DOC-EDIT-008`: viewer and inspectors are selection and inspection surfaces, not free editors
- `INV-DOC-EDIT-009`: `SOURCE` and `DERIVED` documents may provide context but must not be mutated
- `INV-DOC-EDIT-010`: patches must be traceable, reversible, and deterministically applicable

## Consequences for contributors

- crate responsibilities must stay explicit
- pipeline duplication is a governance failure, not a style issue
- presentation concerns must not widen governed vertical outputs
- docs and snapshot artifacts must be updated when the implementation crosses a real phase boundary
- migration decisions that tension inherited doctrine must be stated explicitly

## F11.1-F11.8 action_core contract runtime support

`crates/action_core` now contains non-executing Rust contract types for the `F11.1` to `F11.8` action governance chain.

This support is serialization-ready contract infrastructure only. It does not execute, resolve, dispatch, authorize, persist, or mutate.

Execution boundary remains CLOSED.
