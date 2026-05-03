# Roadmap — DocGraph

This roadmap outlines the controlled evolution of DocGraph from a governed engineering implementation to a fully operational document workflow platform.

The development follows incremental, auditable phases to ensure architectural consistency and avoid uncontrolled expansion.

---

## Phase 1 — Governed Runtime Foundation (COMPLETED)

Core system architecture and execution model established.

Delivered:

- governed project pipeline (`project_runtime`)
- declarative specification loading (`spec_runtime`)
- strict workspace boundaries (`workspace_core`)
- separation between runtime, UI, and tools
- deterministic execution outputs
- audit and validation tooling

Outcome:

A stable, reproducible, and controlled system foundation.

---

## Phase 2 — Application Layer and UI Structure (COMPLETED)

User-facing structure without breaking governance.

Delivered:

- thin application layer (`app_services`)
- CLI entrypoint (`cli_app`)
- UI structural shell (`ui_core`, `ui_slint`)
- document viewer (read-only)
- knowledge panel
- document tree (manifest-based)
- tools panel (governed surface)

Outcome:

A usable system interface aligned with the governed runtime.

---

## Phase 3 — Controlled File Intake and Exposure (COMPLETED)

Safe ingestion and visibility of user documents.

Delivered:

- governed file intake (`io_runtime`)
- strict intake boundaries (sanitization, ownership, traceability)
- explicit exposure gate (`project_runtime`)
- manifest-only visibility (`project_manifest.json`)
- document tree derived exclusively from manifest
- audit guards preventing bypass

Outcome:

Documents become controlled, traceable artifacts within the system.

---

## Phase 4 — Viewer and Interaction Integration (IN PROGRESS)

Reconnect user interaction layers without breaking governance.

Planned:

- viewer reconnection to manifest-based artifacts
- navigation across exposed documents
- improved UI interaction flows
- stabilization of document access patterns

Constraints:

- no filesystem authority from UI
- no bypass of exposure gate
- no implicit execution

---

## Phase 5 — Tool Execution Expansion (PLANNED)

Enable controlled document processing capabilities.

Planned:

- expansion of `tool_runtime` execution coverage
- integration of operational tools into workflows
- improved tool observability and outputs
- enforcement of execution boundaries

Constraints:

- tools remain governed and explicit
- no hidden pipelines
- no implicit chaining

---

## Phase 6 — LLM-Assisted Workflows (PLANNED)

Introduce LLM capabilities under strict governance.

Planned:

- integration of LLM-assisted tools
- controlled prompt execution
- bounded interaction through `Lume`
- explicit tool invocation from LLM context

Constraints:

- no direct LLM authority over filesystem or runtime
- no uncontrolled generation pipelines
- all actions remain auditable

---

## Phase 7 — Semantic Document Layer (PLANNED)

Add structured knowledge capabilities.

Planned:

- embeddings and semantic indexing
- RDF-based document relationships
- SPARQL querying capabilities
- ontology proposal and refinement tools

Constraints:

- semantic layer remains optional and governed
- no implicit knowledge mutation
- no hidden graph state

---

## Phase 8 — Production Hardening (FUTURE)

Prepare system for real-world deployment.

Planned:

- stabilization of APIs and interfaces
- improved UI/UX for end users
- packaging and distribution model
- performance optimization
- security review and hardening

Outcome:

Transition from engineering workspace to production-grade platform.

---

## Development Principles

Across all phases, DocGraph enforces:

- reproducibility over convenience  
- explicit execution over implicit behavior  
- governance over uncontrolled flexibility  
- separation of concerns across all layers  

---

## Strategy

DocGraph does not aim to open all subsystems simultaneously.

Each phase is:

- explicitly defined  
- locally validated  
- audited before expansion  

This ensures long-term maintainability and avoids architectural drift.
