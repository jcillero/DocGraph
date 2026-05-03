# semantic_source_scope

## Status

PROPOSAL / F9/F10_PREP declarative governance.

This document does not implement semantic runtime, quad generation runtime, RDF, Oxigraph, SPARQL, embeddings, LLM execution, tool execution, filesystem mutation, document mutation, or runtime approval.

## Purpose

Define the future source scope from which DocGraph may generate semantic quads.

This scope governs eligible semantic sources and evidence anchors only.

It does not define semantic approval, execution authority, or deterministic derivative ownership.

DocGraph may later generate semantic quad instances from:

- document metadata
- derived chunks
- user knowledge files
- system governance files

The initial goal is not free inference.

The initial goal is to build traceable graphs for:

- document structure
- chunks
- provenance
- metadata
- future semantic knowledge

## Source classes

### user_knowledge_source

User-added documents and knowledge materials, including:

- PDFs
- Markdown
- plain text
- reports
- technical documentation

### structured_metadata_source

Structured file or resource metadata, including:

- file type
- title
- logical path
- hash
- size
- dates, when applicable

### chunk_source

Regenerable chunk derivatives from documents, including:

- offsets
- pages
- sections
- textual fragments

### system_governance_source

System governance materials, including:

- `governance/GOVERNANCE.md`
- `architecture/ARCHITECTURE.md`
- `governance/FUNCTIONAL_SCOPE.md`
- specs
- policies
- invariants

## source_kind alignment

Future `QuadInstance.source_kind` values are limited to:

- `file`
- `chat`
- `artifact`
- `tool_output`
- `metadata`

Alignment:

- `file` resolves to a persisted `StoredObject` with `content_ref`
- `chat` resolves to a logical `StoredObject` or governed message reference
- `artifact` resolves to an artifact or `DocumentHolder` reference
- `tool_output` resolves to an `owner_ref`-backed governed tool output
- `metadata` resolves to a governed metadata snapshot rather than inferred facts

No `source_kind` grants execution or exposure authority.

Eligible source classes do not imply mandatory quad generation.

Future generation remains trigger-bound and scope-bounded under `quad_generation_policy.md`.

## Evidence scope

Allowed evidence anchors may include:

- `file_ref`
- `object_ref`
- `chunk_id`
- `chunk_hash`
- `text_range`
- `metadata_snapshot_hash`
- `message_ref`
- `artifact_ref`
- `tool_run_ref`

Evidence must remain traceable and reference governed sources rather than duplicate full text.

Evidence invalidation and lifecycle consequences are governed by `semantic_quad_lifecycle.md`.

Future semantic relations may reference shared governed evidence, but those relations remain separate graph-edge objects rather than child structures inside quads.

## Graph domains

Separate graph domains may include:

- `structural_graph`
- `metadata_graph`
- `semantic_graph`
- `governance_graph`

Interpretation:

- `structural_graph`: documents, chunks, offsets, and document relations
- `metadata_graph`: file and resource properties
- `semantic_graph`: concepts, relations, summaries, and future semantic proposals
- `governance_graph`: phases, invariants, policies, and system rules

All domains may use the same semantic base model:

- `SemanticQuad` as semantic identity
- `QuadInstance` as generation instance
- `QuadSet` as contextual grouping

They must not be mixed without explicit domain resolution.

## Examples

### Structural and metadata examples

- `(DOC_A, has_chunk, CHUNK_001, a1b2c3d4e5f6)`
- `(CHUNK_001, belongs_to, DOC_A, b2c3d4e5f6a7)`
- `(CHUNK_001, starts_at, 0, c3d4e5f6a7b8)`
- `(CHUNK_001, ends_at, 1200, d4e5f6a7b8c9)`
- `(DOC_A, has_file_type, "pdf", e5f6a7b8c9d0)`

### Governance examples

- `(GOVERNANCE, defines, F9_invariants, f6a7b8c9d0e1)`
- `(tool_runtime, must_not_consume, tools_master_catalog, a7b8c9d0e1f2)`
- `(Lume, must_not_execute, tools, b8c9d0e1f2a3)`

## Semantic Source Scope Invariants

- `INV-SRC-001`: quads generated from metadata, chunks, semantic content, and governance MUST remain domain-scoped.
- `INV-SRC-002`: structural_graph, metadata_graph, semantic_graph, and governance_graph MUST NOT be merged without explicit governed resolution.
- `INV-SRC-003`: chunk-derived quads MUST preserve traceability to the source chunk.
- `INV-SRC-004`: metadata-derived quads MUST NOT become runtime authority.
- `INV-SRC-005`: governance-derived quads describe system rules but MUST NOT execute, approve, or enforce runtime behavior in F9/F10_PREP.
- `INV-SRC-006`: Lume MAY use governance-derived knowledge in a future governed mode, but MUST NOT treat it as execution authority.
- `INV-SRC-007`: user knowledge quads and system governance quads MUST be distinguishable.
- `INV-SRC-008`: filesystem presence MUST NOT imply quad generation.
- `INV-SRC-009`: project_manifest exposure and future policy MUST govern eligible sources.
- `INV-SRC-010`: no source class opens LLM execution, tool execution, RDF persistence, Oxigraph, SPARQL, embeddings, or document mutation.
- `INV-SRC-011`: `source_ref` MUST resolve to a governed source reference or `StoredObject` when the source is persisted.
- `INV-SRC-012`: chunks may provide evidence anchors but MUST NOT become semantic containers.
- `INV-SRC-013`: deterministic derivatives and semantic quad material MUST remain separate layers.
- `INV-SRC-014`: metadata snapshots may feed semantic proposals but MUST NOT be treated as approved knowledge.
- `INV-SRC-015`: eligible semantic sources must not trigger uncontrolled or automatic quad generation without an explicit governed trigger.

## Relationship with existing specs

This source-scope proposal depends on and must remain aligned with:

- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/tools_catalogs.md`

`document_text_runtime.md` defines regenerable chunk and text boundaries.

`storage_policy.md` defines `StoredObject`, `file_ref`, and derivative separation boundaries.

`tools_catalogs.md` remains unrelated to semantic execution authority and must not be treated as semantic source runtime.

## Future work

The following remain future-governed concerns:

- official predicates
- entity types
- semantic rules
- graph domain identifiers
- serialization
- RDF / N-Quads converter
- Oxigraph integration
- lifecycle resolution
- ontology

## Explicit limits

This document:

- does not touch `project_runtime`
- does not touch `tool_runtime`
- does not touch `ui_*`
- does not modify Rust
- does not add crates
- does not modify `tools_master_catalog.json`
- does not activate LLM
- does not activate tools
- does not activate Oxigraph
- does not create real quad persistence

## Related specs

- `docs/specs/semantic_quad_model.md`
- `docs/specs/semantic_quad_lifecycle.md`
- `docs/specs/semantic_proposal_layer.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/quad_generation_policy.md`
- `docs/specs/tools_catalogs.md`
