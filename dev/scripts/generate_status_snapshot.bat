@echo off
setlocal

set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..\..
set "OUTPUT_DIR=%ROOT_DIR%\user\output"
set "SNAPSHOT_FILE=%OUTPUT_DIR%\rust_status_snapshot.md"
set "ENGINEERING_NOTES_FILE=%ROOT_DIR%\docs\ENGINEERING_NOTES.md"

echo [INFO] Generating Rust sandbox status snapshot...

if not exist "%OUTPUT_DIR%" (
    mkdir "%OUTPUT_DIR%"
    if errorlevel 1 (
        echo [ERROR] Failed to create output directory.
        endlocal
        exit /b 1
    )
)

set "CHECK_STATUS=FAIL"
set "TEST_STATUS=FAIL"
set "F9_DECLARATIVE_VALIDATION_STATUS=FAIL"
set "F9_BOUNDARY_DRIFT_AUDIT_STATUS=FAIL"
set "PYTHON_SCOPE_AUDIT_STATUS=FAIL"
set "AI_SPECS_VALIDATION_STATUS=FAIL"
set "ARCH_STABILITY=high"
set "MECH_VALIDATION_STATUS=failing"
set "RUNTIME_VALIDATION_SUMMARY=controlled runtime path awaiting green mechanical validation"

echo [INFO] Running validate_f9_declarations.bat...
call "%SCRIPT_DIR%validate_f9_declarations.bat"
if errorlevel 1 (
    echo [ERROR] validate_f9_declarations.bat failed
    set "F9_DECLARATIVE_VALIDATION_STATUS=FAIL"
) else (
    set "F9_DECLARATIVE_VALIDATION_STATUS=PASS"
    echo [OK] validate_f9_declarations.bat passed
)

echo [INFO] Running audits\audit_f9_boundary_drift.bat...
call "%SCRIPT_DIR%audits\audit_f9_boundary_drift.bat"
if errorlevel 1 (
    echo [ERROR] audit_f9_boundary_drift.bat failed
    set "F9_BOUNDARY_DRIFT_AUDIT_STATUS=FAIL"
) else (
    set "F9_BOUNDARY_DRIFT_AUDIT_STATUS=PASS"
    echo [OK] audit_f9_boundary_drift.bat passed
)

echo [INFO] Running audits\audit_python_scope.bat...
call "%SCRIPT_DIR%audits\audit_python_scope.bat"
if errorlevel 1 (
    echo [ERROR] audit_python_scope.bat failed
    set "PYTHON_SCOPE_AUDIT_STATUS=FAIL"
) else (
    set "PYTHON_SCOPE_AUDIT_STATUS=PASS"
    echo [OK] audit_python_scope.bat passed
)

echo [INFO] Running validate_ai_specs.bat...
call "%SCRIPT_DIR%validate_ai_specs.bat"
if errorlevel 1 (
    echo [ERROR] validate_ai_specs.bat failed
    set "AI_SPECS_VALIDATION_STATUS=FAIL"
) else (
    set "AI_SPECS_VALIDATION_STATUS=PASS"
    echo [OK] validate_ai_specs.bat passed
)

echo [INFO] Running cargo_check.bat...
call "%SCRIPT_DIR%cargo_check.bat"
if errorlevel 1 (
    echo [ERROR] cargo_check.bat failed
    set "CHECK_STATUS=FAIL"
) else (
    set "CHECK_STATUS=PASS"
    echo [OK] cargo_check.bat passed
)

echo [INFO] Running cargo_test.bat...
call "%SCRIPT_DIR%cargo_test.bat"
if errorlevel 1 (
    echo [ERROR] cargo_test.bat failed
    set "TEST_STATUS=FAIL"
) else (
    set "TEST_STATUS=PASS"
    echo [OK] cargo_test.bat passed
)

if /I "%CHECK_STATUS%"=="PASS" if /I "%TEST_STATUS%"=="PASS" (
    set "MECH_VALIDATION_STATUS=passing"
    set "RUNTIME_VALIDATION_SUMMARY=controlled runtime path mechanically validated"
)

rem Intentional canonical health checks below:
rem these fixed paths are presence signals for core governance/architecture
rem anchors, not broad document discovery logic.
(
echo # Rust Sandbox Status Snapshot
echo.
echo - Workspace root: `%ROOT_DIR%`
echo - Generated at: `%DATE% %TIME%`
echo.
echo ---
echo.
echo ## Executive summary
echo.
echo - Product identity: DocGraph
echo - Assistant identity: Lume
echo - Identity scope: declarative docs/resources only; no runtime behavior changed
echo - 3B: closed operationally
echo - F4A-F8: CLOSED
echo - F8: CLOSED ^(knowledge panel plus governed document and tools surfaces remain established over a tabbed workspace shell^)
echo - F9-ready declarative resources: present
echo - F9.5 declarative/mock AI governance: completed
echo - validate_f9_declarations: %F9_DECLARATIVE_VALIDATION_STATUS%
echo - validate_ai_specs: %AI_SPECS_VALIDATION_STATUS%
echo - DocGraph/Lume declarative identity: present
echo - GUI_OBJECTS_v1: present
echo - Lume Help: contextual help only; no runtime, LLM, tools, mutation, or action approval
echo - Lume Help canonical GUI names: present
echo - ActionPolicy declarative resources: present
echo - FlowControlPolicy declarative resources: present
echo - PendingActionState: documented
echo - ActionResolution: documented
echo - full ActionRequest runtime: not implemented
echo - ActionResolution runner: not implemented
echo - SemanticProposal: proposal, not fact; approval requires future explicit human review
echo - Pipeline / Ontology View: readonly/mock presentation only
echo - Oxigraph/RDF/N-Quads: future preparation disabled; no RDF persisted, no SPARQL, no embeddings
echo - F10_PREP_GOVERNED_DOCUMENT_EDITING: documented as future proposal
echo - F10 minimal governed runtime: CONDITIONALLY CLOSED
echo - F10 broad runtime: not opened
echo - F10.1: minimally opened for pure llm_core capability-state resolution
echo - F10.2: minimally opened for assisted-chat envelope preparation
echo - F10.3: minimally opened for readonly EffectiveToolSurfaceSummary awareness
echo - F10.4: minimally opened for precomputed tool-surface summary injection into assisted-chat envelope
echo - F10.5: minimally opened for proposal-only ToolUseProposal modeling
echo - F10.6: minimally opened for inert ActionRequestDraft modeling
echo - F10.7: minimally opened for LlmInteractionTrace observability metadata
echo - F10.8: minimally opened for UI-safe LLM status view model
echo - provider invocation: closed
echo - tool execution: closed
echo - full ActionRequest runtime: not implemented
echo - ActionResolution runner: closed
echo - UI authority: presentation-only
echo - project_runtime authority: unchanged
echo - Governed document editing: proposal only; no runtime implementation opened and no crate changes required
echo - Real execution added: no
echo - Runtime type: governed project pipeline with thin consumers, tabbed workspace shell, readonly viewer tab, knowledge surface, document tree, structured chat references, and separated tools tabs
echo - Text derivation: regenerable text/page/chunk layer over primary documents
echo - Architectural stability: %ARCH_STABILITY%
echo - Mechanical validation: %MECH_VALIDATION_STATUS%
echo - Current mode: phased expansion above a closed F8 baseline
echo.
echo ---
echo.
echo ## Immediate next action
echo.
echo - keep the closed F8 narrative aligned across docs, snapshot, and invariants
echo - keep document text derivation reusable and regenerable
echo - keep the knowledge layer above existing project and workspace-tab boundaries
echo - keep tree, clip, chat, workspace, and viewer roles explicit and non-overlapping
echo - keep operational launch and LLM tool policy clearly separated
echo - keep the workspace content area tabbed without turning it into notebook or layout orchestration
echo - preserve `project_runtime` and `app_services` boundaries
echo - avoid introducing a second project pipeline or cross-layer mixing
echo - keep governed document editing as future proposal until a later phase opens implementation explicitly
echo.
echo ---
echo.
echo ## Maturity level
echo.
echo - stable core pipeline
echo - project vertical consumed by thin application and UI layers
echo - UI shell, controllers, workspace tabs, technical readonly viewer tab, minimal knowledge panel, document tree, structured chat references, and governed tools panel are present
echo - document text derivation and deterministic chunking groundwork are present
echo - F9.5 semantic proposal and AI governance layer is declared, validated, and readonly/mock
echo - GUI_OBJECTS_v1 canonical GUI glossary is present for Lume Help
echo - not production runtime
echo.
echo ---
echo.
echo ## System role ^(current^)
echo.
echo - open a project through a thin application service over the governed project vertical
echo - load and validate the manifest contract
echo - build a usable surface model
echo - resolve viewer targets safely under the project root
echo - expose minimal runtime observability for the controlled flow
echo - serve a structural UI shell with explicit controllers, a tabbed workspace content area, a technical readonly viewer tab, a minimal knowledge panel, a governed document tree, structured chat references, and separated tools tabs
echo - derive regenerable text, optional pages, and deterministic chunks from primary documents
echo.
echo ---
echo.
echo ## Mental model ^(1-line^)
echo.
echo - project_root -^> manifest -^> contract -^> validation -^> surface -^> resolve -^> output
echo.
echo ---
echo.
echo ## Project vertical boundaries
echo.
echo ### Includes
echo - project opening
echo - manifest loading and contract building
echo - validation, surface model, and viewer target resolution
echo - unified project pipeline and runtime observability
echo.
echo ### Excludes
echo - wide app_services
echo - full tool_runtime execution behavior
echo - full LLM integration
echo - production-grade UI workflow breadth
echo - new architectural layers
echo.
echo ---
echo.
echo ## Stable
echo.
echo - 3A closed
echo - 3B closed
echo - F4A closed
echo - F4B closed
echo - F5 closed
echo - F6 closed
echo - F7 closed
echo - F8 CLOSED
echo - F9-ready declarative resources present
echo - validate_f9_declarations: %F9_DECLARATIVE_VALIDATION_STATUS%
echo - DocGraph application identity and Lume assistant identity declared
echo - action and flow-control policies documented with execution disabled
echo - pending action state documented without runtime behavior
echo - ActionResolution documented as request-to-trace governance with execution disabled
echo - F9.5 AI governance and SemanticProposal schema validated with execution disabled
echo - Pipeline / Ontology View documented and presented as readonly/mock
echo - GUI_OBJECTS_v1 documented and available as shared Lume Help vocabulary
echo - Oxigraph/RDF/N-Quads preparation disabled; no semantic store, SPARQL, embeddings, or RDF persistence
echo - F10_PREP_GOVERNED_DOCUMENT_EDITING documented as future proposal
echo - %RUNTIME_VALIDATION_SUMMARY%
echo - project vertical ready for higher-layer consumption
echo - path policy stabilized in base crates
echo - mechanical validation wrappers available
echo.
echo ---
echo.
echo ## Current focus
echo.
echo - preserve the closed F8 workspace model without reopening it
echo - preserve readonly viewer-tab behavior and thin controller boundaries
echo - keep `project_runtime` closed as the governed vertical heart
echo - avoid mixing knowledge concerns into project or tool layers
echo - keep existing governed documents referenced from the tree rather than reimported through chat flows
echo - keep clip-driven intake explicit and workspace-driven
echo - keep `Operational Tools` execution separate from `LLM Tools` policy
echo - keep workspace tabs as controlled views rather than miniapps
echo - maintain handoff clarity and phase discipline
echo - keep patch runtime, active context, and structured asset inspectors documented but not implemented
echo - keep F9.5 semantic proposals as proposals, not facts
echo - keep Pipeline / Ontology View readonly/mock and non-authoritative
echo - keep Lume Help explanations tied to GUI_OBJECTS_v1 canonical names
echo.
echo ---
echo.
echo ## Current risks / watchpoints
echo.
echo - `project_runtime` growing beyond vertical responsibility
echo - `app_services` shadowing the governed project pipeline
echo - `ShellController`, workspace-tab routing, or viewer flow growing into coordination or semantics
echo - workspace tabs growing into notebook, docking, or layout orchestration too early
echo - knowledge concerns drifting into project, tool, or chat workflows too early
echo - chat drifting into a hidden blob store instead of structured references and results
echo - tree and clip responsibilities blurring into duplicate intake paths
echo - UI becoming the source of truth for LLM tool policy or tool definitions
echo - primary documents being treated as if derived text were the source of truth
echo - semantic proposals being mistaken for approved facts
echo - Pipeline / Ontology View drifting into semantic runtime or approval authority
echo - Oxigraph/RDF preparation being mistaken for an enabled store or export path
echo - reopening closed phases by pushing new concerns back into lower layers
echo - `ProjectRuntimeOutput` absorbing presentation or cross-layer concerns
echo - premature subsystem expansion
echo.
echo ---
echo.
echo ## Output contract discipline
echo.
echo - `ProjectRuntimeOutput` contains only project-vertical results
echo - no presentation data
echo - no cross-domain aggregation
echo - no future-layer concerns
echo.
echo ---
echo.
echo ## Expansion policy
echo.
echo - build above the governed project vertical
echo - no new parallel project pipeline
echo - thin consumers stay thin
echo - UI and knowledge layers consume existing boundaries rather than reinterpret them
echo - workspace tabs remain content containers, not a new runtime or domain layer
echo - chat stays reference-oriented; it does not become a second document store
echo - roadmap remains phased and sequential
echo - F9 and F10 remain sequential; governed document editing preparation does not open implementation
echo - F9.5 declarative/mock AI governance does not open F10
echo - no LLM execution, autonomous tools, embeddings, RDF persistence, SPARQL, or document mutation
echo.
echo ---
echo.
echo ## Planned next phases
echo.
echo - F4A-F8 - CLOSED
echo - F9 - preferences / credentials
echo - F9.5 - declarative/mock AI governance and semantic proposal preparation
echo - F10 - LLM chat integration with tools
echo - F11 - final audit / CLOSED state verification
echo - F10_PREP_GOVERNED_DOCUMENT_EDITING - documented proposal only
echo.
echo ---
echo.
echo ## Workspace summary
echo.
echo ### Crates present
for /d %%D in ("%ROOT_DIR%\crates\*") do echo - `%%~nxD`
echo.
echo ### Validation scripts present
for %%F in ("%ROOT_DIR%\dev\scripts\*.bat") do echo - `dev/scripts\%%~nxF`
echo.
echo ### Relevant root docs present
if exist "%ROOT_DIR%\README.md" echo - `README.md`
if exist "%ROOT_DIR%\governance/GOVERNANCE.md" echo - `governance/GOVERNANCE.md`
if exist "%ROOT_DIR%\architecture/ARCHITECTURE.md" echo - `architecture/ARCHITECTURE.md`
if exist "%ROOT_DIR%\architecture/MIGRATION_BASELINE.md" echo - `architecture/MIGRATION_BASELINE.md`
if exist "%ROOT_DIR%\governance/WORKSPACE_RULES.md" echo - `governance/WORKSPACE_RULES.md`
if exist "%ROOT_DIR%\docs\ENGINEERING_NOTES.md" echo - `docs/ENGINEERING_NOTES.md`
echo.
echo ---
echo.
echo ## Mechanical validation
echo.
echo - validate_f9_declarations: %F9_DECLARATIVE_VALIDATION_STATUS%
echo - validate_ai_specs: %AI_SPECS_VALIDATION_STATUS%
echo - cargo_check: %CHECK_STATUS%
echo - cargo_test: %TEST_STATUS%
echo.
echo ---
echo.
echo ## Consolidated capabilities
echo.
echo - end-to-end project runtime pipeline  
echo   ^(open -^> load -^> contract -^> validate -^> surface -^> resolve^)
echo - thin `app_services` consumer over the governed project pipeline
echo - `cli_app` as clean consumer of project and tool boundaries
echo - UI structural shell with explicit controllers, manifest wiring, and workspace tabs
echo - technical readonly viewer over resolved targets as a workspace tab
echo - minimal knowledge panel over project `knowledge/` documents
echo - governed document tree for existing workspace documents
echo - structured chat messages with document references, tool results, and system state
echo - explicit clip-driven document intake and workflow launch
echo - workspace document profiling over primary documents and derived text
echo - governed tools panel with controlled operational launch and declarative LLM tool policy
echo - `document_text_runtime` for regenerable text, pages, and chunk manifests
echo - minimal `tool_runtime` runner for typed accepted execution boundary
echo - `verify_progress` tooling available
echo - governed document editing proposal documented across governance and specs
echo - action policy, flow-control policy, and pending action state documented as F9-ready declarations
echo - ActionResolution documented without runner or execution authority
echo - AI governance resources and SemanticProposal schema validated as F9.5 declarations
echo - Pipeline / Ontology View readonly/mock presentation state available
echo - GUI_OBJECTS_v1 canonical GUI vocabulary available for Lume Help
echo.
echo ---
echo.
echo ## Current developer tooling
echo.
echo - cargo wrappers in `dev/scripts/`
echo - engineering notes
echo - scripts index
echo.
echo ---
echo.
echo ## Recent lessons learned
echo.
echo - base path policy belongs in `workspace_core`
echo - logical relative paths must stay portable
echo - critical runtime transitions must stay observable
echo - reusable project-opening policy belongs to the project vertical
echo.
echo ---
echo.
echo ## Notes usage
echo.
echo - full engineering notes included below for reference
echo - not required for immediate iteration
echo.
echo ---
echo.
echo ## Not guaranteed yet
echo.
echo - stable external API
echo - full production runtime behavior
echo - full LLM integration
echo - wide tool runtime behavior
echo - full product UI integration
echo - governed document patch runtime implementation
echo - direct document mutation from chat or LLM output
echo - action request runtime execution
echo - pending action execution behavior
echo - ActionResolution runner or enforcement runtime
echo - SemanticProposal approval runtime
echo - real LLM semantic derivation
echo - embeddings
echo - RDF persistence, N-Quads output, SPARQL, or Oxigraph runtime
) > "%SNAPSHOT_FILE%"

if not exist "%SNAPSHOT_FILE%" (
    echo [ERROR] Failed to write status snapshot.
    endlocal
    exit /b 1
)

(
echo.
echo ## Engineering notes ^(full^)
echo.
) >> "%SNAPSHOT_FILE%"

if exist "%ENGINEERING_NOTES_FILE%" (
    type "%ENGINEERING_NOTES_FILE%" >> "%SNAPSHOT_FILE%"
) else (
    (
    echo Engineering notes file was not found at `docs/ENGINEERING_NOTES.md`.
    ) >> "%SNAPSHOT_FILE%"
)

echo [OK] Status snapshot written to %SNAPSHOT_FILE%
endlocal
exit /b 0
