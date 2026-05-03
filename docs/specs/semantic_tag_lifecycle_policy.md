# semantic_tag_lifecycle_policy

## Purpose

Define the governed lifecycle for semantic classification tags.

## Lifecycle states

- `PROPOSED`
- `UNDER_REVIEW`
- `ACTIVE`
- `DEPRECATED`
- `REMOVED`
- `REJECTED`

Meanings:

- `PROPOSED`: suggested but not usable as a controlled tag
- `UNDER_REVIEW`: being assessed
- `ACTIVE`: allowed in `StoredObject` metadata
- `DEPRECATED`: still recognized but not recommended for new objects
- `REMOVED`: not allowed for new use; existing references require migration or compatibility handling
- `REJECTED`: rejected proposal and must not be used

## Allowed transitions

- `PROPOSED -> UNDER_REVIEW`
- `UNDER_REVIEW -> ACTIVE`
- `UNDER_REVIEW -> REJECTED`
- `ACTIVE -> DEPRECATED`
- `DEPRECATED -> REMOVED`
- `DEPRECATED -> ACTIVE`
- `PROPOSED -> REJECTED`

Rules:

- `ACTIVE` must never be set automatically by LLM
- transitions must be traceable
- no silent replacement
- no deletion without migration note

## Proposal sources

Allowed proposal sources:

- `user`
- `system`
- `llm`
- `governance`

Rules:

- users may propose tags
- system may suggest tags from duplicate or gap analysis
- LLM may suggest candidate tags
- LLM must not activate tags
- governance authority approves `ACTIVE` tags

## Proposal acceptance and rejection

A new tag may be proposed when:

- no existing tag captures the concept
- a new system scope appears
- a new usage mode appears
- a new recurring knowledge domain appears
- existing tags would create ambiguity

A new tag must be rejected when:

- it is a synonym of an existing tag
- it mixes tag families
- it is too document-specific
- it is subjective or temporary
- it duplicates an alias
- it lacks description

## Deprecation and removal

- `DEPRECATED` tags remain readable
- `DEPRECATED` tags must not be assigned to new objects unless explicitly allowed
- `DEPRECATED` tags should include `replaced_by` when possible
- `REMOVED` tags require migration or compatibility note

## Invariants

- `INV-TAG-004`: LLM may propose but not activate tags
- `INV-TAG-005`: `ACTIVE` requires governance approval
- `INV-TAG-006`: `DEPRECATED` tags remain readable
- `INV-TAG-007`: `REMOVED` tags require migration note
