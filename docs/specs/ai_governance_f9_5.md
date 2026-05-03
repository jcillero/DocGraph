# ai_governance_f9_5

## Status

F9.5 declarative governance. F10 is not opened.

## Purpose

Complete the F9.5 preparation for AI governance without real AI execution.

The AI layer declares prompts, semantic derivation specs, data sensitivity policy, trace schemas, and semantic proposal schemas.

## Principle

The AI does not act.

The AI suggests.

The system validates.

The human decides.

Future runtime may execute only in later phases.

## Declared resources

- `resources/ai/prompt_specs/extract_concepts_v1.json`
- `resources/ai/semantic_specs/concept_extraction_v1.json`
- `resources/ai/policies/data_sensitivity_policy.json`
- `resources/ai/schemas/prompt_spec.schema.json`
- `resources/ai/schemas/semantic_derivation_spec.schema.json`
- `resources/ai/schemas/data_sensitivity_policy.schema.json`
- `resources/ai/schemas/ai_trace_record.schema.json`
- `resources/ai/schemas/semantic_proposal.schema.json`

## F9.5 guarantees

- `execution_enabled = false`
- no LLM calls
- no tool execution
- no embeddings
- no RDF persistence
- no Oxigraph dependency
- no SPARQL
- no document mutation
- no automatic semantic approval
- no duplicate project pipeline

## Readonly UI surface

`Pipeline / Ontology View` is a readonly/mock surface.

It may show:

- document identity
- chunk identity
- prompt spec
- semantic spec
- semantic proposals
- human review state
- future RDF/Oxigraph disabled flags
- trace/governance metadata

It must not approve proposals, execute derivations, persist RDF, mutate files, invoke LLMs, or run tools.

## Future Oxigraph/RDF

Oxigraph is a future store candidate only.

F9.5 uses:

- `oxigraph_enabled = false`
- `rdf_export_enabled = false`
- `semantic_store_enabled = false`

N-Quads may be named as a future format, but no RDF file is generated.

## Boundaries

- `resources/ai/` declares.
- `document_text_runtime` remains the source of regenerable chunks.
- `ui_core` may hold readonly presentational view models.
- `ui_slint` represents state and callbacks only.
- `project_runtime` remains untouched.

## Related specs

- `docs/specs/semantic_proposal_layer.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/ui_core.md`
- `docs/specs/workspace_tabs.md`

