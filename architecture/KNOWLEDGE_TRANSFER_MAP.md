# Knowledge Transfer Map

This map records where Python knowledge has been transferred inside the Rust sandbox.

It records knowledge transfer from Python as historical and doctrinal source material only.
It does not declare a runtime dependency on Python and does not require Python to operate the Rust sandbox.

When a requested Python source name is not physically present in this sandbox, the active local equivalent is used and the transfer type reflects that adaptation.

| Python Source Document | Category | Knowledge Transferred | Rust Target File | Transfer Type |
| --- | --- | --- | --- | --- |
| `OPERATIONAL_DEFINITION_Rev09.txt` | platform constitution | portability, declarative-first architecture, project as primary working unit, workflow-centric system model | `README.md`, `governance/GOVERNANCE.md`, `architecture/ARCHITECTURE.md`, `governance/FUNCTIONAL_SCOPE.md`, `architecture/MIGRATION_BASELINE.md`, `docs/specs/project_runtime.md` | inherited as-is |
| `INVARIANTS_GUIDE.md` | invariants and auditability | structural separation, reproducibility, audit-ready discipline, controlled runtime boundaries | `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md`, `governance/INHERITED_GOVERNANCE.md`, `docs/specs/workspace_core.md`, `docs/specs/ui_core.md` | inherited as-is |
| `GOBERNANCE_INDEX.json` | knowledge hierarchy | active normative precedence and source discovery | `governance/GOVERNANCE.md`, `governance/INHERITED_GOVERNANCE.md`, `architecture/KNOWLEDGE_TRANSFER_MAP.md` | adapted |
| `DECLARED_VS_PRESENT_ARTIFACTS_POLICY.md` | governance policy | declaration takes precedence over sandbox-local physical absence | `governance/GOVERNANCE.md`, `architecture/MIGRATION_BASELINE.md`, `governance/INHERITED_GOVERNANCE.md` | inherited as-is |
| `DEV-DOMAIN-ARCHITECTURE.md` | development domain governance | dev as engineering laboratory, not distributed runtime | `README.md`, `governance/GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | inherited as-is |
| `GUIA - UTILITIES.txt` | implementation discipline | reuse-first utility doctrine and separation between generic utilities and domain logic | `architecture/ARCHITECTURE.md`, `governance/WORKSPACE_RULES.md` | adapted |
| `utils_catalog_oficial.json` | implementation discipline | technical utility coverage and controlled cross-cutting concerns | `architecture/ARCHITECTURE.md`, `architecture/MODULE_MAPPING.md` | adapted |
| `WORKSPACE_STRUCTURE_EXPLAINED.md` | workspace doctrine | normative vs runtime vs tooling boundaries and mutable vs immutable areas | `README.md`, `governance/GOVERNANCE.md`, `docs/specs/workspace_core.md` | adapted |
| `project_manifest_exposure_contract.json` | project contract | manifest-driven visibility, ref model, no direct filesystem-driven exposure | `architecture/ARCHITECTURE.md`, `architecture/MODULE_MAPPING.md`, `docs/specs/project_runtime.md`, `docs/specs/ui_core.md` | inherited as-is |
| `system/utils/project/*` | live project implementation knowledge | validation -> ref resolution -> neutral surface model flow | `architecture/ARCHITECTURE.md`, `architecture/MODULE_MAPPING.md`, `docs/specs/project_runtime.md` | adapted |
| requested `PROJECT_MANIFEST_SPEC_v2.txt` not present locally; active equivalent: `project_manifest_exposure_contract.json` plus `system/utils/project/*` | project manifest governance | ref-driven manifest governance carried into Rust using active equivalents | `governance/INHERITED_GOVERNANCE.md`, `architecture/KNOWLEDGE_TRANSFER_MAP.md`, `docs/specs/project_runtime.md` | adapted |
| requested `PROJECT_FILESYSTEM_SPEC_v2.txt` not present locally; active equivalent: `WORKSPACE_STRUCTURE_EXPLAINED.md` plus current project runtime model | project/workspace layout | project isolation, workspace boundaries, and artifact/output separation | `architecture/MIGRATION_BASELINE.md`, `architecture/MODULE_MAPPING.md`, `docs/specs/workspace_core.md`, `docs/specs/project_runtime.md` | adapted |
| `CONTRATO_UI_CORE.json` | UI/controller/runtime contract | UI as representation, controller boundary, traceability, no direct filesystem access | `architecture/ARCHITECTURE.md`, `governance/WORKSPACE_RULES.md`, `governance/UI_SLINT_POLICY.md`, `docs/specs/ui_core.md` | inherited as-is |
| `UI_LAYOUT_Rev08.md` | UI shell doctrine | tree/chat/viewer/log roles, controller-first flow, viewer passivity, manifest-governed visibility | `architecture/ARCHITECTURE.md`, `governance/UI_SLINT_POLICY.md`, `docs/specs/ui_core.md` | adapted |
| requested `UI_WORKSPACE_SPEC_MASTER_v4.txt` not present locally; active equivalent: `CONTRATO_UI_CORE.json`, `UI_LAYOUT_Rev08.md`, `ui_strings.json`, and `ui_theme.json` | UI workspace governance | presentation-only UI with externalized text and governed theme resources | `governance/I18N_POLICY.md`, `governance/UI_THEME_POLICY.md`, `governance/UI_SLINT_POLICY.md`, `docs/specs/ui_core.md` | adapted |
| `ui_strings.json` and `ui_strings_contract.json` | UI localization | user-visible UI text externalization and bilingual resource governance | `governance/I18N_POLICY.md` | inherited as-is |
| `ui_theme.json` and `ui_theme_contract.json` | UI appearance | light/dark appearance modes and theme resources under governed tokens | `governance/UI_THEME_POLICY.md` | adapted |
| `LLM_CONFIGURATION_GUIDE.md` | LLM configuration doctrine | config layers, deterministic precedence, runtime mode inputs | `governance/LLM_RUNTIME_POLICY.md`, `docs/specs/llm_core.md` | inherited as-is |
| `governance/LLM_RUNTIME_POLICY.md` | LLM runtime doctrine | local/cloud/auto, deterministic execution plan, policy before invocation | `governance/LLM_RUNTIME_POLICY.md`, `docs/specs/llm_core.md` | inherited as-is |
| `LLM_MODEL_CATALOG_GUIDE.md` | model governance | catalog as control layer, approved models only, deterministic validation | `governance/LLM_RUNTIME_POLICY.md`, `docs/specs/llm_core.md` | inherited as-is |
| `LLM_TOOL_INTEGRATION_GUIDE.md` | tool governance | tools requested by the model but executed by the system | `governance/LLM_RUNTIME_POLICY.md`, `docs/specs/llm_core.md` | inherited as-is |
| `LLM_OBSERVABILITY_GUIDE.md` | observability doctrine | structured metadata required to reconstruct runtime decision and outcome | `governance/LLM_RUNTIME_POLICY.md`, `docs/specs/llm_core.md`, `governance/INHERITED_GOVERNANCE.md` | adapted |
| requested `AUDIT_ENGINE_SPEC_Rev00.txt` not present locally; active equivalent: `INVARIANTS_GUIDE.md` plus declared-vs-present policy | audit doctrine | audit-ready structural discipline transferred, concrete Rust audit engine deferred | `governance/INHERITED_GOVERNANCE.md`, `governance/WORKSPACE_RULES.md` | deferred |
| requested `KNOWLEDGE_INDEX_MASTER.md` not present locally; active equivalent: `GOBERNANCE_INDEX.json` | knowledge hierarchy | document precedence and active-source discovery | `governance/GOVERNANCE.md`, `architecture/KNOWLEDGE_TRANSFER_MAP.md` | adapted |
