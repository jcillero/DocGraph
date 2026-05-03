# semantic_layer_boundaries

## Purpose

Define strict boundaries between storage, derivatives, semantic, graph, and analysis layers.

This document is declarative only.

It does not implement runtime, execution, mutation engines, graph engines, semantic generation, or F10 execution slices.

## Layer definitions

- `storage`: physical data and governed logical storage references
- `derivatives`: deterministic transformations derived from storage-backed source material
- `semantic`: interpretative layer expressed as semantic quads and quad instances
- `graph`: explicit relationship layer between semantic quads or other governed artifact relations
- `analysis`: read-only evaluation layer over approved graph material

## Access model

| Layer | Read | Write | Invalidate |
|---|---|---|---|
| `storage` | yes | yes | triggers downstream invalidation |
| `derivatives` | yes | yes | triggers downstream invalidation |
| `semantic` | yes | yes | reacts to upstream invalidation |
| `graph` | yes | yes | reacts to upstream invalidation |
| `analysis` | yes | no | no |

Interpretation:

- storage is the root authority
- derivatives may be written only as deterministic transformations of governed source material
- semantic may write proposal-layer semantic material only
- graph may write explicit relation material only
- analysis is read-only and cannot write or invalidate any layer

## Invalidation flow

The invalidation direction is one-way:

```text
storage -> derivatives -> semantic -> graph
```

Rules:

- no reverse mutation is allowed
- no downstream layer may mutate an upstream layer
- invalidation in an upstream layer may stale or orphan downstream semantic or graph material
- analysis may observe invalid states but must not mutate them

## Cross-layer boundaries

### storage

- may be read by derivatives, semantic evidence resolution, graph traceability, and analysis inputs
- may trigger downstream invalidation
- remains the only physical authority

### derivatives

- may read governed storage-backed material
- may write deterministic, regenerable derivative artifacts
- must not write semantic claims
- may trigger downstream semantic invalidation when evidence anchors change

### semantic

- may read storage-backed sources and derivatives as evidence
- may write proposal-layer quads and quad instances only
- must not write back into derivatives or storage
- must react to upstream invalidation rather than repairing sources locally

### graph

- may read approved semantic material and governed trace references
- may write explicit relation edges only
- must not modify quads, derivatives, or storage
- must react to semantic invalidation rather than mutating semantic source material

### analysis

- may read approved graph and semantic material only
- must not write graph, semantic, derivatives, or storage
- must not invalidate other layers

## Invariants

- `INV-LAYER-001`: storage is root authority
- `INV-LAYER-002`: no semantic writes into derivatives
- `INV-LAYER-003`: graph cannot modify quads
- `INV-LAYER-004`: analysis cannot modify graph
- `INV-LAYER-005`: no reverse mutation is allowed across layer boundaries
- `INV-LAYER-006`: invalidation flows only from storage to derivatives to semantic to graph
- `INV-LAYER-007`: analysis is read-only evaluation only
- `INV-LAYER-008`: semantic and graph writes remain proposal or governed relation material only

## Related specs

- `docs/specs/storage_policy.md`
- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/artifact_graph.md`
- `docs/specs/graph_analysis_policy.md`
