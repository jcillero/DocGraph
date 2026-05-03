# structured_content_assets

## Purpose

Document the future treatment of figures, tables, and equations as governed structured content.

This is a proposal only. It does not design a renderer, export pipeline, or runtime implementation.

## Core idea

Markdown artifacts may contain references to structured assets.

The structured assets live beside the artifact or inside governed project content with their own manifests.

Future structured asset classes may include:

- figures
- tables
- equations

## Manifest ownership

Each structured asset should have a manifest that records:

- stable identifier
- source relation
- artifact relation
- editable metadata fields
- technical readonly fields
- provenance

The manifest is the governed contract for inspection and patch validation.

## Editable metadata

Editable fields may include:

- title
- caption
- alt text
- notes
- display label
- logical role

Edits to these fields must use governed patch proposals.

## Technical readonly fields

Technical fields are not directly editable through the future document editing workflow.

Examples:

- content hash
- binary path
- source extraction coordinates
- page origin
- generated dimensions
- derived text checksum

## Markdown references

A future artifact may reference structured assets from Markdown rather than embedding all structured content inline.

Possible reference kinds:

- figure reference
- table reference
- equation reference

The exact syntax is intentionally out of scope for this phase.

## Phase limits

This spec does not implement:

- structured asset renderer
- editor surface
- export runtime
- equation compiler
- table editor
- figure processing pipeline

It records only the governance principle for future phases.
