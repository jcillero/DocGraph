# LLM Runtime Policy

This sandbox inherits the live Python LLM doctrine.

Supported execution modes:

- `local`
- `cloud`
- `auto`

## Deterministic resolution

The runtime layer must resolve an execution plan before invocation.

Inherited precedence:

`explicit args -> user preferences -> system config -> defaults`

No runtime ambiguity is allowed.

## Layer responsibilities

- `llm_core` defines contracts, policy, mode selection, model catalog rules, tool request governance, and observability requirements
- `llm_local` implements local adapters behind `llm_core`
- `llm_cloud` implements cloud adapters behind `llm_core`

## Required doctrine

- application code must not depend directly on a provider
- providers stay behind governed adapter crates
- model catalog governance is mandatory once real execution starts
- tools may be requested by the model, but they are executed by the system
- runtime policy must remain observable and traceable
- the runtime layer must not execute tools directly
- the runtime layer must not contain UI logic

## Observability baseline

Real execution is deferred, but the inherited requirement is already fixed:

- enough structured metadata must exist to reconstruct provider, model, mode, tool policy, and outcome
