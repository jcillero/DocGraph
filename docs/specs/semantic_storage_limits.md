# semantic_storage_limits

## Status

DECLARED_ONLY / F9-F10_PREP governance.

This document does not implement runtime enforcement, automatic pruning, storage compaction, RDF projection runtime, graph-analysis runtime, or execution authority.

## Purpose

Define governed limits for semantic storage so the semantic layer remains bounded and does not grow without control.

These limits are conceptual and future-configurable.

They exist to constrain semantic growth across quads, quadsets, and relations without opening runtime behavior.

## Conceptual limit types

The semantic layer should support, in a future governed runtime, at minimum these conceptual limits:

- `max_quads_per_object`
- `max_quadsets_per_source`
- `max_relations_per_quad`

### max_quads_per_object

`max_quads_per_object` bounds how many semantic quads or quad instances may be retained for a governed source object.

This limit applies conceptually per `StoredObject` or equivalent governed source reference.

### max_quadsets_per_source

`max_quadsets_per_source` bounds how many contextual `QuadSet` groupings may be retained for the same source.

This prevents repeated re-analysis from creating unbounded contextual groupings.

### max_relations_per_quad

`max_relations_per_quad` bounds how many semantic relations may attach to a given approved or proposed semantic assertion instance.

This prevents relation density from growing without meaningful control.

## Policy levels

Semantic storage limits are governed through two conceptual policy levels:

- soft limits
- hard limits ^(future enforcement^)

### soft limits

Soft limits are advisory thresholds.

When crossed in a future runtime, they should produce warnings, review signals, or generation-scope narrowing guidance.

Soft limits do not imply deletion or automatic pruning by themselves.

### hard limits

Hard limits are future-only enforcement thresholds.

They are not active in the current phase.

When a later governed runtime opens them, they may block additional semantic material from being retained beyond the configured boundary.

## Pruning strategies

Future pruning or reduction strategies may include:

- `oldest_first`
- `lowest_confidence_first`
- `least_referenced_first`

These strategies are conceptual only in this phase.

They do not activate deletion, mutation, or enforcement now.

### oldest_first

Prefer removing or superseding the oldest retained semantic material first when newer governed material is more relevant.

### lowest_confidence_first

Prefer removing or superseding semantic material with the lowest future confidence score first when confidence metadata exists.

### least_referenced_first

Prefer removing or superseding semantic material with the weakest graph or trace usage first when governed reference counts exist.

## Boundary rules

Semantic storage limits must remain aligned with:

- storage as root authority
- semantic generation remaining optional and trigger-bound
- invalidation remaining traceable and downstream-only
- security and sanitization remaining mandatory

Limits must not:

- mutate physical storage authority
- imply approval
- bypass lifecycle
- bypass security policy
- infer project exposure

## Future configurability

Limits must be configurable in a future governed runtime.

Configuration remains future-only and inactive now.

The current phase declares only the existence of governed limit concepts, not their concrete numeric values.

## Non-goals

This policy does not open:

- automatic enforcement
- automatic pruning
- runtime generation
- runtime approval
- graph-analysis runtime
- RDF runtime
- execution slices

## Semantic storage limit invariants

- `INV-LIMIT-001`: the semantic layer must remain bounded.
- `INV-LIMIT-002`: semantic storage must not grow unbounded by default.
- `INV-LIMIT-003`: semantic limits must remain configurable in a future governed runtime.
- `INV-LIMIT-004`: soft limits are advisory only in principle and do not imply deletion.
- `INV-LIMIT-005`: hard limits remain future-only until explicitly opened.
- `INV-LIMIT-006`: pruning strategies are conceptual only in this phase.
- `INV-LIMIT-007`: storage-layer authority must not be mutated by semantic limit policy.
- `INV-LIMIT-008`: semantic limits must remain aligned with lifecycle, invalidation, and security policy.

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/storage_policy.md`
- `docs/specs/quad_generation_policy.md`
- `docs/specs/invalidation_policy.md`
- `docs/specs/security_sanitization_policy.md`
