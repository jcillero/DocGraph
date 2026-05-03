# local_sandbox_context

## Purpose

Define the future authorized local sandbox context for file operations outside the application workspace.

This is declarative only. No filesystem sandbox runtime is implemented here.

Domain separation between `SANDBOX`, `HOST`, and `EXTERNAL` is governed by `docs/specs/sandbox_boundary_model.md`.

## Scope F9

F9 declares policy and user preference placeholders for future authorized sandbox roots.

A valid sandbox is:

- local
- external to the app
- new or empty at authorization time
- explicitly authorized by the user
- not inside `crates`, `resources`, `runtime`, `user`, `output`, `assets`, `dev`, `docs`, or `scripts`

Current enabled actions are readonly or dry-run:

- `read_tree`
- `dry_run_plan`

Mutable actions remain disabled.

The declared sandbox remains the only future writable domain by default.

`HOST` and `EXTERNAL` access remain separately governed and non-active here.

## Forbidden responsibilities

The sandbox context must not:

- allow mutation outside the sandbox
- allow direct LLM execution
- enable mutable actions now
- create hidden logs
- use absolute hardcoded roots
- become a project pipeline

## Future F10/F11 notes

F10 may use this context for governed tool planning if explicit execution is opened.

F11 should audit JSONL logging, authorization, root validation, and mutation gates before enabling write actions.

## Folder organization sandbox profile

- `folder_organization_sandbox` may declare a user-selected source folder.
- The source folder is readonly.
- Mutations are allowed only on a future sandbox working copy.
- F9 does not implement copy, mutation, write-back, or sandbox runtime.
- Write-back to the original folder is a future high-risk action that requires confirmation and trace.
- Sandbox context may be represented in the tabs workspace as a conditional `Sandbox View`.
- `Sandbox Viewer` is the governed inspection surface for that context.
- Sandbox selection may populate the `Sandbox` family in `ActiveObjectContext`.
- In F9 it is declarative only.
- The original selected source folder remains readonly.
- Mutations are future-only and only on sandbox working copy when governed runtime exists.

See `docs/specs/project_profiles.md` for folder sandbox invariants.
