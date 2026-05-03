# project_folder_layout

## Status

NORMATIVE DOCUMENTATION. This file defines the fundamental DocGraph project folder contract. It does not implement runtime behavior.

## Purpose

Define the governed physical layout for a DocGraph project.

The project is the user's work boundary. It scopes project configuration, knowledge files, chats, documents, allowed tools, artifact relations, and traceability.

The governed project pipeline remains:

`project_root -> manifest -> contract -> validation -> surface -> resolve -> output`

`project_runtime` governs. The filesystem organizes. The manifest exposes. The graph explains relations. Folders must not reinterpret the manifest.

## Fundamental tree

```text
project_root/
  project_manifest.json
  registry.json
  config/
    project_config.json
    llm_policy.json
    tool_policy.json
    preferences.json
  knowledge/
    manifests/
    files/
    derived/
      text/
      chunks/
    indexes/
    semantic/
      proposals/
  chats/
    chat_sessions/
      <chat_id>/
        manifests/
          chat_manifest.json
          context_refs.json
        messages.jsonl
        attachments/
          files/
          figures/
        artifacts/
        tool_outputs/
          <tool_id>/
            <run_id>/
              tool_run_manifest.json
              outputs/
              logs/
  documents/
    <document_holder>/
      document.md
      manifests/
        document_manifest.json
        source_context.json
      template/
        template_snapshot.json
        template_overrides.json
        effective_template.json
      references/
      assets/
      lifecycle/
        status.json
        review_log.jsonl
      tool_outputs/
        <tool_id>/
          <run_id>/
            tool_run_manifest.json
            outputs/
            logs/
      derived/
        latex/
        docx/
        pdf/
  graph/
    graph_manifest.json
    artifact_nodes.jsonl
    artifact_edges.jsonl
    snapshots/
  logs/
    project_events.jsonl
```

## Normative decisions

- No `outputs/` folder exists at project root.
- Every tool output must have an `owner_ref`.
- Tool outputs live inside their functional owner.
- Chat-owned tool outputs live under `chats/chat_sessions/<chat_id>/tool_outputs/`.
- Document-owned tool outputs live under `documents/<document_holder>/tool_outputs/`.
- Knowledge-owned derivations live under `knowledge/derived/` or `knowledge/semantic/proposals/`.
- If a tool output contributes to or is used by a `DocumentHolder`, it MUST be stored under the `DocumentHolder` `tool_outputs/`, even if derived from knowledge.
- Knowledge-derived outputs are only valid in `knowledge/derived/` when they are not owned by a document.
- If a tool action has no `owner_ref`, the action is invalid or pending clarification.
- `DocumentHolder` is the document production unit.
- `document.md` is the editable document source.
- `derived/` contains regenerable products: `latex/`, `docx/`, and `pdf/`.
- Chat `attachments/` are user-provided inputs.
- Chat `artifacts/` are session-owned products.
- `knowledge/files/` contains project sources.
- `knowledge/derived/` contains regenerable derivatives.
- `knowledge/semantic/proposals/` contains semantic proposals, not approved facts.
- `graph/` contains relations between artifacts, not source files.
- `registry.json` is a fast navigation index. It does not replace `project_manifest.json` or `graph/`.
- `registry.json` MUST be derivable from `project_manifest.json` and MUST NOT introduce independent state.
- `registry.json` is a navigation accelerator and cannot be a source of truth.
- `manifests/` groups governed metadata inside each domain.
- `lifecycle/` exists only for DocumentHolder state and review.
- `project_profile` is stored as declarative project configuration.
- `config/preferences.json` stores non-secret preferences.
- `llm_policy.json` and `tool_policy.json` may refer conceptually to `credential_ref`.
- Project files MUST NOT store credential secret values.
- Profile selection does not alter the normative folder layout.
- Future sandbox working copy location must not create project-root `outputs/`.

## Invariants

- `INV-PROJ-LAYOUT-001`: `project_root` MUST NOT contain a root `outputs/` directory.
- `INV-PROJ-LAYOUT-002`: every durable artifact MUST have stable id, `owner_ref`, and trace origin.
- `INV-PROJ-LAYOUT-003`: `registry.json` MUST be derivable from `project_manifest.json` and MUST NOT introduce independent state.
- `INV-PROJ-LAYOUT-004`: filesystem presence MUST NOT imply project exposure.
- `INV-PROJ-LAYOUT-005`: project exposure MUST be governed by `project_manifest.json`, not folder scanning.
- `INV-CONSISTENCY-001`: any artifact referenced by another artifact MUST exist, be resolvable, and be exposed by `project_manifest.json`.
- `INV-ID-001`: artifact IDs MUST be stable across moves and renames and MUST NOT depend on filesystem path.
- The filesystem stores physical location.
- The manifest governs exposure.
- `registry.json` accelerates navigation and cannot be a source of truth.
- `graph/` explains relations.
- `graph/` does not decide, execute, or approve.
- `graph/` does not replace `project_runtime`.
- Semantic relations remain proposals until human review.
- No RDF, Oxigraph, SPARQL, embeddings, or semantic store is introduced by this layout.
- No graph runtime is implied.
- No tool execution is implied.
- No UI logic is implied.
- No hardcoded absolute paths are allowed.
- No duplicate project pipeline is allowed.

## Related specs

- `docs/specs/project_runtime.md`
- `docs/specs/artifact_graph.md`
- `docs/specs/document_package.md`
- `docs/specs/chat_document_flows.md`
- `docs/specs/tools_panel.md`

## F12.4 file intake layout alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

F12.4 does not define or create a final runtime intake layout.

Future intake must align with this project folder layout, must not use project-root `outputs/`, must not use `assets/` as runtime storage, and must not treat filesystem presence as project exposure.

## F13.4 manifest exposure layout alignment

`F13.4` is SPEC-only and adds no runtime layout mutation.

Future project exposure may only become visible through a governed `project_manifest.json` declaration containing portable sanitized refs.

Layout rules:

- `project_manifest.json` remains the only exposure authority at project level
- no blob path, copied file path, registry row, graph row, or tree node path may stand in for a `ManifestExposureEntry`
- future manifest exposure entries must not persist private absolute host paths
- future manifest exposure entries must not create project-root outputs or imply derivative folders
- folder presence remains organizational only and must not reinterpret manifest-governed exposure

`project_manifest.json` remains the authority for exposed project content.

## F12.5 file intake layout plan

`F12.5` is plan-only and does not create folders.

Future `F12.6` may use only an owner-scoped intake directory or governed file-store location already allowed by storage policy.

Future `F12.6` must keep copied or candidate files away from project-root outputs, `assets/`, source folders, registry locations, graph locations, and derivative locations unless a later explicit phase opens those targets.
