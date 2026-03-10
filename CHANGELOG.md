# Changelog

All notable changes to the **DocGraph** platform will be documented in this file.

The format of this changelog follows principles inspired by *Keep a Changelog* and semantic versioning practices adapted to early-stage research software.

---

## [0.1.0] — 2026-02-24

### Added

Initial public baseline of the DocGraph portable platform.

Core architectural elements established:

* Portable workspace architecture (`system / user / dev`)
* Declarative runtime configuration model
* Tool execution console via `START_APP.bat`
* Structured execution trace directories
* Deterministic artifact generation workflow

### Tools

Initial productive tools implemented:

* CSV → PDF structured report generator
* PDF merge utility with interactive ordering
* runtime healthcheck tool
* structural inspection utilities for development diagnostics

### Platform Architecture

* Controlled runtime environment using portable Python runtime
* Tool execution governed through configuration registry
* Separation between runtime configuration, user data, and development utilities
* Execution traceability recorded under:

```
user/runs/<tool_id>/<timestamp>/
```

Generated artifacts recorded under:

```
user/output/<tool_id>/<timestamp>/
```

### Documentation

Initial documentation set introduced:

* `README.md`
* `USER_MANUAL.md`
* `BASELINE_PLATFORM.md`

### Governance

Introduction of architectural governance documents under:

```
system/governance/
```

Including:

* operational definition
* invariants guide
* development runbook
* platform roadmap

### Design Principles

The platform baseline establishes the following architectural principles:

* portability
* reproducibility
* traceability
* declarative configuration
* modular extensibility

### Notes

Version **0.1.0** represents the initial architectural baseline of the DocGraph platform.

The project is currently in an early development phase focused on consolidating platform architecture and tool infrastructure.

Future versions will extend the platform with additional tooling capabilities and optional runtime components while preserving architectural invariants.

---

