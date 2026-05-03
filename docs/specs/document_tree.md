# document_tree

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Document Tree` / `Arbol de Documentos` surface for DocGraph.

This spec fixes what the tree is allowed to expose, how nodes are described, and how the tree stays separated from intake, chat, tabs, and viewers.

## Normative definition

`Document Tree` is a governed view of objects exposed by `project_manifest`.

It does not scan the filesystem.
It does not decide visibility by itself.
It does not import documents.
It does not execute actions.

`project_runtime` prepares the surface.
The tree renders and routes governed references only.

## Core rule

The tree is not a free filesystem explorer.

It is a navigation and selection surface for already governed project objects.

The tree must remain distinct from:

- `Clip Panel`, which is reserved for external intake
- `Chat Panel`, which coordinates references and instructions
- `Workspace Tabs`, which host controlled views
- `Readonly Viewer`, which renders governed content

## Relationship to authority

- `project_manifest` governs exposure
- `project_runtime` prepares surface state
- `registry.json` may accelerate navigation but is not authority
- `graph/` may explain relations but does not decide visibility

## Node kinds

The tree may expose these conceptual node kinds:

- `project_root`
- `documents_group`
- `knowledge_group`
- `document_holder`
- `source_document`
- `derived_document`
- `artifact_document`
- `sandbox_context`
- `tool_output_reference`
- `folder_group`

Interpretation:

- `project_root`: top logical node for the governed project surface
- `documents_group`: grouping node for project document domains
- `knowledge_group`: grouping node for governed knowledge items
- `document_holder`: owned documentary grouping context
- `source_document`: governed source document reference
- `derived_document`: governed derived document reference
- `artifact_document`: governed artifact-backed document reference
- `sandbox_context`: governed sandbox context or sandbox visibility node
- `tool_output_reference`: governed tool-output reference when exposed through the tree
- `folder_group`: logical grouping node only, not an arbitrary host folder

## TreeNode conceptual contract

Minimum conceptual fields:

- `node_id`
- `node_kind`
- `display_name`
- `ref`
- `family`
- `document_class`, when applicable
- `is_selectable`
- `is_viewable`
- `is_readonly`
- `status`
- `opens_tab_kind`

Field meaning:

- `node_id`: stable tree-node identity for prepared UI state
- `node_kind`: one of the governed node kinds declared in this spec
- `display_name`: prepared visible name
- `ref`: governed reference for the represented object, if any
- `family`: active-object family used by `ActiveObjectContext`
- `document_class`: source, derived, artifact, or other declared class when the node represents a document
- `is_selectable`: whether the node may participate in governed selection
- `is_viewable`: whether the node may open or activate a view
- `is_readonly`: whether the represented object is readonly in the current phase or policy
- `status`: prepared visual state
- `opens_tab_kind`: governed workspace tab kind that may be activated or opened

## Visual states

Prepared node state may include:

- `ready`
- `readonly`
- `missing`
- `stale`
- `locked`
- `future_actionable`

Interpretation:

- `ready`: node is valid and available for governed viewing or selection
- `readonly`: node is valid but not mutable in current phase or policy
- `missing`: exposed reference no longer resolves
- `stale`: reference exists but prepared state is no longer current enough
- `locked`: node is present but not currently available for the intended action
- `future_actionable`: node may become actionable only in a future governed phase

## Allowed actions

The tree may allow:

- expand
- collapse
- select
- activate or open the corresponding governed tab
- copy a structured reference
- send a structured reference to chat as context
- show metadata

These are governed navigation and reference actions only.

## Forbidden actions

The tree must not allow:

- edit
- delete
- move
- rename
- import
- scan folders
- execute tools
- mutate `SOURCE`
- mutate `DERIVED`

The tree must not become a filesystem runtime or mutation surface.

## Relationship to ActiveObjectContext

Tree selection prepares `ActiveObjectContext`.

The tree may select governed artifacts by family, not as one single global active object.

Left-click on a selectable artifact toggles selection inside its family.

Rules:

- clicking a non-selected artifact selects it in its family and replaces the previous selection of that family
- clicking an already selected artifact deselects it
- clicking an artifact from another family keeps other-family selections intact
- every successful selection change updates `focused_family` to the clicked family
- deselecting the currently focused family sets `focused_family` to `None`

Middle-click may request opening or activating the corresponding governed workspace tab without duplicating an existing tab for the same governed reference.

## Relationship to workspace tabs

The tree may request opening or activating a governed workspace view appropriate for the node kind.

Examples:

- `source_document`, `derived_document`, and `artifact_document` may open `viewer`
- `sandbox_context` may open `sandbox_view`
- `tool_output_reference` may open `tool_result` or another governed output view when declared

Opening or activating a tab does not clear selections in other families.

Opening the same governed object must reuse the existing workspace tab instead of creating duplicates.

## Relationship to Readonly Viewer and Sandbox View

`Readonly Viewer` shows governed document content.

`Sandbox View` shows sandbox context.

The tree does not render documents itself and does not replace either view.

## Relationship to Clip Panel

`Clip Panel` remains reserved for external intake.

The tree must not import, attach, or stage external files. It only exposes already governed objects.

## Relationship to Chat Panel

The tree may provide structured references to chat.

The tree does not authorize chat actions, interpret user intent, or execute operations.

## Failure modes

- `ref_not_exposed_by_manifest`
- `node_ref_missing`
- `node_stale`
- `invalid_node_kind`
- `filesystem_scan_detected`

Interpretation:

- `ref_not_exposed_by_manifest`: the prepared node points to a reference that is not governed by manifest exposure
- `node_ref_missing`: the node reference no longer resolves
- `node_stale`: the node exists but its prepared state is no longer current enough
- `invalid_node_kind`: prepared state contains an unsupported or invalid kind
- `filesystem_scan_detected`: a non-governed host scan was attempted instead of manifest-driven exposure

## Invariants

- `INV-DOCTREE-001`: `Document Tree` MUST remain a governed view of objects exposed by `project_manifest`
- `INV-DOCTREE-002`: `Document Tree` MUST NOT scan the filesystem
- `INV-DOCTREE-003`: `Document Tree` MUST NOT decide visibility by itself
- `INV-DOCTREE-004`: `Document Tree` MUST NOT import documents or external files
- `INV-DOCTREE-005`: `Document Tree` MUST NOT execute tools or mutations
- `INV-DOCTREE-006`: tree selection MUST prepare governed selection context, not execution
- `INV-DOCTREE-007`: tree selection MUST preserve other-family selections unless future governed policy states otherwise
- `INV-DOCTREE-008`: the tree MUST remain separate from `Clip Panel`, `Chat Panel`, `Workspace Tabs`, and `Readonly Viewer`

## Related specs

- `docs/specs/active_object_context.md`
- `docs/specs/active_context.md`
- `docs/specs/workspace_tabs.md`
- `docs/specs/local_sandbox_context.md`
- `docs/specs/gui_objects_v1.md`

## F12.4 file intake tree alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

The document tree must not infer project documents from passive filesystem presence.

Future intake items may appear only as exposed or governed prepared presentation state according to `project_manifest.json` and file-intake rules.

The document tree must not import, copy, classify, expose, derive, register, or execute tools.

## F12.5 file intake tree plan

`F12.5` is plan-only and adds no tree behavior.

Future `F12.6` must not make newly imported or candidate intake files visible in the tree unless a separate exposure slice explicitly opens `project_manifest.json` governance for them.

The tree may later present prepared intake state only when derived from governed metadata, not from filesystem scanning.

## F13.0 project exposure gate tree alignment

`F13.0` is SPEC-only and adds no tree behavior.

After a future governed implementation writes a project exposure entry to `project_manifest.json`, `project_runtime` may prepare document tree nodes for the exposed object.

Tree visibility rules:

- visibility is manifest-driven only
- `imported_not_exposed` items must not appear as project-exposed document nodes
- blocked and unsupported intake items must not appear as project-exposed document nodes
- duplicate exposed content must preserve distinct governed refs when manifest entries are distinct
- reused physical blobs must not collapse tree identity
- tree labels must be sanitized presentation labels
- tree presentation remains readonly unless a later editing phase opens mutation

The document tree must not scan `file_store`, scan intake metadata, generate registry entries, write graph entries, create derivatives, call `document_text_runtime`, call `tool_runtime`, or infer project visibility from filesystem presence.

## F13.1 legacy tree-bypass hardening

`F13.1` is SPEC-only and adds no tree behavior.

The document tree must not use any legacy helper as project visibility authority:

- `list_project_documents` must not remain a production visibility source once `F13` runtime work begins
- copied files in project folders must not appear as exposed merely because they are present on disk
- chat resources must not appear as project tree nodes unless later imported and exposed through the governed path
- derivatives must not create or justify tree visibility

Any temporary legacy listing must be treated as fixture-only or migration-only and must not be interpreted as canonical project exposure.

## F13.4 manifest exposure tree alignment

`F13.4` is SPEC-only and adds no tree behavior.

The tree may only show project artifacts backed by manifest authority and, in a later explicit runtime slice, by a governed `ManifestExposureEntry`.

Tree hardening rules:

- accepted confirmation alone must not make an item visible in the tree
- intake history/index rows must not be consumed as tree authority
- `System View` exposure summaries must not be consumed as tree authority
- duplicate blob reuse must not collapse distinct tree identity when future manifest entries remain distinct
- tree visibility must stay manifest-driven even when storage, registry, graph, or observability metadata already exists

## F13.5A manifest exposure runtime tree checklist

`F13.5A` is SPEC-only and adds no tree behavior.

Future `F13.5B` runtime checklist for tree alignment:

- manifest exposure may prepare future tree visibility only after a valid `ManifestExposureEntry` exists
- tree visibility must remain downstream of manifest authority, never a co-authority
- failed or blocked exposure attempts must not create tree nodes
- duplicate blob reuse must not collapse distinct tree identity when distinct manifest entries exist
