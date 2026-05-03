# I18N Policy

This sandbox inherits the Python rule that user-visible UI text must be externalized and governed outside views.

Supported UI languages:

- English (`en`)
- Spanish (`es`)

## Rules

- user-visible UI strings must not be hardcoded in views
- localized resources must live outside presentation code
- localized resources are consumed through `ui_i18n`
- the fallback order is: preferred language -> English -> controlled fallback
- all user-visible menu labels MUST use i18n keys
- all user-visible popup titles and popup descriptions MUST use i18n keys
- `Slint` views and other presentation views MUST NOT hardcode visible text
- every governed key in `resources/i18n/en/*.ftl` MUST have an equivalent key in `resources/i18n/es/*.ftl`
- every governed key in `resources/i18n/es/*.ftl` MUST have an equivalent key in `resources/i18n/en/*.ftl`
- internal identifiers MUST remain English and stable
- translated labels MUST NOT become runtime authority, identifiers, policy decisions, or execution triggers

## Naming convention

Governed visible-text keys must use stable domain prefixes:

- `menu.*`
- `popup.*`
- `dialog.*`
- `tooltip.*`
- `status.*`
- `help.*`

These prefixes classify visible text only.

They do not define runtime behavior, authority, routing, or execution semantics.

## Language distinctions

- UI language: language of menus, dialogs, labels, and general interface text
- chat language: language used in conversational interaction
- internal control language: implementation and control-layer language used for system logic and stable identifiers

These are related but not identical concerns and must not be conflated.

This sandbox includes placeholder Fluent resources only.

## Governed menu and popup contract

The top-level menu domains and governed popup surfaces must remain label-driven by i18n resources rather than by view-local literals.

This includes:

- top-level menu labels
- menu entry labels
- governed popup titles
- governed popup descriptions
- generic dialog button labels
- governed tooltips for visible UI chrome

Internal object names, popup ids, controller ids, and policy ids remain language-independent and stable even when their user-facing labels are translated.

Deprecated i18n keys may remain temporarily for compatibility or migration safety, but new or updated specs must not reference them as active contract keys.

## Invariants

- `INV-I18N-001`: all visible menu labels MUST come from i18n resources.
- `INV-I18N-002`: all visible popup titles and descriptions MUST come from i18n resources.
- `INV-I18N-003`: English and Spanish i18n files MUST remain key-symmetric for governed menus and popups.
- `INV-I18N-004`: translated labels MUST NOT become identifiers or runtime authority.
- `INV-I18N-005`: internal identifiers MUST remain stable and language-independent.
- `INV-I18N-006`: no visible UI text may be hardcoded in `Slint` or presentation views.
- `INV-I18N-007`: deprecated i18n keys MUST NOT be used by new specs.
