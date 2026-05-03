# transformation_recipes

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the initial governed specification for `TransformationRecipe`.

Recipes prepare reusable declarative transformation patterns derived from prior governed transformation history.

They do not execute anything in F9.

## Definition

A recipe is a reusable declarative sequence of transformations.

It is not:

- execution
- a free macro
- a script

## Origins

A recipe may be:

- derived from `TransformTimeline`
- created manually in a future phase
- suggested by the system in a future phase

F9 defines the concept only. It does not implement recipe creation flows.

## Conceptual model

### TransformationRecipe

Minimum conceptual fields:

- `recipe_id`
- `display_name`
- `description`
- `origin`
- `steps`
- `reusable`
- `status`

Field meaning:

- `recipe_id`: stable governed recipe identifier
- `display_name`: visible human-oriented recipe name
- `description`: concise explanation of recipe purpose
- `origin`: provenance of the recipe, such as timeline-derived, manual-future, or system-suggested-future
- `steps`: ordered declarative `RecipeStep` sequence
- `reusable`: whether the recipe is intended for reuse when future governance allows it
- `status`: current declarative lifecycle state

### RecipeStep

Minimum conceptual fields:

- `step_id`
- `operation_kind`
- `target_policy`
- `instruction_template`
- `required_inputs`
- `output_policy`

Field meaning:

- `step_id`: stable ordered step identifier within the recipe
- `operation_kind`: declared transformation operation category
- `target_policy`: rule describing what kinds of governed targets the step may address
- `instruction_template`: reusable instruction pattern for the step
- `required_inputs`: declared inputs needed before future governed execution could be considered
- `output_policy`: rule describing the expected proposal or output treatment

## States

Prepared recipe state may include:

- `draft`
- `validated_future`
- `reusable_future`
- `deprecated`
- `invalid`

Interpretation:

- `draft`: initial declarative recipe shape exists but is not yet future-validated
- `validated_future`: reserved future state for a later governed validation phase
- `reusable_future`: reserved future state for later governed reuse
- `deprecated`: recipe should no longer be preferred
- `invalid`: recipe structure is not safe or not complete enough even as declarative pattern

## Rules

- no execution in F9
- no mutation
- no LLM invocation
- future application requires `ActionResolution`
- recipe does not substitute `tool_runtime`

Normative form:

- `INV-TRECIPE-001`: recipes MUST remain declarative in F9
- `INV-TRECIPE-002`: recipes MUST NOT execute transformations in F9
- `INV-TRECIPE-003`: recipes MUST NOT mutate files or documents by themselves
- `INV-TRECIPE-004`: recipes MUST NOT invoke LLMs directly
- `INV-TRECIPE-005`: any future recipe application MUST pass through `ActionResolution`
- `INV-TRECIPE-006`: recipes MUST NOT replace `tool_runtime` or create a parallel execution pipeline

## Relationship to timeline

`TransformTimeline` may feed recipe derivation.

The timeline preserves historical attempts and decisions.
The recipe abstracts reusable pattern from that history.

Timeline and recipe must remain distinct:

- timeline = memory and trace
- recipe = reusable declarative pattern

## Relationship to transformation_core

Recipes build on the canonical vocabulary from `transformation_core.md`.

They must not redefine:

- instruction lineage
- proposal identity
- preview semantics
- acceptance semantics

## Failure modes

- `empty_recipe`
- `invalid_step`
- `missing_target_policy`
- `unsupported_operation_kind`
- `unsafe_recipe_execution_blocked`

Interpretation:

- `empty_recipe`: recipe contains no usable declarative steps
- `invalid_step`: at least one step is structurally invalid
- `missing_target_policy`: a step lacks required target-governance policy
- `unsupported_operation_kind`: a step declares an operation kind outside governed support
- `unsafe_recipe_execution_blocked`: future execution must remain blocked because the recipe is unsafe or not authorized

## Invariants

- `INV-TRECIPE-007`: recipe steps MUST remain ordered declarative data, not embedded script logic
- `INV-TRECIPE-008`: recipe provenance SHOULD remain traceable to its origin when available
- `INV-TRECIPE-009`: reusable recipe intent MUST remain distinct from actual execution capability

## Related specs

- `docs/specs/transformation_core.md`
- `docs/specs/transform_timeline.md`
