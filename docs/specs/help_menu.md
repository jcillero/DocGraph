# help_menu

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Help` menu domain for DocGraph.

`Help` is dedicated to:

- user guidance
- documentation navigation
- contextual help
- product information

## Core rule

`Help` is informational, navigational, and assistive.

It is not operational.
It is not executable.
It is not diagnostic.
It is not configurational.

## Menu model

`Help` is a top-level menu entry.

It exposes exactly:

- `Open Lume Help`
- `Help Topics`
- `About DocGraph`

Each entry opens a dedicated popup surface.

## Entry contracts

### Open Lume Help

Opens:

- `LumeHelpPopup`

Purpose:

- contextual assistant
- feature explanation
- navigation guidance

### Help Topics

Opens:

- `LumeHelpTree`

Purpose:

- categorized help topics
- hierarchical navigation
- domain-based grouping

### About DocGraph

Opens:

- `AboutPopup`

Purpose:

- product information
- credits
- version and license display

## Domain rules

`Help`:

- emits intent only
- does not execute
- does not resolve policy
- does not mutate runtime
- does not interact with tools
- does not expose credentials
- does not trigger LLM execution

## UX principles

`Help` must remain:

- easy to find
- visible
- predictable
- consistent
- self-service oriented

The menu should use clear labels and minimal entries.

## Governed i18n keys

The `Help` menu contract uses these stable menu keys:

- `menu.help`
- `menu.help.lume`
- `menu.help.topics`
- `menu.help.about`

The governed Help surfaces use these popup keys:

- `popup.help.lume.title`
- `popup.help.lume.description`
- `popup.help.topics.title`
- `popup.help.topics.description`
- `popup.help.about.title`
- `popup.help.about.description`

Popup descriptions must remain informational only.

They must not imply diagnostics, tool execution, or LLM execution.

## Invariants

- `INV-HELP-001`: `Help` MUST be a top-level menu domain.
- `INV-HELP-002`: `Help` MUST NOT execute actions.
- `INV-HELP-003`: `Help` MUST NOT invoke tools.
- `INV-HELP-004`: `Help` MUST NOT invoke LLM execution.
- `INV-HELP-005`: `Help` MUST NOT access credentials.
- `INV-HELP-006`: `Help` MUST NOT mutate runtime.
- `INV-HELP-007`: `Help` MUST contain only informational surfaces.
- `INV-HELP-008`: each `Help` entry MUST open a dedicated popup.
- `INV-HELP-009`: visible Help labels and governed popup text MUST come from i18n resources.

## Non-goals

Do not implement:

- runtime help assistant
- chatbot execution
- diagnostics
- telemetry
- remote help services
- auto-search
- auto-learning systems

## Related specs

- `docs/specs/app_main_menu.md`
- `docs/specs/lume_help_popup.md`
- `docs/specs/lume_help_tree.md`
- `docs/specs/gui_objects_v1.md`
