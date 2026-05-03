# chat_document_flows

## Purpose

Define the minimum governed interaction model between tree, clip, chat, and workspace tabs.

## Core rule

Tree = existing governed documents  
Clip = external intake and workflow launch  
Chat = structured references and results  
Workspace tabs = where work actually happens

Onboarding chat = ephemeral help context, not project chat

Future assisted chat must remain Lume-fronted.

All user-facing chat interaction enters and exits through Lume.

Internal Planner, Specialist, and Synthesizer roles are conceptual routing and synthesis roles only; they are not user-facing chat participants.

## Chat session folder contract

```text
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
```

`attachments/` contains user-provided inputs for the chat session.  
`artifacts/` contains products owned by the chat session.  
`tool_outputs/` contains chat-owned tool outputs and requires an `owner_ref` pointing to the chat session.

## Invariants

- `INV-CHAT-001`: chat references, not blobs
- `INV-CHAT-002`: existing documents are referenced from the tree
- `INV-CHAT-003`: clip is for external intake and workflow initiation
- `INV-CHAT-004`: profiling is a workspace workflow
- `INV-CHAT-005`: viewer remains readonly
- `INV-CHAT-006`: no duplicate source truth is hidden inside chat
- `INV-CHAT-007`: attachment intent must be explicit
- `INV-CHAT-008`: clip-driven tool launch is operational only
- `INV-CHAT-009`: multi-file selection is controlled and intention-scoped
- `INV-CHAT-010`: project and runtime boundaries remain intact
- `INV-CHAT-011`: point of entry does not redefine the governed documentary workflow
- `INV-CHAT-012`: the viewer remains readonly whether a workflow starts from the tree or from the clip
- `INV-CHAT-013`: chat attachments are session context, not stable project knowledge
- `INV-CHAT-014`: chat artifacts become durable knowledge or document inputs only through explicit promotion
- `INV-CHAT-015`: promotion from chat MUST preserve source chat, message refs, artifact refs, and trace

## Allowed chat payloads

- text
- structured document references
- structured tool results
- system / state messages
- future structured edit requests with referenced selection, active context, and user instruction
- future patch proposal references, previews, and acceptance decisions

## Relationship to active-object context

Chat may consume `ActiveObjectContext` as prepared governed target context.

Rules:

- `focused_family` determines the default target family when the instruction needs one
- if no valid focused family exists, chat must ask for clarification
- if the instruction explicitly names a family, that family may be used only when a valid selection exists
- chat must not invent missing context
- chat must not use the last opened object as fallback

Onboarding chat may carry temporary help text only. It must not persist as project chat or become document storage.

## Future governed LLM request intake

Future LLM-facing chat requests may enter the governed pipeline as `UserInput`.

The user-facing entrypoint is Lume.

Conceptual agent flow:

```text
User
<-> Lume
-> Planner
-> Specialists
-> Synthesizer
-> Lume
-> User
```

That intake may produce:

- `IntentAnalysis`
- `RequestDecomposition`
- `SubRequestPlan[]`

Rules:

- decomposition must preserve user wording references
- ordering must be preserved when order matters
- chat must not execute subrequests
- chat must not bypass governed context requirements
- chat may surface clarification needs explicitly
- chat must not expose internal agents as independent user-facing assistants
- chat must not embed or mutate agent prompts

If a subrequest becomes a `tool_candidate` or executable action candidate, it remains proposal-only until future governed action flow resolves it.

Agent prompts for future conceptual roles are declarative resources governed by `docs/specs/llm_agent_prompts.md`.

## Future referenced editing requests

Future editing requests may enter chat as structured payloads derived from viewer selection or an inspector context.

The payload must reference the target context rather than store a source blob. The LLM may propose a patch, but the system validates it and applies it only after user acceptance.

Chat remains the request and reference surface. It does not become a document store, editor, patch runtime, or filesystem authority.

## Future template proposals

Future LLM-assisted template work may appear in chat as structured `TemplateProposal` or `TemplateDraft` references.

The LLM may suggest creating, editing, renaming, cloning, or deleting a template, but it must not open the template popup directly, execute the operation, or write template files.

Template operations must pass through `ActionResolution` and, when UI capture is needed, through the governed template popup described in `docs/specs/document_template_ui_contract.md`.

## Promotion out of chat

Chat artifacts do not become project knowledge or documents implicitly.

Promotion must be explicit:

- chat artifact to `knowledge/files/` or `knowledge/derived/`
- chat artifact to `knowledge/semantic/proposals/`
- chat artifact to `documents/<document_holder>/`

Promotion must preserve source references, chat context, and traceability. Chat remains a coordination surface, not the durable source of project truth.

## F12.4 intake comment proposals

`docs/specs/file_intake.md` owns governed file intake comment semantics.

Chat may produce a `ChatCommentProposal` for adding or editing an optional `user_comment` tied to an `IntakeItem`.

The flow is:

`user message -> ChatCommentProposal -> explicit promotion step -> FileIntakeDraft / IntakeItem update`

Rules:

- chat output is proposal-only
- chat cannot directly persist intake comments
- chat cannot mutate `IntakeItem`
- chat cannot mutate durable intake metadata
- promotion must be explicit and governed
- promotion must preserve `owner_ref`, `trace_ref`, chat context, and source message reference
- comments must be sanitized before persistence
- comments must not contain secrets, private absolute host paths, credentials, tokens, or raw sensitive data
- comments do not classify, expose, authorize, derive, or register files
- chat must not become a hidden intake metadata store

## Out of scope

- opaque file blobs inside chat
- notebook behavior
- semantic workflows
- automatic tool execution by LLM
- direct execution from decomposed subrequests
- direct opening of template popup by LLM
- LLM-authored template filesystem mutation
- editing through the viewer
- direct LLM file mutation
- onboarding chat as project chat
- direct user interaction with internal agents
- prompt editing or prompt persistence in chat
- hidden agent memory
- implicit promotion from chat artifacts to knowledge or documents
- chat-owned tool outputs without `owner_ref`
