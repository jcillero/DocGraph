# AI_SYSTEM_REPORT_v1

## 1. METADATA
- Generated at: 2026-03-09T20:52:18+00:00
- App root: .
- App version (si existe): unknown
- OS: Windows 11 (10.0.26200)
- Python runtime: D:/MAQUETAS SOFTWARE/APP_DocGraph/App_DocGraph/system/bin/runtimes/python/current/python/python.exe
- Approx bundle size (MB): 247.38
- Workspace fingerprint: 3b1b8ddb92b39f871c9383a3

## 2. ROOT STRUCTURE OVERVIEW

### Root folders detected
- dev
- system
- user

### Missing expected folders
- (none)

### Unexpected root folders
- (none)

### Folder tree (filtered)
```text
App_DocGraph/
  dev/
    ai/
      inputs/
        AI_SYSTEM_REPORT_v1.md
    config/
      dev_tools_registry.json
    docs/
      archive/
        Propuesta arbol UI mínimo Rev01.txt
        UI-ARCH-001_ui_dialogs_structure.md
    plugins/
      PLUGIN_FORMAT_SPEC.md
    runs/
dev/runs/ [no-walk] (dirs=14, files=36, size=0.97 MB)
    scripts/
      ai/
        generate_ai_system_report.py
      diagnostics/
        validate_logs.py
      registry/
        rebuild_registries.py
        tool_rebuild_user_plugins_registry.py
      reports/
        collect_app_bundle.py
        generate_structure_report.py
      ui/
        validate_ui_strings.py
      utils/
      build_icon_ico.py
      dev_tool_launcher.py
    test/
      test_product_telemetry.py
    tools/
      run_validate_logs.bat
      run_validate_ui_strings.bat
    run_dev_tool.bat
  system/
    app/
      core/
        credentials/
        logging/
        plugins/
        preferences/
        runner/
        runtime/
        workspace/
      plugins/
        __init__.py
        plugin_loader.py
        plugin_registry.py
      ui/
        app/
        controllers/
        dialogs/
        fields/
        forms/
        i18n/
        menu/
        panels/
        theme/
        views/
        widgets/
        __init__.py
        UI-FORM-BUILDER-SPEC.md
        UI_DECLARATIVE_FORM_SPEC.md
    assets/
      images/
        logos/
        reports/
        ui/
      templates/
    bin/
      entrypoints/
        app_cli.py
        app_ui.py
        registry_validate.py
      runtimes/
system/bin/runtimes/ [no-walk] (dirs=367, files=4653, size=206.91 MB)
      tools/
        _lib/
        product/
        runtime/
    bootstrap/
    config/
      i18n/
      logging/
        logging_event_types_v1.json
        work_units_catalog_v1.json
      plugin_format/
        PLUGIN_FORMAT_SPEC.md.txt
      entrypoints.json
      plugin_registry.json
      runtime_bootstrap.json
      runtime_registry.json
    governance/
      DEV_RUNBOOK_Rev03.txt
      GOVERNANCE_INDEX.json
      INVARIANTS_GUIDE.md
      OPERATIONAL_DEFINITION_Rev08.txt
      PLATFORM_ROADMAP_Rev03.txt
      UI_DESIGN_USER_GUIDE.md
      UI_LAYOUT_Rev06.md
    plugins/
      plugin_designer_core/
    snippets/
      configs/
      prompts/
      templates/
    spec/
      docs/
        GUI_FRAMEWORK_INTEGRATION_GUIDE.md
        WORKSPACE_STRUCTURE_EXPLAINED.md
      logging/
        logging_spec_v1.md
      telemetry/
        product_report_schema.json
        product_telemetry_spec.json
      ui/
        dialogs/
        dialogs_registry_v1.json
        ui_catalog.json
        ui_strings.json
        ui_strings_contract.json
        ui_theme.json
        ui_theme_contract.json
      CONTRATO_UI_CORE.json
      DATA_MODEL_MIN.json
      plugin_manifest_schema.json
      preferences_catalog.json
      runtime_registry_contract.json
    tools/
      telemetry/
        product_reporter.py
        product_telemetry.py
        product_telemetry_wiring.py
  user/
    data/
    input/
      Capítulo 4_ Ecosistema técnico del MDPS.pdf
      cobroDocumentoPago.pdf
      RE-PD-OBS-GESTION DE OBSOLESCENCIAS_R06.5 simple.csv
    output/
    plugins/
      personal_snippets/
      __init__.py
      user_plugins_registry.json
    projects/
    registry/
      documents/
      credentials.json
      preferences.json
    runs/
user/runs/ [no-walk] (dirs=143, files=653, size=36.56 MB)
  .gitignore
  BASELINE_PLATFORM.md
  CHANGELOG.md
  CITATION.cff
  CONTRIBUTING.md
  LICENSE
  README.md
  START_APP.bat
  START_APP_UI.bat
  USER_MANUAL.md
```

### File counts (global workspace)
- Total .py: 1554
- Total .json: 236
- Total .md: 30
- Total other: 3667

## 3. CONFIGURATION STATE

### runtime_registry.json
- Path: system/config/runtime_registry.json
- Total tools: 18
- Categories:
- runtime: 11
- dev: 4
- product: 3
- Tools without presets:
- (none)
- Tools without runtime_ref:
- (none)
- Duplicate tool ids:
- (none)
- Registry integrity status: OK

### entrypoints.json
- Path: system/config/entrypoints.json
- Declared entrypoints: ['cli_entry', 'ui_entry', 'default_mode']
- Missing referenced scripts:
- (none)
- Orphan scripts detected: not_checked

### llm_provider.json (si existe)
- Path: system/config/llm_provider.json
- LLM declared: not_present
- Mode: unknown
- Capabilities declared: not_declared
- Risk note: None

## 4. GOVERNANCE DISCOVERY
- Discovery mode: indexed
- Governance index found: yes
- Governance index valid: yes
- Governance index path: system/governance/governance_index.json
- Governance index error: None
- Schema version: 1.0
- Base path: system/governance
- Priority order:
- operational_definition
- invariants_guide
- dev_runbook
- ui_layout
- platform_roadmap
- ui_design_user_guide

## 4.5 DEFINICIÓN OPERATIVA
- Found: yes
- Path: system/governance/OPERATIONAL_DEFINITION_Rev08.txt
- Version: unknown
- Estado: unknown
- Search checked files: 0
- Source: governance_index


## 5. SCRIPT_META ANALYSIS
- Total scripts scanned: 26
- Scripts missing SCRIPT_META: 2
- Scripts missing SCRIPT_META (paths):
- dev/scripts/registry/rebuild_registries.py
- dev/scripts/registry/tool_rebuild_user_plugins_registry.py
- Duplicate script_name values:
- (none)
- Scripts with modifies_system=true:
- (none)
- Scripts outside expected location:
- dev/scripts/diagnostics/validate_logs.py (expected: system/devtools/logging)
- Scripts without declared outputs:
- system/bin/entrypoints/app_cli.py
- system/bin/entrypoints/registry_validate.py
- system/bin/tools/runtime/tool_get_credentials.py
- system/bin/tools/runtime/tool_get_preferences.py
- system/bin/tools/runtime/tool_reset_preferences.py
- system/bin/tools/runtime/tool_set_credentials.py
- system/bin/tools/runtime/tool_set_preferences.py
- system/bin/tools/runtime/tool_ui_credentials_dialog.py
- system/bin/tools/runtime/tool_ui_preferences_dialog.py
- system/bin/tools/runtime/tool_validate_preferences.py
- Parse errors: 0
- Scripts with SCRIPT_META parse errors (paths):
- (none)
- (Optional) script_meta_index path: (not_written)

## 6. HEALTHCHECK SUMMARY
- Status: WARN
- Warnings:
- (none)
- Critical findings: 0
- Last run detected: user/runs/healthcheck/20260225_215215
- Runtime path verified: {'runtime_ref': 'python', 'console_executable': 'system/bin/runtimes/python/current/python/python.exe', 'console_exists': True, 'sys_executable': 'D:/MAQUETAS SOFTWARE/APP_Tool_ICOFER/App_Tool/system/bin/runtimes/python/current/python/python.exe', 'python_version': '3.12.10 (tags/v3.12.10:0cc8128, Apr  8 2025, 12:21:36) [MSC v.1943 64 bit (AMD64)]', 'arch': '64bit', 'pip_version': 'pip 26.0.1 from D:\\MAQUETAS SOFTWARE\\APP_Tool_ICOFER\\App_Tool\\system\\bin\\runtimes\\python\\current\\python\\Lib\\site-packages\\pip (python 3.12)'}

## 6.5 PYTHON RUNTIME FOOTPRINT
- Present: True
- Path: system/bin/runtimes/python/current
- Total files: 4653
- Total size (MB): 206.91
- Counts:
- .py: 1476
- .pyc: 0
- .pyd: 32
- .dll: 14
- .exe: 210
- Top subdirs by size:
- python: 206.91 MB
- (root): 0.0 MB

## 7. STRUCTURAL AUDIT

### Runtime portability
- Hardcoded paths detected: no
- Hardcoded examples:
- (none)

### Runtime import hygiene
- Illegal imports detected: no
- Illegal import examples:
- (none)

### Registry coverage
- Orphan tool scripts detected: no
- Orphan tool scripts:
- (none)

- Missing tool scripts detected: no
- Missing tool scripts:
- (none)

- Tool scripts outside system/bin/tools detected: no
- Tool scripts outside system/bin/tools:
- (none)

### Bytecode hygiene
- __pycache__ presence: no
- .pyc files present: no

### Dev signals (non-blocking)
- Hardcoded paths detected in dev/: no
- Hardcoded examples in dev/:
- (none)

### Structural invariants (not yet implemented)
- system/ modified in runtime: yes
- Spec violations detected: yes
- Deviations from Base Tree: not_checked

### Summary
- Structural issues count: 7

## 8. INVARIANTS CONFORMANCE (light checks)
- **INV-001 separation system/dev/user**: OK — folders present
- **INV-002 system/spec declarative**: OK — no executable/script-like files found
- **INV-003 dev not in runtime entrypoints**: OK — no dev/ references in entrypoints
- **INV-004 runtime import hygiene**: OK — no illegal imports or sys.path hacks detected in system/
- **INV-005 portability (no hardcoded paths)**: OK — no hardcoded path patterns detected
- **INV-006 traceability runs/**: OK — user/runs exists
- **INV-007 registry coverage complete**: OK — no orphan scripts and no missing declared scripts
- **INV-008 runtime scripts under system/bin/tools**: OK — all runtime/product tools resolve under system/bin/tools
- **INV-009 SCRIPT_META coverage**: WARN — scripts missing SCRIPT_META (2)
- **INV-010 runtime outputs declared**: WARN — scripts without declared outputs (10)
- **INV-011 bytecode hygiene**: OK — no __pycache__ folders or .pyc files present
- **INV-012 system immutable in runtime**: FAIL — writes detected in system/ (1)
- **INV-013 strict output layout**: OK — output layout follows tool_id/timestamp convention
- **INV-014 every artifact linked to a run**: OK — all detected artifacts have acceptable run linkage
- **INV-016 UI never writes runs/output directly**: OK — no UI writes to runs/output detected
- **INV-022 complete SCRIPT_META contract**: WARN — incomplete SCRIPT_META contracts (23)
- **INV-023 registry drift**: FAIL — registry↔script drift detected (3)
- **INV-024 orphan specs**: WARN — tools without supporting spec match (3)
- **INV-025 tool reproducibility**: WARN — stochastic markers without explicit declaration (1)

## 9. RUNTIME ACTIVITY SNAPSHOT
- Recent run directories detected:
- user/runs/dev_reports
- user/runs/tool_healthcheck_runtime
- user/runs/tool_run_stats
- user/runs/tool_runtime_selftest
- user/runs/tool_pdf_merge
- user/runs/tool_csv_to_report_pdf
- user/runs/telemetry
- user/runs/pycache
- user/runs/healthcheck
- user/runs/csv_report

## 10. AUTOMATED RISK FLAGS
- 🔺 Registry drift detected: 3
- 🔺 INV-012 system immutable in runtime -> FAIL
- 🔺 INV-023 registry drift -> FAIL
- 🔸 Scripts missing SCRIPT_META: 2
- 🔸 Tools without supporting spec match: 3
- 🔸 Reproducibility markers require review: 1
- 🔸 Healthcheck reports WARN.
- 🔸 INV-009 SCRIPT_META coverage -> WARN
- 🔸 INV-010 runtime outputs declared -> WARN
- 🔸 INV-022 complete SCRIPT_META contract -> WARN
- 🔸 INV-024 orphan specs -> WARN
- 🔸 INV-025 tool reproducibility -> WARN

## 11. ARCHITECTURAL MATURITY SIGNALS
- Maturity score (0–100): 88 (GOOD)
- Separation system/dev/user: OK
- Declarative configuration level: OK
- Modularity level: not_scored
- Dependency clarity: OK
- Technical debt level: not_scored
- Portability status: OK

## 12. STRATEGIC OBSERVATIONS (FACTUAL, NOT OPINION)
- Workspace fingerprint: 3b1b8ddb92b39f871c9383a3
- Config presence: registry=yes, entrypoints=yes, llm_provider=no
- SCRIPT_META coverage: 24/26 parsed OK (ratio=0.92); missing=2, parse_errors=0
- Operational Definition discovered at: system/governance/OPERATIONAL_DEFINITION_Rev08.txt (version=unknown, estado=unknown)
- Recent runs detected under user/runs/: 10 dirs (top 10 listed).
- Python runtime footprint: 206.91 MB (4653 files) at system/bin/runtimes/python/current
- Registry drift issues detected: 3
- Tools without supporting spec match: 3
- Reproducibility review required for tools: 1

---
Generated by: dev/scripts/ai/generate_ai_system_report.py -> dev/ai/inputs/AI_SYSTEM_REPORT_v1.md
