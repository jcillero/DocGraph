# Module Mapping

This mapping is responsibility-driven.

It does not justify superficial folder copying from Python into Rust.

| Python responsibility | Live Python source cues | Rust target | Transfer note |
| --- | --- | --- | --- |
| workspace separation, scope control, path discipline | `OPERATIONAL_DEFINITION_Rev09`, `INVARIANTS_GUIDE`, `WORKSPACE_STRUCTURE_EXPLAINED`, `system/utils/workspace` | `workspace_core` | inherit portable workspace boundaries and project scoping |
| generic filesystem, text, and data utilities | `GUIA - UTILITIES`, `utils_catalog_oficial.json`, `system/utils/data`, `system/utils/text`, `system/utils/fs` | `io_runtime` and `workspace_core` | adapt utility discipline, not Python helper names |
| declarative specs, contracts, registries, and config loading | `GOBERNANCE_INDEX.json`, `system/spec/*`, `system/config/*` | `spec_runtime` | inherit declarative-first loading and validation |
| project as primary working unit, manifest/ref model, project-safe operations | `project_manifest_exposure_contract.json`, `system/utils/project`, `system/bin/tools/runtime/project` | `project_runtime` | inherit manifest validation -> ref resolution -> neutral surface model flow |
| LLM runtime policy, modes, model catalog, and observability | `system/app/llm/docs/*` | `llm_core` | inherit deterministic policy and contracts |
| local LLM adapter behavior | local mode doctrine from `governance/LLM_RUNTIME_POLICY.md` and `LLM_CONFIGURATION_GUIDE.md` | `llm_local` | adapters behind contracts only |
| cloud LLM adapter behavior | cloud mode doctrine from `governance/LLM_RUNTIME_POLICY.md` and `LLM_CONFIGURATION_GUIDE.md` | `llm_cloud` | adapters behind contracts only |
| tool execution as system concern | `LLM_TOOL_INTEGRATION_GUIDE.md`, `llm_tools_contract.json` | `tool_runtime` | tools requested by model, executed by system |
| UI state and controller-facing representation contracts | `CONTRATO_UI_CORE.json`, `UI_LAYOUT_Rev08.md` | `ui_core` | representation only, not source of truth |
| user-visible UI text externalization | `ui_strings.json`, `ui_strings_contract.json` | `ui_i18n` | no hardcoded UI strings in views |
| semantic appearance resources | `ui_theme.json`, `ui_theme_contract.json` | `ui_theme` | token-driven light/dark theming |
| Slint presentation integration | UI layout and controller boundary doctrine | `ui_slint` | presentation edge only |
| application orchestration above runtime and below presentation | UI/controller/runtime interaction model and LLM tool policy | `app_services` | compose services without widget coupling |
| future executable surface | current launcher and entrypoint intent | `cli_app` | thin executable only at this stage |
