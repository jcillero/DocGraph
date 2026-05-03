# semantic_quad_lifecycle

## Status

PROPOSAL / F9/F10_PREP declarative governance.

This document does not implement review runtime, approval runtime, graph runtime, RDF runtime, Oxigraph, SPARQL, LLM execution, tool execution, filesystem mutation, or execution authority.

## Purpose

Define the governed lifecycle model for semantic quads and quad instances.

This lifecycle governs:

- proposal state
- review state
- approval state
- rejection
- evidence invalidation
- staleness
- orphaning
- supersession
- future graph, RDF, and analysis eligibility

Associated lifecycle schemas are declarative contracts only.

Schema validity does not approve quads or relations and does not activate lifecycle runtime.

## Relationship to semantic_quad_model

`semantic_quad_model.md` owns:

- `SemanticQuad`
- `QuadInstance`
- `quad_id`
- `quad_instance_id`
- source and evidence structure

This document owns lifecycle interpretation only.

Lifecycle ownership decision:

- lifecycle attaches primarily to `QuadInstance`
- `SemanticQuad` identity remains stable
- multiple `QuadInstance` records may share one `quad_id` with different lifecycle states

Approved knowledge eligibility depends on an approved `QuadInstance`, not on `quad_id` alone.

## Lifecycle states

Normalized states:

- `PROPOSED`
- `UNDER_REVIEW`
- `APPROVED`
- `REJECTED`
- `STALE`
- `SUPERSEDED`
- `ORPHANED`

### PROPOSED

- default state
- generated or declared but not validated
- not usable as approved knowledge

### UNDER_REVIEW

- being evaluated
- not yet approved
- not usable as graph or RDF fact

### APPROVED

- explicitly accepted by future governed review
- eligible for future graph, RDF, or analysis projection
- never automatic

### REJECTED

- evaluated and rejected
- must not be used as approved knowledge

### STALE

- evidence changed or became outdated
- requires review before reuse

### SUPERSEDED

- replaced by a newer or better quad instance
- not current approved knowledge

### ORPHANED

- evidence anchor no longer resolves
- requires repair, review, or rejection

## Allowed transitions

Allowed transitions:

- `PROPOSED -> UNDER_REVIEW`
- `PROPOSED -> REJECTED`
- `UNDER_REVIEW -> APPROVED`
- `UNDER_REVIEW -> REJECTED`
- `APPROVED -> STALE`
- `APPROVED -> SUPERSEDED`
- `STALE -> UNDER_REVIEW`
- `STALE -> REJECTED`
- `ORPHANED -> UNDER_REVIEW`
- `ORPHANED -> REJECTED`

Transition rules:

- `APPROVED` must never be reached automatically
- every transition must be explicit and traceable
- no silent lifecycle transitions are allowed
- generation does not imply approval
- lifecycle state must remain separate from generation metadata

## Evidence invalidation

Invalidation rules:

- `chunk_hash` mismatch -> `STALE`
- `chunk_id` missing -> `ORPHANED` or `STALE`
- `source_ref` missing -> `ORPHANED`
- `metadata_snapshot_hash` mismatch -> `STALE`
- artifact version mismatch -> `STALE`
- `tool_run_ref` missing -> `ORPHANED`

Invalidation rules mean:

- invalidated evidence must not remain silently `APPROVED`
- invalidation marks the quad instance for review
- invalidation does not auto-reject
- invalidation does not mutate source documents
- invalidation reacts to upstream storage or derivative changes and does not flow in reverse
- lazy invalidation is the current declared mode
- eager invalidation remains future-only and separately gated

## Supersession model

Supersession is explicit.

Conceptual fields:

- `supersedes: [quad_instance_id]`
- `superseded_by: [quad_instance_id]`
- `supersession_reason`
- `trace_ref`

Rules:

- supersession must be explicit
- old instance becomes `SUPERSEDED`
- new instance may be `PROPOSED` or `APPROVED` depending on future review
- supersession does not erase history

## Traceability

Every lifecycle transition must record:

- `actor_kind: human | system | future_governed_process`
- `actor_ref`
- `timestamp`
- `previous_state`
- `next_state`
- `reason`
- `trace_ref`

Rules:

- actor metadata must not contain secrets
- `trace_ref` must not imply execution
- lifecycle trace is audit metadata, not authority

## Future graph, RDF, and analysis eligibility

Future eligibility rules:

- graph may consume only `APPROVED` `QuadInstance` records
- graph may consume only `APPROVED` `QuadRelation` edges
- RDF projection may consume only `APPROVED` `QuadInstance` records
- RDF projection may consume only `APPROVED` `QuadRelation` edges
- graph analysis may consume only `APPROVED` `QuadInstance` records and approved relations
- `PROPOSED`, `UNDER_REVIEW`, `REJECTED`, `STALE`, `SUPERSEDED`, and `ORPHANED` are excluded from approved-knowledge projections

Boundaries:

- no graph runtime is opened
- no RDF projection runtime is opened
- no analysis runtime is opened

RDF projection filtering rules are further governed by `rdf_projection_policy.md`.

Graph analysis filtering rules are further governed by `graph_analysis_policy.md`.

## Lifecycle boundary

This document does not open:

- review runtime
- approval runtime
- graph runtime
- RDF projection runtime
- analysis runtime
- execution runtime

Lifecycle remains declarative governance only in `F9/F10_PREP`.

## Semantic Quad Lifecycle Invariants

- `INV-LC-001`: semantic quads are `PROPOSED` by default.
- `INV-LC-002`: `APPROVED` requires explicit governed review.
- `INV-LC-003`: generation never implies approval.
- `INV-LC-004`: lifecycle transitions must be traceable.
- `INV-LC-005`: evidence invalidation affects lifecycle.
- `INV-LC-006`: invalidated approved instances cannot remain silently approved.
- `INV-LC-007`: supersession must be explicit.
- `INV-LC-008`: lifecycle is separate from generation metadata.
- `INV-LC-009`: future graph, RDF, and analysis consume only `APPROVED` instances.
- `INV-LC-010`: no review runtime, approval runtime, RDF runtime, graph runtime, or execution runtime is opened.
- `INV-LC-011`: lifecycle attaches primarily to `QuadInstance`, not to `quad_id` alone.
- `INV-LC-012`: the same `quad_id` may have approved and non-approved instances simultaneously without collapsing lifecycle history.
- `INV-LC-013`: orphaned evidence must not be silently treated as valid support.
- `INV-LC-014`: stale evidence must not be silently treated as current approved knowledge.
- `INV-LC-015`: future approved semantic relation edges must follow the same non-automatic review discipline as approved quad instances.
- `INV-LC-016`: semantic lifecycle invalidation reacts to upstream layers and must not mutate them.

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_source_scope.md`
- `docs/specs/semantic_proposal_layer.md`
- `docs/specs/artifact_graph.md`
- `docs/specs/storage_policy.md`
- `docs/specs/invalidation_policy.md`
