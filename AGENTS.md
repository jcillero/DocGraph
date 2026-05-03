# AGENTS.md

## Scope

This repository is governed.  
All changes must respect specs and phase constraints.

---

## Mandatory context

Before any change, read:

- docs/specs/specs_index.json

Use it as the canonical navigation index.  
Specs are the source of truth.

---

## Scripts context

Before running validations, read:

- dev/scripts/SCRIPTS_INDEX.md

Use it to select the correct scripts.

---

## Execution rules

- Do not modify files outside the requested scope
- Do not redesign architecture
- Do not open F10 unless explicitly requested
- Do not duplicate project_runtime
- Do not place logic in UI layers

---

## Adding tools

Before adding or modifying any tool declaration or runtime behavior, read:

- docs/specs/tool_implementation_governance.md
- docs/specs/tools_catalogs.md
- docs/specs/tools_panel.md
- docs/specs/tool_creation_playbook.md

Rules:

- Classify the tool first: operational, llm, or external_dependency.
- Declare it in the correct catalog.
- Do not implement runtime unless the phase explicitly permits it.
- LLM tools are declarative only in F9/F9.5.
- Persisted outputs require owner_ref and tool_run_manifest.json.
- Never write tool outputs to project-root outputs/.

---

## Validation (required)

Minimum validation:

dev\scripts\validate_f9_declarations.bat  
dev\scripts\validate_ai_specs.bat  
dev\scripts\generate_llm_context_bundle.bat  
dev\scripts\generate_status_snapshot.bat  

If Rust code is modified:

dev\scripts\cargo_check.bat  
dev\scripts\cargo_test.bat  

---

## Notes

- resources/ declares
- crates/ implement
- assets/ is not runtime