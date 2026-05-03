# artifact_graph

## Status

NORMATIVE DOCUMENTATION. This file defines the minimal artifact relationship layer for a DocGraph project. It does not implement a graph runtime.

## Purpose

Define `graph/` as a minimal relationship layer between governed project artifacts.

`graph/` explains relationships. It does not govern, execute, approve, infer, persist RDF, run SPARQL, activate Oxigraph, create embeddings, or replace `project_runtime`.

For semantic material, graph remains explanatory rather than authoritative:

- node = approved semantic assertion instance or other governed artifact node
- edge = explicit governed relation object
- graph does not approve facts
- graph does not execute logic
- graph does not infer new knowledge

Associated graph-edge schemas are declarative contracts only.

Schema validity does not activate graph runtime, RDF projection, graph analysis, or execution.

## Folder contract

```text
graph/
  graph_manifest.json
  artifact_nodes.jsonl
  artifact_edges.jsonl
  snapshots/
```

## Node example

```json
{
  "node_id": "doc_report_f110",
  "type": "document_holder",
  "path": "documents/report_f110"
}
```

## Edge example

```json
{
  "edge_id": "edge_001",
  "from": "doc_report_f110",
  "to": "chat_003",
  "relation": "uses_chat"
}
```

## Minimal relations

- `uses_chat`
- `uses_knowledge_file`
- `cites_source`
- `generated_from`
- `produced_tool_output`
- `derived_export`
- `references_artifact`
- `promotes_to_knowledge`
- `uses_template`
- `reviewed_in`

## Graph update policy

Graph updates MUST be triggered only by governed project actions.

Valid triggers include:

- document creation
- document update affecting references
- tool output registration
- chat-to-document promotion
- knowledge promotion

Graph updates MUST NOT be derived from passive filesystem scanning.

Graph updates MUST NOT infer relations without explicit governed actions.

## Semantic graph principle

For governed semantic relationships:

- node = `QuadInstance` or future approved semantic assertion instance
- edge = `QuadRelation`

Rules:

- relationships must not be embedded as hierarchy inside `SemanticQuad`
- relationships must not be implicit
- relationships must be explicit objects
- graph does not approve facts
- graph does not execute logic
- graph does not infer new knowledge

## QuadRelation model

Conceptual structure:

```json
{
  "relation_id": "rel_xxx",
  "source_quad_instance_id": "inst_A",
  "target_quad_instance_id": "inst_B",
  "relation_type": "supports | contradicts | refines | depends_on | derived_from | equivalent_to | broader_than | narrower_than | uses_as_evidence | supersedes",
  "lifecycle_state": "PROPOSED",
  "evidence": [],
  "metadata": {},
  "trace_ref": "trace_xxx"
}
```

Rules:

- `relation_id` is stable
- source and target must reference `QuadInstance`, not only `quad_id`
- relation lifecycle is independent from quad lifecycle
- relation metadata must not contain secrets
- relation evidence must be traceable

## Controlled relation vocabulary

Allowed `relation_type` values:

- `supports`
- `contradicts`
- `refines`
- `depends_on`
- `derived_from`
- `equivalent_to`
- `broader_than`
- `narrower_than`
- `uses_as_evidence`
- `supersedes`

Rules:

- `relation_type` must not be free text
- new relation types require governed extension
- relation labels are not runtime authority

## Relation lifecycle

Relation lifecycle aligns with semantic lifecycle:

- `PROPOSED`
- `UNDER_REVIEW`
- `APPROVED`
- `REJECTED`
- `STALE`
- `SUPERSEDED`
- `ORPHANED`

Rules:

- relations are `PROPOSED` by default
- `APPROVED` relation requires explicit future review
- invalid source or target instance makes the relation `ORPHANED` or `STALE`
- relation lifecycle is independent, but must remain consistent with source and target states
- no relation may be considered active if source or target is invalid, stale, rejected, orphaned, or superseded
- lazy invalidation is the current declared relation-reaction mode
- eager invalidation remains future-only and separately gated

Relation transition rules are governed by `semantic_quad_lifecycle.md`.

## Relation evidence

Relation evidence may reference:

- shared `file_ref`
- shared `object_ref`
- shared `chunk_id + chunk_hash`
- shared `source_ref`
- `reasoning_trace_ref`
- `review_trace_ref`
- future `tool_run_ref`

Rules:

- evidence references sources rather than duplicated full text
- evidence is not authority by itself
- evidence must not contain secrets
- evidence invalidation affects relation lifecycle

## Conflict handling

- `contradicts` models semantic conflict
- conflicting `PROPOSED` relations may coexist
- conflicting `APPROVED` relations require future governance review
- graph does not automatically resolve conflicts
- graph does not pick winners

## Supersession relationship

Difference:

- lifecycle `SUPERSEDED` marks state
- relation type `supersedes` explains semantic relationship

Rules:

- `supersedes` relation does not by itself mutate lifecycle
- transition to lifecycle `SUPERSEDED` must still be explicit and traceable
- both may refer to the same conceptual replacement while remaining separate mechanisms

## Future graph consumption

Future consumers may rely only on:

- `APPROVED` `QuadInstance` records
- `APPROVED` `QuadRelation` edges

Rules:

- `PROPOSED` relations must not be treated as dependencies
- graph remains explanatory, not authority

Boundaries:

- no graph runtime is opened
- no RDF projection runtime is opened
- no analysis runtime is opened
- no inference engine is opened
- graph reacts to upstream semantic invalidation and must not mutate semantic, derivative, or storage layers

Future RDF relation projection policy is governed by `rdf_projection_policy.md`.

Future graph analysis policy is governed by `graph_analysis_policy.md`.

## Invariants

- `INV-GRAPH-001`: `graph/` explains artifact relations; it is not runtime authority.
- `INV-GRAPH-002`: graph updates MUST originate from governed actions.
- `INV-GRAPH-003`: graph entries MUST be traceable to origin.
- `INV-GRAPH-004`: graph MUST NOT infer relations from passive filesystem scanning.
- `INV-GRAPH-005`: semantic proposals are not approved graph facts.
- The filesystem stores physical location.
- The manifest governs exposure.
- `registry.json` accelerates navigation.
- `graph/` explains relations.
- `graph/` does not decide, execute, approve, or mutate.
- `graph/` does not substitute `project_manifest.json`.
- `graph/` does not substitute `project_runtime`.
- Graph data MUST be consistent with project_manifest exposure.
- Graph MUST NOT introduce relations that are not traceable to governed actions.
- Graph entries MUST be traceable to an origin (document, chat, knowledge, or tool action).
- Artifact relations are traceability data, not runtime authority.
- Semantic relations remain proposals until human review.
- `knowledge/semantic/proposals/` is proposal storage, not an approved semantic store.
- No RDF, Oxigraph, SPARQL, embeddings, N-Quads output, or semantic store is introduced.
- No graph runtime is introduced.
- No tool execution is introduced.
- No UI logic is introduced.
- No project pipeline duplication is allowed.
- `INV-REL-001`: relations are explicit graph edges.
- `INV-REL-002`: relations must not be embedded inside `SemanticQuad` as hierarchy.
- `INV-REL-003`: relations reference `QuadInstance` identities.
- `INV-REL-004`: relations have independent lifecycle.
- `INV-REL-005`: relations are `PROPOSED` by default.
- `INV-REL-006`: `APPROVED` relation requires explicit review.
- `INV-REL-007`: graph does not approve facts.
- `INV-REL-008`: graph does not infer new knowledge.
- `INV-REL-009`: graph does not execute logic.
- `INV-REL-010`: future graph analysis may consume only `APPROVED` instances and `APPROVED` relations.
- `INV-REL-011`: conflict relations do not auto-resolve conflicts.
- `INV-REL-012`: relation evidence must be traceable and secret-free.
- `INV-REL-013`: graph invalidation reacts to upstream semantic changes and must not mutate upstream layers.

## Relationship to semantic proposals

F9.5 `SemanticProposal` records can reference artifacts and graph hints, but they remain proposals, not approved graph facts.

Future `SemanticQuad`, `QuadInstance`, and `QuadSet` materials may align with `graph/` only after explicit governed review.

- `ResolutionCandidate`-like semantic preparation is not graph authority
- `PROPOSED` semantic quads must not become graph facts
- `UNDER_REVIEW`, `REJECTED`, `STALE`, `SUPERSEDED`, and `ORPHANED` instances must not become graph facts
- future graph consumers may rely only on `APPROVED` semantic material
- graph relations must remain traceable to governed actions or future governed semantic approval, not to raw quad presence
- `PROPOSED`, `UNDER_REVIEW`, `REJECTED`, `STALE`, `SUPERSEDED`, and `ORPHANED` relations must not become approved dependencies or approved semantic facts

Future reviewed promotion may create or update governed artifact relations, but that future behavior must pass through the appropriate project, action, and human-review policies. This spec does not implement that behavior.

## Related specs

- `docs/specs/project_folder_layout.md`
- `docs/specs/semantic_proposal_layer.md`
- `docs/specs/project_runtime.md`
- `docs/specs/invalidation_policy.md`
