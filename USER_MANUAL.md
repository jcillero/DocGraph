# DocGraph — User Manual

Version: 0.1.0
Status: Early development
Type: Portable document tool platform
Execution mode: CLI / UI launcher

---

# 1. What is DocGraph?

DocGraph is a portable document tooling platform designed for structured document processing, reproducible execution workflows, and controlled runtime environments.

The system allows users to run technical tools from a structured environment while maintaining full traceability of operations.

Main capabilities include:

* merging PDF files with interactive ordering
* generating structured PDF reports from CSV files
* executing tools from a controlled console environment
* maintaining execution traceability
* organizing document workflows in a reproducible structure

DocGraph is designed with portability, traceability, and architectural clarity as primary goals.

---

# 2. Distribution Model

DocGraph is distributed in two complementary forms.

## Lightweight Repository (GitHub)

The public repository contains the lightweight version of the platform, including:

* source code
* configuration files
* architecture specifications
* governance documents
* development tools
* project structure

Heavy runtime binaries are intentionally excluded from the repository.

This keeps the repository:

* lightweight
* easy to clone
* suitable for development and inspection

Typical use cases for the repository version include:

* development
* architecture exploration
* contribution
* lightweight installations

---

## Portable Runtime Bundle

Portable runtime bundles may be distributed separately.

A portable bundle can include:

* portable Python runtime
* required binary dependencies
* optional runtime components
* preconfigured execution environment

The portable bundle allows the application to run without requiring installation on the host system.

---

# 3. Running the Application

DocGraph is executed from the project root using the launcher.

## Recommended Method

Double-click:

```
START_APP.bat
```

This launches the interactive tool console.

---

## From PowerShell

Open a terminal in the project root and run:

```
.\START_APP.bat shell
```

---

## Execute a Specific Tool

```
.\START_APP.bat run <tool_id>
```

Example:

```
.\START_APP.bat run tool_csv_to_report_pdf
```

---

# 4. Workspace Structure

DocGraph uses a structured workspace layout.

```
system/   runtime configuration and core tools
user/     user data, artifacts, and execution runs
dev/      development utilities
```

This separation ensures:

* system stability
* clear traceability
* safe upgrades
* reproducibility

---

# 5. Working Directories

## user/input/

Place input files here.

Typical examples include:

* CSV files used to generate reports
* PDF files to merge

---

## user/output/

Generated artifacts are stored here.

Structure:

```
user/output/<tool_id>/<timestamp>/
```

Examples of artifacts include:

* generated reports
* merged documents
* tool outputs

---

## user/runs/

This directory stores execution traceability.

Each run includes:

* execution timestamp
* tool identifier (`tool_id`)
* metadata
* logs
* references to generated artifacts

Example:

```
user/runs/<tool_id>/<timestamp>/
```

These directories allow the system to maintain a reproducible execution history.

Run folders should not be removed unless performing maintenance operations.

---

# 6. Included Tools

## CSV → Report PDF

Generates structured PDF reports from CSV files.

Typical features include:

* summary cover page
* one page per record
* consistent document pagination

---

## PDF Merge

Allows users to:

* select PDF files from `user/input/`
* reorder them interactively
* generate a merged output document

---

## Structural Report (Developer Tool)

Generates a diagnostic report describing the current structure and configuration state of the application workspace.

This tool is primarily used for development inspection and platform diagnostics.

---

# 7. Requirements

## Lightweight Repository

Running the repository version requires an available runtime environment.

Typical requirements for the repository version include:

* Windows operating system
* compatible Python runtime
* optional dependencies depending on enabled tools

The repository itself does not include heavy runtime binaries.

---

## Portable Runtime Bundle

Portable runtime bundles include all required runtimes and dependencies.

The application can run directly from the project folder without installation.

---

# 8. Optional Runtime Components

DocGraph supports optional runtime components that extend the capabilities of the platform.

Examples include:

* local LLM runtimes
* embedding models
* vector search engines
* knowledge graph engines
* causal inference engines (DoWhy)

These components are optional extensions and are not required for the core functionality of the platform.

---

# 9. Good Practices

To ensure correct operation:

* do not modify files inside `system/`
* do not run scripts directly
* always launch the application using `START_APP.bat`
* keep filenames simple
* avoid unnecessary special characters in filenames

---

# 10. Portability

DocGraph is designed to be fully portable.

The application folder can be copied to:

* external drives
* internal networks
* other Windows machines

Portable runtime bundles allow execution without installing additional software.

---

# 11. Troubleshooting

If the application fails to start:

1. inspect the console error message
2. verify that input files are located in `user/input/`
3. ensure that the runtime environment is available
4. restart the application from the project root

Execution traces can also be inspected in:

```
user/runs/
```

These traces provide useful diagnostic information.

---

# 12. Extensibility

DocGraph is built on a tool-based architecture.

New capabilities can be introduced by adding new tools without modifying the core system structure.

Tools are registered through the runtime registry.

This architecture allows the platform to evolve while maintaining stability.

---

# 13. Architecture Overview

The platform is governed by a set of architectural documents located in:

```
system/governance/
```

These documents define:

* operational rules
* architectural invariants
* development workflow
* platform evolution roadmap

The separation between `system`, `user`, and `dev` domains guarantees a clear boundary between runtime configuration, user data, and development utilities.

---

# 14. Future Capabilities

Planned capabilities may include:

* local embeddings
* vector search
* knowledge graph integration
* local LLM runtime
* causal analysis engine (DoWhy)

These capabilities will be introduced progressively while preserving the architectural invariants of the system.

---
