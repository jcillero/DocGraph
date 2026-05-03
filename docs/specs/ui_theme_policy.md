# ui_theme_policy

## Purpose

Define the governed semantic theme contract for DocGraph and Lume, including portable typography tokens.

## Core rule

UI appearance must remain resource-driven, token-based, and portable across supported environments.

## Typography profile

DocGraph/Lume uses a Codex-like typography profile based on semantic roles rather than platform-specific font names.

Typography tokens must remain abstract and system-font oriented:

- `typography.ui.family`
- `typography.code.family`
- `typography.chat.family`
- `typography.metadata.family`
- `typography.id.family`
- `typography.technical_compact.family`

Supporting semantic text-size tokens may include:

- `typography.ui.size`
- `typography.chat.size`
- `typography.metadata.size`
- `typography.id.size`
- `typography.technical_compact.size`

## Rules

- UI runtime must consume semantic typography tokens rather than hardcoded font names
- tokens must prefer system fonts through abstract families
- monospace is reserved for:
  - code
  - paths
  - identifiers
  - hashes
  - structured logs
- standard UI copy, narrative chat text, and metadata labels must not default to monospace
- no external font files are embedded by this policy
- no platform-specific font stack is hardcoded by this policy
- typography policy must not change layout logic by itself

## Intended roles

- `ui`: default product UI copy and general readable interface text
- `code`: code blocks and code-like snippets
- `chat`: conversational body text
- `metadata`: secondary labels, captions, timestamps, and contextual annotations
- `id`: ids, refs, hashes, paths, and stable technical identifiers
- `technical_compact`: dense technical summaries, manifests, and structured diagnostics

## Portability

- light and dark themes must expose the same semantic typography token set
- token meaning must remain stable across appearance modes
- typography tokens must preserve theme portability and must not imply host-specific availability

## Invariants

- `INV-THEME-001`: presentation code must consume semantic theme tokens rather than hardcoded colors
- `INV-THEME-002`: presentation code must consume semantic typography roles rather than platform-specific font names
- `INV-THEME-003`: monospace is restricted to code and technical identifier roles
- `INV-THEME-004`: typography tokens must remain portable across supported appearance modes
- `INV-THEME-005`: typography policy does not introduce runtime logic, execution authority, or layout ownership
