# workspace_layout

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Central layout

The central DocGraph layout is composed of:

- `Tree`
- `Chat`
- `Tabs Workspace`

Canonical structure:

`[ Tree ] [ Chat ] [ Tabs Workspace ]`

`Tabs Workspace` may contain:

- `Document Viewer`
- `Tools Panel`
- `Knowledge`
- `Sandbox View`, only when relevant

The viewer belongs to the tabs workspace as a rendered view.

Governed workspace tabs are identified by governed references and must be reused rather than duplicated for the same target.

The tree navigates.
The chat guides.
The tabs workspace hosts views.
The viewer renders content.

## Sandbox tab

Sandbox information MAY be exposed as a workspace tab when relevant.

The `Sandbox View` tab is conditional and must only appear when the project profile or project configuration declares sandbox relevance.

Typical relevant profile:

- `folder_organization_sandbox`

The `Sandbox View` tab is informational and declarative in F9.

It may show:

- sandbox state
- readonly source-folder reference state
- future working-copy state
- dry-run or proposal status if declared
- warnings about readonly source folder
- future actions as non-executable descriptions

It MUST NOT:

- copy files
- move files
- rename files
- delete files
- execute tools
- trigger write-back
- expose host-specific absolute paths as portable truth
- replace `Tools Panel`
- replace `Knowledge`
- become a filesystem runtime

## Invariants

- `INV-WORKSPACE-LAYOUT-001`: central layout MUST remain Tree + Chat + Tabs Workspace.
- `INV-WORKSPACE-LAYOUT-002`: viewer MUST be rendered inside Tabs Workspace.
- `INV-WORKSPACE-LAYOUT-003`: tabs host views; tabs do not execute logic.
- `INV-SANDBOX-VIEW-001`: Sandbox information MAY be exposed as a conditional workspace tab when relevant.
- `INV-SANDBOX-VIEW-002`: Sandbox view MUST remain declarative in F9.
- `INV-SANDBOX-VIEW-003`: Sandbox view MUST NOT execute filesystem operations.
- `INV-SANDBOX-VIEW-004`: Sandbox visibility MUST depend on project profile or governed project configuration.
- `INV-SANDBOX-VIEW-005`: Sandbox view MUST NOT replace Tools Panel, Knowledge, or project runtime.
