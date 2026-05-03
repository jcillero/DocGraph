# workspace_core

## Purpose

Provide the inherited workspace boundary layer for the Rust successor.

## Responsibilities

- represent portable workspace roots and project-scoped roots
- preserve `system/`, `dev/`, and `user/` separation
- normalize and validate workspace-scoped relative paths
- enforce scope control for project-safe path handling

## Inputs

- portable app root
- requested workspace-relative or project-relative paths
- project identifiers or project root descriptors

## Outputs

- normalized workspace descriptors
- validated project-scoped path descriptors
- scope validation results and boundary errors

## Allowed Dependencies

- standard library
- `core_domain`

## Forbidden Responsibilities

- no UI logic
- no LLM logic
- no spec parsing
- no business workflow orchestration
- no arbitrary filesystem discovery as architecture source of truth

## Initial Phase Scope

- root and scope descriptors
- relative path discipline
- project root validation
- portable workspace boundary rules
