# document_patch_runtime

## Purpose

Define the future patch-runtime concept for governed document editing.

This is a proposal only. No runtime is implemented by this spec.

## Future runtime role

A future `document_patch_runtime` may validate, preview, and apply accepted patches to `ARTIFACT` documents.

Popup capture, proposal creation intent, and acceptance-before-application semantics are defined upstream by `docs/specs/transform_popup.md` and `docs/specs/transformation_core.md`.

It must not:

- duplicate the `project_runtime` pipeline
- widen `ProjectRuntimeOutput`
- live inside UI crates
- allow LLMs to write files directly
- mutate `SOURCE` or `DERIVED` documents

## Patch types

Future patch proposals may include:

- `TextPatch`
- `StructuralPatch`
- `TablePatch`
- `FigureMetadataPatch`
- `EquationPatch`

Patch exchange structures may include:

- `PatchProposalSet`
- `PatchPreview`
- `PatchAcceptance`

When aligned with transformation-core vocabulary, these future runtime structures should map cleanly to `TransformProposal`, preview state, and accepted decision trace rather than redefining them independently.

## PatchProposalSet

A `PatchProposalSet` is a numbered set of candidate changes.

Each proposal must identify:

- target artifact
- target range or structured target
- patch type
- proposed operation
- validation status
- preview availability

The user accepts one proposal or requests regeneration through the governed popup flow.

F9 popup governance does not require a separate reject button surface.

## Deterministic minimum operation

`replace_exact_text` is the minimum deterministic text operation.

It requires:

- one target artifact
- exact current text
- replacement text
- validation that exactly one match exists
- preview before application
- acceptance before write

## Future operations

Future governed operations may include:

- `replace_exact_text`
- `replace_paragraph`
- `rewrite_section`
- `add_section`
- `remove_section`
- `move_section`
- `rename_section`
- `insert_table_reference`
- `insert_figure_reference`
- `insert_equation_reference`
- `update_figure_metadata`
- `update_table_metadata`
- `update_equation_latex`

## Error model

The future runtime must reject unsafe or ambiguous proposals.

Required errors:

- `no_match`
- `multi_match`
- `stale_artifact`
- `forbidden_target`

## Application rule

The runtime applies only an accepted patch.

LLM output is proposal input, not filesystem authority.

The user validates intent; the runtime validates determinism and target safety.
