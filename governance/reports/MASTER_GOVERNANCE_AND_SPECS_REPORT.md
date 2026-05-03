# Master Governance And Specs Report

Generated from live sandbox documents.

- Workspace root: `C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\dev\scripts\..\..`
- Generated at: `02/05/2026 16:40:33,61`

---

## Governance and architecture documents

### README.md

# DocGraph Rust Portable App Sandbox

This repository is the governed Rust sandbox for `DocGraph`, the visible application identity for the portable application migration.

It is a multi-crate workspace that advances the Rust implementation in controlled phases while the existing Python application remains the historical doctrinal and behavioral migration reference.

Python is not part of the active Rust runtime and is not required to run the sandbox.

## Product identity

- application display name: `DocGraph`
- assistant display name: `Lume`
- identity scope: declarative documentation and governed resources only
- `DocGraph` remains a governed Rust sandbox, not a final production runtime
- `Lume` remains an interaction and help layer without filesystem, tool, LLM, or pipeline execution authority

## Current state

- sandbox workspace, not production runtime
- architecture governed by phases
- `3A`, `3B`, `F4A`, `F4B`, `F5`, `F6`, `F7`, and `F8` CLOSED
- primary governed vertical: `project_runtime`
- higher layers already present:
  - thin `app_services`
  - clean `cli_app`
  - minimal declarative `tool_runtime`
  - UI shell + controllers + workspace tabs + technical readonly viewer
  - tools panel with separated `Operational Tools` and `LLM Tools` surfaces
  - Lume Help contextual popup as governed help, not runtime
  - `GUI_OBJECTS_v1` canonical GUI glossary for Lume Help explanations
  - F9.5 AI governance resources and readonly/mock Pipeline / Ontology View

## Central pipeline

The governed project pipeline is:

`project_root -> manifest -> contract -> validation -> surface -> resolve -> output`

That pipeline is owned by `project_runtime`. Higher layers consume it; they do not reimplement it.

## Repository structure

Repository structure:

- `crates/` -> runtime implementation
- `resources/` -> declarative data
- `docs/specs/` -> system specifications
- `governance/` -> rules, policies, invariants, and phase scope
- `architecture/` -> system design, migration mapping, and structural references
- `dev/scripts/` -> validation and tooling
- `AGENTS.md` -> agent interaction contract
- `user/output/` -> generated operational artifacts such as status snapshots
- `assets/` -> migration staging area, not canonical runtime resources
- `fixtures/` -> test fixtures and controlled sample inputs

## Main crates

- `workspace_core`: portable workspace boundaries and path discipline
- `spec_runtime`: declarative loading for specs, config, and registries
- `project_runtime`: governed project vertical and central pipeline
- `app_services`: thin consumer layer over governed verticals
- `tool_runtime`: declarative catalog and minimal accepted runner boundary
- `io_runtime`: controlled IO boundaries and persisted resource access
- `cli_app`: thin local entrypoint
- `ui_core`, `ui_slint`: structural UI shell, controllers, viewer, and knowledge presentation

LLM crates also exist in the workspace, but they remain intentionally narrow and are not yet integrated into a broad product workflow.

## Current UI/runtime maturity

The sandbox already includes:

- a thin application-service boundary over the governed project pipeline
- a clean CLI consumer for project and tool flows
- a structural UI shell
- explicit UI controllers and manifest wiring
- a tabbed workspace content area
- a technical readonly viewer over resolved targets as one workspace tab
- a minimal knowledge panel over project `knowledge/` documents
- a governed document tree for existing project documents
- structured chat references plus clip-driven external intake and document workflow launch
- a governed tools panel with:
  - controlled manual launch for `Operational Tools`
  - global plus project override policy for `LLM Tools`
- Lume Help with canonical GUI vocabulary from `GUI_OBJECTS_v1`

Canonical GUI names include `Document Tree`, `Clip Panel`, `Workspace Tabs`, `Readonly Viewer`, `Chat Panel`, `Tools Panel`, `Knowledge Panel`, `Pipeline View`, `Ontology Proposal View`, and `Lume Help`.

Lume Help uses these names to avoid ambiguous spatial explanations. It remains contextual help only and does not execute runtime, LLM, tools, actions, semantic workflows, or filesystem mutation.

The sandbox does not yet include:

- production UI behavior
- notebook or block workspace
- editing or rich drafting surfaces
- complex docking or multi-window layout behavior
- real knowledge graph or semantic knowledge workflows
- semantic proposal approval, RDF persistence, Oxigraph, SPARQL, embeddings, or semantic store
- full tool execution behavior
- broad LLM workflow integration
- wide LLM tool orchestration
- full end-user product parity

## Local validation

This sandbox runs in a local Windows environment. Rust validation must be executed through wrappers in `dev/scripts/`.

Read `dev/scripts/SCRIPTS_INDEX.md` before using them.

Available entry points:

- `dev\scripts\cargo_check.bat`
- `dev\scripts\cargo_test.bat`
- `dev\scripts\cargo_all.bat`
- `dev\scripts\cargo_fmt.bat`
- `dev\scripts\cargo_clippy.bat`
- `dev\scripts\cargo_strict.bat`

Default validation after relevant Rust changes:

- `dev\scripts\cargo_all.bat`

Stricter validation before higher-risk closure:

- `dev\scripts\cargo_strict.bat`

## Roadmap by phases

- `3A`: minimal project runtime baseline, closed
- `3B`: explicit pipeline, runtime output, observability, closed
- `F4A`: thin `app_services`, closed
- `F4B`: clean `cli_app` consumption, closed
- `F5`: UI structural shell, closed
- `F6`: controllers plus manifest wiring, closed
- `F7`: technical readonly viewer, closed
- `F8`: knowledge panel and documentary workspace model, CLOSED
- `F9`: preferences / credentials plus F9.5 declarative/mock AI governance preparation; tools/catalog governance is declaratively closed
- `F10`: LLM chat integration with tools, not opened
- `F11`: final audit / CLOSED state verification

F9 remains declarative only.

- `resources/tools/tools_master_catalog.json` remains a declarative master catalog
- `resources/tool_runtime/*` remains the current operative runtime source
- `EffectiveToolSurfaceResolver` remains a future proposal only
- no real tool, LLM, or external binary execution is opened by F9 documentation

The roadmap remains sequential. The workspace is not trying to open all subsystems at once.

## Maturity disclaimer

This repository is not yet a final application runtime.

It is a governed engineering workspace intended to:

- preserve inherited doctrine from the Python system
- turn thin specs into executable Rust contracts
- validate architectural direction in small, auditable steps

It does not yet provide:

- stable external API commitments
- production UI or runtime behavior
- full integrated tool execution
- full integrated LLM runtime behavior
- semantic runtime, embeddings, RDF persistence, SPARQL, or Oxigraph execution

## Key documents

- `architecture/ARCHITECTURE.md`
- `governance/GOVERNANCE.md`
- `governance/FUNCTIONAL_SCOPE.md`
- `architecture/MIGRATION_BASELINE.md`
- `governance/WORKSPACE_RULES.md`
- `user/output/rust_status_snapshot.md`
- `docs/ENGINEERING_NOTES.md`

---

### !REL_FILE!

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

---

### !REL_FILE!

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

---

### !REL_FILE!

# I18N Policy

This sandbox inherits the Python rule that user-visible UI text must be externalized and governed outside views.

Supported UI languages:

- English (`en`)
- Spanish (`es`)

## Rules

- user-visible UI strings must not be hardcoded in views
- localized resources must live outside presentation code
- localized resources are consumed through `ui_i18n`
- the fallback order is: preferred language -> English -> controlled fallback
- all user-visible menu labels MUST use i18n keys
- all user-visible popup titles and popup descriptions MUST use i18n keys
- `Slint` views and other presentation views MUST NOT hardcode visible text
- every governed key in `resources/i18n/en/*.ftl` MUST have an equivalent key in `resources/i18n/es/*.ftl`
- every governed key in `resources/i18n/es/*.ftl` MUST have an equivalent key in `resources/i18n/en/*.ftl`
- internal identifiers MUST remain English and stable
- translated labels MUST NOT become runtime authority, identifiers, policy decisions, or execution triggers

## Naming convention

Governed visible-text keys must use stable domain prefixes:

- `menu.*`
- `popup.*`
- `dialog.*`
- `tooltip.*`
- `status.*`
- `help.*`

These prefixes classify visible text only.

They do not define runtime behavior, authority, routing, or execution semantics.

## Language distinctions

- UI language: language of menus, dialogs, labels, and general interface text
- chat language: language used in conversational interaction
- internal control language: implementation and control-layer language used for system logic and stable identifiers

These are related but not identical concerns and must not be conflated.

This sandbox includes placeholder Fluent resources only.

## Governed menu and popup contract

The top-level menu domains and governed popup surfaces must remain label-driven by i18n resources rather than by view-local literals.

This includes:

- top-level menu labels
- menu entry labels
- governed popup titles
- governed popup descriptions
- generic dialog button labels
- governed tooltips for visible UI chrome

Internal object names, popup ids, controller ids, and policy ids remain language-independent and stable even when their user-facing labels are translated.

Deprecated i18n keys may remain temporarily for compatibility or migration safety, but new or updated specs must not reference them as active contract keys.

## Invariants

- `INV-I18N-001`: all visible menu labels MUST come from i18n resources.
- `INV-I18N-002`: all visible popup titles and descriptions MUST come from i18n resources.
- `INV-I18N-003`: English and Spanish i18n files MUST remain key-symmetric for governed menus and popups.
- `INV-I18N-004`: translated labels MUST NOT become identifiers or runtime authority.
- `INV-I18N-005`: internal identifiers MUST remain stable and language-independent.
- `INV-I18N-006`: no visible UI text may be hardcoded in `Slint` or presentation views.
- `INV-I18N-007`: deprecated i18n keys MUST NOT be used by new specs.

---

### !REL_FILE!

# Inherited Governance

This file records how live Python governance is transferred into the Rust sandbox.

If a requested Python source name is not physically present in this sandbox, the transfer is anchored to the active local equivalent and marked accordingly.

| Principle | Python Source | Rust Adaptation | Status |
| --- | --- | --- | --- |
| system/dev/user separation | `OPERATIONAL_DEFINITION_Rev09.txt`, `INVARIANTS_GUIDE.md`, `DEV-DOMAIN-ARCHITECTURE.md`, `WORKSPACE_STRUCTURE_EXPLAINED.md` | Rust sandbox remains in `dev/` during migration and future Rust runtime preserves the same domain separation | inherited |
| portability and host independence | `OPERATIONAL_DEFINITION_Rev09.txt`, `PORTABLE_RUNTIME_BASELINE.md` | Rust successor must remain portable and must not depend on host-specific runtime assumptions | inherited |
| declarative-first architecture | `OPERATIONAL_DEFINITION_Rev09.txt`, `GOBERNANCE_INDEX.json`, active specs/contracts | `spec_runtime` becomes the entry point for declared specs, config, and registry loading | inherited |
| traceability and reproducibility | `OPERATIONAL_DEFINITION_Rev09.txt`, `INVARIANTS_GUIDE.md`, `LLM_OBSERVABILITY_GUIDE.md`, `CONTRATO_UI_CORE.json` | Rust runtime and service layers must preserve structured traceability; concrete event emission is deferred | adapted |
| project as primary working unit | `OPERATIONAL_DEFINITION_Rev09.txt`, `project_manifest_exposure_contract.json` | `project_runtime` centers project-scoped operations and manifest-governed visibility | inherited |
| workflow-centric runtime model | `OPERATIONAL_DEFINITION_Rev09.txt` | Rust baseline preserves workflow-centric intent but defers concrete workflow execution logic | deferred |
| manifest/ref-driven visibility | `project_manifest_exposure_contract.json`, `system/utils/project/*`, `UI_LAYOUT_Rev08.md` | `project_runtime` inherits validation -> ref resolution -> neutral surface model flow | inherited |
| artifact/output separation | `OPERATIONAL_DEFINITION_Rev09.txt`, `WORKSPACE_STRUCTURE_EXPLAINED.md`, project-domain runtime model | Rust project/runtime docs preserve separate concerns for artifacts, outputs, and runs | adapted |
| UI as representation, not source of truth | `CONTRATO_UI_CORE.json`, `UI_LAYOUT_Rev08.md` | `ui_core` and `ui_slint` remain representation-only layers over services/runtime | inherited |
| no filesystem/runtime logic in views | `CONTRATO_UI_CORE.json`, `UI_LAYOUT_Rev08.md`, `INVARIANTS_GUIDE.md` | Slint is pure presentation and cannot own runtime or filesystem behavior | inherited |
| externalized user-visible UI strings | `ui_strings.json`, `ui_strings_contract.json` | `ui_i18n` and `resources/i18n/` own user-visible text resources | inherited |
| token-driven theming | `ui_theme.json`, `ui_theme_contract.json` | `ui_theme` and `resources/themes/` define semantic tokens for light/dark modes | adapted |
| local/cloud/auto LLM modes | `LLM_CONFIGURATION_GUIDE.md`, `governance/LLM_RUNTIME_POLICY.md` | `llm_core` defines modes and policy while adapters remain separated | inherited |
| deterministic runtime resolution | `LLM_CONFIGURATION_GUIDE.md`, `governance/LLM_RUNTIME_POLICY.md` | Rust LLM runtime planning must resolve mode/provider/model deterministically before invocation | inherited |
| model catalog governance | `LLM_MODEL_CATALOG_GUIDE.md` | Rust LLM execution must validate models against a governed catalog once execution is implemented | inherited |
| tools requested by model, executed by system | `LLM_TOOL_INTEGRATION_GUIDE.md`, `llm_tools_contract.json` | `llm_core` governs request policy and `tool_runtime` or `app_services` execute tools | inherited |
| observability as a required subsystem concern | `LLM_OBSERVABILITY_GUIDE.md`, `CONTRATO_UI_CORE.json` | Structured observability is mandatory for future Rust execution, but concrete events are deferred | adapted |
| no provider-driven architecture | `governance/LLM_RUNTIME_POLICY.md`, `LLM_CONFIGURATION_GUIDE.md`, `DEV-DOMAIN-ARCHITECTURE.md` | adapters stay behind `llm_core`; application code does not couple directly to providers | inherited |
| dev domain as engineering laboratory | `DEV-DOMAIN-ARCHITECTURE.md` | this Rust workspace remains a governed dev-only migration environment until elevated by future governance | inherited |
| automated auditability | `INVARIANTS_GUIDE.md`; requested `AUDIT_ENGINE_SPEC_Rev00.txt` not present locally | Rust docs preserve audit-ready structural rules; a Rust audit engine contract is deferred | deferred |

---

### !REL_FILE!

# LLM Runtime Policy

This sandbox inherits the live Python LLM doctrine.

Supported execution modes:

- `local`
- `cloud`
- `auto`

## Deterministic resolution

The runtime layer must resolve an execution plan before invocation.

Inherited precedence:

`explicit args -> user preferences -> system config -> defaults`

No runtime ambiguity is allowed.

## Layer responsibilities

- `llm_core` defines contracts, policy, mode selection, model catalog rules, tool request governance, and observability requirements
- `llm_local` implements local adapters behind `llm_core`
- `llm_cloud` implements cloud adapters behind `llm_core`

## Required doctrine

- application code must not depend directly on a provider
- providers stay behind governed adapter crates
- model catalog governance is mandatory once real execution starts
- tools may be requested by the model, but they are executed by the system
- runtime policy must remain observable and traceable
- the runtime layer must not execute tools directly
- the runtime layer must not contain UI logic

## Observability baseline

Real execution is deferred, but the inherited requirement is already fixed:

- enough structured metadata must exist to reconstruct provider, model, mode, tool policy, and outcome

---

### !REL_FILE!

# UI Slint Policy

This sandbox inherits the active Python UI doctrine that presentation is not the source of truth.

Slint is the active presentation technology for this Rust sandbox shell.

## Rules

- business logic must not live inside Slint views
- filesystem logic must not live inside Slint views
- runtime logic must not live inside Slint views
- Slint must consume i18n and theme layers rather than reimplement them
- `ui_slint` consumes `ui_core`, `ui_i18n`, `ui_theme`, and thin controller-facing runtime boundaries without becoming their owner
- `app_services` must remain independent from Slint widgets
- Slint must not execute tools or external workflows directly
- Slint must not couple directly to LLM providers
- Slint routes and renders; controllers decide; governed runtime layers remain authoritative
- the viewer remains readonly regardless of whether a workflow starts from tree or clip
- workspace tabs are controlled content views, not miniapps or a parallel runtime
- Lume onboarding/help surfaces may be presented by Slint, but their visible text must come from i18n resources
- Slint may represent `GUI_OBJECTS_v1` inside Lume Help, but canonical GUI names and visible help text must come from `resources/help/gui_objects.json` and i18n resources
- Slint must not invent GUI object names, hardcode GUI help text, interpret Lume Help, or execute anything through Lume Help
- Slint must not execute Lume, tools, LLM workflows, external binaries, or sandbox filesystem actions directly
- the DocGraph icon may be rendered in top chrome
- its click handler may only emit `OpenLumeHelpRequested` or equivalent navigation intent
- UI must not hardcode runtime paths or execute logic from the icon
- icon visible text or tooltip must come from i18n

## Current scope

- live shell window with explicit controllers
- governed document tree for existing project documents
- explicit clip surface for document intake and workflow launch
- structured chat surface for text, document references, tool results, and state messages
- tabbed workspace content area
- technical readonly viewer as one workspace tab
- knowledge panel integrated into the shell
- tools panel integrated into the shell
- Lume Help GUI Objects section as readonly presentation of canonical names
- no direct product-runtime ownership

## Current role boundaries

- Slint presents the shell and routes user intent into controllers
- controllers translate UI intent into governed runtime calls and presentation state
- tree actions reference and open existing governed documents; they do not import them again
- clip actions intake external files or launch explicit workflows; they do not replace the tree
- chat stores references and results; it does not become a hidden document store
- workspace tabs host controlled views; they do not define new domain pipelines
- the viewer verifies resolved content readonly; it does not become an editor or extractor

## Future governed editing proposal

In a later phase, Slint may capture viewer selection, expose a contextual instruction popup, and render patch previews or accept/reject/regenerate choices.

Slint must not:

- generate patches
- validate patches
- apply patches
- modify the filesystem
- execute LLM calls directly
- execute tools directly

The future UI role is to collect intent and render governed state supplied by controllers or runtime boundaries.

---

### !REL_FILE!

# UI Theme Policy

This sandbox inherits the rule that UI appearance must be resource-driven and token-based.

Supported appearance modes:

- `light`
- `dark`

## Rules

- UI views must not hardcode colors
- only semantic theme tokens may be consumed by presentation code
- theme resources are loaded through `ui_theme`
- appearance is independent from UI language
- theme fallback must resolve through controlled resources rather than ad hoc defaults

## Fallback policy

- preferred theme resource
- default theme resource for the active mode
- controlled failure or diagnostic if required semantic tokens are missing

---

### !REL_FILE!

# Workspace Rules

## Structural rules

- one primary responsibility per crate
- keep dependency direction controlled and inward
- do not create parallel implementations of the same governed flow
- keep edge crates thin
- keep UI crates free of business logic, filesystem ownership, and runtime ownership
- do not couple application code directly to a provider-specific LLM implementation

## Workspace and path discipline

- workspace-relative paths are logical portable paths, not OS-rendered strings
- compare absolute paths robustly; do not depend on platform-specific textual rendering
- fix path policy in base crates rather than patching each higher crate separately
- persist portable relative paths instead of absolute host-specific paths wherever the contract requires it

## Validation discipline

- use the Rust workspace as the center of implementation
- mechanical validation runs through `dev/scripts/` wrappers in the local Windows environment
- read `dev/scripts/SCRIPTS_INDEX.md` before using wrappers
- default validation after relevant Rust changes: `dev\scripts\cargo_all.bat`
- stricter validation before higher-risk closure: `dev\scripts\cargo_strict.bat`

## Current layering rules

- `project_runtime` owns the project pipeline
- `app_services` consumes governed verticals and does not replace them
- `cli_app` stays an entrypoint, not a domain orchestrator
- `tool_runtime` stays minimal and separate from project and knowledge concerns until a later phase explicitly opens more
- `ui_slint` consumes controllers and presentation state; it does not own filesystem or domain policy
- the central workspace area is tabbed; each tab is a controlled view, not an autonomous miniapp
- the readonly viewer is one workspace tab, not the whole workspace model
- the document tree is the reference surface for existing governed documents already inside the system
- the clip is the explicit surface for external intake and document workflow launch
- the chat layer carries structured references and results; it does not become a second document store
- document profiling belongs to workspace tabs and thin controllers; it does not move into `project_runtime`
- the knowledge layer consumes existing viewer and controller boundaries; it does not reinterpret the manifest
- the tools surface stays split between controlled operational launch and declarative LLM tool policy
- project-level LLM tool overrides must remain declarative and intersect with the global allowed surface
- visible product identity is `DocGraph` and assistant identity is `Lume`; this is declarative identity, not runtime authority
- Lume is an interaction and help layer; it does not execute filesystem, tools, or LLM workflows
- Lume Help is contextual governed help; it does not execute, mutate, approve actions, call LLMs, persist history, or create project data
- Lume Help must use `GUI_OBJECTS_v1` canonical names when explaining interface objects
- GUI object explanations should name the object explicitly and avoid vague spatial references without the canonical object name
- visible UI strings must come from governed resources and i18n, not hardcoded Slint views
- `resources/` declares; `runtime/` materializes placeholders; `user/` stores user-owned preferences and data
- external runtime tools remain missing or disabled until a later phase explicitly opens validation and execution
- local sandbox mutation remains disabled until authorized policy and logging are implemented
- action and flow-control policies remain declarative in F9; they do not execute or approve runtime behavior
- ActionResolution remains declarative in F9; it does not create a runner, invoke LLM/tools, or mutate files
- UI may present pending actions in a future phase, but every execution must reference an explicit `action_id`
- document templates remain declarative proposals until a later phase opens template/runtime behavior
- future document export must use package data and export profiles, not UI state
- reference styles remain declarative resources under `resources/reference_styles/`; corporate styles must not be hardcoded in code or UI
- BibTeX and bibliography handling remain proposals; no parser, importer, or export materialization exists yet
- F9.5 semantic proposals remain proposals, not facts; approval requires future explicit human review
- Pipeline / Ontology View is readonly/mock representation; it does not infer, approve, execute, persist RDF, run SPARQL, or mutate files
- Oxigraph/RDF preparation remains disabled: no Oxigraph dependency, embeddings, semantic store, or N-Quads output is created
- project folders must not contain a root `outputs/` directory; outputs belong to the functional owner
- every future tool output must carry an `owner_ref`
- tool outputs live under the owning chat, DocumentHolder, or knowledge derivation/proposal area
- `graph/` explains artifact relations; it does not govern, execute, approve, or replace `project_runtime`
- `registry.json` is a navigation accelerator; it does not replace `project_manifest.json` or `graph/`
- filesystem presence must not imply project exposure; `project_manifest.json` governs exposure
- chat attachments are session context; durable knowledge or document inputs require explicit promotion
- `DocumentHolder` owns document production state; `document.md` is its only editable textual source
- graph entries must originate from governed actions and remain traceable to document, chat, knowledge, or tool origins
- referenced artifacts must exist, resolve, and be exposed by `project_manifest.json`
- artifact IDs must remain stable across moves and renames and must not depend on filesystem path
- prefer stable references over duplicating the same physical file across knowledge, chats, or documents
- persisted tool runs must declare inputs, configuration, `owner_ref`, outputs, status, and trace data in `tool_run_manifest.json`
- ordered inputs must declare `ordered=true` when order affects result
- tool configuration must not be mixed with input data
- persisted tool runs require structured lifecycle, structured errors/warnings, and reproducible inputs/configuration
- UI may collect ordered input lists but must not execute or validate domain logic
- UI may capture `ProjectSetupDraft` and `ProjectSettingsDraft`, but it does not govern project configuration authority
- project profiles declare intention, not runtime authority
- selected external folders are readonly; sandbox mutations may affect only a future working copy
- LLM receives an effective tool surface summary by default, not raw catalogs
- expanded tool context must come from a governed resolver or provider, not from UI state or LLM guessing
- preferences are non-secret configuration
- credentials are referenced, not embedded
- project files must not contain secret values
- LLM context must not receive credentials
- changing preferences or `credential_ref` does not enable execution
- the app must remain usable without LLM
- Lume Static Mode uses declarative help
- Lume help-tree content is declarative navigation, explanation, and procedure state
- missing LLM or tools is visible capability state, not failure
- project setup must not depend on LLM availability
- UI renders prepared help-tree state only and does not infer policy or execute actions
- the main menu remains minimal and stable: `File`, `Preferences`, `Tools`, `Help`
- Tools is a top-level menu domain separate from Preferences and exposes separate governed entries for Operational Tools, LLM Tools, and External Dependencies
- visible menu text must come from i18n resources
- the bottom status bar displays prepared state only
- the bottom status bar exposes no execution, no secrets, and no policy inference
- the bottom status bar may provide navigation intents only
- the top-left DocGraph icon is branding plus a Lume Help shortcut
- clicking the DocGraph icon must not execute tools, LLM, project actions, filesystem mutations, or settings changes
- the central layout remains Tree + Chat + Tabs Workspace
- viewer belongs to Tabs Workspace
- Sandbox View may appear as a conditional tab only when relevant
- Sandbox View is informational in F9 and must not execute filesystem operations

## Migration rules

- Inherited doctrine remains reference only
- Rust is the active implementation target in this sandbox
- implementation growth must follow documented crate boundaries
- governance-relevant changes must be reflected in root docs
- placeholder code is acceptable only when it does not fake implemented product behavior

## Candidate rules

- reusable runtime policies should not remain in an entrypoint once they clearly belong to a governed vertical
- critical steps in the controlled runtime path should not remain outside minimal structured observability
- higher layers should adapt around governed outputs instead of widening them for presentation or cross-layer convenience
- knowledge navigation should remain readonly and should not grow into editing, semantics, or notebook behavior before the corresponding phase opens
- viewer and inspectors are not free editors
- `SOURCE` and `DERIVED` documents remain readonly
- `ARTIFACT` documents may be edited only through governed patch acceptance
- patch validation and application must not live in UI crates
- chat may reference edit requests, selections, and proposals, but it must not store source blobs or act as a document store
- document template guidance is editorial and declarative; it must not execute, mutate, or decide policy
- document references must remain structured proposals; referenced `SOURCE` and `DERIVED` stay readonly
- semantic proposal UI must represent declarations and traces only; no hidden semantic inference belongs in UI
- GUI object names are shared help vocabulary, not runtime authority

## F12.4 file intake boundary

- `docs/specs/file_intake.md` owns governed file intake semantics.
- File intake is SPEC-only in F12.4.
- Source host paths are not portable identity.
- Original host files remain readonly.
- Filesystem presence does not imply project exposure.
- `project_manifest.json` remains exposure authority.
- UI and `app_services` must not mutate storage, import files, derive text, or expose intake items.
- `assets/` must not be used as file-intake runtime storage.

## F12.5 file intake planning boundary

- `F12.5` is plan-only and audit-checklist-only.
- `F12.5` introduces no runtime and touches no crates.
- Future `F12.6` must use explicit user-selected files only.
- Future `F12.6` must keep source folders readonly.
- Future `F12.6` must not persist private absolute host paths as portable truth.
- Future `F12.6` must not mutate `project_manifest.json`, generate registries, write graph entries, or generate derivatives unless a later phase explicitly opens those scopes.
- Future `audit_f12_file_intake_boundary.bat` is mandatory before closure.

---

### !REL_FILE!

# Master Governance And Specs Report

Generated from live sandbox documents.

- Workspace root: `C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\dev\scripts\..\..`
- Generated at: `02/05/2026 16:40:33,61`

---

## Governance and architecture documents

### README.md

# DocGraph Rust Portable App Sandbox

This repository is the governed Rust sandbox for `DocGraph`, the visible application identity for the portable application migration.

It is a multi-crate workspace that advances the Rust implementation in controlled phases while the existing Python application remains the historical doctrinal and behavioral migration reference.

Python is not part of the active Rust runtime and is not required to run the sandbox.

## Product identity

- application display name: `DocGraph`
- assistant display name: `Lume`
- identity scope: declarative documentation and governed resources only
- `DocGraph` remains a governed Rust sandbox, not a final production runtime
- `Lume` remains an interaction and help layer without filesystem, tool, LLM, or pipeline execution authority

## Current state

- sandbox workspace, not production runtime
- architecture governed by phases
- `3A`, `3B`, `F4A`, `F4B`, `F5`, `F6`, `F7`, and `F8` CLOSED
- primary governed vertical: `project_runtime`
- higher layers already present:
  - thin `app_services`
  - clean `cli_app`
  - minimal declarative `tool_runtime`
  - UI shell + controllers + workspace tabs + technical readonly viewer
  - tools panel with separated `Operational Tools` and `LLM Tools` surfaces
  - Lume Help contextual popup as governed help, not runtime
  - `GUI_OBJECTS_v1` canonical GUI glossary for Lume Help explanations
  - F9.5 AI governance resources and readonly/mock Pipeline / Ontology View

## Central pipeline

The governed project pipeline is:

`project_root -> manifest -> contract -> validation -> surface -> resolve -> output`

That pipeline is owned by `project_runtime`. Higher layers consume it; they do not reimplement it.

## Repository structure

Repository structure:

- `crates/` -> runtime implementation
- `resources/` -> declarative data
- `docs/specs/` -> system specifications
- `governance/` -> rules, policies, invariants, and phase scope
- `architecture/` -> system design, migration mapping, and structural references
- `dev/scripts/` -> validation and tooling
- `AGENTS.md` -> agent interaction contract
- `user/output/` -> generated operational artifacts such as status snapshots
- `assets/` -> migration staging area, not canonical runtime resources
- `fixtures/` -> test fixtures and controlled sample inputs

## Main crates

- `workspace_core`: portable workspace boundaries and path discipline
- `spec_runtime`: declarative loading for specs, config, and registries
- `project_runtime`: governed project vertical and central pipeline
- `app_services`: thin consumer layer over governed verticals
- `tool_runtime`: declarative catalog and minimal accepted runner boundary
- `io_runtime`: controlled IO boundaries and persisted resource access
- `cli_app`: thin local entrypoint
- `ui_core`, `ui_slint`: structural UI shell, controllers, viewer, and knowledge presentation

LLM crates also exist in the workspace, but they remain intentionally narrow and are not yet integrated into a broad product workflow.

## Current UI/runtime maturity

The sandbox already includes:

- a thin application-service boundary over the governed project pipeline
- a clean CLI consumer for project and tool flows
- a structural UI shell
- explicit UI controllers and manifest wiring
- a tabbed workspace content area
- a technical readonly viewer over resolved targets as one workspace tab
- a minimal knowledge panel over project `knowledge/` documents
- a governed document tree for existing project documents
- structured chat references plus clip-driven external intake and document workflow launch
- a governed tools panel with:
  - controlled manual launch for `Operational Tools`
  - global plus project override policy for `LLM Tools`
- Lume Help with canonical GUI vocabulary from `GUI_OBJECTS_v1`

Canonical GUI names include `Document Tree`, `Clip Panel`, `Workspace Tabs`, `Readonly Viewer`, `Chat Panel`, `Tools Panel`, `Knowledge Panel`, `Pipeline View`, `Ontology Proposal View`, and `Lume Help`.

Lume Help uses these names to avoid ambiguous spatial explanations. It remains contextual help only and does not execute runtime, LLM, tools, actions, semantic workflows, or filesystem mutation.

The sandbox does not yet include:

- production UI behavior
- notebook or block workspace
- editing or rich drafting surfaces
- complex docking or multi-window layout behavior
- real knowledge graph or semantic knowledge workflows
- semantic proposal approval, RDF persistence, Oxigraph, SPARQL, embeddings, or semantic store
- full tool execution behavior
- broad LLM workflow integration
- wide LLM tool orchestration
- full end-user product parity

## Local validation

This sandbox runs in a local Windows environment. Rust validation must be executed through wrappers in `dev/scripts/`.

Read `dev/scripts/SCRIPTS_INDEX.md` before using them.

Available entry points:

- `dev\scripts\cargo_check.bat`
- `dev\scripts\cargo_test.bat`
- `dev\scripts\cargo_all.bat`
- `dev\scripts\cargo_fmt.bat`
- `dev\scripts\cargo_clippy.bat`
- `dev\scripts\cargo_strict.bat`

Default validation after relevant Rust changes:

- `dev\scripts\cargo_all.bat`

Stricter validation before higher-risk closure:

- `dev\scripts\cargo_strict.bat`

## Roadmap by phases

- `3A`: minimal project runtime baseline, closed
- `3B`: explicit pipeline, runtime output, observability, closed
- `F4A`: thin `app_services`, closed
- `F4B`: clean `cli_app` consumption, closed
- `F5`: UI structural shell, closed
- `F6`: controllers plus manifest wiring, closed
- `F7`: technical readonly viewer, closed
- `F8`: knowledge panel and documentary workspace model, CLOSED
- `F9`: preferences / credentials plus F9.5 declarative/mock AI governance preparation; tools/catalog governance is declaratively closed
- `F10`: LLM chat integration with tools, not opened
- `F11`: final audit / CLOSED state verification

F9 remains declarative only.

- `resources/tools/tools_master_catalog.json` remains a declarative master catalog
- `resources/tool_runtime/*` remains the current operative runtime source
- `EffectiveToolSurfaceResolver` remains a future proposal only
- no real tool, LLM, or external binary execution is opened by F9 documentation

The roadmap remains sequential. The workspace is not trying to open all subsystems at once.

## Maturity disclaimer

This repository is not yet a final application runtime.

It is a governed engineering workspace intended to:

- preserve inherited doctrine from the Python system
- turn thin specs into executable Rust contracts
- validate architectural direction in small, auditable steps

It does not yet provide:

- stable external API commitments
- production UI or runtime behavior
- full integrated tool execution
- full integrated LLM runtime behavior
- semantic runtime, embeddings, RDF persistence, SPARQL, or Oxigraph execution

## Key documents

- `architecture/ARCHITECTURE.md`
- `governance/GOVERNANCE.md`
- `governance/FUNCTIONAL_SCOPE.md`
- `architecture/MIGRATION_BASELINE.md`
- `governance/WORKSPACE_RULES.md`
- `user/output/rust_status_snapshot.md`
- `docs/ENGINEERING_NOTES.md`

---

### !REL_FILE!

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

---

### !REL_FILE!

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

---

### !REL_FILE!

# I18N Policy

This sandbox inherits the Python rule that user-visible UI text must be externalized and governed outside views.

Supported UI languages:

- English (`en`)
- Spanish (`es`)

## Rules

- user-visible UI strings must not be hardcoded in views
- localized resources must live outside presentation code
- localized resources are consumed through `ui_i18n`
- the fallback order is: preferred language -> English -> controlled fallback
- all user-visible menu labels MUST use i18n keys
- all user-visible popup titles and popup descriptions MUST use i18n keys
- `Slint` views and other presentation views MUST NOT hardcode visible text
- every governed key in `resources/i18n/en/*.ftl` MUST have an equivalent key in `resources/i18n/es/*.ftl`
- every governed key in `resources/i18n/es/*.ftl` MUST have an equivalent key in `resources/i18n/en/*.ftl`
- internal identifiers MUST remain English and stable
- translated labels MUST NOT become runtime authority, identifiers, policy decisions, or execution triggers

## Naming convention

Governed visible-text keys must use stable domain prefixes:

- `menu.*`
- `popup.*`
- `dialog.*`
- `tooltip.*`
- `status.*`
- `help.*`

These prefixes classify visible text only.

They do not define runtime behavior, authority, routing, or execution semantics.

## Language distinctions

- UI language: language of menus, dialogs, labels, and general interface text
- chat language: language used in conversational interaction
- internal control language: implementation and control-layer language used for system logic and stable identifiers

These are related but not identical concerns and must not be conflated.

This sandbox includes placeholder Fluent resources only.

## Governed menu and popup contract

The top-level menu domains and governed popup surfaces must remain label-driven by i18n resources rather than by view-local literals.

This includes:

- top-level menu labels
- menu entry labels
- governed popup titles
- governed popup descriptions
- generic dialog button labels
- governed tooltips for visible UI chrome

Internal object names, popup ids, controller ids, and policy ids remain language-independent and stable even when their user-facing labels are translated.

Deprecated i18n keys may remain temporarily for compatibility or migration safety, but new or updated specs must not reference them as active contract keys.

## Invariants

- `INV-I18N-001`: all visible menu labels MUST come from i18n resources.
- `INV-I18N-002`: all visible popup titles and descriptions MUST come from i18n resources.
- `INV-I18N-003`: English and Spanish i18n files MUST remain key-symmetric for governed menus and popups.
- `INV-I18N-004`: translated labels MUST NOT become identifiers or runtime authority.
- `INV-I18N-005`: internal identifiers MUST remain stable and language-independent.
- `INV-I18N-006`: no visible UI text may be hardcoded in `Slint` or presentation views.
- `INV-I18N-007`: deprecated i18n keys MUST NOT be used by new specs.

---

### !REL_FILE!

# Inherited Governance

This file records how live Python governance is transferred into the Rust sandbox.

If a requested Python source name is not physically present in this sandbox, the transfer is anchored to the active local equivalent and marked accordingly.

| Principle | Python Source | Rust Adaptation | Status |
| --- | --- | --- | --- |
| system/dev/user separation | `OPERATIONAL_DEFINITION_Rev09.txt`, `INVARIANTS_GUIDE.md`, `DEV-DOMAIN-ARCHITECTURE.md`, `WORKSPACE_STRUCTURE_EXPLAINED.md` | Rust sandbox remains in `dev/` during migration and future Rust runtime preserves the same domain separation | inherited |
| portability and host independence | `OPERATIONAL_DEFINITION_Rev09.txt`, `PORTABLE_RUNTIME_BASELINE.md` | Rust successor must remain portable and must not depend on host-specific runtime assumptions | inherited |
| declarative-first architecture | `OPERATIONAL_DEFINITION_Rev09.txt`, `GOBERNANCE_INDEX.json`, active specs/contracts | `spec_runtime` becomes the entry point for declared specs, config, and registry loading | inherited |
| traceability and reproducibility | `OPERATIONAL_DEFINITION_Rev09.txt`, `INVARIANTS_GUIDE.md`, `LLM_OBSERVABILITY_GUIDE.md`, `CONTRATO_UI_CORE.json` | Rust runtime and service layers must preserve structured traceability; concrete event emission is deferred | adapted |
| project as primary working unit | `OPERATIONAL_DEFINITION_Rev09.txt`, `project_manifest_exposure_contract.json` | `project_runtime` centers project-scoped operations and manifest-governed visibility | inherited |
| workflow-centric runtime model | `OPERATIONAL_DEFINITION_Rev09.txt` | Rust baseline preserves workflow-centric intent but defers concrete workflow execution logic | deferred |
| manifest/ref-driven visibility | `project_manifest_exposure_contract.json`, `system/utils/project/*`, `UI_LAYOUT_Rev08.md` | `project_runtime` inherits validation -> ref resolution -> neutral surface model flow | inherited |
| artifact/output separation | `OPERATIONAL_DEFINITION_Rev09.txt`, `WORKSPACE_STRUCTURE_EXPLAINED.md`, project-domain runtime model | Rust project/runtime docs preserve separate concerns for artifacts, outputs, and runs | adapted |
| UI as representation, not source of truth | `CONTRATO_UI_CORE.json`, `UI_LAYOUT_Rev08.md` | `ui_core` and `ui_slint` remain representation-only layers over services/runtime | inherited |
| no filesystem/runtime logic in views | `CONTRATO_UI_CORE.json`, `UI_LAYOUT_Rev08.md`, `INVARIANTS_GUIDE.md` | Slint is pure presentation and cannot own runtime or filesystem behavior | inherited |
| externalized user-visible UI strings | `ui_strings.json`, `ui_strings_contract.json` | `ui_i18n` and `resources/i18n/` own user-visible text resources | inherited |
| token-driven theming | `ui_theme.json`, `ui_theme_contract.json` | `ui_theme` and `resources/themes/` define semantic tokens for light/dark modes | adapted |
| local/cloud/auto LLM modes | `LLM_CONFIGURATION_GUIDE.md`, `governance/LLM_RUNTIME_POLICY.md` | `llm_core` defines modes and policy while adapters remain separated | inherited |
| deterministic runtime resolution | `LLM_CONFIGURATION_GUIDE.md`, `governance/LLM_RUNTIME_POLICY.md` | Rust LLM runtime planning must resolve mode/provider/model deterministically before invocation | inherited |
| model catalog governance | `LLM_MODEL_CATALOG_GUIDE.md` | Rust LLM execution must validate models against a governed catalog once execution is implemented | inherited |
| tools requested by model, executed by system | `LLM_TOOL_INTEGRATION_GUIDE.md`, `llm_tools_contract.json` | `llm_core` governs request policy and `tool_runtime` or `app_services` execute tools | inherited |
| observability as a required subsystem concern | `LLM_OBSERVABILITY_GUIDE.md`, `CONTRATO_UI_CORE.json` | Structured observability is mandatory for future Rust execution, but concrete events are deferred | adapted |
| no provider-driven architecture | `governance/LLM_RUNTIME_POLICY.md`, `LLM_CONFIGURATION_GUIDE.md`, `DEV-DOMAIN-ARCHITECTURE.md` | adapters stay behind `llm_core`; application code does not couple directly to providers | inherited |
| dev domain as engineering laboratory | `DEV-DOMAIN-ARCHITECTURE.md` | this Rust workspace remains a governed dev-only migration environment until elevated by future governance | inherited |
| automated auditability | `INVARIANTS_GUIDE.md`; requested `AUDIT_ENGINE_SPEC_Rev00.txt` not present locally | Rust docs preserve audit-ready structural rules; a Rust audit engine contract is deferred | deferred |

---

### !REL_FILE!

# LLM Runtime Policy

This sandbox inherits the live Python LLM doctrine.

Supported execution modes:

- `local`
- `cloud`
- `auto`

## Deterministic resolution

The runtime layer must resolve an execution plan before invocation.

Inherited precedence:

`explicit args -> user preferences -> system config -> defaults`

No runtime ambiguity is allowed.

## Layer responsibilities

- `llm_core` defines contracts, policy, mode selection, model catalog rules, tool request governance, and observability requirements
- `llm_local` implements local adapters behind `llm_core`
- `llm_cloud` implements cloud adapters behind `llm_core`

## Required doctrine

- application code must not depend directly on a provider
- providers stay behind governed adapter crates
- model catalog governance is mandatory once real execution starts
- tools may be requested by the model, but they are executed by the system
- runtime policy must remain observable and traceable
- the runtime layer must not execute tools directly
- the runtime layer must not contain UI logic

## Observability baseline

Real execution is deferred, but the inherited requirement is already fixed:

- enough structured metadata must exist to reconstruct provider, model, mode, tool policy, and outcome

---

### !REL_FILE!

# UI Slint Policy

This sandbox inherits the active Python UI doctrine that presentation is not the source of truth.

Slint is the active presentation technology for this Rust sandbox shell.

## Rules

- business logic must not live inside Slint views
- filesystem logic must not live inside Slint views
- runtime logic must not live inside Slint views
- Slint must consume i18n and theme layers rather than reimplement them
- `ui_slint` consumes `ui_core`, `ui_i18n`, `ui_theme`, and thin controller-facing runtime boundaries without becoming their owner
- `app_services` must remain independent from Slint widgets
- Slint must not execute tools or external workflows directly
- Slint must not couple directly to LLM providers
- Slint routes and renders; controllers decide; governed runtime layers remain authoritative
- the viewer remains readonly regardless of whether a workflow starts from tree or clip
- workspace tabs are controlled content views, not miniapps or a parallel runtime
- Lume onboarding/help surfaces may be presented by Slint, but their visible text must come from i18n resources
- Slint may represent `GUI_OBJECTS_v1` inside Lume Help, but canonical GUI names and visible help text must come from `resources/help/gui_objects.json` and i18n resources
- Slint must not invent GUI object names, hardcode GUI help text, interpret Lume Help, or execute anything through Lume Help
- Slint must not execute Lume, tools, LLM workflows, external binaries, or sandbox filesystem actions directly
- the DocGraph icon may be rendered in top chrome
- its click handler may only emit `OpenLumeHelpRequested` or equivalent navigation intent
- UI must not hardcode runtime paths or execute logic from the icon
- icon visible text or tooltip must come from i18n

## Current scope

- live shell window with explicit controllers
- governed document tree for existing project documents
- explicit clip surface for document intake and workflow launch
- structured chat surface for text, document references, tool results, and state messages
- tabbed workspace content area
- technical readonly viewer as one workspace tab
- knowledge panel integrated into the shell
- tools panel integrated into the shell
- Lume Help GUI Objects section as readonly presentation of canonical names
- no direct product-runtime ownership

## Current role boundaries

- Slint presents the shell and routes user intent into controllers
- controllers translate UI intent into governed runtime calls and presentation state
- tree actions reference and open existing governed documents; they do not import them again
- clip actions intake external files or launch explicit workflows; they do not replace the tree
- chat stores references and results; it does not become a hidden document store
- workspace tabs host controlled views; they do not define new domain pipelines
- the viewer verifies resolved content readonly; it does not become an editor or extractor

## Future governed editing proposal

In a later phase, Slint may capture viewer selection, expose a contextual instruction popup, and render patch previews or accept/reject/regenerate choices.

Slint must not:

- generate patches
- validate patches
- apply patches
- modify the filesystem
- execute LLM calls directly
- execute tools directly

The future UI role is to collect intent and render governed state supplied by controllers or runtime boundaries.

---

### !REL_FILE!

# UI Theme Policy

This sandbox inherits the rule that UI appearance must be resource-driven and token-based.

Supported appearance modes:

- `light`
- `dark`

## Rules

- UI views must not hardcode colors
- only semantic theme tokens may be consumed by presentation code
- theme resources are loaded through `ui_theme`
- appearance is independent from UI language
- theme fallback must resolve through controlled resources rather than ad hoc defaults

## Fallback policy

- preferred theme resource
- default theme resource for the active mode
- controlled failure or diagnostic if required semantic tokens are missing

---

### !REL_FILE!

# Workspace Rules

## Structural rules

- one primary responsibility per crate
- keep dependency direction controlled and inward
- do not create parallel implementations of the same governed flow
- keep edge crates thin
- keep UI crates free of business logic, filesystem ownership, and runtime ownership
- do not couple application code directly to a provider-specific LLM implementation

## Workspace and path discipline

- workspace-relative paths are logical portable paths, not OS-rendered strings
- compare absolute paths robustly; do not depend on platform-specific textual rendering
- fix path policy in base crates rather than patching each higher crate separately
- persist portable relative paths instead of absolute host-specific paths wherever the contract requires it

## Validation discipline

- use the Rust workspace as the center of implementation
- mechanical validation runs through `dev/scripts/` wrappers in the local Windows environment
- read `dev/scripts/SCRIPTS_INDEX.md` before using wrappers
- default validation after relevant Rust changes: `dev\scripts\cargo_all.bat`
- stricter validation before higher-risk closure: `dev\scripts\cargo_strict.bat`

## Current layering rules

- `project_runtime` owns the project pipeline
- `app_services` consumes governed verticals and does not replace them
- `cli_app` stays an entrypoint, not a domain orchestrator
- `tool_runtime` stays minimal and separate from project and knowledge concerns until a later phase explicitly opens more
- `ui_slint` consumes controllers and presentation state; it does not own filesystem or domain policy
- the central workspace area is tabbed; each tab is a controlled view, not an autonomous miniapp
- the readonly viewer is one workspace tab, not the whole workspace model
- the document tree is the reference surface for existing governed documents already inside the system
- the clip is the explicit surface for external intake and document workflow launch
- the chat layer carries structured references and results; it does not become a second document store
- document profiling belongs to workspace tabs and thin controllers; it does not move into `project_runtime`
- the knowledge layer consumes existing viewer and controller boundaries; it does not reinterpret the manifest
- the tools surface stays split between controlled operational launch and declarative LLM tool policy
- project-level LLM tool overrides must remain declarative and intersect with the global allowed surface
- visible product identity is `DocGraph` and assistant identity is `Lume`; this is declarative identity, not runtime authority
- Lume is an interaction and help layer; it does not execute filesystem, tools, or LLM workflows
- Lume Help is contextual governed help; it does not execute, mutate, approve actions, call LLMs, persist history, or create project data
- Lume Help must use `GUI_OBJECTS_v1` canonical names when explaining interface objects
- GUI object explanations should name the object explicitly and avoid vague spatial references without the canonical object name
- visible UI strings must come from governed resources and i18n, not hardcoded Slint views
- `resources/` declares; `runtime/` materializes placeholders; `user/` stores user-owned preferences and data
- external runtime tools remain missing or disabled until a later phase explicitly opens validation and execution
- local sandbox mutation remains disabled until authorized policy and logging are implemented
- action and flow-control policies remain declarative in F9; they do not execute or approve runtime behavior
- ActionResolution remains declarative in F9; it does not create a runner, invoke LLM/tools, or mutate files
- UI may present pending actions in a future phase, but every execution must reference an explicit `action_id`
- document templates remain declarative proposals until a later phase opens template/runtime behavior
- future document export must use package data and export profiles, not UI state
- reference styles remain declarative resources under `resources/reference_styles/`; corporate styles must not be hardcoded in code or UI
- BibTeX and bibliography handling remain proposals; no parser, importer, or export materialization exists yet
- F9.5 semantic proposals remain proposals, not facts; approval requires future explicit human review
- Pipeline / Ontology View is readonly/mock representation; it does not infer, approve, execute, persist RDF, run SPARQL, or mutate files
- Oxigraph/RDF preparation remains disabled: no Oxigraph dependency, embeddings, semantic store, or N-Quads output is created
- project folders must not contain a root `outputs/` directory; outputs belong to the functional owner
- every future tool output must carry an `owner_ref`
- tool outputs live under the owning chat, DocumentHolder, or knowledge derivation/proposal area
- `graph/` explains artifact relations; it does not govern, execute, approve, or replace `project_runtime`
- `registry.json` is a navigation accelerator; it does not replace `project_manifest.json` or `graph/`
- filesystem presence must not imply project exposure; `project_manifest.json` governs exposure
- chat attachments are session context; durable knowledge or document inputs require explicit promotion
- `DocumentHolder` owns document production state; `document.md` is its only editable textual source
- graph entries must originate from governed actions and remain traceable to document, chat, knowledge, or tool origins
- referenced artifacts must exist, resolve, and be exposed by `project_manifest.json`
- artifact IDs must remain stable across moves and renames and must not depend on filesystem path
- prefer stable references over duplicating the same physical file across knowledge, chats, or documents
- persisted tool runs must declare inputs, configuration, `owner_ref`, outputs, status, and trace data in `tool_run_manifest.json`
- ordered inputs must declare `ordered=true` when order affects result
- tool configuration must not be mixed with input data
- persisted tool runs require structured lifecycle, structured errors/warnings, and reproducible inputs/configuration
- UI may collect ordered input lists but must not execute or validate domain logic
- UI may capture `ProjectSetupDraft` and `ProjectSettingsDraft`, but it does not govern project configuration authority
- project profiles declare intention, not runtime authority
- selected external folders are readonly; sandbox mutations may affect only a future working copy
- LLM receives an effective tool surface summary by default, not raw catalogs
- expanded tool context must come from a governed resolver or provider, not from UI state or LLM guessing
- preferences are non-secret configuration
- credentials are referenced, not embedded
- project files must not contain secret values
- LLM context must not receive credentials
- changing preferences or `credential_ref` does not enable execution
- the app must remain usable without LLM
- Lume Static Mode uses declarative help
- Lume help-tree content is declarative navigation, explanation, and procedure state
- missing LLM or tools is visible capability state, not failure
- project setup must not depend on LLM availability
- UI renders prepared help-tree state only and does not infer policy or execute actions
- the main menu remains minimal and stable: `File`, `Preferences`, `Tools`, `Help`
- Tools is a top-level menu domain separate from Preferences and exposes separate governed entries for Operational Tools, LLM Tools, and External Dependencies
- visible menu text must come from i18n resources
- the bottom status bar displays prepared state only
- the bottom status bar exposes no execution, no secrets, and no policy inference
- the bottom status bar may provide navigation intents only
- the top-left DocGraph icon is branding plus a Lume Help shortcut
- clicking the DocGraph icon must not execute tools, LLM, project actions, filesystem mutations, or settings changes
- the central layout remains Tree + Chat + Tabs Workspace
- viewer belongs to Tabs Workspace
- Sandbox View may appear as a conditional tab only when relevant
- Sandbox View is informational in F9 and must not execute filesystem operations

## Migration rules

- Inherited doctrine remains reference only
- Rust is the active implementation target in this sandbox
- implementation growth must follow documented crate boundaries
- governance-relevant changes must be reflected in root docs
- placeholder code is acceptable only when it does not fake implemented product behavior

## Candidate rules

- reusable runtime policies should not remain in an entrypoint once they clearly belong to a governed vertical
- critical steps in the controlled runtime path should not remain outside minimal structured observability
- higher layers should adapt around governed outputs instead of widening them for presentation or cross-layer convenience
- knowledge navigation should remain readonly and should not grow into editing, semantics, or notebook behavior before the corresponding phase opens
- viewer and inspectors are not free editors
- `SOURCE` and `DERIVED` documents remain readonly
- `ARTIFACT` documents may be edited only through governed patch acceptance
- patch validation and application must not live in UI crates
- chat may reference edit requests, selections, and proposals, but it must not store source blobs or act as a document store
- document template guidance is editorial and declarative; it must not execute, mutate, or decide policy
- document references must remain structured proposals; referenced `SOURCE` and `DERIVED` stay readonly
- semantic proposal UI must represent declarations and traces only; no hidden semantic inference belongs in UI
- GUI object names are shared help vocabulary, not runtime authority

## F12.4 file intake boundary

- `docs/specs/file_intake.md` owns governed file intake semantics.
- File intake is SPEC-only in F12.4.
- Source host paths are not portable identity.
- Original host files remain readonly.
- Filesystem presence does not imply project exposure.
- `project_manifest.json` remains exposure authority.
- UI and `app_services` must not mutate storage, import files, derive text, or expose intake items.
- `assets/` must not be used as file-intake runtime storage.

## F12.5 file intake planning boundary

- `F12.5` is plan-only and audit-checklist-only.
- `F12.5` introduces no runtime and touches no crates.
- Future `F12.6` must use explicit user-selected files only.
- Future `F12.6` must keep source folders readonly.
- Future `F12.6` must not persist private absolute host paths as portable truth.
- Future `F12.6` must not mutate `project_manifest.json`, generate registries, write graph entries, or generate derivatives unless a later phase explicitly opens those scopes.
- Future `audit_f12_file_intake_boundary.bat` is mandatory before closure.

---

### !REL_FILE!

# Master Governance And Specs Report

Generated from live sandbox documents.

- Workspace root: `C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\dev\scripts\..\..`
- Generated at: `02/05/2026 16:40:33,61`

---

## Governance and architecture documents

### README.md

# DocGraph Rust Portable App Sandbox

This repository is the govern
---

### !REL_FILE!

# Architecture

This workspace is a governed Rust migration sandbox, not a greenfield redesign.

Its architecture inherits the system model of the active Python application and ports it into explicit crate boundaries and controlled phases.

The visible application identity is `DocGraph`; the assistant/help identity is `Lume`. This is nominal and declarative only. It does not change runtime architecture, crate ownership, or execution authority.

## Current architectural center

The governed center remains the project vertical in `project_runtime`.

Its canonical pipeline is:

`project_root -> manifest -> contract -> validation -> surface -> resolve -> output`

That pipeline is the only project-opening and project-resolution path in the current sandbox.

A DocGraph project is the user's work boundary. It scopes configuration, knowledge files, chats, DocumentHolders, allowed tools, artifact relations, and traceability.

The filesystem organizes project data, the manifest governs exposure, `registry.json` accelerates navigation, and `graph/` explains artifact relations. None of these replace or duplicate the `project_runtime` pipeline.

Project-root `outputs/` is not part of the normative project layout. Tool outputs must be scoped to their functional owner by `owner_ref`: chat, DocumentHolder, or knowledge derivation/proposal area.

## Current layer map

- `core_domain`: shared domain vocabulary and stable concepts
- `workspace_core`: workspace boundaries, path discipline, portable relative-path semantics
- `io_runtime`: controlled filesystem mutations and small readonly IO boundaries
- `spec_runtime`: declarative loading and typed document access
- `project_runtime`: central project vertical, pipeline ownership, validation, surface model, viewer resolution, observability
- `document_text_runtime`: regenerable text derivation, page extraction, and chunking over primary documents
- `app_services`: thin consumer layer over governed verticals
- `tool_runtime`: declarative catalog subsystem plus minimal accepted runner boundary
- `cli_app`: thin executable entrypoint for controlled local flows
- `ui_core`: presentation state and shell contracts
- `ui_slint`: structural shell, thin controllers, workspace tabs, readonly viewer, and knowledge panel wiring
- `llm_core`, `llm_local`, `llm_cloud`: LLM-oriented layers, still narrow and not yet broadly integrated

## Dependency direction

Dependencies remain controlled and inward:

- foundational crates stay below edge crates
- `project_runtime` consumes lower runtime layers; it is not reimplemented above
- `app_services` consumes governed verticals and stays thin
- `cli_app` consumes upper-layer boundaries and remains an entrypoint, not a second orchestrator
- UI crates consume presentation-oriented contracts and do not own runtime logic
- `tool_runtime` and LLM crates remain separate from the governed project pipeline unless a later phase explicitly opens that integration

## Current system boundary

Inside the current system:

- governed workspace path handling
- declarative loading of project-facing data
- explicit project pipeline
- typed project runtime output
- thin application-service consumption
- declarative tool catalog modeling and minimal accepted dispatch
- regenerable document text derivation and chunking under document-local `text/`
- structural UI shell
- explicit UI controllers and manifest wiring
- tabbed workspace content area for central UI views
- technical readonly viewer over resolved targets as one workspace tab
- minimal knowledge panel over project `knowledge/` documents
- governed document tree over existing project documents
- structured chat references plus clip-driven external intake and document workflows
- UI tools surface with separated operational launch and LLM policy tabs

Outside the current system:

- production UI behavior
- notebook or block editing
- governed document editing
- future `document_patch_runtime`
- active context model for patch requests
- structured content assets for figures, tables, and equations
- real Lume execution behavior
- real LLM engine or model execution
- external binary execution through runtime folders
- mutable local sandbox filesystem actions
- real action-request execution and pending-action runtime
- real ActionResolution enforcement or runner behavior
- knowledge graph or semantic knowledge workflows
- real semantic proposal approval, RDF persistence, Oxigraph store, SPARQL, or embeddings
- broad tool execution behavior
- broad LLM workflow integration
- cross-domain orchestration above current phase boundaries
- full product runtime behavior

## Future phase proposal: governed document editing

Governed document editing is a future proposal above the current F8 workspace model.

It may introduce patch proposals, previews, accept/reject/regenerate flow, active-context resolution, and structured content asset inspection. It must not duplicate `project_runtime`, widen `ProjectRuntimeOutput`, turn the readonly viewer into an editor, place patch logic in UI crates, or allow the LLM to mutate files directly.

## F9-ready declarative layout

F9 preparation may materialize declarative resources for the `DocGraph` product identity, Lume, preferences, runtime locations, tools catalogs, LLM catalogs, sandbox policy, and storage policy.

F9 preparation may also materialize declarative action policy, flow-control policy, and pending action state documentation.

F9 may document `ActionResolution` as a future governance chain from request to trace, but no runner, execution authority, LLM/tool invocation, filesystem mutation, or `project_runtime` change is introduced.

F9.5 may declare AI governance specs, semantic proposal schemas, and a readonly/mock Pipeline / Ontology View. This view is presentation only: it does not call LLMs, create embeddings, persist RDF, run SPARQL, activate Oxigraph, approve proposals, or mutate documents.

Architectural ownership remains unchanged: `resources/` declares, `runtime/` materializes placeholders, `user/` contains user-owned data and preferences, `crates/` implement governed resolution and validation when a later phase opens them, and `ui_slint` only presents and routes.

For tools/catalog governance in F9:

- `resources/tools/tools_master_catalog.json` is declarative only
- `resources/tool_runtime/*` remains the current operative source
- `EffectiveToolSurfaceResolver` is future-only and not implemented
- no direct `MasterCatalog -> tool_runtime` runtime consumption exists
- F10 is not opened by these declarations

## Architectural rules that matter now

- no second project pipeline outside `project_runtime`
- governed file intake semantics are owned by `docs/specs/file_intake.md`
- file intake is a future governed pipeline contract, not a `tool_runtime` orchestration shortcut
- source host paths are transient inputs and must not become portable identity
- `project_manifest.json` remains exposure authority; filesystem presence alone does not expose files
- UI and `app_services` may capture or present intake intent only and must not import, copy, classify, register, expose, or derive files
- `F12.5` is a plan-only boundary for future file intake implementation
- future `F12.6` preferred ownership keeps filesystem validation and governed copy concerns in `io_runtime`
- future `F12.6` must not expand `project_runtime`, `tool_runtime`, `app_services`, or UI authority by convenience
- no widening of `ProjectRuntimeOutput` for presentation or cross-layer convenience
- `app_services` stays thin and consumes the vertical rather than shadowing it
- `tool_runtime` stays minimal and separate from project and knowledge concerns
- primary documents remain the source of truth; derived text and chunks are secondary and regenerable
- UI controllers delegate and map; they do not own domain logic
- the workspace content area is a simple tab container, not a new domain layer
- the readonly viewer remains a concrete tab inside that workspace container
- the knowledge panel consumes small technical boundaries and may open content into the readonly viewer tab model
- the document tree references existing governed documents; it is not a second import pipeline
- the clip is explicit external intake and workflow launch; it does not replace the tree as source of truth
- chat stores text, structured document references, tool results, and system state; it does not store opaque source blobs
- document profiling is a workspace workflow that may open a tab and reuse the readonly viewer; it is not a chat-internal blob operation
- a workflow opened from the tree or from the clip must converge into the same governed workspace and readonly-viewer model
- the tools panel stays split:
  - `Operational Tools` may manually invoke the minimal runner boundary
  - `LLM Tools` may govern declarative policy only
- UI does not reinterpret the manifest or rediscover project structure on its own

## Current maturity

The workspace is beyond a baseline scaffold.

It contains a validated project vertical, thin consumers above it, a structural UI with controllers, a tabbed workspace content area, a readonly viewer tab, a governed document tree, clip-driven intake, structured chat references, and the first knowledge and tools surfaces. F8 is CLOSED and established as the documentary workspace model. The sandbox is still an engineering workspace and not a final application runtime.

---

### !REL_FILE!

# Knowledge Transfer Map

This map records where Python knowledge has been transferred inside the Rust sandbox.

It records knowledge transfer from Python as historical and doctrinal source material only.
It does not declare a runtime dependency on Python and does not require Python to operate the Rust sandbox.

When a requested Python source name is not physically present in this sandbox, the active local equivalent is used and the transfer type reflects that adaptation.

| Python Source Document | Category | Knowledge Transferred | Rust Target File | Transfer Type |
| --- | --- | --- | --- | --- |
| `OPERATIONAL_DEFINITION_Rev09.txt` | platform constitution | portability, declarative-first architecture, project as primary working unit, workflow-centric system model | `README.md`, `governance/GOVERNANCE.md`, `architecture/ARCHITECTURE.md`, `governance/FUNCTIONAL_SCOPE.md`, `architecture/MIGRATION_BASELINE.md`, `docs/specs/project_runtime.md` | inherited as-is |
| `INVARIANTS_GUIDE.md` | invariants and auditability | structural separation, reproducibility, audit-ready discipline, controlled runtime boundaries | `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md`, `governance/INHERITED_GOVERNANCE.md`, `docs/specs/workspace_core.md`, `docs/specs/ui_core.md` | inherited as-is |
| `GOBERNANCE_INDEX.json` | knowledge hierarchy | active normative precedence and source discovery | `governance/GOVERNANCE.md`, `governance/INHERITED_GOVERNANCE.md`, `architecture/KNOWLEDGE_TRANSFER_MAP.md` | adapted |
| `DECLARED_VS_PRESENT_ARTIFACTS_POLICY.md` | governance policy | declaration takes precedence over sandbox-local physical absence | `governance/GOVERNANCE.md`, `architecture/MIGRATION_BASELINE.md`, `governance/INHERITED_GOVERNANCE.md` | inherited as-is |
| `DEV-DOMAIN-ARCHITECTURE.md` | development domain governance | dev as engineering laboratory, not distributed runtime | `README.md`, `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | inherited as-is |
| `GUIA - UTILITIES.txt` | implementation discipline | reuse-first utility doctrine and separation between generic utilities and domain logic | `architecture/ARCHITECTURE.md`, `governance/WORKSPACE_RULES.md` | adapted |
| `utils_catalog_oficial.json` | implementation discipline | technical utility coverage and controlled cross-cutting concerns | `architecture/ARCHITECTURE.md`, `architecture/MODULE_MAPPING.md` | adapted |
| `WORKSPACE_STRUCTURE_EXPLAINED.md` | workspace doctrine | normative vs runtime vs tooling boundaries and mutable vs immutable areas | `README.md`, `governance/GOVERNANCE.md`, `docs/specs/workspace_core.md` | adapted |
| `project_manifest_exposure_contract.json` | project contract | manifest-driven visibility, ref model, no direct filesystem-driven exposure | `architecture/ARCHITECTURE.md`, `architecture/MODULE_MAPPING.md`, `docs/specs/project_runtime.md`, `docs/specs/ui_core.md` | inherited as-is |
| `system/utils/project/*` | live project implementation knowledge | validation -> ref resolution -> neutral surface model flow | `architecture/ARCHITECTURE.md`, `architecture/MODULE_MAPPING.md`, `docs/specs/project_runtime.md` | adapted |
| requested `PROJECT_MANIFEST_SPEC_v2.txt` not present locally; active equivalent: `project_manifest_exposure_contract.json` plus `system/utils/project/*` | project manifest governance | ref-driven manifest governance carried into Rust using active equivalents | `governance/INHERITED_GOVERNANCE.md`, `architecture/KNOWLEDGE_TRANSFER_MAP.md`, `docs/specs/project_runtime.md` | adapted |
| requested `PROJECT_FILESYSTEM_SPEC_v2.txt` not present locally; active equivalent: `WORKSPACE_STRUCTURE_EXPLAINED.md` plus current project runtime model | project/workspace layout | project isolation, workspace boundaries, and artifact/output separation | `architecture/MIGRATION_BASELINE.md`, `architecture/MODULE_MAPPING.md`, `docs/specs/workspace_core.md`, `docs/specs/project_runtime.md` | adapted |
| `CONTRATO_UI_CORE.json` | UI/controller/runtime contract | UI as representation, controller boundary, traceability, no direct filesystem access | `architecture/ARCHITECTURE.md`, `governance/WORKSPACE_RULES.md`, `governance/UI_SLINT_POLICY.md`, `docs/specs/ui_core.md` | inherited as-is |
| `UI_LAYOUT_Rev08.md` | UI shell doctrine | tree/chat/viewer/log roles, controller-first flow, viewer passivity, manifest-governed visibility | `architecture/ARCHITECTURE.md`, `governance/UI_SLINT_POLICY.md`, `docs/specs/ui_core.md` | adapted |
| requested `UI_WORKSPACE_SPEC_MASTER_v4.txt` not present locally; active equivalent: `CONTRATO_UI_CORE.json`, `UI_LAYOUT_Rev08.md`, `ui_strings.json`, and `ui_theme.json` | UI workspace governance | presentation-only UI with externalized text and governed theme resources | `governance/I18N_POLICY.md`, `governance/UI_THEME_POLICY.md`, `governance/UI_SLINT_POLICY.md`, `docs/specs/ui_core.md` | adapted |
| `ui_strings.json` and `ui_strings_contract.json` | UI localization | user-visible UI text externalization and bilingual resource governance | `governance/I18N_POLICY.md` | inherited as-is |
| `ui_theme.json` and `ui_theme_contract.json` | UI appearance | light/dark appearance modes and theme resources under governed tokens | `governance/UI_THEME_POLICY.md` | adapted |
| `LLM_CONFIGURATION_GUIDE.md` | LLM configuration doctrine | config layers, deterministic precedence, runtime mode inputs | `governance/LLM_RUNTIME_POLICY.md`, `docs/specs/llm_core.md` | inherited as-is |
| `governance/LLM_RUNTIME_POLICY.md` | LLM runtime doctrine | local/cloud/auto, deterministic execution plan, policy before invocation | `governance/LLM_RUNTIME_POLICY.md`, `docs/specs/llm_core.md` | inherited as-is |
| `LLM_MODEL_CATALOG_GUIDE.md` | model governance | catalog as control layer, approved models only, deterministic validation | `governance/LLM_RUNTIME_POLICY.md`, `docs/specs/llm_core.md` | inherited as-is |
| `LLM_TOOL_INTEGRATION_GUIDE.md` | tool governance | tools requested by the model but executed by the system | `governance/LLM_RUNTIME_POLICY.md`, `docs/specs/llm_core.md` | inherited as-is |
| `LLM_OBSERVABILITY_GUIDE.md` | observability doctrine | structured metadata required to reconstruct runtime decision and outcome | `governance/LLM_RUNTIME_POLICY.md`, `docs/specs/llm_core.md`, `governance/INHERITED_GOVERNANCE.md` | adapted |
| requested `AUDIT_ENGINE_SPEC_Rev00.txt` not present locally; active equivalent: `INVARIANTS_GUIDE.md` plus declared-vs-present policy | audit doctrine | audit-ready structural discipline transferred, concrete Rust audit engine deferred | `governance/INHERITED_GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | deferred |
| requested `KNOWLEDGE_INDEX_MASTER.md` not present locally; active equivalent: `GOBERNANCE_INDEX.json` | knowledge hierarchy | document precedence and active-source discovery | `governance/GOVERNANCE.md`, `architecture/KNOWLEDGE_TRANSFER_MAP.md` | adapted |

---

### !REL_FILE!

# Migration Baseline

The Python application remains the historical doctrinal and behavioral migration reference.

The Rust sandbox is now the active engineering center for the successor implementation.

Python is not the current execution path for this Rust sandbox and is not required to operate it.

## Reference areas in Python

- workspace separation and portability doctrine
- declarative loading of specs, config, and registries
- project runtime and manifest/ref-driven visibility
- IO and utility discipline
- LLM configuration, runtime policy, model catalog, tool policy, and observability
- UI controller boundaries, text externalization, and theme governance

## Rust responsibilities already opened

- workspace boundaries and path discipline
- declarative spec and registry interpretation
- governed project runtime pipeline
- thin application-service consumption above the project vertical
- declarative tool catalog modeling and selection
- local validation and engineering tooling inside the workspace

## What still remains primarily in Python

- the active product runtime
- mature operational workflows
- mature UI behavior
- existing LLM execution paths
- migration-support behavior that has not yet been ported under Rust governance

## Migration rule

Rust does not replace Python by imitation of incidental details.

It replaces Python by progressively absorbing governed responsibilities into explicit Rust contracts, crates, and validated phases.

## Asset migration note

Inherited visual assets may first be consolidated in `assets/` inside this sandbox as a temporary staging area.

That staging step is for:

- cleanup
- filename normalization
- selection of official candidates

`assets/` is not the canonical runtime resource location. Approved assets may later be promoted into `resources/` if governance requires it.

---

### !REL_FILE!

# Module Mapping

This mapping is responsibility-driven.

It does not justify superficial folder copying from Python into Rust.

| Python responsibility | Live Python source cues | Rust target | Transfer note |
| --- | --- | --- | --- |
| workspace separation, scope control, path discipline | `OPERATIONAL_DEFINITION_Rev09`, `INVARIANTS_GUIDE`, `WORKSPACE_STRUCTURE_EXPLAINED`, `system/utils/workspace` | `workspace_core` | inherit portable workspace boundaries and project scoping |
| generic filesystem, text, and data utilities | `GUIA - UTILITIES`, `utils_catalog_oficial.json`, `system/utils/data`, `system/utils/text`, `system/utils/fs` | `io_runtime` and `workspace_core` | adapt utility discipline, not Python helper names |
| declarative specs, contracts, registries, and config loading | `GOBERNANCE_INDEX.json`, `system/spec/*`, `system/config/*` | `spec_runtime` | inherit declarative-first loading and validation |
| project as primary working unit, manifest/ref model, project-safe operations | `project_manifest_exposure_contract.json`, `system/utils/project`, `system/bin/tools/runtime/project` | `project_runtime` | inherit manifest validation -> ref resolution -> neutral surface model flow |
| LLM runtime policy, modes, model catalog, and observability | `system/app/llm/docs/*` | `llm_core` | inherit deterministic policy and contracts |
| local LLM adapter behavior | local mode doctrine from `governance/LLM_RUNTIME_POLICY.md` and `LLM_CONFIGURATION_GUIDE.md` | `llm_local` | adapters behind contracts only |
| cloud LLM adapter behavior | cloud mode doctrine from `governance/LLM_RUNTIME_POLICY.md` and `LLM_CONFIGURATION_GUIDE.md` | `llm_cloud` | adapters behind contracts only |
| tool execution as system concern | `LLM_TOOL_INTEGRATION_GUIDE.md`, `llm_tools_contract.json` | `tool_runtime` | tools requested by model, executed by system |
| UI state and controller-facing representation contracts | `CONTRATO_UI_CORE.json`, `UI_LAYOUT_Rev08.md` | `ui_core` | representation only, not source of truth |
| user-visible UI text externalization | `ui_strings.json`, `ui_strings_contract.json` | `ui_i18n` | no hardcoded UI strings in views |
| semantic appearance resources | `ui_theme.json`, `ui_theme_contract.json` | `ui_theme` | token-driven light/dark theming |
| Slint presentation integration | UI layout and controller boundary doctrine | `ui_slint` | presentation edge only |
| application orchestration above runtime and below presentation | UI/controller/runtime interaction model and LLM tool policy | `app_services` | compose services without widget coupling |
| future executable surface | current launcher and entrypoint intent | `cli_app` | thin executable only at this stage |

---

## Specs

### !REL_FILE!

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

---

### !REL_FILE!

# action_policy

## Purpose

Define the F9-ready declarative governance model for action risk, decision outcomes, and human-in-the-loop approval.

This spec does not implement action execution, tool execution, LLM invocation, or filesystem mutation.

## Scope F9

F9 may declare:

- risk levels
- allow/confirm/reject decisions
- human-in-the-loop policy
- document mutation constraints
- blocked action identifiers

The policy is declarative and read-only in F9.

## Risk levels

- `read_only`
- `low`
- `medium`
- `high`
- `forbidden`

`forbidden` cannot be overridden by user confirmation.

## Decision outcomes

- `allow`
- `confirm`
- `reject`

These outcomes describe future policy resolution only. They do not execute anything in F9.

## Document rules

- `SOURCE`: readonly
- `DERIVED`: readonly
- `ARTIFACT`: confirm before mutation

`SOURCE` and `DERIVED` may provide context but must not become mutation targets.

## Blocked actions

- `mutate_source_document`
- `mutate_derived_document`
- `llm_direct_filesystem_write`
- `execute_unregistered_tool`

## Forbidden responsibilities

The action policy must not:

- duplicate the `project_runtime` pipeline
- execute tools
- invoke LLM providers
- mutate files
- authorize unregistered tools
- place policy decisions in UI crates

## Future F10/F11 notes

F10 may introduce policy resolution over concrete action requests.

F11 should audit that forbidden actions remain non-overridable and that human approval is recorded before any future mutable execution.

---

### !REL_FILE!

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

---

### !REL_FILE!

# active_context

## Purpose

Define the future active-context model for referenced document work.

This is a conceptual contract only. It does not implement UI state or runtime behavior.

`active_context` is derived usage context. It is not the canonical selection model.

The canonical cross-family selection model is defined in `docs/specs/active_object_context.md`.

## Core rule

The active workspace tab may contribute default context for user instructions, but it does not replace `ActiveObjectContext`.

Chat orders apply to prepared active context only when a valid selected family and target exist, or when the user provides an explicit structured reference.

## Active targets

Future context may include:

- active document
- active sandbox
- active figure
- active table
- active equation
- active patch preview

The active target is derived from `ActiveObjectContext`, the selected workspace tab, and controlled view state.

## Context by tab

- Document Viewer: document or selection context
- Sandbox View: sandbox context
- Figure Inspector: figure and figure-metadata context
- future Table Inspector: table context
- future Equation Inspector: equation context
- future Patch Preview: proposal-review context

These tabs remain controlled views, not independent domain runtimes.

Opening or activating a tab does not silently erase other-family selections.

## Confirmation rule

If the target is ambiguous, stale, hidden, or conflicts with an explicit chat reference, the system must ask for confirmation before preparing a patch request.

Confirmation is required when:

- no active artifact exists
- no valid focused family exists
- multiple candidate artifacts match
- the active tab is readonly context only
- the request could target a `SOURCE` or `DERIVED` document
- the selected context changed after the request was prepared

## Context changes

Opening another file or view may change the default focused context only when the change is explicit and observable.

The previous context may remain referenceable only through explicit structured references.

Chat must not silently apply a request to an earlier tab after the active tab changes.

Chat must not use the last opened object as implicit fallback.

---

### !REL_FILE!

# active_object_context

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed active-object context model for DocGraph.

This model separates independent selection by family from optional global focus.

## Core rule

`ActiveObjectContext` is governed context prepared by the system.

It is not pure UI state.
It does not grant permissions.
It does not execute actions.
It does not invent context that the user has not selected or referenced.

## Canonical model

Conceptual structure:

```rust
struct ActiveObjectContext {
    selected_document: Option<DocumentRef>,
    selected_sandbox: Option<SandboxRef>,
    selected_knowledge_item: Option<KnowledgeRef>,
    selected_artifact: Option<ArtifactRef>,
    selected_tool_output: Option<ToolOutputRef>,
    focused_family: Option<ActiveObjectFamily>,
}

enum ActiveObjectFamily {
    Document,
    Sandbox,
    KnowledgeItem,
    Artifact,
    ToolOutput,
}
```

There is no single global active object.

The system may hold:

- zero or one selected document
- zero or one selected sandbox
- zero or one selected knowledge item
- zero or one selected artifact
- zero or one selected tool output
- zero or one focused family

Optional prepared selection context may also exist for the currently selected document without becoming a separate active-object family.

## Family meaning

- `Document`: governed document reference resolved from project context
- `Sandbox`: governed sandbox context or sandbox view target
- `KnowledgeItem`: governed knowledge reference
- `Artifact`: governed artifact reference that is not already modeled as a document, knowledge item, or tool output
- `ToolOutput`: governed tool-output reference

## Selection rule

Selections are independent by family.

Selecting one family must not clear selections in other families unless a stricter governed policy is declared in a future phase.

## Left-click selection in the tree

General rule:

- left-click on a selectable artifact toggles selection inside its family

Cases:

- click on a non-selected artifact:
  - select it inside its family
  - replace any previous selection in that same family
  - set `focused_family` to that family

- click on an already selected artifact:
  - deselect it
  - if that family was focused, set `focused_family` to `None`

- click on an artifact from another family:
  - keep selections in other families
  - select the new artifact in its own family
  - set `focused_family` to the new family

## Relationship to tabs

- `DocumentRef` may activate or open `Readonly Viewer`
- `SandboxRef` may activate or open `Sandbox View`
- other family selections may activate the corresponding governed workspace view in future

Changing the active tab does not clear selections in other families.

An active tab may update `focused_family` only when the change is explicit and observable to the user.

Tab activation must not silently manufacture a selection for a family that has no valid selected reference.

Opening or activating the same governed target must reuse the existing tab identity rather than create duplicate tab context.

## Relationship to chat

`focused_family` determines the default target family for a chat instruction.

Rules:

- if `focused_family = None` and the instruction requires a target, the system must ask for clarification
- if `focused_family` points to a family without a valid selection, the system must ask for clarification
- if the instruction explicitly names a family, the system should use that family when a valid selection exists
- chat must not invent context
- chat must not use the last opened object as fallback

Explicit structured references remain stronger than implicit focus.

When a document selection is scoped further by text selection, the structured `selection_ref` remains subordinate to `document_ref`, not a replacement for it.

## Selection states

Each family selection may conceptually be understood through these minimum states:

- `null`
- `selected`
- `missing`
- `stale`
- `readonly`
- `future_actionable`

Interpretation:

- `null`: no selection exists for that family
- `selected`: a valid governed selection exists
- `missing`: the reference no longer resolves
- `stale`: the reference resolves, but the prepared context is no longer current enough for the intended action
- `readonly`: the selection is valid but cannot be mutated in the current phase or policy
- `future_actionable`: the selection may become actionable only in a future governed phase

These are conceptual readiness states. They do not imply runtime execution.

## Invariants

- `INV-AOC-001`: at most one selection may exist per family
- `INV-AOC-002`: at most one `focused_family` may exist
- `INV-AOC-003`: `focused_family` must correspond to a family with a valid selection or be `None`
- `INV-AOC-004`: the system MUST NOT use implicit fallback to the last opened object
- `INV-AOC-005`: active-object identity MUST NOT be based on physical host paths
- `INV-AOC-006`: selections in one family MUST NOT silently erase selections in other families
- `INV-AOC-007`: focus is a targeting aid, not execution authority

## Failure modes

- `focused_family_without_selection`
- `selected_ref_missing`
- `stale_active_object`
- `ambiguous_chat_target`
- `implicit_last_object_blocked`

Interpretation:

- `focused_family_without_selection`: focus points to a family with no valid selected reference
- `selected_ref_missing`: the stored reference no longer resolves
- `stale_active_object`: the selected context is no longer safe to use as prepared target
- `ambiguous_chat_target`: the instruction requires a target but no unambiguous valid family can be prepared
- `implicit_last_object_blocked`: the system detected an attempt to fall back to a previously opened object without governed selection

## Forbidden responsibilities

`ActiveObjectContext` must not:

- grant permissions
- execute tools
- execute LLM calls
- mutate files
- reinterpret project manifests
- replace `ActionResolution`
- collapse all families into one pseudo-global object

## Related specs

- `docs/specs/active_context.md`
- `docs/specs/document_tree.md`
- `docs/specs/workspace_tabs.md`
- `docs/specs/chat_document_flows.md`
- `docs/specs/local_sandbox_context.md`
- `docs/specs/ui_core.md`
- `docs/specs/text_selection.md`

---

### !REL_FILE!

# ai_governance_f9_5

## Status

F9.5 declarative governance. F10 is not opened.

## Purpose

Complete the F9.5 preparation for AI governance without real AI execution.

The AI layer declares prompts, semantic derivation specs, data sensitivity policy, trace schemas, and semantic proposal schemas.

## Principle

The AI does not act.

The AI suggests.

The system validates.

The human decides.

Future runtime may execute only in later phases.

## Declared resources

- `resources/ai/prompt_specs/extract_concepts_v1.json`
- `resources/ai/semantic_specs/concept_extraction_v1.json`
- `resources/ai/policies/data_sensitivity_policy.json`
- `resources/ai/schemas/prompt_spec.schema.json`
- `resources/ai/schemas/semantic_derivation_spec.schema.json`
- `resources/ai/schemas/data_sensitivity_policy.schema.json`
- `resources/ai/schemas/ai_trace_record.schema.json`
- `resources/ai/schemas/semantic_proposal.schema.json`

## F9.5 guarantees

- `execution_enabled = false`
- no LLM calls
- no tool execution
- no embeddings
- no RDF persistence
- no Oxigraph dependency
- no SPARQL
- no document mutation
- no automatic semantic approval
- no duplicate project pipeline

## Readonly UI surface

`Pipeline / Ontology View` is a readonly/mock surface.

It may show:

- document identity
- chunk identity
- prompt spec
- semantic spec
- semantic proposals
- human review state
- future RDF/Oxigraph disabled flags
- trace/governance metadata

It must not approve proposals, execute derivations, persist RDF, mutate files, invoke LLMs, or run tools.

## Future Oxigraph/RDF

Oxigraph is a future store candidate only.

F9.5 uses:

- `oxigraph_enabled = false`
- `rdf_export_enabled = false`
- `semantic_store_enabled = false`

N-Quads may be named as a future format, but no RDF file is generated.

## Boundaries

- `resources/ai/` declares.
- `document_text_runtime` remains the source of regenerable chunks.
- `ui_core` may hold readonly presentational view models.
- `ui_slint` represents state and callbacks only.
- `project_runtime` remains untouched.

## Related specs

- `docs/specs/semantic_proposal_layer.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/ui_core.md`
- `docs/specs/workspace_tabs.md`


---

### !REL_FILE!

# app_main_menu

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the minimum stable top menu for DocGraph.

## Core rule

The main menu emits structured intent.

The main menu does not execute tools.
The main menu does not call LLM.
The main menu does not decide permissions.
The main menu does not interpret tool catalogs.

## Minimum menu

The main menu remains:

- `File`
- `Preferences`
- `Tools`
- `Help`

## Menu structure

### File

- `New project`
- `Open project`
- `Close project`
- separator
- `Exit`

### Preferences

- `App preferences`
- `Project settings`
- `Credentials management`

Clicking `Preferences` must expose exactly these three entries.

`Preferences` is a top-level configuration domain.

It must remain separate from `Tools`.

Each entry opens its own governed popup surface:

- `App preferences` opens the app-preferences popup
- `Project settings` opens the project-settings popup
- `Credentials management` opens the credentials-management popup

There is no shared Preferences panel.
There are no Preferences tabs.

### Tools

- `Operational Tools`
- `LLM Tools`
- `External Dependencies`

Clicking `Tools` must expose exactly these three entries.

`Tools` is a top-level menu domain.

It must remain separate from `Preferences`.

Each entry opens its own governed panel or popup surface:

- `Operational Tools` opens the operational-tools surface
- `LLM Tools` opens the LLM-tools surface
- `External Dependencies` opens the external-dependencies surface

There is no single combined tools panel with tabs.

### Help

- `Open Lume Help`
- `Help Topics`
- `About DocGraph`

Clicking `Help` must expose exactly these three entries.

`Help` is a top-level informational domain.

It must remain separate from `Preferences` and `Tools`.

Each entry opens its own governed popup surface:

- `Open Lume Help` opens the `Lume Help` popup
- `Help Topics` opens the `LumeHelpTree` popup surface
- `About DocGraph` opens the `AboutPopup`

## Tools menu rule

The main menu routes into the governed tools-management domain.

The menu does not execute tools, prepare hidden execution, install dependencies, or interpret policy.

## Preferences menu rule

The main menu routes into the governed preferences domain.

The menu emits configuration intent only.
The menu does not execute tools.
The menu does not resolve policy.
The menu does not mutate runtime.
The menu does not bypass `ActionResolution`.

## Help rule

`Help` is informational only.

It emits navigation and help intent only.
It does not execute.
It does not resolve policy.
It does not mutate runtime.
It does not interact with tools.
It does not expose credentials.
It does not trigger LLM execution.

`Lume Help` is not LLM-only help.

It must remain usable in `Static Mode`.

`About DocGraph` is the location for about and credits.

## DocGraph icon

- The top app chrome may display the `DocGraph` icon before `File`.
- The icon is branding and a navigation shortcut.
- If clicked, it requests opening `Lume Help`.
- It is equivalent to `Help -> Open Lume Help`.
- It must not execute tools, call LLM, mutate files, change settings, or activate runtime capabilities.
- It must not replace the Help menu entry.

## I18n rule

Visible menu text must come from i18n resources.

The governed top-level menu contract uses these stable keys:

- `menu.file`
- `menu.file.new_project`
- `menu.file.open_project`
- `menu.file.close_project`
- `menu.file.exit`
- `menu.preferences`
- `menu.preferences.app`
- `menu.preferences.project`
- `menu.preferences.credentials`
- `menu.tools`
- `menu.tools.operational`
- `menu.tools.llm`
- `menu.tools.external_dependencies`
- `menu.help`
- `menu.help.lume`
- `menu.help.topics`
- `menu.help.about`

Legacy keys may remain in resources for compatibility, but new specs must not reference:

- `menu.tools.open_panel`
- `menu.tools.external`
- `menu.help.tree`

## Invariants

- `INV-MENU-001`: the main menu MUST remain minimal and stable.
- `INV-MENU-002`: the main menu MUST expose navigation intents, not execution.
- `INV-MENU-003`: the main menu MUST NOT execute tools, LLM calls, filesystem mutations, or project actions.
- `INV-MENU-004`: `Tools` MUST be a top-level menu domain separate from `Preferences`.
- `INV-MENU-005`: `Tools` MUST expose exactly `Operational Tools`, `LLM Tools`, and `External Dependencies`.
- `INV-MENU-006`: each `Tools` entry MUST open its own governed panel or popup surface.
- `INV-MENU-007`: the main menu MUST NOT define a single combined tools panel with tabs.
- `INV-MENU-008`: `Preferences` MUST be a top-level menu domain separate from `Tools`.
- `INV-MENU-009`: `Preferences` MUST expose exactly `App preferences`, `Project settings`, and `Credentials management`.
- `INV-MENU-010`: each `Preferences` entry MUST open a dedicated governed popup surface.
- `INV-MENU-011`: the main menu MUST NOT define a shared Preferences panel with tabs.
- `INV-MENU-012`: `Help` MUST be a top-level menu domain separate from `Preferences` and `Tools`.
- `INV-MENU-013`: `Help` MUST expose exactly `Open Lume Help`, `Help Topics`, and `About DocGraph`.
- `INV-MENU-014`: each `Help` entry MUST open a dedicated informational popup surface.
- `INV-MENU-015`: Help MUST NOT execute actions, invoke tools, invoke LLM execution, access credentials, or mutate runtime.
- `INV-MENU-016`: About/Credits MUST live under Help / About DocGraph.
- `INV-MENU-017`: visible menu text MUST come from i18n resources.
- `INV-MENU-018`: governed menu labels MUST use the stable `menu.*` keys declared by this spec.
- `INV-MENU-019`: deprecated menu keys MUST NOT be used by new specs.
- `INV-BRANDING-001`: the DocGraph icon MAY be displayed in the top-left app chrome as branding.
- `INV-BRANDING-002`: the DocGraph icon MUST NOT execute tools, LLM calls, project actions, filesystem mutations, or settings changes.
- `INV-BRANDING-003`: if clickable, the DocGraph icon MAY open Lume Help as a navigation intent.
- `INV-BRANDING-004`: the DocGraph icon MUST NOT replace the Help menu entry.
- `INV-BRANDING-005`: icon resource paths MUST be declarative and MUST NOT be hardcoded in UI logic.
- `INV-BRANDING-006`: icon tooltip text MUST come from i18n.

## Forbidden responsibilities

The main menu must not:

- execute tools
- call LLM
- mutate filesystem
- resolve policy
- interpret tool catalogs
- place tools inside `Preferences`
- mix `Preferences` with `Tools`
- mix `Help` with `Preferences` or `Tools`
- open F10

## Related specs

- `docs/specs/tools_panel.md`
- `docs/specs/help_menu.md`
- `docs/specs/ui_preferences_popups.md`
- `docs/specs/project_settings_popup.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/credentials_policy.md`
- `docs/specs/lume_help_popup.md`
- `docs/specs/lume_help_tree.md`
- `docs/specs/project_setup_popup.md`

---

### !REL_FILE!

# app_status_bar

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed bottom status bar for DocGraph.

## Core rule

The status bar displays prepared system state.

It does not compute authority, execute tools, call LLM, mutate files, or change configuration.

## Layout

The F9 status bar contains these blocks:

- `project`
- `project_profile`
- `llm`
- `tools`
- `sandbox`
- `mode`

Visual order:

`Project | Profile | LLM | Tools | Sandbox | Mode`

## StatusBarState conceptual contract

Conceptual JSON shape:

```json
{
  "project": {
    "loaded": false,
    "display_name": null
  },
  "project_profile": {
    "profile_id": null,
    "display_i18n": null
  },
  "llm": {
    "desired_llm_mode": "OFF",
    "effective_llm_mode": "OFF",
    "interaction_mode": "GUIDED",
    "llm_available": false,
    "provider_configured": false,
    "credential_ref_present": false,
    "local_engine_available": false,
    "local_model_available": false,
    "policy_allows_local": false,
    "policy_allows_cloud": false,
    "tool_surface_available": false,
    "execution_enabled": false,
    "reason_codes": [
      "llm_off_by_preference",
      "guided_fallback_active"
    ],
    "display_i18n": "status.llm.off"
  },
  "tools": {
    "declared_count": 0,
    "executable_count": 0,
    "active_filter": "all",
    "display_i18n": "status.tools.summary"
  },
  "sandbox": {
    "state": "off",
    "display_i18n": "status.sandbox.off"
  },
  "mode": {
    "lume_mode": "static",
    "display_i18n": "status.mode.static"
  }
}
```

## Allowed LLM states

The status bar consumes prepared `LLMCapabilityState`.

Allowed `desired_llm_mode` and `effective_llm_mode` values:

- `OFF`
- `LOCAL`
- `CLOUD`

Allowed `interaction_mode` values:

- `GUIDED`
- `ASSISTED`

The status bar may display reason codes such as:

- `llm_off_by_preference`
- `local_engine_missing`
- `local_model_missing`
- `local_policy_blocked`
- `cloud_provider_missing`
- `cloud_credential_ref_missing`
- `cloud_policy_blocked`
- `execution_not_open`
- `assisted_mode_unavailable`
- `guided_fallback_active`

Rules:

- status bar MUST NOT display credential values
- status bar MUST NOT display credential secrets
- missing credentials are status, not failure
- LLM absence does not block the app
- `desired_llm_mode` MUST remain distinguishable from `effective_llm_mode`
- `OFF` MUST be displayable as a valid guided mode
- status bar MUST display prepared reason codes rather than infer them

## Tools summary

The tools block shows:

- `executable_count`
- `declared_count`

Format:

`Tools: executable_count/declared_count`

Rules:

- declared vs executable must remain distinguishable
- clicking or selecting tools summary may only navigate to `Tools Panel` in future
- status bar must not execute tools

## Sandbox states

Allowed:

- `off`
- `readonly`
- `working_copy_future`
- `configured_future`

Rules:

- sandbox status is informational
- original external folders are never mutated from the status bar
- write-back is never triggered from the status bar

## Mode states

Allowed:

- `GUIDED`
- `ASSISTED`

Rules:

- `GUIDED` means Lume local or declarative help
- `ASSISTED` means LLM-assisted mode is conceptual or future unless runtime later opens it and capability state is available
- status bar must not activate Assisted mode

## Optional lightweight navigation

Future allowed navigation only:

- LLM block may open Lume Help explanation
- Tools block may open Tools Panel
- Sandbox block may open Lume help topic
- Mode block may open onboarding or help

These are navigation intents only.

They must not execute actions.

## Relationship to Lume

- Lume may explain each status block.
- Status bar may link to Lume Help.
- Lume does not compute status bar state.
- Lume may explain `reason_codes`.
- Status bar and Lume consume the same prepared capability state.

## Relationship to Tool Surface Resolver

- `tools.declared_count` and `tools.executable_count` may come from future effective tool surface summary.
- status bar does not read raw catalogs directly.
- status bar does not infer tool availability.

## Relationship to credentials

- credentials are never shown.
- `credential_ref` is not shown unless future policy explicitly allows non-secret references.
- `credential_ref_present` may influence status text only.

## Invariants

- `INV-STATUSBAR-001`: the status bar MUST display prepared system state only.
- `INV-STATUSBAR-002`: the status bar MUST NOT execute tools, LLM calls, filesystem mutations, or project actions.
- `INV-STATUSBAR-003`: the status bar MUST NOT expose credentials or sensitive data.
- `INV-STATUSBAR-004`: the status bar MUST reflect capability state, not infer or fabricate it.
- `INV-STATUSBAR-005`: the status bar MUST remain usable without LLM.
- `INV-STATUSBAR-006`: the status bar MAY provide navigation intents, but MUST NOT become a control surface.
- `INV-STATUSBAR-007`: displayed text MUST come from i18n resources.
- `INV-STATUSBAR-008`: declared vs executable capability MUST be distinguishable when relevant.
- `INV-STATUSBAR-009`: status bar state MUST be prepared outside UI rendering.
- `INV-STATUSBAR-010`: status bar must not read raw catalogs directly.
- `INV-STATUSBAR-011`: the status bar MUST distinguish desired LLM mode, effective LLM mode, and interaction mode.
- `INV-STATUSBAR-012`: the status bar MUST present `OFF` as a valid guided mode, not an error.
- `INV-STATUSBAR-013`: the status bar MUST NOT decide policy or resolve capability state from UI state.
- `INV-STATUSBAR-014`: status bar navigation intents MUST NOT execute tools, call providers, validate credentials, or approve actions.

## Forbidden responsibilities

The status bar must not:

- execute tools
- activate LLM
- validate credentials
- resolve credentials
- show secrets
- mutate project files
- change preferences
- change project profile
- create sandbox
- trigger write-back
- infer policy from UI state
- duplicate `Tools Panel`
- duplicate `Lume Help`
- open F10

## Related specs

- `docs/specs/lume_onboarding_model.md`
- `docs/specs/llm_tool_surface_resolution.md`
- `docs/specs/credentials_policy.md`
- `docs/specs/tools_panel.md`
- `docs/specs/project_profiles.md`
- `docs/specs/local_sandbox_context.md`

---

### !REL_FILE!

# artifact_graph

## Status

NORMATIVE DOCUMENTATION. This file defines the minimal artifact relationship layer for a DocGraph project. It does not implement a graph runtime.

## Purpose

Define `graph/` as a minimal relationship layer between governed project artifacts.

`graph/` explains relationships. It does not govern, execute, approve, infer, persist RDF, run SPARQL, activate Oxigraph, create embeddings, or replace `project_runtime`.

For semantic material, graph remains explanatory rather than authoritative:

- node = approved semantic assertion instance or other governed artifact node
- edge = explicit governed relation object
- graph does not approve facts
- graph does not execute logic
- graph does not infer new knowledge

Associated graph-edge schemas are declarative contracts only.

Schema validity does not activate graph runtime, RDF projection, graph analysis, or execution.

## Folder contract

```text
graph/
  graph_manifest.json
  artifact_nodes.jsonl
  artifact_edges.jsonl
  snapshots/
```

## Node example

```json
{
  "node_id": "doc_report_f110",
  "type": "document_holder",
  "path": "documents/report_f110"
}
```

## Edge example

```json
{
  "edge_id": "edge_001",
  "from": "doc_report_f110",
  "to": "chat_003",
  "relation": "uses_chat"
}
```

## Minimal relations

- `uses_chat`
- `uses_knowledge_file`
- `cites_source`
- `generated_from`
- `produced_tool_output`
- `derived_export`
- `references_artifact`
- `promotes_to_knowledge`
- `uses_template`
- `reviewed_in`

## Graph update policy

Graph updates MUST be triggered only by governed project actions.

Valid triggers include:

- document creation
- document update affecting references
- tool output registration
- chat-to-document promotion
- knowledge promotion

Graph updates MUST NOT be derived from passive filesystem scanning.

Graph updates MUST NOT infer relations without explicit governed actions.

## Semantic graph principle

For governed semantic relationships:

- node = `QuadInstance` or future approved semantic assertion instance
- edge = `QuadRelation`

Rules:

- relationships must not be embedded as hierarchy inside `SemanticQuad`
- relationships must not be implicit
- relationships must be explicit objects
- graph does not approve facts
- graph does not execute logic
- graph does not infer new knowledge

## QuadRelation model

Conceptual structure:

```json
{
  "relation_id": "rel_xxx",
  "source_quad_instance_id": "inst_A",
  "target_quad_instance_id": "inst_B",
  "relation_type": "supports | contradicts | refines | depends_on | derived_from | equivalent_to | broader_than | narrower_than | uses_as_evidence | supersedes",
  "lifecycle_state": "PROPOSED",
  "evidence": [],
  "metadata": {},
  "trace_ref": "trace_xxx"
}
```

Rules:

- `relation_id` is stable
- source and target must reference `QuadInstance`, not only `quad_id`
- relation lifecycle is independent from quad lifecycle
- relation metadata must not contain secrets
- relation evidence must be traceable

## Controlled relation vocabulary

Allowed `relation_type` values:

- `supports`
- `contradicts`
- `refines`
- `depends_on`
- `derived_from`
- `equivalent_to`
- `broader_than`
- `narrower_than`
- `uses_as_evidence`
- `supersedes`

Rules:

- `relation_type` must not be free text
- new relation types require governed extension
- relation labels are not runtime authority

## Relation lifecycle

Relation lifecycle aligns with semantic lifecycle:

- `PROPOSED`
- `UNDER_REVIEW`
- `APPROVED`
- `REJECTED`
- `STALE`
- `SUPERSEDED`
- `ORPHANED`

Rules:

- relations are `PROPOSED` by default
- `APPROVED` relation requires explicit future review
- invalid source or target instance makes the relation `ORPHANED` or `STALE`
- relation lifecycle is independent, but must remain consistent with source and target states
- no relation may be considered active if source or target is invalid, stale, rejected, orphaned, or superseded
- lazy invalidation is the current declared relation-reaction mode
- eager invalidation remains future-only and separately gated

Relation transition rules are governed by `semantic_quad_lifecycle.md`.

## Relation evidence

Relation evidence may reference:

- shared `file_ref`
- shared `object_ref`
- shared `chunk_id + chunk_hash`
- shared `source_ref`
- `reasoning_trace_ref`
- `review_trace_ref`
- future `tool_run_ref`

Rules:

- evidence references sources rather than duplicated full text
- evidence is not authority by itself
- evidence must not contain secrets
- evidence invalidation affects relation lifecycle

## Conflict handling

- `contradicts` models semantic conflict
- conflicting `PROPOSED` relations may coexist
- conflicting `APPROVED` relations require future governance review
- graph does not automatically resolve conflicts
- graph does not pick winners

## Supersession relationship

Difference:

- lifecycle `SUPERSEDED` marks state
- relation type `supersedes` explains semantic relationship

Rules:

- `supersedes` relation does not by itself mutate lifecycle
- transition to lifecycle `SUPERSEDED` must still be explicit and traceable
- both may refer to the same conceptual replacement while remaining separate mechanisms

## Future graph consumption

Future consumers may rely only on:

- `APPROVED` `QuadInstance` records
- `APPROVED` `QuadRelation` edges

Rules:

- `PROPOSED` relations must not be treated as dependencies
- graph remains explanatory, not authority

Boundaries:

- no graph runtime is opened
- no RDF projection runtime is opened
- no analysis runtime is opened
- no inference engine is opened
- graph reacts to upstream semantic invalidation and must not mutate semantic, derivative, or storage layers

Future RDF relation projection policy is governed by `rdf_projection_policy.md`.

Future graph analysis policy is governed by `graph_analysis_policy.md`.

## Invariants

- `INV-GRAPH-001`: `graph/` explains artifact relations; it is not runtime authority.
- `INV-GRAPH-002`: graph updates MUST originate from governed actions.
- `INV-GRAPH-003`: graph entries MUST be traceable to origin.
- `INV-GRAPH-004`: graph MUST NOT infer relations from passive filesystem scanning.
- `INV-GRAPH-005`: semantic proposals are not approved graph facts.
- The filesystem stores physical location.
- The manifest governs exposure.
- `registry.json` accelerates navigation.
- `graph/` explains relations.
- `graph/` does not decide, execute, approve, or mutate.
- `graph/` does not substitute `project_manifest.json`.
- `graph/` does not substitute `project_runtime`.
- Graph data MUST be consistent with project_manifest exposure.
- Graph MUST NOT introduce relations that are not traceable to governed actions.
- Graph entries MUST be traceable to an origin (document, chat, knowledge, or tool action).
- Artifact relations are traceability data, not runtime authority.
- Semantic relations remain proposals until human review.
- `knowledge/semantic/proposals/` is proposal storage, not an approved semantic store.
- No RDF, Oxigraph, SPARQL, embeddings, N-Quads output, or semantic store is introduced.
- No graph runtime is introduced.
- No tool execution is introduced.
- No UI logic is introduced.
- No project pipeline duplication is allowed.
- `INV-REL-001`: relations are explicit graph edges.
- `INV-REL-002`: relations must not be embedded inside `SemanticQuad` as hierarchy.
- `INV-REL-003`: relations reference `QuadInstance` identities.
- `INV-REL-004`: relations have independent lifecycle.
- `INV-REL-005`: relations are `PROPOSED` by default.
- `INV-REL-006`: `APPROVED` relation requires explicit review.
- `INV-REL-007`: graph does not approve facts.
- `INV-REL-008`: graph does not infer new knowledge.
- `INV-REL-009`: graph does not execute logic.
- `INV-REL-010`: future graph analysis may consume only `APPROVED` instances and `APPROVED` relations.
- `INV-REL-011`: conflict relations do not auto-resolve conflicts.
- `INV-REL-012`: relation evidence must be traceable and secret-free.
- `INV-REL-013`: graph invalidation reacts to upstream semantic changes and must not mutate upstream layers.

## Relationship to semantic proposals

F9.5 `SemanticProposal` records can reference artifacts and graph hints, but they remain proposals, not approved graph facts.

Future `SemanticQuad`, `QuadInstance`, and `QuadSet` materials may align with `graph/` only after explicit governed review.

- `ResolutionCandidate`-like semantic preparation is not graph authority
- `PROPOSED` semantic quads must not become graph facts
- `UNDER_REVIEW`, `REJECTED`, `STALE`, `SUPERSEDED`, and `ORPHANED` instances must not become graph facts
- future graph consumers may rely only on `APPROVED` semantic material
- graph relations must remain traceable to governed actions or future governed semantic approval, not to raw quad presence
- `PROPOSED`, `UNDER_REVIEW`, `REJECTED`, `STALE`, `SUPERSEDED`, and `ORPHANED` relations must not become approved dependencies or approved semantic facts

Future reviewed promotion may create or update governed artifact relations, but that future behavior must pass through the appropriate project, action, and human-review policies. This spec does not implement that behavior.

## Related specs

- `docs/specs/project_folder_layout.md`
- `docs/specs/semantic_proposal_layer.md`
- `docs/specs/project_runtime.md`
- `docs/specs/invalidation_policy.md`

---

### !REL_FILE!

# caching_and_reuse_policy

## Status

DECLARED_ONLY / F9-F10_PREP governance.

This document does not implement cache runtime, semantic-runtime reuse logic, derivative cache runtime, lifecycle mutation, or execution authority.

## Purpose

Define governed reuse and caching rules for semantic quads and derived data.

The goal is to avoid redundant generation, duplicate semantic meaning, and unnecessary regeneration while preserving lifecycle, traceability, and storage boundaries.

## Reuse rules

Future governed reuse should follow these principles:

- reuse quads when the same `quad_id` already exists
- reuse `QuadSet` groupings when the same governed generation context already exists
- reuse deterministic derivatives when equivalent governed source content is unchanged

Rules:

- reuse is preferred over regeneration when identity and governed context match
- reuse must not bypass lifecycle state
- reuse must not convert proposal material into approved knowledge
- reuse must remain bounded by source scope, storage policy, and semantic storage limits

## Semantic deduplication

Identical `quad_id` values must not duplicate semantic meaning.

Rules:

- one semantic meaning should map to one `quad_id`
- multiple `QuadInstance` records may exist for the same `quad_id`
- multiple instances for the same `quad_id` must remain explicitly linked through the shared identity
- duplicate meaning must be represented through reuse of identity rather than creation of a new semantic meaning id

Lifecycle and approval still attach to `QuadInstance`, not to cached identity alone.

## QuadSet reuse

`QuadSet` reuse is allowed when the governed generation context is materially the same.

Conceptually this includes reuse when:

- source scope matches
- generation trigger matches
- generation mode matches
- relevant evidence anchor state matches
- relevant governed generation context matches

If governed generation context differs materially, a new `QuadSet` may be appropriate even when some quads are reused.

## Derivative reuse

Deterministic derivative reuse is preferred when:

- source content hash matches
- governed derivative inputs are unchanged
- derivative contract remains equivalent

Derivative reuse must not:

- bypass invalidation policy
- bypass security and sanitization policy
- be treated as semantic approval

## Conceptual cache model

Future governed caching may use:

- content-hash-based reuse
- generation-context-based reuse

### content-hash-based reuse

Content-hash-based reuse applies when governed source content or evidence anchors are unchanged and deterministic reuse is safe.

### generation-context-based reuse

Generation-context-based reuse applies when the same governed semantic-generation context is repeated and reuse is safer than regeneration.

This remains conceptual only in the current phase.

## Boundary rules

Caching and reuse must not:

- bypass lifecycle filtering
- bypass invalidation
- bypass security and sanitization
- infer project exposure
- mutate storage authority
- imply RDF projection
- imply graph-analysis runtime

Reuse may preserve efficiency, but it does not widen authority.

## Non-goals

This policy does not open:

- runtime caching
- runtime cache persistence
- automatic semantic reuse logic
- runtime deduplication engine
- execution slices

## Caching and reuse invariants

- `INV-REUSE-001`: duplicate semantic meaning should reuse the same `quad_id` rather than creating a new semantic identity.
- `INV-REUSE-002`: reuse is preferred over regeneration when governed identity and context match.
- `INV-REUSE-003`: multiple `QuadInstance` records for the same `quad_id` are allowed but must remain explicitly linked.
- `INV-REUSE-004`: caching and reuse must not bypass lifecycle state or approval boundaries.
- `INV-REUSE-005`: caching and reuse must not bypass invalidation policy.
- `INV-REUSE-006`: caching and reuse must not bypass security and sanitization policy.
- `INV-REUSE-007`: derivative reuse must remain content-based and deterministic.
- `INV-REUSE-008`: governed cache concepts remain declarative only until a later runtime phase explicitly opens them.

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/storage_policy.md`
- `docs/specs/quad_generation_policy.md`
- `docs/specs/semantic_storage_limits.md`
- `docs/specs/invalidation_policy.md`
- `docs/specs/security_sanitization_policy.md`

---

### !REL_FILE!

# chat_document_flows

## Purpose

Define the minimum governed interaction model between tree, clip, chat, and workspace tabs.

## Core rule

Tree = existing governed documents  
Clip = external intake and workflow launch  
Chat = structured references and results  
Workspace tabs = where work actually happens

Onboarding chat = ephemeral help context, not project chat

Future assisted chat must remain Lume-fronted.

All user-facing chat interaction enters and exits through Lume.

Internal Planner, Specialist, and Synthesizer roles are conceptual routing and synthesis roles only; they are not user-facing chat participants.

## Chat session folder contract

```text
chats/
  chat_sessions/
    <chat_id>/
      manifests/
        chat_manifest.json
        context_refs.json
      messages.jsonl
      attachments/
        files/
        figures/
      artifacts/
      tool_outputs/
        <tool_id>/
          <run_id>/
            tool_run_manifest.json
            outputs/
            logs/
```

`attachments/` contains user-provided inputs for the chat session.  
`artifacts/` contains products owned by the chat session.  
`tool_outputs/` contains chat-owned tool outputs and requires an `owner_ref` pointing to the chat session.

## Invariants

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
- `INV-CHAT-011`: point of entry does not redefine the governed documentary workflow
- `INV-CHAT-012`: the viewer remains readonly whether a workflow starts from the tree or from the clip
- `INV-CHAT-013`: chat attachments are session context, not stable project knowledge
- `INV-CHAT-014`: chat artifacts become durable knowledge or document inputs only through explicit promotion
- `INV-CHAT-015`: promotion from chat MUST preserve source chat, message refs, artifact refs, and trace

## Allowed chat payloads

- text
- structured document references
- structured tool results
- system / state messages
- future structured edit requests with referenced selection, active context, and user instruction
- future patch proposal references, previews, and acceptance decisions

## Relationship to active-object context

Chat may consume `ActiveObjectContext` as prepared governed target context.

Rules:

- `focused_family` determines the default target family when the instruction needs one
- if no valid focused family exists, chat must ask for clarification
- if the instruction explicitly names a family, that family may be used only when a valid selection exists
- chat must not invent missing context
- chat must not use the last opened object as fallback

Onboarding chat may carry temporary help text only. It must not persist as project chat or become document storage.

## Future governed LLM request intake

Future LLM-facing chat requests may enter the governed pipeline as `UserInput`.

The user-facing entrypoint is Lume.

Conceptual agent flow:

```text
User
<-> Lume
-> Planner
-> Specialists
-> Synthesizer
-> Lume
-> User
```

That intake may produce:

- `IntentAnalysis`
- `RequestDecomposition`
- `SubRequestPlan[]`

Rules:

- decomposition must preserve user wording references
- ordering must be preserved when order matters
- chat must not execute subrequests
- chat must not bypass governed context requirements
- chat may surface clarification needs explicitly
- chat must not expose internal agents as independent user-facing assistants
- chat must not embed or mutate agent prompts

If a subrequest becomes a `tool_candidate` or executable action candidate, it remains proposal-only until future governed action flow resolves it.

Agent prompts for future conceptual roles are declarative resources governed by `docs/specs/llm_agent_prompts.md`.

## Future referenced editing requests

Future editing requests may enter chat as structured payloads derived from viewer selection or an inspector context.

The payload must reference the target context rather than store a source blob. The LLM may propose a patch, but the system validates it and applies it only after user acceptance.

Chat remains the request and reference surface. It does not become a document store, editor, patch runtime, or filesystem authority.

## Future template proposals

Future LLM-assisted template work may appear in chat as structured `TemplateProposal` or `TemplateDraft` references.

The LLM may suggest creating, editing, renaming, cloning, or deleting a template, but it must not open the template popup directly, execute the operation, or write template files.

Template operations must pass through `ActionResolution` and, when UI capture is needed, through the governed template popup described in `docs/specs/document_template_ui_contract.md`.

## Promotion out of chat

Chat artifacts do not become project knowledge or documents implicitly.

Promotion must be explicit:

- chat artifact to `knowledge/files/` or `knowledge/derived/`
- chat artifact to `knowledge/semantic/proposals/`
- chat artifact to `documents/<document_holder>/`

Promotion must preserve source references, chat context, and traceability. Chat remains a coordination surface, not the durable source of project truth.

## F12.4 intake comment proposals

`docs/specs/file_intake.md` owns governed file intake comment semantics.

Chat may produce a `ChatCommentProposal` for adding or editing an optional `user_comment` tied to an `IntakeItem`.

The flow is:

`user message -> ChatCommentProposal -> explicit promotion step -> FileIntakeDraft / IntakeItem update`

Rules:

- chat output is proposal-only
- chat cannot directly persist intake comments
- chat cannot mutate `IntakeItem`
- chat cannot mutate durable intake metadata
- promotion must be explicit and governed
- promotion must preserve `owner_ref`, `trace_ref`, chat context, and source message reference
- comments must be sanitized before persistence
- comments must not contain secrets, private absolute host paths, credentials, tokens, or raw sensitive data
- comments do not classify, expose, authorize, derive, or register files
- chat must not become a hidden intake metadata store

## Out of scope

- opaque file blobs inside chat
- notebook behavior
- semantic workflows
- automatic tool execution by LLM
- direct execution from decomposed subrequests
- direct opening of template popup by LLM
- LLM-authored template filesystem mutation
- editing through the viewer
- direct LLM file mutation
- onboarding chat as project chat
- direct user interaction with internal agents
- prompt editing or prompt persistence in chat
- hidden agent memory
- implicit promotion from chat artifacts to knowledge or documents
- chat-owned tool outputs without `owner_ref`

---

### !REL_FILE!

# chat_input

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Chat Input` surface for DocGraph.

The input is keyboard-first, compact, controlled multiline, compatible with long text, and extensible with future voice dictation without changing the rule that chat is intent capture rather than execution.

## Core rule

`Chat Input` captures text intent.

It does not execute commands.
It does not resolve targets implicitly.
It does not call LLM directly.
It does not contain domain logic.

## Keyboard behavior

### Enter

- `Enter` sends the message

Rules:

- `Enter` only sends text currently present in the input
- `Enter` does not execute actions directly
- `Enter` is send-intent only

### Shift+Enter

- `Shift+Enter` inserts a new line

### Ctrl+Enter

- `Ctrl+Enter` may be supported as optional send shortcut for advanced users

## Send button

A send button may be present as fallback.

It is not the primary interaction path.

It must not introduce additional logic beyond the same governed submit intent.

## Multiline input

The input is a multiline textarea.

Rules:

- controlled auto-grow
- maximum height around 5-6 lines
- after the limit, internal scroll is required
- infinite vertical growth is prohibited
- the input must not push the main layout indefinitely

## Long text behavior

Pasting large text must not break UI layout.

Rules:

- internal scroll is required
- `char_count` may be shown optionally
- collapsed preview for very large text may exist optionally

Large text remains editable text input, not a command surface.

## Microphone / dictation

A microphone button may be visible in `Chat Input`.

Interaction:

- first click starts recording
- second click stops recording

## Microphone states

- `idle`
- `recording`
- `transcribing`
- `transcription_ready`
- `mic_unavailable`
- `error`

## Audio activity visualization

Audio activity may be shown as compact waveform or bar visualization.

Rules:

- render it outside the textarea
- do not mix audio activity visual with editable text content
- keep it compact

## Transcription

Audio produces text.

Rules:

- transcription result is inserted into the input
- result remains editable before send
- transcription is never sent automatically
- chat works with text, not audio
- voice does not execute actions

## Availability

If no audio input or STT is available:

- state becomes `mic_unavailable`
- normal text input remains fully available

Voice capability absence must not block keyboard input.

## Conceptual model

### ChatInputDraft

Minimum conceptual fields:

- `raw_text`
- `line_count`
- `char_count`
- `source`
- `has_overflow`
- `submitted_at`, future optional

Field meaning:

- `raw_text`: current editable text
- `line_count`: current visible or logical line count
- `char_count`: text length metric
- `source`: `keyboard` or `voice`
- `has_overflow`: whether input is currently using internal scroll
- `submitted_at`: future timestamp for submit trace when needed

## Input states

Prepared input state may include:

- `idle`
- `typing`
- `multiline`
- `overflow_scroll`
- `recording`
- `transcribing`
- `transcription_ready`
- `mic_unavailable`
- `submitting`
- `error`

Interpretation:

- `idle`: no active input interaction
- `typing`: user is entering text
- `multiline`: content spans more than one line but remains within controlled height
- `overflow_scroll`: content exceeds height limit and uses internal scroll
- `recording`: microphone is actively capturing audio
- `transcribing`: audio is being converted to text in future governed flow
- `transcription_ready`: text from voice capture is ready and editable
- `mic_unavailable`: voice capture cannot be used
- `submitting`: governed send intent is being prepared
- `error`: capture or submit preparation failed safely

## Non-goals

- no command execution
- no implicit target resolution
- no audio persistence by default
- no direct LLM invocation
- no direct STT provider integration

## Failure modes

- `empty_submit`
- `input_too_large_future`
- `mic_permission_denied`
- `mic_unavailable`
- `transcription_failed`
- `submit_while_recording_blocked`

Interpretation:

- `empty_submit`: send was requested with no meaningful text
- `input_too_large_future`: future size policy blocks current input
- `mic_permission_denied`: microphone permission was denied
- `mic_unavailable`: recording or STT capability is unavailable
- `transcription_failed`: voice-to-text failed safely
- `submit_while_recording_blocked`: send is blocked while recording is still active

## Invariants

- `INV-CINPUT-001`: `Enter` MUST prepare send intent only; it MUST NOT execute actions
- `INV-CINPUT-002`: `Shift+Enter` MUST insert newline, not send
- `INV-CINPUT-003`: multiline growth MUST remain bounded
- `INV-CINPUT-004`: internal overflow scroll MUST protect the main layout from unlimited growth
- `INV-CINPUT-005`: voice transcription MUST remain editable before send
- `INV-CINPUT-006`: transcribed text MUST NOT be sent automatically
- `INV-CINPUT-007`: absence of mic or STT MUST NOT block normal text input
- `INV-CINPUT-008`: chat input MUST remain an intent-capture surface, not an execution surface

## Related specs

- `docs/specs/chat_document_flows.md`
- `docs/specs/ui_core.md`

---

### !REL_FILE!

# credentials_policy

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define how DocGraph must treat future credentials without exposing secrets or breaking portability.

This spec governs credential handling boundaries.
Menu and popup surfaces are governed separately.

## Core rule

Credentials are secrets.
Credentials are not preferences.
Credentials are not project configuration.
Credentials MUST NOT be stored inside project files.

## Credential references

Project configuration may store only references, never secret values.

Allowed example:

```json
{
  "credential_ref": "cred_openai_default"
}
```

Forbidden example:

```json
{
  "api_key": "secret_value"
}
```

## Credential storage model

F9 declares only the conceptual model:

- credentials live outside project files
- credentials may be resolved by a future secure local store
- credentials may be resolved by an OS-level secret manager
- project files contain only `credential_ref`
- `credential_ref` is not a secret by itself

## LLM boundary

- LLM context MUST NOT include credentials.
- LLM context MUST NOT include raw secret values.
- LLM may receive capability state, never secret material.
- Tool calls must receive credentials only through future authorized runtime resolution.

## Logging boundary

- credentials MUST NOT be written to logs
- credentials MUST NOT appear in `tool_run_manifest.json`
- credentials MUST NOT appear in `graph/`
- credentials MUST NOT appear in chat history
- credentials MUST NOT appear in exported documents

## Project Setup / Settings relationship

Project Setup / Settings may capture intent to configure credentials.

In F9:

- no credential is stored
- no credential is validated
- no credential is resolved
- only future conceptual `credential_ref` may be recorded

In the governed Preferences menu model:

- `Credentials management` is a dedicated popup surface
- the popup manages references and status only
- the popup MUST NOT display raw secret values
- the popup emits `ConfigurationIntent`, not `ExecutionIntent`
- changing credentials MUST NOT enable execution by itself

## Invariants

- `INV-CRED-001`: credentials MUST NOT be stored in project files.
- `INV-CRED-002`: project files MAY store credential references, not secret values.
- `INV-CRED-003`: credentials MUST NOT be exposed to LLM context.
- `INV-CRED-004`: credentials MUST NOT be written to logs, chats, manifests, graph, `tool_run_manifest`, or exports.
- `INV-CRED-005`: credential resolution is future runtime behavior and is not implemented in F9.
- `INV-CRED-006`: UI may capture credential intent but must not become credential authority.
- `INV-CRED-007`: `credential_ref` is an identifier, not execution permission.
- `INV-CRED-008`: changing `credential_ref` MUST NOT enable execution by itself.
- `INV-CRED-009`: `Credentials management` MUST remain separate from `Preferences` content and `Tools` capability flow even when routed from the main menu.
- `INV-CRED-010`: UI MUST NOT display raw secrets in credentials-management surfaces.
- `INV-CRED-011`: status such as `configured` or `missing` MUST NOT imply execution authority.

## Forbidden responsibilities

This spec must not:

- implement secret storage
- select OS credential manager
- validate real API keys
- call providers
- expose credentials to LLM
- write secrets to project files
- create cloud execution
- open F10

## Related specs

- `docs/specs/ui_preferences_popups.md`
- `docs/specs/project_settings_popup.md`
- `docs/specs/preferences_policy.md`

---

### !REL_FILE!

# diff_view

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Diff View` surface for transformation proposals in DocGraph.

The goal is to support visual comparison without introducing editing, persistence, or application behavior.

## Dependencies

This spec depends on:

- `docs/specs/transformation_core.md`
- `docs/specs/patch_preview.md`
- `docs/specs/document_viewer.md`

## Core rule

`Diff View` is visualization only.

It does not edit.
It does not apply changes.
It does not mutate the document.

## Inline Preview

Inline preview is the default review mode.

Characteristics:

- the proposal is shown in context of the document
- it uses the pending proposal overlay model
- it uses soft blue highlight
- it is oriented to novice users

Inline preview prioritizes contextual readability over formal comparison density.

## Side-by-side Diff

Side-by-side diff is on demand.

Conceptual entry:

- `Ver cambios`

Characteristics:

- shows original vs proposed content
- supports more rigorous review
- is not the default view

Side-by-side review is a comparison surface, not a second editor.

## Review rules

- diff is visualization
- diff does not edit
- diff does not apply changes
- diff does not mutate the document
- side-by-side is not default

## Color semantics

- original = neutral
- proposal = soft blue
- optional internal changes = slightly stronger but still soft blue

Default presentation must avoid aggressive red/green commit semantics.

The visual language should communicate:

- original text remains neutral baseline
- proposed text is pending and uncommitted
- comparison remains readable for long-form document review

## States

Prepared diff-view state may include:

- `available`
- `unavailable`
- `stale`
- `superseded`
- `error`

Interpretation:

- `available`: governed diff data is ready for visual presentation
- `unavailable`: diff rendering is not currently available in prepared state
- `stale`: proposal or source context no longer matches current document integrity
- `superseded`: a newer proposal replaced the diff currently being viewed
- `error`: diff rendering failed safely

## Failure modes

- `missing_original_text`
- `missing_proposed_text`
- `stale_proposal`
- `diff_generation_unavailable`

Interpretation:

- `missing_original_text`: original comparison text cannot be prepared
- `missing_proposed_text`: proposed comparison text cannot be prepared
- `stale_proposal`: proposal is no longer current enough for safe review
- `diff_generation_unavailable`: diff comparison state is unavailable in current governed phase or prepared state

## Relationship to PatchPreview

Inline preview remains the default path and should align with `PatchPreviewOverlay`.

`Diff View` is the optional deeper inspection surface.

It must not replace the inline overlay as the default novice path.

## Relationship to Document Viewer

Inline preview may be rendered in `Document Viewer`.

Side-by-side diff may be shown in a separate readonly review surface when requested.

Neither surface may edit or apply the proposal.

## Invariants

- `INV-DIFF-001`: `Diff View` MUST remain visualization-only
- `INV-DIFF-002`: inline preview MUST remain the default review path
- `INV-DIFF-003`: side-by-side diff MUST remain optional and on demand
- `INV-DIFF-004`: default proposal highlighting MUST avoid aggressive red/green semantics
- `INV-DIFF-005`: diff rendering MUST NOT mutate source or proposal state

## Related specs

- `docs/specs/transformation_core.md`
- `docs/specs/patch_preview.md`
- `docs/specs/document_viewer.md`
- `docs/specs/transform_popup.md`

---

### !REL_FILE!

# document_creation_flow

## Status

PROPOSAL / F9-ready declarative governance. This document defines a future flow only. It does not implement package creation, runtime mutation, export, LLM calls, tool execution, or UI behavior.

## Purpose

Define the future governed flow for creating a Markdown `ARTIFACT` document package from a declarative template.

## Future conceptual flow

1. The user opens a governed template creation popup.
2. UI captures a `TemplateDraft`.
3. UI emits a structured draft request.
4. Future resolution selects a template using `project > user > resources`.
5. Future validation checks the draft against template rules.
6. Future policy/action resolution decides whether confirmation is required.
7. Only an authorized future runtime may materialize the package after acceptance.
8. The created package contains `document.md` as the authoritative editable source.

In F9 this flow is documentation only.

## TemplateDraft boundary

`TemplateDraft` is user intent captured by UI. It is not a document package and is not execution.

It may conceptually include:

- draft id
- selected `template_id`
- selected conformity mode
- document title
- document kind
- language
- metadata overrides
- selected export profile ids
- selected `reference_style_id`
- guidance overrides

## Future package result

A future created package should follow `docs/specs/document_package.md` and include:

- `document.md`
- `document_manifest.json`
- `template_snapshot.json`
- `template_overrides.json`
- `effective_template.json`
- `export_profiles.json`
- `reference_entries.json`
- `reference_style.json`
- `bibliography_sources.json`
- `assets/`
- `derived/latex/`
- `derived/pdf/`
- `derived/docx/`

## Non-execution guarantee

This flow does not allow:

- package creation in F9
- filesystem mutation in F9
- export execution
- LLM calls
- tool execution
- document mutation
- project manifest reinterpretation
- duplication of `project_runtime`

## Related specs

- `docs/specs/document_template_ui_contract.md`
- `docs/specs/document_template_resolution.md`
- `docs/specs/document_template_validation.md`
- `docs/specs/document_package.md`
- `docs/specs/action_resolution.md`


---

### !REL_FILE!

# document_export

## Status

PROPOSAL. This document defines future governed export only. No export runtime, external tool execution, or UI-owned logic is implemented.

## Purpose

Define a governed and reproducible export model for `DocumentHolder` packages where Markdown remains canonical source and every export artifact is regenerable.

## Canonical source

`document.md` is the only editable source for document export.

Export must treat the following as inputs to future preparation only:

- `document.md`
- selected `export_profile`
- `template_snapshot.json` and/or `effective_template.json`
- `reference_style`
- bibliography sources

Export must not treat the following as source of truth:

- `derived/latex/document.tex`
- `derived/pdf/document.pdf`
- `derived/docx/document.docx`
- UI state
- viewer state
- hidden chat state
- external binary defaults

## Derived artifacts

Future export may materialize the following regenerable outputs under `DocumentHolder`:

- `derived/latex/document.tex`
- `derived/pdf/document.pdf`
- `derived/docx/document.docx`

Rules:

- Markdown is canonical.
- LaTeX is derived, not authoritative.
- PDF and DOCX are final artifacts only.
- Derived artifacts are never editable source of truth.
- Export artifacts must remain regenerable from governed inputs.

## Conceptual export pipeline

The future conceptual pipeline is:

`document.md`
`+ export_profile`
`+ template_snapshot / effective_template`
`+ reference_style`
`+ bibliography_sources`
`-> LaTeX`
`-> PDF / DOCX`

This pipeline is declarative only in the current phase.

It does not:

- execute Pandoc
- execute Tectonic
- execute LibreOffice
- execute any external binary
- mutate the source document
- create a runtime runner
- move orchestration into `project_runtime`

## Export profile model

Export profiles live as declarative resources under:

`resources/export_profiles/<profile_id>.json`

Each profile must define:

- `id`
- `target_format` (`latex` | `pdf` | `docx`)
- `template_ref`
- `citation_style`
- `include_toc`
- `include_cover`
- `page_geometry`
- `font_config`
- `extra_args`

Rules:

- profiles are declarative resources only
- profiles do not imply that any external tool exists
- profiles do not authorize execution
- `extra_args` is opaque and must not encode hardcoded binary paths
- profile resolution is future governance, not current runtime

## External dependency model

Potential future export toolchains such as:

- Pandoc
- Tectonic
- LibreOffice

must be treated as external tools.

Rules:

- external tools are resolved via configuration, not code
- external tools must not be hardcoded in runtime crates
- external tools are not required for F9 or F10 execution
- declaring an export profile does not require any toolchain to be installed

## Traceability requirement

In a future execution phase, each export should produce:

- `export_manifest.json`

That future manifest should include at least:

- `source_hash`
- `profile_id`
- `timestamp`
- `toolchain_reference`

This traceability requirement is future-ready only. It does not enable export execution now.

## Invariants

- `INV-DOC-EXPORT-001`: Markdown is the only editable source.
- `INV-DOC-EXPORT-002`: LaTeX is derived and not authoritative.
- `INV-DOC-EXPORT-003`: PDF and DOCX are final artifacts only.
- `INV-DOC-EXPORT-004`: export is fully regenerable.
- `INV-DOC-EXPORT-005`: export must not mutate source content.
- `INV-DOC-EXPORT-006`: no UI-driven export logic is allowed.
- `INV-DOC-EXPORT-007`: no hardcoded binary paths are allowed.
- `INV-DOC-EXPORT-008`: export profiles are declarative and non-executing.
- `INV-DOC-EXPORT-009`: LaTeX and PDF are never source of truth.
- `INV-DOC-EXPORT-010`: external tools are future dependencies, not current runtime requirements.

## Forbidden responsibilities

This spec must not:

- implement export runtime
- create new runtime crates
- wire export into `project_runtime`
- assume execution enabled
- place logic in UI
- treat LaTeX as source
- treat PDF as source
- enable RDF, graph analysis, LLM execution, or tool execution

## Future notes

If export is opened in a later phase, implementation must preserve:

- Markdown as canonical source
- reproducible derived artifacts
- configuration-based external tool resolution
- auditable manifest production
- separation between declarative profiles and executable runtime

---

### !REL_FILE!

# document_governed_editing

## Purpose

Document the future governed document editing model without implementing runtime, UI behavior, or new crates.

This is a proposal for a later phase above the CLOSED F8 workspace model.

## Document classes

- `SOURCE`: original source document, readonly.
- `DERIVED`: text, pages, chunks, technical derivatives, or future semantic derivatives; readonly and regenerable.
- `ARTIFACT`: user or LLM-assisted work product; mutable only through governed patch acceptance.

`SOURCE` and `DERIVED` may be used as context, but they are not mutation targets.

## Active artifact and active tab

The active workspace tab defines the default working context.

- active Document Viewer: document context
- active Figure Inspector: figure or figure-metadata context
- future Table Inspector: table context
- future Equation Inspector: equation context

If the active target is ambiguous, the system must request confirmation before building an edit request.

## Readonly viewer selection

The viewer remains readonly.

Future behavior may allow the user to select text in the viewer and open a contextual action, for example through right click or a controlled action surface.

The contextual popup contract is defined in `docs/specs/transform_popup.md`.

The popup input captures the user's instruction and converts it into a structured governed request containing:

- document reference
- selection reference
- active context
- user instruction
- optional word-count target
- optional length constraint

The selection does not make the viewer an editor.

## Governed modification flow

The user does not directly edit documents.

The future workflow is:

1. user selects, comments, or gives an instruction
2. LLM proposes a change
3. system validates the proposal
4. user accepts, rejects, or requests regeneration
5. runtime applies the accepted patch only after acceptance
6. accepted modification creates a version or `patch_log`

The canonical proposal and regeneration vocabulary must follow `docs/specs/transformation_core.md` and `docs/specs/transform_popup.md`.

## Preview requirement

Every mutation proposal must produce a preview before application.

Preview is mandatory for:

- text replacement
- paragraph replacement
- section rewrite
- structural movement
- table metadata update
- figure metadata update
- equation update

## Versioning and patch log

Every accepted modification must be traceable.

The future runtime must create either:

- a new artifact version
- a `patch_log` entry
- both, when required by the artifact policy

The log must preserve enough information to audit, reverse, or replay the accepted change deterministically.

Future structured memory of attempts, proposals, and decisions is governed separately by `docs/specs/transform_timeline.md` and must not be collapsed into the same role as `patch_log`.

## Phase limits

This spec does not implement:

- document mutation runtime
- editor UI
- rich drafting surface
- renderer or export pipeline
- LLM-driven filesystem mutation
- new crates

It only records the governed editing proposal for future F10/F11 alignment.

---

### !REL_FILE!

# document_package

## Status

PROPOSAL. This is a declarative package model only. No package runtime is implemented.

## Purpose

Define a self-contained `DocumentHolder` folder shape for future Markdown `ARTIFACT` documents created from templates.

`DocumentHolder` is the document production unit inside a DocGraph project. It owns the editable Markdown source, governed manifests, template state, references, assets, lifecycle state, owner-scoped tool outputs, and regenerable exports.

## Proposed package layout

```text
document_package/
  document.md
  manifests/
    document_manifest.json
    source_context.json
  template/
    template_snapshot.json
    template_overrides.json
    effective_template.json
  references/
  assets/
  lifecycle/
    status.json
    review_log.jsonl
  tool_outputs/
    <tool_id>/
      <run_id>/
        tool_run_manifest.json
        outputs/
        logs/
  derived/
    latex/
    docx/
    pdf/
```

## File roles

- `document.md`: authoritative editable Markdown source.
- `manifests/document_manifest.json`: local package metadata and document identity.
- `manifests/source_context.json`: governed source and context references used to create or revise the document.
- `template/template_snapshot.json`: immutable local copy of the base template used at creation time.
- `template/template_overrides.json`: typed local changes over the snapshot.
- `template/effective_template.json`: derived, regenerable result of snapshot plus overrides.
- `references/`: structured internal, bibliographic, and cross-reference entries.
- `assets/`: package-local document assets, not application runtime assets.
- `lifecycle/status.json`: document state metadata for the DocumentHolder.
- `lifecycle/review_log.jsonl`: append-oriented review history for the DocumentHolder.
- `tool_outputs/<tool_id>/<run_id>/`: document-owned tool outputs with `tool_run_manifest.json`, `outputs/`, and `logs/`.
- `derived/latex/`: future regenerable LaTeX output.
- `derived/docx/`: future regenerable DOCX output.
- `derived/pdf/`: future regenerable PDF output.

## Rules

- `INV-DOC-HOLDER-001`: `DocumentHolder` is the owner of document production state.
- `INV-DOC-HOLDER-002`: `document.md` is the only editable textual source.
- `INV-DOC-HOLDER-003`: `derived/` contains regenerable exports only.
- `INV-DOC-HOLDER-004`: `tool_outputs/` inside `DocumentHolder` require `tool_run_manifest.json` and `owner_ref`.
- `INV-DOC-HOLDER-005`: `lifecycle/` records state and review only; it is not a workflow runner.
- `INV-TOOL-OUTPUT-003`: document-contributing outputs MUST live under the owning `DocumentHolder`.
- `template_snapshot.json` preserves the original template base.
- `template_overrides.json` records typed local changes.
- `effective_template.json` is derived and regenerable.
- `document.md` is the textual authority.
- `derived/*` contains regenerable outputs only.
- package-local `assets/` are document content assets, not runtime resources.
- references are structured entities, not opaque citation text.
- BibTeX sources are knowledge or bibliography resources, not mutable document bodies.
- tool outputs created for a document must be scoped under the owning `DocumentHolder`.
- document-owned tool output manifests must carry an `owner_ref` pointing to the `DocumentHolder`.
- `lifecycle/` is limited to document status and review data. It is not a workflow runner.
- There is no project-root `outputs/` folder for document exports.

## Forbidden responsibilities

The package model must not imply:

- export execution
- editor implementation
- filesystem mutation outside explicit future package operations
- LLM calls
- external tool execution
- project manifest reinterpretation
- UI-owned document truth

## Future notes

Future package operations should treat the package as user-owned document data and should not move project pipeline responsibility into document export logic.

Future document creation from templates is described in `docs/specs/document_creation_flow.md`.

The UI capture boundary for template drafts is described in `docs/specs/document_template_ui_contract.md`.

---

### !REL_FILE!

# document_patch_runtime

## Purpose

Define the future patch-runtime concept for governed document editing.

This is a proposal only. No runtime is implemented by this spec.

## Future runtime role

A future `document_patch_runtime` may validate, preview, and apply accepted patches to `ARTIFACT` documents.

Popup capture, proposal creation intent, and acceptance-before-application semantics are defined upstream by `docs/specs/transform_popup.md` and `docs/specs/transformation_core.md`.

It must not:

- duplicate the `project_runtime` pipeline
- widen `ProjectRuntimeOutput`
- live inside UI crates
- allow LLMs to write files directly
- mutate `SOURCE` or `DERIVED` documents

## Patch types

Future patch proposals may include:

- `TextPatch`
- `StructuralPatch`
- `TablePatch`
- `FigureMetadataPatch`
- `EquationPatch`

Patch exchange structures may include:

- `PatchProposalSet`
- `PatchPreview`
- `PatchAcceptance`

When aligned with transformation-core vocabulary, these future runtime structures should map cleanly to `TransformProposal`, preview state, and accepted decision trace rather than redefining them independently.

## PatchProposalSet

A `PatchProposalSet` is a numbered set of candidate changes.

Each proposal must identify:

- target artifact
- target range or structured target
- patch type
- proposed operation
- validation status
- preview availability

The user accepts one proposal or requests regeneration through the governed popup flow.

F9 popup governance does not require a separate reject button surface.

## Deterministic minimum operation

`replace_exact_text` is the minimum deterministic text operation.

It requires:

- one target artifact
- exact current text
- replacement text
- validation that exactly one match exists
- preview before application
- acceptance before write

## Future operations

Future governed operations may include:

- `replace_exact_text`
- `replace_paragraph`
- `rewrite_section`
- `add_section`
- `remove_section`
- `move_section`
- `rename_section`
- `insert_table_reference`
- `insert_figure_reference`
- `insert_equation_reference`
- `update_figure_metadata`
- `update_table_metadata`
- `update_equation_latex`

## Error model

The future runtime must reject unsafe or ambiguous proposals.

Required errors:

- `no_match`
- `multi_match`
- `stale_artifact`
- `forbidden_target`

## Application rule

The runtime applies only an accepted patch.

LLM output is proposal input, not filesystem authority.

The user validates intent; the runtime validates determinism and target safety.

---

### !REL_FILE!

# document_references

## Status

PROPOSAL. This document defines future governed document references, bibliography, and citation/export style contracts. It does not implement runtime, BibTeX parsing, export, LLM calls, tool execution, or document mutation.

## Purpose

Define references as structured document entities rather than opaque plain text.

The document template may define:

- structure
- layout
- editorial guidance
- visual style
- export behavior
- references
- bibliography

Markdown keeps symbolic or structured reference intent. Future LaTeX, PDF, and DOCX export materializes final citation formatting from structured entries plus declarative style.

## Conceptual source set

Future export may use:

```text
document.md
+ document_manifest.json
+ effective_template.json
+ reference_entries.json
+ reference_style.json
+ bibliography_sources.json
+ export_profile.json
-> future LaTeX / PDF / DOCX
```

UI state is not part of the source set.

## References vs governed storage

Structured references may point to governed content identifiers or logical stored objects, but they do not duplicate physical content.

- refs declare usage
- refs do not define storage authority
- refs do not imply project exposure
- schema validity does not imply project exposure
- `project_manifest` remains the exposure authority
- future reference structures must remain path-independent when a governed `file_ref` is available

## Reference kinds

### Internal active-project references

- `SOURCE`
- `DERIVED`
- `ARTIFACT`
- `knowledge/*`
- future figures, tables, and equations

Internal references may be used as sources, evidence, or appendices without mutating the referenced object.

### External bibliographic references

- BibTeX
- DOI
- ISBN
- URL
- manual metadata

BibTeX files may be knowledge resources when declared or placed under governed knowledge surfaces. Future import creates structured bibliography entries; it does not convert the BibTeX file into a mutable document.

### Internal cross-references

- sections
- tables
- figures
- appendices

## ReferenceEntry examples

Project document reference:

```json
{
  "ref_id": "proj_src_001",
  "kind": "project_document",
  "target": "source://requirements/original.pdf",
  "label": "Documento de requisitos original",
  "citation_role": "evidence",
  "export_behavior": "footnote"
}
```

BibTeX reference:

```json
{
  "ref_id": "smith2024",
  "kind": "bibtex_entry",
  "source": "knowledge/references/main.bib",
  "bibtex_key": "smith2024",
  "citation_role": "bibliographic",
  "export_behavior": "bibliography"
}
```

## Reference style kinds

- `standard`: established styles such as APA, IEEE, or Chicago.
- `corporate`: official organization-owned styles.
- `custom_project`: project-local styles.

Standard and corporate styles may coexist, but each style must be explicitly identified.

## Corporate reference style example

```json
{
  "style_id": "corp_official_letter_refs_v1",
  "style_kind": "corporate",
  "display_name": "Corporate official letter references",
  "labeling": {
    "mode": "alphabetic",
    "format": "lowercase_letter",
    "entry_pattern": "{label}) {title}",
    "citation_pattern": "Referencia {label})"
  },
  "placement": {
    "block": "references_internal",
    "position": "before_sections"
  },
  "export_targets": ["latex", "pdf", "docx"]
}
```

The future system generates labels such as `a)`, `b)`, and `c)`. Users should not manually maintain those labels.

## Layout blocks

A template may define export layout order independently from editing order:

- `cover_page`
- `table_of_contents`
- `references_internal`
- `sections`
- `list_of_figures`
- `list_of_tables`
- `bibliography`
- `appendices`

## Forbidden responsibilities

This proposal must not:

- implement a BibTeX parser
- import real bibliographic data
- execute Pandoc, Tectonic, LibreOffice, or external tools
- create `document_reference_runtime`
- create `document_export_runtime`
- call LLMs
- mutate documents
- make BibTeX files editable documents
- hardcode corporate styles
- place reference logic in UI
- touch `project_runtime`

## Invariants

- `INV-DOC-REF-001`: document references must be structured entities, not opaque plain text.
- `INV-DOC-REF-002`: templates may declare citation and reference style.
- `INV-DOC-REF-003`: original active-project documents may be referenced as internal sources, evidence, or appendices without mutation.
- `INV-DOC-REF-004`: BibTeX files may be bibliographic knowledge resources when declared or located in governed surfaces.
- `INV-DOC-REF-005`: future BibTeX import creates structured bibliography entries; it does not convert BibTeX into a mutable document.
- `INV-DOC-REF-006`: future export resolves references from structured entries plus template style, not UI heuristics.
- `INV-DOC-REF-007`: Markdown keeps symbolic or structured references; LaTeX and DOCX materialize final format.
- `INV-DOC-REF-008`: referenced `SOURCE` and `DERIVED` remain readonly.
- `INV-DOC-REF-009`: templates may define numeric, alphabetic, symbolic, or standard bibliographic labeling.
- `INV-DOC-REF-010`: templates may define the position of the references block in layout.
- `INV-DOC-REF-011`: citation patterns must be declarative and derivable from `reference_style`.
- `INV-DOC-REF-012`: non-academic reference styles must be supported.
- `INV-DOC-REF-013`: label generation such as `a)`, `b)`, `c)` is future system responsibility, not user responsibility.
- `INV-DOC-REF-014`: standard and corporate styles must be supported as declarative resources.
- `INV-DOC-REF-015`: corporate styles must not be hardcoded in code or UI.
- `INV-DOC-REF-016`: templates may select a corporate or standard `reference_style_id`.
- `INV-DOC-REF-017`: company styles may define labeling, placement, citation patterns, and listing rules.
- `INV-DOC-REF-018`: standard and corporate styles may coexist but must be explicitly identified.

## Future notes

A later phase may add loaders, validators, importers, or exporters. Those systems must preserve structured references as the source of citation truth and keep corporate style definitions in resources.

## F12.4 file intake reference alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

Future intake items may be referenced by governed refs such as `intake_batch_ref`, `intake_item_id`, `file_ref`, `object_ref`, `owner_ref`, and `trace_ref`.

Source paths and filenames must not become document identity.

Chat may reference intake items but must not become a hidden blob store.

F12.4 intake comments are referenced as metadata on an `IntakeItem`, not as standalone documents or document references.

Comment references may use the governed `intake_item_id`, `owner_ref`, and `trace_ref`.

Comments must not become citation authority, classification authority, exposure authority, derivation authority, or hidden chat-owned project metadata.

## F12.5 file intake reference plan

`F12.5` is plan-only and adds no reference runtime.

Future `F12.6` metadata may reference intake candidates by `intake_item_id`, `intake_batch_ref`, `object_ref` or `stored_object_candidate_ref`, `file_ref` or `content_hash`, `owner_ref`, and `trace_ref`.

Future `F12.6` must not use source path, filename, or chat comment text as durable identity.

---

### !REL_FILE!

# document_templates

## Status

PROPOSAL. This document defines future governed Markdown document templates. It does not implement runtime, export, editor behavior, LLM behavior, or tool execution.

## Purpose

Define a declarative model for creating `ARTIFACT` documents from governed templates.

A document template is a layered contract:

`structure + style + export + references + bibliography + editorial guidance`

Templates are resources. They are not execution logic.

## Core model

Each assisted document creation starts from either:

- an explicit declarative template
- a governed default template

The instantiated document is an `ARTIFACT` whose textual source is Markdown.

`document.md` is the authoritative editable source.

Future LaTeX, PDF, and DOCX outputs are derived and regenerable.

## Template identity and ownership

Every template should declare stable identity and governance metadata:

- `template_id`: stable technical identifier; it must not change when a visible name changes.
- `display_name`: user-facing name; user-owned templates may rename this value through a governed future operation.
- `source_layer`: `resources`, `user`, or `project`.
- `ownership`: `system`, `user`, or `project`.
- `editable`: whether the template body may be changed through future governed operations.
- `deletable`: whether the template may be deleted through future governed operations.
- `renameable`: whether `display_name` may be changed through future governed operations.
- `cloneable`: whether a copy may be created from this template.
- `cloned_from`: optional source template reference for cloned templates.

System templates are readonly and may not be edited, renamed, or deleted. They may be cloneable.

User templates may be created, edited, renamed, deleted, or cloned only when future policy permits it.

Project templates are future scoped resources and must remain governed by project-local policy.

Changing `display_name` must not change `template_id`.

Cloning a system template creates a user or project template copy. The clone must preserve `cloned_from`.

## Conformity modes

- `strict`: only template-compatible changes are accepted.
- `guided`: deviations are allowed with warnings.
- `free_from_template`: the template is used as a starting point, with no strict conformance after creation.

## Template layers

- `structure`: document type, sections, required sections, ordering, headings, references.
- `style`: typography, spacing, heading style, citation style, page geometry.
- `export`: future target outputs, export profile references, renderer hints.
- `references`: internal references, cross-references, bibliographic references, and `reference_style_id`.
- `bibliography`: future bibliography sources and listing behavior.
- `metadata`: title, author, language, document kind, version, dates.
- `guidance`: editorial instructions for human or future assisted drafting.

## Typed overrides

Local changes must be recorded as typed overrides:

- `structural_overrides`: section additions, section removals, heading changes, ordering changes.
- `style_overrides`: font, heading style, spacing, citation style.
- `export_overrides`: target format, profile, renderer hints, output naming.
- `reference_overrides`: reference style, reference block placement, citation behavior.
- `metadata_overrides`: title, author, language, document kind, project-specific metadata.
- `guidance_overrides`: section instructions, tone, quality checklist, internal notes.

Examples:

- changing font: `style_override`
- adding a section: `structural_override`
- changing DOCX target behavior: `export_override`
- changing APA to a corporate references style: `reference_override`
- changing title or author: `metadata_override`
- modifying section instructions: `guidance_override`

## Editorial guidance

Templates may declare per-section guidance:

- purpose
- drafting instructions
- recommended tone
- recommended length
- expected content
- forbidden content
- quality checklist
- internal observations
- export notes

Guidance orients future drafting. It does not execute, mutate, approve, block by itself, or replace policy.

Internal observations are not exported unless an export profile explicitly allows them.

## Drift classification

Template deviation must be classified by dimension:

- `style_drift`
- `metadata_drift`
- `export_drift`
- `structure_drift`
- `semantic_drift`

Example severity guidance:

- changing font: `style_drift`, minor
- adding section: `structure_drift`, warning
- removing required section: `structure_drift`, error in `strict`
- changing document type: `semantic_drift`, incompatible

Style changes do not degrade structural conformance by themselves.

## Forbidden responsibilities

Document templates must not:

- implement export
- create `document_template_runtime`
- create `document_export_runtime`
- execute Pandoc, Tectonic, LibreOffice, or external tools
- invoke LLMs
- mutate existing documents
- create an editor
- place logic in UI crates
- reinterpret project manifests
- duplicate `project_runtime`
- use `assets/` as runtime
- hardcode template rules in code
- allow direct mutation of system templates
- treat renaming as a change to `template_id`

## References and bibliography

Templates may select a `reference_style_id` from declarative resources such as:

- `resources/reference_styles/standard/`
- `resources/reference_styles/corporate/`

Reference styles are declarative. They must not be hardcoded in code or UI.

See `docs/specs/document_references.md`.

## Resolution, validation, and UI capture

Template resolution, validation, creation flow, and popup capture are specified separately:

- `docs/specs/document_template_resolution.md`
- `docs/specs/document_template_validation.md`
- `docs/specs/document_creation_flow.md`
- `docs/specs/document_template_ui_contract.md`

The built-in declarative catalog is `resources/document_templates/templates_catalog.json`.

These contracts are proposals only. They do not create runtime, package creation, export, LLM execution, tool execution, or UI authority.

## Invariants

- `INV-DOC-TEMPLATE-001`: every assisted document creation starts from a declarative template or governed default template.
- `INV-DOC-TEMPLATE-002`: templates instantiate Markdown `ARTIFACT` documents.
- `INV-DOC-TEMPLATE-003`: Markdown is the authoritative editable source.
- `INV-DOC-TEMPLATE-004`: LaTeX, PDF, and DOCX are derived and regenerable outputs.
- `INV-DOC-TEMPLATE-005`: every document created from a template preserves `template_id` and `template_snapshot`.
- `INV-DOC-TEMPLATE-006`: the local template snapshot is not destructively modified.
- `INV-DOC-TEMPLATE-007`: local modifications are recorded as typed overrides.
- `INV-DOC-TEMPLATE-008`: `effective_template` is calculated from snapshot plus overrides.
- `INV-DOC-TEMPLATE-009`: the user may choose `strict`, `guided`, or `free_from_template`.
- `INV-DOC-TEMPLATE-010`: style changes do not degrade structural conformance.
- `INV-DOC-TEMPLATE-011`: drift is classified by structure, style, metadata, export, and semantics.
- `INV-DOC-TEMPLATE-012`: templates may include editorial guidance per section.
- `INV-DOC-TEMPLATE-013`: guidance is declarative; it does not execute, mutate, or decide policy.
- `INV-DOC-TEMPLATE-014`: template observations are not exported unless explicitly declared.
- `INV-DOC-TEMPLATE-015`: future export uses `document.md`, `effective_template`, reference data, and export profile, not UI state.

## Future notes

A later phase may introduce a validator, resolver, or export runtime. That phase must keep templates declarative, preserve Markdown as source, and avoid coupling export behavior to UI state.

---

### !REL_FILE!

# document_template_designer_tool

## Status

PROPOSAL / F9-ready declarative governance. This document defines a future `TemplateDesignerTool` concept only. It does not implement `tool_runtime`, template runtime, LLM execution, filesystem mutation, or UI behavior.

## Purpose

Define how a future governed template design surface may relate to the Tools Panel, LLM suggestions, template popup, and `ActionResolution`.

`TemplateDesignerTool` is a governed entrypoint proposal, not an executable tool in F9.

## Normative rule

TemplateDesignerTool = governed template-management proposal surface.

It does not execute, write files, validate as authority, open itself from LLM output, or bypass `ActionResolution`.

## Future entrypoints

The Tools Panel may expose a future explicit user action to open the template popup.

An LLM may suggest creating or editing a template, but it must not open the popup directly.

LLM output may produce:

- `TemplateProposal`
- `TemplateDraft`
- structured rationale
- suggested operation type

LLM output must not:

- write template files
- mutate `resources/`
- mutate `user/`
- run export
- execute tools
- approve actions

## Future operations

The following operations are future `ActionRequest` classes:

- create template
- clone template
- rename template
- edit template
- delete template

Every operation must pass through `ActionResolution`.

Deleting a user template requires confirmation.

Editing, renaming, or deleting a system template must be blocked.

System templates are readonly and only cloneable.

User templates are editable, renameable, cloneable, and deletable only when future policy allows it.

## Relationship with template popup

The template popup captures user intent and draft data.

The popup may present operations supplied by future policy/controller state, but it must not decide permissions.

The popup must not write files, validate as authority, create resources, clone resources, delete resources, or approve actions.

## Relationship with Tools Panel

Tools Panel may present `TemplateDesignerTool` as a future explicit user entrypoint.

In F9 this is documentation only. No `tool_runtime` entry, runner, catalog mutation, or executable integration is introduced.

## Relationship with ActionResolution

Template design operations are future actions.

`ActionResolution` must evaluate:

- controlled origin
- target template
- source layer
- ownership
- operation type
- risk level
- confirmation need
- blocked system-template mutations

No action may be executed merely because it appears in chat, popup state, or Tools Panel state.

## Forbidden responsibilities

`TemplateDesignerTool` must not:

- implement runtime
- create or modify templates in F9
- touch `project_runtime`
- touch `tool_runtime`
- invoke LLMs
- execute tools
- mutate filesystem
- run export
- hardcode UI permissions
- turn the popup into an editor
- bypass user confirmation

## Related specs

- `docs/specs/document_template_ui_contract.md`
- `docs/specs/document_templates.md`
- `docs/specs/document_template_resolution.md`
- `docs/specs/document_template_validation.md`
- `docs/specs/tools_panel.md`
- `docs/specs/action_resolution.md`
- `docs/specs/chat_document_flows.md`


---

### !REL_FILE!

# document_template_resolution

## Status

PROPOSAL / F9-ready declarative governance. This document defines future template resolution rules only. It does not implement runtime, document creation, export, LLM calls, tool execution, or filesystem mutation.

## Purpose

Define how a future governed document creation flow may select a template before creating a Markdown `ARTIFACT` package.

Template resolution is a declarative selection contract, not a new project pipeline.

## Resolution precedence

Future resolution should use this precedence:

1. `project`
2. `user`
3. `resources`

The `resources` layer is the built-in fallback and may be indexed by `resources/document_templates/templates_catalog.json`.

If the same `template_id` exists in multiple layers, precedence determines the selected template. The resolved origin must remain traceable.

## Conceptual inputs

- `template_id`
- optional `template_version`
- optional document kind
- optional conformity mode
- project-local template declarations, future
- user-local template declarations, future
- resource template catalog

No absolute host paths may be required.

## Conceptual result

A future `TemplateResolutionResult` may contain:

- resolved `template_id`
- resolved `template_version`
- source layer: `project`, `user`, or `resources`
- ownership: `project`, `user`, or `system`
- relative template resource path
- default conformity mode
- supported conformity modes
- selected operation capability flags, if supplied by policy
- shadowed lower-precedence template references, when applicable
- warning list
- resolution fingerprint or catalog version

This result is not execution.

## Conflict handling

Template id conflicts are resolved by layer precedence:

- `project` shadows `user`
- `user` shadows `resources`
- `resources` is the fallback

Resolution must not erase the existence of shadowed templates. It should preserve trace data so a future UI or audit report can explain why a template was selected.

Conflicts inside the same source layer are invalid and should produce `duplicate_template_id`.

## Snapshot rule

When a future document package is created, the selected template must be copied into the package as `template_snapshot.json`.

The snapshot is preserved locally and is not destructively modified.

## Failure classes

- `missing_template`
- `ambiguous_template_id`
- `disabled_template`
- `unsupported_conformity_mode`
- `incompatible_template_kind`
- `stale_catalog`
- `duplicate_template_id`
- `forbidden_template_operation`

## Forbidden responsibilities

Template resolution must not:

- create a document package
- write files
- mutate templates
- invoke LLMs
- execute tools
- run export
- reinterpret project manifests
- duplicate `project_runtime`
- depend on UI state as document truth

## Related specs

- `docs/specs/document_templates.md`
- `docs/specs/document_template_validation.md`
- `docs/specs/document_creation_flow.md`
- `docs/specs/document_package.md`

---

### !REL_FILE!

# document_template_ui_contract

## Status

PROPOSAL / F9-ready declarative governance. This document defines a future UI capture contract only. It does not implement UI behavior, runtime mutation, validation authority, LLM calls, tool execution, or document creation.

## Purpose

Define the boundary for a future popup used to create or edit a template draft.

The popup captures intent. It does not write, validate, decide, execute, or create packages.

The popup may be opened only from a confirmed governed action, such as a Tools Panel action accepted by the user or an LLM suggestion that has been explicitly confirmed. The LLM may suggest opening or using the popup, but it must not open it directly or execute template operations.

See `docs/specs/document_template_designer_tool.md` for the future `TemplateDesignerTool` proposal.

## Normative rule

Template popup = `TemplateDraft` capture surface.

It is not a document editor, validator, resolver, runtime, or project pipeline.

## TemplateDraft conceptual fields

- `draft_id`
- `template_id`
- `conformity_mode`
- `document_title`
- `document_kind`
- `language`
- `metadata_overrides`
- `selected_export_profile_ids`
- `reference_style_id`
- `guidance_overrides`
- optional target package hint

The target package hint is not permission to write files.

## UI responsibilities

UI may:

- list declarative templates provided by a controller or future service
- group templates by `system`, `user`, and future `project` ownership
- display template title, kind, summary, and guidance
- display permitted operations supplied by future policy or controller state
- capture user choices
- capture user intent to create, view, rename, edit, delete, or clone a template
- show readonly warnings or future validation findings
- emit a structured draft request

## Template operation capture

System templates may expose these operations:

- `view`
- `clone`

User templates may expose these operations, when permitted by policy:

- `view`
- `edit`
- `rename`
- `delete`
- `clone`

Project templates are future scoped resources and must follow the same policy-driven operation model.

UI presents allowed operations. UI does not decide permissions.

Template creation, rename, edit, delete, and clone are future governed requests. They are not filesystem operations in F9.

When the request originates from a future `TemplateDesignerTool`, the popup still remains a capture surface only. It does not become the tool runner or execution authority.

## UI forbidden responsibilities

UI must not:

- write files
- delete files
- create document packages
- create template resources
- rename template resources
- clone template resources
- validate templates as authority
- resolve template precedence
- decide whether a template is editable, renameable, deletable, or cloneable
- mutate `document.md`
- generate `template_snapshot.json`
- generate `effective_template.json`
- run export
- invoke LLMs
- execute tools
- act as `TemplateDesignerTool`
- approve actions
- hardcode template rules
- use UI state as document truth

## Text and resources

Visible strings should come from i18n or declarative resources.

Template rules should come from declarative template resources, not from Slint code.

## Relationship to workspace tabs

The popup is a capture surface. It is not a workspace tab, miniapp, editor, or runtime surface.

Future previews or package editors, if introduced, must remain governed by their own specs.

## Related specs

- `docs/specs/document_creation_flow.md`
- `docs/specs/document_template_resolution.md`
- `docs/specs/document_template_validation.md`
- `docs/specs/document_templates.md`

---

### !REL_FILE!

# document_template_validation

## Status

PROPOSAL / F9-ready declarative governance. This document defines future validation semantics only. It does not implement a validator, runtime enforcement, export, LLM calls, tool execution, or filesystem mutation.

## Purpose

Define how future document templates, template drafts, overrides, and document packages may be checked before governed document creation or update.

Validation reports findings. It does not decide UI behavior, execute actions, or mutate documents.

## Validation dimensions

- `structure`: required sections, order, heading hierarchy, document kind.
- `style`: typography, heading style, spacing, citation style.
- `export`: declared targets, profile references, renderer hints.
- `metadata`: title, author, language, version, document kind.
- `guidance`: section instructions, tone, quality checklist, internal notes.
- `references`: internal references, cross-references, bibliography entries, `reference_style_id`.

## Conformity modes

- `strict`: incompatible structure or missing required sections are errors.
- `guided`: deviations produce warnings unless forbidden by policy.
- `free_from_template`: template conformance is advisory after creation.

## Drift classification

- `style_drift`
- `metadata_drift`
- `export_drift`
- `structure_drift`
- `semantic_drift`

Severity levels:

- `info`
- `warning`
- `error`
- `incompatible`

## Minimum future checks

- template id is present and stable
- template id is unique within its source layer
- supported conformity mode is selected
- template ownership and source layer are declared consistently
- system templates are readonly
- user template rename changes `display_name`, not `template_id`
- cloned templates preserve `cloned_from` when derived from a system template
- required structural sections are declared consistently
- override types are known
- style overrides do not degrade structural conformance
- selected `reference_style_id` is declarative and resolvable
- export profiles are declarative and do not imply execution
- `document.md` remains the authoritative editable source
- derived outputs remain regenerable
- UI state is not treated as package truth

## Template operation failure classes

Future validation should report:

- `forbidden_template_operation`: editing, deleting, or renaming a system template, or performing any operation disallowed by policy.
- `invalid_template_name`: empty, unsafe, or policy-incompatible `display_name`.
- `duplicate_template_id`: duplicate stable id within a source layer or unresolved duplicate after precedence.
- `missing_cloned_from`: clone metadata is missing for a template derived from a system template.
- `delete_user_template_requires_confirmation`: user template deletion has associated documents or package references and requires explicit confirmation.

These failures are declarative classes only. They do not implement deletion, renaming, cloning, or filesystem mutation.

## UI boundary

UI may display validation findings in a future phase.

UI must not:

- perform template validation as authority
- hardcode validation rules
- decide policy
- mutate documents
- create packages
- run export

## Forbidden responsibilities

Template validation must not:

- execute export tools
- invoke LLMs
- call tools
- write files
- create a runtime
- duplicate `project_runtime`
- reinterpret project manifests

## Related specs

- `docs/specs/document_templates.md`
- `docs/specs/document_template_resolution.md`
- `docs/specs/document_creation_flow.md`
- `docs/specs/document_template_ui_contract.md`

---

### !REL_FILE!

# document_text_runtime

## Purpose

Define the minimum contract for regenerable text derivation over primary project documents.

## Primary vs derived

- the source file remains the primary document
- `text/` contains derived, regenerable artifacts
- future `semantics/` layers build on derived text, not on raw source ownership changes

## Minimum output

Each derivation produces:

- `text/extracted.txt` when ready
- `text/text_manifest.json`
- optional `text/pages/page_XXX.txt`
- optional `text/chunks/chunk_XXXX.txt`
- optional `text/chunks/chunks_manifest.json`

## Status model

- `ready`
- `unsupported`
- `error`

## Traceability

Every derivation and chunk must preserve at least:

- `document_id`
- `source_filename`
- `source_format`
- `source_hash`

Chunk metadata also keeps:

- `chunk_id`
- `char_start`
- `char_end`
- optional `page_start`
- optional `page_end`

Chunk evidence integrity matters for downstream semantic invalidation:

- `chunk_hash` changes may stale downstream semantic evidence
- missing chunk artifacts may orphan downstream semantic evidence
- document-text runtime does not invalidate semantic or graph material directly; it provides governed derivative evidence only

## Out of scope

- OCR by default
- semantic parsing
- embeddings
- graph/quads
- retrieval orchestration
- notebook or editing workflows

## F12.4 file intake derivation alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

F12.4 may plan future derivation only. It does not extract text, pages, chunks, thumbnails, previews, metadata indexes, or checksum indexes.

`document_text_runtime` may later consume exposed or eligible project-owned objects, but F12.4 does not implement that handoff.

## F12.5 file intake derivation plan

`F12.5` is plan-only and adds no derivation runtime.

Future `F12.6` must not generate text, chunks, pages, thumbnails, previews, metadata indexes, checksum indexes, or any derivative.

`document_text_runtime` must not be modified for `F12.6` unless a later explicit derivation phase opens that scope.

Derivatives remain deterministic, regenerable, and not source truth.

## F13.1 legacy derivation bypass hardening

`F13.1` is SPEC-only and adds no derivation runtime.

`derive_document_text` and any equivalent derivation helper must not be used as:

- an intake step
- an exposure precondition
- a project visibility trigger
- a UI shortcut for importing or promoting external files

Direct UI-triggered derivation from external files or newly copied files is legacy-only and must be blocked before any `F13` runtime implementation begins.

---

### !REL_FILE!

# document_tree

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Document Tree` / `Arbol de Documentos` surface for DocGraph.

This spec fixes what the tree is allowed to expose, how nodes are described, and how the tree stays separated from intake, chat, tabs, and viewers.

## Normative definition

`Document Tree` is a governed view of objects exposed by `project_manifest`.

It does not scan the filesystem.
It does not decide visibility by itself.
It does not import documents.
It does not execute actions.

`project_runtime` prepares the surface.
The tree renders and routes governed references only.

## Core rule

The tree is not a free filesystem explorer.

It is a navigation and selection surface for already governed project objects.

The tree must remain distinct from:

- `Clip Panel`, which is reserved for external intake
- `Chat Panel`, which coordinates references and instructions
- `Workspace Tabs`, which host controlled views
- `Readonly Viewer`, which renders governed content

## Relationship to authority

- `project_manifest` governs exposure
- `project_runtime` prepares surface state
- `registry.json` may accelerate navigation but is not authority
- `graph/` may explain relations but does not decide visibility

## Node kinds

The tree may expose these conceptual node kinds:

- `project_root`
- `documents_group`
- `knowledge_group`
- `document_holder`
- `source_document`
- `derived_document`
- `artifact_document`
- `sandbox_context`
- `tool_output_reference`
- `folder_group`

Interpretation:

- `project_root`: top logical node for the governed project surface
- `documents_group`: grouping node for project document domains
- `knowledge_group`: grouping node for governed knowledge items
- `document_holder`: owned documentary grouping context
- `source_document`: governed source document reference
- `derived_document`: governed derived document reference
- `artifact_document`: governed artifact-backed document reference
- `sandbox_context`: governed sandbox context or sandbox visibility node
- `tool_output_reference`: governed tool-output reference when exposed through the tree
- `folder_group`: logical grouping node only, not an arbitrary host folder

## TreeNode conceptual contract

Minimum conceptual fields:

- `node_id`
- `node_kind`
- `display_name`
- `ref`
- `family`
- `document_class`, when applicable
- `is_selectable`
- `is_viewable`
- `is_readonly`
- `status`
- `opens_tab_kind`

Field meaning:

- `node_id`: stable tree-node identity for prepared UI state
- `node_kind`: one of the governed node kinds declared in this spec
- `display_name`: prepared visible name
- `ref`: governed reference for the represented object, if any
- `family`: active-object family used by `ActiveObjectContext`
- `document_class`: source, derived, artifact, or other declared class when the node represents a document
- `is_selectable`: whether the node may participate in governed selection
- `is_viewable`: whether the node may open or activate a view
- `is_readonly`: whether the represented object is readonly in the current phase or policy
- `status`: prepared visual state
- `opens_tab_kind`: governed workspace tab kind that may be activated or opened

## Visual states

Prepared node state may include:

- `ready`
- `readonly`
- `missing`
- `stale`
- `locked`
- `future_actionable`

Interpretation:

- `ready`: node is valid and available for governed viewing or selection
- `readonly`: node is valid but not mutable in current phase or policy
- `missing`: exposed reference no longer resolves
- `stale`: reference exists but prepared state is no longer current enough
- `locked`: node is present but not currently available for the intended action
- `future_actionable`: node may become actionable only in a future governed phase

## Allowed actions

The tree may allow:

- expand
- collapse
- select
- activate or open the corresponding governed tab
- copy a structured reference
- send a structured reference to chat as context
- show metadata

These are governed navigation and reference actions only.

## Forbidden actions

The tree must not allow:

- edit
- delete
- move
- rename
- import
- scan folders
- execute tools
- mutate `SOURCE`
- mutate `DERIVED`

The tree must not become a filesystem runtime or mutation surface.

## Relationship to ActiveObjectContext

Tree selection prepares `ActiveObjectContext`.

The tree may select governed artifacts by family, not as one single global active object.

Left-click on a selectable artifact toggles selection inside its family.

Rules:

- clicking a non-selected artifact selects it in its family and replaces the previous selection of that family
- clicking an already selected artifact deselects it
- clicking an artifact from another family keeps other-family selections intact
- every successful selection change updates `focused_family` to the clicked family
- deselecting the currently focused family sets `focused_family` to `None`

Middle-click may request opening or activating the corresponding governed workspace tab without duplicating an existing tab for the same governed reference.

## Relationship to workspace tabs

The tree may request opening or activating a governed workspace view appropriate for the node kind.

Examples:

- `source_document`, `derived_document`, and `artifact_document` may open `viewer`
- `sandbox_context` may open `sandbox_view`
- `tool_output_reference` may open `tool_result` or another governed output view when declared

Opening or activating a tab does not clear selections in other families.

Opening the same governed object must reuse the existing workspace tab instead of creating duplicates.

## Relationship to Readonly Viewer and Sandbox View

`Readonly Viewer` shows governed document content.

`Sandbox View` shows sandbox context.

The tree does not render documents itself and does not replace either view.

## Relationship to Clip Panel

`Clip Panel` remains reserved for external intake.

The tree must not import, attach, or stage external files. It only exposes already governed objects.

## Relationship to Chat Panel

The tree may provide structured references to chat.

The tree does not authorize chat actions, interpret user intent, or execute operations.

## Failure modes

- `ref_not_exposed_by_manifest`
- `node_ref_missing`
- `node_stale`
- `invalid_node_kind`
- `filesystem_scan_detected`

Interpretation:

- `ref_not_exposed_by_manifest`: the prepared node points to a reference that is not governed by manifest exposure
- `node_ref_missing`: the node reference no longer resolves
- `node_stale`: the node exists but its prepared state is no longer current enough
- `invalid_node_kind`: prepared state contains an unsupported or invalid kind
- `filesystem_scan_detected`: a non-governed host scan was attempted instead of manifest-driven exposure

## Invariants

- `INV-DOCTREE-001`: `Document Tree` MUST remain a governed view of objects exposed by `project_manifest`
- `INV-DOCTREE-002`: `Document Tree` MUST NOT scan the filesystem
- `INV-DOCTREE-003`: `Document Tree` MUST NOT decide visibility by itself
- `INV-DOCTREE-004`: `Document Tree` MUST NOT import documents or external files
- `INV-DOCTREE-005`: `Document Tree` MUST NOT execute tools or mutations
- `INV-DOCTREE-006`: tree selection MUST prepare governed selection context, not execution
- `INV-DOCTREE-007`: tree selection MUST preserve other-family selections unless future governed policy states otherwise
- `INV-DOCTREE-008`: the tree MUST remain separate from `Clip Panel`, `Chat Panel`, `Workspace Tabs`, and `Readonly Viewer`

## Related specs

- `docs/specs/active_object_context.md`
- `docs/specs/active_context.md`
- `docs/specs/workspace_tabs.md`
- `docs/specs/local_sandbox_context.md`
- `docs/specs/gui_objects_v1.md`

## F12.4 file intake tree alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

The document tree must not infer project documents from passive filesystem presence.

Future intake items may appear only as exposed or governed prepared presentation state according to `project_manifest.json` and file-intake rules.

The document tree must not import, copy, classify, expose, derive, register, or execute tools.

## F12.5 file intake tree plan

`F12.5` is plan-only and adds no tree behavior.

Future `F12.6` must not make newly imported or candidate intake files visible in the tree unless a separate exposure slice explicitly opens `project_manifest.json` governance for them.

The tree may later present prepared intake state only when derived from governed metadata, not from filesystem scanning.

## F13.0 project exposure gate tree alignment

`F13.0` is SPEC-only and adds no tree behavior.

After a future governed implementation writes a project exposure entry to `project_manifest.json`, `project_runtime` may prepare document tree nodes for the exposed object.

Tree visibility rules:

- visibility is manifest-driven only
- `imported_not_exposed` items must not appear as project-exposed document nodes
- blocked and unsupported intake items must not appear as project-exposed document nodes
- duplicate exposed content must preserve distinct governed refs when manifest entries are distinct
- reused physical blobs must not collapse tree identity
- tree labels must be sanitized presentation labels
- tree presentation remains readonly unless a later editing phase opens mutation

The document tree must not scan `file_store`, scan intake metadata, generate registry entries, write graph entries, create derivatives, call `document_text_runtime`, call `tool_runtime`, or infer project visibility from filesystem presence.

## F13.1 legacy tree-bypass hardening

`F13.1` is SPEC-only and adds no tree behavior.

The document tree must not use any legacy helper as project visibility authority:

- `list_project_documents` must not remain a production visibility source once `F13` runtime work begins
- copied files in project folders must not appear as exposed merely because they are present on disk
- chat resources must not appear as project tree nodes unless later imported and exposed through the governed path
- derivatives must not create or justify tree visibility

Any temporary legacy listing must be treated as fixture-only or migration-only and must not be interpreted as canonical project exposure.

## F13.4 manifest exposure tree alignment

`F13.4` is SPEC-only and adds no tree behavior.

The tree may only show project artifacts backed by manifest authority and, in a later explicit runtime slice, by a governed `ManifestExposureEntry`.

Tree hardening rules:

- accepted confirmation alone must not make an item visible in the tree
- intake history/index rows must not be consumed as tree authority
- `System View` exposure summaries must not be consumed as tree authority
- duplicate blob reuse must not collapse distinct tree identity when future manifest entries remain distinct
- tree visibility must stay manifest-driven even when storage, registry, graph, or observability metadata already exists

## F13.5A manifest exposure runtime tree checklist

`F13.5A` is SPEC-only and adds no tree behavior.

Future `F13.5B` runtime checklist for tree alignment:

- manifest exposure may prepare future tree visibility only after a valid `ManifestExposureEntry` exists
- tree visibility must remain downstream of manifest authority, never a co-authority
- failed or blocked exposure attempts must not create tree nodes
- duplicate blob reuse must not collapse distinct tree identity when distinct manifest entries exist

---

### !REL_FILE!

# document_viewer

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Document Viewer` surface for DocGraph.

DocGraph does not edit documents.
DocGraph transforms documents.

This spec fixes the viewer as a readonly rendering surface that may expose text selection and future visual proposal overlays without becoming an editor or patch runtime.

## Role

`Document Viewer`:

- shows governed documents
- is readonly
- does not save
- does not edit
- does not apply changes

The viewer renders prepared governed state only.

## Core rule

The viewer is a readonly surface.

It may present content, selection, and visual proposal context.

It must not become:

- an editor
- a file writer
- a patch runtime
- a tool executor
- an LLM invocation surface

## Allowed behavior

The viewer may:

- render a governed document
- allow text selection
- copy visible text
- copy a structured reference
- show visual overlays received as prepared state
- open a contextual popup from a text selection through UI intent only

These actions do not imply mutation or execution.

When text selection exists, it should be represented as governed `TextSelectionRef` state rather than as a new document copy.

## Forbidden behavior

The viewer must not:

- edit directly
- write to a file
- save changes
- mutate `SOURCE`
- mutate `DERIVED`
- apply a proposal without acceptance
- execute tools
- invoke LLM
- become a disguised editor

## Relationship to transformation specs

This document does not redefine selection, popup, preview, diff, or transformation-core contracts.

It must align with:

- `docs/specs/text_selection.md`
- `docs/specs/transform_popup.md`
- `docs/specs/patch_preview.md`
- `docs/specs/diff_view.md`
- `docs/specs/transformation_core.md`

In this viewer spec:

- text selection is treated as prepared viewer interaction state
- popup opening is treated as UI intent only
- overlays are treated as prepared visual state only
- patch preview and diff remain external contracts

The selection structure itself is defined in `docs/specs/text_selection.md`.

## States

Prepared viewer state may include:

- `readonly`
- `selection_active`
- `overlay_active`
- `missing`
- `stale`
- `error`

Interpretation:

- `readonly`: valid rendered document context with no mutation authority
- `selection_active`: readonly viewer state with active text selection
- `overlay_active`: readonly viewer state with prepared visual overlay present
- `missing`: source reference no longer resolves
- `stale`: rendered context exists but is no longer current enough for intended derived interaction
- `error`: rendering or preparation failed in a governed way

## Relationship to Workspace Tabs

`Document Viewer` is rendered inside `Document Viewer Tab`.

The tab identifies the governed target reference.
The viewer renders the prepared document context for that tab.

The viewer does not own tab identity, permission, or runtime authority.

## Relationship to ActiveObjectContext

The viewer may render the currently selected document context prepared through `ActiveObjectContext`.

Viewer visibility does not create or invent active context by itself.

Selection inside the viewer must not silently authorize any transformation.

Selection captured in the viewer must retain `document_ref` and document-integrity linkage through governed selection reference state.

## Relationship to Document Tree

`Document Tree` prepares or requests opening the governed document target.

The viewer shows the resolved governed document.

The viewer does not import, discover, or expose arbitrary host files.

## Relationship to overlays

Visual overlays may be rendered only when prepared state provides them.

An overlay:

- is visual only
- is not persistence
- is not acceptance
- is not application

The viewer may show overlay presence, but it must not apply the underlying proposal.

Pending proposal highlight semantics are defined in `docs/specs/patch_preview.md`.

## Failure modes

- `source_not_resolvable`
- `unsupported_render_format`
- `selection_invalid`
- `overlay_stale`
- `forbidden_edit_attempt`

Interpretation:

- `source_not_resolvable`: governed document target no longer resolves
- `unsupported_render_format`: target exists but cannot be rendered by the current governed viewer contract
- `selection_invalid`: prepared text selection is not valid for the current rendered context
- `overlay_stale`: overlay state exists but no longer matches current viewer context safely
- `forbidden_edit_attempt`: direct edit behavior was attempted on the readonly viewer

## Invariants

- `INV-DVIEW-001`: `Document Viewer` MUST remain readonly
- `INV-DVIEW-002`: `Document Viewer` MUST NOT save or mutate files
- `INV-DVIEW-003`: `Document Viewer` MUST NOT apply proposals by itself
- `INV-DVIEW-004`: text selection MAY exist without turning the viewer into an editor
- `INV-DVIEW-005`: overlays MAY be displayed as prepared visual state only
- `INV-DVIEW-006`: the viewer MUST remain separate from patch runtime and transformation execution
- `INV-DVIEW-007`: the viewer MUST NOT execute tools or invoke LLM

## Related specs

- `docs/specs/workspace_tabs.md`
- `docs/specs/document_tree.md`
- `docs/specs/active_object_context.md`
- `docs/specs/text_selection.md`
- `docs/specs/transform_popup.md`
- `docs/specs/patch_preview.md`
- `docs/specs/diff_view.md`
- `docs/specs/transformation_core.md`

---

### !REL_FILE!

# external_dependency_manager

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed specification for `External Dependencies` management in DocGraph.

The manager governs declared external binaries recommended for the portable sandbox without implementing download, install, execution, or validation runtime in F9.

## Classification

`External Dependency Manager` is runtime infrastructure.

It is not:

- a normal user tool
- an `LLM Tool`
- an LLM-invocable capability
- a functional user action

It may expose visible catalog state to the user, but it remains infrastructure rather than user workflow execution.

## Core rule

External dependencies are declared, inspectable, and verifiable by contract.

They are not downloaded or installed by this specification.

UI may show status and governed buttons later, but UI must not own installation logic.

## Declarative catalog

Canonical catalog:

- `resources/tools/external/tools_external_catalog.json`

Each dependency entry must declare at least:

- `tool_id`
- `display_name`
- `purpose`
- `description_short`
- `description_long`
- `use_cases`
- `recommendation`
- `version`
- `source_url`
- `homepage_url`
- `license`
- `archive_type`
- `archive_sha256`
- `expected_binary_names`
- `install_dir`
- `validation_command`

These fields are declarative contract data.

They do not imply that the dependency is present, trusted, extracted, or executable.

The catalog may also include declarative user-facing fields such as:

- required internal tool or capability references
- expected local path
- download notes
- validation state
- trusted-by-user metadata for custom references

## Lifecycle

Prepared lifecycle state may include:

- `declared`
- `missing`
- `downloaded`
- `verified`
- `extracted`
- `binary_detected`
- `installed`
- `validated`
- `invalid`
- `removed`

Interpretation:

- `declared`: dependency exists in catalog only
- `missing`: dependency is not present in expected runtime layout
- `downloaded`: archive exists in staging in a future runtime phase
- `verified`: checksum has passed in a future runtime phase
- `extracted`: archive contents were extracted in a future runtime phase
- `binary_detected`: expected binary name was found
- `installed`: dependency is present in final runtime location
- `validated`: validation command has passed
- `invalid`: dependency failed integrity or layout checks
- `removed`: prior installed state was intentionally removed

## Layout

Current declarative placeholder layout:

- `runtime/temp/external_install/<tool_id>`
- `runtime/external/<tool_id>`

User-managed dependency root for the governed Tools menu model:

- `user/runtime/external_dependencies/`

Managed dependency location example:

- `user/runtime/external_dependencies/<dependency_id>/`

Referenced dependency locations may live outside the managed root, but DocGraph may only remove the reference, not physically delete those external files.

No downloaded or user-managed external binaries may live under:

- `resources/`
- `assets/`
- `crates/`
- project folders

The dependency must not execute from staging.

The dependency must not become executable merely because it exists under the managed user runtime root.

## Tools menu relationship

`Tools -> External Dependencies` is the governed inspection surface for external dependencies.

This surface is separate from:

- `Operational Tools`
- `LLM Tools`
- `Preferences`

There is no combined tools panel with tabs.

## Install / Configure flow

If a dependency is missing, the user-facing surface may expose:

- `Install / Configure`

This opens a governed popup that may show:

- dependency name
- description
- why DocGraph may need it
- recommended source URL
- recommended version or version range
- expected binary name
- expected destination under `user/runtime/external_dependencies/`
- manual steps for the user
- button to select an existing downloaded binary or folder
- button to validate reference

The system must not:

- download automatically
- execute installers
- modify `PATH`
- run binaries
- mutate system folders
- store secrets
- treat validation as execution authorization

Validation may only record declared metadata and future intended availability.

## Custom external dependency

The governed surface may allow:

- `Add custom external dependency`

This opens a popup where the user may declare:

- dependency id
- display name
- source URL
- download notes
- expected binary name
- expected version, optional
- local path under `user/runtime/external_dependencies/` or selected reference
- `trusted_by_user`
- reason or use note

Rules:

- custom dependency remains declared by the user
- custom dependency is not automatically trusted by DocGraph
- custom dependency does not create a new internal tool
- custom dependency does not enable execution
- custom dependency may only become referenced by future internal tools through governed catalog or policy updates

## Remove / Delete model

Two governed cases exist.

### Managed dependency

A managed dependency under `user/runtime/external_dependencies/` may be deleted from the DocGraph-managed user runtime folder.

This requires explicit confirmation.

Deletion must update dependency status.

### Referenced dependency

A referenced external binary outside the DocGraph-managed runtime folder must not be physically deleted.

DocGraph may only remove the reference.

## Rules

- checksum is mandatory
- `source_url` may come only from catalog declaration
- URLs must not be hardcoded in code
- `assets/` must not be used as runtime
- execution from staging is forbidden
- UI may show buttons or state later, but not perform installation logic
- LLM must not install or execute external dependencies
- user-managed dependencies must use `user/runtime/external_dependencies/` when managed by DocGraph
- external dependency presence does not grant execution permission
- destructive deletion must require confirmation

Normative form:

- `INV-EXTDEP-001`: checksum metadata MUST be declared
- `INV-EXTDEP-002`: `source_url` MUST come only from catalog data
- `INV-EXTDEP-003`: runtime code MUST NOT hardcode dependency URLs
- `INV-EXTDEP-004`: external dependencies MUST NOT execute from staging layout
- `INV-EXTDEP-005`: `assets/` MUST NOT be used as runtime install location
- `INV-EXTDEP-006`: UI MUST remain representation-only for dependency state in F9
- `INV-EXTDEP-007`: LLM MUST NOT install or execute external dependencies
- `INV-EXTDEP-008`: managed user dependencies MUST use `user/runtime/external_dependencies/`
- `INV-EXTDEP-009`: dependencies outside the managed user runtime root MUST NOT be physically deleted
- `INV-EXTDEP-010`: validation MUST NOT imply execution permission

## Failure modes

- `checksum_mismatch`
- `download_failed`
- `unsupported_archive_type`
- `extraction_failed`
- `binary_not_found`
- `validation_failed`
- `manifest_missing`
- `invalid_install_dir`

Interpretation:

- `checksum_mismatch`: archive integrity did not match declared checksum
- `download_failed`: archive retrieval failed in a future runtime phase
- `unsupported_archive_type`: declared archive type is not supported
- `extraction_failed`: archive extraction failed
- `binary_not_found`: none of the expected binaries was found after extraction or install
- `validation_failed`: validation command did not pass
- `manifest_missing`: expected install manifest is absent
- `invalid_install_dir`: installed dependency layout is outside the governed runtime location

Additional governed failures may include:

- `reference_outside_managed_root`
- `managed_delete_requires_confirmation`
- `external_reference_delete_blocked`

## Future audits

Future audit surfaces may include:

- `audit_external_tools_catalog`
- `audit_runtime_external_layout`
- `audit_external_tool_manifests`

These remain future governed inspection or validation capabilities only.

## Relationship to catalogs

External dependencies are declared in tool catalogs as `external_dependency`.

This does not make them user tools or LLM tools.

The external catalog remains separate from:

- internal operational tools
- internal LLM tools
- runtime-consumed `tool_runtime` declarations

## Relationship to runtime resources

Portable runtime locations should remain declared under:

- `resources/runtime/runtime_locations.json`

That resource may declare governed roots such as:

- final external install root
- temporary staging root
- user-managed external dependency root

## Forbidden responsibilities

This spec must not:

- download binaries
- execute binaries
- validate with live runtime behavior
- move logic into UI
- open F10
- create a parallel tool runtime
- place dependencies inside `Preferences`

## Related specs

- `docs/specs/tools_catalogs.md`
- `docs/specs/tool_implementation_governance.md`

---

### !REL_FILE!

# File Intake

Status: F12.4 SPEC-ONLY / DECLARATIVE ONLY.

This spec owns governed file intake semantics for DocGraph.

`F12.4` defines the contract by which explicit user-selected external files may later become portable, traceable, project-owned inputs.

`F12.4` does not implement runtime, copy files, scan host folders, mutate `project_manifest.json`, create registry entries, create graph entries, derive text or chunks, execute tools, call providers, use network, invoke external binaries, open LLM tool calling, or open semantic/RDF/Oxigraph runtime.

## Governed intake chain

The canonical F12.4 chain is:

`user selection intent -> FileIntakeRequest -> FileIntakeDraft -> FileIntakeCandidate -> FileIntakePlan -> future governed runtime -> future StoredObject / project exposure / derivatives`

Filesystem presence is not project exposure.

`project_manifest.json` remains the exposure authority.

## Contract artifacts

`FileIntakeRequest`

- captures explicit user selection intent
- may reference one or multiple selected files
- carries `owner_ref` when known
- carries `trace_ref` when known
- does not classify, copy, hash, expose, derive, or register files

`FileIntakeDraft`

- normalizes the request into a reviewable draft
- records sanitized selection summaries
- may carry optional per-file `user_comment` values captured as text-only metadata proposals
- records missing preconditions
- remains non-mutating

`FileIntakeCandidate`

- represents a single candidate item after pre-runtime validation and classification
- carries `intake_item_id`
- carries sanitized metadata
- may be blocked
- does not imply project exposure

`FileIntakePlan`

- declares future storage, exposure, and derivation intent
- requires `owner_ref` and `trace_ref` before durable import
- remains declarative in F12.4

`FileIntakeResult`

- placeholder for future runtime result only
- must not exist as a durable result in F12.4

`IntakeBatch`

- groups one explicit selection flow
- carries an `intake_batch_ref`
- may contain one or more `IntakeItem` values

`IntakeItem`

- represents one selected source item in a batch
- carries `intake_item_id`
- links to source, classification, metadata, storage plan, exposure plan, derivation plan, and trace summary by refs or sanitized summaries
- may carry optional user comment metadata tied to that intake item
- comment fields are descriptive only and must not affect classification, capability, exposure authority, or derivation eligibility

`IntakeSource`

- describes external/host source input
- source paths are transient runtime inputs only
- source paths must not become portable identity

`IntakeClassification`

- records format classification as a hint
- does not imply exposure, derivation, or semantic eligibility

`IntakeMetadata`

- records sanitized portable metadata only
- must not include secrets, raw payloads, credentials, or unsanitized private absolute host paths

`IntakeStoragePlan`

- declares future storage intent aligned with `docs/specs/storage_policy.md`
- does not invent final layout

`IntakeExposurePlan`

- declares whether and how a stored object may later become visible to a project surface
- exposure requires `project_manifest.json` governance

`IntakeDerivationPlan`

- declares possible future deterministic derivatives
- does not generate derivatives in F12.4

`IntakeTraceSummary`

- records sanitized trace linkage
- carries `trace_ref`
- does not authorize import or exposure

## Selection intent

UI may capture explicit user selection.

Selection may include one or multiple files.

UI must not:

- classify files
- copy files
- hash files
- expose files
- derive text, chunks, thumbnails, or previews
- register files
- mutate storage
- authorize intake

UI may provide an optional comment field per selected file during intake.

UI comment capture rules:

- capture text only
- do not interpret comment meaning
- do not classify based on comment
- do not infer metadata from comment
- do not persist comment directly
- do not store anything directly
- do not run sanitization policy locally beyond basic input safety

UI emits intent only.

When a comment is present, UI emits a `FileIntakeDraft` value that includes the per-file `user_comment` as governed intake metadata awaiting sanitization and promotion.

## Source boundary

Source files are external/host inputs.

Source folders are readonly.

Original source files must not be written back to.

Absolute host paths may be used transiently by a future governed runtime, but they must not be persisted as canonical portable identity.

Source path is not identity.

Filename is not identity.

## Pre-runtime validation

F12.4 declares future validation requirements only.

Required validation checks:

- file exists
- file is readable
- file is not a directory unless future folder intake is explicitly opened
- file size policy
- extension and MIME classification as hints only
- unsupported format becomes blocked, not guessed
- duplicate candidate handling
- unsafe filename handling
- private absolute host path sanitization
- secret metadata minimization

Validation must not copy, import, expose, derive, register, or mutate.

## Classification

Classification fields:

- `original_filename_sanitized`
- `extension`
- `detected_kind`
- `declared_kind`
- `media_type_hint`
- `confidence`
- `classification_source`
- `supported_state`

Initial categories:

- `text`
- `markdown`
- `pdf`
- `docx`
- `image`
- `spreadsheet`
- `presentation`
- `unknown`
- `unsupported`

Classification does not imply project exposure.

Classification does not imply derivation.

Classification does not imply semantic eligibility.

## Identity

Identity rules:

- `file_ref` is content-based or a governed deterministic reference
- `object_ref` identifies a project-owned stored object candidate
- `intake_item_id` identifies an item in an intake batch
- source path must not be identity
- filename must not be identity
- duplicate content should reuse physical blob identity when policy allows
- usage refs declare why the file is used

Identity interpretation:

- two files are the same physical artifact when their canonical content hash matches after byte-level hashing
- two selected files are distinct intake events when they have different `intake_item_id` values, even if they point to the same physical artifact
- two stored object candidates may reference the same `file_ref` when the user selected duplicate content in different intake contexts
- `stored_object_id` or `stored_object_candidate_ref` identifies the logical stored object candidate, not the physical bytes
- `file_ref` identifies the content; it must not encode filename, source path, owner, batch, or project exposure
- `intake_item_id` identifies the selection event inside an `IntakeBatch`; it must not be reused as content identity
- `content_hash` is the hash evidence used to derive or verify `file_ref`

Duplicate handling:

- duplicate content in the same batch should create one `IntakeItem` per explicit user selection
- duplicate content should reuse the existing physical blob when hash and storage policy match
- duplicate content must not silently overwrite metadata, comments, owner refs, trace refs, or blocking reasons from another intake item
- duplicate content may produce a new logical `stored_object_candidate_ref` when the intake context, owner, comment, source label, or usage rationale differs
- exact duplicate selections may be marked as `duplicate_conflict` only when policy cannot safely disambiguate the user intent
- duplicate handling must preserve sanitized source labels so the user can understand which explicit selection produced each item
- reusing a blob does not imply project exposure, derivative readiness, registry generation, graph writes, or document-tree visibility

Reuse policy:

- prefer physical blob reuse when `content_hash` matches and the existing blob is governed, immutable, readable, and within the same storage authority
- create a new logical stored object candidate when metadata, owner, trace, comment, or intended usage differs
- create a new physical blob only when content differs, hash verification fails, or storage policy requires separation
- preserve one `IntakeItem` per explicit selection even when the same blob is reused
- never infer user intent from filename or source path alone

## Storage

Future storage must align with `docs/specs/storage_policy.md` and `docs/specs/project_folder_layout.md`.

Storage rules:

- physical storage must be governed
- blobs are canonical physical storage
- refs declare usage
- types classify/navigation only
- indexes are derivable accelerators
- no project-root `outputs/`
- no `assets/` as runtime
- no filesystem exposure by passive scanning
- `project_manifest.json` remains exposure authority

Comment storage rules:

- comments are stored as governed metadata only in a future durable intake step
- comments are not separate documents
- comments are not chat logs
- comments are not filesystem annotations
- comments are not blobs
- comments belong to `IntakeItem -> StoredObject metadata` in the future durable model

F12.4 does not define a final runtime storage layout.

## Mandatory metadata

Future durable intake metadata must include:

- `object_ref`
- `file_ref`
- `intake_batch_ref`
- `original_filename_sanitized`
- `detected_kind`
- `size_bytes`
- `content_hash`
- `created_at` or `imported_at` as policy allows
- `source_kind`
- `source_ref` sanitized
- `owner_ref`
- `trace_ref`
- `classification`
- `security_sanitization_state`
- `exposure_state`
- `derivation_state`

Optional per-file comment metadata may include:

- `user_comment`
- `comment_author_ref`
- `comment_created_at`
- `comment_sanitization_state`
- `comment_visibility`
- `trace_ref`

Comment metadata rules:

- `user_comment` is optional and empty by default
- `user_comment` is tied to one `IntakeItem`
- comments are descriptive metadata only
- comments are project-scoped
- comments may support human understanding, future search context, and trace explanation
- comments do not affect classification, capability, exposure authority, or derivation eligibility
- comments must be sanitized before durable persistence
- comments must not contain secrets, credentials, tokens, raw sensitive data, or private absolute host paths
- comments must not exceed the governed size limit
- rejected comments must block ingestion only when the active policy explicitly requires it

Metadata must not contain:

- secrets
- raw payloads
- private absolute host paths as portable truth
- credentials
- unsanitized user paths

## Chat comment proposals

`ChatCommentProposal` is a proposal-only chat artifact for intake-level annotations.

Chat may propose:

- adding a comment to an intake item
- editing a comment on an intake item

The governed flow is:

`user message -> ChatCommentProposal -> explicit promotion step -> FileIntakeDraft / IntakeItem update`

Rules:

- chat cannot directly persist comment metadata
- chat cannot mutate `IntakeItem`
- chat cannot mutate `FileIntakeDraft`
- chat output remains proposal-only
- promotion must be explicit and governed
- promotion must generate trace linkage
- promoted comments remain subject to sanitization and policy checks
- chat must not become a hidden comment store

## Exposure

Exposure states:

- `selected`
- `candidate`
- `planned`
- `imported_not_exposed`
- `exposed`
- `blocked`
- `rejected`
- `superseded`

Rules:

- `imported_not_exposed` is allowed
- `exposed` requires `project_manifest.json` governance
- filesystem presence alone does not expose
- document tree shows only exposed or prepared presentation state according to governed rules
- chat may reference intake items but must not become a hidden blob store

## Derivatives

Future derivation planning may include:

- text extraction
- page extraction
- chunking
- thumbnail or preview
- metadata index
- checksum index

Rules:

- derivatives are deterministic and regenerable
- derivatives are not source truth
- derivatives must not be generated in F12.4
- `document_text_runtime` may later consume exposed or eligible objects
- F12.4 does not implement derivation runtime

## Tool relationship

File intake is a governed pipeline, not merely a tool.

File intake may later use small operational/base tools.

`tool_runtime` must not become the intake orchestrator.

`project_runtime` must not be bypassed.

`app_services` remains thin.

UI remains intent/presentation only.

Future intake tools must be single-purpose, auditable, `owner_ref` aware, `trace_ref` aware, and not broad dispatchers.

Candidate future intake tools are declared only:

- `classify_file_format`
- `compute_file_hash`
- `copy_into_file_store`
- `create_stored_object_metadata`
- `plan_derivatives`
- `validate_intake_manifest`

These tools are not implemented or activated in F12.4.

## System View alignment

System View may present:

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

System View must not:

- import
- copy
- classify by itself
- expose
- register
- derive
- execute tools
- authorize
- mutate storage
- edit comments
- persist comments
- generate comments
- infer comments

## Lume alignment

Lume may explain:

- why a file cannot be ingested
- what metadata is missing
- why a format is unsupported
- why source path is not portable truth
- why exposure requires manifest governance
- how intake comments are used
- how to improve a comment safely
- why comment content was flagged

Lume must not:

- import files
- authorize intake
- mutate manifests
- execute tools
- infer project exposure
- inspect raw payloads unless a later governed runtime provides sanitized summaries
- persist comments
- modify comment metadata
- bypass the explicit promotion flow

## Comment sanitization

Comment sanitization aligns with `docs/specs/security_sanitization_policy.md`.

Sanitization must remove or flag:

- secrets
- credentials
- tokens
- private absolute host paths
- raw sensitive data

Sanitization must enforce:

- maximum comment length
- safe encoding
- no executable content

`comment_sanitization_state` values:

- `safe`
- `flagged`
- `rejected`

## Blocking reason codes

- `missing_source`
- `unreadable_source`
- `source_is_directory`
- `unsupported_format`
- `unsafe_filename`
- `unsafe_path`
- `private_absolute_path`
- `missing_owner_ref`
- `missing_trace_ref`
- `missing_file_ref`
- `duplicate_conflict`
- `size_limit_exceeded`
- `sanitization_failed`
- `exposure_not_authorized`
- `derivation_not_available`
- `runtime_not_opened`
- `project_manifest_required`
- `policy_blocked`
- `comment_contains_secrets`
- `comment_contains_private_path`
- `comment_too_large`
- `comment_sanitization_failed`

Comment blocking reason codes are advisory or blocking according to the active policy. They must not block ingestion unless policy explicitly requires blocking.

## Audit preparation

Future advisory audit script:

- `dev/scripts/audits/audit_f12_file_intake_boundary.bat`

The future audit should check:

- no UI filesystem mutation
- no `app_services` intake orchestration
- no `project_runtime` bypass
- no raw host paths persisted as portable truth
- no passive filesystem exposure
- no `assets/` runtime use
- no broad `tool_runtime` orchestrator
- no derivation execution in the spec-only slice

F12.4 does not create this audit script.

## F12.5 implementation plan and audit checklist

Status: PLAN-ONLY / AUDIT-CHECKLIST-ONLY / NOT IMPLEMENTED.

`F12.5` prepares a future `F12.6` minimal governed file intake runtime.

`F12.5` does not implement runtime, copy files, hash files, classify files at runtime, persist metadata, mutate `project_manifest.json`, create registry entries, create graph entries, derive text, execute tools, modify runtime crates, call providers, use network, invoke external binaries, open LLM tool calling, or open semantic/RDF/Oxigraph runtime.

Runtime remains CLOSED after `F12.5`.

### Future crate boundaries

Preferred future `F12.6` ownership:

- `io_runtime`: source file validation, safe copy into governed storage, path normalization, and filesystem boundary checks
- `core_domain`: reusable identity, hash, or ref types only when already present or minimally extended
- `project_runtime`: must not be modified unless a later explicit audit approves project exposure
- `document_text_runtime`: no derivation in `F12.6`
- `tool_runtime`: no intake orchestration in `F12.6`
- `app_services`: no orchestration in `F12.6`; a later UI slice may add a thin adapter only if explicitly opened
- `ui_*`: no implementation in `F12.6`

### Future F12.6 allowed scope

Future `F12.6` may implement only:

- minimal governed file intake runtime
- explicit user-selected files only
- one intake batch per run
- governed `IntakeItem` candidates
- source file existence validation
- source file readability validation
- directory rejection
- safe extension or basic-kind classification only
- content hash only if existing `core_domain` or `io_runtime` support allows it without broad expansion
- copy into governed storage only when storage policy path is explicit
- sanitized metadata creation
- `owner_ref` preservation
- `trace_ref` preservation
- optional `user_comment` preservation after sanitization
- no private absolute host paths persisted as portable truth
- no automatic `project_manifest.json` exposure unless explicitly opened by `F12.6`
- no derivatives

### Future F12.6 forbidden scope

Future `F12.6` must not:

- scan host folders
- passively expose filesystem contents
- accept implicit files
- treat source path as identity
- write back to source folders
- mutate `project_manifest.json` unless separately opened
- generate `registry.json`
- write graph entries
- generate text, chunks, pages, thumbnails, previews, indexes, or derivatives
- use `tool_runtime` as intake orchestrator
- execute tools
- call providers
- use network
- invoke external binaries
- open LLM tool calling
- open semantic/RDF/Oxigraph runtime
- introduce UI or `app_services` authority

### Future accepted input shape

Future `F12.6` input must be explicit and governed:

- one `intake_batch_ref`
- one or more explicit selected source refs
- `owner_ref`
- `trace_ref`
- optional per-item `user_comment`
- policy references needed for size, extension, storage, and sanitization checks

Input must not depend on passive filesystem discovery.

### Future allowed outputs

Future `F12.6` may produce only:

- owner-scoped intake directory or governed file-store location according to existing storage policy
- metadata JSON for an intake item or stored object candidate
- optional batch manifest if explicitly declared before implementation

Future `F12.6` must not produce:

- `project_manifest.json` mutation unless separately opened
- `registry.json`
- graph files
- derivatives
- project-root outputs
- source-folder writes

### Future metadata shape

Future metadata must include:

- `intake_item_id`
- `intake_batch_ref`
- `object_ref` or `stored_object_candidate_ref`
- `file_ref` or `content_hash`
- `owner_ref`
- `trace_ref`
- `original_filename_sanitized`
- `detected_kind`
- `size_bytes`
- `source_kind`
- sanitized source display label only
- `classification`
- `security_sanitization_state`
- `exposure_state`
- `derivation_state`

Optional comment metadata:

- `text`
- `comment_author_ref`
- `comment_created_at`
- `comment_sanitization_state`
- `comment_visibility`

Comment rules:

- `user_comment` is optional
- comments are metadata, not authority
- comments must be sanitized
- comments must not contain secrets
- comments must not contain private absolute host paths
- comments must not affect classification, exposure, or derivation
- chat may propose comments only
- chat persistence is not opened in `F12.6`

### Future error model

Future `F12.6` must use typed error or blocked reason codes including:

- `missing_source`
- `unreadable_source`
- `source_is_directory`
- `unsupported_format`
- `unsafe_filename`
- `unsafe_path`
- `private_absolute_path`
- `missing_owner_ref`
- `missing_trace_ref`
- `duplicate_conflict`
- `size_limit_exceeded`
- `sanitization_failed`
- `comment_contains_secrets`
- `comment_contains_private_path`
- `comment_too_large`
- `comment_sanitization_failed`
- `storage_path_escape`
- `runtime_not_opened`
- `policy_blocked`
- `io_error`

### Required future F12.6 tests

Future `F12.6` must include tests proving:

- valid single file produces intake metadata
- valid batch produces one `IntakeItem` per file
- directory input is rejected
- missing source is rejected
- unreadable source is rejected when testable
- unsupported extension is blocked, not guessed
- unsafe filename is sanitized or rejected according to policy
- private absolute host path is not persisted as portable truth
- `owner_ref` is mandatory
- `trace_ref` is mandatory
- safe `user_comment` is preserved
- unsafe `user_comment` is flagged or rejected according to policy
- comment does not affect classification
- output path cannot escape governed storage root
- no `project_manifest.json` mutation occurs
- no `registry.json` generation occurs
- no graph write occurs
- no derivatives are generated
- no `tool_runtime` orchestration occurs
- no UI or `app_services` authority is introduced

### Required future audit

Future audit script:

- `dev/scripts/audits/audit_f12_file_intake_boundary.bat`

The audit is mandatory before closing `F12.6`.

The audit must check:

- no UI filesystem mutation
- no `app_services` intake orchestration
- no `project_runtime` bypass
- no `project_manifest.json` mutation unless explicitly opened
- no registry generation
- no graph writes
- no derivative generation
- no raw host absolute paths persisted
- no secrets in metadata
- no `assets/` runtime use
- no passive filesystem exposure
- no broad `tool_runtime` orchestrator
- no provider calls
- no network access
- no external binary invocation
- `owner_ref` required
- `trace_ref` required
- `user_comment` sanitized
- source folder remains readonly
- output or copy destination remains governed and owner/project scoped

Closure criteria for `F12.6`:

- implementation scope matches the allowed list
- forbidden scope remains absent
- required tests pass
- `audit_f12_file_intake_boundary.bat` returns acceptable closure status
- declarative validators pass
- Rust validation passes if crates are touched

## F12.5-F12.7 minimal governed file intake baseline closure

Closure state:

- `F12.5`: PLAN-ONLY COMPLETE
- `F12.6`: MINIMAL GOVERNED FILE INTAKE RUNTIME COMPLETE
- `F12.7`: READ-ONLY SYSTEM VIEW VISIBILITY COMPLETE

This baseline now includes:

- explicit user-selected intake batches only
- minimal governed intake runtime for text and markdown only
- mandatory `owner_ref`
- mandatory `trace_ref`
- sanitized per-item metadata
- optional sanitized per-item `user_comment`
- `imported_not_exposed` durable intake state
- blocked-item preservation with sanitized blocking reasons
- readonly `System View` visibility over prepared intake batches
- mandatory boundary audit coverage for the minimal intake slice

This closure does not open:

- project exposure through `project_manifest.json`
- registry generation
- graph writes
- derivatives
- `document_text_runtime` invocation
- `tool_runtime` orchestration
- UI execution authority
- LLM, provider, network, or external-binary execution

## Next allowed expansion candidates

Proposal-only. None of the following are opened by the minimal intake baseline:

1. project exposure gate, defined by `F13.0` as SPEC-only
2. derivatives gate
3. more file formats
4. intake history/index view, defined by `F12.9` as SPEC-only
5. storage dedup hardening, defined by `F12.8` as SPEC-only

## F12.8 storage dedup hardening / intake identity strategy

Status: SPEC-ONLY / HARDENING-STRATEGY-ONLY.

`F12.8` defines duplicate and identity semantics for the minimal governed file intake baseline.

It does not modify crates, copy files, hash files, mutate storage, expose projects, generate registries, write graph entries, create derivatives, invoke `document_text_runtime`, use `tool_runtime`, add UI authority, call providers, use network, invoke external binaries, or change `project_manifest.json`.

### Same artifact rule

Two files are the same physical artifact only when their governed byte-level content hash matches.

Filename equality, source path equality, extension equality, MIME hint equality, size equality, or user comment equality are not sufficient to prove sameness.

If the hash cannot be computed or verified, sameness must remain unknown and intake must not reuse physical storage by assumption.

### Identity relationship

- `content_hash`: hash evidence over file bytes
- `file_ref`: stable content reference derived from or verified by content identity
- `stored_object_id` or `stored_object_candidate_ref`: logical stored object candidate for a governed intake context
- `intake_item_id`: explicit selected-item event inside an intake batch

Relationship rules:

- one `content_hash` may map to one `file_ref`
- one `file_ref` may be referenced by multiple `stored_object_candidate_ref` values
- one `stored_object_candidate_ref` may be produced from one intake item context
- one `intake_item_id` belongs to exactly one `IntakeBatch`
- `intake_item_id` must not become `file_ref`
- `file_ref` must not become project exposure
- `stored_object_candidate_ref` must not imply `project_manifest.json` exposure

### Duplicate import policy

If a user imports duplicate content:

- keep every explicit selection as its own `IntakeItem`
- preserve the sanitized filename/source label for each item
- preserve item-level `owner_ref`, `trace_ref`, comment metadata, and blocking state
- reuse the existing physical blob when allowed by storage policy
- create a new logical stored object candidate when the intake context differs
- mark `duplicate_conflict` only when policy cannot safely represent or disambiguate the duplicate selection

Duplicate import must not:

- overwrite prior metadata
- merge comments
- infer exposure
- mutate `project_manifest.json`
- generate `registry.json`
- write graph entries
- generate derivatives
- call `document_text_runtime`
- call `tool_runtime`

### Future tests

Future implementation hardening should test:

- same bytes produce the same `content_hash`
- same bytes produce or resolve to the same `file_ref`
- different bytes produce different `content_hash` and `file_ref`
- duplicate selections in one batch produce distinct `intake_item_id` values
- duplicate selections reuse physical blob storage when allowed
- duplicate selections preserve distinct sanitized source labels
- duplicate selections preserve distinct user comments
- duplicate selections do not mutate `project_manifest.json`
- duplicate selections do not generate `registry.json`
- duplicate selections do not write graph entries
- duplicate selections do not generate derivatives
- hash unavailable blocks reuse rather than guessing
- `duplicate_conflict` is emitted only for ambiguous duplicate policy cases

### Future audit

Future intake boundary audit should check:

- duplicate content does not create duplicate physical blobs when reuse is allowed
- duplicate content does not collapse distinct `IntakeItem` records
- `file_ref` remains content-based and path-independent
- `stored_object_candidate_ref` remains logical and non-exposing
- `intake_item_id` remains batch-local selection identity
- no raw source path becomes portable identity
- no project exposure, registry, graph, derivative, `document_text_runtime`, or `tool_runtime` side effect is introduced by dedup hardening

## F12.9 intake history/index view

Status: SPEC-ONLY / HISTORY-INDEX-ONLY.

`F12.9` defines a readonly intake history/index view over governed intake metadata.

It does not modify crates, create a runtime index, expose projects, mutate `project_manifest.json`, generate `registry.json`, write graph entries, create derivatives, invoke `document_text_runtime`, use `tool_runtime`, add UI execution authority, call providers, use network, invoke external binaries, or change source storage.

The intake history/index is a derivable, rebuildable, non-authoritative presentation and lookup surface. It may help users understand prior intake batches, duplicate/reuse outcomes, blocked items, owner and trace linkage, and sanitized comments, but it must not become project exposure authority.

### Batch history model

A future intake history view may derive batch-level rows from already governed intake metadata.

Batch history fields may include:

- `intake_batch_ref`
- batch status summary
- item count
- imported item count
- blocked item count
- duplicate/reuse summary
- `owner_ref`
- `trace_ref`
- created or imported timestamp when already present in governed metadata
- sanitized batch comment preview when present
- sanitization summary

Batch history rules:

- batch history is readonly
- batch history is derivable from persisted governed intake metadata
- batch history may be rebuilt from intake records and stored object candidate metadata
- batch history must not be the only source of intake truth
- batch history must not infer project exposure
- missing history index entries must not delete or invalidate intake records

### Item history model

A future intake history view may derive item-level rows from `IntakeItem` metadata.

Item history fields may include:

- `intake_item_id`
- `intake_batch_ref`
- sanitized filename or sanitized source label
- item status: `candidate`, `blocked`, or `imported_not_exposed`
- `file_ref` when available
- `stored_object_candidate_ref` when available
- duplicate/reuse indicator
- blocking reason when present
- `owner_ref`
- `trace_ref`
- sanitization state
- sanitized `user_comment` preview when present

Item history rules:

- item history must preserve one row per explicit `IntakeItem`
- blocked items must remain visible with sanitized blocking reasons
- imported items remain `imported_not_exposed` unless a later project exposure gate explicitly opens exposure
- item history must not contain raw payloads, secrets, credentials, or private absolute host paths
- item history must not classify, reclassify, expose, derive, register, or execute

### Duplicate and reuse visibility

History/index presentation may show duplicate and reuse state as explanation only.

Allowed duplicate/reuse visibility:

- repeated `file_ref` across multiple `IntakeItem` records
- physical blob reuse indicator when governed metadata already records reuse
- distinct `stored_object_candidate_ref` values for distinct intake contexts
- `duplicate_conflict` blocking reason when policy could not disambiguate

Duplicate/reuse visibility must not:

- merge duplicate `IntakeItem` rows
- overwrite item comments or source labels
- infer that reused storage equals project exposure
- create or update dedup mappings
- scan storage to discover duplicates

### Blocked item visibility

Blocked items remain first-class history rows.

History/index presentation may show:

- sanitized blocking reason
- sanitized filename or source label
- owner and trace linkage
- sanitization state
- whether the item produced no stored object candidate

Blocked visibility must not authorize retry, import, exposure, derivation, or any corrective runtime action.

### Owner, trace, and comment visibility

History/index presentation may show `owner_ref` and `trace_ref` as governed references.

Rules:

- `owner_ref` display is not credential authority
- `trace_ref` display is not execution authority
- sanitized comments may be previewed only as metadata
- comments remain non-authoritative for classification, exposure, derivation, and duplicate resolution
- unsafe or rejected comments must be summarized by sanitized warning state only

### Future tests

Future implementation of an intake history/index view should test:

- batch history derives from existing intake metadata
- item history preserves one row per `IntakeItem`
- blocked items remain visible with sanitized blocking reasons
- duplicate selections remain distinct rows
- duplicate/reuse indicators do not merge item identity
- repeated `file_ref` can be displayed without implying project exposure
- `owner_ref` and `trace_ref` are visible as refs only
- sanitized comment previews are visible when safe
- raw source paths are not persisted or displayed as portable truth
- history can be rebuilt after index deletion
- deleting or rebuilding the index does not mutate intake records
- no `project_manifest.json`, `registry.json`, graph, derivative, `document_text_runtime`, `tool_runtime`, provider, network, LLM, or external-binary side effect occurs

### Future audit additions

Future intake boundary audit should also check:

- history/index files, if later implemented, are derivable and rebuildable
- history/index files are non-authoritative
- history/index loss does not imply data loss for intake records
- history/index presence does not imply project exposure
- duplicate/reuse visibility does not collapse distinct `IntakeItem` records
- blocked items remain visible without triggering retries
- sanitized comments are previews only and do not become classification or exposure authority
- no history/index code writes `project_manifest.json`, `registry.json`, graph files, derivatives, or source files
- no history/index code calls `tool_runtime`, `document_text_runtime`, providers, network, LLMs, or external binaries

## F13.0 Project Exposure Gate

Status: SPEC-ONLY / GATE-ONLY / NOT IMPLEMENTED.

`F13.0` defines the governed project exposure gate for promoting an already imported, supported, sanitized `IntakeItem` or `StoredObject` candidate from `imported_not_exposed` into project visibility.

It does not implement runtime, modify crates, create a manifest writer, create a registry generator, write graph entries, create derivatives, call `document_text_runtime`, call `tool_runtime`, add UI authority, expose blocked items, expose unsupported items, infer exposure from `file_store` presence, or scan the filesystem.

The canonical exposure chain is:

`ExposureRequest -> ExposureCandidate -> HumanConfirmation -> future project_manifest update -> exposed_to_project`

Only a later explicit implementation slice may perform the future `project_manifest.json` mutation.

### Exposure request

`ExposureRequest` is a governed request to make an already imported intake object visible to a project.

It must reference:

- `exposure_request_id`
- `intake_item_ref`
- `stored_object_candidate_ref` or `object_ref`
- `file_ref`
- `owner_ref`
- `trace_ref`
- requested project ref
- requested document tree placement or presentation hint
- sanitized display label
- requested exposure rationale

Rules:

- request input must be refs and sanitized metadata only
- request input must not contain raw payloads, private absolute host paths, or secrets
- the request does not mutate `project_manifest.json`
- the request does not expose the item
- the request does not create registry, graph, derivative, or document tree state

### Exposure candidate

`ExposureCandidate` is an inert evaluation artifact prepared from an `ExposureRequest`.

It may summarize:

- whether the intake item is `imported_not_exposed`
- whether the item is supported
- whether the item is not blocked
- whether required refs are present
- whether metadata is sanitized
- whether `owner_ref` and `trace_ref` are preserved
- whether duplicate exposure policy is satisfied
- whether `project_manifest.json` governance is required

Candidate states:

- `candidate`
- `blocked`
- `needs_confirmation`
- `superseded`
- `stale`

Blocked reasons may include:

- `item_not_imported`
- `item_already_exposed`
- `blocked_item`
- `unsupported_item`
- `missing_owner_ref`
- `missing_trace_ref`
- `missing_file_ref`
- `missing_stored_object_ref`
- `unsanitized_metadata`
- `duplicate_exposure_conflict`
- `project_manifest_required`
- `policy_blocked`
- `runtime_not_opened`

`ExposureCandidate` must not perform policy resolution, write manifests, generate registries, write graphs, derive text, execute tools, or expose items.

### Human confirmation requirement

Project exposure requires explicit human confirmation before any future `project_manifest.json` mutation.

Confirmation must preserve:

- `exposure_request_id`
- `exposure_candidate_id`
- `intake_item_ref`
- `stored_object_candidate_ref` or `object_ref`
- `owner_ref`
- `trace_ref`
- decision
- reviewer or confirmer ref
- timestamp when policy allows it

Accepted confirmation means the user approved the exposure proposal. It does not by itself mutate `project_manifest.json` and does not create document tree visibility until a later governed implementation writes the manifest.

Rejected, deferred, stale, blocked, or changes-requested confirmation must keep the item `imported_not_exposed`.

### Exposure transition

The only governed transition opened by this spec as a future possibility is:

`imported_not_exposed -> exposed_to_project`

Transition rules:

- source state must be `imported_not_exposed`
- blocked items must not transition
- unsupported items must not transition
- missing or unsafe metadata must block transition
- `owner_ref` must be preserved
- `trace_ref` must be preserved
- `file_ref` must remain content identity and must not become exposure identity
- `stored_object_candidate_ref` or `object_ref` must remain logical object identity
- project visibility starts only after a governed `project_manifest.json` declaration exists

`exposed_to_project` is the canonical exposure state for post-gate project visibility. Legacy `exposed` wording remains conceptual and must not be used to bypass this gate.

### Project manifest authority

`project_manifest.json` is the only project exposure authority.

Rules:

- filesystem presence is not exposure
- `file_store` blob presence is not exposure
- `StoredObject` metadata is not exposure
- registry presence is not exposure
- graph presence is not exposure
- document tree presentation is not exposure authority
- chat references are not exposure authority
- UI selection is not exposure authority

No project exposure may be inferred from scanning folders, reading `file_store`, seeing intake metadata, or viewing history/index rows.

### Duplicate exposure policy

Duplicate content may be exposed more than once only when the project manifest records distinct governed project entries or usages.

Rules:

- same `file_ref` may appear in multiple exposure candidates
- duplicate content must not collapse distinct `IntakeItem` or logical object identity
- duplicate exposure must preserve distinct display labels, comments, owners, traces, and usage rationale when present
- if the same logical object is already exposed in the same project context, a new exposure candidate must become `item_already_exposed` or `duplicate_exposure_conflict`
- exposing one duplicate does not expose all duplicates sharing the same `file_ref`
- physical blob reuse does not imply project exposure reuse

### Document tree visibility

After a future governed implementation writes the project manifest, `project_runtime` may prepare document tree state from that manifest.

Rules:

- document tree visibility follows `project_manifest.json`
- the document tree must not scan `file_store`
- the document tree must not scan intake metadata
- the document tree must not expose blocked or unsupported items
- document tree nodes must use sanitized display labels
- document tree nodes must remain readonly unless a later editing phase opens mutation

### Registry and graph relationship

`registry.json` remains derivable and non-authoritative.

Rules:

- registry may accelerate navigation only after manifest exposure exists
- registry must be rebuildable from authoritative sources
- registry absence must not remove manifest exposure
- registry presence must not create manifest exposure

Graph entries remain optional and future-only.

Rules:

- graph writes are not opened by `F13.0`
- graph presence must not create exposure
- graph absence must not block manifest-governed exposure unless a later policy explicitly requires it

### Rollback and revocation

Rollback and revocation are future-only.

Future revocation must be governed by a separate request, candidate, confirmation, and manifest update flow.

F13.0 does not define runtime revocation, delete blobs, delete intake metadata, rewrite history, remove registry entries, remove graph entries, or generate tombstones.

### Future tests

Future implementation must test:

- only `imported_not_exposed` supported items can become exposure candidates
- blocked items cannot become exposure candidates
- unsupported items cannot become exposure candidates
- missing `owner_ref` blocks exposure
- missing `trace_ref` blocks exposure
- missing `file_ref` blocks exposure
- accepted human confirmation is required before future manifest mutation
- accepted confirmation alone does not expose without manifest update
- future manifest update preserves `owner_ref` and `trace_ref`
- filesystem presence does not expose items
- `file_store` presence does not expose items
- duplicate `file_ref` exposure preserves distinct `IntakeItem` and object identity
- exposing one duplicate does not expose all duplicates
- document tree shows only manifest-exposed objects
- registry remains derivable and non-authoritative
- no graph write, derivative generation, `document_text_runtime`, `tool_runtime`, provider, network, LLM, or external-binary side effect occurs

### Future audit requirements

Future exposure-gate audit must check:

- no filesystem scanning creates exposure
- no automatic exposure from intake runtime
- no exposure of blocked items
- no exposure of unsupported items
- `project_manifest.json` is the only exposure authority
- registry generation is not required and remains non-authoritative
- graph writes are absent unless a later explicit graph slice opens them
- document tree visibility is manifest-driven only
- duplicate exposure policy preserves refs and does not collapse items
- `owner_ref` and `trace_ref` are preserved across request, candidate, confirmation, and future exposure state
- rollback or revocation code is absent unless separately opened
- no `document_text_runtime`, `tool_runtime`, provider, network, LLM, or external-binary side effect is introduced

## F13.1 Legacy Exposure/Import Bypass Hardening

Status: SPEC-ONLY / AUDIT-PLAN / NOT IMPLEMENTED.

`F13.1` hardens the boundary around `F12` intake and `F13.0` exposure by classifying legacy import, tree-scan, chat-resource, and direct-derivation flows that must not become an implicit project exposure pipeline.

The canonical governed exposure path is:

`explicit user-selected file -> F12 intake -> imported_not_exposed -> F13 exposure request -> F13 exposure candidate -> human confirmation -> future project_manifest update -> document tree visibility`

No legacy helper, direct UI path, chat-resource registration, derivative call, or passive filesystem scan may replace this chain.

### Legacy flow classification

The following legacy flows are classified for boundary hardening:

- `import_project_document`: deprecated for production project intake or project exposure; must be blocked before any `F13` runtime implementation; may remain only for fixtures, migration tests, or explicitly non-authoritative legacy tests until replaced
- `list_project_documents`: deprecated as a source of project visibility; must be blocked before any `F13` runtime implementation when used for document tree authority; may remain only for legacy fixture tests until tree state is manifest-driven
- `register_chat_resource`: legacy tolerated only for chat-local reference staging that does not create project exposure, document tree visibility, storage authority, or derivation authority; it must not be promoted into intake or exposure authority
- `derive_document_text`: allowed only as a separate derivative runtime for already governed eligible objects in a later explicit phase; deprecated as a direct UI-triggered import or exposure side effect and must be blocked before `F13` runtime
- any UI or Slint direct file import, direct project placement, direct exposure, direct chat-to-project promotion, direct tree refresh from filesystem, or direct derivation path: deprecated and must be blocked before `F13` runtime

### Forbidden bypasses

The following are forbidden as project exposure bypasses:

- importing a file into project folders and treating that copy as exposure
- scanning project folders, `file_store`, or intake metadata to infer project visibility
- registering a chat resource and treating the registration as intake, exposure, or project authorization
- deriving text directly from external files or newly copied files as part of intake or exposure
- populating document tree visibility from filesystem presence, copied files, registry rows, graph rows, or chat references
- allowing `app_services` to shadow exposure request, candidate, confirmation, or manifest authority
- allowing UI to import, expose, register, derive, or confirm directly

### Migration notes for legacy functions

Future migration must preserve narrow responsibilities:

- `import_project_document` must migrate behind `F12` intake plus `F13` exposure, or remain fixture-only
- `list_project_documents` must migrate to prepared tree state derived from manifest-governed exposure, or remain fixture-only
- `register_chat_resource` may remain chat-local only if clearly non-authoritative and non-promoting
- `derive_document_text` must remain separate from intake and exposure and may later consume only already governed eligible objects

### STOP conditions before F13 runtime

No `F13` runtime implementation may begin until all of the following are true:

- no production UI path directly calls legacy project import as an exposure substitute
- no production UI path directly refreshes document tree authority from filesystem scanning
- no production UI path directly calls `derive_document_text` as part of import or exposure
- no production UI path promotes chat resources into project visibility without `F12` intake plus `F13` exposure
- `app_services` does not shadow exposure request, candidate, confirmation, or manifest authority
- future `audit_f13_exposure_boundary.bat` exists and passes

### Future required tests

Future implementation and migration must test:

- legacy import does not create exposure
- copied files do not become exposed without manifest update
- chat resource registration does not create intake or exposure
- filesystem scanning does not create document tree exposure
- direct derivation does not run during intake or exposure
- blocked or unsupported items cannot bypass the exposure gate through legacy paths
- duplicate content cannot bypass request, candidate, confirmation, and manifest authority

### Future required audit

Future `audit_f13_exposure_boundary.bat` must check:

- no UI direct import-to-project path remains authoritative
- no UI direct derivation path remains authoritative
- no document tree authority is sourced from `list_project_documents`
- no chat-resource path becomes intake or exposure authority
- no `app_services` layer shadows exposure governance
- no `project_manifest.json` exposure is inferred from copied files, registry rows, graph rows, history rows, or filesystem presence
- no derivative generation is triggered by exposure
- no `tool_runtime`, provider, network, LLM, or external-binary side effect is introduced

## F13.4 Manifest Exposure Contract Hardening

Status: SPEC-ONLY / CONTRACT-HARDENING / NOT IMPLEMENTED.

`F13.4` closes the exact non-executing contract for promoting an already governed `imported_not_exposed` intake artifact into manifest-governed project visibility.

The canonical future lifecycle is:

`selected external file -> F12 intake -> imported_not_exposed -> ExposureRequest -> ExposureCandidate -> HumanConfirmation -> future ManifestExposureEntry -> exposed_to_project`

`F13.4` does not modify crates, implement runtime, create a `project_manifest.json` writer, generate `registry.json`, write graph entries, create derivatives, call `document_text_runtime`, call `tool_runtime`, or add UI or `app_services` authority.

### Canonical terminology

`exposed_to_project` is the canonical project exposure state.

Legacy `exposed` wording is deprecated shorthand only and must either be avoided or explicitly mapped to `exposed_to_project`.

The following never imply project exposure:

- filesystem presence
- `file_store` blob presence
- stored-object metadata presence
- `registry.json` presence
- graph presence
- `System View` visibility
- intake history/index visibility
- document tree visibility before manifest authority exists

### ExposureRequest contract

`ExposureRequest` is the formal non-executing request to make a governed imported artifact eligible for future manifest exposure.

Required fields:

- `exposure_request_id`
- `intake_item_ref` or `stored_object_candidate_ref`
- `requested_by_ref`
- `owner_ref`
- `trace_ref`
- `requested_at`
- `requested_exposure_kind`
- optional sanitized user intent or comment
- expected manifest target kind
- duplicate handling preference when policy allows it
- `request_status`

Rules:

- `ExposureRequest` is non-executing and does not write `project_manifest.json`
- request inputs must be portable refs and sanitized metadata only
- request inputs must not contain raw payloads, secrets, or private absolute host paths
- the request does not create `registry.json`, graph entries, derivatives, document tree entries, or viewer entries
- the request does not authorize exposure and does not imply confirmation

### ExposureCandidate contract

`ExposureCandidate` is the inert evaluation artifact prepared from an `ExposureRequest`.

Required fields:

- `exposure_candidate_id`
- `exposure_request_ref`
- `candidate_status`
- `eligibility_result`
- source intake item refs
- `file_ref` or `content_hash` evidence
- `stored_object_candidate_ref`
- sanitized display label
- blocking reasons
- duplicate assessment
- `owner_ref`
- `trace_ref`

Eligibility rules:

- only `imported_not_exposed` items may become candidates
- blocked items must never become exposure candidates
- unsupported items must never become exposure candidates
- missing or unreadable stored objects must block
- missing `owner_ref` or `trace_ref` must block
- unsafe metadata must block or require sanitized representation before any future runtime exposure
- candidate evaluation is descriptive only and must not resolve policy by side effect

### HumanConfirmation contract

`HumanConfirmation` is the explicit non-executing reviewer decision event required before any future manifest exposure.

Required fields:

- `confirmation_id`
- `exposure_candidate_ref`
- `reviewer_ref`
- decision: `accepted | rejected`
- `confirmed_at`
- `trace_ref`
- `stale_check_result`
- `risk_acknowledgement` when policy requires it

Rules:

- accepted confirmation is required before any future manifest exposure
- confirmation is not execution and does not mutate `project_manifest.json`
- rejected confirmation must not mutate any authority
- stale or unknown candidate state must block future runtime exposure
- confirmation must preserve `owner_ref`, `trace_ref`, and candidate linkage by ref

### ManifestExposureEntry contract

`ManifestExposureEntry` is the minimal future manifest entry shape that a later explicit runtime slice may write to `project_manifest.json`.

Required future fields:

- `manifest_entry_id`
- `exposed_object_ref`
- `file_ref`
- `stored_object_ref` or `stored_object_candidate_ref`, depending on canonical naming active at implementation time
- `source_intake_item_ref`
- `exposure_request_ref`
- `exposure_candidate_ref`
- `confirmation_ref`
- `owner_ref`
- `trace_ref`
- `exposed_to_project_at`
- `artifact_kind`
- `display_label`
- `content_kind` or `basic_kind`
- sanitized metadata refs only
- `exposure_state = exposed_to_project`

Rules:

- `project_manifest.json` is the only project exposure authority
- the manifest entry must use portable refs and sanitized labels only
- the manifest entry must not contain raw payloads, secrets, or private absolute host paths
- the manifest entry must not imply derivative generation
- the manifest entry must not imply graph writes or registry updates
- the manifest entry is a future runtime output and is not created by `F13.4`

### Duplicate exposure policy hardening

Duplicate content remains governed and explicit.

Rules:

- the same `content_hash` or `file_ref` may reuse one physical blob
- physical blob reuse must not collapse `intake_item_id`, `owner_ref`, `trace_ref`, comments, request identity, candidate identity, or confirmation identity
- the same physical file may produce distinct logical manifest entries only when explicitly confirmed and policy allows it
- duplicate exposure must be visible and traceable and must never be silent
- future runtime must either block or warn on duplicate exposure according to explicit policy, never by hidden convenience

### Failure model

Future runtime must report structured safe failures using typed categories such as:

- `source_not_imported`
- `blocked_intake_item`
- `unsupported_kind`
- `missing_owner_ref`
- `missing_trace_ref`
- `stale_candidate`
- `duplicate_policy_block`
- `unsafe_metadata`
- `missing_stored_object`
- `manifest_conflict`
- `unauthorized_transition`

Failure reporting must be safe for `System View` presentation and must not expose secrets, raw payloads, or private absolute host paths.

### System View alignment

`System View` may later present readonly summaries of:

- exposure requests
- exposure candidates
- blocking reasons
- confirmation state
- future manifest exposure status
- duplicate warnings
- `owner_ref`
- `trace_ref`

`System View` must not:

- confirm
- expose
- mutate
- write `project_manifest.json`
- resolve policy
- trigger runtime

### Document Tree alignment

The `Document Tree` may only show artifacts that are exposed through `project_manifest.json`.

The `Document Tree` must not:

- scan the filesystem for exposure authority
- consume intake history/index as authority
- expose items itself
- treat blob presence, registry rows, graph rows, or `System View` presence as project membership

### Future audit hardening

Future exposure audit requirements extend to:

- no `project_manifest.json` writes outside an explicit future exposure runtime slice
- no `list_project_documents` authority
- no filesystem scanning as exposure
- no registry or graph writes from exposure
- no derivatives from exposure
- no UI exposure authority
- no `tool_runtime` orchestration
- no `document_text_runtime` trigger during exposure
- no raw host paths in `ManifestExposureEntry`

### Closure note

`F13.4` is SPEC-only.

It changes no crates, introduces no runtime, adds no manifest writer, implements no project exposure, and only closes the contract required for a later `F13.5` audit or runtime-preparation slice.

## F13.5A Manifest Exposure Runtime Plan

Status: SPEC-ONLY / AUDIT-PLAN-ONLY / NOT IMPLEMENTED.

`F13.5A` converts the `F13.4` manifest exposure contract into the exact implementation checklist and mandatory audit plan for a later minimal runtime slice.

`F13.5A` does not implement runtime, modify crates, write `project_manifest.json`, generate `registry.json`, write graph entries, create derivatives, call `document_text_runtime`, call `tool_runtime`, add UI authority, add `app_services` authority, or change exposure state.

### Future runtime owner

Future minimal manifest exposure runtime ownership is:

- `project_runtime`: canonical owner of future manifest exposure runtime because `project_manifest.json` is project authority
- `io_runtime`: low-level safe file IO only if a later implementation needs it, without becoming exposure authority
- `app_services`: thin prepared call surface only in a later explicit slice
- UI: request intent and readonly prepared state only

### Allowed future F13.5B runtime scope

Future `F13.5B` may implement only:

- promotion of one already `imported_not_exposed` item
- required `ExposureRequest`
- required `ExposureCandidate`
- required accepted `HumanConfirmation`
- required non-stale candidate
- required `owner_ref`
- required `trace_ref`
- write of one `ManifestExposureEntry` to `project_manifest.json`
- preservation of portable refs only
- structured success or failure result

### Forbidden future F13.5B runtime scope

Future `F13.5B` must not:

- expose blocked items
- expose unsupported items
- scan the filesystem for exposure authority
- generate `registry.json`
- write graph entries
- create derivatives
- call `document_text_runtime`
- call `tool_runtime`
- invoke LLMs, providers, network access, or external binaries
- store raw host absolute paths
- collapse duplicate logical identity by blob reuse or convenience
- allow UI or `app_services` to become exposure authority

### Future input contract

Future runtime input must require:

- one `ExposureRequest`
- one `ExposureCandidate`
- one accepted `HumanConfirmation`
- one target `imported_not_exposed` intake or stored-object candidate ref
- `owner_ref`
- `trace_ref`
- duplicate handling policy input when required by policy
- stale-check result proving the candidate remains valid at runtime entry

Rules:

- input artifacts must be portable refs and sanitized metadata only
- input artifacts must not contain raw payloads, secrets, or private absolute host paths
- runtime must reject any request chain that bypasses request, candidate, or accepted confirmation

### Future output contract

Future runtime output may produce only:

- one governed `ManifestExposureEntry` written to `project_manifest.json`
- one structured runtime result describing success or typed safe failure

Rules:

- output must preserve portable refs only
- output must preserve `owner_ref` and `trace_ref`
- output must preserve duplicate-safe logical identity
- output must not imply registry generation, graph writes, derivatives, or tree mutation by convenience

### Future manifest write policy

When `F13.5B` opens later:

- `project_runtime` is the only approved manifest writer boundary
- the manifest write must append or record exactly one governed `ManifestExposureEntry`
- the manifest write must preserve prior manifest authority and must not rewrite exposure history ad hoc
- the manifest write must persist sanitized portable refs only
- the manifest write must not persist raw payloads, secrets, or private absolute host paths

### Duplicate policy enforcement

Future runtime must enforce:

- physical blob reuse is allowed
- logical exposure identity must remain distinct unless explicit policy says the same logical entry is being updated
- duplicate content must never silently collapse `intake_item_id`, request identity, candidate identity, confirmation identity, `owner_ref`, `trace_ref`, comments, or usage rationale
- duplicate blocking or warning behavior must be structured and visible

### Stale candidate enforcement

Future runtime must require a non-stale candidate at execution entry.

Rules:

- rejected confirmation blocks
- stale candidate blocks
- unknown stale-check result blocks
- candidate drift between confirmation and manifest write must block and return a typed safe failure

### Required typed failures

Future `F13.5B` must return structured safe failures including at minimum:

- `source_not_imported`
- `blocked_intake_item`
- `unsupported_kind`
- `missing_owner_ref`
- `missing_trace_ref`
- `rejected_confirmation`
- `stale_candidate`
- `duplicate_policy_block`
- `unsafe_metadata`
- `missing_stored_object`
- `manifest_conflict`
- `unauthorized_transition`

### Required future tests

Future `F13.5B` must test:

- valid exposure from an `imported_not_exposed` item
- blocked item cannot expose
- unsupported item cannot expose
- missing `owner_ref` blocks
- missing `trace_ref` blocks
- rejected confirmation blocks
- stale candidate blocks
- duplicate hash does not collapse logical exposure identity
- manifest entry uses portable refs only
- no raw host paths are persisted
- no registry writes occur
- no graph writes occur
- no derivatives occur
- no `document_text_runtime` call occurs
- no `tool_runtime` orchestration occurs
- no UI or `app_services` authority is introduced

### Mandatory future audit

Future `F13.5B` closure requires `dev/scripts/audits/audit_f13_manifest_exposure_runtime_boundary.bat`.

The audit must check:

- manifest writer exists only in approved `project_runtime` boundary when runtime opens
- no `project_manifest.json` writes from `ui_slint`, `app_services`, `io_runtime`, `tool_runtime`, or `document_text_runtime`
- no `list_project_documents` or filesystem scanning authority
- no registry, graph, or derivative writes from exposure
- no `document_text_runtime` trigger from exposure
- no `tool_runtime` orchestration
- no raw absolute host path patterns in manifest exposure outputs
- no bypass of `ExposureRequest -> ExposureCandidate -> accepted HumanConfirmation`

### Closure criteria before F13.5B runtime

Before `F13.5B` may be declared ready to implement:

- `F13.4` contract remains canonical and unambiguous
- `audit_f13_exposure_boundary.bat` continues to pass with no FAIL findings
- future `audit_f13_manifest_exposure_runtime_boundary.bat` is registered as required
- future runtime ownership remains limited to `project_runtime`
- no new bypass path is opened in UI, `app_services`, `io_runtime`, `tool_runtime`, or `document_text_runtime`

## Invariants

- `INV-FILE-INTAKE-001`: file intake is governed, not a raw copy operation.
- `INV-FILE-INTAKE-002`: UI captures selection intent only.
- `INV-FILE-INTAKE-003`: source paths are not portable identity.
- `INV-FILE-INTAKE-004`: original host files remain readonly.
- `INV-FILE-INTAKE-005`: filesystem presence does not imply project exposure.
- `INV-FILE-INTAKE-006`: `project_manifest.json` remains exposure authority.
- `INV-FILE-INTAKE-007`: every intake item requires `owner_ref` and `trace_ref` before durable import.
- `INV-FILE-INTAKE-008`: metadata must be sanitized and must not contain secrets or raw payloads.
- `INV-FILE-INTAKE-009`: derivatives are regenerable and not source truth.
- `INV-FILE-INTAKE-010`: intake may use tools later, but `tool_runtime` must not become a general intake orchestrator.
- `INV-FILE-INTAKE-011`: `app_services` and UI must remain thin.
- `INV-FILE-INTAKE-012`: F12.4 creates no runtime, files, manifests, registry entries, graph entries, or derivatives.
- `INV-FILE-INTAKE-COMMENT-001`: comments are metadata, not authority.
- `INV-FILE-INTAKE-COMMENT-002`: UI captures comments but does not persist.
- `INV-FILE-INTAKE-COMMENT-003`: chat may propose but not persist comments.
- `INV-FILE-INTAKE-COMMENT-004`: comments must be sanitized.
- `INV-FILE-INTAKE-COMMENT-005`: comments must not contain secrets or private paths.
- `INV-FILE-INTAKE-COMMENT-006`: comments do not affect classification, exposure, or derivation.
- `INV-FILE-INTAKE-COMMENT-007`: comments are project-scoped metadata.
- `INV-FILE-INTAKE-PLAN-001`: F12.5 is plan-only.
- `INV-FILE-INTAKE-PLAN-002`: F12.5 introduces no runtime.
- `INV-FILE-INTAKE-PLAN-003`: F12.6 must not implement project exposure unless separately opened.
- `INV-FILE-INTAKE-PLAN-004`: F12.6 must not generate derivatives.
- `INV-FILE-INTAKE-PLAN-005`: F12.6 must not use `tool_runtime` as intake orchestrator.
- `INV-FILE-INTAKE-PLAN-006`: F12.6 must preserve `owner_ref` and `trace_ref`.
- `INV-FILE-INTAKE-PLAN-007`: source paths are transient and not portable truth.
- `INV-FILE-INTAKE-PLAN-008`: `user_comment` remains sanitized metadata and non-authoritative.
- `INV-FILE-INTAKE-PLAN-009`: future `audit_f12_file_intake_boundary` is mandatory before closure.
- `INV-FILE-INTAKE-PLAN-010`: `project_runtime`, `app_services`, and UI authority remain unchanged.
- `INV-FILE-INTAKE-BASELINE-001`: `F12.5`, `F12.6`, and `F12.7` together define the minimal governed file intake baseline.
- `INV-FILE-INTAKE-BASELINE-002`: the minimal intake baseline remains `imported_not_exposed` only and does not open project exposure.
- `INV-FILE-INTAKE-BASELINE-003`: the minimal intake baseline does not generate registries, graph writes, or derivatives.
- `INV-FILE-INTAKE-BASELINE-004`: the minimal intake baseline does not invoke `document_text_runtime` or use `tool_runtime` as an intake orchestrator.
- `INV-FILE-INTAKE-BASELINE-005`: `System View` visibility remains readonly and consumes prepared intake state only.
- `INV-FILE-INTAKE-BASELINE-006`: UI, providers, network access, LLM execution, and external binaries remain outside the minimal intake baseline.
- `INV-FILE-INTAKE-DEDUP-001`: physical artifact sameness is determined by governed content hash, not filename or source path.
- `INV-FILE-INTAKE-DEDUP-002`: duplicate user selections remain distinct `IntakeItem` records.
- `INV-FILE-INTAKE-DEDUP-003`: duplicate content may reuse one physical blob without merging logical stored object candidates.
- `INV-FILE-INTAKE-DEDUP-004`: `file_ref` identifies content and must not imply project exposure.
- `INV-FILE-INTAKE-DEDUP-005`: `stored_object_candidate_ref` identifies logical intake context and must not imply project exposure.
- `INV-FILE-INTAKE-DEDUP-006`: dedup hardening must not mutate `project_manifest.json`, generate registries, write graph entries, or generate derivatives.
- `INV-FILE-INTAKE-DEDUP-007`: dedup hardening must not invoke `document_text_runtime`, `tool_runtime`, providers, network access, LLM execution, or external binaries.
- `INV-FILE-INTAKE-HISTORY-001`: `F12.9` intake history/index is SPEC-only until separately implemented.
- `INV-FILE-INTAKE-HISTORY-002`: intake history/index is readonly, derivable, rebuildable, and non-authoritative.
- `INV-FILE-INTAKE-HISTORY-003`: intake history/index must not imply project exposure or mutate `project_manifest.json`.
- `INV-FILE-INTAKE-HISTORY-004`: intake history/index must preserve one visible row per governed `IntakeItem`.
- `INV-FILE-INTAKE-HISTORY-005`: blocked items must remain visible with sanitized blocking reasons.
- `INV-FILE-INTAKE-HISTORY-006`: duplicate/reuse visibility must not collapse item identity or create dedup mappings.
- `INV-FILE-INTAKE-HISTORY-007`: history/index comment previews are sanitized metadata only and non-authoritative.
- `INV-FILE-INTAKE-HISTORY-008`: history/index must not generate `registry.json`, graph writes, derivatives, or call `document_text_runtime` or `tool_runtime`.
- `INV-PROJECT-EXPOSURE-001`: `F13.0` Project Exposure Gate is SPEC-only until separately implemented.
- `INV-PROJECT-EXPOSURE-002`: project exposure requires explicit governed request, candidate, and human confirmation before any future manifest mutation.
- `INV-PROJECT-EXPOSURE-003`: `project_manifest.json` is the only project exposure authority.
- `INV-PROJECT-EXPOSURE-004`: filesystem presence, `file_store` presence, registry presence, graph presence, history/index presence, UI selection, and chat references must not imply exposure.
- `INV-PROJECT-EXPOSURE-005`: only supported `imported_not_exposed` items may be candidates for future exposure.
- `INV-PROJECT-EXPOSURE-006`: blocked and unsupported items must not be exposed.
- `INV-PROJECT-EXPOSURE-007`: `owner_ref`, `trace_ref`, `file_ref`, and logical object refs must be preserved through the exposure chain.
- `INV-PROJECT-EXPOSURE-008`: duplicate exposure must not collapse distinct intake items or logical object identity.
- `INV-PROJECT-EXPOSURE-009`: document tree visibility after exposure is manifest-driven and presentation-only.
- `INV-PROJECT-EXPOSURE-010`: registry remains derivable and non-authoritative; graph remains optional and future-only.
- `INV-PROJECT-EXPOSURE-011`: rollback and revocation are future-only and not opened by `F13.0`.
- `INV-PROJECT-EXPOSURE-012`: `F13.0` must not create runtime, writers, registries, graph writes, derivatives, `document_text_runtime` calls, `tool_runtime` calls, providers, network, LLM execution, external binaries, or UI authority.
- `INV-PROJECT-EXPOSURE-013`: `exposed_to_project` is the canonical project exposure state and legacy `exposed` wording is shorthand only.
- `INV-PROJECT-EXPOSURE-014`: `imported_not_exposed` is not project membership and must not appear as project exposure without manifest authority.
- `INV-PROJECT-EXPOSURE-015`: manifest-governed exposure requires `ExposureRequest`, `ExposureCandidate`, and accepted `HumanConfirmation` before any future runtime manifest write.
- `INV-PROJECT-EXPOSURE-016`: accepted confirmation is required but is not itself execution or manifest mutation.
- `INV-PROJECT-EXPOSURE-017`: `ManifestExposureEntry` is a future runtime output only and is not created by `F13.4`.
- `INV-PROJECT-EXPOSURE-018`: duplicate physical blob reuse must not collapse logical exposure identity, request identity, candidate identity, confirmation identity, comments, `owner_ref`, or `trace_ref`.
- `INV-PROJECT-EXPOSURE-019`: `registry.json`, graph state, derivatives, document tree presence, and `System View` presence remain non-authoritative for project exposure.
- `INV-PROJECT-EXPOSURE-020`: UI, `System View`, and `Document Tree` must not authorize exposure.
- `INV-PROJECT-EXPOSURE-021`: manifest exposure refs must be portable and sanitized and must not contain raw payloads, secrets, or private absolute host paths.
- `INV-PROJECT-EXPOSURE-022`: `owner_ref` and `trace_ref` are mandatory across request, candidate, confirmation, and future manifest exposure entry.
- `INV-PROJECT-EXPOSURE-PLAN-001`: `F13.5A` is SPEC-only and introduces no runtime.
- `INV-PROJECT-EXPOSURE-PLAN-002`: future `F13.5B` manifest exposure runtime is owned by `project_runtime` only.
- `INV-PROJECT-EXPOSURE-PLAN-003`: future `F13.5B` must require `ExposureRequest`, `ExposureCandidate`, accepted `HumanConfirmation`, non-stale candidate state, `owner_ref`, and `trace_ref`.
- `INV-PROJECT-EXPOSURE-PLAN-004`: future `F13.5B` must not generate `registry.json`, graph writes, derivatives, `document_text_runtime` calls, or `tool_runtime` orchestration.
- `INV-PROJECT-EXPOSURE-PLAN-005`: future manifest exposure runtime must preserve duplicate-safe logical identity and must not collapse identity by blob reuse.
- `INV-PROJECT-EXPOSURE-PLAN-006`: future `audit_f13_manifest_exposure_runtime_boundary.bat` is mandatory before `F13.5B` closure.
- `INV-LEGACY-BYPASS-001`: the only canonical project exposure path is `F12` intake followed by `F13` exposure gate and future manifest-governed visibility.
- `INV-LEGACY-BYPASS-002`: legacy import, tree scanning, chat-resource registration, and direct derivation must not become implicit project exposure pipelines.
- `INV-LEGACY-BYPASS-003`: filesystem presence, copied files, chat references, registry rows, graph rows, and derivatives must not create project exposure.
- `INV-LEGACY-BYPASS-004`: UI and `app_services` must not import, expose, register, derive, confirm, or shadow manifest authority directly.
- `INV-LEGACY-BYPASS-005`: `import_project_document` and `list_project_documents` are legacy-only and must be blocked from production exposure authority before `F13` runtime.
- `INV-LEGACY-BYPASS-006`: `register_chat_resource` may remain chat-local only and must not become intake or exposure authority.
- `INV-LEGACY-BYPASS-007`: `derive_document_text` remains a separate derivative concern and must not be triggered by intake or exposure.
- `INV-LEGACY-BYPASS-008`: `F13.1` is SPEC-only and creates no runtime, manifest writer, registry generator, graph writer, derivative call, `tool_runtime` call, `document_text_runtime` call, or UI authority.

## Related specs

- `governance/GOVERNANCE.md`
- `governance/FUNCTIONAL_SCOPE.md`
- `architecture/ARCHITECTURE.md`
- `governance/WORKSPACE_RULES.md`
- `docs/specs/storage_policy.md`
- `docs/specs/project_folder_layout.md`
- `docs/specs/project_runtime.md`
- `docs/specs/document_tree.md`
- `docs/specs/document_references.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/security_sanitization_policy.md`
- `docs/specs/system_observability.md`
- `docs/specs/system_view.md`
- `docs/specs/tool_implementation_governance.md`
- `docs/specs/tools_catalogs.md`

---

### !REL_FILE!

# flow_control_policy

## Purpose

Define F9-ready declarative rules for confirmation friction and pending action grouping.

This spec prevents over-confirmation while keeping risky or mutable operations human-governed.

## Scope F9

F9 may declare:

- when confirmation is unnecessary
- when confirmation is required
- how many confirmations a logical flow may request
- when inline confirmation is preferred
- when modal confirmation is allowed

No execution behavior is implemented by this spec.

## Confirmation rules

Do not confirm:

- readonly actions
- low-risk derivations

Confirm only for:

- risk escalation
- `ARTIFACT` mutation
- external execution
- ambiguity
- irreversibility

At most one confirmation should be requested per logical flow.

Pending actions should be grouped when they belong to the same logical flow.

Inline confirmation is preferred.

Modal confirmation is reserved for:

- high risk
- external execution
- irreversible action

## Forbidden responsibilities

The flow-control policy must not:

- execute actions
- decide inside Slint
- call tools
- call LLM providers
- bypass human-in-the-loop policy
- reinterpret project manifests

## Future F10/F11 notes

F10 may connect these rules to a future pending-action model.

F11 should audit that friction control does not silently turn confirmation into implicit approval.

---

### !REL_FILE!

# graph_analysis_policy

## Status

PROPOSAL / F9/F10_PREP declarative governance.

This document does not implement runtime analysis, path computation, graph engines, inference engines, SPARQL, Oxigraph, execution logic, or lifecycle mutation.

## Purpose

Define the governed policy for future semantic graph analysis in DocGraph.

This policy is the owner of future graph-analysis rules over governed semantic graph material.

Analysis may consume only:

- `APPROVED` `QuadInstance`
- `APPROVED` `QuadRelation`

Analysis results are decision-support artifacts, not authoritative truth.

Associated graph-analysis schemas are declarative contracts only.

Schema validity does not enable analysis runtime, inference runtime, RDF projection, or execution.

## Analysis boundary

Graph analysis is future-only and declarative in this phase.

Rules:

- analysis consumes only approved semantic graph material
- analysis results are decision-support artifacts, not facts
- analysis does not modify graph, lifecycle, approval, `project_manifest`, or `StoredObject`
- analysis does not infer authoritative knowledge
- analysis does not execute actions
- RDF projection may be a future input, but RDF remains projection, not authority
- SPARQL and Oxigraph remain disabled
- no algorithms are implemented in this phase
- analysis does not invalidate or repair upstream layers

## Eligible analysis inputs

Future graph analysis may consume:

- approved semantic nodes represented by `QuadInstance`
- approved semantic edges represented by `QuadRelation`

Excluded inputs:

- `PROPOSED`
- `UNDER_REVIEW`
- `REJECTED`
- `STALE`
- `SUPERSEDED`
- `ORPHANED`

Lifecycle filtering remains governed by `semantic_quad_lifecycle.md`.

## Conceptual analysis profiles

Future analysis profiles may include:

- `RISK_PATH`
- `DEPENDENCY_PATH`
- `LOW_CONFIDENCE_PATH`
- `IMPACT_PATH`
- `STALE_PATH`

Interpretation:

- `RISK_PATH`: traces future governed risk propagation across approved semantic relations
- `DEPENDENCY_PATH`: traces approved semantic dependency chains
- `LOW_CONFIDENCE_PATH`: highlights weak-support or weak-quality approved semantic regions
- `IMPACT_PATH`: highlights future likely impact surfaces across approved dependencies
- `STALE_PATH`: highlights approved semantic material threatened by stale supporting context

These profile names are conceptual analysis classes only. They do not implement algorithms now.

## Conceptual node metrics

Future node-oriented analysis may use conceptual metrics such as:

- `confidence`
- `criticality`
- `risk`
- `freshness`
- `review_level`
- `source_quality`

These metrics are analysis inputs or derived analysis descriptors only.

They are not:

- semantic facts
- approval state
- execution authority

## Conceptual edge metrics

Future edge-oriented analysis may use conceptual metrics such as:

- `dependency_strength`
- `risk_propagation`
- `semantic_weight`
- `evidence_strength`

These metrics are analysis descriptors only.

They do not:

- mutate relation lifecycle
- approve relations
- infer authoritative truth

## Traceability requirements

Future graph analysis must preserve traceability to:

- source `QuadInstance`
- source `QuadRelation`
- source evidence references
- source trace metadata

Analysis outputs must remain explainable and attributable to the governed semantic material they consumed.

## Relationship to RDF projection

RDF projection may become a future transport or projection input for analysis.

However:

- RDF remains projection, not authority
- RDF is not required for analysis policy definition
- RDF does not replace the internal governed semantic model
- SPARQL and Oxigraph remain inactive

## Graph analysis invariants

- `INV-ANALYSIS-001`: analysis consumes only `APPROVED` `QuadInstance` records and `APPROVED` `QuadRelation` records.
- `INV-ANALYSIS-002`: analysis does not modify graph or lifecycle.
- `INV-ANALYSIS-003`: analysis results are not authoritative facts.
- `INV-ANALYSIS-004`: analysis must preserve traceability to source quads, relations, evidence, and traces.
- `INV-ANALYSIS-005`: analysis respects lifecycle filtering.
- `INV-ANALYSIS-006`: no inference engine is introduced.
- `INV-ANALYSIS-007`: no execution logic is introduced.
- `INV-ANALYSIS-008`: RDF, SPARQL, and Oxigraph are not required or activated by this policy.
- `INV-ANALYSIS-009`: analysis cannot approve, reject, supersede, stale, or orphan semantic material.
- `INV-ANALYSIS-010`: analysis cannot trigger `ActionResolution` or tool execution.
- `INV-ANALYSIS-011`: analysis cannot invalidate, repair, or mutate storage, derivatives, semantic, or graph layers.

## Related specs

- `docs/specs/artifact_graph.md`
- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/rdf_projection_policy.md`

---

### !REL_FILE!

# gui_objects_v1

## Status

PROPOSAL / F9.5 declarative governance. This spec defines canonical GUI object names for contextual help. It does not implement runtime behavior, LLM calls, tool execution, action creation, document mutation, or persistent help history.

## Purpose

Fix a stable GUI vocabulary so Lume can explain DocGraph without ambiguous spatial language.

Without stable names, contextual help cannot be precise.

## Declarative source

Canonical objects are declared in:

- `resources/help/gui_objects.json`

GUI objects may have stable technical ids for references and localized or canonical display names for presentation.

The help tree must reference technical ids, not localized labels.

Lume Help may reference this glossary through:

- `resources/help/lume_help.json`

## Canonical objects

- `Document Tree`: navigation of existing governed project objects exposed by `project_manifest`. It is not a free filesystem explorer, import surface, or execution surface.
- `Document Tree` prepares governed selection context by family. It does not collapse all selections into one global active object.
- `Clip Panel`: external intake and workflow launch. It is not project storage.
- `Workspace Tabs`: container for active controlled views. It is not a notebook, free docking system, or miniapp surface.
- `Workspace Tabs` reuses one governed tab per governed target reference. It is not a duplicate-tab surface or execution surface.
- `Readonly Viewer`: non-editable governed document viewing. It must be labeled `READONLY`. It is not an editor, patch runtime, or proposal applier.
- `Chat Panel`: coordination, context, and structured references. It is not document storage or an execution engine.
- `Tools Panel`: access to controlled tools. `Operational Tools` are controlled manual launch; `LLM Tools` are declarative policy in F9.5, not execution.
- `Knowledge Panel`: access to knowledge documents. It is not a semantic graph.
- `Pipeline View`: readonly/mock representation of the F9.5 flow `Document -> Chunk -> PromptSpec -> SemanticSpec -> Proposal -> Trace`. It is not real execution.
- `Ontology Proposal View`: representation of semantic proposals. It is proposal, not fact; it is not approval or semantic persistence.
- `Help Menu`: top-level informational menu domain for `Open Lume Help`, `Help Topics`, and `About DocGraph`. It is not a tool surface, settings surface, credential surface, or execution surface.
- `Lume Help`: declarative contextual help. It is not runtime, persistent chat, active LLM, tool runner, or action approver.
- `Help Topics`: declarative hierarchical help navigation. It is not dynamic runtime logic, diagnostics, or execution.
- `About Popup`: product information, version, credits, and license. It is not system access or action surface.
- `Status Bar`: prepared capability and system state summary. It is not an execution surface or control panel.
- `DocGraph Icon`: branding icon in top app chrome. It may open `Lume Help` as navigation intent. It is not an execution control, LLM activator, settings control, or tool runner.
- `Sandbox View`: conditional workspace tab for governed sandbox status and context. It is declarative in F9 and is not a filesystem executor, write-back control, or tool runner.
- `Sandbox node`: governed tree reference to sandbox context. It is not a host-folder browser or mutation control.

The app shell may also expose a minimal top main menu for `File`, `Preferences`, `Tools`, and `Help`. That menu is navigation chrome, not a new functional panel.

## Lume GUI rules

- `RULE-LUME-GUI-001`: Lume must use names defined in `GUI_OBJECTS_v1`.
- `RULE-LUME-GUI-002`: explanations should follow `[GUI Object] + [action] + [state]`.
- `RULE-LUME-GUI-003`: Lume must not introduce undefined alternative names.
- `RULE-LUME-GUI-004`: Lume should adapt the explanation to the active GUI object when context exists.
- `RULE-LUME-GUI-005`: every object should be explainable through what it is, what it allows, what it does not allow, and current state.
- `RULE-LUME-GUI-006`: when a surface is declarative/mock in F9.5, Lume must state no execution, no mutation, no automatic approval, and no semantic runtime.
- `RULE-LUME-GUI-007`: Lume should avoid vague spatial expressions such as "this", "here", "above", or "on the left" unless the canonical GUI object is named.
- `RULE-LUME-GUI-008`: Lume may provide brief definitions and suggestions, but it must not execute actions.

## UI boundary

Lume Help may show a readonly "GUI Objects" / "Objetos de la interfaz" section.

The UI may present canonical names and short descriptions, but must not infer context, create actions, call LLMs, execute tools, mutate documents, or persist help history.

Visible strings should come from i18n/resources, not hardcoded Slint text.

## Related specs

- `docs/specs/lume_help_popup.md`
- `docs/specs/help_menu.md`
- `docs/specs/lume_interaction_model.md`
- `docs/specs/ui_core.md`

---

### !REL_FILE!

# help_menu

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Help` menu domain for DocGraph.

`Help` is dedicated to:

- user guidance
- documentation navigation
- contextual help
- product information

## Core rule

`Help` is informational, navigational, and assistive.

It is not operational.
It is not executable.
It is not diagnostic.
It is not configurational.

## Menu model

`Help` is a top-level menu entry.

It exposes exactly:

- `Open Lume Help`
- `Help Topics`
- `About DocGraph`

Each entry opens a dedicated popup surface.

## Entry contracts

### Open Lume Help

Opens:

- `LumeHelpPopup`

Purpose:

- contextual assistant
- feature explanation
- navigation guidance

### Help Topics

Opens:

- `LumeHelpTree`

Purpose:

- categorized help topics
- hierarchical navigation
- domain-based grouping

### About DocGraph

Opens:

- `AboutPopup`

Purpose:

- product information
- credits
- version and license display

## Domain rules

`Help`:

- emits intent only
- does not execute
- does not resolve policy
- does not mutate runtime
- does not interact with tools
- does not expose credentials
- does not trigger LLM execution

## UX principles

`Help` must remain:

- easy to find
- visible
- predictable
- consistent
- self-service oriented

The menu should use clear labels and minimal entries.

## Governed i18n keys

The `Help` menu contract uses these stable menu keys:

- `menu.help`
- `menu.help.lume`
- `menu.help.topics`
- `menu.help.about`

The governed Help surfaces use these popup keys:

- `popup.help.lume.title`
- `popup.help.lume.description`
- `popup.help.topics.title`
- `popup.help.topics.description`
- `popup.help.about.title`
- `popup.help.about.description`

Popup descriptions must remain informational only.

They must not imply diagnostics, tool execution, or LLM execution.

## Invariants

- `INV-HELP-001`: `Help` MUST be a top-level menu domain.
- `INV-HELP-002`: `Help` MUST NOT execute actions.
- `INV-HELP-003`: `Help` MUST NOT invoke tools.
- `INV-HELP-004`: `Help` MUST NOT invoke LLM execution.
- `INV-HELP-005`: `Help` MUST NOT access credentials.
- `INV-HELP-006`: `Help` MUST NOT mutate runtime.
- `INV-HELP-007`: `Help` MUST contain only informational surfaces.
- `INV-HELP-008`: each `Help` entry MUST open a dedicated popup.
- `INV-HELP-009`: visible Help labels and governed popup text MUST come from i18n resources.

## Non-goals

Do not implement:

- runtime help assistant
- chatbot execution
- diagnostics
- telemetry
- remote help services
- auto-search
- auto-learning systems

## Related specs

- `docs/specs/app_main_menu.md`
- `docs/specs/lume_help_popup.md`
- `docs/specs/lume_help_tree.md`
- `docs/specs/gui_objects_v1.md`

---

### !REL_FILE!

# how_to_add_a_tool

## Status

PROCEDURAL GOVERNANCE GUIDE.

SPEC-ONLY.

No runtime implementation.

## Purpose

Provide a reproducible, low-friction process for adding a new tool in DocGraph without opening F10, bypassing `ActionResolution`, or mixing declaration, visibility, permission, and execution.

Core rule:

`declared != executable`

Tool state progression:

`DECLARED -> VISIBLE -> PERMITTED -> IMPLEMENTED -> AVAILABLE -> EXECUTABLE`

Do not collapse these states.

## Before you start

Read first:

- `docs/specs/tool_implementation_governance.md`
- `docs/specs/tools_catalogs.md`
- `docs/specs/tools_panel.md`
- `docs/specs/action_resolution.md`

This guide does not replace those specs.

## 1. Tool classification

Choose exactly one `tool_class`:

- `operational`
- `llm`
- `external_dependency`
- `base_utility`

### Decision tree

1. Does it perform a DocGraph technical action over governed inputs or outputs?
   - yes -> `operational`
   - no -> continue
2. Is it a future LLM-facing capability surface rather than a manual user tool?
   - yes -> `llm`
   - no -> continue
3. Is it a binary, package, or external software required by a tool?
   - yes -> `external_dependency`
   - no -> stop and reclassify before editing anything

### Classification notes

- `operational` tools are internal DocGraph tools.
- `llm` tools are not manually executable in F9.
- `external_dependency` entries are not tools by themselves.
- `base_utility` entries are internal support capabilities and are not user-facing.

## 2. Phase 1 - Declaration

Choose the correct declaration location:

- `resources/tools/internal/operational/`
- `resources/tools/internal/llm/`
- `resources/tools/external/`

Checklist:

- [ ] `tool_id` defined
- [ ] `capability_id` defined
- [ ] description complete
- [ ] `status = declared_only`
- [ ] `execution_enabled = false`
- [ ] no implementation binding
- [ ] correct `tool_class`
- [ ] correct catalog reference
- [ ] visible text uses declarative i18n fields

Required reminder:

This does NOT make the tool executable.

## 3. Phase 2 - Surface / visibility

After declaration, place the tool in the correct governed surface:

- `Operational Tools`
- `LLM Tools`
- `External Dependencies`

Visibility is not availability.

A visible tool may still be non-implemented, non-permitted, or non-executable.

Checklist:

- [ ] appears in the correct panel
- [ ] surface matches `tool_class`
- [ ] no execution button unless explicitly governed by current phase
- [ ] no UI-triggered execution
- [ ] no menu click implies execution

## 4. Phase 3 - Policy / permission

Policy and visibility are separate concerns.

Future effective state is conceptually derived by `EffectiveToolSurfaceResolver`.

That resolver is not opened as runtime authority in F9.

Checklist:

- [ ] tool may be visible but not usable
- [ ] visibility is not permission
- [ ] permission is not execution
- [ ] no direct execution authority is introduced
- [ ] no UI surface decides permission
- [ ] no LLM surface decides permission

## 5. Phase 4 - Minimal runtime (only if applicable)

Touch `tool_runtime` only if the current phase explicitly permits a minimal governed slice.

Most tools in F9 do NOT reach this phase.

If a minimal slice is allowed, keep it minimal.

Checklist:

- [ ] behavior is deterministic
- [ ] no hidden side effects
- [ ] no hidden IO
- [ ] no external calls unless already declared and phase-permitted
- [ ] no runtime widening outside the minimal slice
- [ ] no F10 drift

## 6. Phase 5 - Output governance

If the tool persists outputs, govern the output first.

Required concepts:

- `owner_ref`
- `result.json`
- `tool_run_manifest.json`

Checklist:

- [ ] outputs stored under `user/output/tool_runs/`
- [ ] `owner_ref` present
- [ ] `tool_run_manifest.json` present
- [ ] `result.json` present for completed runs
- [ ] no orphan outputs
- [ ] no root `outputs/`

## 7. Phase 6 - UI interaction

UI prepares intent only.

UI does not execute.

Use popup-driven preparation when governed.

Checklist:

- [ ] UI does NOT execute
- [ ] UI captures intent only
- [ ] popup includes inputs
- [ ] popup includes config
- [ ] popup includes `owner_ref`
- [ ] popup includes destination
- [ ] popup does not bypass `ActionResolution`

## 8. Phase 7 - LLM integration (if applicable)

LLM tools are not manually executable.

The LLM does not receive the full raw tool catalog by default.

Checklist:

- [ ] tool appears in the LLM surface only if declared
- [ ] no bypass of policy
- [ ] no automatic execution
- [ ] no manual `Run`
- [ ] no raw full catalog injection by default

## 9. Phase 8 - External dependencies (if applicable)

Dependencies are not tools.

Checklist:

- [ ] declared in the external catalog
- [ ] stored or referenced under `user/runtime/external_dependencies/` when applicable
- [ ] no auto install
- [ ] no `PATH` mutation
- [ ] validation does not imply execution
- [ ] dependency presence does not imply tool availability

## 10. Phase 9 - Audits

Required validations:

- `dev\scripts\validate_f9_declarations.bat`
- `dev\scripts\validate_ai_specs.bat`

Optional manual-only audit:

- `dev\scripts\audits\audit_f9_boundary_drift.bat`
- `dev\scripts\audits\audit_tools_compliance.bat`

Checklist:

- [ ] required validations pass
- [ ] optional boundary audit reviewed manually when relevant

## 11. Anti-patterns

Do not:

- must not let UI execute tools
- let LLM trigger execution
- hardcode paths
- place tools in `Preferences`
- store external binaries in `resources/`
- skip `owner_ref`
- skip `tool_run_manifest.json`
- mix declaration, permission, and execution into one field
- mix tools and dependencies
- bypass `ActionResolution`

## 12. Quick checklist

- [ ] declared
- [ ] not executable
- [ ] correct catalog
- [ ] correct surface
- [ ] no UI execution
- [ ] no LLM execution
- [ ] no runtime drift
- [ ] audits pass

## 13. Invariants

- declared != executable
- UI != executor
- LLM != executor
- tools != dependencies
- execution only via governed path

## Related specs

- `docs/specs/tool_implementation_governance.md`
- `docs/specs/tools_catalogs.md`
- `docs/specs/tools_panel.md`
- `docs/specs/action_resolution.md`

---

### !REL_FILE!

# i18n_audit_consistency

## Status

PROCEDURAL / ADVISORY ONLY.

No runtime implementation.

## Purpose

Define the scope and non-authoritative behavior of the i18n consistency audit.

The audit exists to surface likely i18n contract drift early without modifying resources, specs, runtime, or UI behavior.

## Audit scope

The advisory audit may inspect:

- `resources/i18n/en/*.ftl`
- `resources/i18n/es/*.ftl`
- `docs/specs/*`

The audit may report:

- keys present in English but missing in Spanish
- keys present in Spanish but missing in English
- keys that appear unreferenced in specs
- best-effort heuristics for visible UI text that looks hardcoded in specs

## Non-authority

The audit is advisory only.

It must not:

- modify any file
- auto-fix keys
- rewrite specs
- classify runtime authority
- be treated as proof of correctness

It is a review aid, not a source of truth.

## Interpretation rules

- missing-key findings are actionable consistency signals
- unreferenced-key findings are review signals, not automatic deletion candidates
- hardcoded-text findings are heuristic review signals only
- false positives are expected and require human review

## Codex usage

This audit must not be used as automatic patch input.

Human review is required before any change is proposed from its findings.

---

### !REL_FILE!

# identity_system

## Purpose

Define the unified identity system for governed DocGraph entities.

This document is declarative only.

It does not implement runtime generation, persistence, execution, or validation engines.

## Identity types

The governed identity types are:

- `file_ref`
- `object_ref`
- `quad_id`
- `quad_instance_id`
- `relation_id`
- `intent_id`
- `trace_ref`

## Namespaces

Controlled prefixes:

- `file_ref` -> `sha256_`
- `object_ref` -> `obj_`
- `quad_id` -> `q_`
- `quad_instance_id` -> `qi_`
- `relation_id` -> `rel_`
- `intent_id` -> `intent_`
- `trace_ref` -> `trace_`

These namespaces must remain distinct.

No identifier from one namespace may be reused as if it belonged to another namespace.

## Identity roles

### file_ref

- content-addressed identity
- deterministic from content bytes
- independent from filename
- independent from host path

Example:

- `sha256_<hex>`

### object_ref

- logical object identity
- stable logical reference for `StoredObject`
- not derived from UI state
- not derived from portable path text alone

Example:

- `obj_<stable_suffix>`

### quad_id

- deterministic semantic identity
- derived from normalized semantic content and evidence anchor
- represents what is asserted
- does not include generation-instance context

Example:

- `q_<stable_suffix>`

### quad_instance_id

- generation-instance identity
- represents a specific produced instance of a semantic assertion
- may vary across different generation contexts for the same `quad_id`

Example:

- `qi_<instance_suffix>`

### relation_id

- stable relation-edge identity
- identifies an explicit governed relation object
- remains separate from `quad_id` and `quad_instance_id`

Example:

- `rel_<stable_suffix>`

### intent_id

- governed action-intent instance identity
- identifies a declarative `ActionIntent`
- instance-based rather than execution-authorizing

Example:

- `intent_<instance_suffix>`

### trace_ref

- governed observability identity
- links related artifacts across one logical flow
- remains reference-only and non-executing

Example:

- `trace_<stable_or_instance_suffix>`

## Determinism and instance rules

- `file_ref` must be deterministic
- `quad_id` must be deterministic
- `quad_instance_id` is instance-based
- `relation_id` is stable edge identity and must not collide with quad namespaces
- `object_ref` must remain stable for the governed logical object it identifies
- `intent_id` is instance-based and must not imply authorization
- `trace_ref` is observability identity and must not imply authorization

## Collision rules

- no collisions across namespaces
- prefix is part of governed identity interpretation
- a value valid for one identity type must not be interpreted as another identity type

## Boundary rules

- identity must not depend on file path
- identity must not depend on UI
- identity must not depend on translated labels
- identity must not become execution authority
- identity stability must not be broken by presentation changes
- `intent_id` must not become execution authority
- `trace_ref` must not become execution authority

## Invariants

- `INV-ID-SYS-001`: identity must be stable for the entity type it governs
- `INV-ID-SYS-002`: identity must not depend on file path
- `INV-ID-SYS-003`: identity must not depend on UI
- `INV-ID-SYS-004`: deterministic identities must remain deterministic where required
- `INV-ID-SYS-005`: instance identities must remain distinct from deterministic semantic identities
- `INV-ID-SYS-006`: namespaces must not collide
- `INV-ID-SYS-007`: identity must not imply execution authority
- `INV-ID-SYS-008`: `intent_id` and `trace_ref` are governed namespaces and must remain distinct from storage and semantic IDs
- `INV-ID-SYS-009`: `intent_id` and `trace_ref` do not imply authorization, approval, or execution

## Related specs

- `docs/specs/storage_policy.md`
- `docs/specs/semantic_quad_model.md`
- `docs/specs/action_contract.md`
- `docs/specs/system_observability.md`

---

### !REL_FILE!

# invalidation_policy

## Purpose

Define governed invalidation propagation rules across storage-backed evidence, derivatives, semantic quads, semantic relations, and graph material.

This document is declarative only.

It does not implement runtime invalidation engines, execution logic, mutation logic, or F10 execution slices.

## Scope

This policy governs invalidation triggered by:

- chunk changes
- file changes
- metadata changes

## Core triggers

### Chunk triggers

- `chunk_hash` change -> downstream semantic quad instances become `STALE`
- missing `chunk_id` -> downstream semantic quad instances become `ORPHANED`

### File triggers

- `file_ref` or effective source-content change -> derivative evidence may become stale
- missing governed source reference -> downstream semantic quad instances become `ORPHANED`

### Metadata triggers

- `metadata_snapshot_hash` mismatch -> downstream semantic quad instances become `STALE`
- missing required governed metadata evidence -> downstream semantic quad instances become `ORPHANED` or `STALE`, depending on whether identity of evidence is lost or only freshness is lost

## Propagation chain

Invalidation propagates downstream only:

```text
chunk -> quad -> relation -> graph
```

Expanded interpretation:

- chunk or source evidence change affects quad instances first
- quad invalidation affects dependent relations
- relation invalidation affects approved graph consumption

No reverse mutation is allowed.

Graph or relation invalidation must not mutate quads, derivatives, or storage.

## Invalidation modes

### Lazy invalidation

Current governed mode:

- invalidation is recorded conceptually when evidence mismatch is detected
- semantic and relation lifecycle reacts when evaluated or rechecked
- no eager runtime propagation engine is opened by this policy

### Eager invalidation

Future mode only:

- downstream invalidation may be propagated immediately by a future governed runtime
- eager propagation remains future and separately gated

## Layer effects

### Quad effects

- evidence mismatch affecting a quad instance changes lifecycle to `STALE`
- missing evidence anchor changes lifecycle to `ORPHANED`
- approved quads must not remain silently valid when evidence breaks

### Relation effects

- relation lifecycle must react to invalid source or target quad instances
- if a supporting quad becomes `STALE`, dependent relations may become `STALE`
- if a supporting quad becomes `ORPHANED`, dependent relations may become `ORPHANED` or `STALE`

### Graph effects

- invalid semantic relations must not remain approved graph dependencies
- graph consumption must exclude invalidated semantic material
- graph reacts; graph does not repair upstream layers

## Traceability

Every governed invalidation event must remain traceable at the policy level through:

- source evidence reference
- affected downstream entity reference
- invalidation reason
- invalidation mode
- trace reference

This policy does not define a runtime event format, only the requirement that invalidation be traceable.

## Invariants

- `INV-INVALIDATION-001`: approved quads must not remain valid if evidence breaks
- `INV-INVALIDATION-002`: invalidation must be traceable
- `INV-INVALIDATION-003`: invalidation propagates downstream only
- `INV-INVALIDATION-004`: `chunk_hash` change leads to `STALE`
- `INV-INVALIDATION-005`: missing chunk evidence leads to `ORPHANED`
- `INV-INVALIDATION-006`: graph and relations react to upstream invalidation and must not mutate upstream layers
- `INV-INVALIDATION-007`: lazy invalidation is the current declared mode
- `INV-INVALIDATION-008`: eager invalidation is future-only and not active

## Related specs

- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/artifact_graph.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/semantic_layer_boundaries.md`

---

### !REL_FILE!

# knowledge_panel

## Purpose

Define the minimum UI-facing contract for the F8 knowledge panel without introducing notebook behavior, semantic processing, or LLM workflow.

## Unit of knowledge

A knowledge unit in F8 is a readonly project-owned document exposed from `knowledge/` under the project root.

It is represented minimally by:

- `document_id`
- `display_name`
- `logical_path`
- `is_viewable`
- `is_selected`

## F8 behavior

The knowledge panel may:

- list visible knowledge units
- show an empty, ready, or error state
- select one unit at a time
- route the selected unit into the existing readonly viewer tab inside the workspace content area

## F8 data flow

`project_root/knowledge/*` -> small technical listing boundary -> `KnowledgeController` -> `KnowledgePanelState` -> workspace tab selection -> `ViewerController`

The viewer remains readonly, reuses existing technical display behavior, and is treated as one concrete workspace tab rather than as the whole main content area.

The knowledge panel itself is not the workspace container. It is one surface that can select a knowledge unit and open that unit into the viewer tab model already defined by the shell.

Knowledge selection stays distinct from:

- tree-based reference of existing workspace documents
- clip-based intake of external files
- chat-level structured references or tool results

## Out of scope for F8

- notebook or blocks
- graph or edge modeling
- semantic scoring or tagging
- LLM workflows
- automatic promotion from resources to knowledge
- editing, saving, or annotations
- mixing knowledge with tool execution or project pipeline ownership
- introducing notebook, blocks, or complex workspace layout behavior

---

### !REL_FILE!

# llm_agent_prompts

## Status

PROPOSAL / F10_PREP declarative governance.

No runtime implementation.

No prompt files are created by this spec.

## Purpose

Define Lume as the single governed front-facing agent and define declarative storage, format, versioning, and governance rules for all future LLM agent prompts.

This spec is storage and contract only.

It does not implement prompt loading, provider calls, tool execution, prompt editing UI, hidden memory, or ActionResolution execution.

## Canonical agent flow

All future user-facing LLM interaction follows this conceptual flow:

```text
User
<-> Lume
-> Planner
-> Specialists
-> Synthesizer
-> Lume
-> User
```

Rules:

- all user interaction enters through Lume
- all user-facing output exits through Lume
- internal agents are not user-facing
- internal agents are roles, not execution authorities
- internal roles may prepare reasoning artifacts only in a future governed runtime
- no role may execute tools, mutate files, call providers directly, approve actions, expose credentials, or bypass ActionResolution

## Lume front-facing role

Lume is the only governed front-facing assistant identity.

Lume acts as:

- intent detection agent
- guidance/help agent
- orchestration entry and exit point
- capability explainer
- tool-aware but non-executing interface

Lume must not:

- execute tools
- mutate files
- approve actions
- expose credentials
- bypass ActionResolution
- call providers directly
- create hidden persistence
- create or delegate to another front-facing assistant identity

No alternative front-facing assistant may be introduced without a governed proposal, phase decision, i18n/resource review, and explicit update to Lume identity governance.

## Internal agent roles

Internal agents are conceptual roles only.

They are not UI identities, user-facing assistants, runtime authorities, or autonomous workers.

### Planner

Planner performs future conceptual decomposition and routing.

Responsibilities:

- analyze user intent received through Lume
- decompose requests into subrequests
- identify needed context
- route subrequests to scoped specialist roles
- mark tool or action candidates as proposals only

Planner must not execute tools, call providers directly, mutate files, approve actions, or communicate directly with the user.

### Specialists

Specialists perform scoped domain reasoning.

Initial conceptual specialist roles:

- `engineering`
- `mathematics`
- `software`
- `legal`
- `medical`
- `financial`
- `documentation`
- `research`
- `general_reasoning`

Specialists may produce scoped partial reasoning artifacts in a future governed runtime.

Specialist output is not user-facing until integrated by Synthesizer and presented by Lume.

Specialists must not execute tools, call providers directly, mutate files, approve actions, or communicate directly with the user.

Domain-sensitive specialists such as legal, medical, and financial must remain informational and must not present professional advice as authority.

### Synthesizer

Synthesizer integrates partial results.

Responsibilities:

- preserve dependency order
- expose blocked, uncertain, or failed subresults
- avoid contradictions between partial results
- produce one coherent `FinalAnswer` candidate for Lume

Synthesizer must not execute tools, call providers directly, mutate files, approve actions, or communicate directly with the user.

## Declarative prompt storage model

All future agent prompts live under:

```text
resources/llm/agents/
  lume/
    lume_agent.prompt.json
  planner/
    planner_agent.prompt.json
  synthesizer/
    synthesizer_agent.prompt.json
  specialists/
    engineering.prompt.json
    mathematics.prompt.json
    software.prompt.json
    legal.prompt.json
    medical.prompt.json
    financial.prompt.json
    documentation.prompt.json
    research.prompt.json
    general_reasoning.prompt.json
```

This is a declarative storage contract only.

The presence of a prompt file must not imply provider invocation, agent execution, tool execution, prompt activation, UI availability, or F10 opening.

Prompt files must be environment-independent and portable.

Runtime code must not embed prompt text.

UI must not define, edit, or mutate prompt files.

## Prompt format

Each prompt resource must declare:

- `agent_id`
- `version`
- `role`
- `system_prompt`
- `constraints`
- `input_contract`
- `output_contract`
- `allowed_context_refs`
- `forbidden_context`
- `capabilities`
- `tool_surface_policy`
- `privacy_constraints`

Rules:

- prompts are declarative, versioned, and non-executable
- prompts contain no credentials, secrets, tokens, passwords, private keys, or provider material
- prompts do not hardcode tool availability
- prompts reference tools only through `EffectiveToolSurfaceSummary`
- prompts must not include raw full tool catalogs by default
- prompts are environment-independent
- prompts must not contain host-specific absolute paths
- prompts must not grant execution authority
- prompts must not override governance, policy, ActionResolution, credentials policy, tool policy, or project_runtime boundaries

## Prompt versioning and traceability

Prompt resources must be versioned.

Future traces should record:

- `agent_id`
- prompt `version`
- prompt fingerprint or checksum
- prompt resource path
- effective LLM mode
- capability state reference
- tool surface summary reference, when used

Traceability does not imply provider invocation or execution authority.

## Conceptual prompt loading

In a future governed phase, `llm_core` may load prompt resources from `resources/llm/agents/`.

Loading prompts:

- does not imply provider invocation
- does not imply agent execution
- does not imply tool execution
- does not imply ActionResolution execution
- does not expose credentials
- does not mutate catalogs
- does not create hidden memory

Credentials must never enter prompts.

## Tool awareness

Lume and internal roles may receive `EffectiveToolSurfaceSummary`.

Rules:

- no agent receives raw full catalogs by default
- agents may propose tool usage
- agents must not execute tools
- tool intent becomes proposal only
- future execution requires governed ActionResolution before any execution path

## OFF / LOCAL / CLOUD behavior

### OFF

When effective LLM mode is `OFF`:

- Lume uses guided menus and static help
- no LLM calls occur
- internal agent roles are inactive or explanatory only
- prompt loading is not required
- no provider, local model, tool, or runtime action is invoked

### LOCAL

When effective LLM mode is `LOCAL`:

- assisted reasoning may be available only if capability state allows
- internal roles may be used only in a future governed runtime
- local model use remains non-executing with respect to tools/actions
- credentials are not required for local mode

### CLOUD

When effective LLM mode is `CLOUD`:

- assisted reasoning may be available only if capability state allows
- internal roles may be used only in a future governed runtime
- provider configuration and non-secret `credential_ref` are required
- credential values must never enter prompts or LLM context

## Invariants

- `INV-AGENT-001`: Lume is the single governed front-facing agent.
- `INV-AGENT-002`: no alternative front-facing assistant may be introduced without governed proposal and phase decision.
- `INV-AGENT-003`: all user interactions enter and exit through Lume.
- `INV-AGENT-004`: internal agents do not directly interact with the user.
- `INV-AGENT-005`: internal agents are roles, not authorities.
- `INV-AGENT-006`: all agent prompts live under `resources/llm/agents/`.
- `INV-AGENT-007`: prompts are declarative, versioned, and non-executable.
- `INV-AGENT-008`: prompts contain no credentials or secrets.
- `INV-AGENT-009`: prompts do not hardcode tool availability.
- `INV-AGENT-010`: runtime must not embed prompt text in code.
- `INV-AGENT-011`: UI must not define or modify prompts.
- `INV-AGENT-012`: prompts are loadable independently of runtime execution.
- `INV-AGENT-013`: `EffectiveToolSurface` governs tool awareness.
- `INV-AGENT-014`: agents may propose but not execute tools or actions.
- `INV-AGENT-015`: prompt loading must be deterministic, traceable, and testable without provider calls.

## Non-goals

- no runtime implementation
- no provider calls
- no tool execution
- no ActionResolution runner
- no UI authority
- no prompt editing UI
- no catalog mutation
- no credential handling
- no hidden memory
- no prompt resource creation in this phase
- no new front-facing agent

## Related specs

- `docs/specs/lume_interaction_model.md`
- `docs/specs/lume_help_popup.md`
- `docs/specs/lume_help_tree.md`
- `docs/specs/chat_document_flows.md`
- `docs/specs/llm_core.md`
- `docs/specs/llm_tool_surface_resolution.md`

---

### !REL_FILE!

# llm_core

## Purpose

Define the inherited LLM contract and policy layer for the Rust successor.

## Responsibilities

- define governed LLM modes: `OFF`, `LOCAL`, `CLOUD`
- define conceptual capability state and effective mode resolution
- define guided fallback behavior when effective LLM mode is `OFF`
- resolve deterministic governed plans
- apply configuration precedence and policy
- govern model catalog validation
- define tool request policy
- define required observability payloads

## Inputs

- explicit runtime arguments
- user preferences when available
- conceptual user profile and consent references, when available
- system LLM configuration
- model catalog
- available tool specifications
- governed user input
- governed context references

## Outputs

- deterministic governed plans
- validated provider and model selections
- effective tool policy
- structured observability metadata requirements
- conceptual request-analysis artifacts
- conceptual response-synthesis artifacts

## Allowed Dependencies

- standard library
- `core_domain`
- `spec_runtime`

## Forbidden Responsibilities

- no direct provider SDK integration
- no direct tool execution
- no UI logic
- no Slint dependency
- no filesystem ownership outside governed config loading

## Initial Phase Scope

- policy and contract definitions
- LLM mode and model catalog rules
- capability-state and guided fallback contracts
- agent role and prompt resource governance
- tool request governance
- observability requirements
- conceptual input-analysis and response-synthesis pipeline
- no real provider invocation yet

## F10 step-1 opening objective

The first explicit F10 opening gate is to allow a minimum governed LLM-assisted runtime slice without opening tool execution or `ActionResolution` execution.

This step may open runtime concern for:

- capability-state resolution
- effective-mode resolution
- governed context filtering
- declarative prompt loading
- assisted-request preparation
- guided fallback behavior

This step does not require provider invocation.

Provider binding remains optional and phase-gated.

## Governed LLM modes

The only governed LLM modes are:

- `OFF`
- `LOCAL`
- `CLOUD`

No `auto` mode is authorized by this spec.

### OFF

`OFF` means:

- no LLM generation
- no provider calls
- no local model usage
- deterministic guided menus may be used
- DocGraph remains usable

`OFF` is not an error state.

### LOCAL

`LOCAL` means local backend intent only.

It requires:

- local engine capability
- local model capability
- policy allowing local LLM use

It does not require cloud credentials.

Availability is derived from `LLMCapabilityState`, not from UI state.

### CLOUD

`CLOUD` means cloud backend intent only.

It requires:

- provider configuration
- non-secret `credential_ref`
- policy allowing cloud LLM use

Credential values must never be exposed to LLM context, logs, status surfaces, prompts, tool manifests, or UI state.

Availability is derived from `LLMCapabilityState`, not from UI state.

## Interaction modes

The governed interaction modes are:

- `GUIDED`
- `ASSISTED`

`GUIDED` works without LLM capability.

`ASSISTED` requires an available effective LLM capability.

Interaction mode does not imply tool execution, file mutation, provider calls, or action authority.

## LLMCapabilityState

Before any future Lume interaction, the system must conceptually resolve:

`desired_llm_mode -> LLMCapabilityState -> effective_llm_mode -> interaction_mode`

Conceptual fields:

- `desired_llm_mode`: `OFF | LOCAL | CLOUD`
- `effective_llm_mode`: `OFF | LOCAL | CLOUD`
- `interaction_mode`: `GUIDED | ASSISTED`
- `llm_available`: bool
- `provider_configured`: bool
- `credential_ref_present`: bool
- `local_engine_available`: bool
- `local_model_available`: bool
- `policy_allows_local`: bool
- `policy_allows_cloud`: bool
- `tool_surface_available`: bool
- `execution_enabled`: bool
- `reason_codes`: string[]

Rules:

- all capability states must be explicit
- missing capability must not fail silently
- credentials and secret values must never be exposed
- `desired_llm_mode` is not the same as `effective_llm_mode`
- `credential_ref` is an identifier, not execution permission
- `execution_enabled=false` remains the current governed default

Required reason codes include:

- `llm_off_by_preference`
- `local_engine_missing`
- `local_model_missing`
- `local_policy_blocked`
- `cloud_provider_missing`
- `cloud_credential_ref_missing`
- `cloud_policy_blocked`
- `execution_not_open`
- `assisted_mode_unavailable`
- `guided_fallback_active`

## Effective mode resolution

Resolution is conceptual only. It does not implement runtime logic.

Rules:

- if `desired_llm_mode=OFF`, then `effective_llm_mode=OFF`, `interaction_mode=GUIDED`, and `reason_codes` includes `llm_off_by_preference`
- if `LOCAL` is requested but local engine is missing, local model is missing, or policy blocks local mode, then `effective_llm_mode=OFF`, `interaction_mode=GUIDED`, and `reason_codes` must explain the missing or blocked capability
- if `CLOUD` is requested but provider configuration is missing, `credential_ref` is missing, or policy blocks cloud mode, then `effective_llm_mode=OFF`, `interaction_mode=GUIDED`, and `reason_codes` must explain the missing or blocked capability
- if local engine exists, local model exists, and policy allows local mode, then `effective_llm_mode=LOCAL`, `interaction_mode=ASSISTED`, and `llm_available=true`
- if provider configuration exists, `credential_ref` is present, and policy allows cloud mode, then `effective_llm_mode=CLOUD`, `interaction_mode=ASSISTED`, and `llm_available=true`
- no mode grants tool execution authority
- no mode bypasses `ActionResolution`

When future execution remains closed, `execution_enabled=false` and `reason_codes` must include `execution_not_open` when relevant.

## GuidedMenuEngine contract

`GuidedMenuEngine` is a conceptual deterministic menu contract for `GUIDED` interaction.

It is not an LLM, runtime, executor, provider adapter, or UI authority.

Conceptual structures:

- `GuidedMenu`
- `GuidedMenuOption`
- `GuidedMenuSelection`
- `GuidedMenuResult`

Allowed menu intents:

- `OpenProjectIntent`
- `OpenPreferencesIntent`
- `OpenProjectSettingsIntent`
- `OpenToolsPanelIntent`
- `OpenLumeHelpIntent`
- `ShowCapabilityStateIntent`
- `ExplainMissingCapabilityIntent`
- `ConfigureLocalLlmIntent`
- `ConfigureCloudLlmIntent`

Forbidden menu intents:

- `ExecuteToolIntent`
- `MutateFileIntent`
- `MutateDocumentIntent`
- `CallLlmProviderIntent`
- `ValidateCredentialSecretIntent`
- `RunExternalBinaryIntent`
- `ApproveActionIntent`
- `ExecuteActionResolutionIntent`

OFF-mode menu example:

1. Open project
2. Review documents
3. Open Tools Panel
4. Configure local LLM
5. Configure cloud LLM
6. Show capability state
7. Open Lume Help

The example is informational only. It does not define UI implementation or runtime behavior.

## Agent role model

Lume is the single governed front-facing assistant identity.

Future assisted reasoning may use internal conceptual roles:

- Planner
- Specialists
- Synthesizer

Canonical flow:

```text
User
<-> Lume
-> Planner
-> Specialists
-> Synthesizer
-> Lume
-> User
```

Rules:

- all user interactions enter and exit through Lume
- internal roles do not directly interact with the user
- internal roles are roles, not authorities
- internal roles may propose tool or action candidates only as proposals
- internal roles must not execute tools, mutate files, approve actions, expose credentials, call providers directly, or bypass ActionResolution

Prompt storage, prompt format, versioning, and privacy constraints are governed by `docs/specs/llm_agent_prompts.md`.

## Agent prompt resource contract

All future agent prompts must live under:

`resources/llm/agents/`

Prompt resources are declarative, versioned, and non-executable.

Runtime must not embed prompt text in code.

UI must not define or modify prompts.

Prompt loading in a future phase does not imply provider invocation, agent execution, tool execution, credential resolution, or ActionResolution execution.

Future traces should record prompt version and fingerprint when prompts are used.

## Governed LLM analysis pipeline

Future governed LLM interaction follows this conceptual sequence:

`UserInput`
-> `IntentAnalysis`
-> `RequestDecomposition`
-> `SubRequestPlan[]`
-> `IndependentSubRequestResolution[]`
-> `ResponseSynthesis`
-> `FinalAnswer`

This sequence is declarative only.

It does not implement runtime execution, provider calls, tool execution, or filesystem mutation.

### IntentAnalysis

`IntentAnalysis` must produce:

- `primary_intent`
- `secondary_intents`
- `user_goal`
- `required_context`
- `ambiguity_level`
- `risk_level`
- `needs_clarification`
- `may_require_tools`
- `may_require_action_resolution`

Purpose:

- identify the dominant user objective
- identify secondary asks that must not be lost
- detect missing context
- detect whether any later governed tool or action path may be relevant

### RequestDecomposition

`RequestDecomposition` must split governed user input into atomic subrequests.

Rules:

- preserve original wording references
- preserve ordering when order matters
- do not invent extra user requests
- do not silently discard lower-priority but still relevant asks

Allowed subrequest classes:

- `informational`
- `planning`
- `code_change`
- `document_change`
- `tool_candidate`
- `clarification_needed`
- `blocked`

### SubRequestPlan

Each subrequest must include:

- `subrequest_id`
- `source_span` or `source_summary`
- `intent`
- `required_context`
- `dependencies`
- `can_resolve_independently`
- `requires_tool_surface`
- `requires_action_resolution`
- `risk_level`
- `expected_output_type`

`SubRequestPlan` is planning metadata only.

It does not authorize execution.

### IndependentSubRequestResolution

Independent resolution is logical independence only.

It means a subrequest can be analyzed or answered without waiting for unrelated sibling subrequests.

This spec does not require physical concurrency.

Parallel execution is a future optimization only.

Each partial resolution may therefore produce:

- `resolved`
- `uncertain`
- `blocked`
- `clarification_required`

### ResponseSynthesis

`ResponseSynthesis` must:

- preserve dependency order
- expose blocked or uncertain subresults
- avoid hiding failed subrequests
- avoid contradiction between partial answers
- produce one coherent final response

`FinalAnswer` is the single synthesized answer returned to the user after governed synthesis.

## Governed user profile, privacy, and consent contract

Future governed LLM interaction may conceptually consume:

- optional `UserProfile`
- optional `UserPreferences`
- optional `ConsentState`

These structures influence:

- `desired_llm_mode`
- `interaction_mode`
- context filtering

They do not:

- trigger tool execution
- bypass `ActionResolution`
- grant access to restricted data
- expose credentials

`UserProfile` remains optional and non-blocking.

Consent must be explicit before any future LLM processing.

Preferences remain advisory only.

Privacy level affects exposure filtering only.

It does not grant execution authority.

Future allowed context may include only:

- explicit user input
- explicitly allowed knowledge sources
- `EffectiveToolSurfaceSummary`, when allowed
- non-sensitive system-state summaries

Future forbidden context includes:

- credentials
- secret values
- denied context references
- hidden system prompts
- raw full tool catalogs by default
- raw host-specific filesystem paths

## F10 step-1 minimum runtime slice

If F10 opens its first minimum governed slice, `llm_core` runtime responsibility may be limited to:

- resolve `LLMCapabilityState`
- resolve `effective_llm_mode`
- enforce consent-aware and privacy-bounded context filtering
- load declarative prompt resources and prompt metadata
- prepare a bounded LLM request envelope
- produce deterministic guided fallback when assistance is unavailable

The first F10 slice must not open:

- broad tool execution
- `ActionResolution` runner
- filesystem mutation
- hidden memory
- raw full tool-catalog injection
- UI authority

## Tool surface note

Future LLM context must receive an effective tool surface, not the raw full catalog by default.

A future tool-surface context provider is policy-oriented and read-only. It does not execute tools or filesystem actions.

Tool visibility does not imply execution permission.

Future LLM tool intent becomes proposal only and must be governed before any execution.

## Credential boundary note

Future LLM execution may require `credential_ref` conceptually.

`llm_core` policy may reference credentials conceptually, but LLM context must never receive credential material.

Credential resolution is a future runtime concern.

LLM availability is optional.

No LLM available must resolve to a non-blocking capability state.

LLM absence must not block core DocGraph usage.

## Invariants

- LLM decomposition does not grant execution authority.
- subrequests do not execute tools directly.
- any tool or action candidate becomes a proposal only.
- `ActionResolution` governs future executable actions.
- credentials are never exposed to LLM context.
- raw full tool catalog is not injected by default.
- Lume is the single governed front-facing agent.
- internal agents do not directly interact with the user.
- internal agents are roles, not authorities.
- all agent prompts live under `resources/llm/agents/`.
- prompts are declarative, versioned, and non-executable.
- prompts contain no credentials or secrets.
- prompts do not hardcode tool availability.
- runtime must not embed prompt text in code.
- UI must not define or modify prompts.
- governed LLM modes are limited to `OFF`, `LOCAL`, and `CLOUD`.
- `OFF` is not an error and must not call LLM providers, local models, tools, or runtime actions.
- `GUIDED` works without LLM capability.
- `ASSISTED` requires available LLM capability.
- `desired_llm_mode` does not equal `effective_llm_mode` by default.
- capability state must expose missing, blocked, unavailable, and fallback states explicitly.
- `credential_ref` grants no execution authority.
- changing preferences must not trigger execution.
- UI, status bar, and tools panel display prepared capability state only.
- tool visibility does not imply execution permission.
- user profile is optional and non-blocking.
- consent must be explicit and separate from execution permission.
- preferences are advisory only.
- no hidden context injection is allowed.
- privacy levels affect exposure, not authority.
- future implementation must be deterministic, traceable, and testable without provider calls.

## Non-goals

- no runtime implementation
- no physical parallelism
- no tool execution
- no document mutation
- no provider calls
- no credential resolution
- no ActionResolution runner
- no filesystem mutation
- no external binary validation
- no UI authority
- no prompt editing UI
- no prompt embedding in code
- no hidden memory
- no user database
- no telemetry
- no behavioral tracking
- no project_runtime changes
- no raw tool catalog injection
- no new pipeline outside `llm_core`
- no UI orchestration

For F10 step 1 specifically:

- no provider binding is implied
- no provider invocation is required
- no tool execution is authorized

---

### !REL_FILE!

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

---

### !REL_FILE!

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

---

### !REL_FILE!

# local_sandbox_context

## Purpose

Define the future authorized local sandbox context for file operations outside the application workspace.

This is declarative only. No filesystem sandbox runtime is implemented here.

Domain separation between `SANDBOX`, `HOST`, and `EXTERNAL` is governed by `docs/specs/sandbox_boundary_model.md`.

## Scope F9

F9 declares policy and user preference placeholders for future authorized sandbox roots.

A valid sandbox is:

- local
- external to the app
- new or empty at authorization time
- explicitly authorized by the user
- not inside `crates`, `resources`, `runtime`, `user`, `output`, `assets`, `dev`, `docs`, or `scripts`

Current enabled actions are readonly or dry-run:

- `read_tree`
- `dry_run_plan`

Mutable actions remain disabled.

The declared sandbox remains the only future writable domain by default.

`HOST` and `EXTERNAL` access remain separately governed and non-active here.

## Forbidden responsibilities

The sandbox context must not:

- allow mutation outside the sandbox
- allow direct LLM execution
- enable mutable actions now
- create hidden logs
- use absolute hardcoded roots
- become a project pipeline

## Future F10/F11 notes

F10 may use this context for governed tool planning if explicit execution is opened.

F11 should audit JSONL logging, authorization, root validation, and mutation gates before enabling write actions.

## Folder organization sandbox profile

- `folder_organization_sandbox` may declare a user-selected source folder.
- The source folder is readonly.
- Mutations are allowed only on a future sandbox working copy.
- F9 does not implement copy, mutation, write-back, or sandbox runtime.
- Write-back to the original folder is a future high-risk action that requires confirmation and trace.
- Sandbox context may be represented in the tabs workspace as a conditional `Sandbox View`.
- `Sandbox Viewer` is the governed inspection surface for that context.
- Sandbox selection may populate the `Sandbox` family in `ActiveObjectContext`.
- In F9 it is declarative only.
- The original selected source folder remains readonly.
- Mutations are future-only and only on sandbox working copy when governed runtime exists.

See `docs/specs/project_profiles.md` for folder sandbox invariants.

---

### !REL_FILE!

# lume_help_popup

## Purpose

Define the F9.5 `Lume Help` contextual help popup for DocGraph.

Lume Help = capa de ayuda contextual gobernada basada en fuentes declarativas.

Lume Help is part of Lume, the single governed front-facing assistant identity.

It must not introduce another assistant identity or expose internal agent roles directly.

Lume Help can operate as local declarative help when LLM is unavailable.

Missing LLM is a state to explain, not an app failure.

In the governed Help menu model, `Open Lume Help` is one dedicated Help entry.

## Scope F9.5

The popup is a readonly, ephemeral help surface. It may explain current governance, phases, restrictions, and usage rules already declared in documentation or resources.

It is conceptually separate from onboarding:

- onboarding is initial help
- Lume Help is contextual help available during use

## Allowed responsibilities

- present governed help text
- present declared suggestions
- present an input placeholder as a visual affordance
- present navigation hints toward help topics and documentation surfaces
- route future user intent to governed surfaces only after a later phase opens that behavior

## Forbidden responsibilities

- no tool execution
- no LLM calls
- no file mutation
- no action creation
- no action approval
- no ActionResolution implementation
- no RAG engine
- no document search
- no parser of governance or specs
- no conversational memory
- no popup history persistence
- no project data storage
- no workspace tab behavior
- no filesystem access
- no credential access
- no internet calls in F9
- no prompt editing UI
- no internal agent selector
- no alternative assistant identity

## Declarative sources

Current declared sources:

- `resources/help/lume_help.json`
- `resources/help/gui_objects.json`
- `resources/lume/lume_help_tree.json`
- `resources/i18n/es/lume_messages.ftl`
- `resources/i18n/en/lume_messages.ftl`
- `docs/specs/lume_interaction_model.md`
- `docs/specs/lume_onboarding_modal.md`
- `docs/specs/gui_objects_v1.md`
- `docs/specs/llm_agent_prompts.md`

## GUI Objects section

Lume Help may present a readonly `GUI Objects` / `Objetos de la interfaz` section.

The section uses `GUI_OBJECTS_v1` canonical names so explanations avoid ambiguous phrases like "this", "here", or "on the left" without naming the GUI object.

The section is declarative help only. It does not infer active context, create actions, call LLMs, execute tools, persist history, or mutate files.

## Help tree section

Lume Help may render a readonly help tree from `resources/lume/lume_help_tree.json`.

The tree may organize navigation, explanation, and procedure nodes for Static Mode guidance.

The popup renders prepared selection state only. It does not interpret policy, execute actions, or decide permissions from tree content.

The main menu may open `Lume Help` directly or request focus on the help tree branch, but it does not change the popup into an execution surface.

Lume Help may also be opened from the DocGraph top-left icon.

Help menu and DocGraph icon are navigation intents to the same `Lume Help` surface.

The icon does not change Lume into an execution surface.

`Help` opens `Lume Help` as an informational popup only.

The popup must not interact with `Tools`, `Preferences`, credentials, or runtime actions.

## Agent and prompt boundary

Lume Help may explain that future assisted reasoning can use internal Planner, Specialist, and Synthesizer roles.

It must present those roles as internal conceptual roles only.

It must not:

- expose internal roles as separate user-facing agents
- let the user select or invoke internal roles directly
- define prompts
- edit prompts
- persist prompt changes
- embed prompt text in UI state
- call providers to test prompts

## Invariants

- `INV-LUME-HELP-001`: Lume Help is contextual help, not runtime.
- `INV-LUME-HELP-002`: Lume Help is based on governed declarative sources or i18n/help resources.
- `INV-LUME-HELP-003`: Lume Help does not execute, mutate, call LLMs, or approve actions.
- `INV-LUME-HELP-004`: the help popup is ephemeral and does not create project data.
- `INV-LUME-HELP-005`: visible text must come from i18n/resources, not hardcoded Slint views.
- `INV-LUME-HELP-006`: Lume Help uses `GUI_OBJECTS_v1` canonical names when explaining interface objects.
- `INV-LUME-HELP-007`: Lume Help MUST NOT access filesystem, credentials, or internet in F9.
- `INV-LUME-HELP-008`: Lume Help MUST remain informational and MUST NOT trigger `ActionResolution`.
- `INV-LUME-HELP-009`: Lume Help MUST remain part of Lume and MUST NOT introduce another front-facing assistant.
- `INV-LUME-HELP-010`: Lume Help MUST NOT define, edit, persist, or test agent prompts.

## Future notes

A later phase may connect the visual input to governed help routing. That must not open LLM execution, tool execution, filesystem mutation, or project-pipeline changes unless a phase explicitly authorizes them.

## Related specs

- `docs/specs/help_menu.md`
- `docs/specs/lume_help_tree.md`
- `docs/specs/gui_objects_v1.md`

---

### !REL_FILE!

# lume_help_tree

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the declarative `lume_help_tree` resource for Static Mode help navigation without LLM.

In the governed Help menu model, `Help Topics` opens this help-tree surface.

The help tree belongs to Lume, the single governed front-facing assistant identity.

It must not introduce another assistant, role selector, or prompt-editing surface.

## Core rule

Lume must be able to present a tree of help topics in Static Mode without LLM.

The tree guides "what to do to..." through navigation, explanation, and procedure nodes.

Visible text must come from i18n keys.

UI only renders prepared state. It does not interpret policy or execute actions.

## Node model

Every help-tree node must declare:

- `node_id`
- `kind`
- `title_i18n`
- `summary_i18n`
- `children`, when applicable
- `steps_i18n`, for procedure nodes
- `related_gui_objects`, when applicable
- `related_tool_ids`, when applicable
- `requires_llm`
- `executes_action`

`related_gui_objects` stores canonical GUI object ids aligned with `GUI_OBJECTS_v1`, not localized visible labels.

Visible GUI names are resolved through help or i18n resources.

## Supported node kinds

- `navigation`
- `explanation`
- `procedure`

Procedure nodes must declare `steps_i18n`.

## Minimal tree coverage

The declared tree must include at least:

- `root`
- `getting_started`
- `create_first_project`
- `tools`
- `merge_pdfs`
- `llm_assistance`
- `static_mode`
- `assisted_mode`

## Static Mode expectations

- help-tree navigation must work without LLM
- nodes may explain current capability limits
- nodes may include procedure steps through i18n references
- nodes must not create actions, mutate files, or execute tools

## Relationship to Lume onboarding

- onboarding may reuse or reference help-tree branches
- the help tree is a reusable declarative navigation surface
- the help tree does not replace onboarding flow or policy

## Relationship to Lume Help popup

- Lume Help may render the declared tree or selected branches
- popup state decides selection and expansion only as presentational state
- popup does not infer permissions or execute operations from tree content

## Relationship to Help menu

- `Help Topics` is a dedicated Help-menu entry
- it opens a dedicated informational popup or equivalent governed help-topics surface
- it remains static or declarative content only
- it does not execute, mutate runtime, or interact with tools

## Relationship to tools

- help nodes may reference tool ids
- referenced tools may be declared, available, non-executable, or future
- tool references are explanatory only

## Relationship to internal agent roles

Help nodes may explain future internal roles:

- Planner
- Specialists
- Synthesizer

Rules:

- internal roles must be described as conceptual roles only
- internal roles must not appear as user-facing assistants
- help nodes must not route directly to internal roles
- help nodes must not define, edit, or display raw agent prompts
- prompt storage is governed by `docs/specs/llm_agent_prompts.md`

## Invariants

- `INV-LUME-TREE-001`: `lume_help_tree` MUST support Static Mode help without LLM.
- `INV-LUME-TREE-002`: visible help-tree text MUST be referenced through i18n keys, not embedded visible strings.
- `INV-LUME-TREE-003`: help-tree nodes MAY describe procedures but MUST NOT execute actions.
- `INV-LUME-TREE-004`: help-tree nodes MAY reference GUI objects and tools but MUST NOT grant authority.
- `INV-LUME-TREE-005`: UI renders prepared tree state only; it does not interpret policy, execute tools, or resolve permissions.
- `INV-LUME-TREE-006`: nodes with `requires_llm=true` MUST remain explanatory when LLM is unavailable.
- `INV-LUME-TREE-007`: `executes_action` MUST remain `false` for F9 declarative help-tree nodes.
- `INV-LUME-TREE-008`: Help Topics MUST remain informational and MUST NOT introduce dynamic runtime logic.
- `INV-LUME-TREE-009`: help-tree content MUST NOT introduce another front-facing assistant.
- `INV-LUME-TREE-010`: help-tree content MUST NOT define, edit, or expose raw agent prompts.

## Forbidden responsibilities

The help tree must not:

- call LLM providers
- execute tools
- create actions
- approve operations
- mutate files
- validate policies
- persist help history
- expose internal agent roles as selectable assistants
- edit prompts
- open F10

## Related specs

- `docs/specs/lume_onboarding_model.md`
- `docs/specs/help_menu.md`
- `docs/specs/lume_help_popup.md`
- `docs/specs/gui_objects_v1.md`
- `docs/specs/llm_tool_surface_resolution.md`
- `docs/specs/llm_agent_prompts.md`

---

### !REL_FILE!

# lume_interaction_model

## Purpose

Define Lume as the governed interaction and help layer for `DocGraph`, the visible application identity of the portable Rust sandbox.

Lume is not a runtime, tool executor, filesystem owner, LLM engine, or project pipeline.

Declared does not mean implemented or executed.

`DocGraph` is the application display name. `Lume` is the assistant display name. This identity is nominal and declarative only; it does not grant runtime behavior or execution authority.

Lume is the single governed front-facing assistant identity for DocGraph.

No second front-facing assistant may be introduced without a governed proposal, phase decision, i18n/resource review, and explicit update to this spec.

## Scope F9

F9 may use declarative resources to describe:

- app display name `DocGraph`
- assistant display name `Lume`
- assistant optional short name `Lu`
- technical label `LUM`
- controlled tone policy
- closed symbol set
- onboarding and help interaction contracts
- transparent user-profile messaging
- `GUI_OBJECTS_v1` canonical names for interface explanations

All visible strings must come from i18n resources, not from Slint hardcoding.

## Interaction pipeline

Lume-facing interaction follows this declarative sequence:

`intent -> response_kind -> tone -> symbol -> i18n message`

The sequence describes presentation policy only. It does not grant execution authority.

Before any future Lume interaction, prepared system state must conceptually resolve:

`desired_llm_mode -> LLMCapabilityState -> effective_llm_mode -> interaction_mode`

Lume consumes this prepared state only.

It does not compute capability state, decide policy, resolve credentials, call providers, or activate modes.

If `effective_llm_mode=OFF`, Lume uses `GUIDED` interaction and may present deterministic guided menus.

If `interaction_mode=ASSISTED`, Lume may present future governed LLM-assisted results only after capability state reports an available LLM mode.

`OFF` is a valid guided mode, not a failure.

For future LLM-assisted flows, Lume may consume a governed synthesized result produced by:

`UserInput`
-> `IntentAnalysis`
-> `RequestDecomposition`
-> `SubRequestPlan[]`
-> `IndependentSubRequestResolution[]`
-> `ResponseSynthesis`
-> `FinalAnswer`

Lume consumes prepared state only.

It does not perform the runtime analysis itself.

## Governed agent flow

Future assisted interaction must preserve this conceptual flow:

```text
User
<-> Lume
-> Planner
-> Specialists
-> Synthesizer
-> Lume
-> User
```

Rules:

- all user interaction enters through Lume
- all user-facing output exits through Lume
- internal roles are not user-facing
- internal roles are conceptual only in F9/F10_PREP
- internal roles may propose tool or action candidates only as proposals
- internal roles do not execute tools, mutate files, approve actions, expose credentials, or bypass ActionResolution

Lume acts as:

- intent detection agent
- guidance/help agent
- orchestration entry and exit point
- capability explainer
- tool-aware but non-executing interface

Lume may also explain:

- current capability state
- missing requirements
- privacy settings
- consent status
- blocked prepared system state shown through future readonly `System View`
- linked trace summaries and non-executing tool-surface summaries shown through future readonly `System View`

Lume must not:

- infer consent
- store hidden data
- escalate permissions
- expose restricted context

Internal prompt governance is defined in `docs/specs/llm_agent_prompts.md`.

## GUI object naming

Lume must use canonical GUI names from `resources/help/gui_objects.json`.

Explanations should follow:

`[GUI Object] + [action] + [state]`

Example: `The document opens in the Readonly Viewer, which is non-editable.`

Lume should avoid spatial or vague expressions such as "this", "here", "above", or "on the left" unless the canonical GUI object is also named.

Lume must not invent alternative names for GUI objects not defined in `GUI_OBJECTS_v1`.

## Tone and symbols

Allowed tones are:

- `neutral`
- `friendly`
- `focused`
- `caution`
- `deescalating`
- `supportive_pedagogical`
- `light_ironic`

Closed symbols are:

- `greeting`: 👋
- `completed`: ✔
- `warning`: ⚠
- `suggestion`: 💡
- `processing`: …

No additional symbols are authorized yet.

## Forbidden responsibilities

Lume must not:

- claim real emotions
- use emotional anthropomorphism
- execute tools
- invoke real LLM workflows
- modify files
- own filesystem policy
- duplicate `project_runtime`
- reinterpret manifests
- store hidden profile data
- trigger workflows
- call LLM providers
- create another front-facing assistant identity
- define or mutate prompt resources

## Non-execution guarantee

Lume cannot:

- execute tools
- mutate filesystem
- trigger workflows
- call LLM providers
- validate credentials
- approve actions
- execute ActionResolution

Lume may explain, guide, request confirmation, or route intent to governed surfaces. The actual runtime authority remains outside Lume and outside Slint.

## Guided menu fallback

When effective LLM mode is `OFF`, Lume may present a deterministic guided menu.

Allowed guided intents are limited to navigation, explanation, or configuration intent:

- `OpenProjectIntent`
- `OpenPreferencesIntent`
- `OpenProjectSettingsIntent`
- `OpenToolsPanelIntent`
- `OpenLumeHelpIntent`
- `ShowCapabilityStateIntent`
- `ExplainMissingCapabilityIntent`
- `ConfigureLocalLlmIntent`
- `ConfigureCloudLlmIntent`

Guided menus must not emit:

- `ExecuteToolIntent`
- `MutateFileIntent`
- `MutateDocumentIntent`
- `CallLlmProviderIntent`
- `ValidateCredentialSecretIntent`
- `RunExternalBinaryIntent`
- `ApproveActionIntent`
- `ExecuteActionResolutionIntent`

Guided menu output remains prepared intent only.

## Response synthesis note

When future governed LLM interaction is available, Lume presentation must reflect synthesized results coherently.

Rules:

- preserve dependency order in user-facing explanation
- expose blocked or uncertain subresults explicitly
- do not hide unresolved subrequests
- avoid contradiction between partial results
- present one coherent final response rather than unrelated fragments

Prepared response state may therefore include summary plus subresult annotations, but it must remain presentation-only.

Humor is allowed only in light contexts. If the user is frustrated, the tone is `supportive_pedagogical` and humor is disabled. If the user is hostile or insulting, the tone is `deescalating`.

Irony is allowed only about the situation, never about the user.

Analogies are allowed only when they clarify a concept. They must be short, technical, and non-anthropomorphic.

## Future F10/F11 notes

F10 may connect Lume-facing chat to governed LLM/tool surfaces, but Lume remains an interaction layer.

F11 should audit that Lume text, tone, symbols, and profile transparency remain declarative and i18n-driven.

Profile, privacy, and consent governance are defined separately in:

- `docs/specs/user_profile_policy.md`
- `docs/specs/privacy_and_consent_model.md`

## Invariants

- UI only displays prepared state.
- Lume does not execute tools, actions, or decomposition steps.
- blocked or uncertain synthesized subresults must remain visible when relevant.
- Lume consumes `LLMCapabilityState`; it does not compute it.
- `GUIDED` interaction must work without LLM capability.
- `ASSISTED` interaction requires available LLM capability.
- `OFF` mode must not call LLM providers, local models, tools, or runtime actions.
- guided menu intents must remain non-executing navigation, explanation, or configuration intents only.
- Lume is the single governed front-facing agent.
- all user interaction must enter and exit through Lume.
- internal agent roles must not directly interact with the user.
- Lume must not define, edit, or mutate agent prompts.
- Lume must not manage or store private user data.
- consent must remain explicit and separate from execution permission.
- future `System View` interpretation by Lume must remain explanatory only and must remain compatible with Static Mode.

---

### !REL_FILE!

# lume_onboarding_modal

## Purpose

Define the declarative onboarding and initial help modal for Lume inside `DocGraph`.

The onboarding modal is not a project, not a persistent chat, and not an execution surface.

Declared does not mean implemented or executed.

`DocGraph` is the visible application identity. `Lume` is the assistant/help identity. The onboarding modal uses these names declaratively and does not create runtime, tool, LLM, filesystem, or project-pipeline authority.

## Scope F9

F9 declares:

- modal id `lume_onboarding_help`
- ephemeral help chat
- startup preference `show_onboarding_modal`
- manual opening from the help menu
- optional intro video placeholder
- i18n keys for greeting, input placeholder, checkbox, and common labels

Onboarding must work in `Static Mode` without LLM.

Onboarding may show capability state.

Onboarding must not imply LLM availability.

The modal may be hidden at startup by user preference, but it must remain manually openable from the help menu.

## UX constraints

The onboarding modal should remain:

- simple
- optional
- skippable
- context-help oriented
- limited to one primary action

The modal is help, not workspace work.

## Forbidden responsibilities

The onboarding modal must not:

- persist chat history
- create project data
- generate file outputs
- execute tools
- invoke real LLM execution
- mutate filesystem
- hardcode visible strings in Slint
- belong to workspace tabs
- save artifacts
- interact with `project_runtime`

## Chat boundary

The onboarding chat is ephemeral help context.

It is not:

- project chat
- document storage
- a workspace tab
- a tool invocation surface
- an LLM execution surface

## Future F10/F11 notes

F10 may route onboarding questions through governed LLM policy only if real execution is explicitly opened.

F11 should verify that disabling startup display does not remove manual help access.

---

### !REL_FILE!

# lume_onboarding_model

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define Lume as governed onboarding that works without LLM and can be expanded with future LLM assistance.

## Core rule

DocGraph MUST remain usable when no LLM engine, model, provider, credential, or executable tool is available.

Lume onboarding MUST distinguish:

- `Static Mode`: local declarative help, no LLM.
- `Assisted Mode`: future LLM-assisted help, only when explicitly available.

For LLM capability governance, these map to interaction modes:

- `Static Mode` corresponds to `GUIDED`.
- `Assisted Mode` corresponds to `ASSISTED`.

`GUIDED` must remain available when `desired_llm_mode`, `effective_llm_mode`, or capability state resolves to `OFF`.

## Lume modes

### Static Mode

- default when no LLM is available
- uses declarative help, i18n, and resources
- may use `lume_help_tree` as a navigable static help structure
- may explain UI objects, phases, limitations, project setup, and available surfaces
- may guide the user through project creation and configuration
- does not interpret free-form intent as executable action
- does not call providers
- does not execute tools

### Assisted Mode

Future only.

- may use governed LLM execution when F10+ opens it
- must still respect `ActionResolution`, tool policy, credentials policy, and Tool Surface Resolver
- must never expose credentials to LLM context

## StartupCapabilityState conceptual fields

- `llm_available`
- `local_model_available`
- `credential_ref_present`
- `operational_tools_available`
- `executable_tools_count`
- `declared_tools_count`
- `project_loaded`
- `declarative_help_available`
- `current_lume_mode`

`StartupCapabilityState` may also feed a future bottom status bar.

The status bar may expose compact capability state, while Lume explains it.

## LLMCapabilityState onboarding contract

Before onboarding chooses guided or assisted presentation, the prepared capability state must include:

- `desired_llm_mode`: `OFF | LOCAL | CLOUD`
- `effective_llm_mode`: `OFF | LOCAL | CLOUD`
- `interaction_mode`: `GUIDED | ASSISTED`
- `llm_available`
- `provider_configured`
- `credential_ref_present`
- `local_engine_available`
- `local_model_available`
- `policy_allows_local`
- `policy_allows_cloud`
- `tool_surface_available`
- `execution_enabled`
- `reason_codes`

Rules:

- `desired_llm_mode=OFF` resolves to `effective_llm_mode=OFF` and `interaction_mode=GUIDED`
- missing local engine, missing local model, or local policy block resolves requested `LOCAL` to `OFF/GUIDED` with reason codes
- missing provider, missing `credential_ref`, or cloud policy block resolves requested `CLOUD` to `OFF/GUIDED` with reason codes
- available and policy-allowed `LOCAL` resolves to `LOCAL/ASSISTED`
- available and policy-allowed `CLOUD` resolves to `CLOUD/ASSISTED`
- no mode grants tool execution authority

Required reason codes include:

- `llm_off_by_preference`
- `local_engine_missing`
- `local_model_missing`
- `local_policy_blocked`
- `cloud_provider_missing`
- `cloud_credential_ref_missing`
- `cloud_policy_blocked`
- `execution_not_open`
- `assisted_mode_unavailable`
- `guided_fallback_active`

## User-facing onboarding goals

At first launch, Lume should explain:

- DocGraph can be used without AI
- LLM assistance is optional and may be activated later
- current system capability state
- what the user can do now
- what is declared but not executable
- next recommended setup step

## Minimal onboarding flow

1. welcome
2. show capability state
3. explain current mode
4. guide project creation or opening
5. guide project profile selection
6. guide preferences
7. optionally explain credentials as future references, not secrets
8. finish with "ready for configuration" or future equivalent state

## Required user message in Static Mode

Include this conceptual message:

"Lume is running in local help mode. No LLM is active. You can still create and configure projects, inspect documents, use available non-LLM surfaces, and activate assisted capabilities later."

## OFF-mode guided menu

When `effective_llm_mode=OFF`, onboarding may present a deterministic menu such as:

1. Open project
2. Review documents
3. Open Tools Panel
4. Configure local LLM
5. Configure cloud LLM
6. Show capability state
7. Open Lume Help

This menu is conceptual only.

It may emit navigation, explanation, or configuration intent, but it must not execute tools, call providers, validate credential secrets, mutate files, approve actions, or execute ActionResolution.

## Relationship to Project Setup / Settings

- Lume guides.
- Project Setup / Settings captures drafts.
- Future runtime validates and persists.
- Lume does not write project files.
- Lume does not decide profile, tools, credentials, or policies.

## Relationship to tools

- Lume may display effective tool surface summary.
- Lume must not inject raw full catalog by default.
- Lume must not execute tools.
- Lume may explain `declared`, `available`, `not executable`, and `future`.

## Relationship to credentials

- Lume must not require credentials for Static Mode.
- Lume must not see credential values.
- Missing credentials must not block core app usage.

## Invariants

- `INV-LUME-ONBOARD-001`: DocGraph MUST remain usable without any LLM engine, model, provider, credential, or executable tool.
- `INV-LUME-ONBOARD-002`: Lume MUST support Static Mode based on declarative local help.
- `INV-LUME-ONBOARD-003`: Lume MUST distinguish Static Mode from Assisted Mode.
- `INV-LUME-ONBOARD-004`: Lume MUST expose capability state instead of failing silently when LLM is unavailable.
- `INV-LUME-ONBOARD-005`: Missing LLM capability MUST NOT block project creation, project setup, preferences, readonly navigation, or declarative help.
- `INV-LUME-ONBOARD-006`: Lume MUST NOT require credentials in Static Mode.
- `INV-LUME-ONBOARD-007`: Lume MUST NOT execute tools, mutate files, or approve actions in either mode.
- `INV-LUME-ONBOARD-008`: Assisted Mode MUST remain governed by LLM policy, credentials policy, Tool Surface Resolver, and ActionResolution.
- `INV-LUME-ONBOARD-009`: `OFF` mode MUST resolve to guided onboarding and MUST NOT be treated as an error.
- `INV-LUME-ONBOARD-010`: `desired_llm_mode` MUST NOT be treated as `effective_llm_mode`.
- `INV-LUME-ONBOARD-011`: capability state MUST expose missing, blocked, unavailable, and guided fallback reason codes.
- `INV-LUME-ONBOARD-012`: `credential_ref` MUST NOT grant execution authority.
- `INV-LUME-ONBOARD-013`: guided menu fallback MUST NOT emit executing intents.

## Forbidden responsibilities

Lume onboarding must not:

- call LLM providers
- execute tools
- validate credentials
- store secrets
- write project files
- mutate `project_manifest`
- decide project policy
- become Project Setup runtime
- bypass `ActionResolution`
- open F10

## Related specs

- `docs/specs/lume_onboarding_modal.md`
- `docs/specs/lume_help_popup.md`
- `docs/specs/lume_interaction_model.md`
- `docs/specs/project_setup_popup.md`
- `docs/specs/llm_core.md`
- `docs/specs/credentials_policy.md`
- `docs/specs/llm_tool_surface_resolution.md`

---

### !REL_FILE!

# patch_preview

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `PatchPreviewOverlay` model and semantic highlight behavior for DocGraph.

The goal is to show an in-place proposal over a valid selection inside `Document Viewer` while making it clear that the underlying document has not changed.

## Dependencies

This spec depends on:

- `docs/specs/transformation_core.md`
- `docs/specs/document_viewer.md`
- `docs/specs/transform_popup.md`

## Canonical model

`PatchPreviewOverlay` uses the transformation-core vocabulary and may include:

- `overlay_id`
- `proposal_id`
- `selection_ref`
- `target_ref`
- `original_text_hash`
- `proposed_text`
- `overlay_kind`
- `overlay_state`
- `highlight_token`

Field meaning:

- `overlay_id`: stable overlay identity for the prepared visual layer
- `proposal_id`: governed proposal identity associated with the preview
- `selection_ref`: governed selection scope for the preview
- `target_ref`: governed document target
- `original_text_hash`: integrity marker tying preview to the original selected content
- `proposed_text`: proposed replacement or transformed text snapshot
- `overlay_kind`: governed overlay rendering kind
- `overlay_state`: prepared visual state of the overlay
- `highlight_token`: semantic theme token used for the pending proposal highlight

## Main rule

`Document source + PatchPreviewOverlay = viewer rendering`

The real document does not change.

The overlay is visual composition only.

It is not:

- persistence
- acceptance
- application
- document mutation

## Overlay states

Prepared overlay state may include:

- `pending_proposal`
- `accepted_visual_feedback_future`
- `cancelled`
- `stale`
- `superseded`
- `error`

Interpretation:

- `pending_proposal`: proposal exists and is awaiting governed user validation
- `accepted_visual_feedback_future`: future-only visual state after acceptance feedback
- `cancelled`: popup flow was cancelled and overlay should disappear
- `stale`: overlay no longer matches current document or selection integrity
- `superseded`: a newer proposal replaced the previous one
- `error`: overlay could not be rendered safely as prepared state

## Highlight semantics

The preview should use semantic color tokens:

- `proposal.pending.background`
- `proposal.pending.border`, optional

Semantic meaning:

- blue = proposal pending validation
- normal text remains normal text
- the overlay must not look like saved text

## Highlight rules

- blue indicates a pending proposal awaiting validation
- the highlight must not resemble committed or saved document state
- the highlight must not fatigue reading
- the highlight must not block legibility
- the highlight should guide attention, not hijack it
- strong red or green should not be the default pending-preview language

## Animation

Subtle fade-in is optional.

No animation is required in F9.

Animation must not be necessary to understand that the overlay is pending preview only.

## Relationship to Document Viewer

`Document Viewer` may render the overlay when prepared state provides it.

The viewer still renders a readonly document surface.

The overlay does not make the viewer editable and does not apply the proposal.

## Relationship to Transform Popup

The popup may generate or cancel the preview flow.

`Cancelar` should clear the overlay representation.

`Aceptar` may leave future visual feedback state, but no application occurs in F9.

For deeper comparison under user demand, side-by-side review is governed separately in `docs/specs/diff_view.md`.

## Relationship to transformation_core

This spec consumes `PatchPreviewOverlay` from `transformation_core.md`.

It must not redefine proposal lineage, acceptance, cancellation, or supersession semantics outside that shared vocabulary.

## Failure modes

- `overlay_without_proposal`
- `stale_overlay`
- `overlay_target_missing`
- `overlay_conflict`
- `unsupported_overlay_kind`

Interpretation:

- `overlay_without_proposal`: prepared overlay exists without valid proposal lineage
- `stale_overlay`: overlay no longer matches current document or selection integrity
- `overlay_target_missing`: target reference no longer resolves
- `overlay_conflict`: two incompatible overlay states compete for the same governed target region
- `unsupported_overlay_kind`: prepared overlay kind cannot be rendered by the current governed viewer contract

## Invariants

- `INV-PPREV-001`: `PatchPreviewOverlay` MUST remain visual-only rendering state
- `INV-PPREV-002`: rendering an overlay MUST NOT mutate the document
- `INV-PPREV-003`: pending preview highlight MUST communicate unvalidated proposal state
- `INV-PPREV-004`: default pending-preview highlight MUST avoid strong red/green commit semantics
- `INV-PPREV-005`: overlay legibility MUST remain subordinate to document readability
- `INV-PPREV-006`: cancellation MUST clear overlay rendering state

## Related specs

- `docs/specs/transformation_core.md`
- `docs/specs/document_viewer.md`
- `docs/specs/transform_popup.md`
- `docs/specs/diff_view.md`

---

### !REL_FILE!

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

---

### !REL_FILE!

# preferences_policy

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define user and project preferences as non-sensitive configuration.

This spec governs preference content.
Popup surfaces and menu routing are defined separately.

## Core rule

Preferences are non-secret behavior configuration.
Preferences are not credentials.
Preferences are not execution authority.

## Preference scopes

- `app_preferences`
- `user_preferences`
- `project_preferences`

Popup mapping:

- `App preferences` edits global non-sensitive user-scoped configuration
- `Project settings` edits project-scoped portable configuration
- `Credentials management` is not a preference surface and is governed separately

## Allowed preference examples

- `language`
- `theme`
- `default_agent_profile`
- `lume_help_level`
- `show_onboarding_modal`
- `desired_llm_mode`
- `default_tool_surface_scope`
- `preferred_project_profile`
- `document_default_language`
- `confirmation_friction_level`

## LLM preference contract

Preferences may store a desired LLM mode only as non-secret user intent.

Conceptual `UserPreferences` fields may include:

- `desired_llm_mode`
- `interaction_mode_preference`
- `privacy_level`
- `tool_visibility_preference`
- `explanation_level`

Allowed values:

- `OFF`
- `LOCAL`
- `CLOUD`

`interaction_mode_preference` may use:

- `GUIDED`
- `ASSISTED`

`privacy_level` may use:

- `STRICT`
- `BALANCED`
- `PERMISSIVE`

`explanation_level` may use:

- `LOW`
- `MEDIUM`
- `HIGH`

Rules:

- `desired_llm_mode` is not `effective_llm_mode`
- `interaction_mode_preference` is advisory only
- `privacy_level` is advisory only
- `tool_visibility_preference` is advisory only
- `explanation_level` is advisory only
- changing `desired_llm_mode` does not call providers
- changing `desired_llm_mode` does not load local models
- changing `desired_llm_mode` does not validate credentials
- changing `desired_llm_mode` does not execute tools
- changing `desired_llm_mode` does not enable ActionResolution execution
- future mode availability must be derived from prepared `LLMCapabilityState`

If `desired_llm_mode` cannot be satisfied, the effective mode must fall back to `OFF` and `GUIDED` interaction with reason codes.

Preferences influence future prepared state only.

They do not override policy, grant authority, or authorize restricted context exposure.

## Forbidden preference contents

Preferences MUST NOT contain:

- API keys
- tokens
- passwords
- private keys
- provider secrets
- raw credentials
- sensitive file contents
- hidden user profiling data

## Project Setup / Settings relationship

Project Setup may capture initial preferences.
Project Settings may edit preferences later.

UI captures preference drafts.
Future governed runtime validates and persists.
Changing preferences MUST NOT enable execution by itself.

In the governed Preferences menu model:

- `App preferences` MUST remain user-scoped
- `App preferences` MUST live outside project structure
- `Project settings` MAY persist project-scoped configuration inside project structure
- `Preferences` popups emit `ConfigurationIntent`, not `ExecutionIntent`

## Invariants

- `INV-PREF-001`: preferences MUST NOT contain secrets.
- `INV-PREF-002`: preferences MAY configure behavior, language, theme, defaults, and help level.
- `INV-PREF-003`: preferences MUST NOT grant execution authority.
- `INV-PREF-004`: preferences MUST NOT bypass project policy, tool policy, LLM policy, or `ActionResolution`.
- `INV-PREF-005`: project preferences MUST remain portable and avoid host-specific absolute paths.
- `INV-PREF-006`: preferences may be edited through Project Settings in a future governed flow.
- `INV-PREF-007`: preference changes affecting risk or policy require confirmation.
- `INV-PREF-008`: preferences must be distinguishable from credentials in schema and storage.
- `INV-PREF-009`: `App preferences` MUST NOT contain filesystem paths, execution toggles, tool activation flags, or LLM execution authority.
- `INV-PREF-010`: `Preferences` surfaces MUST remain separate from `Tools`.
- `INV-PREF-011`: `desired_llm_mode` MAY store `OFF`, `LOCAL`, or `CLOUD` intent only.
- `INV-PREF-012`: changing LLM preferences MUST NOT trigger provider calls, local model usage, tool execution, credential validation, or filesystem mutation.
- `INV-PREF-013`: preferences MUST NOT decide effective LLM availability.
- `INV-PREF-014`: preference values are advisory and MUST NOT override policy or consent.
- `INV-PREF-015`: privacy-related preferences may affect future context filtering but MUST NOT grant execution authority.

## Forbidden responsibilities

This spec must not:

- implement preferences runtime
- create UI
- write project files
- validate live credentials
- enable LLM
- call LLM providers
- resolve effective LLM mode
- enable tools
- execute tools
- open F10

## Related specs

- `docs/specs/ui_preferences_popups.md`
- `docs/specs/project_settings_popup.md`
- `docs/specs/credentials_policy.md`
- `docs/specs/user_profile_policy.md`
- `docs/specs/privacy_and_consent_model.md`

---

### !REL_FILE!

# privacy_and_consent_model

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed privacy and consent contract for future Lume and LLM-facing interaction.

This spec defines exposure boundaries only.

It does not implement storage, consent UI, provider invocation, or data processing.

## Core rule

Context exposure must be explicit, bounded, and privacy-safe.

Consent is required before any future LLM processing.

Consent does not imply execution permission.

## ConsentState model

Conceptual structure:

- `consent_version`
- `consent_given`
- `consent_scope`
- `consent_timestamp`

`consent_scope` may include:

- `allow_llm_processing`
- `allow_context_usage`
- `allow_tool_suggestions`

Rules:

- no implicit consent
- no hidden consent escalation
- consent is explicit or absent
- consent must remain separate from execution permission
- consent must remain separate from credential handling

## Context exposure contract

Allowed future context may include:

- explicit user input
- explicitly allowed knowledge sources
- `EffectiveToolSurfaceSummary`, when allowed
- non-sensitive system-state summaries

Forbidden future context includes:

- credentials
- secret values
- raw host-specific filesystem paths
- hidden system prompts
- full raw tool catalogs by default
- denied context references

Rules:

- context exposure must be explicit and bounded
- hidden context injection is forbidden
- exposure must respect consent and privacy level
- bounded context does not imply execution authority

## Privacy levels

The governed privacy levels are:

- `STRICT`
- `BALANCED`
- `PERMISSIVE`

### STRICT

- minimal context only
- no tool-surface exposure
- no inferred context
- guided interaction preferred

### BALANCED

- bounded context
- limited tool awareness
- explicit sources only

### PERMISSIVE

- broader governed context
- more explainability
- still no credentials or secrets

Rules:

- privacy level never bypasses governance
- privacy level does not grant execution authority
- privacy level does not override denied context references

## Lume privacy boundary

Lume may explain:

- current capability state
- missing requirements
- privacy settings
- consent status

Lume must not:

- infer consent
- store hidden data
- escalate permissions
- expose restricted context

## Relationship to future LLM interaction

Privacy and consent may influence:

- future context filtering
- desired interaction presentation
- whether LLM processing is allowed conceptually

Privacy and consent must not:

- trigger tool execution
- bypass ActionResolution
- grant access to restricted data
- expose credentials

## Invariants

- `INV-PRIV-001`: consent must be explicit.
- `INV-PRIV-002`: consent is separate from execution permission.
- `INV-PRIV-003`: hidden context injection is forbidden.
- `INV-PRIV-004`: credentials must never enter LLM context.
- `INV-PRIV-005`: privacy levels affect exposure, not authority.
- `INV-PRIV-006`: denied context references must remain excluded.
- `INV-PRIV-007`: no runtime persistence is defined in F9/F10_PREP.
- `INV-PRIV-008`: Lume must not manage or store private data.

## Non-goals

- no runtime storage implementation
- no consent database
- no authentication system
- no encryption-layer definition
- no cloud sync
- no telemetry
- no behavioral tracking
- no UI consent forms
- no real data handling

## Related specs

- `docs/specs/user_profile_policy.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/lume_interaction_model.md`
- `docs/specs/llm_core.md`
- `docs/specs/llm_tool_surface_resolution.md`

---

### !REL_FILE!

# project_folder_layout

## Status

NORMATIVE DOCUMENTATION. This file defines the fundamental DocGraph project folder contract. It does not implement runtime behavior.

## Purpose

Define the governed physical layout for a DocGraph project.

The project is the user's work boundary. It scopes project configuration, knowledge files, chats, documents, allowed tools, artifact relations, and traceability.

The governed project pipeline remains:

`project_root -> manifest -> contract -> validation -> surface -> resolve -> output`

`project_runtime` governs. The filesystem organizes. The manifest exposes. The graph explains relations. Folders must not reinterpret the manifest.

## Fundamental tree

```text
project_root/
  project_manifest.json
  registry.json
  config/
    project_config.json
    llm_policy.json
    tool_policy.json
    preferences.json
  knowledge/
    manifests/
    files/
    derived/
      text/
      chunks/
    indexes/
    semantic/
      proposals/
  chats/
    chat_sessions/
      <chat_id>/
        manifests/
          chat_manifest.json
          context_refs.json
        messages.jsonl
        attachments/
          files/
          figures/
        artifacts/
        tool_outputs/
          <tool_id>/
            <run_id>/
              tool_run_manifest.json
              outputs/
              logs/
  documents/
    <document_holder>/
      document.md
      manifests/
        document_manifest.json
        source_context.json
      template/
        template_snapshot.json
        template_overrides.json
        effective_template.json
      references/
      assets/
      lifecycle/
        status.json
        review_log.jsonl
      tool_outputs/
        <tool_id>/
          <run_id>/
            tool_run_manifest.json
            outputs/
            logs/
      derived/
        latex/
        docx/
        pdf/
  graph/
    graph_manifest.json
    artifact_nodes.jsonl
    artifact_edges.jsonl
    snapshots/
  logs/
    project_events.jsonl
```

## Normative decisions

- No `outputs/` folder exists at project root.
- Every tool output must have an `owner_ref`.
- Tool outputs live inside their functional owner.
- Chat-owned tool outputs live under `chats/chat_sessions/<chat_id>/tool_outputs/`.
- Document-owned tool outputs live under `documents/<document_holder>/tool_outputs/`.
- Knowledge-owned derivations live under `knowledge/derived/` or `knowledge/semantic/proposals/`.
- If a tool output contributes to or is used by a `DocumentHolder`, it MUST be stored under the `DocumentHolder` `tool_outputs/`, even if derived from knowledge.
- Knowledge-derived outputs are only valid in `knowledge/derived/` when they are not owned by a document.
- If a tool action has no `owner_ref`, the action is invalid or pending clarification.
- `DocumentHolder` is the document production unit.
- `document.md` is the editable document source.
- `derived/` contains regenerable products: `latex/`, `docx/`, and `pdf/`.
- Chat `attachments/` are user-provided inputs.
- Chat `artifacts/` are session-owned products.
- `knowledge/files/` contains project sources.
- `knowledge/derived/` contains regenerable derivatives.
- `knowledge/semantic/proposals/` contains semantic proposals, not approved facts.
- `graph/` contains relations between artifacts, not source files.
- `registry.json` is a fast navigation index. It does not replace `project_manifest.json` or `graph/`.
- `registry.json` MUST be derivable from `project_manifest.json` and MUST NOT introduce independent state.
- `registry.json` is a navigation accelerator and cannot be a source of truth.
- `manifests/` groups governed metadata inside each domain.
- `lifecycle/` exists only for DocumentHolder state and review.
- `project_profile` is stored as declarative project configuration.
- `config/preferences.json` stores non-secret preferences.
- `llm_policy.json` and `tool_policy.json` may refer conceptually to `credential_ref`.
- Project files MUST NOT store credential secret values.
- Profile selection does not alter the normative folder layout.
- Future sandbox working copy location must not create project-root `outputs/`.

## Invariants

- `INV-PROJ-LAYOUT-001`: `project_root` MUST NOT contain a root `outputs/` directory.
- `INV-PROJ-LAYOUT-002`: every durable artifact MUST have stable id, `owner_ref`, and trace origin.
- `INV-PROJ-LAYOUT-003`: `registry.json` MUST be derivable from `project_manifest.json` and MUST NOT introduce independent state.
- `INV-PROJ-LAYOUT-004`: filesystem presence MUST NOT imply project exposure.
- `INV-PROJ-LAYOUT-005`: project exposure MUST be governed by `project_manifest.json`, not folder scanning.
- `INV-CONSISTENCY-001`: any artifact referenced by another artifact MUST exist, be resolvable, and be exposed by `project_manifest.json`.
- `INV-ID-001`: artifact IDs MUST be stable across moves and renames and MUST NOT depend on filesystem path.
- The filesystem stores physical location.
- The manifest governs exposure.
- `registry.json` accelerates navigation and cannot be a source of truth.
- `graph/` explains relations.
- `graph/` does not decide, execute, or approve.
- `graph/` does not replace `project_runtime`.
- Semantic relations remain proposals until human review.
- No RDF, Oxigraph, SPARQL, embeddings, or semantic store is introduced by this layout.
- No graph runtime is implied.
- No tool execution is implied.
- No UI logic is implied.
- No hardcoded absolute paths are allowed.
- No duplicate project pipeline is allowed.

## Related specs

- `docs/specs/project_runtime.md`
- `docs/specs/artifact_graph.md`
- `docs/specs/document_package.md`
- `docs/specs/chat_document_flows.md`
- `docs/specs/tools_panel.md`

## F12.4 file intake layout alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

F12.4 does not define or create a final runtime intake layout.

Future intake must align with this project folder layout, must not use project-root `outputs/`, must not use `assets/` as runtime storage, and must not treat filesystem presence as project exposure.

## F13.4 manifest exposure layout alignment

`F13.4` is SPEC-only and adds no runtime layout mutation.

Future project exposure may only become visible through a governed `project_manifest.json` declaration containing portable sanitized refs.

Layout rules:

- `project_manifest.json` remains the only exposure authority at project level
- no blob path, copied file path, registry row, graph row, or tree node path may stand in for a `ManifestExposureEntry`
- future manifest exposure entries must not persist private absolute host paths
- future manifest exposure entries must not create project-root outputs or imply derivative folders
- folder presence remains organizational only and must not reinterpret manifest-governed exposure

`project_manifest.json` remains the authority for exposed project content.

## F12.5 file intake layout plan

`F12.5` is plan-only and does not create folders.

Future `F12.6` may use only an owner-scoped intake directory or governed file-store location already allowed by storage policy.

Future `F12.6` must keep copied or candidate files away from project-root outputs, `assets/`, source folders, registry locations, graph locations, and derivative locations unless a later explicit phase opens those targets.

---

### !REL_FILE!

# project_profiles

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define declarative project profiles for DocGraph.

## Core rule

`project_profile` declares intended capabilities and default policies.

`project_profile` does not execute anything.
`project_profile` does not enable runtime by itself.

## Required profile field

Every project must declare:

- `project_profile`

## Initial project profiles

- `document_engineering`
- `knowledge_analysis`
- `code_sandbox`
- `folder_organization_sandbox`

## Profile: document_engineering

Purpose:

- technical report creation and analysis
- `DocumentHolder`
- references
- future export

It does not imply export runtime.

## Profile: knowledge_analysis

Purpose:

- analysis of knowledge files
- reading, references, and future semantic proposals

It does not imply embeddings, RDF, Oxigraph, or real LLM execution.

## Profile: code_sandbox

Purpose:

- sandbox project oriented to programming and code
- future controlled code analysis and execution

F9:

- `declared_only`
- no code execution
- no filesystem mutation
- no external runtime execution

## Profile: folder_organization_sandbox

Purpose:

- the user selects a folder from the personal PC
- the original selected folder is treated as readonly
- DocGraph prepares a future working copy inside project sandbox scope
- future tools or LLM may propose organization
- future mutations apply only to the working copy inside sandbox scope

Critical rule:

Original selected folder = readonly.  
Sandbox working copy = mutable only when a future governed execution phase is opened.

This profile may cause the workspace to expose a `Sandbox View` tab.

The tab is visibility and context only in F9.

Profile selection does not enable filesystem mutation.

## folder_organization_sandbox conceptual flow

1. user selects source folder
2. DocGraph records explicit access intent
3. source folder is read in readonly mode
4. future working copy is created in project sandbox scope
5. future tool or LLM proposes organization plan
6. future mutations apply only to working copy
7. user reviews result
8. any write-back to source requires separate future action, explicit confirmation, and trace

## folder_organization_sandbox policy sketch

The following example is conceptual only. It does not execute anything.

```json
{
  "project_profile": "folder_organization_sandbox",
  "source_access": {
    "kind": "explicit_user_selected_folder",
    "mode": "readonly",
    "persist_absolute_path": false
  },
  "sandbox_policy": {
    "enabled": true,
    "mutation_scope": "project_sandbox_working_copy_only",
    "allowed_actions_f9": [
      "declare_source_folder",
      "dry_run_plan"
    ],
    "future_actions": [
      "read_tree",
      "copy_into_sandbox",
      "rename_in_sandbox",
      "move_in_sandbox",
      "deduplicate_in_sandbox",
      "export_plan_to_source"
    ]
  }
}
```

## Invariants

- `INV-PROJECT-PROFILE-001`: every project MUST declare a `project_profile`.
- `INV-PROJECT-PROFILE-002`: `project_profile` declares intended capabilities and default policies, not runtime authority.
- `INV-PROJECT-PROFILE-003`: changing `project_profile` MUST NOT enable execution by itself.
- `INV-FOLDER-SANDBOX-001`: the user-selected source folder MUST be treated as readonly unless a future explicit export or write-back phase is opened.
- `INV-FOLDER-SANDBOX-002`: folder organization mutations MUST operate only on the sandbox working copy, never on the original selected source folder.
- `INV-FOLDER-SANDBOX-003`: host-specific absolute source paths MUST NOT become portable project truth.
- `INV-FOLDER-SANDBOX-004`: LLM output may propose an organization plan, but only governed tools may materialize changes inside the sandbox.
- `INV-FOLDER-SANDBOX-005`: any future write-back from sandbox to source folder MUST require explicit user confirmation and trace.
- `INV-FOLDER-SANDBOX-006`: sandbox working copy MUST be distinguishable from original source folder in manifests and trace.

## Forbidden responsibilities

`project_profiles` must not:

- implement sandbox
- implement folder copy
- execute code
- execute tools
- call LLM
- mutate source folder
- create graph runtime
- reinterpret `project_manifest`
- duplicate `project_runtime`

---

### !REL_FILE!

# project_runtime

## Purpose

Provide the Rust successor layer for project-scoped runtime behavior, inheriting the live Python project model.

## Responsibilities

- treat the project as the primary working unit
- preserve project isolation and project-scoped path safety
- validate manifest-driven exposure rules
- resolve authorized refs from manifest declarations
- build neutral surface models for controller or UI-facing layers
- preserve separation between artifacts, outputs, and runs

## Inputs

- workspace root and project root descriptors
- project metadata and manifest payloads
- scoped reference requests
- controller or service-side project operations

## Outputs

- project descriptors
- manifest validation results
- normalized authorized refs
- neutral surface models
- runtime-safe project operation errors

## Allowed Dependencies

- standard library
- `core_domain`
- `workspace_core`
- `io_runtime`
- `spec_runtime`

## Forbidden Responsibilities

- no direct UI widget logic
- no filesystem-driven visibility model
- no direct provider logic
- no uncontrolled absolute path exposure
- no workflow execution engine yet

## Initial Phase Scope

- project identity and scope checks
- manifest validation
- ref resolution
- neutral surface model construction
- no full project execution lifecycle yet

## F9 limits

- no `registry.json` generator is implemented
- no graph writer or graph runtime is implemented
- no passive filesystem scanning may expose project artifacts
- no runtime `owner_ref` enforcement is implemented yet
- no persisted `tool_run_manifest.json` generation is implemented yet
- no chat artifact promotion runtime is implemented yet
- no document creation or export runtime is implemented yet
- no Project Setup runtime is implemented
- no Project Settings runtime is implemented
- no `project_profile` enforcement runtime is implemented
- no sandbox working copy creation is implemented

## F12.4 file intake project-runtime boundary

`docs/specs/file_intake.md` owns governed file intake semantics.

F12.4 does not modify `project_runtime` behavior.

Future file intake must not bypass the governed project pipeline. `project_manifest.json` remains exposure authority, and passive filesystem presence must not create project exposure.

`project_runtime` must not be expanded by convenience for intake without a later explicit implementation slice and audit.

## F12.5 file intake project-runtime plan

`F12.5` is plan-only and does not modify `project_runtime`.

Future `F12.6` must not modify `project_runtime` unless a later explicit audit approves project exposure or project-pipeline participation.

Future `F12.6` must not mutate `project_manifest.json`, infer exposure, bypass `project_runtime`, or add convenience project authority for intake.

## F13.0 project exposure gate boundary

`F13.0` is SPEC-ONLY and does not modify `project_runtime`.

`docs/specs/file_intake.md` owns the Project Exposure Gate semantics.

Future project exposure must remain manifest-driven:

- `project_manifest.json` is the only exposure authority
- future exposure may only promote supported `imported_not_exposed` intake objects after governed request, candidate, and human confirmation
- blocked and unsupported intake items must not be exposed
- `owner_ref` and `trace_ref` must be preserved into any future manifest-governed exposure record
- duplicate exposure must not collapse distinct `IntakeItem`, `file_ref`, or logical object identity

`project_runtime` may later prepare document tree state from manifest-exposed objects only after a separate implementation slice explicitly opens the manifest update path.

`project_runtime` must not infer exposure from filesystem scanning, `file_store` presence, intake metadata, registry presence, graph presence, history/index rows, chat references, or UI selection.

F13.0 does not create a manifest writer, registry generator, graph writer, derivative runtime, `document_text_runtime` call, `tool_runtime` call, rollback flow, or revocation flow.

## F13.1 legacy bypass hardening boundary

`F13.1` is SPEC-only and does not modify `project_runtime`.

Before any future `F13` runtime implementation:

- `project_runtime` must not accept `list_project_documents` or any filesystem scan as project exposure authority
- `project_runtime` must not treat copied files, chat resources, registry rows, graph rows, or derivatives as exposure authority
- `project_runtime` must not accept UI-triggered import or derivation as an exposure substitute
- prepared tree state must remain manifest-driven only

Legacy filesystem listing may remain for fixture or migration tests only until a later migration removes or isolates it from production authority.

## F13.4 manifest exposure contract boundary

`F13.4` is SPEC-only and does not modify `project_runtime`.

Future `project_runtime` may consume manifest-governed exposure only through a later explicit implementation slice that reads a governed `ManifestExposureEntry` shape from `project_manifest.json`.

Rules:

- `project_runtime` must treat `ManifestExposureEntry` as the minimum future exposure authority record
- `project_runtime` must not infer exposure from intake history/index, filesystem presence, blob presence, registry rows, graph rows, or `System View` visibility
- `project_runtime` must preserve `owner_ref`, `trace_ref`, source intake linkage, and duplicate-safe logical identity when preparing any future tree state
- `project_runtime` must not treat accepted human confirmation as exposure unless a later explicit runtime slice also writes manifest authority
- `project_runtime` must not create registry rows, graph rows, derivatives, or `document_text_runtime` side effects as part of future exposure by convenience

## F13.5A manifest exposure runtime plan boundary

`F13.5A` is SPEC-only and does not modify `project_runtime`.

Future `F13.5B` runtime ownership is reserved to `project_runtime`.

Boundary rules:

- `project_runtime` is the only approved future manifest writer boundary
- `project_runtime` may expose one already `imported_not_exposed` item per minimal runtime invocation
- `project_runtime` must require `ExposureRequest`, `ExposureCandidate`, accepted `HumanConfirmation`, non-stale candidate state, `owner_ref`, and `trace_ref`
- `project_runtime` must reject blocked items, unsupported items, stale candidates, rejected confirmations, and duplicate-policy violations
- `project_runtime` must not generate `registry.json`, graph writes, derivatives, or `document_text_runtime` side effects as part of manifest exposure

---

### !REL_FILE!

# project_settings_popup

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Project settings` popup as the project-scoped configuration surface in DocGraph.

## Core rule

`Project settings` is configuration only.

It does not execute.
It does not resolve policy.
It does not trigger tools.
It does not mutate runtime directly.

## Scope

`Project settings` may capture project-scoped configuration such as:

- `project_profile`
- declarative sandbox references
- optional tool defaults
- project metadata

## Forbidden contents

`Project settings` MUST NOT contain:

- raw credentials
- secrets
- binary references
- execution toggles
- host-specific absolute paths

## Portability rule

Project-scoped settings must remain portable across machines.

They may live inside project structure only when represented as declarative portable configuration.

## Popup contract

The popup is independent.

It is not a tab inside a shared Preferences panel.

The popup captures:

- project-scoped configuration draft
- user confirmation intent

The popup emits:

- `ConfigurationIntent`

The popup MUST NOT emit:

- `ExecutionIntent`

## Relationship to credentials

`Project settings` is not a credentials surface.

Credential references may be connected conceptually in future governed flows, but credential management remains external to this popup.

## Invariants

- `INV-PROJ-SETTINGS-001`: `Project settings` MUST remain a dedicated popup surface.
- `INV-PROJ-SETTINGS-002`: `Project settings` MUST remain project-scoped configuration only.
- `INV-PROJ-SETTINGS-003`: `Project settings` MUST NOT contain secrets or raw credentials.
- `INV-PROJ-SETTINGS-004`: `Project settings` MUST remain portable across machines.
- `INV-PROJ-SETTINGS-005`: `Project settings` MUST NOT trigger tools, LLM execution, external dependencies, or runtime mutation.

## Forbidden responsibilities

This spec must not:

- implement popup runtime
- define secret storage
- enable tool execution
- enable LLM execution
- mutate filesystem
- open F10

## Related specs

- `docs/specs/ui_preferences_popups.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/credentials_policy.md`

---

### !REL_FILE!

# project_setup_popup

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the initial project setup popup and the later project settings popup for DocGraph.

The same governed popup may eventually operate in two modes:

- `mode = create`
- `mode = edit`

`Project Setup Popup` is used when creating a new project.  
`Project Settings Popup` is used when editing an already opened project.

## Core rule

The popup captures intent and configuration.

The popup does not validate as authority.
The popup does not write directly.
The popup does not execute tools.
The popup does not enable LLM.
The popup does not activate a real sandbox.

Project Setup may be guided by Static Mode Lume.

No LLM is required to create or configure a project.

It emits either a conceptual `ProjectSetupDraft` or a conceptual `ProjectSettingsDraft`.

## ProjectSetupDraft conceptual fields

- `project_name`
- `project_profile`
- `default_agent_profile`
- `allowed_operational_tools`
- `allowed_llm_tools_policy`
- `sandbox_policy`
- `llm_policy`
- `preferences`
- `credential_ref`
- `language`
- `help_level`
- `trace_metadata`

## ProjectSettingsDraft conceptual fields

The `ProjectSettingsDraft` includes the same conceptual fields, but targets an already opened project and future governed update flow.

`preferences` may be included as non-secret configuration.

`credential_ref` may be included only as a conceptual identifier. Secret values are not included.

`llm_policy` remains optional and future-facing.

## Editable fields

Low risk:

- visible project name
- language
- theme
- Lume help level
- declarative default agent profile

Medium risk:

- allowed tools
- declarative LLM policy
- sandbox readonly / dry-run policy
- project preferences

High risk / future:

- credentials
- cloud LLM
- mutable sandbox
- write-back to external folder
- tool execution

## Confirmation rules

- Low-risk changes may be saved through a future governed flow.
- Medium-risk changes require confirmation.
- High-risk changes are future proposal or `BLOCKED` in F9.
- Credentials must not be stored in `project_config.json`.
- No credential validation is performed in F9.
- Changing configuration does not enable real execution by itself.

## Forbidden responsibilities

The popup must not:

- write files directly
- mutate `project_manifest.json` directly
- execute tools
- execute LLM
- decide permissions
- validate as authority
- create a real sandbox
- mutate user folders
- reinterpret the manifest
- duplicate `project_runtime`
- store secrets in plain text
- use absolute paths as portable truth

## Related specs

- `docs/specs/credentials_policy.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/project_profiles.md`
- `docs/specs/project_folder_layout.md`
- `docs/specs/local_sandbox_context.md`
- `docs/specs/tools_panel.md`
- `docs/specs/tool_implementation_governance.md`
- `docs/specs/action_resolution.md`
- `docs/specs/lume_interaction_model.md`

---

### !REL_FILE!

# quad_generation_policy

## Status

DECLARED_ONLY / F9-F10_PREP governance.

This document does not implement runtime quad generation, LLM invocation, tool execution, filesystem mutation, semantic approval, RDF projection runtime, or graph-analysis runtime.

## Purpose

Define a governed policy for when and how semantic quads may be generated in the future.

The goal is to prevent uncontrolled semantic explosion while preserving traceability, storage discipline, and security boundaries.

Quad generation is optional and trigger-bound.

It is not mandatory for all governed sources.

## Allowed generation triggers

Allowed triggers are:

- `user_requested`
- `artifact_analysis_requested`
- `chat_analysis_requested`
- `tool_output_analysis_requested`
- `system_suggested` ^(future only^)

Rules:

- quad generation must not occur automatically without an explicit governed trigger
- `system_suggested` remains future-only and does not activate generation by itself
- trigger identity must remain traceable in the resulting `QuadInstance` or equivalent future generation trace

## Generation scope

Generation scope must be bounded explicitly.

Allowed bounded scopes include:

- per `StoredObject`
- per `source_kind`
- per session
- per request

Rules:

- generation scope must be declared before future generation begins
- scope must not silently widen from one object to unrelated sources
- scope must not imply whole-project semantic expansion by default
- `project_manifest` exposure and governed source references remain authoritative for eligible sources

## Generation modes

Conceptual future generation modes are:

- `minimal`
- `balanced`
- `exhaustive` ^(future, restricted^)

### minimal

`minimal` means only key facts and trace-critical relations should be proposed.

This mode is the safest future default if generation is later opened.

### balanced

`balanced` means a broader but still governed semantic proposal set that stays within bounded scope and explicit trigger.

### exhaustive

`exhaustive` is future-only and restricted.

It must not be assumed available by default and must not be opened implicitly by this policy.

## Generation boundaries

Generation must respect all existing layer and governance boundaries:

- storage remains root authority
- derivatives remain deterministic and non-semantic
- semantic generation must not mutate storage or derivatives
- semantic generation must not imply approval
- graph and RDF remain downstream consumers only

Generation must also respect:

- invalidation policy
- security and sanitization policy
- source-scope governance
- lifecycle governance

## Traceability

Every future generation event must remain traceable at minimum through:

- trigger kind
- governed source reference
- generation scope
- generation mode
- resulting `quad_id` / `quad_instance_id` linkage
- trace reference

Traceability does not imply approval or execution authority.

## Non-goals

This policy does not open:

- runtime generation
- automatic background semantic extraction
- LLM execution
- tool execution
- semantic approval
- RDF projection runtime
- graph-analysis runtime
- execution slices

## Quad generation invariants

- `INV-GEN-001`: quad generation must not occur without an explicit governed trigger.
- `INV-GEN-002`: quad generation is optional and must not be treated as mandatory for all sources.
- `INV-GEN-003`: generation scope must remain bounded per governed object, source kind, session, or request.
- `INV-GEN-004`: uncontrolled or silent whole-project quad generation is forbidden.
- `INV-GEN-005`: generation must be traceable.
- `INV-GEN-006`: generation must respect storage-layer and derivative-layer boundaries.
- `INV-GEN-007`: generation must respect security and sanitization policy.
- `INV-GEN-008`: generated quads remain proposals by default and do not become approved knowledge automatically.
- `INV-GEN-009`: `system_suggested` generation remains future-only and non-active unless a later governed runtime opens it.
- `INV-GEN-010`: exhaustive generation remains future-only and restricted.

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/semantic_source_scope.md`
- `docs/specs/storage_policy.md`
- `docs/specs/invalidation_policy.md`
- `docs/specs/security_sanitization_policy.md`

---

### !REL_FILE!

# rdf_projection_policy

## Status

PROPOSAL / F9/F10_PREP declarative governance.

This document does not implement RDF runtime, Oxigraph, SPARQL, N-Quads persistence, TriG persistence, RDF file generation, graph analysis runtime, or execution authority.

## Purpose

Define RDF as a future projection layer over the internal governed semantic graph.

The internal source model remains:

- `StoredObject`
- `SemanticQuad`
- `QuadInstance`
- `QuadRelation`
- lifecycle state
- evidence
- trace

RDF is explicitly non-authoritative.

## RDF is projection, not authority

Conceptual direction:

`Internal governed semantic model -> future RDF projection`

Rules:

- RDF does not replace `StoredObject`
- RDF does not replace `SemanticQuad` or `QuadInstance`
- RDF does not replace `QuadRelation`
- RDF does not govern lifecycle
- RDF does not approve facts
- RDF is never the source of truth

## Eligibility rules

Only these may be projected in the future:

- `APPROVED` `QuadInstance`
- `APPROVED` `QuadRelation`

These must be excluded:

- `PROPOSED`
- `UNDER_REVIEW`
- `REJECTED`
- `STALE`
- `SUPERSEDED`
- `ORPHANED`

Rules:

- `PROPOSED` material must never be projected as factual RDF
- lifecycle filtering must not be bypassed
- approval must remain explicit and traceable

Lifecycle ownership and filtering rules remain governed by `semantic_quad_lifecycle.md`.

## Named graph mapping

Future named graph policy:

- `domain_knowledge`
- `document_governance`
- `ai_governance`
- `system_governance`

Rules:

- named graphs separate semantic facts, provenance, AI trace, and system governance
- graph identity is part of semantic identity
- lifecycle and provenance data belong in governance graphs
- no secrets may appear in any named graph

## RDF identity mapping

Future conceptual mapping:

- `quad_id -> stable semantic IRI`
- `quad_instance_id -> provenance_or_review IRI`
- `relation_id -> relation IRI`
- `object_ref -> source object IRI`
- `file_ref -> content-addressed source IRI`

Rules:

- no raw absolute host paths in RDF
- no secrets in IRIs
- identifiers must be stable and portable
- RDF identity mapping is declarative only

## Quad projection

Conceptual projection:

`SemanticQuad(subject, predicate, object, graph) -> RDF quad`

```text
<subject> <predicate> <object> <graph> .
```

Rules:

- projection may occur only from an `APPROVED` `QuadInstance`
- projection must preserve linkage to `quad_id` and `quad_instance_id`
- projection must preserve source and evidence linkage through governance graphs
- raw evidence text should not be duplicated

## Relation projection

`QuadRelation` becomes a future RDF relationship triple or quad.

Example conceptual mapping:

```text
<quad_instance_A> <docgraph:supports> <quad_instance_B> <graph:domain_knowledge> .
```

Rules:

- `relation_type` maps to a governed RDF predicate
- only `APPROVED` `QuadRelation` may be projected
- relation lifecycle must be preserved in governance graph
- `contradicts` does not auto-resolve conflict

## Evidence and provenance mapping

Future provenance links may reference:

- `source_ref`
- `object_ref`
- `file_ref`
- `chunk_id`
- `chunk_hash`
- `metadata_snapshot_hash`
- `message_ref`
- `artifact_ref`
- `tool_run_ref`
- `trace_ref`

Rules:

- evidence is referenced, not duplicated as raw text
- provenance graphs must preserve enough metadata for audit
- no secret-bearing metadata may be projected
- sanitized inputs must remain sanitized in projection and must not be re-expanded

## Dataset boundary

Future dataset boundary:

- each DocGraph project may have its own future RDF dataset
- global RDF aggregation remains future-only
- RDF dataset does not replace `project_manifest`
- RDF dataset does not infer project exposure

## Future formats

Future projection targets may include:

- `N-Quads`
- `TriG`

But:

- no files are generated now
- no RDF store is created now
- no Oxigraph dependency is enabled now
- no SPARQL endpoint or query runtime exists now

Future graph analysis may consume projected RDF only as a derived input surface. RDF still remains projection, not authority.

## RDF Projection Invariants

- `INV-RDF-001`: RDF is projection, not authority.
- `INV-RDF-002`: only `APPROVED` `QuadInstance` records may be projected.
- `INV-RDF-003`: only `APPROVED` `QuadRelation` records may be projected.
- `INV-RDF-004`: lifecycle filtering must not be bypassed.
- `INV-RDF-005`: RDF must preserve traceability.
- `INV-RDF-006`: named graphs separate concerns.
- `INV-RDF-007`: RDF must not contain secrets.
- `INV-RDF-008`: RDF must not contain raw absolute host paths.
- `INV-RDF-009`: RDF datasets do not replace `project_manifest`.
- `INV-RDF-010`: Oxigraph, SPARQL, and N-Quads persistence remain inactive.
- `INV-RDF-011`: RDF projection does not imply graph analysis runtime.
- `INV-RDF-012`: RDF projection does not approve or infer knowledge.
- `INV-RDF-013`: RDF projection must preserve sanitization boundaries and must not expose private absolute host paths, secret values, or unsanitized personal data.

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/artifact_graph.md`
- `docs/specs/storage_policy.md`
- `docs/specs/security_sanitization_policy.md`

---

### !REL_FILE!

# runtime_layout

## Purpose

Document the preliminary runtime layout for future external tools, LLM engines, models, cache, and temporary data.

This layout is declarative and materialized as empty or placeholder folders only.

## Scope F9

F9 may declare these runtime roots:

- `runtime/external`
- `runtime/engines`
- `runtime/models`
- `runtime/cache`
- `runtime/temp`
- `user/runtime/external_dependencies`

The minimal external runtime placeholder is Tectonic under `runtime/external/tectonic/`.

Tectonic is not installed. No binary is downloaded or faked.

`runtime/external` remains the current declarative placeholder root used by existing F9 validation and placeholder manifests.

`user/runtime/external_dependencies/` is the governed user-managed location for future external dependency references in the Tools menu model.

Declaring that user-managed location does not:

- install dependencies
- execute binaries
- replace `resources/tool_runtime/*`
- open F10

## Forbidden responsibilities

The runtime layout must not:

- execute external binaries
- install dependencies
- create tool calling
- run LLM engines
- store user project truth
- replace `resources/` declarations
- duplicate `tool_runtime`
- treat user-managed dependency presence as execution authority

## Future F10/F11 notes

F10 may resolve declared engines, models, and external tools through governed policy.

F11 should audit checksums, install status, and explicit execution gates before any real runtime execution is accepted.

---

### !REL_FILE!

# sandbox_boundary_model

## Status

DECLARED_ONLY / F9-F11_PREP governance.

This document does not implement sandbox enforcement runtime, filesystem mutation, host access runtime, network access runtime, tool execution, or authorization.

## Purpose

Define a strict governed boundary model separating:

- `SANDBOX`
- `HOST`
- `EXTERNAL`

This model prepares future action constraints without enabling enforcement or execution now.

## Domains

Controlled domains:

- `SANDBOX`
- `HOST`
- `EXTERNAL`

### SANDBOX

Governed local app or project environment.

### HOST

Filesystem and local environment outside the governed sandbox.

### EXTERNAL

Network, internet, APIs, and remote services.

Rules:

- domain must be explicit when an action, tool, or capability needs access
- unspecified domain must not imply permission
- domain declaration does not authorize access

## SANDBOX domain

`SANDBOX` is the only future writable domain by default.

It may contain:

- `user/file_store/`
- project workspace
- `user/output/`
- future tool outputs

Rules:

- write access is future-governed and not active now
- delete access remains separately governed
- sandbox access does not imply project exposure
- `project_manifest` remains exposure authority

## HOST domain

`HOST` is anything outside the governed sandbox on the local machine.

Rules:

- no implicit host access
- future host access is read-only by default
- host write access is forbidden by default
- `HOST -> SANDBOX` import may be allowed only as an explicit future governed operation
- `SANDBOX -> HOST` export is forbidden by default unless separately gated

## EXTERNAL domain

`EXTERNAL` is network, internet, and remote API access.

Rules:

- no implicit network access
- future access requires `access_network` capability
- external access is disabled in the current phase
- credentials or secrets must not be inferred from environment
- no telemetry upload is implied

## Capability-domain alignment

Capability-domain alignment is governed conceptually as follows:

- `read_file`: `SANDBOX`, or controlled future `HOST` read
- `write_file`: `SANDBOX` only in a future governed phase
- `create_file`: `SANDBOX` only in a future governed phase
- `delete_file`: `SANDBOX` only and separately gated in a future phase
- `access_network`: `EXTERNAL` only, disabled now
- `call_external_binary`: future `SANDBOX`-scoped operation
- `call_llm`: provider boundary remains separately gated
- `generate_semantic_quads`: semantic-layer concern only; no filesystem mutation by itself
- `project_rdf`: future-only, non-active
- `analyze_graph`: future-only, non-active

Rules:

- capability declaration must include domain assumptions where relevant
- capability cannot expand domain by itself
- domain policy may further restrict capability use

## Action constraints alignment

Future `ActionIntent`, `ActionRequest`, or plan steps should declare:

- `required_domain`
- `access_level`
- `sandbox_scope`
- `host_access`
- `network_access`

Conceptual example:

```json
{
  "constraints": {
    "required_domain": "SANDBOX",
    "access_level": "read_write",
    "sandbox_scope": "project_only",
    "host_access": "none",
    "network_access": false
  }
}
```

Rules:

- constraints are declarative in this phase
- constraints are not enforced now
- constraints do not authorize execution
- `ActionIntent` and `ActionRequest` may carry these constraints in the future without becoming executable
- future `ResolutionCandidate` artifacts may summarize domain evaluation over these constraints without granting access
- future `AuthorizedExecutionRequest` artifacts may require `SANDBOX` scoped execution as a contract while still not granting runtime access
- future `SingleToolExecution` artifacts may declare `SANDBOX`-only first execution scope, but that declaration still does not grant runtime access or mutate filesystem
- future readonly presentation may summarize these constraints through `docs/specs/system_view.md` without granting permission

## F12.0 / F11.RUNTIME-0 sandbox proposal note

`F12.0 / F11.RUNTIME-0` is proposal-only.

The first possible runtime opening is constrained to one local deterministic `SANDBOX`-only tool.

It must not open:

- `HOST` write
- `EXTERNAL`
- network
- provider invocation
- external binary invocation
- multi-tool orchestration
- autonomous execution

`SANDBOX` scope in this proposal does not by itself grant filesystem mutation authority.

Any future output must be owner-scoped and governed by a later explicit implementation slice.

## Import/export model

Definitions:

- `Import`: explicit future operation bringing data from `HOST` to `SANDBOX`
- `Export`: explicit future operation sending data from `SANDBOX` to `HOST` or `EXTERNAL`

Rules:

- imports must be traceable
- exports are forbidden by default
- no implicit file copy is allowed
- no runtime import/export is implemented now

## Storage alignment

- `file_store` lives inside `SANDBOX`
- `file_store/blobs` remains physical authority
- `project_manifest` remains exposure authority
- host files are not part of `file_store` until explicitly imported
- external resources are not `StoredObject` values until explicitly imported or ingested in a future governed phase

## Non-goals

This policy does not open:

- filesystem mutation
- host write access
- network access
- tool execution
- execution layer
- active sandbox enforcement

## Sandbox boundary invariants

- `INV-SANDBOX-001`: `SANDBOX` is the only future writable domain by default.
- `INV-SANDBOX-002`: there is no implicit `HOST` access.
- `INV-SANDBOX-003`: there is no implicit `EXTERNAL` or network access.
- `INV-SANDBOX-004`: `HOST` write is forbidden by default.
- `INV-SANDBOX-005`: `EXTERNAL` access requires explicit future capability and gate.
- `INV-SANDBOX-006`: capability declaration does not expand domain access.
- `INV-SANDBOX-007`: future `ActionIntent` and `ActionPlan` constraints must declare domain needs.
- `INV-SANDBOX-008`: `project_manifest` remains exposure authority.
- `INV-SANDBOX-009`: import and export are future governed operations and are not active behavior.
- `INV-SANDBOX-010`: sandbox policy does not activate enforcement runtime.

## Related specs

- `docs/specs/local_sandbox_context.md`
- `docs/specs/tool_capability_model.md`
- `docs/specs/action_resolution.md`
- `docs/specs/storage_policy.md`
- `docs/specs/system_view.md`

---

### !REL_FILE!

# sandbox_viewer

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Sandbox Viewer` surface for DocGraph.

The sandbox is represented as an inspectable governed object that may be selected from `Document Tree` and opened in `Workspace Tabs`.

## Role

`Sandbox Viewer` is inspection of sandbox context.

It is not:

- an executor
- a filesystem explorer
- a write-back UI

It does not execute tools.
It does not mutate files.
It does not perform write-back.

## Core rule

`Sandbox Viewer` renders prepared governed sandbox state only.

It does not discover arbitrary host filesystem state by itself.
It does not authorize mutation.
It does not replace runtime policy.

## Conceptual content

Prepared sandbox-view state may include:

- `sandbox_ref`
- `inputs`
- `scope`
- selected source-folder metadata, without absolute path as portable truth
- `working_copy_future`
- proposed operations, future
- capability state
- readonly status

Interpretation:

- `sandbox_ref`: governed sandbox identity
- `inputs`: declared or prepared sandbox-related inputs
- `scope`: governed sandbox scope boundary
- selected source-folder metadata: descriptive context about the selected source folder without persisting host absolute path as portable authority
- `working_copy_future`: future working-copy representation only
- proposed operations, future: non-executable descriptions of possible later governed actions
- capability state: readonly summary of currently available or unavailable sandbox capabilities
- readonly status: explicit declaration that the current viewer is non-mutating in F9

## Allowed actions

The viewer may allow:

- inspect
- copy a structured reference
- send sandbox context to chat
- view status

These are inspection and reference actions only.

## Forbidden actions

The viewer must not allow:

- execute filesystem operations
- mutate folders
- write to the original source
- launch tools directly
- apply changes
- perform write-back

## Relationship to ActiveObjectContext

- `selected_sandbox` may hold the currently selected sandbox reference
- `focused_family = Sandbox` when sandbox activation is explicit and observable
- chat may use sandbox context when `Sandbox` is the focused family and a valid selection exists

The viewer must not invent sandbox context when no governed `selected_sandbox` exists.

## Relationship to Document Tree

`Document Tree` may expose sandbox context as governed node.

Selecting that node may prepare `selected_sandbox` and request opening or activation of `Sandbox Viewer`.

The tree does not turn sandbox context into a free folder browser.

## Relationship to Workspace Tabs

`Sandbox Viewer` is hosted inside `Sandbox Viewer Tab`.

Opening the same `sandbox_ref` must reuse the existing governed tab rather than duplicate it.

The tab hosts the view; it does not execute sandbox behavior.

## Relationship to local sandbox context

This viewer consumes the policy and context declared in `docs/specs/local_sandbox_context.md`.

The original selected source folder remains readonly.

Any future mutation must remain limited to future governed working-copy scope.

## States

Prepared viewer state may include:

- `declared`
- `readonly`
- `missing`
- `invalid`
- `future_actionable`
- `stale`

Interpretation:

- `declared`: sandbox context is declared and available as governed inspection state
- `readonly`: sandbox context is visible but non-mutable in current phase
- `missing`: `sandbox_ref` no longer resolves
- `invalid`: prepared sandbox state is not valid for governed rendering
- `future_actionable`: sandbox may become actionable only in a future governed phase
- `stale`: prepared sandbox context no longer safely matches current declared state

## Failure modes

- `sandbox_ref_missing`
- `source_folder_unavailable`
- `absolute_path_persistence_detected`
- `mutation_requested_in_f9`
- `writeback_blocked`

Interpretation:

- `sandbox_ref_missing`: the selected governed sandbox reference no longer resolves
- `source_folder_unavailable`: descriptive source-folder context cannot be prepared
- `absolute_path_persistence_detected`: host absolute path is being treated as portable project truth
- `mutation_requested_in_f9`: a mutation attempt was requested through a non-mutating F9 viewer
- `writeback_blocked`: a write-back attempt was requested even though write-back is not allowed here

## Invariants

- `INV-SVIEW-001`: `Sandbox Viewer` MUST remain inspection-only in F9
- `INV-SVIEW-002`: `Sandbox Viewer` MUST NOT become a filesystem explorer
- `INV-SVIEW-003`: `Sandbox Viewer` MUST NOT execute filesystem operations
- `INV-SVIEW-004`: selected source-folder metadata MUST NOT persist host absolute paths as portable truth
- `INV-SVIEW-005`: sandbox chat context MAY be referenced only from valid governed sandbox selection
- `INV-SVIEW-006`: opening the same `sandbox_ref` MUST reuse the same governed tab identity

## Related specs

- `docs/specs/local_sandbox_context.md`
- `docs/specs/workspace_tabs.md`
- `docs/specs/active_object_context.md`
- `docs/specs/document_tree.md`
- `docs/specs/ui_core.md`

---

### !REL_FILE!

# security_sanitization_policy

## Status

DECLARED_ONLY / F9-F10_PREP governance.

This document does not implement runtime sanitization, encryption, secret scanning, key management, provider execution, tool execution, filesystem mutation, or RDF runtime.

## Purpose

Define governed security and sanitization rules for metadata, derivatives, semantic quads, relations, RDF projection, and trace records.

The goal is to prevent leakage of sensitive data across all declarative and future-derived layers.

## Sensitive data definition

Sensitive data includes, at minimum:

- credentials
- access tokens
- API keys
- bearer tokens
- secret values embedded in configuration or prompts
- private absolute host paths
- personal data when present

Personal data may include, when applicable:

- personal email addresses
- phone numbers
- government or institutional identifiers
- home addresses
- other directly identifying personal fields not required for governed traceability

## Forbidden locations

Sensitive data must not appear in:

- `StoredObject.metadata.semantic`
- `StoredObject.metadata.technical`
- `StoredObject.metadata.operational`
- deterministic derivatives
- semantic quads
- semantic relations
- RDF projections
- trace records
- graph-analysis artifacts
- storage indexes

The only allowed representation is a governed non-secret reference, redacted value, masked value, or stable hash when a trace need exists.

## Sanitization rules

Sanitization must be governed by the following strategies:

- redact full secret values when the field meaning must remain visible
- mask partial values only when the residual fragment does not reconstruct the secret
- hash values only when stable correlation is required without exposing the underlying value
- replace private absolute host paths with portable governed refs or sanitized placeholders
- minimize personal data to the least information necessary for governed traceability

### Redaction patterns

Allowed conceptual redaction patterns include:

- `[REDACTED_CREDENTIAL]`
- `[REDACTED_TOKEN]`
- `[REDACTED_API_KEY]`
- `[REDACTED_PERSONAL_DATA]`
- `[REDACTED_PRIVATE_PATH]`

Redaction must preserve type meaning without preserving the secret.

### Hashing rules

Hashing may be used when:

- a stable comparison key is required
- a duplicate-detection or trace-correlation purpose exists
- the unhashed value must remain hidden

Hashing must not be treated as semantic authority, approval, or execution permission.

### Masking rules

Masking may be used when:

- a human needs to distinguish multiple governed references
- the remaining visible fragment is non-sensitive

Masking must not expose enough material to reconstruct the original secret.

## Layer rules

### Storage and metadata

- metadata must not store credentials, token values, API keys, private absolute paths, or personal data beyond governed minimum need
- storage indexes must not leak secrets through duplicated metadata or path capture
- `file_ref`, `object_ref`, and governed ids remain preferred over raw host-specific path truth

### Deterministic derivatives

- derivatives must not duplicate secret-bearing source fragments unless a future governed exception explicitly opens that behavior
- structured derivative metadata must be sanitized before becoming portable declarative state

### Semantic layer

- semantic quads and relations must not contain secret values as subject, predicate, object, evidence payload, metadata, or trace material
- semantic evidence must reference governed sources rather than duplicate sensitive raw text
- no semantic proposal may treat secret-bearing material as approved knowledge

### RDF projection

- RDF projections must remain sanitized derived views only
- no secrets may appear in IRIs, literals, named graphs, provenance links, or governance graphs
- RDF must not re-expand previously sanitized material

### Traceability

- traceability is required, but traceability must not leak secrets
- trace refs, prompt refs, model refs, tool refs, and source refs are allowed only as non-secret identifiers
- human-review refs such as `reviewer_ref` are allowed only as non-secret identifiers and must not expose credentials or personal data beyond governed minimum need
- trace metadata must remain sufficient for audit without exposing underlying secret values
- future readonly presentation surfaces such as `docs/specs/system_view.md` must expose only sanitized references, summaries, and explanation state
- `FULL_TRACE` review presentation must remain sanitized and must not expose raw payloads, secrets, private absolute host paths, or redacted content
- `AuthorizedExecutionRequest.safety_snapshot` and owner-requirement metadata must remain sanitized summaries and must not include raw payloads, secrets, private absolute host paths, or credential material
- `SingleToolExecution.input_refs`, `output_plan`, result placeholders, trace refs, and future manifest metadata must remain sanitized summaries and must not include raw payloads, secrets, private absolute host paths, or credential material
- future `ToolRunManifest.inputs`, `configuration`, `outputs`, and future `TraceRecord.links` must remain sanitized and must not include raw payloads, secrets, or private absolute host paths

## F12.1 text.measure sanitization gate

`F12.1` is gate-only and creates no runtime sanitization.

For the later first `text.measure` implementation slice:

- accepted input must be a governed text input ref
- raw payloads are forbidden
- secrets are forbidden
- private absolute host paths are forbidden
- unsafe input must block future execution
- stale input must block future execution
- manifest metadata must remain sanitized
- trace metadata must remain sanitized

Future-only error codes include:

- `unsafe_input`
- `stale_input`

## F12.2A text.measure sanitization implementation plan

`F12.2A` is plan-only and creates no runtime sanitization.

Future `F12.2B` must enforce:

- governed text input only
- no raw payload persistence
- no secrets in input-derived metadata
- no private absolute host paths in manifest, result, or trace metadata
- unsafe input blocks execution
- stale input blocks execution if staleness is present
- `owner_ref` is mandatory
- `trace_ref` is mandatory
- manifest metadata remains sanitized
- trace metadata remains sanitized

Future `F12.2B` must not call providers, access network, invoke external binaries, run LLM tools, run agent tools, or introduce document/semantic/graph mutation.

## Non-goals

This policy does not open:

- encryption runtime
- secret vault runtime
- active credential resolution
- secret scanning runtime
- RDF runtime
- graph analysis runtime
- execution authority

## Security and sanitization invariants

- `INV-SEC-001`: no secrets may appear in metadata.
- `INV-SEC-002`: no secrets may appear in deterministic derivatives.
- `INV-SEC-003`: no secrets may appear in semantic quads or relations.
- `INV-SEC-004`: no secrets may appear in RDF projections.
- `INV-SEC-005`: no secrets may appear in trace records or storage indexes.
- `INV-SEC-006`: private absolute host paths must not become portable truth.
- `INV-SEC-007`: sanitization must preserve traceability without exposing secret values.
- `INV-SEC-008`: hashing, masking, and redaction do not imply authority, approval, or execution permission.
- `INV-SEC-009`: semantic and RDF layers must not re-expand previously sanitized material.
- `INV-SEC-010`: personal data must be minimized and sanitized when not strictly required for governed traceability.

## Related specs

- `docs/specs/storage_policy.md`
- `docs/specs/semantic_quad_model.md`
- `docs/specs/rdf_projection_policy.md`
- `docs/specs/graph_analysis_policy.md`
- `docs/specs/system_view.md`

## F12.4 file intake sanitization alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

Future intake metadata must be sanitized and must not contain secrets, raw payloads, credentials, unsanitized user paths, or private absolute host paths as portable truth.

Original filenames must be sanitized before durable metadata use.

Source paths may be transient future runtime inputs only.

F12.4 intake comments are governed metadata and must be sanitized before durable persistence.

Comment sanitization must:

- remove or flag secrets
- remove or flag credentials
- remove or flag tokens
- remove or flag private absolute host paths
- reject or flag raw sensitive data according to active policy
- enforce maximum length
- enforce safe encoding
- reject executable content

Valid `comment_sanitization_state` values are:

- `safe`
- `flagged`
- `rejected`

Rejected comments block ingestion only when the active policy explicitly requires blocking.

## F12.5 file intake sanitization plan

`F12.5` is plan-only and adds no sanitization runtime.

Future `F12.6` must sanitize:

- original filenames
- source display labels
- intake metadata
- optional `user_comment`

Future `F12.6` metadata must not contain secrets, credentials, tokens, raw sensitive data, private absolute host paths as portable truth, or unsanitized user paths.

Future `F12.6` must use typed sanitization or blocked reason codes including `sanitization_failed`, `comment_contains_secrets`, `comment_contains_private_path`, `comment_too_large`, and `comment_sanitization_failed`.

---

### !REL_FILE!

# semantic_classification_policy

## Purpose

Define how semantic classification metadata is attached to governed objects without opening semantic runtime.

## Classification scope

Semantic classification metadata is limited to:

- `keywords`
- `system_tags`
- `usage_tags`

Classification is declarative only.

It does not:

- invoke LLM
- activate tags automatically
- mutate stored objects by itself
- generate quads
- approve knowledge
- open execution

## Integration rules

- `StoredObject.metadata.semantic` is the integration surface
- `system_tags` and `usage_tags` must resolve to `ACTIVE` catalog entries
- `DEPRECATED` catalog entries may remain readable for compatibility
- `PROPOSED`, `UNDER_REVIEW`, and `REJECTED` tags must not be used as active object metadata
- keywords follow the semi-controlled catalog policy in `semantic_tag_catalog.md`

## Catalog boundary

- the tag catalog is authority for controlled tags
- schemas are declarative contracts only
- schema validity does not imply approval
- schema validity does not imply project exposure
- schema validity does not imply execution authority

## StoredObject boundary

Semantic classification does not make `StoredObject` semantic authority.

It only provides governed metadata slots for future interpretation.

## Invariants

- `INV-TAG-001`: controlled tags must exist in the catalog
- `INV-TAG-003`: tag families must not be mixed
- `INV-TAG-008`: documents reference `tag_id`, not meanings
- `INV-TAG-009`: free-form `system_tags` and `usage_tags` are forbidden
- `INV-TAG-010`: tag-based quads remain proposal-only unless separately lifecycle-approved

---

### !REL_FILE!

# semantic_layer_boundaries

## Purpose

Define strict boundaries between storage, derivatives, semantic, graph, and analysis layers.

This document is declarative only.

It does not implement runtime, execution, mutation engines, graph engines, semantic generation, or F10 execution slices.

## Layer definitions

- `storage`: physical data and governed logical storage references
- `derivatives`: deterministic transformations derived from storage-backed source material
- `semantic`: interpretative layer expressed as semantic quads and quad instances
- `graph`: explicit relationship layer between semantic quads or other governed artifact relations
- `analysis`: read-only evaluation layer over approved graph material

## Access model

| Layer | Read | Write | Invalidate |
|---|---|---|---|
| `storage` | yes | yes | triggers downstream invalidation |
| `derivatives` | yes | yes | triggers downstream invalidation |
| `semantic` | yes | yes | reacts to upstream invalidation |
| `graph` | yes | yes | reacts to upstream invalidation |
| `analysis` | yes | no | no |

Interpretation:

- storage is the root authority
- derivatives may be written only as deterministic transformations of governed source material
- semantic may write proposal-layer semantic material only
- graph may write explicit relation material only
- analysis is read-only and cannot write or invalidate any layer

## Invalidation flow

The invalidation direction is one-way:

```text
storage -> derivatives -> semantic -> graph
```

Rules:

- no reverse mutation is allowed
- no downstream layer may mutate an upstream layer
- invalidation in an upstream layer may stale or orphan downstream semantic or graph material
- analysis may observe invalid states but must not mutate them

## Cross-layer boundaries

### storage

- may be read by derivatives, semantic evidence resolution, graph traceability, and analysis inputs
- may trigger downstream invalidation
- remains the only physical authority

### derivatives

- may read governed storage-backed material
- may write deterministic, regenerable derivative artifacts
- must not write semantic claims
- may trigger downstream semantic invalidation when evidence anchors change

### semantic

- may read storage-backed sources and derivatives as evidence
- may write proposal-layer quads and quad instances only
- must not write back into derivatives or storage
- must react to upstream invalidation rather than repairing sources locally

### graph

- may read approved semantic material and governed trace references
- may write explicit relation edges only
- must not modify quads, derivatives, or storage
- must react to semantic invalidation rather than mutating semantic source material

### analysis

- may read approved graph and semantic material only
- must not write graph, semantic, derivatives, or storage
- must not invalidate other layers

## Invariants

- `INV-LAYER-001`: storage is root authority
- `INV-LAYER-002`: no semantic writes into derivatives
- `INV-LAYER-003`: graph cannot modify quads
- `INV-LAYER-004`: analysis cannot modify graph
- `INV-LAYER-005`: no reverse mutation is allowed across layer boundaries
- `INV-LAYER-006`: invalidation flows only from storage to derivatives to semantic to graph
- `INV-LAYER-007`: analysis is read-only evaluation only
- `INV-LAYER-008`: semantic and graph writes remain proposal or governed relation material only

## Related specs

- `docs/specs/storage_policy.md`
- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/artifact_graph.md`
- `docs/specs/graph_analysis_policy.md`

---

### !REL_FILE!

# semantic_proposal_layer

## Status

PROPOSAL / F9.5 declarative governance. This document does not implement LLM execution, embeddings, RDF persistence, Oxigraph, SPARQL, tools, document mutation, or runtime approval.

## Purpose

Define `SemanticProposal` as the governed representation for future AI-derived semantic suggestions.

A proposal is not a fact.

## Core model

Future semantic derivations may produce proposal-shaped records with:

- `proposal_id`
- `subject_ref`
- `predicate`
- `object_ref`
- `graph_hint`
- `source_ref`
- `prompt_ref`
- `semantic_spec_ref`
- `trace_ref`
- `confidence_label = mock_only`
- `review_state`
- `created_by`
- `execution_enabled = false`

In F9.5 these records are model/schema only.

## Review lifecycle

Allowed states:

- `Generated`
- `NeedsReview`
- `Approved`
- `Rejected`
- `Stale`
- `Expired`

Default state: `NeedsReview`.

No proposal may become `Approved` without explicit human review in a future governed flow.

## Future graph hints

Future named graph hints:

- `domain_knowledge`
- `system_governance`
- `ai_governance`
- `document_governance`
- `lume_policy`

Graph hints do not create RDF, persist triples, or activate Oxigraph.

## Disabled future flags

- `oxigraph_enabled = false`
- `rdf_export_enabled = false`
- `semantic_store_enabled = false`
- `execution_enabled = false`

## Lume role

If Lume appears in future graph data, it must be represented as `InteractionLayer`.

Lume must not be represented as `Executor`, `Runtime`, `ToolRunner`, `LLMProvider`, or `FilesystemOwner`.

## Invariants

- `INV-SEM-001`: proposal is not fact. Every semantic output generated in F9.5 must be represented as `SemanticProposal`; it is not approved knowledge.
- `INV-SEM-002`: no automatic approval. No semantic proposal may move to `Approved` without explicit human decision.
- `INV-SEM-003`: every node must be referenceable through a stable identifier.
- `INV-SEM-004`: every proposed relation must preserve document/chunk origin, `prompt_ref`, `semantic_spec_ref`, and `trace_ref`.
- `INV-SEM-005`: future export does not imply persistence. `FutureGraphRef`, `graph_hint`, or N-Quads do not create RDF or activate Oxigraph.
- `INV-SEM-006`: Oxigraph is future, not current runtime. In F9.5 `oxigraph_enabled`, `rdf_export_enabled`, and `semantic_store_enabled` are false.
- `INV-SEM-007`: the graph does not decide, approve, execute, or mutate actions.
- `INV-SEM-008`: ontology proposes context, not authority. It does not replace policy, `ActionResolution`, human review, or authorized runtime.
- `INV-SEM-009`: named graph separation distinguishes `domain_knowledge`, `system_governance`, `ai_governance`, `document_governance`, and `lume_policy`.
- `INV-SEM-010`: governance is also knowledge, but it remains declarative.
- `INV-SEM-011`: Lume is not an executor. In graph terms Lume is `InteractionLayer`, never `Executor`, `Runtime`, `ToolRunner`, `LLMProvider`, or `FilesystemOwner`.
- `INV-SEM-012`: every `SemanticProposal` must have lifecycle state.
- `INV-SEM-013`: there is no hidden UI semantics. UI represents proposals, relations, and traces; it does not infer, approve, or create facts.
- `INV-SEM-014`: the semantic layer does not duplicate the project pipeline. It consumes governed documents, chunks, and references without reinterpreting manifests or paths.
- `INV-SEM-015`: prepare RDF/Oxigraph without premature implementation: no Oxigraph dependency, store, SPARQL, embeddings, or LLM.

## Related specs

- `docs/specs/ai_governance_f9_5.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/action_resolution.md`
- `docs/specs/pending_action_state.md`


---

### !REL_FILE!

# semantic_quad_lifecycle

## Status

PROPOSAL / F9/F10_PREP declarative governance.

This document does not implement review runtime, approval runtime, graph runtime, RDF runtime, Oxigraph, SPARQL, LLM execution, tool execution, filesystem mutation, or execution authority.

## Purpose

Define the governed lifecycle model for semantic quads and quad instances.

This lifecycle governs:

- proposal state
- review state
- approval state
- rejection
- evidence invalidation
- staleness
- orphaning
- supersession
- future graph, RDF, and analysis eligibility

Associated lifecycle schemas are declarative contracts only.

Schema validity does not approve quads or relations and does not activate lifecycle runtime.

## Relationship to semantic_quad_model

`semantic_quad_model.md` owns:

- `SemanticQuad`
- `QuadInstance`
- `quad_id`
- `quad_instance_id`
- source and evidence structure

This document owns lifecycle interpretation only.

Lifecycle ownership decision:

- lifecycle attaches primarily to `QuadInstance`
- `SemanticQuad` identity remains stable
- multiple `QuadInstance` records may share one `quad_id` with different lifecycle states

Approved knowledge eligibility depends on an approved `QuadInstance`, not on `quad_id` alone.

## Lifecycle states

Normalized states:

- `PROPOSED`
- `UNDER_REVIEW`
- `APPROVED`
- `REJECTED`
- `STALE`
- `SUPERSEDED`
- `ORPHANED`

### PROPOSED

- default state
- generated or declared but not validated
- not usable as approved knowledge

### UNDER_REVIEW

- being evaluated
- not yet approved
- not usable as graph or RDF fact

### APPROVED

- explicitly accepted by future governed review
- eligible for future graph, RDF, or analysis projection
- never automatic

### REJECTED

- evaluated and rejected
- must not be used as approved knowledge

### STALE

- evidence changed or became outdated
- requires review before reuse

### SUPERSEDED

- replaced by a newer or better quad instance
- not current approved knowledge

### ORPHANED

- evidence anchor no longer resolves
- requires repair, review, or rejection

## Allowed transitions

Allowed transitions:

- `PROPOSED -> UNDER_REVIEW`
- `PROPOSED -> REJECTED`
- `UNDER_REVIEW -> APPROVED`
- `UNDER_REVIEW -> REJECTED`
- `APPROVED -> STALE`
- `APPROVED -> SUPERSEDED`
- `STALE -> UNDER_REVIEW`
- `STALE -> REJECTED`
- `ORPHANED -> UNDER_REVIEW`
- `ORPHANED -> REJECTED`

Transition rules:

- `APPROVED` must never be reached automatically
- every transition must be explicit and traceable
- no silent lifecycle transitions are allowed
- generation does not imply approval
- lifecycle state must remain separate from generation metadata

## Evidence invalidation

Invalidation rules:

- `chunk_hash` mismatch -> `STALE`
- `chunk_id` missing -> `ORPHANED` or `STALE`
- `source_ref` missing -> `ORPHANED`
- `metadata_snapshot_hash` mismatch -> `STALE`
- artifact version mismatch -> `STALE`
- `tool_run_ref` missing -> `ORPHANED`

Invalidation rules mean:

- invalidated evidence must not remain silently `APPROVED`
- invalidation marks the quad instance for review
- invalidation does not auto-reject
- invalidation does not mutate source documents
- invalidation reacts to upstream storage or derivative changes and does not flow in reverse
- lazy invalidation is the current declared mode
- eager invalidation remains future-only and separately gated

## Supersession model

Supersession is explicit.

Conceptual fields:

- `supersedes: [quad_instance_id]`
- `superseded_by: [quad_instance_id]`
- `supersession_reason`
- `trace_ref`

Rules:

- supersession must be explicit
- old instance becomes `SUPERSEDED`
- new instance may be `PROPOSED` or `APPROVED` depending on future review
- supersession does not erase history

## Traceability

Every lifecycle transition must record:

- `actor_kind: human | system | future_governed_process`
- `actor_ref`
- `timestamp`
- `previous_state`
- `next_state`
- `reason`
- `trace_ref`

Rules:

- actor metadata must not contain secrets
- `trace_ref` must not imply execution
- lifecycle trace is audit metadata, not authority

## Future graph, RDF, and analysis eligibility

Future eligibility rules:

- graph may consume only `APPROVED` `QuadInstance` records
- graph may consume only `APPROVED` `QuadRelation` edges
- RDF projection may consume only `APPROVED` `QuadInstance` records
- RDF projection may consume only `APPROVED` `QuadRelation` edges
- graph analysis may consume only `APPROVED` `QuadInstance` records and approved relations
- `PROPOSED`, `UNDER_REVIEW`, `REJECTED`, `STALE`, `SUPERSEDED`, and `ORPHANED` are excluded from approved-knowledge projections

Boundaries:

- no graph runtime is opened
- no RDF projection runtime is opened
- no analysis runtime is opened

RDF projection filtering rules are further governed by `rdf_projection_policy.md`.

Graph analysis filtering rules are further governed by `graph_analysis_policy.md`.

## Lifecycle boundary

This document does not open:

- review runtime
- approval runtime
- graph runtime
- RDF projection runtime
- analysis runtime
- execution runtime

Lifecycle remains declarative governance only in `F9/F10_PREP`.

## Semantic Quad Lifecycle Invariants

- `INV-LC-001`: semantic quads are `PROPOSED` by default.
- `INV-LC-002`: `APPROVED` requires explicit governed review.
- `INV-LC-003`: generation never implies approval.
- `INV-LC-004`: lifecycle transitions must be traceable.
- `INV-LC-005`: evidence invalidation affects lifecycle.
- `INV-LC-006`: invalidated approved instances cannot remain silently approved.
- `INV-LC-007`: supersession must be explicit.
- `INV-LC-008`: lifecycle is separate from generation metadata.
- `INV-LC-009`: future graph, RDF, and analysis consume only `APPROVED` instances.
- `INV-LC-010`: no review runtime, approval runtime, RDF runtime, graph runtime, or execution runtime is opened.
- `INV-LC-011`: lifecycle attaches primarily to `QuadInstance`, not to `quad_id` alone.
- `INV-LC-012`: the same `quad_id` may have approved and non-approved instances simultaneously without collapsing lifecycle history.
- `INV-LC-013`: orphaned evidence must not be silently treated as valid support.
- `INV-LC-014`: stale evidence must not be silently treated as current approved knowledge.
- `INV-LC-015`: future approved semantic relation edges must follow the same non-automatic review discipline as approved quad instances.
- `INV-LC-016`: semantic lifecycle invalidation reacts to upstream layers and must not mutate them.

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_source_scope.md`
- `docs/specs/semantic_proposal_layer.md`
- `docs/specs/artifact_graph.md`
- `docs/specs/storage_policy.md`
- `docs/specs/invalidation_policy.md`

---

### !REL_FILE!

# semantic_quad_model

## Status

PROPOSAL / F9/F10_PREP declarative governance.

This document does not implement runtime generation, LLM invocation, tool execution, RDF, Oxigraph, SPARQL, N-Quads persistence, filesystem mutation, or approval runtime.

## Purpose

Define the governed semantic quad model for DocGraph as a proposal-only semantic layer.

This model must support:

- deterministic semantic identity through `quad_id`
- generation-instance identity through `quad_instance_id`
- explicit `source_kind` and `source_ref`
- alignment with `StoredObject`
- evidence anchoring
- contextual `QuadSet` grouping
- lifecycle preparation
- future RDF projection readiness without enabling RDF runtime now

Associated JSON schemas are declarative contracts only.

Schema validity does not imply approval, project exposure, execution, RDF projection, or graph-analysis runtime.

## Identity model

### quad_id

`quad_id` is deterministic semantic identity.

It represents what is asserted, not how or when it was produced.

`quad_id` is derived from normalized:

- `subject`
- `predicate`
- `object`
- `graph`
- evidence anchor

`quad_id` must not include generation context.

### quad_instance_id

`quad_instance_id` is generation-instance identity.

It represents how, when, or by what governed process the quad instance was produced.

It may include or depend on:

- `prompt_ref`
- `model_ref`
- `trace_ref`
- generation timestamp
- other future generation metadata

Multiple instances may share one `quad_id`.

Reuse of the same `quad_id` is preferred over creation of duplicate semantic identity when governed meaning and evidence anchor are unchanged.

## SemanticQuad structure

Conceptual structure:

```json
{
  "quad_id": "q_xxx",
  "subject": "...",
  "predicate": "...",
  "object": "...",
  "graph": "domain_knowledge | document_governance | ai_governance | system_governance"
}
```

Rules:

- `graph` is part of semantic identity
- quads are semantic proposals by default
- quads do not live inside chunks
- quads do not live inside deterministic derivatives
- a `SemanticQuad` expresses the asserted meaning only
- semantic relations must not be embedded as child hierarchy inside `SemanticQuad`
- semantic relations must be modeled as explicit graph edges outside the quad structure

## QuadInstance structure

Conceptual structure:

```json
{
  "quad_instance_id": "qi_xxx",
  "quad_id": "q_xxx",
  "source_kind": "file | chat | artifact | tool_output | metadata",
  "source_ref": "...",
  "evidence": [],
  "generation": {
    "prompt_ref": "...",
    "model_ref": "...",
    "trace_ref": "...",
    "timestamp": "..."
  },
  "lifecycle_state": "PROPOSED"
}
```

Rules:

- `lifecycle_state` defaults conceptually to `PROPOSED`
- `source_ref` must trace back to a `StoredObject` or another governed source reference
- lifecycle is not approval by generation
- generation metadata is trace, not authority
- a `QuadInstance` carries provenance and generation context, not semantic identity
- lifecycle attaches primarily to `QuadInstance`
- the same `quad_id` may have multiple instances with different lifecycle states

## source_kind

Allowed `source_kind` values:

- `file`
- `chat`
- `artifact`
- `tool_output`
- `metadata`

Alignment rules:

- `file` -> `StoredObject` with `content_ref`
- `chat` -> logical `StoredObject` or governed message refs
- `artifact` -> governed artifact or `DocumentHolder` reference
- `tool_output` -> `owner_ref`-backed tool output
- `metadata` -> semantic metadata snapshot derived from governed metadata

No `source_kind` implies execution authority.

## Evidence anchoring

Evidence records may include:

- `file_ref`
- `object_ref`
- `chunk_id`
- `chunk_hash`
- `text_range`
- `metadata_snapshot_hash`
- `message_ref`
- `artifact_ref`
- `tool_run_ref`

Rules:

- evidence should reference sources rather than duplicate full text
- evidence must be traceable
- evidence and semantic fields must not contain secrets or private absolute host paths
- `chunk_id + chunk_hash` anchor derivative evidence
- if `chunk_hash` changes, dependent quads become `STALE`
- if `chunk_id` disappears, dependent quads become `ORPHANED` or `STALE`

## QuadSet

`QuadSet` is a contextual grouping artifact.

Conceptual structure:

```json
{
  "quadset_id": "...",
  "source_kind": "...",
  "source_ref": "...",
  "generation_context": {},
  "quad_instances": []
}
```

Rules:

- quadsets are grouping artifacts, not approved knowledge
- multiple quadsets per source are allowed
- different prompts or models may produce different quadsets
- a future `semantic_quadset` `StoredObject` may represent the grouping artifact
- future retained semantic material should remain bounded by governed semantic storage limits
- reuse of an existing `QuadSet` is preferred when governed generation context is materially the same

## Relationship to StoredObject

`StoredObject` is the required logical alignment layer for persisted sources.

- `source_ref` should resolve to a governed `StoredObject` when the source is persisted
- `content_ref` and `object_ref` remain storage-governed, not semantically inferred
- semantic quads must not infer project exposure from `StoredObject`
- metadata-derived quads must remain distinct from the underlying metadata snapshot

## Lifecycle preparation boundary

Lifecycle states, allowed transitions, invalidation, and supersession are governed by `semantic_quad_lifecycle.md`.

This model aligns with:

- `PROPOSED`
- `UNDER_REVIEW`
- `APPROVED`
- `REJECTED`
- `STALE`
- `SUPERSEDED`
- `ORPHANED`

Boundaries:

- no review runtime is opened
- no approval runtime is opened
- `PROPOSED` is the default state
- `APPROVED` requires future explicit governed review
- future graph, RDF, or analysis consumers may consume only `APPROVED` quads

Future trigger, scope, and mode rules for quad generation are governed by `quad_generation_policy.md`.

Future semantic-retention bounds are governed by `semantic_storage_limits.md`.

Future reuse and caching rules are governed by `caching_and_reuse_policy.md`.

## Separation from deterministic derivatives

Deterministic derivatives live under derivative-governed outputs.

Semantic quad material is separate:

- quads are not deterministic derivatives
- chunks may be evidence anchors, not semantic containers
- derivatives may be regenerated without semantic mutation
- derivative regeneration may stale evidence without approving or deleting quads
- semantic material must not write back into derivatives or storage
- semantic invalidation reacts to upstream storage or derivative changes only
- semantic material must remain sanitized and must not re-expand redacted or masked upstream content

## Future RDF projection readiness

Semantic quads are future projection inputs.

However:

- RDF remains disabled
- N-Quads persistence remains disabled
- Oxigraph remains disabled
- SPARQL remains disabled

Future projection must preserve:

- lifecycle state
- source traceability
- generation traceability
- evidence anchoring

`PROPOSED` quads must not be projected as approved facts.

Future RDF projection policy is governed by `rdf_projection_policy.md`.

Future graph analysis policy is governed by `graph_analysis_policy.md`.

## Semantic Quad Invariants

- `INV-QUAD-001`: `quad_id` is deterministic semantic identity.
- `INV-QUAD-002`: `quad_instance_id` represents generation context.
- `INV-QUAD-003`: quads are proposals by default.
- `INV-QUAD-004`: quads are not approved facts without explicit review.
- `INV-QUAD-005`: evidence must be traceable.
- `INV-QUAD-006`: quads do not live inside chunks.
- `INV-QUAD-007`: quads do not live inside derivatives.
- `INV-QUAD-008`: `source_kind` is mandatory.
- `INV-QUAD-009`: quadsets are contextual, not authoritative.
- `INV-QUAD-010`: RDF, Oxigraph, and SPARQL remain disabled.
- `INV-QUAD-011`: generation metadata is trace, not authority.
- `INV-QUAD-012`: `StoredObject` alignment is required for persisted sources.
- `INV-QUAD-013`: same meaning plus same evidence anchor must resolve to the same `quad_id`.
- `INV-QUAD-014`: different generation context may yield different `quad_instance_id` values for the same `quad_id`.
- `INV-QUAD-015`: semantic identity and generation-instance identity must never be conflated.
- `INV-QUAD-016`: graph is part of semantic identity.
- `INV-QUAD-017`: semantic quad material must remain separate from deterministic derivative state.
- `INV-QUAD-018`: no quad may open execution, provider invocation, tool execution, filesystem mutation, or runtime approval.
- `INV-QUAD-019`: approved-knowledge eligibility depends on approved `QuadInstance` records, not on `quad_id` alone.
- `INV-QUAD-020`: semantic relations must not be embedded as quad hierarchy and must remain explicit edge objects.
- `INV-QUAD-021`: semantic writes must not mutate derivatives or storage.
- `INV-QUAD-022`: semantic material must not contain secrets, private absolute host paths, or unsanitized personal data.
- `INV-QUAD-023`: quad generation must remain trigger-bound, scope-bounded, and proposal-only unless a later governed runtime opens it.
- `INV-QUAD-024`: retained semantic material should remain bounded by governed semantic storage limits.
- `INV-QUAD-025`: reuse of existing semantic identity is preferred over duplicate semantic meaning when governed identity and context match.

## Related specs

- `docs/specs/semantic_source_scope.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/semantic_proposal_layer.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/storage_policy.md`
- `docs/specs/quad_generation_policy.md`
- `docs/specs/semantic_storage_limits.md`
- `docs/specs/caching_and_reuse_policy.md`
- `docs/specs/security_sanitization_policy.md`
- `docs/specs/system_observability.md`

---

### !REL_FILE!

# semantic_source_scope

## Status

PROPOSAL / F9/F10_PREP declarative governance.

This document does not implement semantic runtime, quad generation runtime, RDF, Oxigraph, SPARQL, embeddings, LLM execution, tool execution, filesystem mutation, document mutation, or runtime approval.

## Purpose

Define the future source scope from which DocGraph may generate semantic quads.

This scope governs eligible semantic sources and evidence anchors only.

It does not define semantic approval, execution authority, or deterministic derivative ownership.

DocGraph may later generate semantic quad instances from:

- document metadata
- derived chunks
- user knowledge files
- system governance files

The initial goal is not free inference.

The initial goal is to build traceable graphs for:

- document structure
- chunks
- provenance
- metadata
- future semantic knowledge

## Source classes

### user_knowledge_source

User-added documents and knowledge materials, including:

- PDFs
- Markdown
- plain text
- reports
- technical documentation

### structured_metadata_source

Structured file or resource metadata, including:

- file type
- title
- logical path
- hash
- size
- dates, when applicable

### chunk_source

Regenerable chunk derivatives from documents, including:

- offsets
- pages
- sections
- textual fragments

### system_governance_source

System governance materials, including:

- `governance/GOVERNANCE.md`
- `architecture/ARCHITECTURE.md`
- `governance/FUNCTIONAL_SCOPE.md`
- specs
- policies
- invariants

## source_kind alignment

Future `QuadInstance.source_kind` values are limited to:

- `file`
- `chat`
- `artifact`
- `tool_output`
- `metadata`

Alignment:

- `file` resolves to a persisted `StoredObject` with `content_ref`
- `chat` resolves to a logical `StoredObject` or governed message reference
- `artifact` resolves to an artifact or `DocumentHolder` reference
- `tool_output` resolves to an `owner_ref`-backed governed tool output
- `metadata` resolves to a governed metadata snapshot rather than inferred facts

No `source_kind` grants execution or exposure authority.

Eligible source classes do not imply mandatory quad generation.

Future generation remains trigger-bound and scope-bounded under `quad_generation_policy.md`.

## Evidence scope

Allowed evidence anchors may include:

- `file_ref`
- `object_ref`
- `chunk_id`
- `chunk_hash`
- `text_range`
- `metadata_snapshot_hash`
- `message_ref`
- `artifact_ref`
- `tool_run_ref`

Evidence must remain traceable and reference governed sources rather than duplicate full text.

Evidence invalidation and lifecycle consequences are governed by `semantic_quad_lifecycle.md`.

Future semantic relations may reference shared governed evidence, but those relations remain separate graph-edge objects rather than child structures inside quads.

## Graph domains

Separate graph domains may include:

- `structural_graph`
- `metadata_graph`
- `semantic_graph`
- `governance_graph`

Interpretation:

- `structural_graph`: documents, chunks, offsets, and document relations
- `metadata_graph`: file and resource properties
- `semantic_graph`: concepts, relations, summaries, and future semantic proposals
- `governance_graph`: phases, invariants, policies, and system rules

All domains may use the same semantic base model:

- `SemanticQuad` as semantic identity
- `QuadInstance` as generation instance
- `QuadSet` as contextual grouping

They must not be mixed without explicit domain resolution.

## Examples

### Structural and metadata examples

- `(DOC_A, has_chunk, CHUNK_001, a1b2c3d4e5f6)`
- `(CHUNK_001, belongs_to, DOC_A, b2c3d4e5f6a7)`
- `(CHUNK_001, starts_at, 0, c3d4e5f6a7b8)`
- `(CHUNK_001, ends_at, 1200, d4e5f6a7b8c9)`
- `(DOC_A, has_file_type, "pdf", e5f6a7b8c9d0)`

### Governance examples

- `(GOVERNANCE, defines, F9_invariants, f6a7b8c9d0e1)`
- `(tool_runtime, must_not_consume, tools_master_catalog, a7b8c9d0e1f2)`
- `(Lume, must_not_execute, tools, b8c9d0e1f2a3)`

## Semantic Source Scope Invariants

- `INV-SRC-001`: quads generated from metadata, chunks, semantic content, and governance MUST remain domain-scoped.
- `INV-SRC-002`: structural_graph, metadata_graph, semantic_graph, and governance_graph MUST NOT be merged without explicit governed resolution.
- `INV-SRC-003`: chunk-derived quads MUST preserve traceability to the source chunk.
- `INV-SRC-004`: metadata-derived quads MUST NOT become runtime authority.
- `INV-SRC-005`: governance-derived quads describe system rules but MUST NOT execute, approve, or enforce runtime behavior in F9/F10_PREP.
- `INV-SRC-006`: Lume MAY use governance-derived knowledge in a future governed mode, but MUST NOT treat it as execution authority.
- `INV-SRC-007`: user knowledge quads and system governance quads MUST be distinguishable.
- `INV-SRC-008`: filesystem presence MUST NOT imply quad generation.
- `INV-SRC-009`: project_manifest exposure and future policy MUST govern eligible sources.
- `INV-SRC-010`: no source class opens LLM execution, tool execution, RDF persistence, Oxigraph, SPARQL, embeddings, or document mutation.
- `INV-SRC-011`: `source_ref` MUST resolve to a governed source reference or `StoredObject` when the source is persisted.
- `INV-SRC-012`: chunks may provide evidence anchors but MUST NOT become semantic containers.
- `INV-SRC-013`: deterministic derivatives and semantic quad material MUST remain separate layers.
- `INV-SRC-014`: metadata snapshots may feed semantic proposals but MUST NOT be treated as approved knowledge.
- `INV-SRC-015`: eligible semantic sources must not trigger uncontrolled or automatic quad generation without an explicit governed trigger.

## Relationship with existing specs

This source-scope proposal depends on and must remain aligned with:

- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/tools_catalogs.md`

`document_text_runtime.md` defines regenerable chunk and text boundaries.

`storage_policy.md` defines `StoredObject`, `file_ref`, and derivative separation boundaries.

`tools_catalogs.md` remains unrelated to semantic execution authority and must not be treated as semantic source runtime.

## Future work

The following remain future-governed concerns:

- official predicates
- entity types
- semantic rules
- graph domain identifiers
- serialization
- RDF / N-Quads converter
- Oxigraph integration
- lifecycle resolution
- ontology

## Explicit limits

This document:

- does not touch `project_runtime`
- does not touch `tool_runtime`
- does not touch `ui_*`
- does not modify Rust
- does not add crates
- does not modify `tools_master_catalog.json`
- does not activate LLM
- does not activate tools
- does not activate Oxigraph
- does not create real quad persistence

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/semantic_proposal_layer.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/quad_generation_policy.md`
- `docs/specs/tools_catalogs.md`

---

### !REL_FILE!

# semantic_storage_limits

## Status

DECLARED_ONLY / F9-F10_PREP governance.

This document does not implement runtime enforcement, automatic pruning, storage compaction, RDF projection runtime, graph-analysis runtime, or execution authority.

## Purpose

Define governed limits for semantic storage so the semantic layer remains bounded and does not grow without control.

These limits are conceptual and future-configurable.

They exist to constrain semantic growth across quads, quadsets, and relations without opening runtime behavior.

## Conceptual limit types

The semantic layer should support, in a future governed runtime, at minimum these conceptual limits:

- `max_quads_per_object`
- `max_quadsets_per_source`
- `max_relations_per_quad`

### max_quads_per_object

`max_quads_per_object` bounds how many semantic quads or quad instances may be retained for a governed source object.

This limit applies conceptually per `StoredObject` or equivalent governed source reference.

### max_quadsets_per_source

`max_quadsets_per_source` bounds how many contextual `QuadSet` groupings may be retained for the same source.

This prevents repeated re-analysis from creating unbounded contextual groupings.

### max_relations_per_quad

`max_relations_per_quad` bounds how many semantic relations may attach to a given approved or proposed semantic assertion instance.

This prevents relation density from growing without meaningful control.

## Policy levels

Semantic storage limits are governed through two conceptual policy levels:

- soft limits
- hard limits ^(future enforcement^)

### soft limits

Soft limits are advisory thresholds.

When crossed in a future runtime, they should produce warnings, review signals, or generation-scope narrowing guidance.

Soft limits do not imply deletion or automatic pruning by themselves.

### hard limits

Hard limits are future-only enforcement thresholds.

They are not active in the current phase.

When a later governed runtime opens them, they may block additional semantic material from being retained beyond the configured boundary.

## Pruning strategies

Future pruning or reduction strategies may include:

- `oldest_first`
- `lowest_confidence_first`
- `least_referenced_first`

These strategies are conceptual only in this phase.

They do not activate deletion, mutation, or enforcement now.

### oldest_first

Prefer removing or superseding the oldest retained semantic material first when newer governed material is more relevant.

### lowest_confidence_first

Prefer removing or superseding semantic material with the lowest future confidence score first when confidence metadata exists.

### least_referenced_first

Prefer removing or superseding semantic material with the weakest graph or trace usage first when governed reference counts exist.

## Boundary rules

Semantic storage limits must remain aligned with:

- storage as root authority
- semantic generation remaining optional and trigger-bound
- invalidation remaining traceable and downstream-only
- security and sanitization remaining mandatory

Limits must not:

- mutate physical storage authority
- imply approval
- bypass lifecycle
- bypass security policy
- infer project exposure

## Future configurability

Limits must be configurable in a future governed runtime.

Configuration remains future-only and inactive now.

The current phase declares only the existence of governed limit concepts, not their concrete numeric values.

## Non-goals

This policy does not open:

- automatic enforcement
- automatic pruning
- runtime generation
- runtime approval
- graph-analysis runtime
- RDF runtime
- execution slices

## Semantic storage limit invariants

- `INV-LIMIT-001`: the semantic layer must remain bounded.
- `INV-LIMIT-002`: semantic storage must not grow unbounded by default.
- `INV-LIMIT-003`: semantic limits must remain configurable in a future governed runtime.
- `INV-LIMIT-004`: soft limits are advisory only in principle and do not imply deletion.
- `INV-LIMIT-005`: hard limits remain future-only until explicitly opened.
- `INV-LIMIT-006`: pruning strategies are conceptual only in this phase.
- `INV-LIMIT-007`: storage-layer authority must not be mutated by semantic limit policy.
- `INV-LIMIT-008`: semantic limits must remain aligned with lifecycle, invalidation, and security policy.

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/storage_policy.md`
- `docs/specs/quad_generation_policy.md`
- `docs/specs/invalidation_policy.md`
- `docs/specs/security_sanitization_policy.md`

---

### !REL_FILE!

# semantic_tag_catalog

## Purpose

Define the governed catalog authority for semantic classification tags used by `StoredObject.metadata.semantic`.

## Catalog authority

`resources/semantic/semantic_tag_catalog.json` is the authority for controlled semantic tags.

Rules:

- stored objects reference `tag_id` only
- tag meaning lives in the catalog, not in each document
- free-form uncontrolled `system_tags` are forbidden
- free-form uncontrolled `usage_tags` are forbidden
- `keyword` governance is semi-controlled in the current declaration:
  - keywords must use normalized `tag_id` entries from the catalog when available
  - new keyword candidates may be proposed before activation

## Tag families

Controlled tag families:

- `keyword`
- `system_tag`
- `usage_tag`

Rules:

- keywords describe topic or domain
- `system_tag` describes where the object belongs in DocGraph or Lume scope
- `usage_tag` describes intended use
- families must not be mixed

Example:

```json
{
  "keywords": ["system_design", "action_layer", "capabilities"],
  "system_tags": ["docgraph_core", "lume_core", "critical_knowledge"],
  "usage_tags": ["internal_reference", "llm_reasoning"]
}
```

## Tag entry model

Conceptual entry:

```json
{
  "tag_id": "llm_reasoning",
  "tag_family": "usage_tag",
  "label": "LLM reasoning",
  "description": "Content intended to support LLM reasoning workflows.",
  "status": "ACTIVE",
  "introduced_in": "F10_PREP",
  "allowed_for": [
    "source_file",
    "document",
    "artifact",
    "chat",
    "bibliographic_entry"
  ],
  "aliases": [],
  "replaces": [],
  "replaced_by": null,
  "quad_mapping": {
    "predicate": "intended_usage",
    "object": "LLM reasoning"
  }
}
```

## Naming rules

- `tag_id` must use `snake_case`
- lowercase only
- no spaces
- no punctuation except underscore
- once `ACTIVE`, `tag_id` is stable
- an active `tag_id` must never be reused for a different meaning

Good:

- `llm_reasoning`
- `docgraph_core`
- `internal_reference`

Bad:

- `LLM Reasoning`
- `docGraphCore`
- `paper_juan_important`
- `use-for-lume`

## Alias and merge rules

- aliases may point to one `ACTIVE` primary tag
- aliases are not primary `tag_id`
- duplicate proposals should become aliases when appropriate
- merges must be explicit

Example:

```json
{
  "tag_id": "llm_context",
  "aliases": ["llm_reference", "prompt_context"]
}
```

## StoredObject integration

`StoredObject.metadata.semantic` may contain:

- `keywords`
- `system_tags`
- `usage_tags`

Rules:

- `system_tags` must reference `ACTIVE` catalog entries
- `usage_tags` must reference `ACTIVE` catalog entries
- `DEPRECATED` entries may be read but should warn
- `PROPOSED` and `REJECTED` entries must not be used as active metadata
- keyword policy remains semi-controlled:
  - normalized catalog entries are preferred
  - proposals may exist before activation

## Future quad mapping

Tags may feed future proposal-only quad generation.

Examples:

- `system_tag`: `doc_X belongs_to_system_scope docgraph_core`
- `usage_tag`: `doc_X intended_usage llm_reasoning`
- `keyword`: `doc_X has_keyword system_design`

Rules:

- quad generation remains separately governed
- tag presence does not approve generated quads
- tag lifecycle affects future quad eligibility

## Invariants

- `INV-TAG-001`: controlled tags must exist in the catalog
- `INV-TAG-002`: `tag_id` is stable and never reused
- `INV-TAG-003`: tag families must not be mixed
- `INV-TAG-008`: documents reference `tag_id`, not meanings
- `INV-TAG-009`: free-form `system_tags` and `usage_tags` are forbidden
- `INV-TAG-010`: tag-based quads remain proposal-only unless separately approved by lifecycle-governed semantic flow

---

### !REL_FILE!

# semantic_tag_lifecycle_policy

## Purpose

Define the governed lifecycle for semantic classification tags.

## Lifecycle states

- `PROPOSED`
- `UNDER_REVIEW`
- `ACTIVE`
- `DEPRECATED`
- `REMOVED`
- `REJECTED`

Meanings:

- `PROPOSED`: suggested but not usable as a controlled tag
- `UNDER_REVIEW`: being assessed
- `ACTIVE`: allowed in `StoredObject` metadata
- `DEPRECATED`: still recognized but not recommended for new objects
- `REMOVED`: not allowed for new use; existing references require migration or compatibility handling
- `REJECTED`: rejected proposal and must not be used

## Allowed transitions

- `PROPOSED -> UNDER_REVIEW`
- `UNDER_REVIEW -> ACTIVE`
- `UNDER_REVIEW -> REJECTED`
- `ACTIVE -> DEPRECATED`
- `DEPRECATED -> REMOVED`
- `DEPRECATED -> ACTIVE`
- `PROPOSED -> REJECTED`

Rules:

- `ACTIVE` must never be set automatically by LLM
- transitions must be traceable
- no silent replacement
- no deletion without migration note

## Proposal sources

Allowed proposal sources:

- `user`
- `system`
- `llm`
- `governance`

Rules:

- users may propose tags
- system may suggest tags from duplicate or gap analysis
- LLM may suggest candidate tags
- LLM must not activate tags
- governance authority approves `ACTIVE` tags

## Proposal acceptance and rejection

A new tag may be proposed when:

- no existing tag captures the concept
- a new system scope appears
- a new usage mode appears
- a new recurring knowledge domain appears
- existing tags would create ambiguity

A new tag must be rejected when:

- it is a synonym of an existing tag
- it mixes tag families
- it is too document-specific
- it is subjective or temporary
- it duplicates an alias
- it lacks description

## Deprecation and removal

- `DEPRECATED` tags remain readable
- `DEPRECATED` tags must not be assigned to new objects unless explicitly allowed
- `DEPRECATED` tags should include `replaced_by` when possible
- `REMOVED` tags require migration or compatibility note

## Invariants

- `INV-TAG-004`: LLM may propose but not activate tags
- `INV-TAG-005`: `ACTIVE` requires governance approval
- `INV-TAG-006`: `DEPRECATED` tags remain readable
- `INV-TAG-007`: `REMOVED` tags require migration note

---

### !REL_FILE!

# SPECS_INDEX

## Status

GOVERNED INDEX.

This file is a navigation and responsibility map for modular specs.

It does not replace the authority of each individual spec, does not merge spec content, and does not open F10.

## Purpose

Avoid dispersion across modular specs.

Clarify which spec governs each responsibility.

Reduce concept duplication by pointing to the correct owner instead of copying rules.

Make dependencies, non-responsibilities, phase status, and runtime effect explicit for Codex consumption.

## Rules

- reuse an existing spec when it already governs the responsibility
- do not duplicate responsibilities across sibling specs
- transversal specs must be referenced, not copied
- consolidation candidates are advisory metadata only
- `specs_index.json` is authoritative for navigation, not a normative source
- individual specs remain the normative source for their owned contracts
- `transformation_core.md` is the conceptual source for transformation vocabulary
- F10 remains not opened by this index

## Modular index

| Spec | Domain | Responsibility | Depends On | Must Not Define | Phase / Runtime |
| --- | --- | --- | --- | --- | --- |
| `action_policy.md` | action/policy | Declares F9 action risk, decision outcomes, human-in-the-loop policy, and blocked actions. | `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | Execution, mutation, unregistered tool authorization | `F9 / declarative_only` |
| `action_resolution.md` | action/policy | Defines declarative ActionRequest-to-trace governance before any future execution. | `action_policy.md`, `flow_control_policy.md`, `pending_action_state.md` | Runner, executor, tool execution, LLM invocation, project_runtime mutation | `F9 / declarative_only` |
| `active_context.md` | workspace/layout | Defines future active-context model derived from selected workspace tab and explicit references. | `workspace_tabs.md`, `document_governed_editing.md` | Stale target fallback, implicit target resolution, execution | `future / future_runtime` |
| `active_object_context.md` | workspace/layout | Governs cross-family selected object context and explicit focus references. | `active_context.md`, `chat_document_flows.md`, `document_tree.md`, `local_sandbox_context.md`, `text_selection.md`, `ui_core.md`, `workspace_tabs.md` | Execution, implicit targets, host-path identity, workspace-tab authority | `F9 / declarative_only` |
| `ai_governance_f9_5.md` | LLM/governance | Declares F9.5 AI governance resources, semantic proposal preparation, disabled flags, and readonly/mock UI surface. | `governance/LLM_RUNTIME_POLICY.md`, `semantic_proposal_layer.md`, `document_text_runtime.md` | LLM execution, embeddings, RDF persistence, SPARQL, document mutation | `F9.5 / declarative_only` |
| `app_main_menu.md` | UI/presentation | Defines governed top-level File, Preferences, Tools, and Help menu domains. | `credentials_policy.md`, `help_menu.md`, `lume_help_popup.md`, `lume_help_tree.md`, `preferences_policy.md`, `project_settings_popup.md`, `project_setup_popup.md`, `tools_panel.md`, `ui_preferences_popups.md` | Tool execution, LLM calls, filesystem mutation, runtime authority | `F9 / presentation_only` |
| `app_status_bar.md` | UI/presentation | Defines bottom status-bar presentation of prepared state, capability hints, and navigation intents. | `credentials_policy.md`, `llm_tool_surface_resolution.md`, `local_sandbox_context.md`, `lume_onboarding_model.md`, `project_profiles.md`, `tools_panel.md` | Execution, secret display, policy inference, runtime mutation | `F9 / presentation_only` |
| `artifact_graph.md` | knowledge/semantic | Defines graph folder as traceable artifact relationship layer, not runtime authority. | `project_folder_layout.md`, `project_runtime.md`, `semantic_proposal_layer.md` | Filesystem inference, project_runtime replacement, RDF persistence, semantic approval | `F9 / declarative_only` |
| `chat_document_flows.md` | workspace/layout | Defines tree, clip, chat, workspace tab interactions, chat references, and chat session folder contract. | `workspace_tabs.md`, `tools_panel.md` | Hidden document store, autonomous tool execution, duplicated document storage | `F8/F9 / declarative_only` |
| `chat_input.md` | UI/presentation | Governs keyboard-first multiline chat input and capture-state boundaries. | `chat_document_flows.md`, `ui_core.md` | Command execution, target resolution, STT integration, provider invocation | `F9 / presentation_only` |
| `credentials_policy.md` | sandbox/preferences/profile/privacy | Defines credential references, secret non-exposure, and non-authority boundaries. | `preferences_policy.md`, `project_settings_popup.md`, `ui_preferences_popups.md` | Project-stored secrets, LLM credential exposure, raw secret display, execution authority | `F9 / declarative_only` |
| `diff_view.md` | documents/templates/export | Defines readonly proposal comparison behavior for future document transformations. | `document_viewer.md`, `patch_preview.md`, `transform_popup.md`, `transformation_core.md` | Editing, patch application, tool execution, timeline memory | `F9 / presentation_only` |
| `document_creation_flow.md` | documents/templates/export | Defines future governed flow for creating Markdown ARTIFACT packages from templates. | `document_templates.md`, `document_package.md`, `action_resolution.md` | Package creation in F9, mutation, export execution, LLM calls | `future / future_runtime` |
| `document_export.md` | documents/templates/export | Defines future export governance from Markdown ARTIFACT packages to derived outputs. | `document_package.md`, `document_references.md` | Pandoc/Tectonic/LibreOffice execution, UI state as export source | `future / future_runtime` |
| `document_governed_editing.md` | documents/templates/export | Defines future governed editing model for ARTIFACT documents. | `active_context.md`, `document_patch_runtime.md`, `action_resolution.md` | Source/derived mutation, UI patch logic, LLM direct file writes | `future / future_runtime` |
| `document_package.md` | documents/templates/export | Defines future DocumentHolder package layout and ownership model. | `document_creation_flow.md`, `document_template_ui_contract.md` | Package runtime, root outputs, source/derived mutation | `future / future_runtime` |
| `document_patch_runtime.md` | documents/templates/export | Defines future patch proposal and application model. | `transform_popup.md`, `transformation_core.md` | Current patch runtime, UI-owned validation, direct mutation | `future / future_runtime` |
| `document_references.md` | documents/templates/export | Defines future governed document references, bibliography, and citation/export style contracts. | `document_templates.md` | Parser runtime, export runtime, LLM calls, document mutation | `future / future_runtime` |
| `document_template_designer_tool.md` | tools/catalogs | Defines future TemplateDesignerTool proposal and relationship to Tools Panel, LLM suggestions, and ActionResolution. | `action_resolution.md`, `chat_document_flows.md`, `document_template_resolution.md`, `document_template_ui_contract.md`, `document_template_validation.md`, `document_templates.md`, `tools_panel.md` | Tool runtime, template runtime, LLM execution, filesystem mutation, UI behavior | `future / future_runtime` |
| `document_template_resolution.md` | documents/templates/export | Defines future declarative rules for selecting templates before governed document creation. | `document_creation_flow.md`, `document_package.md`, `document_template_validation.md`, `document_templates.md` | Resolver runtime, document creation, export, LLM calls, tool execution | `future / future_runtime` |
| `document_template_ui_contract.md` | documents/templates/export | Defines future UI capture boundary for template popup flows. | `document_creation_flow.md`, `document_template_designer_tool.md`, `document_template_resolution.md`, `document_template_validation.md`, `document_templates.md` | UI runtime behavior, validation authority, LLM calls, tool execution | `future / presentation_only` |
| `document_template_validation.md` | documents/templates/export | Defines future validation dimensions and findings model for templates, overrides, and packages. | `document_creation_flow.md`, `document_template_resolution.md`, `document_template_ui_contract.md`, `document_templates.md` | Validator runtime, export runtime, LLM calls, tool execution | `future / future_runtime` |
| `document_templates.md` | documents/templates/export | Defines governed Markdown document template model. | `document_creation_flow.md`, `document_references.md`, `document_template_resolution.md`, `document_template_ui_contract.md`, `document_template_validation.md` | Runtime, export, editor behavior, LLM behavior, tool execution | `future / future_runtime` |
| `document_text_runtime.md` | documents/templates/export | Defines current regenerable text derivation contract over primary documents. | None declared | UI text derivation, semantic approval, LLM execution | `F8/F9 / none` |
| `document_tree.md` | workspace/layout | Governs manifest-exposed project object navigation and selection entrypoints. | `active_context.md`, `active_object_context.md`, `gui_objects_v1.md`, `local_sandbox_context.md`, `workspace_tabs.md` | Filesystem scan, import, mutation, viewer logic, clip behavior | `F9 / presentation_only` |
| `document_viewer.md` | UI/presentation | Governs readonly document rendering, selection presence, and prepared overlay consumption. | `active_object_context.md`, `diff_view.md`, `document_tree.md`, `patch_preview.md`, `text_selection.md`, `transform_popup.md`, `transformation_core.md`, `workspace_tabs.md` | Editing, patch runtime, tool execution, popup/diff internals | `F9 / presentation_only` |
| `external_dependency_manager.md` | tools/catalogs | Declares governed handling of portable external dependency metadata. | `tools_catalogs.md`, `tool_implementation_governance.md` | Download, extraction, execution, UI logic, F10 runtime | `F9 / declarative_only` |
| `flow_control_policy.md` | action/policy | Defines confirmation and flow-control rules. | `action_resolution.md` | Execution, mutation, bypassing approval policy | `F9 / declarative_only` |
| `gui_objects_v1.md` | Lume/help/onboarding | Defines canonical GUI object vocabulary for Lume contextual help. | `help_menu.md`, `lume_help_popup.md`, `lume_interaction_model.md`, `ui_core.md` | Runtime behavior, LLM calls, tool execution, action creation, mutation | `F9.5 / declarative_only` |
| `help_menu.md` | Lume/help/onboarding | Defines governed Help menu entries and informational-only Help surfaces. | `app_main_menu.md`, `gui_objects_v1.md`, `lume_help_popup.md`, `lume_help_tree.md` | Actions, tools, LLM execution, credentials, runtime mutation | `F9 / presentation_only` |
| `how_to_add_a_tool.md` | tools/catalogs | Provides procedural checklist for governed tool onboarding. | `action_resolution.md`, `tool_implementation_governance.md`, `tools_catalogs.md`, `tools_panel.md` | Runtime logic, UI logic, execution authority, architecture redefinition | `F9 / declarative_only` |
| `i18n_audit_consistency.md` | audits/meta | Defines advisory-only interpretation rules for i18n consistency auditing. | `governance/I18N_POLICY.md` | Runtime authority, auto-fix behavior, destructive changes, UI/runtime modification | `F9 / none` |
| `knowledge_panel.md` | knowledge/semantic | Defines minimum UI-facing contract for readonly project knowledge navigation. | `project_runtime.md`, `ui_core.md` | Editing, semantic inference, manifest reinterpretation, runtime authority | `F8/F9 / presentation_only` |
| `llm_core.md` | LLM/governance | Defines inherited LLM contract, input analysis, decomposition, and response synthesis policy for future governed LLM integration. | `governance/LLM_RUNTIME_POLICY.md` | Provider execution in F9, tool execution, document mutation, UI orchestration | `F9/F10 / future_runtime` |
| `llm_agent_prompts.md` | LLM/governance | Defines Lume as the single front-facing assistant identity and governs declarative storage, format, versioning, and privacy constraints for agent prompt resources. | `llm_core.md`, `lume_interaction_model.md`, `llm_tool_surface_resolution.md` | Runtime loading, provider calls, tool execution, prompt embedding in code, second front-facing assistant | `F10_PREP / declarative_only` |
| `llm_tool_runtime.md` | LLM/governance | Defines LLM Tools as governed capability requests, not model-executed functions. | `llm_tool_surface_resolution.md`, `action_resolution.md`, `tools_catalogs.md`, `tool_implementation_governance.md` | Provider calls, tool execution, duplicated tool_runtime, F10 opening | `F9 / declarative_only` |
| `llm_tool_surface_resolution.md` | LLM/governance | Defines future EffectiveToolSurface contract, modes, explainability, and bounded LLM context. | `tools_catalogs.md`, `tool_implementation_governance.md`, `action_resolution.md`, `llm_core.md` | Resolver code, execution authority, raw catalog injection, UI authority | `F10_PREP / future_runtime` |
| `local_sandbox_context.md` | sandbox/preferences/profile/privacy | Defines declarative policy for future authorized local sandbox roots. | `project_profiles.md` | Sandbox runtime, write-back, mutable actions, absolute-path authority | `F9 / declarative_only` |
| `lume_help_popup.md` | Lume/help/onboarding | Defines F9.5 contextual help popup as readonly, ephemeral, and non-executing. | `gui_objects_v1.md`, `help_menu.md`, `lume_help_tree.md`, `lume_interaction_model.md`, `lume_onboarding_modal.md` | Execution, mutation, action approval, provider calls, persistent help history | `F9.5 / presentation_only` |
| `lume_help_tree.md` | Lume/help/onboarding | Defines declarative help-tree navigation, explanation, and procedure state. | `gui_objects_v1.md`, `help_menu.md`, `llm_tool_surface_resolution.md`, `lume_help_popup.md`, `lume_onboarding_model.md` | Execution, policy inference, LLM calls, runtime state persistence | `F9 / presentation_only` |
| `lume_interaction_model.md` | Lume/help/onboarding | Defines Lume as declarative interaction and help layer, not executor. | `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | Filesystem execution, tool execution, LLM execution, project pipeline authority | `F9 / declarative_only` |
| `lume_onboarding_modal.md` | Lume/help/onboarding | Defines declarative onboarding/help modal without creating project, chat persistence, or execution. | `lume_interaction_model.md` | Project creation, LLM calls, tool execution, chat persistence | `F9 / presentation_only` |
| `lume_onboarding_model.md` | Lume/help/onboarding | Defines dual-mode Lume onboarding and capability-state explanation. | `credentials_policy.md`, `llm_core.md`, `llm_tool_surface_resolution.md`, `lume_help_popup.md`, `lume_interaction_model.md`, `lume_onboarding_modal.md`, `project_setup_popup.md` | LLM requirement, credential requirement, tool execution, blocking project setup | `F9 / declarative_only` |
| `patch_preview.md` | documents/templates/export | Defines readonly in-place preview overlay semantics for pending proposals. | `diff_view.md`, `document_viewer.md`, `transform_popup.md`, `transformation_core.md` | Patch application, persistence, execution, diff review ownership | `F9 / presentation_only` |
| `pending_action_state.md` | action/policy | Defines conceptual pending-action state for future explicit action execution references. | `action_resolution.md` | Runner, executor, hidden mutation, automatic execution | `F9 / declarative_only` |
| `preferences_policy.md` | sandbox/preferences/profile/privacy | Defines non-secret preference configuration boundaries and non-authority rules. | `credentials_policy.md`, `project_settings_popup.md`, `ui_preferences_popups.md` | Secrets, execution authority, policy bypass, host-path truth | `F9 / declarative_only` |
| `privacy_and_consent_model.md` | sandbox/preferences/profile/privacy | Defines governed privacy, consent, and bounded-context exposure rules for future Lume and LLM-facing interaction. | `preferences_policy.md`, `llm_core.md`, `lume_interaction_model.md`, `user_profile_policy.md` | Runtime storage, implicit consent is forbidden, credential exposure, tool execution, UI authority | `F9/F10_PREP / declarative_only` |
| `project_folder_layout.md` | project/runtime | Defines normative project folder contract, owner-scoped outputs, and manifest-governed exposure. | `project_runtime.md`, `artifact_graph.md`, `chat_document_flows.md`, `document_package.md`, `tools_panel.md` | Root outputs, filesystem exposure authority, project_runtime replacement | `F9 / declarative_only` |
| `project_profiles.md` | sandbox/preferences/profile/privacy | Defines project-profile declarations as intended capabilities and default policies. | None declared | Execution enabling, project-policy override, tool authority, F10 opening | `F9 / declarative_only` |
| `project_runtime.md` | project/runtime | Defines central project pipeline. | `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | UI logic, tool runtime expansion, duplicated pipeline | `3A-F9 / none` |
| `project_settings_popup.md` | sandbox/preferences/profile/privacy | Defines future project-settings intent capture. | `credentials_policy.md`, `preferences_policy.md`, `ui_preferences_popups.md` | Project config write-back, tool execution, secret display, policy authority | `F9 / presentation_only` |
| `project_setup_popup.md` | sandbox/preferences/profile/privacy | Defines future project setup draft capture. | `action_resolution.md`, `credentials_policy.md`, `local_sandbox_context.md`, `lume_interaction_model.md`, `preferences_policy.md`, `project_folder_layout.md`, `project_profiles.md`, `tool_implementation_governance.md`, `tools_panel.md` | Project creation, folder mutation, LLM invocation, tool execution | `F9 / presentation_only` |
| `runtime_layout.md` | project/runtime | Documents declarative runtime folder layout for future external tools, engines, models, cache, and temp data. | `governance/WORKSPACE_RULES.md` | Runtime materialization, execution, PATH mutation, dependency installation | `F9 / declarative_only` |
| `sandbox_viewer.md` | sandbox/preferences/profile/privacy | Governs readonly sandbox-context inspection as a workspace object. | `active_object_context.md`, `document_tree.md`, `local_sandbox_context.md`, `ui_core.md`, `workspace_tabs.md` | Filesystem exploration, write-back UI, tool launch, mutation | `F9 / presentation_only` |
| `semantic_proposal_layer.md` | knowledge/semantic | Defines SemanticProposal as proposal, not approved knowledge. | `action_resolution.md`, `ai_governance_f9_5.md`, `document_text_runtime.md`, `pending_action_state.md` | LLM execution, embeddings, RDF persistence, Oxigraph, SPARQL, approval runtime | `F9.5 / declarative_only` |
| `semantic_quad_lifecycle.md` | knowledge/semantic | Defines lifecycle states for future semantic quads as proposal governance. | `ai_governance_f9_5.md`, `document_text_runtime.md`, `semantic_proposal_layer.md`, `semantic_quad_model.md` | Semantic approval, RDF persistence, SPARQL, LLM execution | `F9/F10_PREP / declarative_only` |
| `semantic_quad_model.md` | knowledge/semantic | Defines future semantic quad representation for proposal-only semantic preparation. | `action_resolution.md`, `ai_governance_f9_5.md`, `document_text_runtime.md`, `semantic_proposal_layer.md` | Fact creation, RDF persistence, semantic store, LLM execution | `F9/F10_PREP / declarative_only` |
| `semantic_source_scope.md` | knowledge/semantic | Defines source-scope boundaries for semantic proposal preparation. | `architecture/ARCHITECTURE.md`, `governance/FUNCTIONAL_SCOPE.md`, `governance/GOVERNANCE.md`, `document_text_runtime.md`, `semantic_proposal_layer.md`, `semantic_quad_lifecycle.md`, `semantic_quad_model.md`, `tools_catalogs.md` | Raw catalog injection, runtime expansion, knowledge approval, tool execution | `F9/F10_PREP / declarative_only` |
| `spec_runtime.md` | project/runtime | Defines declarative loading and normalized lookup responsibilities for specs, configs, registries, and contracts. | `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | Runtime authority beyond declared lookup, UI policy, project pipeline ownership | `3A-F9 / none` |
| `SPECS_INDEX.md` | audits/meta | Human-readable responsibility map for modular specs. | `specs_index.json` | Normative replacement, F10 opening, runtime behavior, spec-content merging | `global / none` |
| `storage_policy.md` | storage/i18n/theme | Defines F9-ready file-based storage policy, checksum deduplication, and regenerable JSONL index rules. | `governance/WORKSPACE_RULES.md` | Database runtime, hidden mutation, root outputs, non-regenerable authority | `F9 / declarative_only` |
| `structured_content_assets.md` | documents/templates/export | Defines future governed treatment of figures, tables, and equations as structured assets. | `document_package.md`, `document_governed_editing.md` | Editing surfaces, export runtime, inspector runtime, source mutation | `future / future_runtime` |
| `text_selection.md` | documents/templates/export | Governs structured text selection references tied to document integrity and trace. | `active_object_context.md`, `chat_document_flows.md`, `document_viewer.md`, `transform_popup.md`, `transformation_core.md` | Popup UX, diff behavior, execution, independent source truth | `F9 / presentation_only` |
| `tool_implementation_governance.md` | tools/catalogs | Defines rules and invariants for tools and implementation discipline. | `how_to_add_a_tool.md`, `tools_catalogs.md` | UI execution, LLM execution, dependency/tool confusion, ActionResolution bypass | `F9 / declarative_only` |
| `tools_catalogs.md` | tools/catalogs | Defines tool catalog structure and declaration boundaries. | `how_to_add_a_tool.md`, `llm_tool_surface_resolution.md`, `tool_implementation_governance.md` | Runtime execution, catalog-as-runtime, dependency installation, F10 opening | `F9 / declarative_only` |
| `tools_panel.md` | tools/catalogs | Defines UI contract for Operational Tools, LLM Tools, and External Dependencies surfaces. | `action_resolution.md`, `app_main_menu.md`, `external_dependency_manager.md`, `how_to_add_a_tool.md`, `tool_implementation_governance.md`, `tools_catalogs.md` | Tool execution policy, LLM execution, catalog mutation, shared Preferences panel | `F8/F9 / presentation_only` |
| `transform_popup.md` | documents/templates/export | Governs contextual transformation intent capture and proposal-state controls. | `diff_view.md`, `document_governed_editing.md`, `document_patch_runtime.md`, `document_viewer.md`, `text_selection.md`, `transformation_core.md` | Patch application, runtime validation, diff semantics, editing | `F9 / presentation_only` |
| `transform_timeline.md` | documents/templates/export | Governs structured memory of transformation attempts, proposals, and user decisions. | `diff_view.md`, `document_governed_editing.md`, `patch_preview.md`, `transform_popup.md`, `transformation_core.md`, `transformation_recipes.md`, `workspace_tabs.md` | Graph authority, tool manifests, execution, patch application | `F9 / declarative_only` |
| `transformation_core.md` | documents/templates/export | Canonical conceptual source for transformation vocabulary, proposal lineage, overlay, preview, timeline, and recipe concepts. | `active_context.md`, `document_governed_editing.md`, `document_patch_runtime.md`, `workspace_tabs.md` | Viewer UX, popup UX, diff UX, runtime execution details | `F9 / declarative_only` |
| `transformation_recipes.md` | documents/templates/export | Governs reusable declarative transformation patterns derived from timeline or future suggestion. | `transformation_core.md`, `transform_timeline.md` | Script logic, macro execution, runtime orchestration, LLM invocation | `F9 / declarative_only` |
| `ui_core.md` | UI/presentation | Defines UI state model and presentation-only boundaries. | `governance/UI_SLINT_POLICY.md`, `governance/WORKSPACE_RULES.md` | Filesystem ownership, domain policy, LLM calls, tool execution, semantic inference | `F5-F9 / presentation_only` |
| `ui_preferences_popups.md` | sandbox/preferences/profile/privacy | Defines governed preference, credential, and settings popup presentation boundaries and i18n keys. | `app_main_menu.md`, `credentials_policy.md`, `preferences_policy.md`, `project_settings_popup.md` | Execution, secret display, ActionResolution bypass, implied tool/LLM availability | `F9 / presentation_only` |
| `user_profile_policy.md` | sandbox/preferences/profile/privacy | Defines optional, opaque, non-sensitive user-profile references for future Lume and LLM-facing flows. | `preferences_policy.md`, `llm_core.md`, `lume_interaction_model.md`, `privacy_and_consent_model.md` | Sensitive data requirements, runtime persistence, credential exposure, tool execution, authentication runtime | `F9/F10_PREP / declarative_only` |
| `workspace_core.md` | workspace/layout | Defines workspace roots and path discipline. | `governance/WORKSPACE_RULES.md` | Project pipeline ownership, UI policy, host-specific persisted truth | `3A-F9 / none` |
| `workspace_layout.md` | workspace/layout | Defines central layout relationships for tree, chat, tabs workspace, viewer, and conditional sandbox presentation. | `ui_core.md`, `workspace_tabs.md` | Execution logic, project_runtime ownership, file mutation, tools/knowledge replacement | `F9 / presentation_only` |
| `workspace_tabs.md` | workspace/layout | Governs tab identity, non-duplication, mouse interaction, and representation-only tab behavior. | `active_object_context.md`, `document_tree.md`, `document_viewer.md`, `gui_objects_v1.md`, `sandbox_viewer.md`, `transform_timeline.md`, `ui_core.md`, `workspace_layout.md` | Viewer internals, diff behavior, popup behavior, execution, permission decisions | `F8/F9 / presentation_only` |

## Related advisory scripts

| Script | Purpose | Must Not Do |
| --- | --- | --- |
| `dev/scripts/audits/audit_tools_compliance.bat` | Advisory audit for tool onboarding compliance across catalogs, specs, and tool-runtime declarations. | Auto-fix, execute tools, mutate catalogs, open F10 |
| `dev/scripts/audits/audit_i18n_consistency.bat` | Advisory audit for i18n key consistency and best-effort hardcoded text detection. | Auto-fix, mutate i18n files, enforce destructive changes |

## Consolidation Candidates

These groups are advisory only. They do not merge, deprecate, delete, or weaken any current spec.

| Candidate target | Sources | Risk | Preservation rule |
| --- | --- | --- | --- |
| `lume_help_and_onboarding.md` | `lume_help_popup.md`, `lume_help_tree.md`, `lume_onboarding_modal.md`, `lume_onboarding_model.md`, `gui_objects_v1.md`, `help_menu.md` | LOW | Lume remains help/interaction only; no execution, mutation, provider calls, or action approval |
| `tool_governance.md` | `how_to_add_a_tool.md`, selected procedural sections of `tool_implementation_governance.md` | LOW | Preserve declared != executable, UI != executor, LLM != executor, tools != dependencies |
| `preferences_surfaces.md` | `preferences_policy.md`, `project_settings_popup.md`, `project_setup_popup.md`, `ui_preferences_popups.md` | MEDIUM | Keep `credentials_policy.md` separate; preferences do not contain secrets or grant authority |
| `document_template_governance.md` | `document_templates.md`, `document_template_resolution.md`, `document_template_validation.md`, `document_template_ui_contract.md`, `document_creation_flow.md` | MEDIUM | Preserve proposal/future status and no document creation/export runtime |
| `semantic_governance.md` | `ai_governance_f9_5.md`, `semantic_proposal_layer.md`, `semantic_quad_model.md`, `semantic_quad_lifecycle.md`, `semantic_source_scope.md` | MEDIUM | Preserve proposal != fact, no RDF/Oxigraph/SPARQL/embeddings/runtime approval |
| `document_transformation_proposals.md` | `transformation_core.md`, `text_selection.md`, `transform_popup.md`, `patch_preview.md`, `diff_view.md`, `transform_timeline.md`, `transformation_recipes.md` | HIGH | Preserve UI-only/proposal boundaries and keep patch application outside UI |
| `workspace_surfaces.md` | `workspace_layout.md`, `workspace_tabs.md`, `document_tree.md`, `sandbox_viewer.md`, `active_object_context.md`, `active_context.md` | HIGH | Preserve UI presentation only; UI must not decide policy or own runtime authority |

## Old To New Candidate Map

This map is advisory only. It is not a deprecation notice and does not authorize deletion.

| Current spec | Candidate target |
| --- | --- |
| `lume_help_popup.md` | `lume_help_and_onboarding.md` |
| `lume_help_tree.md` | `lume_help_and_onboarding.md` |
| `lume_onboarding_modal.md` | `lume_help_and_onboarding.md` |
| `lume_onboarding_model.md` | `lume_help_and_onboarding.md` |
| `gui_objects_v1.md` | `lume_help_and_onboarding.md` |
| `help_menu.md` | `lume_help_and_onboarding.md` |
| `preferences_policy.md` | `preferences_surfaces.md` |
| `project_settings_popup.md` | `preferences_surfaces.md` |
| `project_setup_popup.md` | `preferences_surfaces.md` |
| `ui_preferences_popups.md` | `preferences_surfaces.md` |
| `credentials_policy.md` | `DO_NOT_MERGE_PRIVACY_BOUNDARY` |
| `how_to_add_a_tool.md` | `tool_governance.md` |
| `tool_implementation_governance.md` | `tool_governance.md` |
| `document_templates.md` | `document_template_governance.md` |
| `document_template_resolution.md` | `document_template_governance.md` |
| `document_template_validation.md` | `document_template_governance.md` |
| `document_template_ui_contract.md` | `document_template_governance.md` |
| `document_creation_flow.md` | `document_template_governance.md` |
| `ai_governance_f9_5.md` | `semantic_governance.md` |
| `semantic_proposal_layer.md` | `semantic_governance.md` |
| `semantic_quad_model.md` | `semantic_governance.md` |
| `semantic_quad_lifecycle.md` | `semantic_governance.md` |
| `semantic_source_scope.md` | `semantic_governance.md` |
| `transformation_core.md` | `document_transformation_proposals.md` |
| `text_selection.md` | `document_transformation_proposals.md` |
| `transform_popup.md` | `document_transformation_proposals.md` |
| `patch_preview.md` | `document_transformation_proposals.md` |
| `diff_view.md` | `document_transformation_proposals.md` |
| `transform_timeline.md` | `document_transformation_proposals.md` |
| `transformation_recipes.md` | `document_transformation_proposals.md` |
| `workspace_layout.md` | `workspace_surfaces.md` |
| `workspace_tabs.md` | `workspace_surfaces.md` |
| `document_tree.md` | `workspace_surfaces.md` |
| `sandbox_viewer.md` | `workspace_surfaces.md` |
| `active_object_context.md` | `workspace_surfaces.md` |
| `active_context.md` | `workspace_surfaces.md` |

## Transversal note

For transformation-related responsibilities:

- `transformation_core.md` defines concepts
- `text_selection.md` defines governed selection reference
- `transform_popup.md` defines intent capture
- `patch_preview.md` defines inline overlay preview
- `diff_view.md` defines comparison review
- `transform_timeline.md` defines memory and trace
- `transformation_recipes.md` defines reusable declarative patterns

These specs should reference each other when needed, but they must not copy each other's responsibilities.

---

### !REL_FILE!

# spec_runtime

## Purpose

Load and interpret active specs, contracts, config, and registries through inherited declarative-first rules.

## Responsibilities

- resolve active spec and config locations under governed workspace rules
- load declarative documents and machine-readable contracts
- expose normalized lookup surfaces for higher runtime layers
- preserve source provenance for traceability

## Inputs

- workspace root
- target spec, config, registry, or contract identifiers
- document paths governed by active sandbox rules

## Outputs

- normalized spec/config payloads
- source provenance metadata
- validation diagnostics for missing or incompatible declarations

## Allowed Dependencies

- standard library
- `workspace_core`
- `io_runtime`
- `core_domain`

## Forbidden Responsibilities

- no UI logic
- no direct project business logic
- no tool execution
- no provider invocation
- no mutation of normative documents

## Initial Phase Scope

- loading active sandbox docs, specs, and resource contracts
- normalized access to declarative runtime inputs
- no full schema engine yet

---

### !REL_FILE!

# storage_policy

## Purpose

Define the F9-ready storage model for user data, project data, and regenerable indexes.

The current model is file-based.

## Scope F9

F9 declares:

- file-based storage
- checksum deduplication
- regenerable JSONL indexes
- `user/file_store/blobs/<sha256_hash>/` as the only physical blob authority
- governed JSON schemas for `file_ref`, `BlobRecord`, `StoredObject`, `UsageRef`, and `DerivationManifest` as declarative contracts only
- source files plus manifests as source of truth
- SQLite disabled
- Oxigraph disabled

Indexes are derived data and may be rebuilt.

## file_store as physical authority

`file_store` is the physical storage authority for governed blob content:

- blobs live under `user/file_store/blobs/<sha256_hash>/`
- blobs are content-addressed by `sha256`
- blobs are immutable
- blob content is not duplicated by usage kind
- blob content is not stored under project folders

The blob store is the only physical source of truth for governed persisted content.

`file_store` belongs to the governed `SANDBOX` domain and does not imply `HOST` or `EXTERNAL` authority.

## StoredObject as logical abstraction

`StoredObject` is the canonical logical abstraction above physical blob storage.

It must include:

- `object_ref`
- `object_kind`
- `content_ref`
- `metadata.semantic`
- `metadata.technical`
- `metadata.operational`
- `lifecycle_state`
- `derivation_capabilities`
- `quad_flags`

`StoredObject` is logical, not physical:

- one blob may map to multiple `StoredObject` declarations
- `StoredObject` does not imply project exposure
- metadata is mandatory even when its subobjects are empty
- metadata must not contain secrets
- metadata must be sanitized before becoming portable governed state
- portable truth must not depend on absolute host paths

## refs vs storage

Refs declare usage; they do not duplicate content and they do not define storage authority.

- `file_ref` is a stable content identifier, not a filename or path
- `usage_ref` declares usage, not ownership of physical content
- schema validity does not imply project exposure
- `project_manifest` remains the exposure authority
- storage indexes and refs must not leak secrets or private absolute paths

## Intake dedup identity

F12.8 defines storage dedup hardening for governed file intake.

The storage layer treats content hash as physical identity evidence:

- same governed byte-level hash means same physical blob candidate
- different governed byte-level hash means different physical content
- missing or unverifiable hash means physical sameness is unknown

Identity relationship:

- `content_hash` is hash evidence
- `file_ref` is content identity
- `StoredObject` or `stored_object_candidate_ref` is logical usage context
- `intake_item_id` is intake selection identity

Storage reuse policy:

- physical blob reuse is preferred when content hash matches and the existing blob is governed and immutable
- logical stored object candidates may be distinct while sharing the same `file_ref`
- duplicate intake selections must not duplicate blob bytes when reuse is allowed
- duplicate intake selections must not collapse metadata, comments, traces, owners, or blocking reasons
- blob reuse does not imply project exposure
- blob reuse does not imply derivative generation
- blob reuse does not update `project_manifest`

## derivatives and semantic boundary

Derivatives are:

- deterministic
- regenerable
- non-semantic

Examples include text extraction, pages, chunks, previews, and structured metadata.

Derivative presence or folder presence alone is not authority.

The semantic layer remains separate:

- semantic quads are not generated here
- semantic runtime is not active here
- RDF, Oxigraph, and SPARQL are not enabled here
- storage invalidation flows downstream only through derivatives, semantic, and graph layers
- no downstream semantic, graph, or analysis layer may mutate storage
- downstream semantic or RDF layers must not re-expand sanitized storage material

## Invariants

- `INV-STORAGE-001`: the same physical file SHOULD NOT be duplicated across `knowledge/files`, chats, or documents; stable references SHOULD be preferred.
- `INV-STORAGE-002`: `file_store/blobs` is the only physical blob authority.
- `INV-STORAGE-003`: `StoredObject` is logical, not storage.
- `INV-STORAGE-004`: schema presence does not imply runtime validation.
- `INV-STORAGE-005`: schema validity does not imply project exposure.
- `INV-STORAGE-006`: schema validity does not imply execution authority.
- `INV-STORAGE-007`: `file_ref` is content-addressed and path-independent.
- `INV-STORAGE-008`: `usage_ref` declares usage and does not duplicate physical content.
- `INV-STORAGE-009`: derivation-manifest state must be explicit and must not be inferred from folder presence.
- `INV-STORAGE-010`: derivatives are deterministic and non-semantic.
- `INV-STORAGE-011`: semantic preparation remains separate and inactive at this layer.
- `INV-STORAGE-012`: storage is root authority for downstream invalidation flow.
- `INV-STORAGE-013`: no downstream layer may mutate physical storage authority.
- `INV-STORAGE-014`: storage metadata and indexes must not expose secrets or private absolute host paths.
- `INV-STORAGE-015`: duplicate intake content should reuse physical blob identity when hash and policy allow.
- `INV-STORAGE-016`: physical blob reuse must not merge logical `StoredObject` or intake item metadata.
- `INV-STORAGE-017`: missing or unverifiable content hash must not trigger assumed deduplication.
- Source files plus governed manifests remain the source of truth.
- Regenerable indexes MUST NOT become authoritative state.
- Selected external folders are not project truth.
- Sandbox working copies are project-owned working artifacts when future runtime creates them.
- Host absolute paths must not become portable truth.
- Project storage must not contain secret values.
- Credential references are not credentials.
- Logs, manifests, graph data, and tool outputs must not contain secret values.

## Forbidden responsibilities

Storage policy must not:

- introduce SQLite now
- introduce Oxigraph now
- make JSONL indexes authoritative
- hide source truth inside chat
- create onboarding projects
- bypass governed manifests

## Future F10/F11 notes

F10 may rely on file-based policy for LLM/tool context selection.

Future compatibility:

- `StoredObject` may feed future semantic quad generation
- `StoredObject` may feed future RDF projection
- `StoredObject` may feed future graph analysis
- approved semantic relation edges may reference governed `StoredObject` evidence or source refs in the future
- semantic quad sources must align with governed `StoredObject`, `file_ref`, or equivalent governed source references
- semantic quad material remains separate from deterministic derivatives and from physical blob authority
- future retained semantic material must remain bounded by governed semantic storage limits rather than unrestricted accumulation

These future consumers do not activate semantic runtime, RDF/Oxigraph, or execution here.

Stored content and derivatives may support future quad generation only through explicit governed triggers and bounded scope.

Stored content and deterministic derivatives may support future governed reuse when content identity and governed context match.

Any future RDF dataset remains a projection target only and does not replace physical blob authority, `StoredObject`, or `project_manifest`.

## F11.7 manifest and trace storage alignment

Future `ToolRunManifest` and `TraceRecord` artifacts must follow governed storage discipline:

- outputs must be owner-scoped
- no project-root `outputs/`
- manifest path remains future
- trace path remains future
- filesystem presence does not imply project exposure
- `project_manifest` remains exposure authority

Rules:

- `F11.7` creates no files or folders
- `F11.7` does not update `project_manifest`
- `F11.7` does not update graph
- `F11.7` does not update registry

F11 should audit checksum behavior, index regeneration, and optional database decisions before any database-backed storage is introduced.

## F12.0 / F11.RUNTIME-0 output discipline proposal

`F12.0 / F11.RUNTIME-0` is proposal-only and creates no files.

A future first runtime opening must produce, when explicitly implemented later:

- owner-scoped result
- `tool_run_manifest.json`
- trace record or trace metadata

It must not produce:

- project-root `outputs/`
- raw private host paths
- secrets
- source-artifact mutation
- registry mutation
- graph mutation
- `project_manifest` mutation

Until a later implementation slice opens runtime, `ToolRunManifest` remains future-required and `TraceRecord` remains future-required.

## F12.1 text.measure storage gate

`F12.1` is gate-only and creates no files.

For the future `text.measure` implementation slice, the output contract must be:

- `result.json`
- owner-scoped location only
- `owner_ref` mandatory
- no project-root `outputs/`
- no raw private host paths
- no secrets
- no source-artifact mutation

`ToolRunManifest` is mandatory for future execution, but manifest path policy is declared only and no `tool_run_manifest.json` is created in `F12.1`.

`TraceRecord` is mandatory for future execution, but trace path policy is declared only and no `TraceRecord` is persisted in `F12.1`.

## F12.2A text.measure storage implementation plan

`F12.2A` is plan-only and creates no files.

Future `F12.2B` storage scope is limited to `text.measure` and `crates/tool_runtime`.

Future allowed storage effects:

- create an owner-scoped output directory
- create owner-scoped `result.json` after successful execution
- create `tool_run_manifest.json` after successful execution or governed failed run if explicitly allowed
- create `TraceRecord`-compatible metadata only if already declared by `F12.1` and `F11.7`

Future storage acceptance checklist:

- `owner_ref` is mandatory
- `trace_ref` is mandatory
- missing `owner_ref` blocks execution
- missing `trace_ref` blocks execution
- output path cannot escape owner-scoped sandbox
- no project-root outputs
- no `HOST` write
- no source-artifact mutation
- no registry, graph, or `project_manifest` mutation
- manifest contains inputs, configuration, `owner_ref`, outputs, status, and trace data
- manifest contains no private absolute host paths
- result, manifest, and trace metadata contain no secrets

## F12.4 governed file intake storage alignment

`docs/specs/file_intake.md` owns file intake semantics.

F12.4 declares future intake storage only. It creates no files, blobs, manifests, registry entries, graph entries, indexes, or derivatives.

Future intake storage must preserve these storage rules:

- physical storage must be governed
- blobs are canonical physical storage
- refs declare usage
- types classify/navigation only
- indexes are derivable accelerators
- no project-root `outputs/`
- no `assets/` as runtime
- no passive filesystem exposure
- `project_manifest.json` remains exposure authority
- source host paths must not be persisted as portable truth

F12.4 intake comments are future governed metadata only.

Comment storage rules:

- comments belong to `IntakeItem -> StoredObject metadata`
- comments are not separate documents
- comments are not chat logs
- comments are not filesystem annotations
- comments are not blobs
- comments must be sanitized before durable storage
- comments must not contain secrets, credentials, tokens, raw sensitive data, or private absolute host paths
- comments do not create project exposure
- comments do not authorize derivation

## F12.5 file intake storage plan

`F12.5` is plan-only and creates no files.

Future `F12.6` may write only to an owner-scoped intake directory or governed file-store location when the storage policy path is explicit.

Future `F12.6` may create metadata JSON for an intake item or stored object candidate, and may create an optional batch manifest only if explicitly declared before implementation.

Future `F12.6` must not create project-root outputs, write to source folders, use `assets/` as runtime storage, mutate `project_manifest.json`, generate `registry.json`, write graph entries, or generate derivatives.

Future intake metadata must not persist private absolute host paths as portable truth.

## F12.8 intake dedup hardening storage strategy

`F12.8` is SPEC-ONLY / HARDENING-STRATEGY-ONLY.

It opens no new runtime and creates no files.

Future dedup hardening must preserve:

- `file_store/blobs/<sha256_hash>/` as the only physical blob authority
- `file_ref` as content identity
- `StoredObject` as logical metadata and usage context
- `intake_item_id` as the explicit selection event identity
- `project_manifest` as the only project exposure authority

Future dedup hardening must not create project-root outputs, write to source folders, use `assets/` as runtime storage, mutate `project_manifest.json`, generate `registry.json`, write graph entries, generate derivatives, call `document_text_runtime`, or use `tool_runtime` orchestration.

## F13.0 project exposure storage alignment

`F13.0` is SPEC-ONLY / GATE-ONLY and creates no files.

Project exposure remains separate from storage:

- `file_store/blobs/<sha256_hash>/` is physical blob authority
- `StoredObject` is logical metadata and usage context
- `file_ref` is content identity
- `project_manifest.json` is the only project exposure authority

Storage presence must not create exposure:

- blob presence does not expose a file to a project
- `StoredObject` metadata presence does not expose a file to a project
- intake metadata presence does not expose a file to a project
- registry presence does not expose a file to a project
- graph presence does not expose a file to a project

Future project exposure may reference governed storage objects only through sanitized refs and after request, candidate, and human confirmation defined by `docs/specs/file_intake.md`.

F13.0 must not create project-root outputs, write source folders, use `assets/` as runtime storage, mutate `project_manifest.json`, generate `registry.json`, write graph entries, generate derivatives, call `document_text_runtime`, call `tool_runtime`, or create rollback/revocation storage effects.

## F13.1 legacy storage-bypass hardening

`F13.1` is SPEC-only and creates no files.

Storage and copied files remain non-authoritative even when legacy helpers exist:

- a file copied by `import_project_document` must not become exposed by storage presence
- chat-resource storage must not become project storage authority
- derivative files must not become exposure authority
- any legacy copy path must remain separate from manifest-governed exposure

## F13.4 manifest exposure storage hardening

`F13.4` is SPEC-only and creates no files.

Future manifest exposure remains storage-aware but storage-non-authoritative.

Rules:

- `ManifestExposureEntry` may reference governed stored objects only through portable refs
- blob reuse is allowed but must not collapse logical exposure identity, request identity, candidate identity, confirmation identity, comments, `owner_ref`, or `trace_ref`
- manifest exposure entries must not persist private absolute host paths, raw payloads, or secrets
- manifest exposure entries must not imply `registry.json` generation, graph writes, or derivative creation
- storage presence, checksum presence, or stored-object metadata presence must remain insufficient for project exposure

## F13.5A manifest exposure storage checklist

`F13.5A` is SPEC-only and creates no files.

Future storage checklist for `F13.5B`:

- manifest exposure may reference existing governed storage only through portable refs
- no raw host absolute path may appear in manifest exposure outputs
- duplicate physical blob reuse must remain storage-level only and must not collapse logical exposure identity
- exposure runtime must not create registries, graph artifacts, or derivatives as storage side effects

---

### !REL_FILE!

# structured_content_assets

## Purpose

Document the future treatment of figures, tables, and equations as governed structured content.

This is a proposal only. It does not design a renderer, export pipeline, or runtime implementation.

## Core idea

Markdown artifacts may contain references to structured assets.

The structured assets live beside the artifact or inside governed project content with their own manifests.

Future structured asset classes may include:

- figures
- tables
- equations

## Manifest ownership

Each structured asset should have a manifest that records:

- stable identifier
- source relation
- artifact relation
- editable metadata fields
- technical readonly fields
- provenance

The manifest is the governed contract for inspection and patch validation.

## Editable metadata

Editable fields may include:

- title
- caption
- alt text
- notes
- display label
- logical role

Edits to these fields must use governed patch proposals.

## Technical readonly fields

Technical fields are not directly editable through the future document editing workflow.

Examples:

- content hash
- binary path
- source extraction coordinates
- page origin
- generated dimensions
- derived text checksum

## Markdown references

A future artifact may reference structured assets from Markdown rather than embedding all structured content inline.

Possible reference kinds:

- figure reference
- table reference
- equation reference

The exact syntax is intentionally out of scope for this phase.

## Phase limits

This spec does not implement:

- structured asset renderer
- editor surface
- export runtime
- equation compiler
- table editor
- figure processing pipeline

It records only the governance principle for future phases.

---

### !REL_FILE!

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

---

### !REL_FILE!

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

---

### !REL_FILE!

# text_selection

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define governed text selection in `Document Viewer` as a structured reference.

Text selection must not become a new source of truth.

It must remain a governed reference to a document context that may later feed popup capture, chat references, and future transformation workflows.

## Core rule

Text selection references a governed document.

It does not modify the document.
It does not become an independent document copy.
It does not replace the governed source.

## Canonical model

Conceptual selection reference may be named `TextSelectionRef` or `ContextSelectionRef`.

Minimum conceptual fields:

- `selection_ref`
- `document_ref`
- `range_start`
- `range_end`
- `selected_text_hash`
- `selected_text_preview`
- `document_version_or_hash`
- `created_at`
- `status`

Field meaning:

- `selection_ref`: stable governed identifier for the prepared selection reference
- `document_ref`: governed target document reference
- `range_start`: start position in the governed rendered or textual model
- `range_end`: end position in the governed rendered or textual model
- `selected_text_hash`: integrity hash of the selected text snapshot
- `selected_text_preview`: non-authoritative preview text for display only
- `document_version_or_hash`: document version marker used to detect drift or stale selection state
- `created_at`: creation timestamp for trace and ordering
- `status`: prepared selection state

## Rules

- selection does not modify the document
- selection references the document
- selection must not lose `document_ref`
- selection must not use host absolute paths
- if the document changes, the selection becomes `stale`
- if `selection_ref` cannot be resolved, any transformation depending on it must be blocked

Normative form:

- `INV-TSEL-001`: text selection MUST NOT modify the document
- `INV-TSEL-002`: text selection MUST retain `document_ref`
- `INV-TSEL-003`: text selection MUST NOT use host absolute paths as identity or source of truth
- `INV-TSEL-004`: document drift MUST move the selection to `stale`
- `INV-TSEL-005`: unresolved `selection_ref` MUST block transformation preparation that depends on it
- `INV-TSEL-006`: `selected_text_preview` is display aid only and MUST NOT become project source truth

## Relationship to Document Viewer

`Document Viewer` captures visual text selection.

Prepared selection state may be passed to popup or other governed context as structured reference.

The viewer does not decide permissions and does not authorize transformations.

Text selection does not turn the viewer into an editor.

## Relationship to chat

Text selection may be sent to chat as a structured reference.

Chat must not treat selected text as an opaque blob source of truth.

If text content is shown in chat, it remains explanatory only; the governed reference remains authoritative.

## Relationship to transformation

Future transformation requests may use `selection_ref` as optional scoped context.

`selection_ref` must remain linked to:

- `document_ref`
- document version or hash
- selected text integrity hash

This spec does not define popup behavior, patch preview, diff behavior, or transformation execution.

## States

Prepared selection state may include:

- `active`
- `cleared`
- `stale`
- `missing`
- `invalid`

Interpretation:

- `active`: valid selection reference exists and matches current document context
- `cleared`: selection was intentionally removed and no active selection remains
- `stale`: selection reference exists but no longer matches current document version or hash
- `missing`: `selection_ref` no longer resolves
- `invalid`: range or integrity constraints are not valid for current prepared context

## Failure modes

- `empty_selection`
- `selection_out_of_range`
- `document_hash_mismatch`
- `selection_ref_missing`
- `selection_stale`

Interpretation:

- `empty_selection`: no non-empty governed selection was captured
- `selection_out_of_range`: prepared range is outside the valid governed document model
- `document_hash_mismatch`: captured selection no longer matches current document version or hash
- `selection_ref_missing`: `selection_ref` no longer resolves
- `selection_stale`: selection exists but is no longer current enough for safe transformation preparation

## Forbidden responsibilities

Text selection must not:

- create a new durable source document
- detach from `document_ref`
- bypass `Document Viewer`
- authorize transformation by itself
- mutate document content
- replace `ActionResolution`

## Related specs

- `docs/specs/document_viewer.md`
- `docs/specs/active_object_context.md`
- `docs/specs/chat_document_flows.md`
- `docs/specs/transformation_core.md`
- `docs/specs/transform_popup.md`

---

### !REL_FILE!

# tools_catalogs

## Purpose

Define the F9-ready declarative catalog layout for internal operational tools, internal LLM tools, external runtime dependencies, and the proposal-only master capability catalog.

Catalogs declare availability and policy shape. They do not implement execution.

Catalogs are functional domains.

They are not menu surfaces, not UI authority, and not physical runtime locations.

## Current canonical source

`resources/tool_runtime/* is the current operative canonical source` for the existing `tool_runtime` crate.

The F8/F9 sandbox still treats:

- `resources/tool_runtime/meta_catalog.json`
- `resources/tool_runtime/llm_tool_policy.json`

as the runtime-consumed declarations.

`resources/tools/* is not consumed by runtime in the current phase`.

## Scope F9

F9 declares:

- `resources/tools/tools_metacatalog.json`
- `resources/tools/tools_master_catalog.json`, proposal only
- operational document tools catalog
- empty LLM document tools catalog
- external tools catalog containing only `tectonic`

This layout is F9-ready and declarative only.

The metacatalog indexes catalogs, not concrete execution logic.

Tool implementation governance is defined in `docs/specs/tool_implementation_governance.md`.

The practical declaration workflow is summarized in:

- `docs/specs/how_to_add_a_tool.md`

Catalogs are source declarations.

Catalogs may declare tool identity and possible capabilities, but catalog presence alone does not authorize any capability.

An LLM-facing tool surface is derived and policy-filtered.

The full raw catalog is not injected into LLM context by default.

Operational tools may declare `document_profile` as enabled and requiring confirmation, but no new execution is implemented by this documentation update.

Operational tools may also declare future non-executable entries such as `merge_pdfs` with `execution_enabled=false`. Such entries are F9-ready declarations only and do not create runtime behavior.

LLM tools remain empty.

External tools declare Tectonic as optional and missing.

Declared does not mean installed, validated, or executable.

External tools are not executable when:

- `execution_implemented` is `false`
- runtime status is `missing`
- the declared binary is not present
- the future runtime dependency validation has not passed

`enabled=true` in declarative catalogs does not imply executable behavior when `execution_enabled=false` or `execution_implemented=false`.

Legacy or minimal tool declarations must progressively declare `status`, canonical `tool_kind`, schemas, and execution flags before becoming executable.

## Tools menu domain

`Tools` is a top-level menu domain separate from `Preferences`.

It exposes exactly:

- `Operational Tools`
- `LLM Tools`
- `External Dependencies`

These menu entries do not redefine catalogs.

They route into governed inspection or preparation surfaces only.

There is no single combined tools panel with tabs.

Catalog classification remains independent from menu placement.

## F9 master catalog proposal

F9 may also declare:

- `resources/tools/tools_master_catalog.json`

This file is a proposal-only declarative master catalog for tool identity and capability mapping.

In F9:

- master catalog = declarative master source
- current `tool_runtime` = operative runtime source
- `EffectiveToolSurface` is derived separately
- visibility is derived by policy
- `implementation_ref` is indirect and non-executing

The master catalog does not replace current per-domain catalogs in F9.

The master catalog does not replace:

- `resources/tools/tools_metacatalog.json`
- operational, LLM, or external catalogs
- `resources/tool_runtime/*`

Its purpose is to unify tool identity without opening execution or changing current consumers.

The master catalog should use functional domains, not surfaces or physical placement, for `catalog` values.

Current proposed master-catalog domains are:

- `core_base`
- `document_ops`
- `semantic_ops`
- `external_runtime`

Do not use master-catalog domain values such as:

- `llm`
- `operational`
- `internal`

## Master catalog shape

Each master entry should declare at least:

- `tool_id`
- `capability_id`
- `catalog`
- `tool_kind`
- `surfaces`
- `status`
- `declaration`
- `visibility`
- `implementation_ref`
- `execution_enabled`
- `proposal_only`

Future capability modeling is governed separately by `docs/specs/tool_capability_model.md`.

Interpretation:

- `declaration`: canonical tool identity, semantic role, and stable descriptive contract
- `tool_kind`: one of `operational`, `llm`, `external`, `agent`, or `base`
- `surfaces`: declarative surface hints only; not effective surface authority
- `visibility`: policy-derived exposure hints; not effective visibility truth
- `implementation_ref`: indirect reference to operative or future implementation boundary; not execution authority

Canonical `tool_kind` meanings:

- `operational`: deterministic system action that may become executable only in a later governed phase
- `llm`: capability available to LLM reasoning surfaces; not manually executable in F9
- `external`: external binary or runtime dependency; not a tool by itself
- `agent`: composite tool-like provider shape; future-only unless explicitly opened
- `base`: internal support capability; not user-facing

`tool_kind` defines classification only.

It does not define execution.

Tool visibility does not imply execution permission.

LLM tools are not agent roles.

Legacy `tool_class` is deprecated as an alias only.

If historical declarations still contain `tool_class`, they should map as follows:

- `operational` -> `operational`
- `llm` -> `llm`
- `external_dependency` -> `external`
- `base_utility` -> `base`

Agent roles are governed through `tool_kind=agent`, not through legacy `tool_class`.

`capability_id` must be unique across the master catalog.

One capability must not be duplicated across operational, LLM, external, or support surfaces as separate semantic truths.

`execution_enabled` must remain `false` for F9 and F10_PREP declarative master-catalog entries.

`proposal_only` must remain `true` for F9 and F10_PREP declarative master-catalog entries.

`json.validate` may be declared only as `base` and should remain `hidden_support`, not a public user tool.

Do not declare `json_read` as a public master-catalog capability.

`external_runtime` entries do not install or execute binaries in F9.

`external.tectonic` is a runtime-dependency identity only. It does not open installation or execution behavior.

## Minimal F9/F10_PREP subset

The minimal proposal subset for `tools_master_catalog.json` may include 8 to 10 capabilities:

- `document.profile`
- `document.extract_text`
- `document.merge_pdfs`
- `document.summarize`
- `text.measure`
- `file.diff`
- `json.validate`
- `semantic.extract_concepts`
- `semantic.propose_relations`
- `external.tectonic`

This subset is sufficient to establish master identity and capability coverage without opening runtime execution.

The F9 subset is frozen.

New capabilities require an explicit phase or governance decision.

Do not add tools for convenience.

Do not duplicate capabilities by surface.

## Governance block

The master catalog may contain an internal `governance` block.

That block should declare rules such as:

- execution disabled
- no runtime authority
- no raw catalog injection into LLM context by default
- binaries in `resources/` forbidden
- UI non-executing
- indirect implementation resolution only

The governance block is declarative metadata only.

It does not create runtime policy by itself.

## Coexistence rule

Do not merge the catalogs in F9.

Do not move files between `resources/tool_runtime/*` and `resources/tools/*`.

Do not make the UI or runtime consume `resources/tools/*` until F10 explicitly opens a controlled transition.

Do not make the master catalog the operative runtime source in F9.

## Catalogs vs surfaces vs storage

Functional catalogs remain:

- `core_base`
- `document_ops`
- `semantic_ops`
- `external_runtime`

These values classify capability domain only.

They do not mean:

- top-level menu category
- execution surface
- runtime path
- permission

Surface remains derived separately.

Visibility remains policy-derived and non-authoritative.

Physical storage remains governed by runtime layout and external dependency governance, not by catalog domain.

## External dependency storage boundary

`external_runtime` entries remain declaration-only capability or dependency identities.

They do not install binaries and do not execute binaries.

User-managed downloaded or referenced external dependencies belong under:

- `user/runtime/external_dependencies/`

This user runtime root does not make the dependency executable.

It does not replace the current operative `resources/tool_runtime/*` source.

It does not authorize `tool_runtime` to consume `resources/tools/*`.

## LLM catalog note

`resources/tools/internal/llm/tools_llm_document_catalog.json` may remain empty in current F9 declarations.

An empty LLM catalog is compatible with F9.

The `LLM Tools` panel may therefore show an empty declared surface without opening execution or widening master-catalog authority.

## Master catalog and effective surface boundary

### MasterCatalog

`resources/tools/tools_master_catalog.json` is the declarative source of tool capabilities.

It:

- does not execute
- does not authorize
- does not resolve availability
- does not replace `resources/tool_runtime/*` in F9

### EffectiveToolSurfaceResolver

`EffectiveToolSurfaceResolver` is a future mandatory component before any operative consumption of master-catalog declarations.

It must derive `EffectiveToolSurface` from:

- `MasterCatalog`
- phase policy
- project policy
- global tool policy
- implementation availability
- external dependency availability
- credentials or capability state, when applicable

It must not live in UI.

It must not be the LLM.

It must not bypass `ActionResolution`.

### EffectiveToolSurface

`EffectiveToolSurface` is derived output only.

It may contain only effective capabilities and states such as:

- `declared`
- `visible`
- `allowed`
- `implemented`
- `available`
- `executable`

`visible` is not permission.

`surfaces` is not execution availability.

`executable` may become `true` only in a later explicitly opened phase.

Effective surface is not the raw catalog.

Future structured surface details are governed by `docs/specs/llm_tool_surface_resolution.md`.

### Explicit prohibitions

- `tool_runtime` MUST NOT consume `resources/tools/tools_master_catalog.json` directly in F9.
- UI MUST NOT derive tool permissions from `resources/tools/tools_master_catalog.json`.
- LLM MUST NOT receive the raw full master catalog by default.
- `visibility` MUST NOT be interpreted as permission.
- `surfaces` MUST NOT be interpreted as execution availability.
- `external_runtime` MUST NOT imply binary installation or execution.
- `tool_id` MUST NOT be interpreted as execution authority by itself.

## F10 step-1 closure

The first explicit F10 opening gate does not make tool catalogs executable.

In F10 step 1:

- tool catalogs remain declarative sources
- `tools_master_catalog.json` remains non-operative as direct runtime input
- only bounded `EffectiveToolSurfaceSummary` runtime consumption may open
- tool execution remains closed
- provider binding is not implied by catalog presence

## F11.0 execution gate note

`F11.0` is DECLARED / NOT ACTIVE.

Catalog declaration does not imply execution.

Executable eligibility must be resolved by future governed policy.

Raw catalog presence is not authority.

## F12.0 / F11.RUNTIME-0 catalog proposal note

`F12.0 / F11.RUNTIME-0` is proposal-only and does not make any catalog executable.

The first future runtime proposal may consider only one deterministic local tool whose `tool_kind` is `operational` or `base`.

Recommended first tool: `text.measure`.

Catalog entries for `merge_pdfs`, external tools, LLM tools, and agent tools remain non-executable until a later explicit phase opens them.

Catalog declaration, `enabled=true`, visibility, or implementation metadata must not be used by UI, LLM, or runtime to infer execution authority.

## F12.1 text.measure catalog gate

`F12.1` is gate-only and does not make any catalog executable.

The only tool that may be considered for the first later implementation slice is:

- `text.measure`

Catalog eligibility does not imply execution.

`text.measure` may proceed to a later implementation proposal only if its future runtime slice remains one local deterministic `SANDBOX`-only tool with mandatory `owner_ref`, mandatory `trace_ref`, mandatory `ToolRunManifest`, and mandatory `TraceRecord`.

`merge_pdfs`, external tools, LLM tools, and agent tools remain outside the first gate.

### Future transition gate

Any connection between `tools_master_catalog.json` and `tool_runtime` requires the explicit future phase:

- `F10_PREP_EFFECTIVE_TOOL_SURFACE`

That phase must introduce its own tests and validators.

Until that phase opens explicitly, `resources/tool_runtime/*` remains the current operative source.

## Future roadmap note: F10_PREP_EFFECTIVE_TOOL_SURFACE

`F10_PREP_EFFECTIVE_TOOL_SURFACE` is a future proposal only.

Objective:

- derive `EffectiveToolSurface` from declarative capability identity and governed policies without making the master catalog an operative source by itself

Inputs:

- `MasterCatalog`
- phase policy
- project policy
- global tool policy
- implementation availability
- external dependency availability
- credentials or capability state, when applicable

Output:

- `EffectiveToolSurface`

Prohibitions:

- no UI authority
- no LLM authority
- no direct `MasterCatalog -> tool_runtime`
- no execution without a later explicit phase

## Forbidden responsibilities

Tool catalogs must not:

- execute tools
- call LLMs
- download binaries
- add Pandoc, Poppler, Tesseract, LibreOffice, SevenZip, ripgrep, or jq yet
- bypass `tool_runtime`
- duplicate project pipeline behavior
- store binaries in `resources/`
- collapse declaration, effective surface, visibility policy, and implementation authority into one field

## Future F10/F11 notes

F10 may migrate `tool_runtime` toward the `resources/tools/*` layout through a controlled transition.

That transition should:

- preserve current `tool_runtime` behavior until replacement is validated
- avoid duplicate effective-policy truth
- keep operational, LLM, and external runtime dependency concepts separate
- treat legacy `resources/tool_runtime/*` as fallback or migration input only after an explicit decision

F11 should verify enabled states, confirmation requirements, external binary validation, and policy separation.

## F12.4 file intake catalog alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

Future intake tools may be declared later as narrow `operational` or `base` tools only.

`F12.4` does not add, implement, activate, or execute intake tools.

Catalog presence must not imply file intake runtime availability.

## F9 Invariants (Non-Negotiable)

- The master catalog MUST remain declarative and MUST NOT be executable in F9.
- The master catalog MUST NOT be consumed by `tool_runtime` in F9.
- The master catalog MUST NOT define `EffectiveToolSurface` directly.
- `execution_enabled=false` MUST remain mandatory and binding for every master-catalog entry in F9.
- `proposal_only=true` MUST remain mandatory and MUST prevent any execution in F9.
- No tool declared in the master catalog MAY execute in F9.
- No `external` entry declared in the master catalog MAY install or execute in F9.
- LLM capabilities declared in the master catalog MUST remain proposal-only and MUST NOT activate real pipelines in F9.
- UI surfaces MUST NOT interpret, authorize, or execute tools from the master catalog in F9.
- `project_runtime` MUST NOT depend on the master catalog in F9.
- `tool_kind` is the canonical governed classification term.
- Only the defined `tool_kind` values are valid: `operational`, `llm`, `external`, `agent`, `base`.
- `tool_kind` MUST be explicit for every governed tool declaration.
- No tool declaration MAY rely on implicit or inferred `tool_kind`.
- Legacy `tool_class` may remain only as a deprecated alias in historical declarations and must not override `tool_kind`.
- No new `tool_kind` value may be introduced without an explicit governance update.
- Violation of any invariant implies unintended transition to F10.
- The first F10 opening gate MUST NOT convert catalog visibility, declaration, or summary presence into execution authority.

---

### !REL_FILE!

# tools_panel

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed tools-management surfaces for DocGraph without opening orchestration, runtime execution, or installer behavior.

`Tools` is a top-level menu domain.

It is not part of `Preferences`.

## Core rule

There is no single combined tools panel with tabs.

The `Tools` menu exposes exactly three governed entries:

- `Operational Tools`
- `LLM Tools`
- `External Dependencies`

Each entry opens its own panel or popup surface.

Each surface emits intent only.

No surface executes tools, installs dependencies, calls LLM, or interprets policy as authority.

## Governed i18n keys

The `Tools` menu contract uses these stable menu keys:

- `menu.tools`
- `menu.tools.operational`
- `menu.tools.llm`
- `menu.tools.external_dependencies`

The governed Tools surfaces use these popup keys:

- `popup.tools.operational.title`
- `popup.tools.operational.description`
- `popup.tools.llm.title`
- `popup.tools.llm.description`
- `popup.tools.external_dependencies.title`
- `popup.tools.external_dependencies.description`

Popup descriptions must remain informational only.

They must not imply execution, installation, dependency readiness, or LLM availability.

Legacy menu keys may remain in resources for compatibility, but new specs must not reference:

- `menu.tools.open_panel`
- `menu.tools.external`

## Operational Tools panel contract

`Operational Tools` is the governed inspection and preparation surface for internal DocGraph operational tools.

Declaration source:

- `resources/tools/internal/operational/tools_operational_document_catalog.json`

The panel may show:

- tool list
- tool id
- display name
- description
- status
- execution availability
- dependencies, if any
- selected tool inspector

The panel may allow:

- selecting one declared operational tool
- opening `Prepare execution`
- inspecting tool input and output contract
- inspecting owner and persistence requirements

The panel must not:

- execute on menu click
- execute on tool selection
- execute on `Prepare execution`
- chain tools
- create workflow graphs
- act as `LLM Tools` surface

## Operational Tools prepare-execution popup

`Prepare execution` opens a governed popup.

The popup captures structured intent only.

It may capture:

- inputs
- configuration
- ordered input policy, when relevant
- `owner_ref`
- output destination
- risk summary
- confirmation intent

The popup emits preparation intent only.

It must not execute.

Any future execution must pass through `ActionRequest -> ActionResolution -> authorized executor -> trace`.

## LLM Tools panel contract

`LLM Tools` is the governed inspection surface for internal LLM tool declarations.

Declaration source:

- `resources/tools/internal/llm/tools_llm_document_catalog.json`

The panel may show:

- declared LLM tools
- visibility and status
- future availability state
- capability limits
- reason codes, when applicable
- prepared `LLMCapabilityState` summary
- prepared `EffectiveToolSurface` state
- available, disabled, missing, and explanation states

The panel must not:

- show `Run`
- show `Prepare execution`
- manually execute tools
- bypass `EffectiveToolSurfaceResolver`
- inject raw full tool catalogs into LLM context
- compute permissions
- decide effective LLM mode
- call providers
- validate credential secrets

The surface is valid even when the current LLM tools catalog is empty.

## Capability state consumption

The Tools Panel consumes prepared state only.

It may display:

- `desired_llm_mode`
- `effective_llm_mode`
- `interaction_mode`
- `llm_available`
- `tool_surface_available`
- `execution_enabled`
- `reason_codes`

It must not derive these values from raw catalogs, UI state, credential fields, or filesystem presence.

Tool visibility does not imply execution permission.

If `execution_enabled=false`, the panel may explain `execution_not_open`, but must not offer execution controls.

## External Dependencies panel contract

`External Dependencies` is the governed inspection surface for external binaries or software required by internal tools.

External dependencies are not tools by themselves.

Declaration source:

- `resources/tools/external/tools_external_catalog.json`

User-managed dependency root:

- `user/runtime/external_dependencies/`

The panel may show:

- dependency list
- which internal tool or capability may require the dependency
- installed, missing, or invalid status
- expected binary name
- expected local path
- source URL
- download notes
- validation state

The panel must not:

- execute dependencies
- install automatically
- modify `PATH`
- run installers
- delete external binaries outside DocGraph-managed runtime folders

## Install / Configure popup

If an external dependency is missing, the panel may expose `Install / Configure`.

This opens a governed popup.

The popup may show:

- dependency name
- description
- why DocGraph may need it
- recommended source URL
- recommended version or version range
- expected binary name
- expected destination under `user/runtime/external_dependencies/`
- manual steps for the user
- button to select an existing downloaded binary or folder
- button to validate reference

The popup must not:

- download automatically
- execute installers
- mutate system folders
- store secrets
- treat validation as execution authorization

Validation records declared metadata and intended availability only.

## Custom external dependency flow

The `External Dependencies` surface may expose:

- `Add custom external dependency`

This opens a governed popup where the user may declare:

- dependency id
- display name
- source URL
- download notes
- expected binary name
- expected version, optional
- local path under `user/runtime/external_dependencies/` or selected reference
- `trusted_by_user`
- reason or use note

Rules:

- custom dependency remains user-declared
- custom dependency is not automatically trusted by DocGraph
- custom dependency does not create a new internal tool
- custom dependency does not enable execution
- custom dependency may only become referenced by future internal tools through governed catalog or policy updates

## Remove / Delete model

Two governed cases exist.

### Managed dependency

A managed dependency under `user/runtime/external_dependencies/` may expose:

- `Delete dependency`

It requires explicit confirmation.

The deletion updates dependency status only.

### Referenced dependency

A referenced dependency outside the DocGraph-managed runtime folder may expose:

- `Remove reference`

It must not physically delete the external binary or folder.

Removing the reference does not delete the internal tool declaration.

If a required dependency becomes unavailable, the internal tool becomes `unavailable_missing_dependency`.

## Invariants

- `INV-TOOLS-PANEL-001`: there MUST NOT be a single combined tools panel with tabs.
- `INV-TOOLS-PANEL-002`: `Operational Tools`, `LLM Tools`, and `External Dependencies` MUST open separate governed surfaces.
- `INV-TOOLS-OP-001`: operational tools are internal DocGraph tools.
- `INV-TOOLS-OP-002`: operational tools MAY be manually prepared but MUST NOT execute directly from menu click.
- `INV-TOOLS-OP-003`: operational-tool execution preparation MUST use a governed popup.
- `INV-TOOLS-LLM-001`: LLM tools MUST NOT be manually executable.
- `INV-TOOLS-LLM-002`: LLM tools expose governed policy or surface only.
- `INV-TOOLS-EXT-001`: external dependencies are not tools.
- `INV-TOOLS-EXT-002`: external dependencies are binaries or software required by internal tools.
- `INV-TOOLS-EXT-003`: user-managed external dependencies MUST use `user/runtime/external_dependencies/` as the governed user runtime root.
- `INV-TOOLS-EXT-004`: downloaded external dependencies MUST NOT live under `resources/`, `assets/`, `crates/`, or project folders.
- `INV-TOOLS-EXT-005`: external dependency installation MUST NOT be automatic in F9.
- `INV-TOOLS-EXT-006`: external dependency validation MUST NOT imply execution permission.
- `INV-TOOLS-EXT-007`: managed dependencies may be deleted only from DocGraph-managed user runtime folders.
- `INV-TOOLS-EXT-008`: referenced external binaries outside DocGraph runtime folders MUST NOT be physically deleted.
- `INV-TOOLS-ACTION-001`: all future tool execution MUST go through `ActionResolution`.
- `INV-TOOLS-ACTION-002`: UI panels and popups MUST emit intent only; they MUST NOT execute.
- `INV-TOOLS-I18N-001`: visible Tools labels and governed popup text MUST come from i18n resources.
- `INV-TOOLS-CAP-001`: Tools Panel MUST consume prepared capability and EffectiveToolSurface state only.
- `INV-TOOLS-CAP-002`: Tools Panel MUST NOT compute permissions, effective LLM mode, or execution availability from UI state.
- `INV-TOOLS-CAP-003`: Tool visibility MUST NOT imply execution permission.
- `INV-TOOLS-CAP-004`: raw tool catalogs MUST NOT be injected into LLM context by the Tools Panel.
- `INV-TOOLS-CAP-005`: LLM tool intent MUST become proposal only and MUST NOT execute directly.

## Out of scope

- runtime execution
- filesystem mutation beyond future governed runtime authority
- real download
- installer execution
- provider calls
- LLM calls
- LLM mode resolution
- credential secret validation
- automatic dependency validation
- `PATH` modification
- tool runner expansion
- UI business logic

## Related specs

- `docs/specs/app_main_menu.md`
- `docs/specs/tools_catalogs.md`
- `docs/specs/tool_implementation_governance.md`
- `docs/specs/how_to_add_a_tool.md`
- `docs/specs/external_dependency_manager.md`
- `docs/specs/action_resolution.md`

---

### !REL_FILE!

# tool_capability_model

## Status

DECLARED_ONLY / F9-F11_PREP governance.

This document does not implement runtime capability resolution, authorization, dispatch, tool execution, provider invocation, runners, or UI authority.

## Purpose

Define a governed capability-based model for tools that separates:

- `ToolIdentity`
- `ToolCapability`
- `CapabilitySurface`
- `CapabilityRequirement`
- future authorization

The goal is to make future execution governable by capabilities rather than by direct tool names alone.

## ToolIdentity

`ToolIdentity` is stable tool identity.

Conceptual structure:

```json
{
  "tool_id": "pdf_merge",
  "tool_kind": "operational | llm | external | agent | base",
  "implementation_ref": "...",
  "visibility": "visible | hidden | internal"
}
```

Rules:

- tool identity does not imply permission
- tool presence does not imply execution
- tool visibility does not imply authorization
- `tool_id` is not an execution grant

## ToolCapability

`ToolCapability` is an atomic governable ability.

Initial controlled vocabulary:

- `read_file`
- `write_file`
- `create_file`
- `delete_file`
- `read_metadata`
- `write_metadata`
- `extract_text`
- `merge_documents`
- `validate_json`
- `measure_text`
- `call_llm`
- `call_external_binary`
- `access_network`
- `generate_semantic_quads`
- `project_rdf`
- `analyze_graph`

Rules:

- capabilities are declarative
- capabilities do not execute
- capabilities may be required by future `ActionIntent`, `ActionRequest`, or plan steps
- capability presence does not imply authorization
- capability declaration does not expand domain access by itself

## CapabilitySurface

`CapabilitySurface` is the readonly declared capability set exposed by a tool.

Conceptual structure:

```json
{
  "tool_id": "pdf_merge",
  "capabilities": [
    "read_file",
    "write_file",
    "merge_documents"
  ],
  "surface_status": "declared | implemented | disabled | future"
}
```

Rules:

- capability surface is readonly
- capability surface is not authorization
- effective surface may be filtered by policy in the future
- `disabled` or `future` capabilities must not be selected for execution

## CapabilityRequirement

`CapabilityRequirement` is the capability set needed by an action, request, or plan step.

Conceptual structure:

```json
{
  "required_capabilities": [
    "read_file",
    "write_file"
  ],
  "required_domain": "SANDBOX | HOST | EXTERNAL | UNSPECIFIED",
  "permission_hint": "read | write | delete | network | execute"
}
```

Rules:

- future actions reference required capabilities
- tools are candidate providers only
- requirement matching does not execute anything
- requirement matching does not authorize anything
- the canonical domain vocabulary is uppercase:
  - `SANDBOX`
  - `HOST`
  - `EXTERNAL`
  - `UNSPECIFIED`

## Tool kinds

Controlled `tool_kind` values:

- `operational`
- `llm`
- `external`
- `agent`
- `base`

Rules:

- `base` tools may be hidden or internal
- `external` tools require dependency governance
- `agent` tools are composite and future-only unless separately opened
- `llm` tools do not execute directly
- `operational` tools may become future execution candidates only through governed action flow
- `tool_kind` is the canonical governed classification term
- legacy `tool_class` references, if present in older documentation, are deprecated aliases only

## Authorization boundary

Core distinction:

`capability declared != capability authorized`

Future authorization may consider:

- sandbox scope
- user confirmation
- project policy
- tool policy
- security and sanitization policy
- cost policy
- credentials policy
- capability status

Domain boundaries are governed separately by `docs/specs/sandbox_boundary_model.md`.

But in the current phase:

- no authorization runtime is opened
- no capability resolver is implemented
- no execution is selected

## Relationship with current specs

Catalogs declare tools and possible capabilities.

The capability model defines governable abilities.

`ActionResolution` may use capability requirements in the future.

`ActionIntent` and `ActionRequest` may use capability requirements before any future candidate-tool selection.

Future `ResolutionCandidate` artifacts may summarize capability evaluation results, but capability authority remains owned by this spec.

Future `AuthorizedExecutionRequest` artifacts may carry required capabilities in an execution-scope summary, but that summary does not grant execution and does not replace capability governance.

Future `SingleToolExecution` artifacts may bind only `operational` or `base` tool kinds for the first declared execution-contract scope.

That binding is contractual only: it does not invoke the tool, select execution at runtime, load binaries, or grant authority.

UI may present capabilities in the future, but it must not authorize them.

Future readonly presentation may summarize effective capability and tool-surface state through `docs/specs/system_view.md` without granting authority.

## Non-goals

This policy does not open:

- tool execution
- tool dispatch
- runners
- provider invocation
- `project_runtime` authority changes
- UI authorization

## Capability invariants

- `INV-CAP-001`: tools do not imply authorization.
- `INV-CAP-002`: capabilities are declarative.
- `INV-CAP-003`: capability surface is readonly.
- `INV-CAP-004`: future actions reference capabilities before tools.
- `INV-CAP-005`: capability declaration does not grant execution.
- `INV-CAP-006`: external tools require explicit future dependency governance.
- `INV-CAP-007`: agent tools are composite and non-executing unless separately opened.
- `INV-CAP-008`: disabled or future capabilities cannot be selected for execution.
- `INV-CAP-009`: UI must not authorize capabilities.
- `INV-CAP-010`: `project_runtime` remains unchanged unless a future phase explicitly opens authority.
- `INV-CAP-011`: `tool_kind` is the canonical governed classification term; `tool_class` is a deprecated alias only if retained in historical references.
- `INV-CAP-012`: the canonical domain enum is uppercase across action, capability, and sandbox contracts.

## Related specs

- `docs/specs/tools_catalogs.md`
- `docs/specs/tool_implementation_governance.md`
- `docs/specs/action_resolution.md`
- `docs/specs/action_contract.md`
- `docs/specs/security_sanitization_policy.md`
- `docs/specs/system_observability.md`
- `docs/specs/system_view.md`

---

### !REL_FILE!

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

---

### !REL_FILE!

# transformation_core

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Principle

DocGraph does not edit documents.

DocGraph transforms documents.

## Purpose

Centralize the transversal document-transformation model.

This spec avoids duplicating concepts across viewer, popup, diff, timeline, and future recipes.

It defines canonical names for instruction, proposal, preview, overlay, event, timeline, and future recipe concepts.

## Canonical concepts

- `TransformInstruction`
- `TransformProposal`
- `PatchPreviewOverlay`
- `PatchPreview`
- `TransformationEvent`
- `TransformTimeline`
- `TransformationRecipe`, future

## TransformInstruction

Conceptual fields:

- `instruction_id`
- `source`
- `raw_instruction`
- `target_ref`
- `selection_ref`, optional
- `created_at`

## TransformProposal

Conceptual fields:

- `proposal_id`
- `instruction_id`
- `target_ref`
- `selection_ref`
- `original_text_hash`
- `proposed_text`
- `attempt_index`
- `status`
- `created_at`

## PatchPreviewOverlay

Conceptual fields:

- `overlay_id`
- `proposal_id`
- `target_ref`
- `selection_ref`
- `overlay_kind`
- `overlay_state`
- `highlight_token`

## PatchPreview

`PatchPreview` is the governed preview representation associated with a transformation proposal.

It is the preview contract consumed by review surfaces.

It is not persistence, not acceptance, and not application.

Minimum conceptual relationship:

- one `TransformProposal` may produce one preview representation
- `PatchPreviewOverlay` is the in-view rendering layer for that preview
- preview may be represented in popup, diff, or future dedicated patch-preview surfaces

## TransformationEvent

Conceptual fields:

- `transformation_id`
- `proposal_id`
- `instruction_id`
- `target_ref`
- `selection_ref`
- `user_decision`
- `status`
- `trace_ref`
- `timestamp`

## TransformTimeline

`TransformTimeline` is the ordered trace of transformation attempts and decisions for a target or instruction lineage.

It may include:

- original `TransformInstruction`
- one or more `TransformProposal` attempts
- associated preview lifecycle
- user decisions
- supersession or cancellation events

It is a traceability surface, not an execution engine.

## TransformationRecipe

Future only.

`TransformationRecipe` is a future higher-level declarative structure for reusable transformation patterns.

It must build on canonical transformation concepts rather than redefining them.

## Canonical states

- `draft`
- `proposed`
- `pending_user_validation`
- `accepted`
- `cancelled`
- `superseded`
- `stale`
- `applied_future`
- `error`

## Invariants

- proposal does not modify document
- overlay is not persistence
- acceptance does not apply in F9
- regenerate creates a new `proposal_id`
- cancel clears overlay
- every attempt preserves traceability

Normative form:

- `INV-TRANSFORM-001`: a `TransformProposal` MUST NOT modify the document by itself.
- `INV-TRANSFORM-002`: `PatchPreviewOverlay` MUST NOT be treated as persistence.
- `INV-TRANSFORM-003`: accepting a proposal MUST NOT apply changes in F9.
- `INV-TRANSFORM-004`: regeneration MUST create a new `proposal_id`.
- `INV-TRANSFORM-005`: cancellation MUST clear the active overlay representation.
- `INV-TRANSFORM-006`: every attempt MUST preserve traceability to instruction, target, selection, and proposal lineage.

## Failure modes

- `invalid_target_ref`
- `stale_selection`
- `proposal_superseded`
- `proposal_without_instruction`
- `missing_original_hash`

## Relationship to existing specs

- `document_governed_editing.md` describes future governed editing flow; `transformation_core.md` defines the shared transformation vocabulary.
- `document_patch_runtime.md` may later validate, preview, and apply accepted patches; it should reuse `TransformProposal`, `PatchPreview`, and `TransformationEvent` terminology instead of redefining them.
- `active_context.md` provides target and selection context; it should remain the source of active-context interpretation.
- `workspace_tabs.md` may host future `patch_preview` views, but view hosting must not redefine transformation concepts.

## Specs that should reference this spec

- `docs/specs/document_governed_editing.md`
- `docs/specs/document_patch_runtime.md`
- `docs/specs/active_context.md`
- `docs/specs/workspace_tabs.md`

Potential future references:

- viewer selection popup spec
- diff or patch-preview spec
- transformation timeline spec
- transformation recipe spec

## Risks

- if future specs redefine proposal, preview, or event concepts locally, terminology drift will reappear
- if preview and overlay are conflated, UI state may be mistaken for persistence
- if accepted and applied are not kept distinct, F9 documentation may overstate execution

## Validation

Recommended:

```bat
dev\scripts\generate_llm_context_bundle.bat
dev\scripts\generate_status_snapshot.bat
dev\scripts\validate_f9_declarations.bat
```

---

### !REL_FILE!

# transformation_recipes

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the initial governed specification for `TransformationRecipe`.

Recipes prepare reusable declarative transformation patterns derived from prior governed transformation history.

They do not execute anything in F9.

## Definition

A recipe is a reusable declarative sequence of transformations.

It is not:

- execution
- a free macro
- a script

## Origins

A recipe may be:

- derived from `TransformTimeline`
- created manually in a future phase
- suggested by the system in a future phase

F9 defines the concept only. It does not implement recipe creation flows.

## Conceptual model

### TransformationRecipe

Minimum conceptual fields:

- `recipe_id`
- `display_name`
- `description`
- `origin`
- `steps`
- `reusable`
- `status`

Field meaning:

- `recipe_id`: stable governed recipe identifier
- `display_name`: visible human-oriented recipe name
- `description`: concise explanation of recipe purpose
- `origin`: provenance of the recipe, such as timeline-derived, manual-future, or system-suggested-future
- `steps`: ordered declarative `RecipeStep` sequence
- `reusable`: whether the recipe is intended for reuse when future governance allows it
- `status`: current declarative lifecycle state

### RecipeStep

Minimum conceptual fields:

- `step_id`
- `operation_kind`
- `target_policy`
- `instruction_template`
- `required_inputs`
- `output_policy`

Field meaning:

- `step_id`: stable ordered step identifier within the recipe
- `operation_kind`: declared transformation operation category
- `target_policy`: rule describing what kinds of governed targets the step may address
- `instruction_template`: reusable instruction pattern for the step
- `required_inputs`: declared inputs needed before future governed execution could be considered
- `output_policy`: rule describing the expected proposal or output treatment

## States

Prepared recipe state may include:

- `draft`
- `validated_future`
- `reusable_future`
- `deprecated`
- `invalid`

Interpretation:

- `draft`: initial declarative recipe shape exists but is not yet future-validated
- `validated_future`: reserved future state for a later governed validation phase
- `reusable_future`: reserved future state for later governed reuse
- `deprecated`: recipe should no longer be preferred
- `invalid`: recipe structure is not safe or not complete enough even as declarative pattern

## Rules

- no execution in F9
- no mutation
- no LLM invocation
- future application requires `ActionResolution`
- recipe does not substitute `tool_runtime`

Normative form:

- `INV-TRECIPE-001`: recipes MUST remain declarative in F9
- `INV-TRECIPE-002`: recipes MUST NOT execute transformations in F9
- `INV-TRECIPE-003`: recipes MUST NOT mutate files or documents by themselves
- `INV-TRECIPE-004`: recipes MUST NOT invoke LLMs directly
- `INV-TRECIPE-005`: any future recipe application MUST pass through `ActionResolution`
- `INV-TRECIPE-006`: recipes MUST NOT replace `tool_runtime` or create a parallel execution pipeline

## Relationship to timeline

`TransformTimeline` may feed recipe derivation.

The timeline preserves historical attempts and decisions.
The recipe abstracts reusable pattern from that history.

Timeline and recipe must remain distinct:

- timeline = memory and trace
- recipe = reusable declarative pattern

## Relationship to transformation_core

Recipes build on the canonical vocabulary from `transformation_core.md`.

They must not redefine:

- instruction lineage
- proposal identity
- preview semantics
- acceptance semantics

## Failure modes

- `empty_recipe`
- `invalid_step`
- `missing_target_policy`
- `unsupported_operation_kind`
- `unsafe_recipe_execution_blocked`

Interpretation:

- `empty_recipe`: recipe contains no usable declarative steps
- `invalid_step`: at least one step is structurally invalid
- `missing_target_policy`: a step lacks required target-governance policy
- `unsupported_operation_kind`: a step declares an operation kind outside governed support
- `unsafe_recipe_execution_blocked`: future execution must remain blocked because the recipe is unsafe or not authorized

## Invariants

- `INV-TRECIPE-007`: recipe steps MUST remain ordered declarative data, not embedded script logic
- `INV-TRECIPE-008`: recipe provenance SHOULD remain traceable to its origin when available
- `INV-TRECIPE-009`: reusable recipe intent MUST remain distinct from actual execution capability

## Related specs

- `docs/specs/transformation_core.md`
- `docs/specs/transform_timeline.md`

---

### !REL_FILE!

# transform_popup

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed contextual transformation popup for DocGraph.

The popup lets the user order a transformation over a valid text selection while keeping selected text readonly and user instruction editable.

DocGraph does not edit documents directly.

## Dependencies

This spec depends on:

- `docs/specs/transformation_core.md`
- `docs/specs/text_selection.md`
- `docs/specs/document_viewer.md`

## Opening rule

The popup may open from right-click on a valid selection in `Document Viewer`.

If no valid `TextSelectionRef` exists:

- the popup must not open, or
- it must open in blocked state with no transformation flow available

The popup is a governed intent-capture surface.

It is not an editor.
It is not a runtime.
It does not apply changes.

## Popup structure

The popup contains two conceptual fields.

### Selection Preview

The upper field is `Selection Preview`.

It:

- shows selected text
- is readonly
- may allow copy or local text selection if presentation requires it
- must not allow editing

The preview is display state only and must remain tied to the underlying `TextSelectionRef`.

### Transform Instruction

The lower field is `Transform Instruction`.

It:

- is editable
- captures the user's transformation order
- changes only the instruction text
- does not modify the document

Editing the instruction does not change the selected text and does not mutate the target document.

## Buttons by state

### Before generation

- `Generar propuesta`
- `Cancelar`

### After generation

- `Aceptar`
- `Regenerar`
- `Cancelar`

### Prohibited buttons

The popup must not introduce:

- separate `Rechazar` button
- separate `Corregir instruccion` button
- document edit button

## Semantic actions

### Generar propuesta

`Generar propuesta` conceptually creates a `TransformProposal`.

It uses:

- current instruction text
- target `document_ref`
- current `selection_ref`

No document application occurs in F9.

### Regenerar

`Regenerar` uses the current instruction text and creates a new `proposal_id`.

It must preserve lineage through `instruction_id`, target, selection, and trace.

### Cancelar

`Cancelar` discards the current popup flow and clears the active overlay representation.

It does not save.
It does not apply.

### Aceptar

`Aceptar` marks the proposal as accepted.

In F9, acceptance does not apply the proposal.

Any future application must pass through `ActionResolution` and later governed runtime.

## States

Prepared popup state may include:

- `selection_ready`
- `instruction_draft`
- `proposal_requested`
- `proposal_ready`
- `accepted`
- `cancelled`
- `regenerate_requested`
- `error`

Interpretation:

- `selection_ready`: valid selection context is present and popup may begin instruction capture
- `instruction_draft`: user is editing or has edited the instruction field
- `proposal_requested`: proposal generation was requested conceptually
- `proposal_ready`: a governed proposal representation is available
- `accepted`: user accepted the proposal, but no application occurred in F9
- `cancelled`: popup flow was abandoned and overlay should be cleared
- `regenerate_requested`: a new proposal attempt was requested using current instruction text
- `error`: popup flow cannot continue safely

## Failure modes

- `missing_selection`
- `empty_instruction`
- `stale_selection`
- `proposal_generation_unavailable`
- `pending_overlay_conflict`

Interpretation:

- `missing_selection`: no valid `TextSelectionRef` is available
- `empty_instruction`: no meaningful transform instruction was provided
- `stale_selection`: selection no longer matches current document version or hash
- `proposal_generation_unavailable`: proposal creation is not available in current phase or prepared state
- `pending_overlay_conflict`: popup flow conflicts with an already active governed overlay state

## Relationship to transformation_core

This popup captures and advances the flow of:

- `TransformInstruction`
- `TransformProposal`
- `PatchPreviewOverlay`

But it must not redefine those models.

The popup is one consumer surface of the transformation-core vocabulary.

## Relationship to Document Viewer

`Document Viewer` supplies readonly selection context.

The popup may be opened from viewer selection, but the viewer remains readonly and separate from popup instruction capture.

The popup must not turn the viewer into an editor.

## Relationship to text_selection

The popup requires valid governed `TextSelectionRef` state.

If `selection_ref` is missing, invalid, or stale, transformation flow must be blocked.

## Relationship to future runtime

The popup does not validate determinism.
The popup does not apply patches.
The popup does not mutate files.

Those concerns remain future runtime responsibilities.

The popup may conceptually expose an optional `Ver cambios` path to governed diff visualization, but side-by-side diff remains external to popup semantics and is defined in `docs/specs/diff_view.md`.

## Invariants

- `INV-TPOP-001`: `Selection Preview` MUST remain readonly
- `INV-TPOP-002`: instruction editing MUST modify only instruction state, not the document
- `INV-TPOP-003`: `Generar propuesta` MUST create conceptual proposal state only
- `INV-TPOP-004`: `Regenerar` MUST create a new `proposal_id`
- `INV-TPOP-005`: `Cancelar` MUST clear active overlay representation
- `INV-TPOP-006`: `Aceptar` MUST NOT apply changes in F9
- `INV-TPOP-007`: popup flow MUST be blocked when no valid `selection_ref` exists

## Related specs

- `docs/specs/transformation_core.md`
- `docs/specs/text_selection.md`
- `docs/specs/document_viewer.md`
- `docs/specs/document_governed_editing.md`
- `docs/specs/document_patch_runtime.md`
- `docs/specs/diff_view.md`

---

### !REL_FILE!

# transform_timeline

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `TransformTimeline` surface for DocGraph.

The timeline is a structured memory of transformation attempts, proposals, and user decisions.

It enables future inspection without executing, editing, or applying changes.

## Dependencies

This spec depends on:

- `docs/specs/transformation_core.md`
- `docs/specs/transform_popup.md`
- `docs/specs/patch_preview.md`
- `docs/specs/diff_view.md`

## Role

`TransformTimeline` is structured transformation memory.

It is not an editor.
It does not execute.
It does not apply changes.

## TransformationEvent

Minimum conceptual fields:

- `transformation_id`
- `document_ref`
- `selection_ref`
- `instruction_id`
- `proposal_id`
- `attempt_index`
- `status`
- `user_decision`
- `input_hash`
- `output_hash`
- `timestamp`
- `trace_ref`

Field meaning:

- `transformation_id`: stable event lineage identifier
- `document_ref`: governed target document reference
- `selection_ref`: governed scoped selection reference when applicable
- `instruction_id`: originating transform instruction
- `proposal_id`: associated proposal identity
- `attempt_index`: ordered attempt number within lineage
- `status`: current lifecycle state for the event
- `user_decision`: governed user review outcome when present
- `input_hash`: integrity hash of relevant input snapshot
- `output_hash`: integrity hash of relevant proposal or output snapshot
- `timestamp`: event ordering timestamp
- `trace_ref`: governed trace reference for further inspection

## States

Prepared timeline event state may include:

- `draft`
- `proposed`
- `accepted`
- `cancelled`
- `superseded`
- `stale`
- `applied_future`
- `error`

Interpretation:

- `draft`: instruction or attempt exists before proposal is ready
- `proposed`: proposal exists and is awaiting user validation
- `accepted`: user accepted the proposal, but no application is implied in F9
- `cancelled`: flow was cancelled
- `superseded`: a newer proposal replaced the prior attempt
- `stale`: event lineage no longer matches current document or selection integrity safely
- `applied_future`: reserved future state for a later execution phase
- `error`: the attempt or trace state failed safely

## Conceptual UX

The timeline may conceptually provide:

- a list of transformations per document or artifact
- click on an event may highlight the associated overlay or diff
- hover may show conceptual preview
- save as recipe remains future-only

This is inspection UX only.

It does not authorize mutation.

## Rules

- timeline does not substitute `graph/`
- timeline is not `tool_run_manifest`
- timeline does not apply changes
- timeline preserves traceability
- timeline may feed future recipes

Future recipe abstraction is governed separately by `docs/specs/transformation_recipes.md`.

Normative form:

- `INV-TTL-001`: `TransformTimeline` MUST preserve transformation traceability
- `INV-TTL-002`: `TransformTimeline` MUST NOT replace `graph/`
- `INV-TTL-003`: `TransformTimeline` MUST NOT be treated as `tool_run_manifest`
- `INV-TTL-004`: `TransformTimeline` MUST NOT apply changes
- `INV-TTL-005`: timeline memory MAY feed future recipe construction without becoming execution authority

## Relationship to transformation_core

`TransformTimeline` consumes `TransformationEvent` and related lineage from `transformation_core.md`.

It must not redefine proposal, preview, acceptance, cancellation, or supersession semantics independently.

## Relationship to popup, preview, and diff

- popup creates or advances proposal lineage
- preview may provide associated in-place overlay state
- diff may provide associated comparative inspection state
- timeline records attempts and decisions across those surfaces

## Failure modes

- `event_without_target`
- `event_without_proposal`
- `stale_event`
- `missing_trace_ref_future`
- `duplicate_transformation_id`

Interpretation:

- `event_without_target`: event exists without valid document or artifact target reference
- `event_without_proposal`: event lineage lacks required proposal identity for the expected state
- `stale_event`: event no longer matches safe current target integrity
- `missing_trace_ref_future`: trace reference required for future deeper inspection is absent
- `duplicate_transformation_id`: two separate events collide on the same transformation identity

## Invariants

- `INV-TTL-006`: timeline interaction MUST remain inspection-only in F9
- `INV-TTL-007`: clicking or hovering a timeline event MAY expose overlay or diff context, but MUST NOT execute or apply anything
- `INV-TTL-008`: accepted timeline state MUST remain distinct from `applied_future`

## Related specs

- `docs/specs/transformation_core.md`
- `docs/specs/transform_popup.md`
- `docs/specs/patch_preview.md`
- `docs/specs/diff_view.md`
- `docs/specs/document_governed_editing.md`
- `docs/specs/workspace_tabs.md`
- `docs/specs/transformation_recipes.md`

---

### !REL_FILE!

# ui_core

## Purpose

Define UI-facing state and interaction contracts while preserving the inherited rule that UI is representation, not truth.

## Responsibilities

- define view-facing state contracts
- define minimal panel and shell-level presentation models
- consume controller or service-authorized results
- separate representation concerns from runtime and filesystem authority

## Inputs

- controller or service results
- localized text keys
- semantic theme tokens
- manifest-authorized viewer models
- readonly knowledge-listing results from a small technical boundary
- readonly project-document tree results from a small technical boundary
- workspace-tab selection and routing state
- readonly F9.5 pipeline / ontology mock state

## Outputs

- UI-facing state models
- shell scaffold and panel contracts
- workspace-tab, viewer, tree, chat, knowledge, tools, and document-workflow presentation contracts
- pipeline / ontology proposal view contracts
- future readonly `System View` presentation contracts

## Allowed Dependencies

- standard library
- `core_domain`

## Forbidden Responsibilities

- no direct filesystem access
- no direct business logic
- no direct tool execution
- no direct provider invocation
- no direct Slint dependency

## Current Scope

- shell scaffold contracts
- project, tree, tool, viewer, chat, knowledge, workspace-tab, document-profile, and status presentation state
- Lume Help popup presentation state
- Pipeline / Ontology View presentation state for F9.5 semantic proposals
- Lume Help GUI Objects presentation state based on `GUI_OBJECTS_v1`
- separated `Operational Tools` and `LLM Tools` panel state
- prepared LLM capability and interaction-mode state
- no widget implementation
- no domain ownership
- no editing, notebook, semantic inference, semantic approval, or semantic store behavior
- no Lume Help runtime behavior, LLM call, tool call, filesystem mutation, or history persistence

## Active object context

`ui_core` may expose presentation-facing state derived from `ActiveObjectContext`.

Conceptually, UI state may distinguish:

- one selected document or none
- one selected sandbox or none
- one selected knowledge item or none
- one selected artifact or none
- one selected tool output or none
- one focused family or none

This is not pure widget state and not execution authority.

UI may render and route prepared active-object context, but it must not invent fallback targets, grant permissions, or execute actions from that context alone.

## Pipeline / Ontology View

`PipelineOntologyViewState` is a readonly/mock presentation contract for F9.5 AI governance.

It may represent:

- source document identity
- derived chunk identity
- `PromptSpec`
- `SemanticDerivationSpec`
- `SemanticProposal` rows
- human review state
- future RDF/Oxigraph disabled flags
- trace and policy metadata

It must not infer semantics, approve proposals, call LLMs, create embeddings, persist RDF, run SPARQL, execute tools, mutate documents, or reinterpret project manifests.

## Lume Help popup

`LumeHelpPopupState` is a presentation-only contract for the F9.5 Lume Help popup.

It is:

- readonly
- ephemeral
- not a workspace tab
- not project chat
- not a runtime
- not an action approval surface

Slint may present this state. Slint must not derive governance answers, parse specs, call LLMs, execute tools, mutate files, or persist popup history.

`LumeHelpPopupState` may include a readonly GUI Objects section. The canonical source is `resources/help/gui_objects.json`; UI state only presents a minimal view of canonical names and boundaries.

## Chat and document interaction

The UI-facing model now distinguishes:

- existing governed documents referenced from the tree
- external intake and workflow launch from the clip
- structured references and results in chat
- real work happening in workspace tabs and readonly viewer flows
- a common governed workspace workflow even when the entry point differs

This does not imply blob storage inside chat, editing behavior, or notebook growth.

Conceptually, `ui_core` may expose a `ChatInputDraft` plus readonly input-state markers for multiline growth, overflow, microphone availability, recording, and transcription readiness.

These remain presentation contracts only. They must not execute commands, resolve targets implicitly, invoke LLMs, or bind directly to STT providers.

Conceptually, `ui_core` may also expose readonly LLM-tool request state as proposal or intent representation only. It must not execute tools, call providers, or bypass governed tool-surface and action-resolution policies.

## LLM capability presentation

Conceptually, `ui_core` may expose a readonly `LLMCapabilityStateView`.

It presents prepared state only:

- `desired_llm_mode`
- `effective_llm_mode`
- `interaction_mode`
- `llm_available`
- `provider_configured`
- `credential_ref_present`
- `local_engine_available`
- `local_model_available`
- `policy_allows_local`
- `policy_allows_cloud`
- `tool_surface_available`
- `execution_enabled`
- `reason_codes`

Allowed mode values:

- `OFF`
- `LOCAL`
- `CLOUD`

Allowed interaction values:

- `GUIDED`
- `ASSISTED`

UI must not:

- compute capability state
- decide policy
- resolve credentials
- display secrets
- call providers
- load local models
- execute tools
- execute ActionResolution
- mutate files

When `effective_llm_mode=OFF`, UI may present prepared guided menu state, but that menu may emit navigation, explanation, or configuration intent only.

## Workspace tabs

The main content area is a simple workspace tab container.

Minimum conceptual model:

- `WorkspaceTabsState`
- `WorkspaceTabKind`
- `WorkspaceTabRef`
- `WorkspaceTabTargetRef`

Current concrete kinds may include:

- `viewer`
- `sandbox_view`
- `knowledge_detail`
- `system_view`
- `tool_result`
- `home`
- `pipeline_ontology`

This does not imply notebook behavior, editing, docking, or layout orchestration.

The readonly viewer remains one concrete tab kind inside the workspace container.

It verifies resolved content inside the workspace model; it does not become a document owner, editor, or intake surface depending on where the workflow started.

Conceptually, `ui_core` may expose a `DocumentViewerState` as a readonly rendering contract with optional selection state and prepared visual overlays.

That state must not imply editing, persistence, patch application, tool execution, or LLM execution.

Conceptually, prepared overlay rendering may be represented through a readonly `PatchPreviewOverlayState` using semantic highlight tokens rather than hardcoded success/error colors.

Conceptually, readonly comparison may also be represented through a `DiffViewState` that supports inline-by-default review and optional side-by-side comparison on demand.

Conceptually, `ui_core` may expose a `SandboxViewerState` as readonly governed sandbox-context presentation with capability state, descriptive source metadata, and future working-copy placeholders only.

Conceptually, `ui_core` may expose readonly `SystemViewState`, `NothingHappensState`, `BlockedExplanationViewState`, `TraceViewState`, `TraceSummaryViewState`, `EffortViewState`, `EffectiveToolSurfaceSummaryViewState`, `ToolSurfaceGroupViewState`, and `LumeInterpretationViewState` as presentation contracts only, governed by `docs/specs/system_view.md`.

Those states must remain presentation-ready and sanitized:

- no raw payloads
- no secrets
- no private absolute host paths
- no policy inference
- no authority grants
- refs only where governed linking is needed
- visible labels represented through future i18n keys rather than literal strings

Conceptually, `ui_core` may expose one reusable tab per governed target reference.

Opening the same governed target should activate the existing tab state rather than duplicate it.

Mouse interaction over tab headers may be represented as navigation or close intents only. It must not execute tools, mutate files, or decide permissions.

## Future governed editing concepts

The following names are conceptual contracts only. They do not imply implementation in `ui_core` yet.

- `ActiveContextState`: current tab-derived context for document, figure, table, equation, or patch preview work
- `ReferencedPatchRequestState`: structured user request derived from a referenced selection or active inspector context
- `PatchPreviewState`: readonly preview of a validated patch proposal before acceptance
- `StructuredAssetInspectorState`: readonly inspection state for future figure, table, and equation assets

These future states must remain presentation contracts. Patch validation, patch application, filesystem mutation, LLM execution, and tool execution stay outside UI crates.

## Invariants

- `INV-UI-LLM-001`: UI MUST display prepared LLM capability state only.
- `INV-UI-LLM-002`: UI MUST NOT compute effective LLM mode, interaction mode, policy, or reason codes.
- `INV-UI-LLM-003`: UI MUST NOT expose credentials or credential secrets.
- `INV-UI-LLM-004`: UI MUST NOT call providers, load local models, execute tools, execute ActionResolution, or mutate files.
- `INV-UI-LLM-005`: guided menu presentation MUST NOT emit executing intents.
- `INV-UI-LLM-006`: tool visibility in UI MUST NOT imply execution permission.
- `INV-UI-LLM-007`: raw tool catalogs MUST NOT be injected into LLM context by UI state.

---

### !REL_FILE!

# ui_preferences_popups

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed Preferences menu and popup system for DocGraph.

## Menu model

`Preferences` is a top-level menu entry.

It is separate from `Tools`.

It exposes exactly:

- `App preferences`
- `Project settings`
- `Credentials management`

Each entry opens a distinct governed popup surface.

There is no shared Preferences panel.
There are no Preferences tabs.

## Core rule

The Preferences menu emits intent only.

It does not execute.
It does not bypass policy.
It does not mutate runtime.

## App Preferences popup

Purpose:

Global non-sensitive configuration.

Fields may include:

- `ui_language`
- `theme_mode`
- `lume_assistance_level`
- `ui_behavior_flags`

Rules:

- no secrets
- no filesystem paths
- no execution toggles
- no tool activation
- no LLM execution

Persistence:

- user-scoped
- outside project structure

## Project Settings popup

Purpose:

Project-scoped configuration.

Fields may include:

- `project_profile`
- declarative sandbox references
- optional tool defaults
- metadata

Rules:

- no credentials
- no execution
- no binary references
- portable across machines

Persistence:

- inside project structure

## Credentials Management popup

Purpose:

Secure management of credential references.

Fields may include:

- `provider_id`
- `credential_ref`
- `status`

Status remains conceptual, for example:

- `configured`
- `missing`

Rules:

- NEVER store raw secrets in `project/`
- NEVER store raw secrets in `resources/`
- UI MUST NOT display secrets
- credentials MUST NOT be exposed to LLM
- credentials MUST NOT trigger execution
- credentials MUST NOT grant execution authority

Behavior:

- manages references only
- integrates with `docs/specs/credentials_policy.md`

## Popup system rules

Each popup MUST:

- be independent
- capture configuration only
- require explicit confirmation
- not execute any action
- not mutate runtime
- not trigger tools

Popup emits:

- `ConfigurationIntent`

Popup MUST NOT emit:

- `ExecutionIntent`

## Governed popup i18n keys

Each governed Preferences popup must use i18n-managed title and description keys.

Required keys:

- `popup.preferences.app.title`
- `popup.preferences.app.description`
- `popup.preferences.project.title`
- `popup.preferences.project.description`
- `popup.credentials.title`
- `popup.credentials.description`

Popup titles must remain short.

Descriptions must remain informational only.

Descriptions must not imply execution, tool availability, LLM availability, or secret disclosure.

## Domain separation

Strict boundaries:

- `Preferences` -> configuration only
- `Tools` -> capabilities and execution preparation
- `External Dependencies` -> runtime requirements
- `Credentials` -> secure access references

Never:

- `Preferences` triggering tools
- `Credentials` enabling execution
- `Project settings` storing secrets
- `App preferences` altering runtime behavior directly

## Invariants

- `INV-PREF-001`: Preferences MUST NOT contain secrets.
- `INV-PREF-002`: Preferences MUST NOT execute actions.
- `INV-PREF-003`: Preferences MUST NOT bypass `ActionResolution`.
- `INV-PREF-004`: Project settings MUST remain portable.
- `INV-CRED-001`: Credentials MUST NOT be stored in project files.
- `INV-CRED-002`: Credentials MUST NOT be exposed to LLM.
- `INV-CRED-003`: Credentials MUST NOT imply execution authority.
- `INV-UI-POPUP-001`: each Preferences entry MUST open a dedicated popup.
- `INV-UI-POPUP-002`: no shared Preferences panel with tabs is allowed.
- `INV-UI-POPUP-003`: governed Preferences popup titles and descriptions MUST come from i18n resources.

## Non-goals

Do not implement:

- runtime logic
- tool execution
- credential storage engine
- encryption
- API calls
- filesystem mutation
- sandbox interaction
- dependency installation

## Related specs

- `docs/specs/app_main_menu.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/credentials_policy.md`
- `docs/specs/project_settings_popup.md`

---

### !REL_FILE!

# ui_theme_policy

## Purpose

Define the governed semantic theme contract for DocGraph and Lume, including portable typography tokens.

## Core rule

UI appearance must remain resource-driven, token-based, and portable across supported environments.

## Typography profile

DocGraph/Lume uses a Codex-like typography profile based on semantic roles rather than platform-specific font names.

Typography tokens must remain abstract and system-font oriented:

- `typography.ui.family`
- `typography.code.family`
- `typography.chat.family`
- `typography.metadata.family`
- `typography.id.family`
- `typography.technical_compact.family`

Supporting semantic text-size tokens may include:

- `typography.ui.size`
- `typography.chat.size`
- `typography.metadata.size`
- `typography.id.size`
- `typography.technical_compact.size`

## Rules

- UI runtime must consume semantic typography tokens rather than hardcoded font names
- tokens must prefer system fonts through abstract families
- monospace is reserved for:
  - code
  - paths
  - identifiers
  - hashes
  - structured logs
- standard UI copy, narrative chat text, and metadata labels must not default to monospace
- no external font files are embedded by this policy
- no platform-specific font stack is hardcoded by this policy
- typography policy must not change layout logic by itself

## Intended roles

- `ui`: default product UI copy and general readable interface text
- `code`: code blocks and code-like snippets
- `chat`: conversational body text
- `metadata`: secondary labels, captions, timestamps, and contextual annotations
- `id`: ids, refs, hashes, paths, and stable technical identifiers
- `technical_compact`: dense technical summaries, manifests, and structured diagnostics

## Portability

- light and dark themes must expose the same semantic typography token set
- token meaning must remain stable across appearance modes
- typography tokens must preserve theme portability and must not imply host-specific availability

## Invariants

- `INV-THEME-001`: presentation code must consume semantic theme tokens rather than hardcoded colors
- `INV-THEME-002`: presentation code must consume semantic typography roles rather than platform-specific font names
- `INV-THEME-003`: monospace is restricted to code and technical identifier roles
- `INV-THEME-004`: typography tokens must remain portable across supported appearance modes
- `INV-THEME-005`: typography policy does not introduce runtime logic, execution authority, or layout ownership

---

### !REL_FILE!

# user_profile_policy

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define a governed, privacy-safe conceptual `UserProfile` contract for future Lume and LLM-facing flows.

This spec does not create persistence, authentication, telemetry, identity resolution, or real user data handling.

## Core rule

User profile data is optional, minimal, and non-sensitive by default.

Core DocGraph operation must not depend on a populated user profile.

## UserProfile model

Conceptual structure:

- `user_id`
- `profile_version`
- `preferences_ref`
- `consent_state_ref`
- `allowed_context_refs`
- `denied_context_refs`

Field meaning:

- `user_id`: opaque, non-sensitive identifier only
- `profile_version`: declarative schema or contract version
- `preferences_ref`: conceptual reference to governed non-secret preferences
- `consent_state_ref`: conceptual reference to explicit consent state
- `allowed_context_refs`: optional bounded context references explicitly allowed for future governed use
- `denied_context_refs`: optional bounded context references explicitly denied for future governed use

## UserProfile rules

- no personal sensitive data is required
- no implicit data collection is allowed
- user identity remains opaque
- profile presence is optional, not required for operation
- profile references do not imply persistence by this spec
- profile references do not imply authorization
- profile references do not imply execution permission

## Relationship to preferences and consent

`UserProfile` is a lightweight coordination surface only.

It may conceptually point to:

- `UserPreferences`
- `ConsentState`

It must not absorb or duplicate:

- credential material
- execution policy
- tool authorization
- provider configuration

## Relationship to Lume and future LLM interaction

Lume may explain the existence or absence of governed profile information.

Lume may explain:

- whether a profile is present
- whether preferences are available
- whether consent is present
- whether context references are allowed or denied conceptually

Lume must not:

- infer missing profile values
- create hidden profile data
- persist profile data implicitly
- expose denied context references

## Allowed profile exposure

Future governed LLM-facing exposure may use only:

- opaque `user_id`, when required for traceability
- non-sensitive preference summaries
- explicit consent-state summaries
- allowed context-reference summaries

The profile must not expose:

- credentials
- secrets
- denied context references
- raw host-specific filesystem paths
- hidden prompts

## Invariants

- `INV-USER-PROFILE-001`: user profile is optional and non-blocking.
- `INV-USER-PROFILE-002`: no sensitive data is required for normal operation.
- `INV-USER-PROFILE-003`: `user_id` MUST remain opaque and non-sensitive.
- `INV-USER-PROFILE-004`: no implicit user-data collection is allowed.
- `INV-USER-PROFILE-005`: profile references MUST NOT grant execution authority.
- `INV-USER-PROFILE-006`: profile references MUST NOT bypass policy, consent, or ActionResolution.
- `INV-USER-PROFILE-007`: credentials MUST NOT appear in the profile model.
- `INV-USER-PROFILE-008`: this spec defines no runtime persistence in F9/F10_PREP.

## Non-goals

- no runtime storage implementation
- no user database
- no authentication system
- no encryption-layer definition
- no cloud sync
- no telemetry
- no behavioral tracking
- no profile mutation flows
- no UI editing forms
- no real data handling

## Related specs

- `docs/specs/privacy_and_consent_model.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/lume_interaction_model.md`
- `docs/specs/llm_core.md`

---

### !REL_FILE!

# workspace_core

## Purpose

Provide the inherited workspace boundary layer for the Rust successor.

## Responsibilities

- represent portable workspace roots and project-scoped roots
- preserve `system/`, `dev/`, and `user/` separation
- normalize and validate workspace-scoped relative paths
- enforce scope control for project-safe path handling

## Inputs

- portable app root
- requested workspace-relative or project-relative paths
- project identifiers or project root descriptors

## Outputs

- normalized workspace descriptors
- validated project-scoped path descriptors
- scope validation results and boundary errors

## Allowed Dependencies

- standard library
- `core_domain`

## Forbidden Responsibilities

- no UI logic
- no LLM logic
- no spec parsing
- no business workflow orchestration
- no arbitrary filesystem discovery as architecture source of truth

## Initial Phase Scope

- root and scope descriptors
- relative path discipline
- project root validation
- portable workspace boundary rules

---

### !REL_FILE!

# workspace_layout

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Central layout

The central DocGraph layout is composed of:

- `Tree`
- `Chat`
- `Tabs Workspace`

Canonical structure:

`[ Tree ] [ Chat ] [ Tabs Workspace ]`

`Tabs Workspace` may contain:

- `Document Viewer`
- `Tools Panel`
- `Knowledge`
- `Sandbox View`, only when relevant

The viewer belongs to the tabs workspace as a rendered view.

Governed workspace tabs are identified by governed references and must be reused rather than duplicated for the same target.

The tree navigates.
The chat guides.
The tabs workspace hosts views.
The viewer renders content.

## Sandbox tab

Sandbox information MAY be exposed as a workspace tab when relevant.

The `Sandbox View` tab is conditional and must only appear when the project profile or project configuration declares sandbox relevance.

Typical relevant profile:

- `folder_organization_sandbox`

The `Sandbox View` tab is informational and declarative in F9.

It may show:

- sandbox state
- readonly source-folder reference state
- future working-copy state
- dry-run or proposal status if declared
- warnings about readonly source folder
- future actions as non-executable descriptions

It MUST NOT:

- copy files
- move files
- rename files
- delete files
- execute tools
- trigger write-back
- expose host-specific absolute paths as portable truth
- replace `Tools Panel`
- replace `Knowledge`
- become a filesystem runtime

## Invariants

- `INV-WORKSPACE-LAYOUT-001`: central layout MUST remain Tree + Chat + Tabs Workspace.
- `INV-WORKSPACE-LAYOUT-002`: viewer MUST be rendered inside Tabs Workspace.
- `INV-WORKSPACE-LAYOUT-003`: tabs host views; tabs do not execute logic.
- `INV-SANDBOX-VIEW-001`: Sandbox information MAY be exposed as a conditional workspace tab when relevant.
- `INV-SANDBOX-VIEW-002`: Sandbox view MUST remain declarative in F9.
- `INV-SANDBOX-VIEW-003`: Sandbox view MUST NOT execute filesystem operations.
- `INV-SANDBOX-VIEW-004`: Sandbox visibility MUST depend on project profile or governed project configuration.
- `INV-SANDBOX-VIEW-005`: Sandbox view MUST NOT replace Tools Panel, Knowledge, or project runtime.

---

### !REL_FILE!

# workspace_tabs

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Workspace Tabs` model for DocGraph.

Tabs represent governed objects opened in the workspace.

This spec defines tab kinds, tab identity, mouse interaction, relation to `ActiveObjectContext`, and the rule that tabs are representation only.

## Core rule

`Workspace Tabs` is a controlled representation surface.

Tabs do not execute actions.
Tabs do not mutate documents.
Tabs do not decide permissions.
Tabs do not replace runtime authority.

## Relationship to the central layout

The central layout remains:

`[ Tree ] [ Chat ] [ Tabs Workspace ]`

The tree selects governed objects.
The chat captures intention.
The tabs workspace hosts controlled views.
The viewers render context.

## Supported tab kinds

Current or declared kinds may include:

- `Document Viewer Tab`
- `Sandbox Viewer Tab`
- `Knowledge Viewer Tab`, when applicable
- `System View Tab`, future readonly inspection tab governed by `docs/specs/system_view.md`
- `Tool Output Viewer Tab`, future
- `Timeline Tab`, future inspection tab governed by `docs/specs/transform_timeline.md`

Concrete workspace identifiers may still use governed internal kinds such as:

- `viewer`
- `sandbox_view`
- `knowledge_detail`
- `system_view`
- `tool_result`

This document does not define text selection, popup, overlay, diff, or timeline behavior beyond tab-hosting boundaries.

## Tab identity

Tabs are identified by governed references, not by filesystem paths or visible titles.

Examples:

- `viewer::document::doc://f110`
- `viewer::sandbox::sandbox://org_main`
- `viewer::knowledge::knowledge://main/ref_001`
- `viewer::tool_output::tool_output://run_001`

Rule:

- one governed object = one reusable tab
- opening the same governed object activates the existing tab
- the system must not duplicate tabs for the same governed reference

## Conceptual model

Minimum conceptual fields may include:

- `tab_id`
- `tab_kind`
- `target_ref`
- `family`
- `state`
- `is_active`
- `is_closable`
- `is_pinned_future`
- `has_pending_visual_proposal`, when future integration requires it

## Mouse interaction

### Left-click on tab header

- activates the tab
- may update `focused_family` when the family is explicit and observable
- must not duplicate the tab
- must not execute anything

### Middle-click on tab header

- closes the tab when it is closable
- if a pending proposal is associated, close is conceptually blocked or requires confirmation
- does not save
- does not execute

### Right-click on tab header

Context menu may include:

- close tab
- close others
- close all
- copy reference
- pin tab, future optional

The context menu must not:

- execute a tool
- apply a proposal
- mutate a document
- open the filesystem

### Mouse interaction from Document Tree

Left-click from `Document Tree`:

- selects the governed object
- activates or creates the corresponding tab
- updates `ActiveObjectContext`

Middle-click from `Document Tree`:

- opens or activates the corresponding tab without duplication
- does not execute

## Relationship to ActiveObjectContext

- activating a `Document Viewer Tab` may focus family `Document`
- activating a `Sandbox Viewer Tab` may focus family `Sandbox`
- activating a tab does not clear selections of other families
- `focused_family` must be observable in the prepared state
- the system must not use hidden fallback to the last opened tab

Tab activation must not silently manufacture a new target reference when no governed selection or ref exists.

## Tab states

Prepared tab state may include:

- `active`
- `inactive`
- `readonly`
- `missing`
- `stale`
- `invalid`
- `closable`
- `pinned_future`
- `has_pending_visual_proposal`

Interpretation:

- `active`: current visible workspace tab
- `inactive`: open but not currently active
- `readonly`: valid and non-mutable in current phase or policy
- `missing`: target reference no longer resolves
- `stale`: target exists but prepared view state is no longer current enough
- `invalid`: tab kind or target is not valid for governed rendering
- `closable`: tab may be closed
- `pinned_future`: future optional pinned state only
- `has_pending_visual_proposal`: future readonly flag for proposal-related visual integration

## Relationship to viewers

`Document Viewer Tab` renders governed document context.

Its internal rendered surface is defined separately in `docs/specs/document_viewer.md`.

`Sandbox Viewer Tab` renders governed sandbox context.

Its internal rendered surface is defined separately in `docs/specs/sandbox_viewer.md`.

`Knowledge Viewer Tab` may render governed knowledge context when applicable.

Tabs host these views, but they do not replace the viewers and do not own filesystem or runtime authority.

## Relationship to Document Tree

The tree prepares selection and may request opening or activation of a governed tab.

Tabs do not replace tree navigation, and the tree does not create duplicate tabs for the same governed reference.

## Failure modes

- `duplicate_tab_ref`
- `stale_tab_ref`
- `missing_tab_target`
- `close_blocked_pending_proposal`
- `invalid_tab_kind`

Interpretation:

- `duplicate_tab_ref`: a second tab was attempted for the same governed reference
- `stale_tab_ref`: the tab target resolves, but view state is no longer current enough
- `missing_tab_target`: the governed reference no longer resolves
- `close_blocked_pending_proposal`: a close action was attempted while governed pending visual proposal rules block it
- `invalid_tab_kind`: tab state references an unsupported kind

## Invariants

- `INV-WTAB-001`: no duplicate tab may exist for the same governed reference
- `INV-WTAB-002`: tab identity MUST be based on governed reference, not path or title
- `INV-WTAB-003`: tabs are representation only
- `INV-WTAB-004`: tabs MUST NOT execute actions
- `INV-WTAB-005`: tabs MUST NOT mutate documents or filesystem state
- `INV-WTAB-006`: tabs MUST NOT decide permissions
- `INV-WTAB-007`: activating a tab MUST NOT erase other-family selections
- `INV-WTAB-008`: tabs host views; tabs do not execute logic

## Out of scope

- text selection
- popup behavior
- patch overlay
- diff rendering
- timeline behavior
- editing
- notebook behavior
- docking
- multi-window
- semantic orchestration
- patch application

## Related specs

- `docs/specs/workspace_layout.md`
- `docs/specs/document_tree.md`
- `docs/specs/active_object_context.md`
- `docs/specs/gui_objects_v1.md`
- `docs/specs/ui_core.md`
- `docs/specs/system_view.md`

---

