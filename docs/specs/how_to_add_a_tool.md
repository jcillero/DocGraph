# how_to_add_a_tool

## Status

PROCEDURAL GOVERNANCE GUIDE.

SPEC-ONLY.

No runtime implementation.

## Purpose

Provide a reproducible, low-friction process for adding a new tool in DocGraph without opening F10, bypassing `ActionResolution`, or mixing declaration, visibility, permission, and execution.

Core rule:

`declared != executable`

Tool state progression:

`DECLARED -> VISIBLE -> PERMITTED -> IMPLEMENTED -> AVAILABLE -> EXECUTABLE`

Do not collapse these states.

## Before you start

Read first:

- `docs/specs/tool_implementation_governance.md`
- `docs/specs/tools_catalogs.md`
- `docs/specs/tools_panel.md`
- `docs/specs/action_resolution.md`

This guide does not replace those specs.

## 1. Tool classification

Choose exactly one `tool_class`:

- `operational`
- `llm`
- `external_dependency`
- `base_utility`

### Decision tree

1. Does it perform a DocGraph technical action over governed inputs or outputs?
   - yes -> `operational`
   - no -> continue
2. Is it a future LLM-facing capability surface rather than a manual user tool?
   - yes -> `llm`
   - no -> continue
3. Is it a binary, package, or external software required by a tool?
   - yes -> `external_dependency`
   - no -> stop and reclassify before editing anything

### Classification notes

- `operational` tools are internal DocGraph tools.
- `llm` tools are not manually executable in F9.
- `external_dependency` entries are not tools by themselves.
- `base_utility` entries are internal support capabilities and are not user-facing.

## 2. Phase 1 - Declaration

Choose the correct declaration location:

- `resources/tools/internal/operational/`
- `resources/tools/internal/llm/`
- `resources/tools/external/`

Checklist:

- [ ] `tool_id` defined
- [ ] `capability_id` defined
- [ ] description complete
- [ ] `status = declared_only`
- [ ] `execution_enabled = false`
- [ ] no implementation binding
- [ ] correct `tool_class`
- [ ] correct catalog reference
- [ ] visible text uses declarative i18n fields

Required reminder:

This does NOT make the tool executable.

## 3. Phase 2 - Surface / visibility

After declaration, place the tool in the correct governed surface:

- `Operational Tools`
- `LLM Tools`
- `External Dependencies`

Visibility is not availability.

A visible tool may still be non-implemented, non-permitted, or non-executable.

Checklist:

- [ ] appears in the correct panel
- [ ] surface matches `tool_class`
- [ ] no execution button unless explicitly governed by current phase
- [ ] no UI-triggered execution
- [ ] no menu click implies execution

## 4. Phase 3 - Policy / permission

Policy and visibility are separate concerns.

Future effective state is conceptually derived by `EffectiveToolSurfaceResolver`.

That resolver is not opened as runtime authority in F9.

Checklist:

- [ ] tool may be visible but not usable
- [ ] visibility is not permission
- [ ] permission is not execution
- [ ] no direct execution authority is introduced
- [ ] no UI surface decides permission
- [ ] no LLM surface decides permission

## 5. Phase 4 - Minimal runtime (only if applicable)

Touch `tool_runtime` only if the current phase explicitly permits a minimal governed slice.

Most tools in F9 do NOT reach this phase.

If a minimal slice is allowed, keep it minimal.

Checklist:

- [ ] behavior is deterministic
- [ ] no hidden side effects
- [ ] no hidden IO
- [ ] no external calls unless already declared and phase-permitted
- [ ] no runtime widening outside the minimal slice
- [ ] no F10 drift

## 6. Phase 5 - Output governance

If the tool persists outputs, govern the output first.

Required concepts:

- `owner_ref`
- `result.json`
- `tool_run_manifest.json`

Checklist:

- [ ] outputs stored under `user/output/tool_runs/`
- [ ] `owner_ref` present
- [ ] `tool_run_manifest.json` present
- [ ] `result.json` present for completed runs
- [ ] no orphan outputs
- [ ] no root `outputs/`

## 7. Phase 6 - UI interaction

UI prepares intent only.

UI does not execute.

Use popup-driven preparation when governed.

Checklist:

- [ ] UI does NOT execute
- [ ] UI captures intent only
- [ ] popup includes inputs
- [ ] popup includes config
- [ ] popup includes `owner_ref`
- [ ] popup includes destination
- [ ] popup does not bypass `ActionResolution`

## 8. Phase 7 - LLM integration (if applicable)

LLM tools are not manually executable.

The LLM does not receive the full raw tool catalog by default.

Checklist:

- [ ] tool appears in the LLM surface only if declared
- [ ] no bypass of policy
- [ ] no automatic execution
- [ ] no manual `Run`
- [ ] no raw full catalog injection by default

## 9. Phase 8 - External dependencies (if applicable)

Dependencies are not tools.

Checklist:

- [ ] declared in the external catalog
- [ ] stored or referenced under `user/runtime/external_dependencies/` when applicable
- [ ] no auto install
- [ ] no `PATH` mutation
- [ ] validation does not imply execution
- [ ] dependency presence does not imply tool availability

## 10. Phase 9 - Audits

Required validations:

- `dev\scripts\validate_f9_declarations.bat`
- `dev\scripts\validate_ai_specs.bat`

Optional manual-only audit:

- `dev\scripts\audits\audit_f9_boundary_drift.bat`
- `dev\scripts\audits\audit_tools_compliance.bat`

Checklist:

- [ ] required validations pass
- [ ] optional boundary audit reviewed manually when relevant

## 11. Anti-patterns

Do not:

- must not let UI execute tools
- let LLM trigger execution
- hardcode paths
- place tools in `Preferences`
- store external binaries in `resources/`
- skip `owner_ref`
- skip `tool_run_manifest.json`
- mix declaration, permission, and execution into one field
- mix tools and dependencies
- bypass `ActionResolution`

## 12. Quick checklist

- [ ] declared
- [ ] not executable
- [ ] correct catalog
- [ ] correct surface
- [ ] no UI execution
- [ ] no LLM execution
- [ ] no runtime drift
- [ ] audits pass

## 13. Invariants

- declared != executable
- UI != executor
- LLM != executor
- tools != dependencies
- execution only via governed path

## Related specs

- `docs/specs/tool_implementation_governance.md`
- `docs/specs/tools_catalogs.md`
- `docs/specs/tools_panel.md`
- `docs/specs/action_resolution.md`
