# Security Model — DocGraph

DocGraph is designed around controlled execution, explicit data promotion, and traceable document workflows.

The security model is based on architectural boundaries rather than implicit trust between components.

---

## Core Security Principles

DocGraph follows these principles:

- no implicit execution
- no hidden data pipelines
- no filesystem authority from UI layers
- no project exposure by filesystem presence
- explicit ownership and traceability for operations
- deterministic and auditable runtime behavior

---

## Controlled Execution

Execution in DocGraph is never assumed from user interface state, file presence, or document visibility.

Runtime behavior must pass through governed boundaries.

Key rules:

- UI layers may present state but must not execute domain logic
- tool execution must be explicit and governed
- LLM-related components must not execute tools or mutate files directly
- runtime expansion must be introduced through explicit phases

---

## File Intake and Exposure

Documents do not become project artifacts simply because they exist on disk.

DocGraph separates:

```text
file selected by user
→ governed intake
→ imported_not_exposed
→ explicit exposure
→ project_manifest visibility
````

The authoritative project exposure source is:

```text
project_manifest.json
```

Filesystem presence, file-store presence, registry entries, graph entries, or UI visibility do not imply project membership.

---

## Manifest-Driven Authority

Project visibility is governed by manifest entries.

The document tree must derive visible project documents from manifest exposure entries only.

Forbidden authority sources:

* passive filesystem scanning
* copied files inside project folders
* registry-derived visibility
* graph-derived visibility
* UI-side inference
* chat attachment presence

---

## UI Boundary

The UI is a presentation and interaction layer.

It may:

* display prepared state
* collect user intent
* show validation results
* present safe errors
* open read-only views

It must not:

* mutate project files
* write manifests
* execute tools
* derive documents
* scan the filesystem as authority
* promote files into project scope
* reinterpret governance policy

---

## Tool Boundary

Tools are governed runtime capabilities.

Tool execution must remain explicit, traceable, and owner-scoped.

Rules:

* tools must not run implicitly
* tools must not write to project-root outputs
* persisted tool outputs require ownership metadata
* tool runs must produce traceable manifests
* broad tool orchestration must remain gated

---

## LLM Boundary

Lume and LLM-related crates are not execution authorities.

LLM-assisted workflows must remain bounded by governance rules.

Rules:

* Lume may guide, explain, or assist
* Lume must not execute tools
* Lume must not mutate files
* Lume must not approve actions
* LLM outputs must not become runtime decisions without explicit governed handling

---

## Traceability

DocGraph uses explicit references to preserve accountability.

Important trace fields include:

* `owner_ref`
* `trace_ref`
* tool run manifests
* exposure request references
* exposure candidate references
* human confirmation references

These references allow operations to be inspected and audited after execution.

---

## Data Sanitization

DocGraph avoids turning host-specific or sensitive data into portable project truth.

Rules:

* raw absolute host paths must not become portable references
* metadata must be sanitized before persistence
* unsafe filenames or labels must be rejected or normalized
* secrets must not be stored in manifests, traces, logs, or UI state

---

## Audit Guards

DocGraph uses validation and audit scripts to detect boundary drift.

Examples of guarded areas include:

* file intake boundaries
* manifest exposure boundaries
* tool execution boundaries
* AI/LLM execution boundaries
* Python runtime exclusion
* storage and file-store contracts

Audits are part of the development process and help prevent architectural drift.

---

## Current Limitations

This security model describes architectural and runtime-boundary controls.

It does not claim:

* formal security certification
* cryptographic verification of all artifacts
* sandboxing against hostile operating systems
* protection against malicious local administrators
* hardened multi-user access control

DocGraph is currently focused on controlled local execution, traceability, and reproducible document workflows.

---

## Future Hardening

Future security work may include:

* stronger manifest validation
* signed or hashed artifact chains
* stricter secret scanning
* role-based confirmation policies
* controlled export/write-back policies
* package integrity checks
* external dependency verification

---

## Summary

DocGraph security is based on a simple rule:

```text
nothing becomes authoritative by accident
```

Files, tools, LLM outputs, UI state, registry entries, and graph projections must pass through explicit governed boundaries before they can affect the project state.

