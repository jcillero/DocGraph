# i18n_audit_consistency

## Status

PROCEDURAL / ADVISORY ONLY.

No runtime implementation.

## Purpose

Define the scope and non-authoritative behavior of the i18n consistency audit.

The audit exists to surface likely i18n contract drift early without modifying resources, specs, runtime, or UI behavior.

## Audit scope

The advisory audit may inspect:

- `resources/i18n/en/*.ftl`
- `resources/i18n/es/*.ftl`
- `docs/specs/*`

The audit may report:

- keys present in English but missing in Spanish
- keys present in Spanish but missing in English
- keys that appear unreferenced in specs
- best-effort heuristics for visible UI text that looks hardcoded in specs

## Non-authority

The audit is advisory only.

It must not:

- modify any file
- auto-fix keys
- rewrite specs
- classify runtime authority
- be treated as proof of correctness

It is a review aid, not a source of truth.

## Interpretation rules

- missing-key findings are actionable consistency signals
- unreferenced-key findings are review signals, not automatic deletion candidates
- hardcoded-text findings are heuristic review signals only
- false positives are expected and require human review

## Codex usage

This audit must not be used as automatic patch input.

Human review is required before any change is proposed from its findings.
