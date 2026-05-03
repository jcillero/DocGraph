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
