# active_object_context

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed active-object context model for DocGraph.

This model separates independent selection by family from optional global focus.

## Core rule

`ActiveObjectContext` is governed context prepared by the system.

It is not pure UI state.
It does not grant permissions.
It does not execute actions.
It does not invent context that the user has not selected or referenced.

## Canonical model

Conceptual structure:

```rust
struct ActiveObjectContext {
    selected_document: Option<DocumentRef>,
    selected_sandbox: Option<SandboxRef>,
    selected_knowledge_item: Option<KnowledgeRef>,
    selected_artifact: Option<ArtifactRef>,
    selected_tool_output: Option<ToolOutputRef>,
    focused_family: Option<ActiveObjectFamily>,
}

enum ActiveObjectFamily {
    Document,
    Sandbox,
    KnowledgeItem,
    Artifact,
    ToolOutput,
}
```

There is no single global active object.

The system may hold:

- zero or one selected document
- zero or one selected sandbox
- zero or one selected knowledge item
- zero or one selected artifact
- zero or one selected tool output
- zero or one focused family

Optional prepared selection context may also exist for the currently selected document without becoming a separate active-object family.

## Family meaning

- `Document`: governed document reference resolved from project context
- `Sandbox`: governed sandbox context or sandbox view target
- `KnowledgeItem`: governed knowledge reference
- `Artifact`: governed artifact reference that is not already modeled as a document, knowledge item, or tool output
- `ToolOutput`: governed tool-output reference

## Selection rule

Selections are independent by family.

Selecting one family must not clear selections in other families unless a stricter governed policy is declared in a future phase.

## Left-click selection in the tree

General rule:

- left-click on a selectable artifact toggles selection inside its family

Cases:

- click on a non-selected artifact:
  - select it inside its family
  - replace any previous selection in that same family
  - set `focused_family` to that family

- click on an already selected artifact:
  - deselect it
  - if that family was focused, set `focused_family` to `None`

- click on an artifact from another family:
  - keep selections in other families
  - select the new artifact in its own family
  - set `focused_family` to the new family

## Relationship to tabs

- `DocumentRef` may activate or open `Readonly Viewer`
- `SandboxRef` may activate or open `Sandbox View`
- other family selections may activate the corresponding governed workspace view in future

Changing the active tab does not clear selections in other families.

An active tab may update `focused_family` only when the change is explicit and observable to the user.

Tab activation must not silently manufacture a selection for a family that has no valid selected reference.

Opening or activating the same governed target must reuse the existing tab identity rather than create duplicate tab context.

## Relationship to chat

`focused_family` determines the default target family for a chat instruction.

Rules:

- if `focused_family = None` and the instruction requires a target, the system must ask for clarification
- if `focused_family` points to a family without a valid selection, the system must ask for clarification
- if the instruction explicitly names a family, the system should use that family when a valid selection exists
- chat must not invent context
- chat must not use the last opened object as fallback

Explicit structured references remain stronger than implicit focus.

When a document selection is scoped further by text selection, the structured `selection_ref` remains subordinate to `document_ref`, not a replacement for it.

## Selection states

Each family selection may conceptually be understood through these minimum states:

- `null`
- `selected`
- `missing`
- `stale`
- `readonly`
- `future_actionable`

Interpretation:

- `null`: no selection exists for that family
- `selected`: a valid governed selection exists
- `missing`: the reference no longer resolves
- `stale`: the reference resolves, but the prepared context is no longer current enough for the intended action
- `readonly`: the selection is valid but cannot be mutated in the current phase or policy
- `future_actionable`: the selection may become actionable only in a future governed phase

These are conceptual readiness states. They do not imply runtime execution.

## Invariants

- `INV-AOC-001`: at most one selection may exist per family
- `INV-AOC-002`: at most one `focused_family` may exist
- `INV-AOC-003`: `focused_family` must correspond to a family with a valid selection or be `None`
- `INV-AOC-004`: the system MUST NOT use implicit fallback to the last opened object
- `INV-AOC-005`: active-object identity MUST NOT be based on physical host paths
- `INV-AOC-006`: selections in one family MUST NOT silently erase selections in other families
- `INV-AOC-007`: focus is a targeting aid, not execution authority

## Failure modes

- `focused_family_without_selection`
- `selected_ref_missing`
- `stale_active_object`
- `ambiguous_chat_target`
- `implicit_last_object_blocked`

Interpretation:

- `focused_family_without_selection`: focus points to a family with no valid selected reference
- `selected_ref_missing`: the stored reference no longer resolves
- `stale_active_object`: the selected context is no longer safe to use as prepared target
- `ambiguous_chat_target`: the instruction requires a target but no unambiguous valid family can be prepared
- `implicit_last_object_blocked`: the system detected an attempt to fall back to a previously opened object without governed selection

## Forbidden responsibilities

`ActiveObjectContext` must not:

- grant permissions
- execute tools
- execute LLM calls
- mutate files
- reinterpret project manifests
- replace `ActionResolution`
- collapse all families into one pseudo-global object

## Related specs

- `docs/specs/active_context.md`
- `docs/specs/document_tree.md`
- `docs/specs/workspace_tabs.md`
- `docs/specs/chat_document_flows.md`
- `docs/specs/local_sandbox_context.md`
- `docs/specs/ui_core.md`
- `docs/specs/text_selection.md`
