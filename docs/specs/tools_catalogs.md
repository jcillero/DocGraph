# tools_catalogs

## Purpose

Define the F9-ready declarative catalog layout for internal operational tools, internal LLM tools, external runtime dependencies, and the proposal-only master capability catalog.

Catalogs declare availability and policy shape. They do not implement execution.

Catalogs are functional domains.

They are not menu surfaces, not UI authority, and not physical runtime locations.

## Current canonical source

`resources/tool_runtime/* is the current operative canonical source` for the existing `tool_runtime` crate.

The F8/F9 sandbox still treats:

- `resources/tool_runtime/meta_catalog.json`
- `resources/tool_runtime/llm_tool_policy.json`

as the runtime-consumed declarations.

`resources/tools/* is not consumed by runtime in the current phase`.

## Scope F9

F9 declares:

- `resources/tools/tools_metacatalog.json`
- `resources/tools/tools_master_catalog.json`, proposal only
- operational document tools catalog
- empty LLM document tools catalog
- external tools catalog containing only `tectonic`

This layout is F9-ready and declarative only.

The metacatalog indexes catalogs, not concrete execution logic.

Tool implementation governance is defined in `docs/specs/tool_implementation_governance.md`.

The practical declaration workflow is summarized in:

- `docs/specs/how_to_add_a_tool.md`

Catalogs are source declarations.

Catalogs may declare tool identity and possible capabilities, but catalog presence alone does not authorize any capability.

An LLM-facing tool surface is derived and policy-filtered.

The full raw catalog is not injected into LLM context by default.

Operational tools may declare `document_profile` as enabled and requiring confirmation, but no new execution is implemented by this documentation update.

Operational tools may also declare future non-executable entries such as `merge_pdfs` with `execution_enabled=false`. Such entries are F9-ready declarations only and do not create runtime behavior.

LLM tools remain empty.

External tools declare Tectonic as optional and missing.

Declared does not mean installed, validated, or executable.

External tools are not executable when:

- `execution_implemented` is `false`
- runtime status is `missing`
- the declared binary is not present
- the future runtime dependency validation has not passed

`enabled=true` in declarative catalogs does not imply executable behavior when `execution_enabled=false` or `execution_implemented=false`.

Legacy or minimal tool declarations must progressively declare `status`, canonical `tool_kind`, schemas, and execution flags before becoming executable.

## Tools menu domain

`Tools` is a top-level menu domain separate from `Preferences`.

It exposes exactly:

- `Operational Tools`
- `LLM Tools`
- `External Dependencies`

These menu entries do not redefine catalogs.

They route into governed inspection or preparation surfaces only.

There is no single combined tools panel with tabs.

Catalog classification remains independent from menu placement.

## F9 master catalog proposal

F9 may also declare:

- `resources/tools/tools_master_catalog.json`

This file is a proposal-only declarative master catalog for tool identity and capability mapping.

In F9:

- master catalog = declarative master source
- current `tool_runtime` = operative runtime source
- `EffectiveToolSurface` is derived separately
- visibility is derived by policy
- `implementation_ref` is indirect and non-executing

The master catalog does not replace current per-domain catalogs in F9.

The master catalog does not replace:

- `resources/tools/tools_metacatalog.json`
- operational, LLM, or external catalogs
- `resources/tool_runtime/*`

Its purpose is to unify tool identity without opening execution or changing current consumers.

The master catalog should use functional domains, not surfaces or physical placement, for `catalog` values.

Current proposed master-catalog domains are:

- `core_base`
- `document_ops`
- `semantic_ops`
- `external_runtime`

Do not use master-catalog domain values such as:

- `llm`
- `operational`
- `internal`

## Master catalog shape

Each master entry should declare at least:

- `tool_id`
- `capability_id`
- `catalog`
- `tool_kind`
- `surfaces`
- `status`
- `declaration`
- `visibility`
- `implementation_ref`
- `execution_enabled`
- `proposal_only`

Future capability modeling is governed separately by `docs/specs/tool_capability_model.md`.

Interpretation:

- `declaration`: canonical tool identity, semantic role, and stable descriptive contract
- `tool_kind`: one of `operational`, `llm`, `external`, `agent`, or `base`
- `surfaces`: declarative surface hints only; not effective surface authority
- `visibility`: policy-derived exposure hints; not effective visibility truth
- `implementation_ref`: indirect reference to operative or future implementation boundary; not execution authority

Canonical `tool_kind` meanings:

- `operational`: deterministic system action that may become executable only in a later governed phase
- `llm`: capability available to LLM reasoning surfaces; not manually executable in F9
- `external`: external binary or runtime dependency; not a tool by itself
- `agent`: composite tool-like provider shape; future-only unless explicitly opened
- `base`: internal support capability; not user-facing

`tool_kind` defines classification only.

It does not define execution.

Tool visibility does not imply execution permission.

LLM tools are not agent roles.

Legacy `tool_class` is deprecated as an alias only.

If historical declarations still contain `tool_class`, they should map as follows:

- `operational` -> `operational`
- `llm` -> `llm`
- `external_dependency` -> `external`
- `base_utility` -> `base`

Agent roles are governed through `tool_kind=agent`, not through legacy `tool_class`.

`capability_id` must be unique across the master catalog.

One capability must not be duplicated across operational, LLM, external, or support surfaces as separate semantic truths.

`execution_enabled` must remain `false` for F9 and F10_PREP declarative master-catalog entries.

`proposal_only` must remain `true` for F9 and F10_PREP declarative master-catalog entries.

`json.validate` may be declared only as `base` and should remain `hidden_support`, not a public user tool.

Do not declare `json_read` as a public master-catalog capability.

`external_runtime` entries do not install or execute binaries in F9.

`external.tectonic` is a runtime-dependency identity only. It does not open installation or execution behavior.

## Minimal F9/F10_PREP subset

The minimal proposal subset for `tools_master_catalog.json` may include 8 to 10 capabilities:

- `document.profile`
- `document.extract_text`
- `document.merge_pdfs`
- `document.summarize`
- `text.measure`
- `file.diff`
- `json.validate`
- `semantic.extract_concepts`
- `semantic.propose_relations`
- `external.tectonic`

This subset is sufficient to establish master identity and capability coverage without opening runtime execution.

The F9 subset is frozen.

New capabilities require an explicit phase or governance decision.

Do not add tools for convenience.

Do not duplicate capabilities by surface.

## Governance block

The master catalog may contain an internal `governance` block.

That block should declare rules such as:

- execution disabled
- no runtime authority
- no raw catalog injection into LLM context by default
- binaries in `resources/` forbidden
- UI non-executing
- indirect implementation resolution only

The governance block is declarative metadata only.

It does not create runtime policy by itself.

## Coexistence rule

Do not merge the catalogs in F9.

Do not move files between `resources/tool_runtime/*` and `resources/tools/*`.

Do not make the UI or runtime consume `resources/tools/*` until F10 explicitly opens a controlled transition.

Do not make the master catalog the operative runtime source in F9.

## Catalogs vs surfaces vs storage

Functional catalogs remain:

- `core_base`
- `document_ops`
- `semantic_ops`
- `external_runtime`

These values classify capability domain only.

They do not mean:

- top-level menu category
- execution surface
- runtime path
- permission

Surface remains derived separately.

Visibility remains policy-derived and non-authoritative.

Physical storage remains governed by runtime layout and external dependency governance, not by catalog domain.

## External dependency storage boundary

`external_runtime` entries remain declaration-only capability or dependency identities.

They do not install binaries and do not execute binaries.

User-managed downloaded or referenced external dependencies belong under:

- `user/runtime/external_dependencies/`

This user runtime root does not make the dependency executable.

It does not replace the current operative `resources/tool_runtime/*` source.

It does not authorize `tool_runtime` to consume `resources/tools/*`.

## LLM catalog note

`resources/tools/internal/llm/tools_llm_document_catalog.json` may remain empty in current F9 declarations.

An empty LLM catalog is compatible with F9.

The `LLM Tools` panel may therefore show an empty declared surface without opening execution or widening master-catalog authority.

## Master catalog and effective surface boundary

### MasterCatalog

`resources/tools/tools_master_catalog.json` is the declarative source of tool capabilities.

It:

- does not execute
- does not authorize
- does not resolve availability
- does not replace `resources/tool_runtime/*` in F9

### EffectiveToolSurfaceResolver

`EffectiveToolSurfaceResolver` is a future mandatory component before any operative consumption of master-catalog declarations.

It must derive `EffectiveToolSurface` from:

- `MasterCatalog`
- phase policy
- project policy
- global tool policy
- implementation availability
- external dependency availability
- credentials or capability state, when applicable

It must not live in UI.

It must not be the LLM.

It must not bypass `ActionResolution`.

### EffectiveToolSurface

`EffectiveToolSurface` is derived output only.

It may contain only effective capabilities and states such as:

- `declared`
- `visible`
- `allowed`
- `implemented`
- `available`
- `executable`

`visible` is not permission.

`surfaces` is not execution availability.

`executable` may become `true` only in a later explicitly opened phase.

Effective surface is not the raw catalog.

Future structured surface details are governed by `docs/specs/llm_tool_surface_resolution.md`.

### Explicit prohibitions

- `tool_runtime` MUST NOT consume `resources/tools/tools_master_catalog.json` directly in F9.
- UI MUST NOT derive tool permissions from `resources/tools/tools_master_catalog.json`.
- LLM MUST NOT receive the raw full master catalog by default.
- `visibility` MUST NOT be interpreted as permission.
- `surfaces` MUST NOT be interpreted as execution availability.
- `external_runtime` MUST NOT imply binary installation or execution.
- `tool_id` MUST NOT be interpreted as execution authority by itself.

## F10 step-1 closure

The first explicit F10 opening gate does not make tool catalogs executable.

In F10 step 1:

- tool catalogs remain declarative sources
- `tools_master_catalog.json` remains non-operative as direct runtime input
- only bounded `EffectiveToolSurfaceSummary` runtime consumption may open
- tool execution remains closed
- provider binding is not implied by catalog presence

## F11.0 execution gate note

`F11.0` is DECLARED / NOT ACTIVE.

Catalog declaration does not imply execution.

Executable eligibility must be resolved by future governed policy.

Raw catalog presence is not authority.

## F12.0 / F11.RUNTIME-0 catalog proposal note

`F12.0 / F11.RUNTIME-0` is proposal-only and does not make any catalog executable.

The first future runtime proposal may consider only one deterministic local tool whose `tool_kind` is `operational` or `base`.

Recommended first tool: `text.measure`.

Catalog entries for `merge_pdfs`, external tools, LLM tools, and agent tools remain non-executable until a later explicit phase opens them.

Catalog declaration, `enabled=true`, visibility, or implementation metadata must not be used by UI, LLM, or runtime to infer execution authority.

## F12.1 text.measure catalog gate

`F12.1` is gate-only and does not make any catalog executable.

The only tool that may be considered for the first later implementation slice is:

- `text.measure`

Catalog eligibility does not imply execution.

`text.measure` may proceed to a later implementation proposal only if its future runtime slice remains one local deterministic `SANDBOX`-only tool with mandatory `owner_ref`, mandatory `trace_ref`, mandatory `ToolRunManifest`, and mandatory `TraceRecord`.

`merge_pdfs`, external tools, LLM tools, and agent tools remain outside the first gate.

### Future transition gate

Any connection between `tools_master_catalog.json` and `tool_runtime` requires the explicit future phase:

- `F10_PREP_EFFECTIVE_TOOL_SURFACE`

That phase must introduce its own tests and validators.

Until that phase opens explicitly, `resources/tool_runtime/*` remains the current operative source.

## Future roadmap note: F10_PREP_EFFECTIVE_TOOL_SURFACE

`F10_PREP_EFFECTIVE_TOOL_SURFACE` is a future proposal only.

Objective:

- derive `EffectiveToolSurface` from declarative capability identity and governed policies without making the master catalog an operative source by itself

Inputs:

- `MasterCatalog`
- phase policy
- project policy
- global tool policy
- implementation availability
- external dependency availability
- credentials or capability state, when applicable

Output:

- `EffectiveToolSurface`

Prohibitions:

- no UI authority
- no LLM authority
- no direct `MasterCatalog -> tool_runtime`
- no execution without a later explicit phase

## Forbidden responsibilities

Tool catalogs must not:

- execute tools
- call LLMs
- download binaries
- add Pandoc, Poppler, Tesseract, LibreOffice, SevenZip, ripgrep, or jq yet
- bypass `tool_runtime`
- duplicate project pipeline behavior
- store binaries in `resources/`
- collapse declaration, effective surface, visibility policy, and implementation authority into one field

## Future F10/F11 notes

F10 may migrate `tool_runtime` toward the `resources/tools/*` layout through a controlled transition.

That transition should:

- preserve current `tool_runtime` behavior until replacement is validated
- avoid duplicate effective-policy truth
- keep operational, LLM, and external runtime dependency concepts separate
- treat legacy `resources/tool_runtime/*` as fallback or migration input only after an explicit decision

F11 should verify enabled states, confirmation requirements, external binary validation, and policy separation.

## F12.4 file intake catalog alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

Future intake tools may be declared later as narrow `operational` or `base` tools only.

`F12.4` does not add, implement, activate, or execute intake tools.

Catalog presence must not imply file intake runtime availability.

## F9 Invariants (Non-Negotiable)

- The master catalog MUST remain declarative and MUST NOT be executable in F9.
- The master catalog MUST NOT be consumed by `tool_runtime` in F9.
- The master catalog MUST NOT define `EffectiveToolSurface` directly.
- `execution_enabled=false` MUST remain mandatory and binding for every master-catalog entry in F9.
- `proposal_only=true` MUST remain mandatory and MUST prevent any execution in F9.
- No tool declared in the master catalog MAY execute in F9.
- No `external` entry declared in the master catalog MAY install or execute in F9.
- LLM capabilities declared in the master catalog MUST remain proposal-only and MUST NOT activate real pipelines in F9.
- UI surfaces MUST NOT interpret, authorize, or execute tools from the master catalog in F9.
- `project_runtime` MUST NOT depend on the master catalog in F9.
- `tool_kind` is the canonical governed classification term.
- Only the defined `tool_kind` values are valid: `operational`, `llm`, `external`, `agent`, `base`.
- `tool_kind` MUST be explicit for every governed tool declaration.
- No tool declaration MAY rely on implicit or inferred `tool_kind`.
- Legacy `tool_class` may remain only as a deprecated alias in historical declarations and must not override `tool_kind`.
- No new `tool_kind` value may be introduced without an explicit governance update.
- Violation of any invariant implies unintended transition to F10.
- The first F10 opening gate MUST NOT convert catalog visibility, declaration, or summary presence into execution authority.
