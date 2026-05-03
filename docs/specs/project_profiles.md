# project_profiles

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define declarative project profiles for DocGraph.

## Core rule

`project_profile` declares intended capabilities and default policies.

`project_profile` does not execute anything.
`project_profile` does not enable runtime by itself.

## Required profile field

Every project must declare:

- `project_profile`

## Initial project profiles

- `document_engineering`
- `knowledge_analysis`
- `code_sandbox`
- `folder_organization_sandbox`

## Profile: document_engineering

Purpose:

- technical report creation and analysis
- `DocumentHolder`
- references
- future export

It does not imply export runtime.

## Profile: knowledge_analysis

Purpose:

- analysis of knowledge files
- reading, references, and future semantic proposals

It does not imply embeddings, RDF, Oxigraph, or real LLM execution.

## Profile: code_sandbox

Purpose:

- sandbox project oriented to programming and code
- future controlled code analysis and execution

F9:

- `declared_only`
- no code execution
- no filesystem mutation
- no external runtime execution

## Profile: folder_organization_sandbox

Purpose:

- the user selects a folder from the personal PC
- the original selected folder is treated as readonly
- DocGraph prepares a future working copy inside project sandbox scope
- future tools or LLM may propose organization
- future mutations apply only to the working copy inside sandbox scope

Critical rule:

Original selected folder = readonly.  
Sandbox working copy = mutable only when a future governed execution phase is opened.

This profile may cause the workspace to expose a `Sandbox View` tab.

The tab is visibility and context only in F9.

Profile selection does not enable filesystem mutation.

## folder_organization_sandbox conceptual flow

1. user selects source folder
2. DocGraph records explicit access intent
3. source folder is read in readonly mode
4. future working copy is created in project sandbox scope
5. future tool or LLM proposes organization plan
6. future mutations apply only to working copy
7. user reviews result
8. any write-back to source requires separate future action, explicit confirmation, and trace

## folder_organization_sandbox policy sketch

The following example is conceptual only. It does not execute anything.

```json
{
  "project_profile": "folder_organization_sandbox",
  "source_access": {
    "kind": "explicit_user_selected_folder",
    "mode": "readonly",
    "persist_absolute_path": false
  },
  "sandbox_policy": {
    "enabled": true,
    "mutation_scope": "project_sandbox_working_copy_only",
    "allowed_actions_f9": [
      "declare_source_folder",
      "dry_run_plan"
    ],
    "future_actions": [
      "read_tree",
      "copy_into_sandbox",
      "rename_in_sandbox",
      "move_in_sandbox",
      "deduplicate_in_sandbox",
      "export_plan_to_source"
    ]
  }
}
```

## Invariants

- `INV-PROJECT-PROFILE-001`: every project MUST declare a `project_profile`.
- `INV-PROJECT-PROFILE-002`: `project_profile` declares intended capabilities and default policies, not runtime authority.
- `INV-PROJECT-PROFILE-003`: changing `project_profile` MUST NOT enable execution by itself.
- `INV-FOLDER-SANDBOX-001`: the user-selected source folder MUST be treated as readonly unless a future explicit export or write-back phase is opened.
- `INV-FOLDER-SANDBOX-002`: folder organization mutations MUST operate only on the sandbox working copy, never on the original selected source folder.
- `INV-FOLDER-SANDBOX-003`: host-specific absolute source paths MUST NOT become portable project truth.
- `INV-FOLDER-SANDBOX-004`: LLM output may propose an organization plan, but only governed tools may materialize changes inside the sandbox.
- `INV-FOLDER-SANDBOX-005`: any future write-back from sandbox to source folder MUST require explicit user confirmation and trace.
- `INV-FOLDER-SANDBOX-006`: sandbox working copy MUST be distinguishable from original source folder in manifests and trace.

## Forbidden responsibilities

`project_profiles` must not:

- implement sandbox
- implement folder copy
- execute code
- execute tools
- call LLM
- mutate source folder
- create graph runtime
- reinterpret `project_manifest`
- duplicate `project_runtime`
