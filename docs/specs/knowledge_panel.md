# knowledge_panel

## Purpose

Define the minimum UI-facing contract for the F8 knowledge panel without introducing notebook behavior, semantic processing, or LLM workflow.

## Unit of knowledge

A knowledge unit in F8 is a readonly project-owned document exposed from `knowledge/` under the project root.

It is represented minimally by:

- `document_id`
- `display_name`
- `logical_path`
- `is_viewable`
- `is_selected`

## F8 behavior

The knowledge panel may:

- list visible knowledge units
- show an empty, ready, or error state
- select one unit at a time
- route the selected unit into the existing readonly viewer tab inside the workspace content area

## F8 data flow

`project_root/knowledge/*` -> small technical listing boundary -> `KnowledgeController` -> `KnowledgePanelState` -> workspace tab selection -> `ViewerController`

The viewer remains readonly, reuses existing technical display behavior, and is treated as one concrete workspace tab rather than as the whole main content area.

The knowledge panel itself is not the workspace container. It is one surface that can select a knowledge unit and open that unit into the viewer tab model already defined by the shell.

Knowledge selection stays distinct from:

- tree-based reference of existing workspace documents
- clip-based intake of external files
- chat-level structured references or tool results

## Out of scope for F8

- notebook or blocks
- graph or edge modeling
- semantic scoring or tagging
- LLM workflows
- automatic promotion from resources to knowledge
- editing, saving, or annotations
- mixing knowledge with tool execution or project pipeline ownership
- introducing notebook, blocks, or complex workspace layout behavior
