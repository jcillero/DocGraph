# sandbox_boundary_model

## Status

DECLARED_ONLY / F9-F11_PREP governance.

This document does not implement sandbox enforcement runtime, filesystem mutation, host access runtime, network access runtime, tool execution, or authorization.

## Purpose

Define a strict governed boundary model separating:

- `SANDBOX`
- `HOST`
- `EXTERNAL`

This model prepares future action constraints without enabling enforcement or execution now.

## Domains

Controlled domains:

- `SANDBOX`
- `HOST`
- `EXTERNAL`

### SANDBOX

Governed local app or project environment.

### HOST

Filesystem and local environment outside the governed sandbox.

### EXTERNAL

Network, internet, APIs, and remote services.

Rules:

- domain must be explicit when an action, tool, or capability needs access
- unspecified domain must not imply permission
- domain declaration does not authorize access

## SANDBOX domain

`SANDBOX` is the only future writable domain by default.

It may contain:

- `user/file_store/`
- project workspace
- `user/output/`
- future tool outputs

Rules:

- write access is future-governed and not active now
- delete access remains separately governed
- sandbox access does not imply project exposure
- `project_manifest` remains exposure authority

## HOST domain

`HOST` is anything outside the governed sandbox on the local machine.

Rules:

- no implicit host access
- future host access is read-only by default
- host write access is forbidden by default
- `HOST -> SANDBOX` import may be allowed only as an explicit future governed operation
- `SANDBOX -> HOST` export is forbidden by default unless separately gated

## EXTERNAL domain

`EXTERNAL` is network, internet, and remote API access.

Rules:

- no implicit network access
- future access requires `access_network` capability
- external access is disabled in the current phase
- credentials or secrets must not be inferred from environment
- no telemetry upload is implied

## Capability-domain alignment

Capability-domain alignment is governed conceptually as follows:

- `read_file`: `SANDBOX`, or controlled future `HOST` read
- `write_file`: `SANDBOX` only in a future governed phase
- `create_file`: `SANDBOX` only in a future governed phase
- `delete_file`: `SANDBOX` only and separately gated in a future phase
- `access_network`: `EXTERNAL` only, disabled now
- `call_external_binary`: future `SANDBOX`-scoped operation
- `call_llm`: provider boundary remains separately gated
- `generate_semantic_quads`: semantic-layer concern only; no filesystem mutation by itself
- `project_rdf`: future-only, non-active
- `analyze_graph`: future-only, non-active

Rules:

- capability declaration must include domain assumptions where relevant
- capability cannot expand domain by itself
- domain policy may further restrict capability use

## Action constraints alignment

Future `ActionIntent`, `ActionRequest`, or plan steps should declare:

- `required_domain`
- `access_level`
- `sandbox_scope`
- `host_access`
- `network_access`

Conceptual example:

```json
{
  "constraints": {
    "required_domain": "SANDBOX",
    "access_level": "read_write",
    "sandbox_scope": "project_only",
    "host_access": "none",
    "network_access": false
  }
}
```

Rules:

- constraints are declarative in this phase
- constraints are not enforced now
- constraints do not authorize execution
- `ActionIntent` and `ActionRequest` may carry these constraints in the future without becoming executable
- future `ResolutionCandidate` artifacts may summarize domain evaluation over these constraints without granting access
- future `AuthorizedExecutionRequest` artifacts may require `SANDBOX` scoped execution as a contract while still not granting runtime access
- future `SingleToolExecution` artifacts may declare `SANDBOX`-only first execution scope, but that declaration still does not grant runtime access or mutate filesystem
- future readonly presentation may summarize these constraints through `docs/specs/system_view.md` without granting permission

## F12.0 / F11.RUNTIME-0 sandbox proposal note

`F12.0 / F11.RUNTIME-0` is proposal-only.

The first possible runtime opening is constrained to one local deterministic `SANDBOX`-only tool.

It must not open:

- `HOST` write
- `EXTERNAL`
- network
- provider invocation
- external binary invocation
- multi-tool orchestration
- autonomous execution

`SANDBOX` scope in this proposal does not by itself grant filesystem mutation authority.

Any future output must be owner-scoped and governed by a later explicit implementation slice.

## Import/export model

Definitions:

- `Import`: explicit future operation bringing data from `HOST` to `SANDBOX`
- `Export`: explicit future operation sending data from `SANDBOX` to `HOST` or `EXTERNAL`

Rules:

- imports must be traceable
- exports are forbidden by default
- no implicit file copy is allowed
- no runtime import/export is implemented now

## Storage alignment

- `file_store` lives inside `SANDBOX`
- `file_store/blobs` remains physical authority
- `project_manifest` remains exposure authority
- host files are not part of `file_store` until explicitly imported
- external resources are not `StoredObject` values until explicitly imported or ingested in a future governed phase

## Non-goals

This policy does not open:

- filesystem mutation
- host write access
- network access
- tool execution
- execution layer
- active sandbox enforcement

## Sandbox boundary invariants

- `INV-SANDBOX-001`: `SANDBOX` is the only future writable domain by default.
- `INV-SANDBOX-002`: there is no implicit `HOST` access.
- `INV-SANDBOX-003`: there is no implicit `EXTERNAL` or network access.
- `INV-SANDBOX-004`: `HOST` write is forbidden by default.
- `INV-SANDBOX-005`: `EXTERNAL` access requires explicit future capability and gate.
- `INV-SANDBOX-006`: capability declaration does not expand domain access.
- `INV-SANDBOX-007`: future `ActionIntent` and `ActionPlan` constraints must declare domain needs.
- `INV-SANDBOX-008`: `project_manifest` remains exposure authority.
- `INV-SANDBOX-009`: import and export are future governed operations and are not active behavior.
- `INV-SANDBOX-010`: sandbox policy does not activate enforcement runtime.

## Related specs

- `docs/specs/local_sandbox_context.md`
- `docs/specs/tool_capability_model.md`
- `docs/specs/action_resolution.md`
- `docs/specs/storage_policy.md`
- `docs/specs/system_view.md`
