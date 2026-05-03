# caching_and_reuse_policy

## Status

DECLARED_ONLY / F9-F10_PREP governance.

This document does not implement cache runtime, semantic-runtime reuse logic, derivative cache runtime, lifecycle mutation, or execution authority.

## Purpose

Define governed reuse and caching rules for semantic quads and derived data.

The goal is to avoid redundant generation, duplicate semantic meaning, and unnecessary regeneration while preserving lifecycle, traceability, and storage boundaries.

## Reuse rules

Future governed reuse should follow these principles:

- reuse quads when the same `quad_id` already exists
- reuse `QuadSet` groupings when the same governed generation context already exists
- reuse deterministic derivatives when equivalent governed source content is unchanged

Rules:

- reuse is preferred over regeneration when identity and governed context match
- reuse must not bypass lifecycle state
- reuse must not convert proposal material into approved knowledge
- reuse must remain bounded by source scope, storage policy, and semantic storage limits

## Semantic deduplication

Identical `quad_id` values must not duplicate semantic meaning.

Rules:

- one semantic meaning should map to one `quad_id`
- multiple `QuadInstance` records may exist for the same `quad_id`
- multiple instances for the same `quad_id` must remain explicitly linked through the shared identity
- duplicate meaning must be represented through reuse of identity rather than creation of a new semantic meaning id

Lifecycle and approval still attach to `QuadInstance`, not to cached identity alone.

## QuadSet reuse

`QuadSet` reuse is allowed when the governed generation context is materially the same.

Conceptually this includes reuse when:

- source scope matches
- generation trigger matches
- generation mode matches
- relevant evidence anchor state matches
- relevant governed generation context matches

If governed generation context differs materially, a new `QuadSet` may be appropriate even when some quads are reused.

## Derivative reuse

Deterministic derivative reuse is preferred when:

- source content hash matches
- governed derivative inputs are unchanged
- derivative contract remains equivalent

Derivative reuse must not:

- bypass invalidation policy
- bypass security and sanitization policy
- be treated as semantic approval

## Conceptual cache model

Future governed caching may use:

- content-hash-based reuse
- generation-context-based reuse

### content-hash-based reuse

Content-hash-based reuse applies when governed source content or evidence anchors are unchanged and deterministic reuse is safe.

### generation-context-based reuse

Generation-context-based reuse applies when the same governed semantic-generation context is repeated and reuse is safer than regeneration.

This remains conceptual only in the current phase.

## Boundary rules

Caching and reuse must not:

- bypass lifecycle filtering
- bypass invalidation
- bypass security and sanitization
- infer project exposure
- mutate storage authority
- imply RDF projection
- imply graph-analysis runtime

Reuse may preserve efficiency, but it does not widen authority.

## Non-goals

This policy does not open:

- runtime caching
- runtime cache persistence
- automatic semantic reuse logic
- runtime deduplication engine
- execution slices

## Caching and reuse invariants

- `INV-REUSE-001`: duplicate semantic meaning should reuse the same `quad_id` rather than creating a new semantic identity.
- `INV-REUSE-002`: reuse is preferred over regeneration when governed identity and context match.
- `INV-REUSE-003`: multiple `QuadInstance` records for the same `quad_id` are allowed but must remain explicitly linked.
- `INV-REUSE-004`: caching and reuse must not bypass lifecycle state or approval boundaries.
- `INV-REUSE-005`: caching and reuse must not bypass invalidation policy.
- `INV-REUSE-006`: caching and reuse must not bypass security and sanitization policy.
- `INV-REUSE-007`: derivative reuse must remain content-based and deterministic.
- `INV-REUSE-008`: governed cache concepts remain declarative only until a later runtime phase explicitly opens them.

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/storage_policy.md`
- `docs/specs/quad_generation_policy.md`
- `docs/specs/semantic_storage_limits.md`
- `docs/specs/invalidation_policy.md`
- `docs/specs/security_sanitization_policy.md`
