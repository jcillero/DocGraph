# external_dependency_manager

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed specification for `External Dependencies` management in DocGraph.

The manager governs declared external binaries recommended for the portable sandbox without implementing download, install, execution, or validation runtime in F9.

## Classification

`External Dependency Manager` is runtime infrastructure.

It is not:

- a normal user tool
- an `LLM Tool`
- an LLM-invocable capability
- a functional user action

It may expose visible catalog state to the user, but it remains infrastructure rather than user workflow execution.

## Core rule

External dependencies are declared, inspectable, and verifiable by contract.

They are not downloaded or installed by this specification.

UI may show status and governed buttons later, but UI must not own installation logic.

## Declarative catalog

Canonical catalog:

- `resources/tools/external/tools_external_catalog.json`

Each dependency entry must declare at least:

- `tool_id`
- `display_name`
- `purpose`
- `description_short`
- `description_long`
- `use_cases`
- `recommendation`
- `version`
- `source_url`
- `homepage_url`
- `license`
- `archive_type`
- `archive_sha256`
- `expected_binary_names`
- `install_dir`
- `validation_command`

These fields are declarative contract data.

They do not imply that the dependency is present, trusted, extracted, or executable.

The catalog may also include declarative user-facing fields such as:

- required internal tool or capability references
- expected local path
- download notes
- validation state
- trusted-by-user metadata for custom references

## Lifecycle

Prepared lifecycle state may include:

- `declared`
- `missing`
- `downloaded`
- `verified`
- `extracted`
- `binary_detected`
- `installed`
- `validated`
- `invalid`
- `removed`

Interpretation:

- `declared`: dependency exists in catalog only
- `missing`: dependency is not present in expected runtime layout
- `downloaded`: archive exists in staging in a future runtime phase
- `verified`: checksum has passed in a future runtime phase
- `extracted`: archive contents were extracted in a future runtime phase
- `binary_detected`: expected binary name was found
- `installed`: dependency is present in final runtime location
- `validated`: validation command has passed
- `invalid`: dependency failed integrity or layout checks
- `removed`: prior installed state was intentionally removed

## Layout

Current declarative placeholder layout:

- `runtime/temp/external_install/<tool_id>`
- `runtime/external/<tool_id>`

User-managed dependency root for the governed Tools menu model:

- `user/runtime/external_dependencies/`

Managed dependency location example:

- `user/runtime/external_dependencies/<dependency_id>/`

Referenced dependency locations may live outside the managed root, but DocGraph may only remove the reference, not physically delete those external files.

No downloaded or user-managed external binaries may live under:

- `resources/`
- `assets/`
- `crates/`
- project folders

The dependency must not execute from staging.

The dependency must not become executable merely because it exists under the managed user runtime root.

## Tools menu relationship

`Tools -> External Dependencies` is the governed inspection surface for external dependencies.

This surface is separate from:

- `Operational Tools`
- `LLM Tools`
- `Preferences`

There is no combined tools panel with tabs.

## Install / Configure flow

If a dependency is missing, the user-facing surface may expose:

- `Install / Configure`

This opens a governed popup that may show:

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

The system must not:

- download automatically
- execute installers
- modify `PATH`
- run binaries
- mutate system folders
- store secrets
- treat validation as execution authorization

Validation may only record declared metadata and future intended availability.

## Custom external dependency

The governed surface may allow:

- `Add custom external dependency`

This opens a popup where the user may declare:

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

- custom dependency remains declared by the user
- custom dependency is not automatically trusted by DocGraph
- custom dependency does not create a new internal tool
- custom dependency does not enable execution
- custom dependency may only become referenced by future internal tools through governed catalog or policy updates

## Remove / Delete model

Two governed cases exist.

### Managed dependency

A managed dependency under `user/runtime/external_dependencies/` may be deleted from the DocGraph-managed user runtime folder.

This requires explicit confirmation.

Deletion must update dependency status.

### Referenced dependency

A referenced external binary outside the DocGraph-managed runtime folder must not be physically deleted.

DocGraph may only remove the reference.

## Rules

- checksum is mandatory
- `source_url` may come only from catalog declaration
- URLs must not be hardcoded in code
- `assets/` must not be used as runtime
- execution from staging is forbidden
- UI may show buttons or state later, but not perform installation logic
- LLM must not install or execute external dependencies
- user-managed dependencies must use `user/runtime/external_dependencies/` when managed by DocGraph
- external dependency presence does not grant execution permission
- destructive deletion must require confirmation

Normative form:

- `INV-EXTDEP-001`: checksum metadata MUST be declared
- `INV-EXTDEP-002`: `source_url` MUST come only from catalog data
- `INV-EXTDEP-003`: runtime code MUST NOT hardcode dependency URLs
- `INV-EXTDEP-004`: external dependencies MUST NOT execute from staging layout
- `INV-EXTDEP-005`: `assets/` MUST NOT be used as runtime install location
- `INV-EXTDEP-006`: UI MUST remain representation-only for dependency state in F9
- `INV-EXTDEP-007`: LLM MUST NOT install or execute external dependencies
- `INV-EXTDEP-008`: managed user dependencies MUST use `user/runtime/external_dependencies/`
- `INV-EXTDEP-009`: dependencies outside the managed user runtime root MUST NOT be physically deleted
- `INV-EXTDEP-010`: validation MUST NOT imply execution permission

## Failure modes

- `checksum_mismatch`
- `download_failed`
- `unsupported_archive_type`
- `extraction_failed`
- `binary_not_found`
- `validation_failed`
- `manifest_missing`
- `invalid_install_dir`

Interpretation:

- `checksum_mismatch`: archive integrity did not match declared checksum
- `download_failed`: archive retrieval failed in a future runtime phase
- `unsupported_archive_type`: declared archive type is not supported
- `extraction_failed`: archive extraction failed
- `binary_not_found`: none of the expected binaries was found after extraction or install
- `validation_failed`: validation command did not pass
- `manifest_missing`: expected install manifest is absent
- `invalid_install_dir`: installed dependency layout is outside the governed runtime location

Additional governed failures may include:

- `reference_outside_managed_root`
- `managed_delete_requires_confirmation`
- `external_reference_delete_blocked`

## Future audits

Future audit surfaces may include:

- `audit_external_tools_catalog`
- `audit_runtime_external_layout`
- `audit_external_tool_manifests`

These remain future governed inspection or validation capabilities only.

## Relationship to catalogs

External dependencies are declared in tool catalogs as `external_dependency`.

This does not make them user tools or LLM tools.

The external catalog remains separate from:

- internal operational tools
- internal LLM tools
- runtime-consumed `tool_runtime` declarations

## Relationship to runtime resources

Portable runtime locations should remain declared under:

- `resources/runtime/runtime_locations.json`

That resource may declare governed roots such as:

- final external install root
- temporary staging root
- user-managed external dependency root

## Forbidden responsibilities

This spec must not:

- download binaries
- execute binaries
- validate with live runtime behavior
- move logic into UI
- open F10
- create a parallel tool runtime
- place dependencies inside `Preferences`

## Related specs

- `docs/specs/tools_catalogs.md`
- `docs/specs/tool_implementation_governance.md`
