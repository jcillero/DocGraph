# graph_analysis_policy

## Status

PROPOSAL / F9/F10_PREP declarative governance.

This document does not implement runtime analysis, path computation, graph engines, inference engines, SPARQL, Oxigraph, execution logic, or lifecycle mutation.

## Purpose

Define the governed policy for future semantic graph analysis in DocGraph.

This policy is the owner of future graph-analysis rules over governed semantic graph material.

Analysis may consume only:

- `APPROVED` `QuadInstance`
- `APPROVED` `QuadRelation`

Analysis results are decision-support artifacts, not authoritative truth.

Associated graph-analysis schemas are declarative contracts only.

Schema validity does not enable analysis runtime, inference runtime, RDF projection, or execution.

## Analysis boundary

Graph analysis is future-only and declarative in this phase.

Rules:

- analysis consumes only approved semantic graph material
- analysis results are decision-support artifacts, not facts
- analysis does not modify graph, lifecycle, approval, `project_manifest`, or `StoredObject`
- analysis does not infer authoritative knowledge
- analysis does not execute actions
- RDF projection may be a future input, but RDF remains projection, not authority
- SPARQL and Oxigraph remain disabled
- no algorithms are implemented in this phase
- analysis does not invalidate or repair upstream layers

## Eligible analysis inputs

Future graph analysis may consume:

- approved semantic nodes represented by `QuadInstance`
- approved semantic edges represented by `QuadRelation`

Excluded inputs:

- `PROPOSED`
- `UNDER_REVIEW`
- `REJECTED`
- `STALE`
- `SUPERSEDED`
- `ORPHANED`

Lifecycle filtering remains governed by `semantic_quad_lifecycle.md`.

## Conceptual analysis profiles

Future analysis profiles may include:

- `RISK_PATH`
- `DEPENDENCY_PATH`
- `LOW_CONFIDENCE_PATH`
- `IMPACT_PATH`
- `STALE_PATH`

Interpretation:

- `RISK_PATH`: traces future governed risk propagation across approved semantic relations
- `DEPENDENCY_PATH`: traces approved semantic dependency chains
- `LOW_CONFIDENCE_PATH`: highlights weak-support or weak-quality approved semantic regions
- `IMPACT_PATH`: highlights future likely impact surfaces across approved dependencies
- `STALE_PATH`: highlights approved semantic material threatened by stale supporting context

These profile names are conceptual analysis classes only. They do not implement algorithms now.

## Conceptual node metrics

Future node-oriented analysis may use conceptual metrics such as:

- `confidence`
- `criticality`
- `risk`
- `freshness`
- `review_level`
- `source_quality`

These metrics are analysis inputs or derived analysis descriptors only.

They are not:

- semantic facts
- approval state
- execution authority

## Conceptual edge metrics

Future edge-oriented analysis may use conceptual metrics such as:

- `dependency_strength`
- `risk_propagation`
- `semantic_weight`
- `evidence_strength`

These metrics are analysis descriptors only.

They do not:

- mutate relation lifecycle
- approve relations
- infer authoritative truth

## Traceability requirements

Future graph analysis must preserve traceability to:

- source `QuadInstance`
- source `QuadRelation`
- source evidence references
- source trace metadata

Analysis outputs must remain explainable and attributable to the governed semantic material they consumed.

## Relationship to RDF projection

RDF projection may become a future transport or projection input for analysis.

However:

- RDF remains projection, not authority
- RDF is not required for analysis policy definition
- RDF does not replace the internal governed semantic model
- SPARQL and Oxigraph remain inactive

## Graph analysis invariants

- `INV-ANALYSIS-001`: analysis consumes only `APPROVED` `QuadInstance` records and `APPROVED` `QuadRelation` records.
- `INV-ANALYSIS-002`: analysis does not modify graph or lifecycle.
- `INV-ANALYSIS-003`: analysis results are not authoritative facts.
- `INV-ANALYSIS-004`: analysis must preserve traceability to source quads, relations, evidence, and traces.
- `INV-ANALYSIS-005`: analysis respects lifecycle filtering.
- `INV-ANALYSIS-006`: no inference engine is introduced.
- `INV-ANALYSIS-007`: no execution logic is introduced.
- `INV-ANALYSIS-008`: RDF, SPARQL, and Oxigraph are not required or activated by this policy.
- `INV-ANALYSIS-009`: analysis cannot approve, reject, supersede, stale, or orphan semantic material.
- `INV-ANALYSIS-010`: analysis cannot trigger `ActionResolution` or tool execution.
- `INV-ANALYSIS-011`: analysis cannot invalidate, repair, or mutate storage, derivatives, semantic, or graph layers.

## Related specs

- `docs/specs/artifact_graph.md`
- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/rdf_projection_policy.md`
