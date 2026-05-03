# lume_help_tree

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the declarative `lume_help_tree` resource for Static Mode help navigation without LLM.

In the governed Help menu model, `Help Topics` opens this help-tree surface.

The help tree belongs to Lume, the single governed front-facing assistant identity.

It must not introduce another assistant, role selector, or prompt-editing surface.

## Core rule

Lume must be able to present a tree of help topics in Static Mode without LLM.

The tree guides "what to do to..." through navigation, explanation, and procedure nodes.

Visible text must come from i18n keys.

UI only renders prepared state. It does not interpret policy or execute actions.

## Node model

Every help-tree node must declare:

- `node_id`
- `kind`
- `title_i18n`
- `summary_i18n`
- `children`, when applicable
- `steps_i18n`, for procedure nodes
- `related_gui_objects`, when applicable
- `related_tool_ids`, when applicable
- `requires_llm`
- `executes_action`

`related_gui_objects` stores canonical GUI object ids aligned with `GUI_OBJECTS_v1`, not localized visible labels.

Visible GUI names are resolved through help or i18n resources.

## Supported node kinds

- `navigation`
- `explanation`
- `procedure`

Procedure nodes must declare `steps_i18n`.

## Minimal tree coverage

The declared tree must include at least:

- `root`
- `getting_started`
- `create_first_project`
- `tools`
- `merge_pdfs`
- `llm_assistance`
- `static_mode`
- `assisted_mode`

## Static Mode expectations

- help-tree navigation must work without LLM
- nodes may explain current capability limits
- nodes may include procedure steps through i18n references
- nodes must not create actions, mutate files, or execute tools

## Relationship to Lume onboarding

- onboarding may reuse or reference help-tree branches
- the help tree is a reusable declarative navigation surface
- the help tree does not replace onboarding flow or policy

## Relationship to Lume Help popup

- Lume Help may render the declared tree or selected branches
- popup state decides selection and expansion only as presentational state
- popup does not infer permissions or execute operations from tree content

## Relationship to Help menu

- `Help Topics` is a dedicated Help-menu entry
- it opens a dedicated informational popup or equivalent governed help-topics surface
- it remains static or declarative content only
- it does not execute, mutate runtime, or interact with tools

## Relationship to tools

- help nodes may reference tool ids
- referenced tools may be declared, available, non-executable, or future
- tool references are explanatory only

## Relationship to internal agent roles

Help nodes may explain future internal roles:

- Planner
- Specialists
- Synthesizer

Rules:

- internal roles must be described as conceptual roles only
- internal roles must not appear as user-facing assistants
- help nodes must not route directly to internal roles
- help nodes must not define, edit, or display raw agent prompts
- prompt storage is governed by `docs/specs/llm_agent_prompts.md`

## Invariants

- `INV-LUME-TREE-001`: `lume_help_tree` MUST support Static Mode help without LLM.
- `INV-LUME-TREE-002`: visible help-tree text MUST be referenced through i18n keys, not embedded visible strings.
- `INV-LUME-TREE-003`: help-tree nodes MAY describe procedures but MUST NOT execute actions.
- `INV-LUME-TREE-004`: help-tree nodes MAY reference GUI objects and tools but MUST NOT grant authority.
- `INV-LUME-TREE-005`: UI renders prepared tree state only; it does not interpret policy, execute tools, or resolve permissions.
- `INV-LUME-TREE-006`: nodes with `requires_llm=true` MUST remain explanatory when LLM is unavailable.
- `INV-LUME-TREE-007`: `executes_action` MUST remain `false` for F9 declarative help-tree nodes.
- `INV-LUME-TREE-008`: Help Topics MUST remain informational and MUST NOT introduce dynamic runtime logic.
- `INV-LUME-TREE-009`: help-tree content MUST NOT introduce another front-facing assistant.
- `INV-LUME-TREE-010`: help-tree content MUST NOT define, edit, or expose raw agent prompts.

## Forbidden responsibilities

The help tree must not:

- call LLM providers
- execute tools
- create actions
- approve operations
- mutate files
- validate policies
- persist help history
- expose internal agent roles as selectable assistants
- edit prompts
- open F10

## Related specs

- `docs/specs/lume_onboarding_model.md`
- `docs/specs/help_menu.md`
- `docs/specs/lume_help_popup.md`
- `docs/specs/gui_objects_v1.md`
- `docs/specs/llm_tool_surface_resolution.md`
- `docs/specs/llm_agent_prompts.md`
