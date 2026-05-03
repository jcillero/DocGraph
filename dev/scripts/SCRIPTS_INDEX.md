# Scripts Index

This directory contains the local Windows wrapper scripts used for mechanical Rust validation in this sandbox.

## Available scripts

- `cargo_check.bat`
  - purpose: compilation validation for the Rust workspace
  - when to use: when verifying that the workspace still builds after Rust changes
  - execution order: standalone or after `cargo_fmt.bat` and `cargo_clippy.bat`

- `cargo_test.bat`
  - purpose: test validation for the Rust workspace
  - when to use: when verifying behavioral correctness after Rust changes
  - execution order: standalone or after `cargo_check.bat`

- `cargo_fmt.bat`
  - purpose: formatting validation without applying changes
  - when to use: when checking formatting consistency or before strict validation
  - execution order: first step in `cargo_strict.bat`

- `cargo_clippy.bat`
  - purpose: lint validation with warnings treated as errors
  - when to use: when checking code quality or before strict validation
  - execution order: after `cargo_fmt.bat` in `cargo_strict.bat`

- `cargo_all.bat`
  - purpose: standard Rust validation pipeline
  - when to use: default validation after meaningful Rust changes
  - execution order: runs `cargo_check.bat` then `cargo_test.bat`

- `cargo_strict.bat`
  - purpose: full strict Rust validation pipeline
  - when to use: recommended before commits or when a higher-confidence validation pass is needed
  - execution order: runs `cargo_fmt.bat`, `cargo_clippy.bat`, `cargo_check.bat`, then `cargo_test.bat`

- `generate_status_snapshot.bat`
  - purpose: generate a compact Markdown snapshot of the current Rust sandbox state
  - when to use: when preparing context for a new chat or a lightweight status handoff
  - execution order: standalone; runs `cargo_check.bat` and `cargo_test.bat` internally and writes `user/output/rust_status_snapshot.md`

- `generate_master_governance_report.bat`
  - purpose: regenerate the consolidated governance and specs report under `governance/reports/`
  - path policy: canonical `README.md` seed retained intentionally; `governance/`, `architecture/`, and `docs/specs/` content collected recursively for layout robustness
  - when to use: when preparing a compact handoff or single-file reading surface for architecture and governance work
  - execution order: standalone; writes `governance/reports/MASTER_GOVERNANCE_AND_SPECS_REPORT.md`

- `generate_crates_context_report.bat`
  - purpose: generate a crates context report; it does not modify runtime
  - when to use: when preparing crate/dependency context for architecture review, handoff, or workspace inspection
  - execution order: standalone; writes `user/output/crates_context_report.md`

- `generate_llm_context_bundle.bat`
  - status: STABLE / Codex-critical
  - purpose: generate structured LLM context bundle from project specs and outputs
  - path policy: intentional canonical seed list for core governance/architecture anchors, plus recursive discovery for broad specs/resources/output coverage
  - codex usage: primary input for Codex reasoning
  - when to use: before invoking Codex or AI-assisted workflows
  - execution order: after status snapshot

- `validate_f9_declarations.bat`
  - purpose: validate F9-ready declarative resources without opening runtime execution
  - when to use: after changes to Lume, preferences, runtime placeholders, LLM catalogs, tools catalogs, sandbox policy, or storage policy
  - execution order: standalone; writes `user/output/validate_f9_declarations_report.json`

- `validate_f9_tool_run_output.bat`
  - purpose: validate F9 output discipline for persisted tool runs without modifying runtime or executing tools
  - scope: owner-scoped `user/output/tool_runs/` manifests, owner refs, statuses, result presence, and legacy-run exclusion from the F10 gate
  - when to use: after local F9 operational slices persist outputs under `user/output/tool_runs/`
  - execution order: standalone; writes `user/output/validate_f9_tool_run_output_report.txt`

- `audit_f9_operational_boundary.bat`
  - purpose: audit F9 operational boundary compliance
  - scope: minimal runtime slice verification for `text.measure`, owner-scoped persisted outputs, UI boundary, LLM execution closure, ActionRequest drift, and master-catalog runtime isolation
  - limitation: textual scan; some findings are advisory
  - when to use: after Hito 2 and before declaring F9 operativo cerrado
  - execution order: standalone; writes `user/output/audit_f9_operational_boundary_report.md`

- `audits/audit_f9_boundary_drift.bat`
  - status: EXPERIMENTAL / advisory only
  - purpose: provide a coarse textual scan for possible F9/F10_PREP boundary drift in specs
  - limitation: this script is not Codex-ready and must not be used as an automatic fix input because textual matches can confuse governance rules with real violations
  - codex usage: DO NOT USE as automatic patch input; human review only
  - when to use: only after spec work that may blur declarative boundaries or reopen execution semantics implicitly
  - execution order: standalone; writes `user/output/audit_f9_boundary_drift_report.txt`

- `audits/audit_python_scope.bat`
  - status: STABLE / Codex-readable
  - purpose: ensure `Python` remains confined to allowed governance/architecture doctrinal and migration documents and only in historical or non-runtime context
  - path policy: explicit allowlist is intentional because it is part of the security/governance boundary, not broad document discovery
  - codex usage: use as context only; Codex may propose minimal documentation wording fixes for unexpected active-runtime Python references
  - limitation: do not use it to delete historical Python doctrine, migration notes, or governance references
  - when to use: after documentation changes that might reintroduce Python as runtime, dependency, or active implementation
  - execution order: standalone; writes `user/output/audit_python_scope_report.txt`

- `validate_ai_specs.bat`
  - status: STABLE / declarative validation
  - purpose: validate F9.3/F9.4 declarative AI governance contracts without opening AI execution
  - codex usage: use as validation evidence only; Codex may fix schema/resource inconsistencies but must not introduce runtime AI execution
  - limitation: validates declarative AI specs only; it must not be interpreted as readiness for LLM invocation, embeddings, semantic runtime, or provider execution
  - when to use: after changes to `resources/ai/` prompt specs, semantic specs, sensitivity policy, or AI schemas
  - execution order: standalone; writes `user/output/validate_ai_specs_report.json`

- `audits/audit_f9_artifact_lineage.bat`
  - status: STABLE / Codex-ready
  - purpose: audit F9 artifact lineage semantics across SOURCE, DERIVED, and ARTIFACT references
  - codex usage: use as patch input only for entries explicitly classified as actionable; Codex must ignore guarded, declarative, test, and allowed artifact contexts
  - limitation: heuristic textual auditor; do not use it to redesign document runtime or change lineage contracts
  - when to use: after changes affecting document artifacts, derived text, editable artifacts, document holders, or artifact/source terminology
  - execution order: standalone; writes `user/output/audit_f9_artifact_lineage_codex.json`

- `audits/audit_f9_output_ownership.bat`
  - status: STABLE / Codex-ready
  - purpose: audit F9 ownership and traceability of generated outputs, tool outputs, owner references, and tool run manifests
  - codex usage: use as patch input only for missing ownership/traceability findings; Codex must not modify layout declarations, guarded declarations, declarative context, tests, or error messages
  - limitation: heuristic textual auditor; do not use it to introduce runtime owner enforcement unless explicitly requested
  - when to use: after changes affecting `tool_outputs/`, `tool_run_manifest.json`, `owner_ref`, chat-owned outputs, DocumentHolder-owned outputs, or knowledge-owned outputs
  - execution order: standalone; writes `user/output/audit_f9_output_ownership_codex.json`

- `audits/audit_f10_llm_execution_guard_codex.bat`
  - status: STABLE / Codex-ready
  - purpose: audit accidental LLM/provider/embedding/SPARQL/semantic-runtime execution drift before F10 opens runtime execution
  - codex usage: use as patch input only for `REVIEW_REQUIRED` entries after classification; Codex must not modify declarative capability entries, guarded declarations, or real execution surfaces without approval
  - limitation: does not prove absence of all execution paths; it is a targeted textual guard for LLM and semantic execution boundaries
  - when to use: after changes affecting LLM catalogs, AI specs, tools policy, provider wiring, semantic store terminology, or runtime execution surfaces
  - execution order: standalone; writes `user/output/audit_f10_llm_execution_guard_codex.json`

- `audits/audit_f10_step1_boundary.bat`
  - status: ADVISORY
  - purpose: scan for possible F10 step-1 boundary drift before runtime implementation begins
  - codex usage: DO NOT use as automatic patch input
  - limitation: textual scan only; false positives expected; human review required
  - when to use: before F10.1 runtime work and after any F10 governance/spec change
  - execution order: standalone; writes `user/output/audit_f10_step1_boundary_report.txt`

- `audits/audit_f10_final_boundary.bat`
  - status: ADVISORY
  - purpose: scan for possible F10 boundary drift after minimal governed runtime slices are present
  - codex usage: DO NOT use as automatic patch input
  - limitation: textual scan only; false positives expected; human review required
  - when to use: after F10 minimal runtime slices change and before declaring boundary integrity
  - execution order: standalone; writes `user/output/audit_f10_final_boundary_report.txt`

- `audits/audit_f12_minimal_runtime_boundary.bat`
  - status: FUTURE / REQUIRED FOR F12.2B
  - purpose: audit the first minimal governed `text.measure` runtime boundary before closing a later implementation slice
  - scope: no `project_runtime` changes, no `app_services` or UI execution authority, no general dispatcher, no provider calls, no external binary invocation, no network access, no `HOST` write, only `text.measure` executable, required `owner_ref`, required `trace_ref`, required manifest, owner-scoped outputs, and no project-root outputs
  - codex usage: required validation evidence for a future `F12.2B` implementation; do not treat this entry as implementing or executing the audit script
  - limitation: declared in `F12.2A` as future required; script may not exist until the later implementation slice explicitly creates it
  - when to use: after the future `F12.2B` minimal `text.measure` runtime implementation and before declaring closure
  - execution order: standalone; report path must be declared by the future script

- `audits/audit_f12_file_intake_boundary.bat`
  - status: BASELINE VALIDATION / F12.6 CLOSED
  - purpose: verify governed file intake boundaries for the minimal intake baseline and before any later intake expansion
  - scope: no UI filesystem mutation, no `app_services` intake orchestration, no `project_runtime` bypass, no `project_manifest.json` mutation unless explicitly opened, no registry generation, no graph writes, no derivative generation, no raw host paths persisted as portable truth, no secrets in metadata, no `assets/` runtime use, no passive filesystem exposure, no broad `tool_runtime` orchestrator, no provider/network/external binary calls, required `owner_ref`, required `trace_ref`, sanitized `user_comment`, readonly source folder, governed owner/project-scoped output destination, content-hash dedup discipline, distinct `IntakeItem` identity for duplicate selections, no project exposure from blob reuse, readonly derivable non-authoritative intake history/index visibility, and future project exposure gate checks for explicit confirmation, manifest-only authority, duplicate-safe exposure, no blocked or unsupported exposure, and no automatic exposure from intake or `file_store` presence
  - codex usage: mandatory validation evidence for `F12.6` closure and for later intake-boundary changes; use after relevant validation and do not use as automatic patch input without reviewing findings
  - limitation: advisory/static where git metadata is unavailable
  - output: `user/output/audit_f12_file_intake_boundary_report.txt`
  - when to use: after changes affecting the minimal governed intake baseline and before declaring any later intake expansion
  - execution order: standalone after relevant Rust validation

- `audits/audit_f13_exposure_boundary.bat`
  - status: ACTIVE / REQUIRED BEFORE F13 RUNTIME CLOSURE
  - purpose: verify that project exposure remains manifest-governed and that no legacy import, filesystem scan, chat-resource, direct derivation, UI, or `app_services` path becomes an implicit exposure pipeline
  - scope: canonical `F12 intake -> F13 exposure gate -> manifest -> document tree` path only; no `import_project_document` production authority, no `list_project_documents` tree authority, no `register_chat_resource` promotion into intake or exposure authority, no direct UI derivation, no `project_manifest.json` inference from copied files or filesystem presence, no registry or graph authority, no derivative side effects from exposure, no `document_text_runtime` trigger during exposure, no `tool_runtime` orchestration, and no raw host-path style manifest exposure shortcuts
  - codex usage: active validation evidence only; review FAIL and WARN findings before any F13 runtime work
  - limitation: scans production Rust code only, ignores lines after the first `#[cfg(test)]` marker in each file, and still emits WARN findings for tolerated legacy chat-local helper presence
  - output: `user/output/audit_f13_exposure_boundary_report.txt`
  - when to use: before any future `F13` runtime closure and after any migration that touches legacy import, tree visibility, chat resource, or derivation paths
  - execution order: standalone after declarative validation and any relevant Rust validation

- `audits/audit_f13_manifest_exposure_runtime_boundary.bat`
  - status: ACTIVE / REQUIRED BEFORE F13.5B CLOSURE
  - purpose: verify that any future manifest exposure runtime remains confined to the approved `project_runtime` boundary and obeys the `F13.4` request, candidate, confirmation, and manifest-authority chain
  - scope: no `project_manifest.json` writes from `ui_slint`, `app_services`, `io_runtime`, `tool_runtime`, or `document_text_runtime`; no filesystem-scanning authority; no registry, graph, or derivative writes from exposure; no `document_text_runtime` trigger; no `tool_runtime` orchestration; no raw host absolute paths in manifest exposure outputs; no bypass of `ExposureRequest -> ExposureCandidate -> accepted HumanConfirmation`
  - codex usage: active mandatory validation evidence for exposure-boundary hardening and for any future `F13.5B` runtime implementation
  - limitation: deterministic pattern/path-based audit only; avoids deep semantic flow inference and may emit WARN findings for tolerated legacy chat-local or migration-only helper presence
  - output: `user/output/audit_f13_manifest_exposure_runtime_boundary_report.txt`
  - when to use: before and after any future `F13.5B` manifest exposure runtime work and before declaring `F13.5B` closure
  - execution order: standalone after declarative validation, `audit_f13_exposure_boundary.bat`, and any relevant Rust validation

- `audits/audit_file_store_contract.bat`
  - status: ADVISORY / human review only
  - purpose: detect possible textual drift in governed `file_store`, `StoredObject`, refs, and storage-authority contracts
  - codex usage: DO NOT USE as automatic patch input
  - limitation: textual scan only; possible false positives; no autofix
  - when to use: after storage-policy or governed schema changes
  - execution order: standalone; writes `user/output/audit_file_store_contract_report.txt`

- `audits/audit_semantic_quad_contract.bat`
  - status: ADVISORY / human review only
  - purpose: detect possible textual drift in semantic quad identity, lifecycle, and relation boundaries
  - codex usage: DO NOT USE as automatic patch input
  - limitation: textual scan only; possible false positives
  - when to use: after semantic quad, lifecycle, or relation spec/schema changes
  - execution order: standalone; writes `user/output/audit_semantic_quad_contract_report.txt`

- `audits/audit_rdf_projection_boundary.bat`
  - status: ADVISORY / human review only
  - purpose: detect possible textual drift in RDF projection boundaries and lifecycle-filtering rules
  - codex usage: DO NOT USE as automatic patch input
  - limitation: textual scan only; possible false positives; RDF remains future projection only
  - when to use: after RDF projection policy or semantic projection changes
  - execution order: standalone; writes `user/output/audit_rdf_projection_boundary_report.txt`

- `audits/audit_graph_analysis_boundary.bat`
  - status: ADVISORY / human review only
  - purpose: detect possible textual drift in future semantic graph analysis boundaries
  - codex usage: DO NOT USE as automatic patch input
  - limitation: textual scan only; possible false positives; no inference or execution runtime is opened
  - when to use: after graph-analysis policy or semantic analysis boundary changes
  - execution order: standalone; writes `user/output/audit_graph_analysis_boundary_report.txt`

- `audit_cargo_tooling_report.bat`
  - purpose: generate cargo tooling report without modifying runtime
  - when to use: when checking cargo workspace tooling availability and lightweight diagnostics
  - execution order: standalone; writes `user/output/audit_cargo_tooling_report.md`

- `audit_f12_minimal_runtime_boundary.bat`
  - status: REQUIRED (F12.2B)
  - purpose: ensure minimal runtime implementation does not break governance boundaries
  - limitation: heuristic scan, may produce warnings
  - codex usage: advisory only, not patch input
  - when to use: before closing F12.2B
  - execution order: after cargo_check and cargo_test
  - output: user/output/audit_f12_minimal_runtime_boundary_report.txt
  - path resolution: script-relative using `%~dp0` to avoid working-directory issues
  
## Usage rule

Use these scripts as the execution interface for Rust mechanical validation in this sandbox.

After meaningful Rust changes, run `cargo_all.bat`.

Use `cargo_strict.bat` for strict validation. It is recommended before commits.
