# semantic_proposal_layer

## Status

PROPOSAL / F9.5 declarative governance. This document does not implement LLM execution, embeddings, RDF persistence, Oxigraph, SPARQL, tools, document mutation, or runtime approval.

## Purpose

Define `SemanticProposal` as the governed representation for future AI-derived semantic suggestions.

A proposal is not a fact.

## Core model

Future semantic derivations may produce proposal-shaped records with:

- `proposal_id`
- `subject_ref`
- `predicate`
- `object_ref`
- `graph_hint`
- `source_ref`
- `prompt_ref`
- `semantic_spec_ref`
- `trace_ref`
- `confidence_label = mock_only`
- `review_state`
- `created_by`
- `execution_enabled = false`

In F9.5 these records are model/schema only.

## Review lifecycle

Allowed states:

- `Generated`
- `NeedsReview`
- `Approved`
- `Rejected`
- `Stale`
- `Expired`

Default state: `NeedsReview`.

No proposal may become `Approved` without explicit human review in a future governed flow.

## Future graph hints

Future named graph hints:

- `domain_knowledge`
- `system_governance`
- `ai_governance`
- `document_governance`
- `lume_policy`

Graph hints do not create RDF, persist triples, or activate Oxigraph.

## Disabled future flags

- `oxigraph_enabled = false`
- `rdf_export_enabled = false`
- `semantic_store_enabled = false`
- `execution_enabled = false`

## Lume role

If Lume appears in future graph data, it must be represented as `InteractionLayer`.

Lume must not be represented as `Executor`, `Runtime`, `ToolRunner`, `LLMProvider`, or `FilesystemOwner`.

## Invariants

- `INV-SEM-001`: proposal is not fact. Every semantic output generated in F9.5 must be represented as `SemanticProposal`; it is not approved knowledge.
- `INV-SEM-002`: no automatic approval. No semantic proposal may move to `Approved` without explicit human decision.
- `INV-SEM-003`: every node must be referenceable through a stable identifier.
- `INV-SEM-004`: every proposed relation must preserve document/chunk origin, `prompt_ref`, `semantic_spec_ref`, and `trace_ref`.
- `INV-SEM-005`: future export does not imply persistence. `FutureGraphRef`, `graph_hint`, or N-Quads do not create RDF or activate Oxigraph.
- `INV-SEM-006`: Oxigraph is future, not current runtime. In F9.5 `oxigraph_enabled`, `rdf_export_enabled`, and `semantic_store_enabled` are false.
- `INV-SEM-007`: the graph does not decide, approve, execute, or mutate actions.
- `INV-SEM-008`: ontology proposes context, not authority. It does not replace policy, `ActionResolution`, human review, or authorized runtime.
- `INV-SEM-009`: named graph separation distinguishes `domain_knowledge`, `system_governance`, `ai_governance`, `document_governance`, and `lume_policy`.
- `INV-SEM-010`: governance is also knowledge, but it remains declarative.
- `INV-SEM-011`: Lume is not an executor. In graph terms Lume is `InteractionLayer`, never `Executor`, `Runtime`, `ToolRunner`, `LLMProvider`, or `FilesystemOwner`.
- `INV-SEM-012`: every `SemanticProposal` must have lifecycle state.
- `INV-SEM-013`: there is no hidden UI semantics. UI represents proposals, relations, and traces; it does not infer, approve, or create facts.
- `INV-SEM-014`: the semantic layer does not duplicate the project pipeline. It consumes governed documents, chunks, and references without reinterpreting manifests or paths.
- `INV-SEM-015`: prepare RDF/Oxigraph without premature implementation: no Oxigraph dependency, store, SPARQL, embeddings, or LLM.

## Related specs

- `docs/specs/ai_governance_f9_5.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/action_resolution.md`
- `docs/specs/pending_action_state.md`

