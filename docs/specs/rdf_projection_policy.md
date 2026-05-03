# rdf_projection_policy

## Status

PROPOSAL / F9/F10_PREP declarative governance.

This document does not implement RDF runtime, Oxigraph, SPARQL, N-Quads persistence, TriG persistence, RDF file generation, graph analysis runtime, or execution authority.

## Purpose

Define RDF as a future projection layer over the internal governed semantic graph.

The internal source model remains:

- `StoredObject`
- `SemanticQuad`
- `QuadInstance`
- `QuadRelation`
- lifecycle state
- evidence
- trace

RDF is explicitly non-authoritative.

## RDF is projection, not authority

Conceptual direction:

`Internal governed semantic model -> future RDF projection`

Rules:

- RDF does not replace `StoredObject`
- RDF does not replace `SemanticQuad` or `QuadInstance`
- RDF does not replace `QuadRelation`
- RDF does not govern lifecycle
- RDF does not approve facts
- RDF is never the source of truth

## Eligibility rules

Only these may be projected in the future:

- `APPROVED` `QuadInstance`
- `APPROVED` `QuadRelation`

These must be excluded:

- `PROPOSED`
- `UNDER_REVIEW`
- `REJECTED`
- `STALE`
- `SUPERSEDED`
- `ORPHANED`

Rules:

- `PROPOSED` material must never be projected as factual RDF
- lifecycle filtering must not be bypassed
- approval must remain explicit and traceable

Lifecycle ownership and filtering rules remain governed by `semantic_quad_lifecycle.md`.

## Named graph mapping

Future named graph policy:

- `domain_knowledge`
- `document_governance`
- `ai_governance`
- `system_governance`

Rules:

- named graphs separate semantic facts, provenance, AI trace, and system governance
- graph identity is part of semantic identity
- lifecycle and provenance data belong in governance graphs
- no secrets may appear in any named graph

## RDF identity mapping

Future conceptual mapping:

- `quad_id -> stable semantic IRI`
- `quad_instance_id -> provenance_or_review IRI`
- `relation_id -> relation IRI`
- `object_ref -> source object IRI`
- `file_ref -> content-addressed source IRI`

Rules:

- no raw absolute host paths in RDF
- no secrets in IRIs
- identifiers must be stable and portable
- RDF identity mapping is declarative only

## Quad projection

Conceptual projection:

`SemanticQuad(subject, predicate, object, graph) -> RDF quad`

```text
<subject> <predicate> <object> <graph> .
```

Rules:

- projection may occur only from an `APPROVED` `QuadInstance`
- projection must preserve linkage to `quad_id` and `quad_instance_id`
- projection must preserve source and evidence linkage through governance graphs
- raw evidence text should not be duplicated

## Relation projection

`QuadRelation` becomes a future RDF relationship triple or quad.

Example conceptual mapping:

```text
<quad_instance_A> <docgraph:supports> <quad_instance_B> <graph:domain_knowledge> .
```

Rules:

- `relation_type` maps to a governed RDF predicate
- only `APPROVED` `QuadRelation` may be projected
- relation lifecycle must be preserved in governance graph
- `contradicts` does not auto-resolve conflict

## Evidence and provenance mapping

Future provenance links may reference:

- `source_ref`
- `object_ref`
- `file_ref`
- `chunk_id`
- `chunk_hash`
- `metadata_snapshot_hash`
- `message_ref`
- `artifact_ref`
- `tool_run_ref`
- `trace_ref`

Rules:

- evidence is referenced, not duplicated as raw text
- provenance graphs must preserve enough metadata for audit
- no secret-bearing metadata may be projected
- sanitized inputs must remain sanitized in projection and must not be re-expanded

## Dataset boundary

Future dataset boundary:

- each DocGraph project may have its own future RDF dataset
- global RDF aggregation remains future-only
- RDF dataset does not replace `project_manifest`
- RDF dataset does not infer project exposure

## Future formats

Future projection targets may include:

- `N-Quads`
- `TriG`

But:

- no files are generated now
- no RDF store is created now
- no Oxigraph dependency is enabled now
- no SPARQL endpoint or query runtime exists now

Future graph analysis may consume projected RDF only as a derived input surface. RDF still remains projection, not authority.

## RDF Projection Invariants

- `INV-RDF-001`: RDF is projection, not authority.
- `INV-RDF-002`: only `APPROVED` `QuadInstance` records may be projected.
- `INV-RDF-003`: only `APPROVED` `QuadRelation` records may be projected.
- `INV-RDF-004`: lifecycle filtering must not be bypassed.
- `INV-RDF-005`: RDF must preserve traceability.
- `INV-RDF-006`: named graphs separate concerns.
- `INV-RDF-007`: RDF must not contain secrets.
- `INV-RDF-008`: RDF must not contain raw absolute host paths.
- `INV-RDF-009`: RDF datasets do not replace `project_manifest`.
- `INV-RDF-010`: Oxigraph, SPARQL, and N-Quads persistence remain inactive.
- `INV-RDF-011`: RDF projection does not imply graph analysis runtime.
- `INV-RDF-012`: RDF projection does not approve or infer knowledge.
- `INV-RDF-013`: RDF projection must preserve sanitization boundaries and must not expose private absolute host paths, secret values, or unsanitized personal data.

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/artifact_graph.md`
- `docs/specs/storage_policy.md`
- `docs/specs/security_sanitization_policy.md`
