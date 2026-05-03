# invalidation_policy

## Purpose

Define governed invalidation propagation rules across storage-backed evidence, derivatives, semantic quads, semantic relations, and graph material.

This document is declarative only.

It does not implement runtime invalidation engines, execution logic, mutation logic, or F10 execution slices.

## Scope

This policy governs invalidation triggered by:

- chunk changes
- file changes
- metadata changes

## Core triggers

### Chunk triggers

- `chunk_hash` change -> downstream semantic quad instances become `STALE`
- missing `chunk_id` -> downstream semantic quad instances become `ORPHANED`

### File triggers

- `file_ref` or effective source-content change -> derivative evidence may become stale
- missing governed source reference -> downstream semantic quad instances become `ORPHANED`

### Metadata triggers

- `metadata_snapshot_hash` mismatch -> downstream semantic quad instances become `STALE`
- missing required governed metadata evidence -> downstream semantic quad instances become `ORPHANED` or `STALE`, depending on whether identity of evidence is lost or only freshness is lost

## Propagation chain

Invalidation propagates downstream only:

```text
chunk -> quad -> relation -> graph
```

Expanded interpretation:

- chunk or source evidence change affects quad instances first
- quad invalidation affects dependent relations
- relation invalidation affects approved graph consumption

No reverse mutation is allowed.

Graph or relation invalidation must not mutate quads, derivatives, or storage.

## Invalidation modes

### Lazy invalidation

Current governed mode:

- invalidation is recorded conceptually when evidence mismatch is detected
- semantic and relation lifecycle reacts when evaluated or rechecked
- no eager runtime propagation engine is opened by this policy

### Eager invalidation

Future mode only:

- downstream invalidation may be propagated immediately by a future governed runtime
- eager propagation remains future and separately gated

## Layer effects

### Quad effects

- evidence mismatch affecting a quad instance changes lifecycle to `STALE`
- missing evidence anchor changes lifecycle to `ORPHANED`
- approved quads must not remain silently valid when evidence breaks

### Relation effects

- relation lifecycle must react to invalid source or target quad instances
- if a supporting quad becomes `STALE`, dependent relations may become `STALE`
- if a supporting quad becomes `ORPHANED`, dependent relations may become `ORPHANED` or `STALE`

### Graph effects

- invalid semantic relations must not remain approved graph dependencies
- graph consumption must exclude invalidated semantic material
- graph reacts; graph does not repair upstream layers

## Traceability

Every governed invalidation event must remain traceable at the policy level through:

- source evidence reference
- affected downstream entity reference
- invalidation reason
- invalidation mode
- trace reference

This policy does not define a runtime event format, only the requirement that invalidation be traceable.

## Invariants

- `INV-INVALIDATION-001`: approved quads must not remain valid if evidence breaks
- `INV-INVALIDATION-002`: invalidation must be traceable
- `INV-INVALIDATION-003`: invalidation propagates downstream only
- `INV-INVALIDATION-004`: `chunk_hash` change leads to `STALE`
- `INV-INVALIDATION-005`: missing chunk evidence leads to `ORPHANED`
- `INV-INVALIDATION-006`: graph and relations react to upstream invalidation and must not mutate upstream layers
- `INV-INVALIDATION-007`: lazy invalidation is the current declared mode
- `INV-INVALIDATION-008`: eager invalidation is future-only and not active

## Related specs

- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/artifact_graph.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/semantic_layer_boundaries.md`
