# identity_system

## Purpose

Define the unified identity system for governed DocGraph entities.

This document is declarative only.

It does not implement runtime generation, persistence, execution, or validation engines.

## Identity types

The governed identity types are:

- `file_ref`
- `object_ref`
- `quad_id`
- `quad_instance_id`
- `relation_id`
- `intent_id`
- `trace_ref`

## Namespaces

Controlled prefixes:

- `file_ref` -> `sha256_`
- `object_ref` -> `obj_`
- `quad_id` -> `q_`
- `quad_instance_id` -> `qi_`
- `relation_id` -> `rel_`
- `intent_id` -> `intent_`
- `trace_ref` -> `trace_`

These namespaces must remain distinct.

No identifier from one namespace may be reused as if it belonged to another namespace.

## Identity roles

### file_ref

- content-addressed identity
- deterministic from content bytes
- independent from filename
- independent from host path

Example:

- `sha256_<hex>`

### object_ref

- logical object identity
- stable logical reference for `StoredObject`
- not derived from UI state
- not derived from portable path text alone

Example:

- `obj_<stable_suffix>`

### quad_id

- deterministic semantic identity
- derived from normalized semantic content and evidence anchor
- represents what is asserted
- does not include generation-instance context

Example:

- `q_<stable_suffix>`

### quad_instance_id

- generation-instance identity
- represents a specific produced instance of a semantic assertion
- may vary across different generation contexts for the same `quad_id`

Example:

- `qi_<instance_suffix>`

### relation_id

- stable relation-edge identity
- identifies an explicit governed relation object
- remains separate from `quad_id` and `quad_instance_id`

Example:

- `rel_<stable_suffix>`

### intent_id

- governed action-intent instance identity
- identifies a declarative `ActionIntent`
- instance-based rather than execution-authorizing

Example:

- `intent_<instance_suffix>`

### trace_ref

- governed observability identity
- links related artifacts across one logical flow
- remains reference-only and non-executing

Example:

- `trace_<stable_or_instance_suffix>`

## Determinism and instance rules

- `file_ref` must be deterministic
- `quad_id` must be deterministic
- `quad_instance_id` is instance-based
- `relation_id` is stable edge identity and must not collide with quad namespaces
- `object_ref` must remain stable for the governed logical object it identifies
- `intent_id` is instance-based and must not imply authorization
- `trace_ref` is observability identity and must not imply authorization

## Collision rules

- no collisions across namespaces
- prefix is part of governed identity interpretation
- a value valid for one identity type must not be interpreted as another identity type

## Boundary rules

- identity must not depend on file path
- identity must not depend on UI
- identity must not depend on translated labels
- identity must not become execution authority
- identity stability must not be broken by presentation changes
- `intent_id` must not become execution authority
- `trace_ref` must not become execution authority

## Invariants

- `INV-ID-SYS-001`: identity must be stable for the entity type it governs
- `INV-ID-SYS-002`: identity must not depend on file path
- `INV-ID-SYS-003`: identity must not depend on UI
- `INV-ID-SYS-004`: deterministic identities must remain deterministic where required
- `INV-ID-SYS-005`: instance identities must remain distinct from deterministic semantic identities
- `INV-ID-SYS-006`: namespaces must not collide
- `INV-ID-SYS-007`: identity must not imply execution authority
- `INV-ID-SYS-008`: `intent_id` and `trace_ref` are governed namespaces and must remain distinct from storage and semantic IDs
- `INV-ID-SYS-009`: `intent_id` and `trace_ref` do not imply authorization, approval, or execution

## Related specs

- `docs/specs/storage_policy.md`
- `docs/specs/semantic_quad_model.md`
- `docs/specs/action_contract.md`
- `docs/specs/system_observability.md`
