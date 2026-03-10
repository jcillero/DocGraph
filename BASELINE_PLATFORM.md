# DocGraph — Platform Baseline

Baseline Version: **0.1.0**
Consolidation Date: **24 February 2026**
Status: **Stable — Ready for cloning**
Type: **Lightweight portable platform base**

---

# 1. Platform Identity

Name: **DocGraph**
Author: **J. Cillero**
Version: **0.1.0**

Primary execution mode: **CLI (interactive shell)**
Architecture model: **Declarative runtime governed by `runtime_registry.json`**

This document defines the **reference baseline of the DocGraph portable platform** and establishes the initial **reproducible system state** from which future platform versions will evolve.

---

# 2. Platform Size

Approximate disk footprint: **41 MB**
Files: **1,668**
Directories: **235**

Reference comparison:

Previous development runtime (full WinPython distribution): approximately **3.23 GB**

Current runtime model: **slim portable runtime**

Estimated reduction: **~98.7 %**

This reduction was achieved by removing non-essential packages and retaining only the dependencies required by the productive toolchain.

---

# 3. Runtime Environment

Runtime location:

```
system/bin/runtimes/python/current/Scripts/python.exe
```

Runtime version:

```
Python 3.12.10 (AMD64)
```

The runtime is treated as a **controlled internal component of the platform**.

Public source repositories may omit large runtime binaries in order to maintain lightweight distribution sizes. Portable runtime bundles may optionally include the runtime for fully standalone execution.

---

# 4. Frozen Dependencies

Dependency lock file:

```
system/bin/runtimes/python/current/requirements.lock.txt
```

Dependencies included in this baseline:

```
charset-normalizer==3.4.4
pillow==12.1.1
pypdf==6.7.3
reportlab==4.4.10
```

Design principle:

**Minimal runtime dependency set required by productive tools.**

Dependency updates must be explicitly documented in future baseline revisions.

---

# 5. Productive Tools

The baseline includes the following operational tools:

| Tool                | Status      |
| ------------------- | ----------- |
| CSV → Report PDF    | Operational |
| PDF Merge           | Operational |
| Runtime Healthcheck | Operational |

These tools represent the **initial productive functionality of the platform**.

---

# 6. Development Tooling

The following development utilities are included:

| Tool                          | Status      |
| ----------------------------- | ----------- |
| dev_collect_app_bundle        | Operational |
| dev_generate_structure_report | Operational |

These utilities assist in **inspection, packaging, and structural validation** of the platform.

---

# 7. Runtime Integrity Validation

A runtime healthcheck was executed successfully.

Result:

```
status = OK
runtime = current/Scripts/python.exe
exit_code = 0
```

Output location:

```
user/runs/healthcheck/<timestamp>/healthcheck.json
```

This validation confirms that the **runtime environment and registry configuration operate correctly**.

---

# 8. Guaranteed Architectural Invariants

The following invariants are guaranteed by this baseline:

* No hardcoded version-dependent paths
* Runtime declared through configuration registry
* Strict separation between `system`, `user`, and `dev`
* Outputs generated exclusively inside `user/` or `dev/`
* Frozen runtime dependencies
* Reproducible healthcheck execution
* Registry free of orphan references
* No dependency on full external Python distributions

These invariants ensure **platform portability, reproducibility, and structural integrity**.

---

# 9. Confirmed Base Structure

```
DocGraph/
├── system/
│   ├── bin/
│   │   ├── entrypoints/
│   │   ├── tools/
│   │   └── runtimes/python/current/
│   ├── config/
│   └── assets/
├── dev/
├── user/
└── START_APP.bat
```

The directory structure separates:

* runtime configuration
* development tooling
* user data

This separation is fundamental to maintaining **platform portability and governance integrity**.

---

# 10. Architectural Status

The current baseline is considered:

* fully portable
* minimal runtime footprint
* declarative in configuration
* structurally audited
* reproducible
* suitable as a foundation for future extensions

The architecture has been designed to support future integration of:

* graphical interfaces
* plugin ecosystems
* optional local AI runtimes

---

# 11. Evolution Rules

Future modifications must follow these rules:

1. New dependencies require updates to `requirements.lock.txt`.
2. New tools must be declared in the runtime registry.
3. Functional changes require specification updates when behavior changes.
4. Runtime bloat must be avoided; only minimal dependencies should be included.

These rules preserve **long-term maintainability and reproducibility** of the platform.

---

# 12. Baseline Declaration

This document defines the **official platform baseline for DocGraph**.

Future versions of the platform may extend this baseline while preserving the architectural invariants described above.

---
