# Functional Scope

This document states what the sandbox currently implements, what it is actively expanding, and what remains outside the current scope.

## Implemented today

- multi-crate Rust workspace with governed architectural boundaries
- portable workspace and path discipline
- declarative loading of project-facing specs and registries
- explicit project pipeline in `project_runtime`
- typed project runtime output and minimal observability
- thin `app_services` consumption of the project vertical
- clean `cli_app` consumption of project and tool boundaries
- declarative `tool_runtime` for catalogs, entries, selection, validation, and minimal accepted dispatch
- regenerable document text derivation with manifest, optional pages, and deterministic chunks
- structural UI shell
- explicit UI controllers and manifest wiring
- tabbed workspace content area
- technical readonly viewer over resolved targets as one workspace tab
- minimal knowledge panel over project `knowledge/` documents
- governed document tree for existing project documents
- structured chat messages with document references, tool results, and system state
- clip-driven external intake and document workflow launch
- workspace document profiling flow over primary documents and derived text
- tools panel with separated manual operational launch and declarative LLM tool policy
- Lume Help contextual popup as declarative help, not runtime
- Pipeline / Ontology View as readonly/mock F9.5 presentation state
- local validation tooling through workspace wrappers

## Current phase status

`F8` is CLOSED.

`F10` minimal governed runtime is CONDITIONALLY CLOSED.

Closed phases:

- `3A`
- `3B`
- `F4A`
- `F4B`
- `F5`
- `F6`
- `F7`
- `F8`

## Planned next phases

- `F10`: LLM chat integration with governed tools
- `F11`: final audit and CLOSED state verification

These are roadmap intentions beyond the declarative F9/F10_PREP layer already reached.

## F10 opening gate: first minimal slice

The first F10 opening gate is limited to a minimum governed LLM-assisted runtime slice.

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

This closure does not authorize execution.

That slice may open runtime responsibility for:

- capability-state resolution
- effective mode resolution
- consent-aware context filtering
- declarative prompt loading
- bounded `EffectiveToolSurfaceSummary`
- guided fallback and assisted-request preparation
- inert proposal/draft-to-resolution-candidate preparation
- inert `ActionRequest`-to-`ResolutionCandidate` evaluation preparation
- inert pending-candidate preparation with trace and blocking metadata

That slice does not open:

- broad tool execution
- `ActionResolution` runner
- raw full catalog injection
- provider-binding by implication
- UI execution authority
- provider invocation
- filesystem mutation

## F9-ready declarations

The following declarations are reached and available declaratively in `F9/F10_PREP`.

They do not open F10 runtime.

- product identity: application `DocGraph`, assistant `Lume`
- Lume interaction and onboarding resources
- `GUI_OBJECTS_v1` canonical GUI vocabulary for Lume Help explanations
- Lume Help canonical names for `Document Tree`, `Clip Panel`, `Workspace Tabs`, `Readonly Viewer`, `Chat Panel`, `Tools Panel`, `Knowledge Panel`, `Pipeline View`, `Ontology Proposal View`, and `Lume Help`
- Lume i18n message resources
- app preferences and transparent user profile placeholders
- runtime location declarations and missing Tectonic placeholder
- LLM engine/model catalogs with real execution disabled
- tools metacatalog and initial internal/external catalogs
- authorized local sandbox policy with mutable actions disabled
- file-based storage policy with checksum deduplication and regenerable JSONL indexes
- governed file-store and `StoredObject` JSON schemas as declarative validation contracts only
- action policy and flow-control policy with execution disabled
- conceptual pending action state documentation
- ActionResolution specification as declarative request-to-trace governance
- F9.5 AI governance and `SemanticProposal` declarations; `SemanticProposal` remains proposal, not fact
- semantic quad identity, evidence, quadset, and lifecycle-preparation contracts declared only
- semantic classification tag catalog and lifecycle contracts declared only
- semantic layer boundary policy declared only
- security and sanitization policy declared only
- quad generation policy declared only
- semantic storage limits policy declared only
- caching and reuse policy declared only
- tool capability model declared only
- sandbox boundary model declared only
- action contract declared only
- rich ActionRequest contract declared only
- rich ResolutionCandidate contract declared only
- rich PendingActionCandidate contract declared only
- HumanConfirmation contract declared only
- AuthorizedExecutionRequest contract declared only
- SingleToolExecution contract declared only
- system observability declared only
- System View declared only
- System View readonly UI state model declared only
- System View effective tool surface summary consumption declared only
- RDF projection policy declared only
- graph analysis policy declared only
- semantic quad, relation, quadset, lifecycle, and graph-analysis schemas declared only
- semantic tag catalog schema declared only
- Pipeline / Ontology View declared as readonly/mock
- F10 remains not opened

## F9 reached

F9 is reached declaratively.

This means the documentary contract, declarative resources, and bounded prepared-state specs are available.

It does not mean F10 runtime behavior is opened.

It also does not mean schema presence activates runtime validation, project exposure, or execution authority.

- declarative project folder layout
- artifact graph contract
- `GUI_OBJECTS_v1`
- Lume Help canonical vocabulary
- ActionPolicy / FlowControlPolicy / PendingActionState declarations
- ActionResolution specification without runner
- SemanticProposal declaration
- Pipeline / Ontology readonly/mock presentation
- DocumentHolder proposal alignment
- `owner_ref` governance
- no project-root `outputs/`
- project profile proposal documented
- credentials and preferences policy documented
- Lume dual onboarding model documented
- optional LLM capability state documented
- F9 validation resources passing
- tools/catalog governance declaratively closed
- `tools_master_catalog.json` documented as declarative `MasterCatalog`
- `resources/tool_runtime/*` retained as current operative source
- `EffectiveToolSurfaceResolver` documented as future-only
- one minimal operational tool persists governed output
- `text.measure` persists `result.json` plus `tool_run_manifest.json`
- the minimum persisted tool slice enforces owner-scoped output under `user/output/tool_runs/<owner-kind>/<owner-id>/<run_id>/`
- an inert `ActionRequest` contract type may exist without runner, dispatcher, executor, or mutation path
- default-build cloud provider execution remains disabled

## F9 not reached / not implemented yet

- no broad operational tool execution beyond existing minimal boundary
- no `merge_pdfs` implementation
- no broad runtime `owner_ref` enforcement beyond the existing minimum persisted slice
- no broad persisted `tool_run_manifest.json` generation beyond the existing minimum persisted slice
- no `registry.json` generator
- no graph writer
- no graph runtime
- no passive filesystem graph scanning
- no chat artifact promotion runtime
- no setup popup runtime
- no settings popup runtime
- no folder sandbox runtime
- no write-back to external source folders
- no credential store runtime
- no execution-authorizing credential resolution
- no document creation runtime
- no document export runtime
- no ActionRequest runner, dispatcher, executor, or mutation path
- no full ActionRequest runtime
- no ActionResolution runner
- no executable proposal-to-resolution bridge
- no Lume Assisted runtime
- no LLM availability resolver runtime
- no capability state runtime beyond the minimal pure resolver in `llm_core`
- no assisted-chat runtime beyond the minimal pure envelope preparation in `llm_core`
- no onboarding runtime beyond existing declared surfaces
- no LLM execution in the default build
- no LLM tool calling
- no live provider validation
- no RDF, Oxigraph, SPARQL, embeddings, or semantic store
- no semantic quad generation runtime
- no semantic quad review or approval runtime
- no semantic relation graph runtime
- no approved semantic fact runtime
- no RDF projection runtime
- no graph analysis runtime
- no `EffectiveToolSurfaceResolver` implementation
- no operative consumption of `tools_master_catalog.json`

## F9 exit criteria before F10

- one minimal operational tool can persist a governed output
- every persisted tool output has `owner_ref`
- every persisted tool run has `tool_run_manifest.json`
- the output owner is chat, DocumentHolder, or knowledge, and the persisted output path is owner-scoped
- `registry.json` remains derivable and non-authoritative
- graph updates are either not implemented or are written only by governed actions
- no UI layer owns tool execution logic
- `cargo_check` and `cargo_test` pass
- `validate_f9_declarations` and `validate_ai_specs` pass
- `llm_context_bundle` regenerates with zero warnings

## F10 step-1 preconditions

The first F10 opening gate should require:

- declarative F9/F10_PREP specs closed and consistent
- explicit capability-state contract present
- explicit privacy and consent contract present
- explicit bounded `EffectiveToolSurface` contract present
- `ActionResolution` still declarative and runner-free
- no broad tool execution beyond the existing minimum F9 operational boundary
- validation wrappers passing

## F10 execution boundary

F10 produces governed action intentions.

F10 does not execute actions.

Execution remains explicitly out of scope.

## F11.0 declared execution gate

`F11.0` is DECLARED / NOT ACTIVE.

`F11.0` prepares:

- the future governed execution opening gate
- the future minimum execution chain
- the future distinction between confirmable and executable states
- the future requirement for `owner_ref`, `tool_run_manifest`, and trace

Future minimum execution slice:

`ToolUseProposal / ActionRequestDraft -> ResolutionCandidate -> PendingActionCandidate -> HumanConfirmation -> AuthorizedExecutionRequest -> SingleToolExecution -> ToolRunManifest -> TraceRecord`

What remains out of scope:

- provider invocation
- tool execution
- `ActionResolution` runner
- external binary execution
- document mutation
- UI execution authority
- `project_runtime` expansion

Execution remains CLOSED until a later explicit F11 implementation slice activates it.

## F11.0 interpretation note

The system remains non-executing.

Confirmation is not execution.

Execution lifecycle elements are not active.

No component is allowed to perform side effects.

## F11.1-F11.5 audit stabilization note

The F11.1-F11.5 action-governance audit found no critical findings.

Legacy/base resolution vocabulary and future UI pending-action vocabulary are clarified as non-authoritative for the F11.1-F11.5 artifact chain.

Execution boundary remains CLOSED.

`F11.6` may proceed as SPEC-only after this cleanup.

## F11.6 SingleToolExecution scope note

`SingleToolExecution` is declared as the future minimum governed execution artifact only.

It does not implement runtime execution, runner, dispatcher, executor, tool invocation, provider invocation, external binary invocation, filesystem mutation, real outputs, `ToolRunManifest` production, or `TraceRecord` persistence.

Execution boundary remains CLOSED.

## F11.7 ToolRunManifest and TraceRecord scope note

`ToolRunManifest` and `TraceRecord` are declared as mandatory future downstream artifacts for `SingleToolExecution` only.

`F11.7` does not implement manifest creation, trace persistence, filesystem mutation, registry updates, graph updates, or `project_manifest` updates.

Execution boundary remains CLOSED.

## F11.8 TraceRecord ownership cleanup note

`TraceRecord` ownership is clarified in `docs/specs/system_observability.md`.

This cleanup changes no contracts, introduces no runtime behavior, and keeps execution CLOSED.

The system is ready for `action_core` Rust type design after validation.

## F11.10 pre-runtime vocabulary cleanup note

Generic trace status and `TraceRecord.status` are distinct.

Legacy action-resolution vocabulary is non-authoritative for F11 runtime and type design.

No contract semantics changed.

No crates changed.

Execution boundary remains CLOSED.

After validation, the workspace is READY FOR MINIMAL RUNTIME PROPOSAL.

## F12.0 / F11.RUNTIME-0 minimal governed runtime proposal

`F12.0 / F11.RUNTIME-0` is PROPOSAL / NOT IMPLEMENTED.

It defines the first possible runtime opening after F11 contracts, but runtime remains closed until a later explicit implementation slice.

Proposed first runtime scope:

- one local deterministic `SANDBOX`-only tool
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

Recommended first tool: `text.measure`.

Reason:

- already minimal
- deterministic
- local
- no provider
- no external binary
- low risk
- aligned with existing governed output discipline

Future execution would require:

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

This proposal creates no runner, dispatcher, executor, outputs, manifests, traces, provider calls, external binary invocation, or filesystem mutation.

## F11.1-F11.8 action_core contract runtime support

`crates/action_core` now provides non-executing Rust contract types for:

- `ActionRequest`
- `ResolutionCandidate`
- `PendingActionCandidate`
- `HumanConfirmation`
- `AuthorizedExecutionRequest`
- `SingleToolExecution`
- `ToolRunManifest`
- `TraceRecord`

This is contract runtime support only. It does not open execution, persistence, authorization, provider calls, or filesystem mutation.

## Minimal contract runtime support

Minimal Rust contract support for storage and semantic schemas now exists as non-executing infrastructure.

This support covers:

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

This closes the preparation layer required before future `F11` execution slices, while keeping the system non-executing.

No execution boundary has been opened.

## Recommended next implementation slice

Future proposal only. Do not treat this as implemented F9 behavior.

MODE: ARQUITECTO  
SCOPE: F12.0 / F11.RUNTIME-0

First candidate:

- minimal `text.measure` runtime proposal
- explicit `owner_ref`
- explicit `trace_ref`
- declared `ToolRunManifest` requirement
- declared `TraceRecord` requirement
- owner-scoped output plan
- no LLM
- no external binary
- no graph runtime
- no UI logic

`merge_pdfs` remains future and is not the first proposed runtime opening.

## F12.1 Minimal Runtime Gate SPEC

`F12.1` is GATE-ONLY / NOT IMPLEMENTED.

Runtime remains CLOSED after this gate.

First eligible tool:

- `text.measure`

Gate contract:

- accepted input is a governed text input ref
- raw payloads are forbidden
- secrets are forbidden
- private absolute host paths are forbidden
- ordered input is not required unless explicitly declared later
- output is future `result.json`
- output location must be owner-scoped
- project-root `outputs/` remains forbidden
- `owner_ref` is mandatory
- `trace_ref` is mandatory
- `ToolRunManifest` is mandatory for future execution but not created in `F12.1`
- `TraceRecord` is mandatory for future execution but not persisted in `F12.1`

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

The later implementation audit must verify no `project_runtime` expansion, no broad dispatcher, and explicit `owner_ref`, `trace_ref`, and manifest requirements.

## F12.2A implementation plan and audit checklist

`F12.2A` is SPEC-ONLY / PLAN-ONLY / AUDIT-CHECKLIST-ONLY.

Runtime remains CLOSED after `F12.2A`.

Future `F12.2B` implementation scope:

- future crate touched: `crates/tool_runtime` only
- only `text.measure` may be implemented
- only a single local deterministic adapter may be introduced
- `owner_ref` and `trace_ref` are mandatory
- output must be owner-scoped
- future success may create `result.json`
- future success may create `tool_run_manifest.json`
- future `TraceRecord`-compatible metadata may be created only as already declared by `F12.1` and `F11.7`

Explicitly forbidden for `F12.2B`:

- `project_runtime`
- `app_services`
- `ui_core`
- `ui_slint`
- `llm_core`
- `llm_cloud`
- `llm_local`
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

Mandatory `F12.2B` checks:

- `tool_id` is exactly `text.measure`
- governed text input only
- no raw payload persistence
- missing `owner_ref` blocks execution
- missing `trace_ref` blocks execution
- unsafe input blocks execution
- stale input blocks execution if staleness is present
- no project-root outputs
- manifest contains no private absolute host paths
- result, manifest, and trace metadata contain no secrets
- `tool_runtime` does not become a general dispatcher
- `project_runtime` remains unchanged

Mandatory audit before closure:

- `dev/scripts/audits/audit_f12_minimal_runtime_boundary.bat`

## F12.2B minimal text.measure runtime

`F12.2B` is IMPLEMENTED as the first minimal governed runtime slice.

Implemented scope:

- crate touched: `crates/tool_runtime`
- exact executable tool: `text.measure`
- governed text input ref only
- mandatory `owner_ref`
- mandatory `trace_ref`
- owner-scoped output under `user/output`
- `result.json`
- `tool_run_manifest.json`
- sanitized TraceRecord-compatible metadata

Still closed:

- `project_runtime`
- `app_services`
- `ui_core`
- `ui_slint`
- `llm_core`
- `llm_cloud`
- `llm_local`
- general dispatcher
- multi-tool runner
- providers
- network
- external binaries
- `HOST` write
- LLM tools
- agent tools
- `merge_pdfs`
- graph, RDF, or semantic runtime
- document mutation
- UI-triggered execution

## F12.3 minimal runtime closure audit

`F12.3` audited and hardened the first minimal governed `text.measure` runtime slice.

Closure status:

- `text.measure` remains the only opened executable tool path.
- `tool_runtime` remains narrow.
- `project_runtime` remains unchanged.
- UI and `app_services` do not execute.
- broad runtime remains closed.
- future file intake remains separate and is not opened by `F12.3`.

## F12.4 governed file intake SPEC

`F12.4` is SPEC-ONLY and not implemented.

It defines governed file intake in `docs/specs/file_intake.md` as the contract for turning explicit user-selected external files into future portable, traceable, project-owned inputs.

F12.4 opens no runtime. It does not copy files, scan host folders, mutate `project_manifest.json`, create registry or graph entries, derive text/chunks, execute tools, add tools, modify `tool_runtime` or `project_runtime`, or add UI/app_services authority.

Future file intake remains separate from the closed `text.measure` runtime slice and from future document derivation runtime.

F12.4 also defines optional per-file intake comments as governed metadata.

Comment scope:

- comments are tied to `IntakeItem`
- comments are optional and empty by default
- comments are descriptive only
- comments do not classify, expose, authorize, derive, or register files
- UI may capture comment text but must not persist it
- chat may propose comment additions or edits but must not persist metadata directly
- explicit governed promotion is required before a chat proposal can update `FileIntakeDraft` or `IntakeItem`
- comments must be sanitized, project-scoped, and trace-linked
- comments must not contain secrets, credentials, tokens, raw sensitive data, or private absolute host paths

## F12.5 file intake implementation plan

`F12.5` is PLAN-ONLY / AUDIT-CHECKLIST-ONLY and not implemented.

It prepares a future `F12.6` minimal governed file intake runtime without opening runtime in this slice.

Future `F12.6` scope is limited to explicit user-selected files, one intake batch, governed `IntakeItem` metadata, source existence/readability validation, directory rejection, safe extension/basic-kind classification, optional hash if existing support allows it, governed storage only when the storage path is explicit, mandatory `owner_ref`, mandatory `trace_ref`, and optional sanitized `user_comment`.

Still closed after `F12.5`:

- crates changes
- runtime implementation
- project exposure
- `project_manifest.json` mutation
- registry generation
- graph writes
- derivatives
- `tool_runtime` orchestration
- UI/app_services authority
- providers, network, external binaries, LLM tools, and semantic/RDF/Oxigraph runtime

`dev/scripts/audits/audit_f12_file_intake_boundary.bat` is mandatory before any future `F12.6` closure.

## F12.5-F12.7 minimal governed file intake baseline closure

`F12.5`, `F12.6`, and `F12.7` are now closed as the minimal governed file intake baseline.

This baseline includes explicit user-selected intake batches, minimal governed intake runtime, sanitized metadata, optional sanitized comments, preserved `imported_not_exposed` state, blocked-item visibility, and readonly `System View` presentation.

Still out of scope:

- project exposure
- registry generation
- graph writes
- derivatives
- `document_text_runtime`
- `tool_runtime` orchestration
- UI execution authority
- LLM/provider/network/external-binary execution

Future expansion candidates remain proposal-only:

1. project exposure gate, defined by `F13.0` as SPEC-only
2. derivatives gate
3. more file formats
4. intake history/index view
5. storage dedup hardening

## F12.8 storage dedup hardening / intake identity strategy

`F12.8` is SPEC-ONLY / HARDENING-STRATEGY-ONLY.

It defines duplicate and identity strategy for the minimal file intake baseline without changing crates or runtime behavior.

Functional meaning:

- same physical artifact means same governed byte-level content hash
- duplicate user selections remain distinct `IntakeItem` records
- `content_hash` is hash evidence
- `file_ref` is content identity
- `stored_object_candidate_ref` is logical stored-object context
- `intake_item_id` is explicit batch item identity
- physical blob reuse is preferred when safe
- new logical references may be created for distinct intake context
- project exposure remains closed and controlled only by `project_manifest`

Still out of scope:

- `project_manifest.json` mutation
- `registry.json` generation
- graph writes
- derivatives
- `document_text_runtime`
- `tool_runtime` orchestration
- UI execution authority
- LLM/provider/network/external-binary execution

## F12.9 intake history/index view

`F12.9` is SPEC-ONLY / HISTORY-INDEX-ONLY.

It defines a future readonly, derivable, rebuildable, non-authoritative intake history/index view for file intake batches and items.

Functional meaning:

- batch history may show intake batch refs, item counts, blocked/imported summaries, owner refs, trace refs, and sanitized comment previews
- item history may show sanitized source labels, item status, duplicate/reuse indicators, blocked reasons, owner refs, trace refs, and sanitized comment previews
- duplicate/reuse visibility is explanatory only and must not collapse distinct `IntakeItem` rows
- blocked item visibility is explanatory only and must not authorize retry or runtime work
- history/index state may be rebuilt from governed intake metadata and must not become source truth

Still out of scope:

- project exposure
- `project_manifest.json` mutation
- `registry.json` generation
- graph writes
- derivatives
- `document_text_runtime`
- `tool_runtime`
- UI execution authority
- providers, network, external binaries, and LLM execution

## F13.0 Project Exposure Gate

`F13.0` is SPEC-ONLY / GATE-ONLY and not implemented.

It defines the future governed path for promoting an already imported `IntakeItem` or `StoredObject` candidate from `imported_not_exposed` to `exposed_to_project`.

Functional meaning:

- exposure begins with an explicit exposure request
- an inert exposure candidate checks refs, support state, blocked state, sanitization, duplicate policy, `owner_ref`, and `trace_ref`
- explicit human confirmation is mandatory before any future manifest mutation
- `project_manifest.json` is the only project exposure authority
- document tree visibility may occur only after manifest-governed exposure
- registry remains derivable and non-authoritative
- graph remains optional and future-only
- rollback and revocation remain future-only

Still out of scope:

- runtime implementation
- crates changes
- manifest writer
- registry generator
- graph writes
- derivatives
- `document_text_runtime`
- `tool_runtime`
- UI execution or confirmation authority
- blocked or unsupported item exposure
- automatic exposure from intake or `file_store` presence

## F13.1 Legacy Exposure/Import Bypass Hardening

`F13.1` is SPEC-only / AUDIT-PLAN and not implemented.

Functional meaning:

- the only canonical project exposure path is `F12` intake followed by `F13` exposure gate
- legacy import, tree scan, chat-resource registration, and direct derivation paths must not serve as production exposure paths
- UI and `app_services` must not shadow import, exposure, confirmation, or manifest authority
- derivatives remain separate and must not be triggered by exposure

STOP before future `F13` runtime:

- any production use of `import_project_document` as exposure
- any production use of `list_project_documents` as tree authority
- any UI-triggered direct derivation from external or newly copied files
- any chat-resource shortcut that behaves like intake or exposure
- absence of `audit_f13_exposure_boundary.bat`

## F13.4 Manifest Exposure Contract Hardening

`F13.4` is SPEC-only and not implemented.

Functional meaning:

- closes the exact future contract for promoting `imported_not_exposed` artifacts into manifest-governed project visibility
- makes `exposed_to_project` the canonical exposure state
- requires request, candidate, accepted confirmation, and a future manifest entry before project exposure exists
- clarifies that duplicate blob reuse does not collapse logical exposure identity
- keeps `System View`, `Document Tree`, registry, graph, and trace visibility non-authoritative until a later explicit runtime slice writes manifest authority

Still out of scope:

- runtime implementation
- crates changes
- manifest writer
- registry generator
- graph writes
- derivatives
- `document_text_runtime`
- `tool_runtime`
- UI or `app_services` exposure authority
- project exposure itself

## Future proposals

- governed document editing workflow
- patch proposal, preview, acceptance, rejection, and regeneration workflow
- structured content assets for figures, tables, and equations
- active-context model driven by the active workspace tab

## Not included yet

- production UI behavior
- production product runtime behavior for `DocGraph`
- notebook or blocks workspace
- editing, drafting, or rich document mutation
- governed document patch runtime
- direct mutation of artifacts from chat or LLM output
- figure, table, or equation inspectors as functional editing surfaces
- complex layout management, docking, or multi-window workspace behavior
- knowledge graph or semantic knowledge workflows
- embeddings, graph storage, and retrieval layers over derived text
- semantic proposal approval runtime
- RDF persistence, Oxigraph runtime, N-Quads output, or SPARQL execution
- broad tool execution behavior
- integrated LLM runtime behavior across the application
- real LLM model or engine execution
- external binary execution
- mutable local sandbox actions
- pending action runtime
- action request execution
- ActionResolution runner or enforcement runtime
- runtime installation or binary download
- rich tool orchestration, history, or agent-side workflow editing
- broad service orchestration
- full product-runtime parity with the Python application
