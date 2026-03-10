# DocGraph — Portable Reproducible Documentation Platform

**Portable Reproducible Documentation and Workflow Platform**

DocGraph is a **portable platform for structured document processing, reproducible workflows, and AI-assisted knowledge work**.

The system is designed to run **entirely from a portable directory**, allowing deployment in restricted environments without complex installation procedures.

DocGraph emphasizes:

* portability
* reproducibility
* structured governance
* auditable execution
* extensible workflows

The platform provides a controlled environment for **document engineering, workflow automation, and knowledge analysis**.

---

# Project Status

Status: Early development
Version: 0.1.0

DocGraph is currently under active architectural development. The current repository represents the initial platform baseline and supporting governance documentation.


---

# Core Principles

DocGraph is built around several fundamental principles.

### Portability

The platform can be distributed as a **self-contained directory**.

No system-level installation is required.

All components remain inside the project workspace.

---

### Reproducibility

Every execution generates a **traceable run record**.

Outputs and execution logs are stored deterministically:

```
user/runs/<tool_id>/<timestamp>/
user/output/<tool_id>/<timestamp>/
```

This guarantees that every result can be reproduced and audited.

---

### Declarative Architecture

The system follows a **spec-driven architecture**.

Configuration and system behavior are declared through structured specifications rather than implicit runtime logic.

This improves:

* auditability
* maintainability
* system introspection
* AI-assisted architecture analysis

---

### Structured Governance

The platform architecture is governed by formal documents stored in:

```
system/governance/
```

Key governance documents include:

| Document               | Role                        |
| ---------------------- | --------------------------- |
| Operational Definition | Platform constitution       |
| Invariants Guide       | Architectural rules         |
| Dev Runbook            | Operational procedures      |
| Platform Roadmap       | Strategic evolution         |
| UI Layout              | User interface architecture |

This structure ensures that the system evolves **without architectural drift**.

---

# Platform Architecture

DocGraph follows a strict separation between **core system**, **development environment**, and **user workspace**.

```
DocGraph/
│
├─ system/
│   Core platform implementation
│
├─ user/
│   User workspace and generated data
│
├─ dev/
│   Development and maintenance tools
```

---

## system/

Contains the **core platform runtime**.

Examples:

* tool implementations
* runtime configuration
* specifications
* UI architecture
* governance documents

This directory **must never be modified at runtime**.

---

## user/

Contains **all user-generated data**.

Examples:

* workflow runs
* artifacts
* outputs
* input documents
* project data

Typical structure:

```
user/
├─ input/
├─ output/
├─ runs/
└─ projects/
```

---

## dev/

Contains **development utilities** used to maintain the platform.

Examples:

* diagnostics
* registry rebuild tools
* architecture validation scripts
* developer utilities

The `dev/` directory may be removed in production distributions.

---

# Workflow-Centric Project Model

DocGraph organizes work around **projects and workflows**.

Each project acts as an isolated knowledge workspace.

```
Project
 ├ Workflow
 ├ Notebooks
 ├ Sources
 ├ Prompts
 ├ Data
 ├ Artifacts
 ├ Outputs
 └ Runs
```

This structure allows:

* structured document generation
* repeatable workflows
* reusable process definitions
* traceable project evolution

Projects remain **fully isolated** to guarantee reproducibility.

---

# Process Catalog

DocGraph supports reusable **process definitions**.

Processes are defined in a centralized **process catalog** and can be reused across multiple workflows.

Examples of processes:

* document ingestion
* regulatory analysis
* document comparison
* structured drafting
* artifact generation

Complex workflows can be constructed by chaining multiple processes.

---

# User Interaction Model

The platform provides a **workspace-oriented user interface**.

Core interaction elements include:

* project selector
* project tree
* notebook viewer
* contextual input bar
* execution log panel

Primary interaction views include:

* **Chat views** for conversational interaction
* **Notebook views** for structured document workflows

This model supports both:

* exploratory work
* structured workflow execution

---

# Current Capabilities

Current implemented tools include:

* CSV → PDF report generation
* PDF merging utilities
* system healthcheck tools
* structural architecture validators

These tools demonstrate the platform’s **reproducible workflow execution model**.

---

# Planned Capabilities

The platform roadmap includes several major capabilities.

### Semantic Knowledge Layer

* embeddings
* semantic search
* structured document retrieval

---

### Knowledge Graph Integration

Integration with RDF and graph technologies for structured knowledge representation.

---

### Local LLM Runtime

Support for locally hosted language models enabling:

* document drafting
* workflow assistance
* structured analysis

---

### Causal Analysis Engine

Integration with causal inference frameworks such as **DoWhy**.

---

### Autonomous Knowledge Workflows

Future versions will support:

* AI-assisted workflow execution
* structured reasoning pipelines
* automated document drafting workflows

---

# Why DocGraph

DocGraph aims to combine several capabilities rarely found together:

* portable deployment
* reproducible computational workflows
* structured governance
* AI-assisted knowledge processing
* extensible architecture

The result is a platform designed for **engineering-grade knowledge work**.

---


# Research Context

DocGraph is developed as part of ongoing work on portable software architectures, reproducible tooling environments, and modular document processing systems.

---

# License

This project is released under the MIT License.

See the `LICENSE` file in this repository for details.


---

# Contributing

See `CONTRIBUTING.md` for contribution guidelines.

---

# Author

J. Cillero

---