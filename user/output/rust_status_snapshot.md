# Rust Sandbox Status Snapshot

- Workspace root: `C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\dev\scripts\..\..`
- Generated at: `02/05/2026 19:18:13,30`

---

## Executive summary

- Product identity: DocGraph
- Assistant identity: Lume
- Identity scope: declarative docs/resources only; no runtime behavior changed
- 3B: closed operationally
- F4A-F8: CLOSED
- F8: CLOSED (knowledge panel plus governed document and tools surfaces remain established over a tabbed workspace shell)
- F9-ready declarative resources: present
- F9.5 declarative/mock AI governance: completed
- validate_f9_declarations: PASS
- validate_ai_specs: PASS
- DocGraph/Lume declarative identity: present
- GUI_OBJECTS_v1: present
- Lume Help: contextual help only; no runtime, LLM, tools, mutation, or action approval
- Lume Help canonical GUI names: present
- ActionPolicy declarative resources: present
- FlowControlPolicy declarative resources: present
- PendingActionState: documented
- ActionResolution: documented
- full ActionRequest runtime: not implemented
- ActionResolution runner: not implemented
- SemanticProposal: proposal, not fact; approval requires future explicit human review
- Pipeline / Ontology View: readonly/mock presentation only
- Oxigraph/RDF/N-Quads: future preparation disabled; no RDF persisted, no SPARQL, no embeddings
- F10_PREP_GOVERNED_DOCUMENT_EDITING: documented as future proposal
- F10 minimal governed runtime: CONDITIONALLY CLOSED
- F10 broad runtime: not opened
- F10.1: minimally opened for pure llm_core capability-state resolution
- F10.2: minimally opened for assisted-chat envelope preparation
- F10.3: minimally opened for readonly EffectiveToolSurfaceSummary awareness
- F10.4: minimally opened for precomputed tool-surface summary injection into assisted-chat envelope
- F10.5: minimally opened for proposal-only ToolUseProposal modeling
- F10.6: minimally opened for inert ActionRequestDraft modeling
- F10.7: minimally opened for LlmInteractionTrace observability metadata
- F10.8: minimally opened for UI-safe LLM status view model
- provider invocation: closed
- tool execution: closed
- full ActionRequest runtime: not implemented
- ActionResolution runner: closed
- UI authority: presentation-only
- project_runtime authority: unchanged
- Governed document editing: proposal only; no runtime implementation opened and no crate changes required
- Real execution added: no
- Runtime type: governed project pipeline with thin consumers, tabbed workspace shell, readonly viewer tab, knowledge surface, document tree, structured chat references, and separated tools tabs
- Text derivation: regenerable text/page/chunk layer over primary documents
- Architectural stability: high
- Mechanical validation: passing
- Current mode: phased expansion above a closed F8 baseline

---

## Immediate next action

- keep the closed F8 narrative aligned across docs, snapshot, and invariants
- keep document text derivation reusable and regenerable
- keep the knowledge layer above existing project and workspace-tab boundaries
- keep tree, clip, chat, workspace, and viewer roles explicit and non-overlapping
- keep operational launch and LLM tool policy clearly separated
- keep the workspace content area tabbed without turning it into notebook or layout orchestration
- preserve `project_runtime` and `app_services` boundaries
- avoid introducing a second project pipeline or cross-layer mixing
- keep governed document editing as future proposal until a later phase opens implementation explicitly

---

## Maturity level

- stable core pipeline
- project vertical consumed by thin application and UI layers
- UI shell, controllers, workspace tabs, technical readonly viewer tab, minimal knowledge panel, document tree, structured chat references, and governed tools panel are present
- document text derivation and deterministic chunking groundwork are present
- F9.5 semantic proposal and AI governance layer is declared, validated, and readonly/mock
- GUI_OBJECTS_v1 canonical GUI glossary is present for Lume Help
- not production runtime

---

## System role (current)

- open a project through a thin application service over the governed project vertical
- load and validate the manifest contract
- build a usable surface model
- resolve viewer targets safely under the project root
- expose minimal runtime observability for the controlled flow
- serve a structural UI shell with explicit controllers, a tabbed workspace content area, a technical readonly viewer tab, a minimal knowledge panel, a governed document tree, structured chat references, and separated tools tabs
- derive regenerable text, optional pages, and deterministic chunks from primary documents

---

## Mental model (1-line)

- project_root -> manifest -> contract -> validation -> surface -> resolve -> output

---

## Project vertical boundaries

### Includes
- project opening
- manifest loading and contract building
- validation, surface model, and viewer target resolution
- unified project pipeline and runtime observability

### Excludes
- wide app_services
- full tool_runtime execution behavior
- full LLM integration
- production-grade UI workflow breadth
- new architectural layers

---

## Stable

- 3A closed
- 3B closed
- F4A closed
- F4B closed
- F5 closed
- F6 closed
- F7 closed
- F8 CLOSED
- F9-ready declarative resources present
- validate_f9_declarations: PASS
- DocGraph application identity and Lume assistant identity declared
- action and flow-control policies documented with execution disabled
- pending action state documented without runtime behavior
- ActionResolution documented as request-to-trace governance with execution disabled
- F9.5 AI governance and SemanticProposal schema validated with execution disabled
- Pipeline / Ontology View documented and presented as readonly/mock
- GUI_OBJECTS_v1 documented and available as shared Lume Help vocabulary
- Oxigraph/RDF/N-Quads preparation disabled; no semantic store, SPARQL, embeddings, or RDF persistence
- F10_PREP_GOVERNED_DOCUMENT_EDITING documented as future proposal
- controlled runtime path mechanically validated
- project vertical ready for higher-layer consumption
- path policy stabilized in base crates
- mechanical validation wrappers available

---

## Current focus

- preserve the closed F8 workspace model without reopening it
- preserve readonly viewer-tab behavior and thin controller boundaries
- keep `project_runtime` closed as the governed vertical heart
- avoid mixing knowledge concerns into project or tool layers
- keep existing governed documents referenced from the tree rather than reimported through chat flows
- keep clip-driven intake explicit and workspace-driven
- keep `Operational Tools` execution separate from `LLM Tools` policy
- keep workspace tabs as controlled views rather than miniapps
- maintain handoff clarity and phase discipline
- keep patch runtime, active context, and structured asset inspectors documented but not implemented
- keep F9.5 semantic proposals as proposals, not facts
- keep Pipeline / Ontology View readonly/mock and non-authoritative
- keep Lume Help explanations tied to GUI_OBJECTS_v1 canonical names

---

## Current risks / watchpoints

- `project_runtime` growing beyond vertical responsibility
- `app_services` shadowing the governed project pipeline
- `ShellController`, workspace-tab routing, or viewer flow growing into coordination or semantics
- workspace tabs growing into notebook, docking, or layout orchestration too early
- knowledge concerns drifting into project, tool, or chat workflows too early
- chat drifting into a hidden blob store instead of structured references and results
- tree and clip responsibilities blurring into duplicate intake paths
- UI becoming the source of truth for LLM tool policy or tool definitions
- primary documents being treated as if derived text were the source of truth
- semantic proposals being mistaken for approved facts
- Pipeline / Ontology View drifting into semantic runtime or approval authority
- Oxigraph/RDF preparation being mistaken for an enabled store or export path
- reopening closed phases by pushing new concerns back into lower layers
- `ProjectRuntimeOutput` absorbing presentation or cross-layer concerns
- premature subsystem expansion

---

## Output contract discipline

- `ProjectRuntimeOutput` contains only project-vertical results
- no presentation data
- no cross-domain aggregation
- no future-layer concerns

---

## Expansion policy

- build above the governed project vertical
- no new parallel project pipeline
- thin consumers stay thin
- UI and knowledge layers consume existing boundaries rather than reinterpret them
- workspace tabs remain content containers, not a new runtime or domain layer
- chat stays reference-oriented; it does not become a second document store
- roadmap remains phased and sequential
- F9 and F10 remain sequential; governed document editing preparation does not open implementation
- F9.5 declarative/mock AI governance does not open F10
- no LLM execution, autonomous tools, embeddings, RDF persistence, SPARQL, or document mutation

---

## Planned next phases

- F4A-F8 - CLOSED
- F9 - preferences / credentials
- F9.5 - declarative/mock AI governance and semantic proposal preparation
- F10 - LLM chat integration with tools
- F11 - final audit / CLOSED state verification
- F10_PREP_GOVERNED_DOCUMENT_EDITING - documented proposal only

---

## Workspace summary

### Crates present
- `action_core`
- `app_services`
- `cli_app`
- `core_domain`
- `document_text_runtime`
- `io_runtime`
- `llm_cloud`
- `llm_core`
- `llm_local`
- `project_runtime`
- `spec_runtime`
- `tool_runtime`
- `ui_core`
- `ui_i18n`
- `ui_slint`
- `ui_theme`
- `verify_progress`
- `workspace_core`

### Validation scripts present
- `dev/scripts\audit_cargo_tooling_report.bat`
- `dev/scripts\audit_f9_operational_boundary.bat`
- `dev/scripts\cargo_all.bat`
- `dev/scripts\cargo_check.bat`
- `dev/scripts\cargo_clippy.bat`
- `dev/scripts\cargo_fmt.bat`
- `dev/scripts\cargo_strict.bat`
- `dev/scripts\cargo_test.bat`
- `dev/scripts\generate_crates_context_report.bat`
- `dev/scripts\generate_llm_context_bundle.bat`
- `dev/scripts\generate_master_governance_report.bat`
- `dev/scripts\generate_status_snapshot.bat`
- `dev/scripts\validate_ai_specs.bat`
- `dev/scripts\validate_f9_declarations.bat`
- `dev/scripts\validate_f9_tool_run_output.bat`

### Relevant root docs present
- `README.md`
- `governance/GOVERNANCE.md`
- `architecture/ARCHITECTURE.md`
- `architecture/MIGRATION_BASELINE.md`
- `governance/WORKSPACE_RULES.md`
- `docs/ENGINEERING_NOTES.md`

---

## Mechanical validation

- validate_f9_declarations: PASS
- validate_ai_specs: PASS
- cargo_check: PASS
- cargo_test: PASS

---

## Consolidated capabilities

- end-to-end project runtime pipeline  
  (open -> load -> contract -> validate -> surface -> resolve)
- thin `app_services` consumer over the governed project pipeline
- `cli_app` as clean consumer of project and tool boundaries
- UI structural shell with explicit controllers, manifest wiring, and workspace tabs
- technical readonly viewer over resolved targets as a workspace tab
- minimal knowledge panel over project `knowledge/` documents
- governed document tree for existing workspace documents
- structured chat messages with document references, tool results, and system state
- explicit clip-driven document intake and workflow launch
- workspace document profiling over primary documents and derived text
- governed tools panel with controlled operational launch and declarative LLM tool policy
- `document_text_runtime` for regenerable text, pages, and chunk manifests
- minimal `tool_runtime` runner for typed accepted execution boundary
- `verify_progress` tooling available
- governed document editing proposal documented across governance and specs
- action policy, flow-control policy, and pending action state documented as F9-ready declarations
- ActionResolution documented without runner or execution authority
- AI governance resources and SemanticProposal schema validated as F9.5 declarations
- Pipeline / Ontology View readonly/mock presentation state available
- GUI_OBJECTS_v1 canonical GUI vocabulary available for Lume Help

---

## Current developer tooling

- cargo wrappers in `dev/scripts/`
- engineering notes
- scripts index

---

## Recent lessons learned

- base path policy belongs in `workspace_core`
- logical relative paths must stay portable
- critical runtime transitions must stay observable
- reusable project-opening policy belongs to the project vertical

---

## Notes usage

- full engineering notes included below for reference
- not required for immediate iteration

---

## Not guaranteed yet

- stable external API
- full production runtime behavior
- full LLM integration
- wide tool runtime behavior
- full product UI integration
- governed document patch runtime implementation
- direct document mutation from chat or LLM output
- action request runtime execution
- pending action execution behavior
- ActionResolution runner or enforcement runtime
- SemanticProposal approval runtime
- real LLM semantic derivation
- embeddings
- RDF persistence, N-Quads output, SPARQL, or Oxigraph runtime

## Engineering notes (full)

# Engineering Notes

## Purpose

This file keeps lightweight engineering lessons from the Rust sandbox without promoting them prematurely to formal invariants or broad architecture rules.

## Path comparison in tests

Windows path rendering can differ from logical path meaning.

Do not compare paths in tests through `display().to_string()` when correctness depends on path semantics rather than text rendering.

Prefer direct `Path` / `PathBuf` comparison for absolute paths, and normalize logical relative paths to a stable form when a textual comparison is still useful.

## Runtime observability for critical transitions

Critical transitions in the controlled runtime path should remain observable.

The `snapshot -> contract` step was a real example: the flow was working, but the transition was initially outside the minimal event model and therefore reduced traceability on failure.

## Project opening policy ownership

If project opening follows a reusable vertical convention, that convention should not remain retained in `cli_app`.

The entrypoint may still validate inputs and orchestrate the flow, but reusable project-opening policy belongs to the project vertical.

## Sandbox assets are staging, not runtime truth

The sandbox `assets/` folder is a manual staging area for cleanup, naming normalization, and candidate selection.

It is not the canonical runtime resource location. Governed runtime resources still belong in `resources/` when and if promotion is approved.

## Root docs should track runtime maturity

Root sandbox documents should reflect the real maturity of the Rust runtime path without overstating it.

When the sandbox gains real controlled runtime behavior, the docs should stop describing it as structure-only, while still making clear that it is not the final product runtime.

## Base path policy belongs in `workspace_core`

A Windows-specific path issue first appeared in higher crates, but the real policy boundary was the workspace layer.

Portable path behavior should be fixed in the base path layer rather than patched independently in each consuming crate.

Impact: once the base policy is corrected, `spec_runtime`, `project_runtime`, and their tests can consume a consistent path model instead of compensating locally.

## Relative workspace paths are logical portable paths

Workspace-relative paths represent logical system structure, not OS-native rendering.

Their portable form should stay stable across platforms and should not depend on native separator rendering.

Impact: portable metadata and tests can rely on one logical path form without treating Windows rendering as source of truth.

## Absolute Windows paths need robust equivalence

Absolute system paths on Windows may differ in textual rendering while still identifying the same location, especially with the `\\?\` prefix.

Comparisons for absolute paths should therefore use robust equivalence or small normalization rather than literal rendered strings.

Impact: workspace boundary checks and runtime-safe path assertions become less fragile on Windows without changing runtime behavior.

## Portable metadata should not depend on native path rendering

In `spec_runtime`, consumable metadata for declarative documents is part of the portable system model.

That metadata should expose a stable logical representation instead of depending on `display().to_string()`.

Impact: portable document descriptors remain stable across operating systems and are safer to use in tests and higher runtime layers.

## Phase closure requires mechanical validation

A phase should not be treated as formally closed on architecture alone.

Closure should wait for real mechanical validation such as `cargo check` and `cargo test`.

Impact: the sandbox keeps a clean distinction between architectural readiness and verified build/test closure.
