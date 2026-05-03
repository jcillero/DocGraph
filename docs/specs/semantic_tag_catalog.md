# semantic_tag_catalog

## Purpose

Define the governed catalog authority for semantic classification tags used by `StoredObject.metadata.semantic`.

## Catalog authority

`resources/semantic/semantic_tag_catalog.json` is the authority for controlled semantic tags.

Rules:

- stored objects reference `tag_id` only
- tag meaning lives in the catalog, not in each document
- free-form uncontrolled `system_tags` are forbidden
- free-form uncontrolled `usage_tags` are forbidden
- `keyword` governance is semi-controlled in the current declaration:
  - keywords must use normalized `tag_id` entries from the catalog when available
  - new keyword candidates may be proposed before activation

## Tag families

Controlled tag families:

- `keyword`
- `system_tag`
- `usage_tag`

Rules:

- keywords describe topic or domain
- `system_tag` describes where the object belongs in DocGraph or Lume scope
- `usage_tag` describes intended use
- families must not be mixed

Example:

```json
{
  "keywords": ["system_design", "action_layer", "capabilities"],
  "system_tags": ["docgraph_core", "lume_core", "critical_knowledge"],
  "usage_tags": ["internal_reference", "llm_reasoning"]
}
```

## Tag entry model

Conceptual entry:

```json
{
  "tag_id": "llm_reasoning",
  "tag_family": "usage_tag",
  "label": "LLM reasoning",
  "description": "Content intended to support LLM reasoning workflows.",
  "status": "ACTIVE",
  "introduced_in": "F10_PREP",
  "allowed_for": [
    "source_file",
    "document",
    "artifact",
    "chat",
    "bibliographic_entry"
  ],
  "aliases": [],
  "replaces": [],
  "replaced_by": null,
  "quad_mapping": {
    "predicate": "intended_usage",
    "object": "LLM reasoning"
  }
}
```

## Naming rules

- `tag_id` must use `snake_case`
- lowercase only
- no spaces
- no punctuation except underscore
- once `ACTIVE`, `tag_id` is stable
- an active `tag_id` must never be reused for a different meaning

Good:

- `llm_reasoning`
- `docgraph_core`
- `internal_reference`

Bad:

- `LLM Reasoning`
- `docGraphCore`
- `paper_juan_important`
- `use-for-lume`

## Alias and merge rules

- aliases may point to one `ACTIVE` primary tag
- aliases are not primary `tag_id`
- duplicate proposals should become aliases when appropriate
- merges must be explicit

Example:

```json
{
  "tag_id": "llm_context",
  "aliases": ["llm_reference", "prompt_context"]
}
```

## StoredObject integration

`StoredObject.metadata.semantic` may contain:

- `keywords`
- `system_tags`
- `usage_tags`

Rules:

- `system_tags` must reference `ACTIVE` catalog entries
- `usage_tags` must reference `ACTIVE` catalog entries
- `DEPRECATED` entries may be read but should warn
- `PROPOSED` and `REJECTED` entries must not be used as active metadata
- keyword policy remains semi-controlled:
  - normalized catalog entries are preferred
  - proposals may exist before activation

## Future quad mapping

Tags may feed future proposal-only quad generation.

Examples:

- `system_tag`: `doc_X belongs_to_system_scope docgraph_core`
- `usage_tag`: `doc_X intended_usage llm_reasoning`
- `keyword`: `doc_X has_keyword system_design`

Rules:

- quad generation remains separately governed
- tag presence does not approve generated quads
- tag lifecycle affects future quad eligibility

## Invariants

- `INV-TAG-001`: controlled tags must exist in the catalog
- `INV-TAG-002`: `tag_id` is stable and never reused
- `INV-TAG-003`: tag families must not be mixed
- `INV-TAG-008`: documents reference `tag_id`, not meanings
- `INV-TAG-009`: free-form `system_tags` and `usage_tags` are forbidden
- `INV-TAG-010`: tag-based quads remain proposal-only unless separately approved by lifecycle-governed semantic flow
