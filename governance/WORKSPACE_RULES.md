# Workspace Rules

## Structural rules

- one primary responsibility per crate
- keep dependency direction controlled and inward
- do not create parallel implementations of the same governed flow
- keep edge crates thin
- keep UI crates free of business logic, filesystem ownership, and runtime ownership
- do not couple application code directly to a provider-specific LLM implementation

## Workspace and path discipline

- workspace-relative paths are logical portable paths, not OS-rendered strings
- compare absolute paths robustly; do not depend on platform-specific textual rendering
- fix path policy in base crates rather than patching each higher crate separately
- persist portable relative paths instead of absolute host-specific paths wherever the contract requires it

## Validation discipline

- use the Rust workspace as the center of implementation
- mechanical validation runs through `dev/scripts/` wrappers in the local Windows environment
- read `dev/scripts/SCRIPTS_INDEX.md` before using wrappers
- default validation after relevant Rust changes: `dev\scripts\cargo_all.bat`
- stricter validation before higher-risk closure: `dev\scripts\cargo_strict.bat`

## Current layering rules

- `project_runtime` owns the project pipeline
- `app_services` consumes governed verticals and does not replace them
- `cli_app` stays an entrypoint, not a domain orchestrator
- `tool_runtime` stays minimal and separate from project and knowledge concerns until a later phase explicitly opens more
- `ui_slint` consumes controllers and presentation state; it does not own filesystem or domain policy
- the central workspace area is tabbed; each tab is a controlled view, not an autonomous miniapp
- the readonly viewer is one workspace tab, not the whole workspace model
- the document tree is the reference surface for existing governed documents already inside the system
- the clip is the explicit surface for external intake and document workflow launch
- the chat layer carries structured references and results; it does not become a second document store
- document profiling belongs to workspace tabs and thin controllers; it does not move into `project_runtime`
- the knowledge layer consumes existing viewer and controller boundaries; it does not reinterpret the manifest
- the tools surface stays split between controlled operational launch and declarative LLM tool policy
- project-level LLM tool overrides must remain declarative and intersect with the global allowed surface
- visible product identity is `DocGraph` and assistant identity is `Lume`; this is declarative identity, not runtime authority
- Lume is an interaction and help layer; it does not execute filesystem, tools, or LLM workflows
- Lume Help is contextual governed help; it does not execute, mutate, approve actions, call LLMs, persist history, or create project data
- Lume Help must use `GUI_OBJECTS_v1` canonical names when explaining interface objects
- GUI object explanations should name the object explicitly and avoid vague spatial references without the canonical object name
- visible UI strings must come from governed resources and i18n, not hardcoded Slint views
- `resources/` declares; `runtime/` materializes placeholders; `user/` stores user-owned preferences and data
- external runtime tools remain missing or disabled until a later phase explicitly opens validation and execution
- local sandbox mutation remains disabled until authorized policy and logging are implemented
- action and flow-control policies remain declarative in F9; they do not execute or approve runtime behavior
- ActionResolution remains declarative in F9; it does not create a runner, invoke LLM/tools, or mutate files
- UI may present pending actions in a future phase, but every execution must reference an explicit `action_id`
- document templates remain declarative proposals until a later phase opens template/runtime behavior
- future document export must use package data and export profiles, not UI state
- reference styles remain declarative resources under `resources/reference_styles/`; corporate styles must not be hardcoded in code or UI
- BibTeX and bibliography handling remain proposals; no parser, importer, or export materialization exists yet
- F9.5 semantic proposals remain proposals, not facts; approval requires future explicit human review
- Pipeline / Ontology View is readonly/mock representation; it does not infer, approve, execute, persist RDF, run SPARQL, or mutate files
- Oxigraph/RDF preparation remains disabled: no Oxigraph dependency, embeddings, semantic store, or N-Quads output is created
- project folders must not contain a root `outputs/` directory; outputs belong to the functional owner
- every future tool output must carry an `owner_ref`
- tool outputs live under the owning chat, DocumentHolder, or knowledge derivation/proposal area
- `graph/` explains artifact relations; it does not govern, execute, approve, or replace `project_runtime`
- `registry.json` is a navigation accelerator; it does not replace `project_manifest.json` or `graph/`
- filesystem presence must not imply project exposure; `project_manifest.json` governs exposure
- chat attachments are session context; durable knowledge or document inputs require explicit promotion
- `DocumentHolder` owns document production state; `document.md` is its only editable textual source
- graph entries must originate from governed actions and remain traceable to document, chat, knowledge, or tool origins
- referenced artifacts must exist, resolve, and be exposed by `project_manifest.json`
- artifact IDs must remain stable across moves and renames and must not depend on filesystem path
- prefer stable references over duplicating the same physical file across knowledge, chats, or documents
- persisted tool runs must declare inputs, configuration, `owner_ref`, outputs, status, and trace data in `tool_run_manifest.json`
- ordered inputs must declare `ordered=true` when order affects result
- tool configuration must not be mixed with input data
- persisted tool runs require structured lifecycle, structured errors/warnings, and reproducible inputs/configuration
- UI may collect ordered input lists but must not execute or validate domain logic
- UI may capture `ProjectSetupDraft` and `ProjectSettingsDraft`, but it does not govern project configuration authority
- project profiles declare intention, not runtime authority
- selected external folders are readonly; sandbox mutations may affect only a future working copy
- LLM receives an effective tool surface summary by default, not raw catalogs
- expanded tool context must come from a governed resolver or provider, not from UI state or LLM guessing
- preferences are non-secret configuration
- credentials are referenced, not embedded
- project files must not contain secret values
- LLM context must not receive credentials
- changing preferences or `credential_ref` does not enable execution
- the app must remain usable without LLM
- Lume Static Mode uses declarative help
- Lume help-tree content is declarative navigation, explanation, and procedure state
- missing LLM or tools is visible capability state, not failure
- project setup must not depend on LLM availability
- UI renders prepared help-tree state only and does not infer policy or execute actions
- the main menu remains minimal and stable: `File`, `Preferences`, `Tools`, `Help`
- Tools is a top-level menu domain separate from Preferences and exposes separate governed entries for Operational Tools, LLM Tools, and External Dependencies
- visible menu text must come from i18n resources
- the bottom status bar displays prepared state only
- the bottom status bar exposes no execution, no secrets, and no policy inference
- the bottom status bar may provide navigation intents only
- the top-left DocGraph icon is branding plus a Lume Help shortcut
- clicking the DocGraph icon must not execute tools, LLM, project actions, filesystem mutations, or settings changes
- the central layout remains Tree + Chat + Tabs Workspace
- viewer belongs to Tabs Workspace
- Sandbox View may appear as a conditional tab only when relevant
- Sandbox View is informational in F9 and must not execute filesystem operations

## Migration rules

- Inherited doctrine remains reference only
- Rust is the active implementation target in this sandbox
- implementation growth must follow documented crate boundaries
- governance-relevant changes must be reflected in root docs
- placeholder code is acceptable only when it does not fake implemented product behavior

## Candidate rules

- reusable runtime policies should not remain in an entrypoint once they clearly belong to a governed vertical
- critical steps in the controlled runtime path should not remain outside minimal structured observability
- higher layers should adapt around governed outputs instead of widening them for presentation or cross-layer convenience
- knowledge navigation should remain readonly and should not grow into editing, semantics, or notebook behavior before the corresponding phase opens
- viewer and inspectors are not free editors
- `SOURCE` and `DERIVED` documents remain readonly
- `ARTIFACT` documents may be edited only through governed patch acceptance
- patch validation and application must not live in UI crates
- chat may reference edit requests, selections, and proposals, but it must not store source blobs or act as a document store
- document template guidance is editorial and declarative; it must not execute, mutate, or decide policy
- document references must remain structured proposals; referenced `SOURCE` and `DERIVED` stay readonly
- semantic proposal UI must represent declarations and traces only; no hidden semantic inference belongs in UI
- GUI object names are shared help vocabulary, not runtime authority

## F12.4 file intake boundary

- `docs/specs/file_intake.md` owns governed file intake semantics.
- File intake is SPEC-only in F12.4.
- Source host paths are not portable identity.
- Original host files remain readonly.
- Filesystem presence does not imply project exposure.
- `project_manifest.json` remains exposure authority.
- UI and `app_services` must not mutate storage, import files, derive text, or expose intake items.
- `assets/` must not be used as file-intake runtime storage.

## F12.5 file intake planning boundary

- `F12.5` is plan-only and audit-checklist-only.
- `F12.5` introduces no runtime and touches no crates.
- Future `F12.6` must use explicit user-selected files only.
- Future `F12.6` must keep source folders readonly.
- Future `F12.6` must not persist private absolute host paths as portable truth.
- Future `F12.6` must not mutate `project_manifest.json`, generate registries, write graph entries, or generate derivatives unless a later phase explicitly opens those scopes.
- Future `audit_f12_file_intake_boundary.bat` is mandatory before closure.
