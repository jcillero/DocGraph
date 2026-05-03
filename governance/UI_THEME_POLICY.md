# UI Theme Policy

This sandbox inherits the rule that UI appearance must be resource-driven and token-based.

Supported appearance modes:

- `light`
- `dark`

## Rules

- UI views must not hardcode colors
- only semantic theme tokens may be consumed by presentation code
- theme resources are loaded through `ui_theme`
- appearance is independent from UI language
- theme fallback must resolve through controlled resources rather than ad hoc defaults

## Fallback policy

- preferred theme resource
- default theme resource for the active mode
- controlled failure or diagnostic if required semantic tokens are missing
