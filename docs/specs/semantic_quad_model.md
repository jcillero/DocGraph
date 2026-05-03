# semantic_quad_model

## Status

PROPOSAL / F9/F10_PREP declarative governance.

This document does not implement runtime generation, LLM invocation, tool execution, RDF, Oxigraph, SPARQL, N-Quads persistence, filesystem mutation, or approval runtime.

## Purpose

Define the governed semantic quad model for DocGraph as a proposal-only semantic layer.

This model must support:

- deterministic semantic identity through `quad_id`
- generation-instance identity through `quad_instance_id`
- explicit `source_kind` and `source_ref`
- alignment with `StoredObject`
- evidence anchoring
- contextual `QuadSet` grouping
- lifecycle preparation
- future RDF projection readiness without enabling RDF runtime now

Associated JSON schemas are declarative contracts only.

Schema validity does not imply approval, project exposure, execution, RDF projection, or graph-analysis runtime.

## Identity model

### quad_id

`quad_id` is deterministic semantic identity.

It represents what is asserted, not how or when it was produced.

`quad_id` is derived from normalized:

- `subject`
- `predicate`
- `object`
- `graph`
- evidence anchor

`quad_id` must not include generation context.

### quad_instance_id

`quad_instance_id` is generation-instance identity.

It represents how, when, or by what governed process the quad instance was produced.

It may include or depend on:

- `prompt_ref`
- `model_ref`
- `trace_ref`
- generation timestamp
- other future generation metadata

Multiple instances may share one `quad_id`.

Reuse of the same `quad_id` is preferred over creation of duplicate semantic identity when governed meaning and evidence anchor are unchanged.

## SemanticQuad structure

Conceptual structure:

```json
{
  "quad_id": "q_xxx",
  "subject": "...",
  "predicate": "...",
  "object": "...",
  "graph": "domain_knowledge | document_governance | ai_governance | system_governance"
}
```

Rules:

- `graph` is part of semantic identity
- quads are semantic proposals by default
- quads do not live inside chunks
- quads do not live inside deterministic derivatives
- a `SemanticQuad` expresses the asserted meaning only
- semantic relations must not be embedded as child hierarchy inside `SemanticQuad`
- semantic relations must be modeled as explicit graph edges outside the quad structure

## QuadInstance structure

Conceptual structure:

```json
{
  "quad_instance_id": "qi_xxx",
  "quad_id": "q_xxx",
  "source_kind": "file | chat | artifact | tool_output | metadata",
  "source_ref": "...",
  "evidence": [],
  "generation": {
    "prompt_ref": "...",
    "model_ref": "...",
    "trace_ref": "...",
    "timestamp": "..."
  },
  "lifecycle_state": "PROPOSED"
}
```

Rules:

- `lifecycle_state` defaults conceptually to `PROPOSED`
- `source_ref` must trace back to a `StoredObject` or another governed source reference
- lifecycle is not approval by generation
- generation metadata is trace, not authority
- a `QuadInstance` carries provenance and generation context, not semantic identity
- lifecycle attaches primarily to `QuadInstance`
- the same `quad_id` may have multiple instances with different lifecycle states

## source_kind

Allowed `source_kind` values:

- `file`
- `chat`
- `artifact`
- `tool_output`
- `metadata`

Alignment rules:

- `file` -> `StoredObject` with `content_ref`
- `chat` -> logical `StoredObject` or governed message refs
- `artifact` -> governed artifact or `DocumentHolder` reference
- `tool_output` -> `owner_ref`-backed tool output
- `metadata` -> semantic metadata snapshot derived from governed metadata

No `source_kind` implies execution authority.

## Evidence anchoring

Evidence records may include:

- `file_ref`
- `object_ref`
- `chunk_id`
- `chunk_hash`
- `text_range`
- `metadata_snapshot_hash`
- `message_ref`
- `artifact_ref`
- `tool_run_ref`

Rules:

- evidence should reference sources rather than duplicate full text
- evidence must be traceable
- evidence and semantic fields must not contain secrets or private absolute host paths
- `chunk_id + chunk_hash` anchor derivative evidence
- if `chunk_hash` changes, dependent quads become `STALE`
- if `chunk_id` disappears, dependent quads become `ORPHANED` or `STALE`

## QuadSet

`QuadSet` is a contextual grouping artifact.

Conceptual structure:

```json
{
  "quadset_id": "...",
  "source_kind": "...",
  "source_ref": "...",
  "generation_context": {},
  "quad_instances": []
}
```

Rules:

- quadsets are grouping artifacts, not approved knowledge
- multiple quadsets per source are allowed
- different prompts or models may produce different quadsets
- a future `semantic_quadset` `StoredObject` may represent the grouping artifact
- future retained semantic material should remain bounded by governed semantic storage limits
- reuse of an existing `QuadSet` is preferred when governed generation context is materially the same

## Relationship to StoredObject

`StoredObject` is the required logical alignment layer for persisted sources.

- `source_ref` should resolve to a governed `StoredObject` when the source is persisted
- `content_ref` and `object_ref` remain storage-governed, not semantically inferred
- semantic quads must not infer project exposure from `StoredObject`
- metadata-derived quads must remain distinct from the underlying metadata snapshot

## Lifecycle preparation boundary

Lifecycle states, allowed transitions, invalidation, and supersession are governed by `semantic_quad_lifecycle.md`.

This model aligns with:

- `PROPOSED`
- `UNDER_REVIEW`
- `APPROVED`
- `REJECTED`
- `STALE`
- `SUPERSEDED`
- `ORPHANED`

Boundaries:

- no review runtime is opened
- no approval runtime is opened
- `PROPOSED` is the default state
- `APPROVED` requires future explicit governed review
- future graph, RDF, or analysis consumers may consume only `APPROVED` quads

Future trigger, scope, and mode rules for quad generation are governed by `quad_generation_policy.md`.

Future semantic-retention bounds are governed by `semantic_storage_limits.md`.

Future reuse and caching rules are governed by `caching_and_reuse_policy.md`.

## Separation from deterministic derivatives

Deterministic derivatives live under derivative-governed outputs.

Semantic quad material is separate:

- quads are not deterministic derivatives
- chunks may be evidence anchors, not semantic containers
- derivatives may be regenerated without semantic mutation
- derivative regeneration may stale evidence without approving or deleting quads
- semantic material must not write back into derivatives or storage
- semantic invalidation reacts to upstream storage or derivative changes only
- semantic material must remain sanitized and must not re-expand redacted or masked upstream content

## Future RDF projection readiness

Semantic quads are future projection inputs.

However:

- RDF remains disabled
- N-Quads persistence remains disabled
- Oxigraph remains disabled
- SPARQL remains disabled

Future projection must preserve:

- lifecycle state
- source traceability
- generation traceability
- evidence anchoring

`PROPOSED` quads must not be projected as approved facts.

Future RDF projection policy is governed by `rdf_projection_policy.md`.

Future graph analysis policy is governed by `graph_analysis_policy.md`.

## Semantic Quad Invariants

- `INV-QUAD-001`: `quad_id` is deterministic semantic identity.
- `INV-QUAD-002`: `quad_instance_id` represents generation context.
- `INV-QUAD-003`: quads are proposals by default.
- `INV-QUAD-004`: quads are not approved facts without explicit review.
- `INV-QUAD-005`: evidence must be traceable.
- `INV-QUAD-006`: quads do not live inside chunks.
- `INV-QUAD-007`: quads do not live inside derivatives.
- `INV-QUAD-008`: `source_kind` is mandatory.
- `INV-QUAD-009`: quadsets are contextual, not authoritative.
- `INV-QUAD-010`: RDF, Oxigraph, and SPARQL remain disabled.
- `INV-QUAD-011`: generation metadata is trace, not authority.
- `INV-QUAD-012`: `StoredObject` alignment is required for persisted sources.
- `INV-QUAD-013`: same meaning plus same evidence anchor must resolve to the same `quad_id`.
- `INV-QUAD-014`: different generation context may yield different `quad_instance_id` values for the same `quad_id`.
- `INV-QUAD-015`: semantic identity and generation-instance identity must never be conflated.
- `INV-QUAD-016`: graph is part of semantic identity.
- `INV-QUAD-017`: semantic quad material must remain separate from deterministic derivative state.
- `INV-QUAD-018`: no quad may open execution, provider invocation, tool execution, filesystem mutation, or runtime approval.
- `INV-QUAD-019`: approved-knowledge eligibility depends on approved `QuadInstance` records, not on `quad_id` alone.
- `INV-QUAD-020`: semantic relations must not be embedded as quad hierarchy and must remain explicit edge objects.
- `INV-QUAD-021`: semantic writes must not mutate derivatives or storage.
- `INV-QUAD-022`: semantic material must not contain secrets, private absolute host paths, or unsanitized personal data.
- `INV-QUAD-023`: quad generation must remain trigger-bound, scope-bounded, and proposal-only unless a later governed runtime opens it.
- `INV-QUAD-024`: retained semantic material should remain bounded by governed semantic storage limits.
- `INV-QUAD-025`: reuse of existing semantic identity is preferred over duplicate semantic meaning when governed identity and context match.

## Related specs

- `docs/specs/semantic_source_scope.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/semantic_proposal_layer.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/storage_policy.md`
- `docs/specs/quad_generation_policy.md`
- `docs/specs/semantic_storage_limits.md`
- `docs/specs/caching_and_reuse_policy.md`
- `docs/specs/security_sanitization_policy.md`
- `docs/specs/system_observability.md`
