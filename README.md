# DocGraph — Portable Reproducible Document Platform

DocGraph is a portable, reproducible, and auditable document workflow platform built in Rust.

It is designed for engineering and regulated environments where document processing must be deterministic, traceable, and executable without external dependencies.

DocGraph replaces fragile, environment-dependent document pipelines with a governed and inspectable system.

DocGraph provides a deterministic execution model where results are reproducible and independently verifiable.

---

## Real-world scenario

In regulated engineering environments (e.g. naval systems, defense documentation, or certified industrial processes), document workflows must be:

- reproducible
- auditable
- traceable across versions

DocGraph provides a deterministic pipeline to guarantee that:

same input → same output → same trace

even years later, on a different machine, without external dependencies.

---

## What DocGraph provides

- portable execution from a local directory (no installation required)
- reproducible document workflows with deterministic outputs
- controlled processing environment with explicit boundaries
- full traceability of all operations
- stable and inspectable output structure

---

## Why it matters

Most document workflows today are:

- opaque
- non-reproducible
- dependent on complex environments
- difficult to audit or validate

DocGraph provides a controlled alternative:

- deterministic execution
- full traceability
- portable runtime
- governed processing model

---

## NGI Alignment

DocGraph aligns with Next Generation Internet (NGI) objectives by contributing to:

- **Digital sovereignty**: fully local, offline-capable execution without external dependencies  
- **Trustworthy systems**: deterministic pipelines with verifiable outputs  
- **Open infrastructure**: transparent, inspectable, and extensible architecture  
- **Reproducible workflows**: identical results across environments and time  

DocGraph can be positioned as a foundational layer for:

- verifiable document and knowledge pipelines  
- auditable engineering and regulatory processes  
- future trustworthy AI-assisted workflows  

The system is designed to evolve toward a **verifiable knowledge execution platform**, bridging structured workflows, reproducibility, and controlled AI integration.

DocGraph contributes toward a future internet where computation and knowledge processing can be independently verified, not just executed.

---

## Example use cases

- Engineering documentation pipelines with full traceability  
- Regulatory and compliance workflows  
- Offline document processing in restricted environments  
- Controlled transformation of technical documents  
- Future: governed AI-assisted document analysis

---

# DocGraph Rust Implementation

This repository contains the governed Rust implementation of DocGraph, a portable platform for reproducible and traceable document workflows.

It is structured as a multi-crate workspace that advances the system through controlled, auditable phases, ensuring architectural consistency and long-term maintainability.

The current implementation is fully Rust-based and does not depend on any external runtime or prior system.

## Product identity

- application name: `DocGraph`
- assistant name: `Lume`

DocGraph is implemented as a governed Rust workspace, designed to provide structured and traceable document workflows under strict architectural control.

It is not yet positioned as a final production release, but already enforces clear execution boundaries and controlled system behavior.

Lume is a contextual interaction and help layer. It does not execute filesystem operations, tools, LLM workflows, or pipeline actions, and serves strictly as a guidance interface within the governed environment.

## Current state

The workspace evolves through controlled, versioned phases.

Core architectural phases are completed and the system provides:

- a governed project runtime with a central pipeline
- structured application layers (CLI, UI shell, app services)
- controlled IO and tool boundaries
- a read-only document viewer and knowledge panel
- a governed tools surface and UI structure

Detailed phase tracking (3A–F11) is maintained for engineering traceability.

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

## System model

DocGraph is built around a governed pipeline:

project_root → manifest → contract → validation → surface → resolve → output

This pipeline is owned by `project_runtime` and consumed by all higher layers.

## Main crates

The workspace is organized into focused crates with clear responsibilities:

### Core runtime

- `project_runtime`: governed project vertical and central pipeline  
- `spec_runtime`: declarative loading for specifications, configuration, and registries  
- `workspace_core`: portable workspace boundaries and path discipline  

### Application layer

- `app_services`: thin consumer layer over governed runtime components  
- `cli_app`: minimal local entrypoint for interacting with the system  

### Execution and IO

- `tool_runtime`: declarative tool catalog and controlled execution boundary  
- `io_runtime`: controlled IO boundaries and persisted resource access  

### UI

- `ui_core`, `ui_slint`: structural UI shell, controllers, viewer, and knowledge presentation  

LLM-related crates are present in the workspace but remain intentionally limited in scope and are not yet integrated into the main product workflow.

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

## Planned extensions

The following capabilities are planned as future extensions:

- full end-user UI behavior and editing surfaces
- extended tool execution coverage
- integrated LLM-assisted workflows
- semantic runtime capabilities (embeddings, RDF, SPARQL)

These will be introduced progressively while preserving the system’s governance model.

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

This repository represents an actively evolving implementation of DocGraph.

It provides a governed, Rust-based architecture for reproducible and traceable document workflows, with core runtime structure and validation mechanisms already in place.

The current focus of the project is to:

- consolidate a fully Rust-based implementation
- turn declarative specifications into executable runtime contracts
- validate architectural direction through controlled, auditable iterations

The system is not yet positioned as a final production release and does not currently include:

- stable external API guarantees
- full end-user UI behavior
- complete tool execution coverage
- integrated LLM workflows
- semantic runtime features such as embeddings, RDF persistence, or SPARQL execution

These capabilities are planned as future extensions and will be introduced progressively while preserving the system’s governance and architectural invariants.

## Key documents

- `architecture/ARCHITECTURE.md`
- `governance/GOVERNANCE.md`
- `governance/FUNCTIONAL_SCOPE.md`
- `architecture/MIGRATION_BASELINE.md`
- `governance/WORKSPACE_RULES.md`
- `user/output/rust_status_snapshot.md`
- `docs/ENGINEERING_NOTES.md`

---

## Author

Jesús J. Cillero  
PhD (in progress) in Applied Mathematics  
Naval & Ocean Engineering background  

Focus areas:
- reproducible systems  
- structured knowledge  
- engineering workflows  
