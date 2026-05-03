# project_runtime

## Purpose

Provide the Rust successor layer for project-scoped runtime behavior, inheriting the live Python project model.

## Responsibilities

- treat the project as the primary working unit
- preserve project isolation and project-scoped path safety
- validate manifest-driven exposure rules
- resolve authorized refs from manifest declarations
- build neutral surface models for controller or UI-facing layers
- preserve separation between artifacts, outputs, and runs

## Inputs

- workspace root and project root descriptors
- project metadata and manifest payloads
- scoped reference requests
- controller or service-side project operations

## Outputs

- project descriptors
- manifest validation results
- normalized authorized refs
- neutral surface models
- runtime-safe project operation errors

## Allowed Dependencies

- standard library
- `core_domain`
- `workspace_core`
- `io_runtime`
- `spec_runtime`

## Forbidden Responsibilities

- no direct UI widget logic
- no filesystem-driven visibility model
- no direct provider logic
- no uncontrolled absolute path exposure
- no workflow execution engine yet

## Initial Phase Scope

- project identity and scope checks
- manifest validation
- ref resolution
- neutral surface model construction
- no full project execution lifecycle yet

## F9 limits

- no `registry.json` generator is implemented
- no graph writer or graph runtime is implemented
- no passive filesystem scanning may expose project artifacts
- no runtime `owner_ref` enforcement is implemented yet
- no persisted `tool_run_manifest.json` generation is implemented yet
- no chat artifact promotion runtime is implemented yet
- no document creation or export runtime is implemented yet
- no Project Setup runtime is implemented
- no Project Settings runtime is implemented
- no `project_profile` enforcement runtime is implemented
- no sandbox working copy creation is implemented

## F12.4 file intake project-runtime boundary

`docs/specs/file_intake.md` owns governed file intake semantics.

F12.4 does not modify `project_runtime` behavior.

Future file intake must not bypass the governed project pipeline. `project_manifest.json` remains exposure authority, and passive filesystem presence must not create project exposure.

`project_runtime` must not be expanded by convenience for intake without a later explicit implementation slice and audit.

## F12.5 file intake project-runtime plan

`F12.5` is plan-only and does not modify `project_runtime`.

Future `F12.6` must not modify `project_runtime` unless a later explicit audit approves project exposure or project-pipeline participation.

Future `F12.6` must not mutate `project_manifest.json`, infer exposure, bypass `project_runtime`, or add convenience project authority for intake.

## F13.0 project exposure gate boundary

`F13.0` is SPEC-ONLY and does not modify `project_runtime`.

`docs/specs/file_intake.md` owns the Project Exposure Gate semantics.

Future project exposure must remain manifest-driven:

- `project_manifest.json` is the only exposure authority
- future exposure may only promote supported `imported_not_exposed` intake objects after governed request, candidate, and human confirmation
- blocked and unsupported intake items must not be exposed
- `owner_ref` and `trace_ref` must be preserved into any future manifest-governed exposure record
- duplicate exposure must not collapse distinct `IntakeItem`, `file_ref`, or logical object identity

`project_runtime` may later prepare document tree state from manifest-exposed objects only after a separate implementation slice explicitly opens the manifest update path.

`project_runtime` must not infer exposure from filesystem scanning, `file_store` presence, intake metadata, registry presence, graph presence, history/index rows, chat references, or UI selection.

F13.0 does not create a manifest writer, registry generator, graph writer, derivative runtime, `document_text_runtime` call, `tool_runtime` call, rollback flow, or revocation flow.

## F13.1 legacy bypass hardening boundary

`F13.1` is SPEC-only and does not modify `project_runtime`.

Before any future `F13` runtime implementation:

- `project_runtime` must not accept `list_project_documents` or any filesystem scan as project exposure authority
- `project_runtime` must not treat copied files, chat resources, registry rows, graph rows, or derivatives as exposure authority
- `project_runtime` must not accept UI-triggered import or derivation as an exposure substitute
- prepared tree state must remain manifest-driven only

Legacy filesystem listing may remain for fixture or migration tests only until a later migration removes or isolates it from production authority.

## F13.4 manifest exposure contract boundary

`F13.4` is SPEC-only and does not modify `project_runtime`.

Future `project_runtime` may consume manifest-governed exposure only through a later explicit implementation slice that reads a governed `ManifestExposureEntry` shape from `project_manifest.json`.

Rules:

- `project_runtime` must treat `ManifestExposureEntry` as the minimum future exposure authority record
- `project_runtime` must not infer exposure from intake history/index, filesystem presence, blob presence, registry rows, graph rows, or `System View` visibility
- `project_runtime` must preserve `owner_ref`, `trace_ref`, source intake linkage, and duplicate-safe logical identity when preparing any future tree state
- `project_runtime` must not treat accepted human confirmation as exposure unless a later explicit runtime slice also writes manifest authority
- `project_runtime` must not create registry rows, graph rows, derivatives, or `document_text_runtime` side effects as part of future exposure by convenience

## F13.5A manifest exposure runtime plan boundary

`F13.5A` is SPEC-only and does not modify `project_runtime`.

Future `F13.5B` runtime ownership is reserved to `project_runtime`.

Boundary rules:

- `project_runtime` is the only approved future manifest writer boundary
- `project_runtime` may expose one already `imported_not_exposed` item per minimal runtime invocation
- `project_runtime` must require `ExposureRequest`, `ExposureCandidate`, accepted `HumanConfirmation`, non-stale candidate state, `owner_ref`, and `trace_ref`
- `project_runtime` must reject blocked items, unsupported items, stale candidates, rejected confirmations, and duplicate-policy violations
- `project_runtime` must not generate `registry.json`, graph writes, derivatives, or `document_text_runtime` side effects as part of manifest exposure
