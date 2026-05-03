# document_template_resolution

## Status

PROPOSAL / F9-ready declarative governance. This document defines future template resolution rules only. It does not implement runtime, document creation, export, LLM calls, tool execution, or filesystem mutation.

## Purpose

Define how a future governed document creation flow may select a template before creating a Markdown `ARTIFACT` package.

Template resolution is a declarative selection contract, not a new project pipeline.

## Resolution precedence

Future resolution should use this precedence:

1. `project`
2. `user`
3. `resources`

The `resources` layer is the built-in fallback and may be indexed by `resources/document_templates/templates_catalog.json`.

If the same `template_id` exists in multiple layers, precedence determines the selected template. The resolved origin must remain traceable.

## Conceptual inputs

- `template_id`
- optional `template_version`
- optional document kind
- optional conformity mode
- project-local template declarations, future
- user-local template declarations, future
- resource template catalog

No absolute host paths may be required.

## Conceptual result

A future `TemplateResolutionResult` may contain:

- resolved `template_id`
- resolved `template_version`
- source layer: `project`, `user`, or `resources`
- ownership: `project`, `user`, or `system`
- relative template resource path
- default conformity mode
- supported conformity modes
- selected operation capability flags, if supplied by policy
- shadowed lower-precedence template references, when applicable
- warning list
- resolution fingerprint or catalog version

This result is not execution.

## Conflict handling

Template id conflicts are resolved by layer precedence:

- `project` shadows `user`
- `user` shadows `resources`
- `resources` is the fallback

Resolution must not erase the existence of shadowed templates. It should preserve trace data so a future UI or audit report can explain why a template was selected.

Conflicts inside the same source layer are invalid and should produce `duplicate_template_id`.

## Snapshot rule

When a future document package is created, the selected template must be copied into the package as `template_snapshot.json`.

The snapshot is preserved locally and is not destructively modified.

## Failure classes

- `missing_template`
- `ambiguous_template_id`
- `disabled_template`
- `unsupported_conformity_mode`
- `incompatible_template_kind`
- `stale_catalog`
- `duplicate_template_id`
- `forbidden_template_operation`

## Forbidden responsibilities

Template resolution must not:

- create a document package
- write files
- mutate templates
- invoke LLMs
- execute tools
- run export
- reinterpret project manifests
- duplicate `project_runtime`
- depend on UI state as document truth

## Related specs

- `docs/specs/document_templates.md`
- `docs/specs/document_template_validation.md`
- `docs/specs/document_creation_flow.md`
- `docs/specs/document_package.md`
