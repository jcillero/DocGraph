# tools_panel

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed tools-management surfaces for DocGraph without opening orchestration, runtime execution, or installer behavior.

`Tools` is a top-level menu domain.

It is not part of `Preferences`.

## Core rule

There is no single combined tools panel with tabs.

The `Tools` menu exposes exactly three governed entries:

- `Operational Tools`
- `LLM Tools`
- `External Dependencies`

Each entry opens its own panel or popup surface.

Each surface emits intent only.

No surface executes tools, installs dependencies, calls LLM, or interprets policy as authority.

## Governed i18n keys

The `Tools` menu contract uses these stable menu keys:

- `menu.tools`
- `menu.tools.operational`
- `menu.tools.llm`
- `menu.tools.external_dependencies`

The governed Tools surfaces use these popup keys:

- `popup.tools.operational.title`
- `popup.tools.operational.description`
- `popup.tools.llm.title`
- `popup.tools.llm.description`
- `popup.tools.external_dependencies.title`
- `popup.tools.external_dependencies.description`

Popup descriptions must remain informational only.

They must not imply execution, installation, dependency readiness, or LLM availability.

Legacy menu keys may remain in resources for compatibility, but new specs must not reference:

- `menu.tools.open_panel`
- `menu.tools.external`

## Operational Tools panel contract

`Operational Tools` is the governed inspection and preparation surface for internal DocGraph operational tools.

Declaration source:

- `resources/tools/internal/operational/tools_operational_document_catalog.json`

The panel may show:

- tool list
- tool id
- display name
- description
- status
- execution availability
- dependencies, if any
- selected tool inspector

The panel may allow:

- selecting one declared operational tool
- opening `Prepare execution`
- inspecting tool input and output contract
- inspecting owner and persistence requirements

The panel must not:

- execute on menu click
- execute on tool selection
- execute on `Prepare execution`
- chain tools
- create workflow graphs
- act as `LLM Tools` surface

## Operational Tools prepare-execution popup

`Prepare execution` opens a governed popup.

The popup captures structured intent only.

It may capture:

- inputs
- configuration
- ordered input policy, when relevant
- `owner_ref`
- output destination
- risk summary
- confirmation intent

The popup emits preparation intent only.

It must not execute.

Any future execution must pass through `ActionRequest -> ActionResolution -> authorized executor -> trace`.

## LLM Tools panel contract

`LLM Tools` is the governed inspection surface for internal LLM tool declarations.

Declaration source:

- `resources/tools/internal/llm/tools_llm_document_catalog.json`

The panel may show:

- declared LLM tools
- visibility and status
- future availability state
- capability limits
- reason codes, when applicable
- prepared `LLMCapabilityState` summary
- prepared `EffectiveToolSurface` state
- available, disabled, missing, and explanation states

The panel must not:

- show `Run`
- show `Prepare execution`
- manually execute tools
- bypass `EffectiveToolSurfaceResolver`
- inject raw full tool catalogs into LLM context
- compute permissions
- decide effective LLM mode
- call providers
- validate credential secrets

The surface is valid even when the current LLM tools catalog is empty.

## Capability state consumption

The Tools Panel consumes prepared state only.

It may display:

- `desired_llm_mode`
- `effective_llm_mode`
- `interaction_mode`
- `llm_available`
- `tool_surface_available`
- `execution_enabled`
- `reason_codes`

It must not derive these values from raw catalogs, UI state, credential fields, or filesystem presence.

Tool visibility does not imply execution permission.

If `execution_enabled=false`, the panel may explain `execution_not_open`, but must not offer execution controls.

## External Dependencies panel contract

`External Dependencies` is the governed inspection surface for external binaries or software required by internal tools.

External dependencies are not tools by themselves.

Declaration source:

- `resources/tools/external/tools_external_catalog.json`

User-managed dependency root:

- `user/runtime/external_dependencies/`

The panel may show:

- dependency list
- which internal tool or capability may require the dependency
- installed, missing, or invalid status
- expected binary name
- expected local path
- source URL
- download notes
- validation state

The panel must not:

- execute dependencies
- install automatically
- modify `PATH`
- run installers
- delete external binaries outside DocGraph-managed runtime folders

## Install / Configure popup

If an external dependency is missing, the panel may expose `Install / Configure`.

This opens a governed popup.

The popup may show:

- dependency name
- description
- why DocGraph may need it
- recommended source URL
- recommended version or version range
- expected binary name
- expected destination under `user/runtime/external_dependencies/`
- manual steps for the user
- button to select an existing downloaded binary or folder
- button to validate reference

The popup must not:

- download automatically
- execute installers
- mutate system folders
- store secrets
- treat validation as execution authorization

Validation records declared metadata and intended availability only.

## Custom external dependency flow

The `External Dependencies` surface may expose:

- `Add custom external dependency`

This opens a governed popup where the user may declare:

- dependency id
- display name
- source URL
- download notes
- expected binary name
- expected version, optional
- local path under `user/runtime/external_dependencies/` or selected reference
- `trusted_by_user`
- reason or use note

Rules:

- custom dependency remains user-declared
- custom dependency is not automatically trusted by DocGraph
- custom dependency does not create a new internal tool
- custom dependency does not enable execution
- custom dependency may only become referenced by future internal tools through governed catalog or policy updates

## Remove / Delete model

Two governed cases exist.

### Managed dependency

A managed dependency under `user/runtime/external_dependencies/` may expose:

- `Delete dependency`

It requires explicit confirmation.

The deletion updates dependency status only.

### Referenced dependency

A referenced dependency outside the DocGraph-managed runtime folder may expose:

- `Remove reference`

It must not physically delete the external binary or folder.

Removing the reference does not delete the internal tool declaration.

If a required dependency becomes unavailable, the internal tool becomes `unavailable_missing_dependency`.

## Invariants

- `INV-TOOLS-PANEL-001`: there MUST NOT be a single combined tools panel with tabs.
- `INV-TOOLS-PANEL-002`: `Operational Tools`, `LLM Tools`, and `External Dependencies` MUST open separate governed surfaces.
- `INV-TOOLS-OP-001`: operational tools are internal DocGraph tools.
- `INV-TOOLS-OP-002`: operational tools MAY be manually prepared but MUST NOT execute directly from menu click.
- `INV-TOOLS-OP-003`: operational-tool execution preparation MUST use a governed popup.
- `INV-TOOLS-LLM-001`: LLM tools MUST NOT be manually executable.
- `INV-TOOLS-LLM-002`: LLM tools expose governed policy or surface only.
- `INV-TOOLS-EXT-001`: external dependencies are not tools.
- `INV-TOOLS-EXT-002`: external dependencies are binaries or software required by internal tools.
- `INV-TOOLS-EXT-003`: user-managed external dependencies MUST use `user/runtime/external_dependencies/` as the governed user runtime root.
- `INV-TOOLS-EXT-004`: downloaded external dependencies MUST NOT live under `resources/`, `assets/`, `crates/`, or project folders.
- `INV-TOOLS-EXT-005`: external dependency installation MUST NOT be automatic in F9.
- `INV-TOOLS-EXT-006`: external dependency validation MUST NOT imply execution permission.
- `INV-TOOLS-EXT-007`: managed dependencies may be deleted only from DocGraph-managed user runtime folders.
- `INV-TOOLS-EXT-008`: referenced external binaries outside DocGraph runtime folders MUST NOT be physically deleted.
- `INV-TOOLS-ACTION-001`: all future tool execution MUST go through `ActionResolution`.
- `INV-TOOLS-ACTION-002`: UI panels and popups MUST emit intent only; they MUST NOT execute.
- `INV-TOOLS-I18N-001`: visible Tools labels and governed popup text MUST come from i18n resources.
- `INV-TOOLS-CAP-001`: Tools Panel MUST consume prepared capability and EffectiveToolSurface state only.
- `INV-TOOLS-CAP-002`: Tools Panel MUST NOT compute permissions, effective LLM mode, or execution availability from UI state.
- `INV-TOOLS-CAP-003`: Tool visibility MUST NOT imply execution permission.
- `INV-TOOLS-CAP-004`: raw tool catalogs MUST NOT be injected into LLM context by the Tools Panel.
- `INV-TOOLS-CAP-005`: LLM tool intent MUST become proposal only and MUST NOT execute directly.

## Out of scope

- runtime execution
- filesystem mutation beyond future governed runtime authority
- real download
- installer execution
- provider calls
- LLM calls
- LLM mode resolution
- credential secret validation
- automatic dependency validation
- `PATH` modification
- tool runner expansion
- UI business logic

## Related specs

- `docs/specs/app_main_menu.md`
- `docs/specs/tools_catalogs.md`
- `docs/specs/tool_implementation_governance.md`
- `docs/specs/how_to_add_a_tool.md`
- `docs/specs/external_dependency_manager.md`
- `docs/specs/action_resolution.md`
