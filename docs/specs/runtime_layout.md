# runtime_layout

## Purpose

Document the preliminary runtime layout for future external tools, LLM engines, models, cache, and temporary data.

This layout is declarative and materialized as empty or placeholder folders only.

## Scope F9

F9 may declare these runtime roots:

- `runtime/external`
- `runtime/engines`
- `runtime/models`
- `runtime/cache`
- `runtime/temp`
- `user/runtime/external_dependencies`

The minimal external runtime placeholder is Tectonic under `runtime/external/tectonic/`.

Tectonic is not installed. No binary is downloaded or faked.

`runtime/external` remains the current declarative placeholder root used by existing F9 validation and placeholder manifests.

`user/runtime/external_dependencies/` is the governed user-managed location for future external dependency references in the Tools menu model.

Declaring that user-managed location does not:

- install dependencies
- execute binaries
- replace `resources/tool_runtime/*`
- open F10

## Forbidden responsibilities

The runtime layout must not:

- execute external binaries
- install dependencies
- create tool calling
- run LLM engines
- store user project truth
- replace `resources/` declarations
- duplicate `tool_runtime`
- treat user-managed dependency presence as execution authority

## Future F10/F11 notes

F10 may resolve declared engines, models, and external tools through governed policy.

F11 should audit checksums, install status, and explicit execution gates before any real runtime execution is accepted.
