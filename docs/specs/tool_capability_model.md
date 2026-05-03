# tool_capability_model

## Status

DECLARED_ONLY / F9-F11_PREP governance.

This document does not implement runtime capability resolution, authorization, dispatch, tool execution, provider invocation, runners, or UI authority.

## Purpose

Define a governed capability-based model for tools that separates:

- `ToolIdentity`
- `ToolCapability`
- `CapabilitySurface`
- `CapabilityRequirement`
- future authorization

The goal is to make future execution governable by capabilities rather than by direct tool names alone.

## ToolIdentity

`ToolIdentity` is stable tool identity.

Conceptual structure:

```json
{
  "tool_id": "pdf_merge",
  "tool_kind": "operational | llm | external | agent | base",
  "implementation_ref": "...",
  "visibility": "visible | hidden | internal"
}
```

Rules:

- tool identity does not imply permission
- tool presence does not imply execution
- tool visibility does not imply authorization
- `tool_id` is not an execution grant

## ToolCapability

`ToolCapability` is an atomic governable ability.

Initial controlled vocabulary:

- `read_file`
- `write_file`
- `create_file`
- `delete_file`
- `read_metadata`
- `write_metadata`
- `extract_text`
- `merge_documents`
- `validate_json`
- `measure_text`
- `call_llm`
- `call_external_binary`
- `access_network`
- `generate_semantic_quads`
- `project_rdf`
- `analyze_graph`

Rules:

- capabilities are declarative
- capabilities do not execute
- capabilities may be required by future `ActionIntent`, `ActionRequest`, or plan steps
- capability presence does not imply authorization
- capability declaration does not expand domain access by itself

## CapabilitySurface

`CapabilitySurface` is the readonly declared capability set exposed by a tool.

Conceptual structure:

```json
{
  "tool_id": "pdf_merge",
  "capabilities": [
    "read_file",
    "write_file",
    "merge_documents"
  ],
  "surface_status": "declared | implemented | disabled | future"
}
```

Rules:

- capability surface is readonly
- capability surface is not authorization
- effective surface may be filtered by policy in the future
- `disabled` or `future` capabilities must not be selected for execution

## CapabilityRequirement

`CapabilityRequirement` is the capability set needed by an action, request, or plan step.

Conceptual structure:

```json
{
  "required_capabilities": [
    "read_file",
    "write_file"
  ],
  "required_domain": "SANDBOX | HOST | EXTERNAL | UNSPECIFIED",
  "permission_hint": "read | write | delete | network | execute"
}
```

Rules:

- future actions reference required capabilities
- tools are candidate providers only
- requirement matching does not execute anything
- requirement matching does not authorize anything
- the canonical domain vocabulary is uppercase:
  - `SANDBOX`
  - `HOST`
  - `EXTERNAL`
  - `UNSPECIFIED`

## Tool kinds

Controlled `tool_kind` values:

- `operational`
- `llm`
- `external`
- `agent`
- `base`

Rules:

- `base` tools may be hidden or internal
- `external` tools require dependency governance
- `agent` tools are composite and future-only unless separately opened
- `llm` tools do not execute directly
- `operational` tools may become future execution candidates only through governed action flow
- `tool_kind` is the canonical governed classification term
- legacy `tool_class` references, if present in older documentation, are deprecated aliases only

## Authorization boundary

Core distinction:

`capability declared != capability authorized`

Future authorization may consider:

- sandbox scope
- user confirmation
- project policy
- tool policy
- security and sanitization policy
- cost policy
- credentials policy
- capability status

Domain boundaries are governed separately by `docs/specs/sandbox_boundary_model.md`.

But in the current phase:

- no authorization runtime is opened
- no capability resolver is implemented
- no execution is selected

## Relationship with current specs

Catalogs declare tools and possible capabilities.

The capability model defines governable abilities.

`ActionResolution` may use capability requirements in the future.

`ActionIntent` and `ActionRequest` may use capability requirements before any future candidate-tool selection.

Future `ResolutionCandidate` artifacts may summarize capability evaluation results, but capability authority remains owned by this spec.

Future `AuthorizedExecutionRequest` artifacts may carry required capabilities in an execution-scope summary, but that summary does not grant execution and does not replace capability governance.

Future `SingleToolExecution` artifacts may bind only `operational` or `base` tool kinds for the first declared execution-contract scope.

That binding is contractual only: it does not invoke the tool, select execution at runtime, load binaries, or grant authority.

UI may present capabilities in the future, but it must not authorize them.

Future readonly presentation may summarize effective capability and tool-surface state through `docs/specs/system_view.md` without granting authority.

## Non-goals

This policy does not open:

- tool execution
- tool dispatch
- runners
- provider invocation
- `project_runtime` authority changes
- UI authorization

## Capability invariants

- `INV-CAP-001`: tools do not imply authorization.
- `INV-CAP-002`: capabilities are declarative.
- `INV-CAP-003`: capability surface is readonly.
- `INV-CAP-004`: future actions reference capabilities before tools.
- `INV-CAP-005`: capability declaration does not grant execution.
- `INV-CAP-006`: external tools require explicit future dependency governance.
- `INV-CAP-007`: agent tools are composite and non-executing unless separately opened.
- `INV-CAP-008`: disabled or future capabilities cannot be selected for execution.
- `INV-CAP-009`: UI must not authorize capabilities.
- `INV-CAP-010`: `project_runtime` remains unchanged unless a future phase explicitly opens authority.
- `INV-CAP-011`: `tool_kind` is the canonical governed classification term; `tool_class` is a deprecated alias only if retained in historical references.
- `INV-CAP-012`: the canonical domain enum is uppercase across action, capability, and sandbox contracts.

## Related specs

- `docs/specs/tools_catalogs.md`
- `docs/specs/tool_implementation_governance.md`
- `docs/specs/action_resolution.md`
- `docs/specs/action_contract.md`
- `docs/specs/security_sanitization_policy.md`
- `docs/specs/system_observability.md`
- `docs/specs/system_view.md`
