# app_main_menu

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the minimum stable top menu for DocGraph.

## Core rule

The main menu emits structured intent.

The main menu does not execute tools.
The main menu does not call LLM.
The main menu does not decide permissions.
The main menu does not interpret tool catalogs.

## Minimum menu

The main menu remains:

- `File`
- `Preferences`
- `Tools`
- `Help`

## Menu structure

### File

- `New project`
- `Open project`
- `Close project`
- separator
- `Exit`

### Preferences

- `App preferences`
- `Project settings`
- `Credentials management`

Clicking `Preferences` must expose exactly these three entries.

`Preferences` is a top-level configuration domain.

It must remain separate from `Tools`.

Each entry opens its own governed popup surface:

- `App preferences` opens the app-preferences popup
- `Project settings` opens the project-settings popup
- `Credentials management` opens the credentials-management popup

There is no shared Preferences panel.
There are no Preferences tabs.

### Tools

- `Operational Tools`
- `LLM Tools`
- `External Dependencies`

Clicking `Tools` must expose exactly these three entries.

`Tools` is a top-level menu domain.

It must remain separate from `Preferences`.

Each entry opens its own governed panel or popup surface:

- `Operational Tools` opens the operational-tools surface
- `LLM Tools` opens the LLM-tools surface
- `External Dependencies` opens the external-dependencies surface

There is no single combined tools panel with tabs.

### Help

- `Open Lume Help`
- `Help Topics`
- `About DocGraph`

Clicking `Help` must expose exactly these three entries.

`Help` is a top-level informational domain.

It must remain separate from `Preferences` and `Tools`.

Each entry opens its own governed popup surface:

- `Open Lume Help` opens the `Lume Help` popup
- `Help Topics` opens the `LumeHelpTree` popup surface
- `About DocGraph` opens the `AboutPopup`

## Tools menu rule

The main menu routes into the governed tools-management domain.

The menu does not execute tools, prepare hidden execution, install dependencies, or interpret policy.

## Preferences menu rule

The main menu routes into the governed preferences domain.

The menu emits configuration intent only.
The menu does not execute tools.
The menu does not resolve policy.
The menu does not mutate runtime.
The menu does not bypass `ActionResolution`.

## Help rule

`Help` is informational only.

It emits navigation and help intent only.
It does not execute.
It does not resolve policy.
It does not mutate runtime.
It does not interact with tools.
It does not expose credentials.
It does not trigger LLM execution.

`Lume Help` is not LLM-only help.

It must remain usable in `Static Mode`.

`About DocGraph` is the location for about and credits.

## DocGraph icon

- The top app chrome may display the `DocGraph` icon before `File`.
- The icon is branding and a navigation shortcut.
- If clicked, it requests opening `Lume Help`.
- It is equivalent to `Help -> Open Lume Help`.
- It must not execute tools, call LLM, mutate files, change settings, or activate runtime capabilities.
- It must not replace the Help menu entry.

## I18n rule

Visible menu text must come from i18n resources.

The governed top-level menu contract uses these stable keys:

- `menu.file`
- `menu.file.new_project`
- `menu.file.open_project`
- `menu.file.close_project`
- `menu.file.exit`
- `menu.preferences`
- `menu.preferences.app`
- `menu.preferences.project`
- `menu.preferences.credentials`
- `menu.tools`
- `menu.tools.operational`
- `menu.tools.llm`
- `menu.tools.external_dependencies`
- `menu.help`
- `menu.help.lume`
- `menu.help.topics`
- `menu.help.about`

Legacy keys may remain in resources for compatibility, but new specs must not reference:

- `menu.tools.open_panel`
- `menu.tools.external`
- `menu.help.tree`

## Invariants

- `INV-MENU-001`: the main menu MUST remain minimal and stable.
- `INV-MENU-002`: the main menu MUST expose navigation intents, not execution.
- `INV-MENU-003`: the main menu MUST NOT execute tools, LLM calls, filesystem mutations, or project actions.
- `INV-MENU-004`: `Tools` MUST be a top-level menu domain separate from `Preferences`.
- `INV-MENU-005`: `Tools` MUST expose exactly `Operational Tools`, `LLM Tools`, and `External Dependencies`.
- `INV-MENU-006`: each `Tools` entry MUST open its own governed panel or popup surface.
- `INV-MENU-007`: the main menu MUST NOT define a single combined tools panel with tabs.
- `INV-MENU-008`: `Preferences` MUST be a top-level menu domain separate from `Tools`.
- `INV-MENU-009`: `Preferences` MUST expose exactly `App preferences`, `Project settings`, and `Credentials management`.
- `INV-MENU-010`: each `Preferences` entry MUST open a dedicated governed popup surface.
- `INV-MENU-011`: the main menu MUST NOT define a shared Preferences panel with tabs.
- `INV-MENU-012`: `Help` MUST be a top-level menu domain separate from `Preferences` and `Tools`.
- `INV-MENU-013`: `Help` MUST expose exactly `Open Lume Help`, `Help Topics`, and `About DocGraph`.
- `INV-MENU-014`: each `Help` entry MUST open a dedicated informational popup surface.
- `INV-MENU-015`: Help MUST NOT execute actions, invoke tools, invoke LLM execution, access credentials, or mutate runtime.
- `INV-MENU-016`: About/Credits MUST live under Help / About DocGraph.
- `INV-MENU-017`: visible menu text MUST come from i18n resources.
- `INV-MENU-018`: governed menu labels MUST use the stable `menu.*` keys declared by this spec.
- `INV-MENU-019`: deprecated menu keys MUST NOT be used by new specs.
- `INV-BRANDING-001`: the DocGraph icon MAY be displayed in the top-left app chrome as branding.
- `INV-BRANDING-002`: the DocGraph icon MUST NOT execute tools, LLM calls, project actions, filesystem mutations, or settings changes.
- `INV-BRANDING-003`: if clickable, the DocGraph icon MAY open Lume Help as a navigation intent.
- `INV-BRANDING-004`: the DocGraph icon MUST NOT replace the Help menu entry.
- `INV-BRANDING-005`: icon resource paths MUST be declarative and MUST NOT be hardcoded in UI logic.
- `INV-BRANDING-006`: icon tooltip text MUST come from i18n.

## Forbidden responsibilities

The main menu must not:

- execute tools
- call LLM
- mutate filesystem
- resolve policy
- interpret tool catalogs
- place tools inside `Preferences`
- mix `Preferences` with `Tools`
- mix `Help` with `Preferences` or `Tools`
- open F10

## Related specs

- `docs/specs/tools_panel.md`
- `docs/specs/help_menu.md`
- `docs/specs/ui_preferences_popups.md`
- `docs/specs/project_settings_popup.md`
- `docs/specs/preferences_policy.md`
- `docs/specs/credentials_policy.md`
- `docs/specs/lume_help_popup.md`
- `docs/specs/lume_help_tree.md`
- `docs/specs/project_setup_popup.md`
