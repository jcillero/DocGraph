# quad_generation_policy

## Status

DECLARED_ONLY / F9-F10_PREP governance.

This document does not implement runtime quad generation, LLM invocation, tool execution, filesystem mutation, semantic approval, RDF projection runtime, or graph-analysis runtime.

## Purpose

Define a governed policy for when and how semantic quads may be generated in the future.

The goal is to prevent uncontrolled semantic explosion while preserving traceability, storage discipline, and security boundaries.

Quad generation is optional and trigger-bound.

It is not mandatory for all governed sources.

## Allowed generation triggers

Allowed triggers are:

- `user_requested`
- `artifact_analysis_requested`
- `chat_analysis_requested`
- `tool_output_analysis_requested`
- `system_suggested` ^(future only^)

Rules:

- quad generation must not occur automatically without an explicit governed trigger
- `system_suggested` remains future-only and does not activate generation by itself
- trigger identity must remain traceable in the resulting `QuadInstance` or equivalent future generation trace

## Generation scope

Generation scope must be bounded explicitly.

Allowed bounded scopes include:

- per `StoredObject`
- per `source_kind`
- per session
- per request

Rules:

- generation scope must be declared before future generation begins
- scope must not silently widen from one object to unrelated sources
- scope must not imply whole-project semantic expansion by default
- `project_manifest` exposure and governed source references remain authoritative for eligible sources

## Generation modes

Conceptual future generation modes are:

- `minimal`
- `balanced`
- `exhaustive` ^(future, restricted^)

### minimal

`minimal` means only key facts and trace-critical relations should be proposed.

This mode is the safest future default if generation is later opened.

### balanced

`balanced` means a broader but still governed semantic proposal set that stays within bounded scope and explicit trigger.

### exhaustive

`exhaustive` is future-only and restricted.

It must not be assumed available by default and must not be opened implicitly by this policy.

## Generation boundaries

Generation must respect all existing layer and governance boundaries:

- storage remains root authority
- derivatives remain deterministic and non-semantic
- semantic generation must not mutate storage or derivatives
- semantic generation must not imply approval
- graph and RDF remain downstream consumers only

Generation must also respect:

- invalidation policy
- security and sanitization policy
- source-scope governance
- lifecycle governance

## Traceability

Every future generation event must remain traceable at minimum through:

- trigger kind
- governed source reference
- generation scope
- generation mode
- resulting `quad_id` / `quad_instance_id` linkage
- trace reference

Traceability does not imply approval or execution authority.

## Non-goals

This policy does not open:

- runtime generation
- automatic background semantic extraction
- LLM execution
- tool execution
- semantic approval
- RDF projection runtime
- graph-analysis runtime
- execution slices

## Quad generation invariants

- `INV-GEN-001`: quad generation must not occur without an explicit governed trigger.
- `INV-GEN-002`: quad generation is optional and must not be treated as mandatory for all sources.
- `INV-GEN-003`: generation scope must remain bounded per governed object, source kind, session, or request.
- `INV-GEN-004`: uncontrolled or silent whole-project quad generation is forbidden.
- `INV-GEN-005`: generation must be traceable.
- `INV-GEN-006`: generation must respect storage-layer and derivative-layer boundaries.
- `INV-GEN-007`: generation must respect security and sanitization policy.
- `INV-GEN-008`: generated quads remain proposals by default and do not become approved knowledge automatically.
- `INV-GEN-009`: `system_suggested` generation remains future-only and non-active unless a later governed runtime opens it.
- `INV-GEN-010`: exhaustive generation remains future-only and restricted.

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/semantic_source_scope.md`
- `docs/specs/storage_policy.md`
- `docs/specs/invalidation_policy.md`
- `docs/specs/security_sanitization_policy.md`
