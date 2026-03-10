# INVARIANTS GUIDE — Rev05 Premium

Structural Invariants and Governance Rules for the Portable Workflow Platform

---

# 1. Purpose

This document defines the **structural invariants** governing the platform.

An invariant is a rule that **must always hold** for the system to remain:

* structurally coherent
* portable
* auditable
* extensible without compromising the core architecture

These invariants are intended to be **verifiable through automated structural audits**.

They focus on **architecture, configuration governance, and filesystem integrity**, rather than runtime behavior or functional logic.

---

# 2. Architectural Philosophy

The platform follows a set of architectural principles designed to ensure long-term stability and controlled evolution:

1. Core minimalism
2. Declarative configuration
3. Portable runtime environment
4. Strict separation between system and user layers
5. Plugin-based extensibility
6. Isolation of user data and artifacts
7. Reproducible execution runs
8. Auditable filesystem organization
9. Workflow-centric project architecture

The purpose of these principles is to enable continuous evolution while **preventing architectural drift and structural degradation**.

---

# 3. Severity Levels

Invariant validation results are classified using the following severity levels:

| Level       | Meaning                                              |
| ----------- | ---------------------------------------------------- |
| OK          | Invariant satisfied                                  |
| WARN        | Suspicious condition that may indicate future issues |
| FAIL        | Structural violation that must be corrected          |
| NOT_CHECKED | Invariant not currently evaluated by the auditor     |

These levels allow automated tools to distinguish **acceptable states, potential risks, and critical violations**.

---

# 4. Invariant Categories

The invariant system is organized into thematic categories to simplify auditing and governance.

| Category       | Purpose                                            |
| -------------- | -------------------------------------------------- |
| Structure      | Filesystem architecture and workspace layout       |
| Runtime        | Python runtime integrity and discovery             |
| Registry       | Consistency of system configuration registries     |
| Metadata       | Validation of `SCRIPT_META` declarations           |
| Traceability   | Reproducibility and traceability of execution runs |
| Governance     | Protection of core system integrity                |
| Bootstrap      | Runtime acquisition and initialization             |
| Extension      | Plugin and snippet management                      |
| UI             | User interface shell architecture                  |
| Workflow       | Workflow definition and execution integrity        |
| Project        | Structural integrity of projects                   |
| ProcessCatalog | Governance of reusable process definitions         |
| DataLayer      | Persistence architecture and data isolation        |

This categorization allows the auditing system to **evaluate each architectural layer independently while preserving global system coherence**.


# 5. Invariant Catalogue

---

# 5.1 Structure

### INV-001 — system/dev/user separation

Workspace must contain:

```
system/
dev/
user/
```

Purpose:

* isolate runtime system
* isolate development environment
* isolate user data

FAIL if any layer is missing.

---

### INV-002 — declarative system/spec

Files inside:

```
system/spec/
```

must contain **declarative specifications only**.

Executable code is forbidden.

Purpose:

* enforce spec-driven architecture
* maintain machine-readable system definitions

---

# 5.2 Runtime

### INV-003 — dev tools not exposed in runtime entrypoints

Files under:

```
dev/
```

must never appear in:

```
system/config/entrypoints.json
```

Purpose:

prevent development scripts from becoming runtime entrypoints.

---

### INV-004 — portability (no hardcoded paths)

Runtime scripts must not contain:

* absolute filesystem paths
* drive-specific paths
* machine-specific directories

Example violations:

```
C:\Users\
/home/user/
D:\projects\
```

Purpose:

guarantee workspace portability.

---

### INV-005 — bytecode hygiene

Workspace must not contain:

```
__pycache__
*.pyc
```

Purpose:

ensure portable source-only distribution.

---

### INV-006 — UI must not write runs/output

Files under:

```
system/app/ui
```

must not write directly to:

```
user/runs
user/output
```

Purpose:

UI must delegate execution to the **Runner layer**.

---

### INV-007 — tool reproducibility

Tools should produce deterministic results unless explicitly declared otherwise.

Indicators of non-determinism include:

```
random
numpy.random
time-dependent outputs
```

Purpose:

support reproducible workflows.

---

# 5.3 Registry

### INV-008 — runtime tools registered

Every runtime tool must appear in:

```
system/config/runtime_registry.json
```

Purpose:

prevent untracked executables.

---

### INV-009 — registry scripts exist

Every script declared in the runtime registry must exist in the filesystem.

Purpose:

prevent registry drift.

---

### INV-010 — registry category coherence

Tool category declared in the registry must match tool location.

Example:

```
runtime → system/bin/tools/runtime
product → system/bin/tools/product
```

Purpose:

avoid misclassification.

---

### INV-011 — registry drift detection

Mismatch between registry and filesystem must be detected.

Violations include:

* undeclared scripts
* registry entries pointing to missing files

Purpose:

maintain a reliable execution catalog.

---

# 5.4 Metadata

### INV-012 — SCRIPT_META presence

Every tool script must contain a valid:

```
SCRIPT_META
```

block describing:

* script name
* version
* type
* inputs
* outputs

Purpose:

self-describing tools.

---

Tu bloque ya es bueno, pero puede **ganar mucha claridad arquitectónica y auditabilidad** si se perfilan tres cosas:

* lenguaje normativo consistente
* reglas de validación explícitas
* propósito más claro (auditable)

Sin añadir nuevos invariantes, te dejo la **versión perfilada premium**.

---

# 5.4 Script Metadata

### INV-013 — Complete `SCRIPT_META` contract

Every executable script must contain a valid **`SCRIPT_META` block**.

The metadata block must include the following fields:

```
script_name
version
type
category
description
location_expected
entry_point
inputs
outputs
```

Validation rules:

* the `SCRIPT_META` block must appear at the beginning of the script
* the internal structure must be valid JSON
* `script_name` must match the filename
* `location_expected` must match the script location

Purpose:

* ensure tool metadata completeness
* enable automated script discovery
* support structural auditing of executable modules

---

# 5.5 Traceability

### INV-014 — Run traceability

The workspace must contain the directory:

```
user/runs/
```

Every execution triggered by the runtime system must generate a **run record**.

Run records must include:

* execution timestamp
* executed tool identifier
* execution status
* generated artifacts (if any)

Purpose:

* guarantee operational traceability
* ensure reproducibility of system executions

---

### INV-015 — Strict output layout

Artifacts generated by tools must follow the canonical layout:

```
user/output/<tool_id>/<timestamp>/
```

Rules:

* files must never be written directly under `user/output/`
* tool outputs must remain grouped by tool identifier and execution time

Purpose:

* guarantee deterministic artifact storage
* simplify artifact discovery and auditing

---

### INV-016 — Artifact linked to run

Every generated artifact must correspond to a recorded **run**.

Validation rule:

* each artifact must reference a valid run identifier
* artifacts without a corresponding run record are invalid

Purpose:

* guarantee complete provenance of generated artifacts
* ensure traceability between execution and output

---

# 5.6 Governance

### INV-017 — System immutability at runtime

Runtime tools must never modify the directory:

```
system/
```

Rules:

* runtime tools may only write to `user/`
* modifications to `system/` are allowed only during development or installation phases

Purpose:

* prevent architectural corruption
* preserve system integrity in portable deployments

---

### INV-018 — Orphan specifications

Specifications located in:

```
system/spec/
```

that are not referenced by runtime components must be **flagged during structural audit**.

Purpose:

* detect obsolete or unused specifications
* prevent documentation drift between specification and implementation

---

# 5.7 Bootstrap

### INV-019 — Runtime bootstrap declaration

The runtime bootstrap configuration file must exist:

```
system/config/runtime_bootstrap.json
```

This file declares the acquisition and initialization of required runtimes.

Purpose:

* guarantee deterministic runtime initialization
* allow portable deployment without external configuration

---

### INV-020 — Runtime not implemented as plugin

The **primary runtime environment** must not be implemented as a plugin.

Runtime environments represent **core infrastructure components** and must remain part of the core system.

Purpose:

* ensure runtime availability before plugin loading
* prevent circular dependencies during system startup

---

### INV-021 — Runtime location

The primary runtime environment must reside in:

```
system/bin/runtimes/python/current/
```

Rules:

* the path must be resolvable during system startup
* the runtime must be discoverable through bootstrap configuration

Purpose:

* guarantee deterministic runtime discovery
* support portable execution environments

---

# 5.8 Extensions (Plugins & Snippets)

### INV-022 — Plugin registry existence

The file:

```
system/config/plugin_registry.json
```

must exist if plugins are present in the system.

Purpose:

* provide a canonical registry of installed plugins
* enable plugin discovery and validation

---

### INV-023 — Plugin manifest presence

Each plugin must contain the file:

```
plugin_manifest.json
```

The manifest defines:

* plugin identity
* plugin capabilities
* plugin entry points (if applicable)

Purpose:

* ensure plugins are self-describing
* enable automated validation of plugin packages

---

### INV-024 — Plugin registry coherence

The plugin registry must remain coherent with the filesystem.

Rules:

* every declared plugin must exist in the plugin directory
* every plugin directory must have a corresponding registry entry

Purpose:

* prevent registry drift
* guarantee consistent plugin discovery

---

### INV-025 — System plugin immutability

Plugins located under:

```
system/plugins/
```

must not be modified at runtime.

Purpose:

* guarantee stability of core extensions
* prevent uncontrolled system mutation

---

### INV-026 — User plugin isolation

Plugins located under:

```
user/plugins/
```

must not modify:

```
system/
```

Rules:

* user plugins operate only on user scope resources
* system directories must remain protected

Purpose:

* preserve core system integrity
* maintain safe extensibility

---

# 5.9 UI Architecture

### INV-027 — Single UI root

All UI source code must reside under:

```
system/app/ui/
```

Purpose:

* maintain a clear architectural boundary for the UI layer
* simplify UI discovery and maintenance

---

### INV-028 — Central workspace host

The UI must provide a workspace composed of the following elements:

* project tree
* workspace host
* content viewer
* global input bar

These components form the **central interaction model** of the system.

Purpose:

* ensure consistent workspace navigation
* maintain a coherent UI architecture

---

### INV-029 — Preferences are core

System preferences must belong to the **core system configuration**.

Plugins must not control or override core system preferences.

Purpose:

* preserve stability of system configuration
* prevent plugin interference with core behavior

---

### INV-030 — UI text externalization

User-visible text strings must reside in:

```
system/spec/ui/ui_strings.json
```

Rules:

* UI code must not hardcode user-visible strings
* text resources must be loaded from the externalized specification

Purpose:

* enable internationalization
* centralize UI text management

---


# 5.10 Workflow Invariants

### INV-031 — Project workflow existence

Every project **must define exactly one primary workflow**.

The workflow represents the operational process governing the project.

Constraints:

* A project without a workflow is **invalid**.
* Workflow definition must exist inside the project structure.

Purpose:

* guarantee deterministic execution model
* enforce workflow-centric architecture

---

### INV-032 — Workflow DAG constraint

Workflow precedence relations must form a **Directed Acyclic Graph (DAG)**.

Rules:

* cycles are forbidden
* self-dependencies are forbidden
* precedence relations must resolve to a valid topological order

Purpose:

* guarantee deterministic execution order
* prevent infinite execution loops

---

### INV-033 — Process instance reference

Every process instance in a workflow must reference a **valid process type**.

Reference source:

```
system/process_catalog/registry/
```

Validation rules:

* process_type_ref must exist in the process catalog registry
* referenced process definition must exist

Purpose:

* ensure workflows use only registered process types
* prevent undefined operations

---

### INV-034 — Process ID uniqueness

Process identifiers must be **unique within a workflow**.

Rules:

* process_id must not repeat
* references in precedence relations must resolve to an existing process

Purpose:

* guarantee graph integrity
* ensure traceable execution

---

### INV-035 — Composite workflow integrity

Composite processes must reference **valid subworkflows**.

Rules:

* referenced subworkflow must exist
* nested workflows must also satisfy DAG constraints
* recursive references are forbidden

Purpose:

* allow modular workflow reuse
* maintain workflow integrity

---

# 5.11 Project Invariants

### INV-036 — Project minimal structure

Each project must contain the following conceptual components:

```
Workflow
Notebooks
Sources
Prompts
Artifacts
Outputs
Runs
```

These components represent the **minimum operational structure** of a project.

Purpose:

* guarantee reproducibility
* maintain structural consistency across projects

---

### INV-037 — Artifact / output separation

Intermediate workflow artifacts must **never be stored in the outputs directory**.

Definitions:

Artifacts → intermediate results
Outputs → final deliverables

Example separation:

```
Artifacts → intermediate analysis
Outputs → final report
```

Purpose:

* prevent contamination of final deliverables
* preserve workflow traceability

---

### INV-038 — Project isolation

Projects must remain **operationally isolated**.

Rules:

* project resources must not directly access other project resources
* cross-project dependencies must be mediated through explicit imports or tools

Purpose:

* preserve project portability
* prevent hidden dependencies

---

# 5.12 Process Catalog Invariants

### INV-039 — Process catalog registry existence

The process catalog registry must exist at:

```
system/process_catalog/registry/
```

The registry is the **authoritative index of available process types**.

Purpose:

* guarantee discoverability of processes
* enable workflow validation

---

### INV-040 — Process definition integrity

Each process definition must match a registry entry.

Rules:

* registry entry must reference an existing definition file
* definitions must declare process metadata

Purpose:

* maintain registry consistency
* prevent orphan process definitions

---

### INV-041 — Composite process resolution

Composite processes must resolve **all referenced subworkflows**.

Rules:

* referenced subworkflows must exist
* unresolved references invalidate the composite process

Purpose:

* guarantee composite workflow execution
* prevent runtime failures

---

# 5.13 Data Layer Invariants

### INV-042 — Project data isolation

Project-specific databases must remain **within the project scope**.

Example locations:

```
user/projects/<project_id>/data/
```

Purpose:

* ensure portability
* avoid cross-project contamination

---

### INV-043 — Persistence layer separation

Analytical tools must **not act as persistence layers**.

Example:

```
DoWhy
```

must be used only for **analysis**, not as a database.

Persistence layers must be limited to dedicated storage systems such as:

```
SQLite
Oxigraph
```

Purpose:

* maintain architectural separation
* prevent misuse of analytical frameworks as storage systems

---

