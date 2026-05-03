# Engineering Notes

## Purpose

This file keeps lightweight engineering lessons from the Rust sandbox without promoting them prematurely to formal invariants or broad architecture rules.

## Path comparison in tests

Windows path rendering can differ from logical path meaning.

Do not compare paths in tests through `display().to_string()` when correctness depends on path semantics rather than text rendering.

Prefer direct `Path` / `PathBuf` comparison for absolute paths, and normalize logical relative paths to a stable form when a textual comparison is still useful.

## Runtime observability for critical transitions

Critical transitions in the controlled runtime path should remain observable.

The `snapshot -> contract` step was a real example: the flow was working, but the transition was initially outside the minimal event model and therefore reduced traceability on failure.

## Project opening policy ownership

If project opening follows a reusable vertical convention, that convention should not remain retained in `cli_app`.

The entrypoint may still validate inputs and orchestrate the flow, but reusable project-opening policy belongs to the project vertical.

## Sandbox assets are staging, not runtime truth

The sandbox `assets/` folder is a manual staging area for cleanup, naming normalization, and candidate selection.

It is not the canonical runtime resource location. Governed runtime resources still belong in `resources/` when and if promotion is approved.

## Root docs should track runtime maturity

Root sandbox documents should reflect the real maturity of the Rust runtime path without overstating it.

When the sandbox gains real controlled runtime behavior, the docs should stop describing it as structure-only, while still making clear that it is not the final product runtime.

## Base path policy belongs in `workspace_core`

A Windows-specific path issue first appeared in higher crates, but the real policy boundary was the workspace layer.

Portable path behavior should be fixed in the base path layer rather than patched independently in each consuming crate.

Impact: once the base policy is corrected, `spec_runtime`, `project_runtime`, and their tests can consume a consistent path model instead of compensating locally.

## Relative workspace paths are logical portable paths

Workspace-relative paths represent logical system structure, not OS-native rendering.

Their portable form should stay stable across platforms and should not depend on native separator rendering.

Impact: portable metadata and tests can rely on one logical path form without treating Windows rendering as source of truth.

## Absolute Windows paths need robust equivalence

Absolute system paths on Windows may differ in textual rendering while still identifying the same location, especially with the `\\?\` prefix.

Comparisons for absolute paths should therefore use robust equivalence or small normalization rather than literal rendered strings.

Impact: workspace boundary checks and runtime-safe path assertions become less fragile on Windows without changing runtime behavior.

## Portable metadata should not depend on native path rendering

In `spec_runtime`, consumable metadata for declarative documents is part of the portable system model.

That metadata should expose a stable logical representation instead of depending on `display().to_string()`.

Impact: portable document descriptors remain stable across operating systems and are safer to use in tests and higher runtime layers.

## Phase closure requires mechanical validation

A phase should not be treated as formally closed on architecture alone.

Closure should wait for real mechanical validation such as `cargo check` and `cargo test`.

Impact: the sandbox keeps a clean distinction between architectural readiness and verified build/test closure.
