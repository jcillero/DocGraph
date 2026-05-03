# semantic_classification_policy

## Purpose

Define how semantic classification metadata is attached to governed objects without opening semantic runtime.

## Classification scope

Semantic classification metadata is limited to:

- `keywords`
- `system_tags`
- `usage_tags`

Classification is declarative only.

It does not:

- invoke LLM
- activate tags automatically
- mutate stored objects by itself
- generate quads
- approve knowledge
- open execution

## Integration rules

- `StoredObject.metadata.semantic` is the integration surface
- `system_tags` and `usage_tags` must resolve to `ACTIVE` catalog entries
- `DEPRECATED` catalog entries may remain readable for compatibility
- `PROPOSED`, `UNDER_REVIEW`, and `REJECTED` tags must not be used as active object metadata
- keywords follow the semi-controlled catalog policy in `semantic_tag_catalog.md`

## Catalog boundary

- the tag catalog is authority for controlled tags
- schemas are declarative contracts only
- schema validity does not imply approval
- schema validity does not imply project exposure
- schema validity does not imply execution authority

## StoredObject boundary

Semantic classification does not make `StoredObject` semantic authority.

It only provides governed metadata slots for future interpretation.

## Invariants

- `INV-TAG-001`: controlled tags must exist in the catalog
- `INV-TAG-003`: tag families must not be mixed
- `INV-TAG-008`: documents reference `tag_id`, not meanings
- `INV-TAG-009`: free-form `system_tags` and `usage_tags` are forbidden
- `INV-TAG-010`: tag-based quads remain proposal-only unless separately lifecycle-approved
