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
