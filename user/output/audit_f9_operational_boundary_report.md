# F9 Operational Boundary Audit Report

- generated_at: 2026-04-30 21:41:15
- workspace_root: C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app
- status: PASS
- manifests_checked: 2
- valid_text_measure_runs: 1
- legacy_runs_excluded: 1

## Verified

- minimum governed output must be owner-scoped under `user/output/tool_runs/<owner-kind>/<owner-id>/<run_id>/`
- completed `text.measure` runs require `tool_run_manifest.json`, `owner_ref`, and `result.json`
- UI, LLM cloud, ActionRequest, and master-catalog boundaries are scanned for F10 drift signals

## Warnings

- legacy non-owner-scoped run excluded from F10 gate: text_measure_1777408460421477700

## Errors

- none

## Conclusion

- minimum governed F9 execution slice present
- no clear runtime drift to F10 detected by this audit
- F10 remains `NOT OPENED`
