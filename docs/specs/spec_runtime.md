# spec_runtime

## Purpose

Load and interpret active specs, contracts, config, and registries through inherited declarative-first rules.

## Responsibilities

- resolve active spec and config locations under governed workspace rules
- load declarative documents and machine-readable contracts
- expose normalized lookup surfaces for higher runtime layers
- preserve source provenance for traceability

## Inputs

- workspace root
- target spec, config, registry, or contract identifiers
- document paths governed by active sandbox rules

## Outputs

- normalized spec/config payloads
- source provenance metadata
- validation diagnostics for missing or incompatible declarations

## Allowed Dependencies

- standard library
- `workspace_core`
- `io_runtime`
- `core_domain`

## Forbidden Responsibilities

- no UI logic
- no direct project business logic
- no tool execution
- no provider invocation
- no mutation of normative documents

## Initial Phase Scope

- loading active sandbox docs, specs, and resource contracts
- normalized access to declarative runtime inputs
- no full schema engine yet
