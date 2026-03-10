# UI_LAYOUT — DocGraph

**Version:** Rev06
**Nature:** Declarative
**Status:** Active
**Aligned with:** INVARIANTS GUIDE Rev04 / Operational Definition Rev08

---

# 1. Principles

The UI follows a strict set of architectural rules designed to maintain **portability, traceability and separation of responsibilities**.

The user interface is a **presentation and interaction layer** that reflects the state of the platform but does not govern system behavior.

Primary principles:

* **Workspace-centric interface**
* Minimalistic and deterministic layout
* No domain logic inside the UI layer
* Every executable user action must trigger a **registered tool or controller action**
* The UI must never expose inactive capabilities
* The UI must not duplicate persistent state defined in the conceptual model
* The UI reflects system state; it does not control it
* UI components must remain compatible with the invariant system, especially:

  * workspace host existence
  * UI text externalization
  * core/plugin separation

---

### Core interaction model

The UI is structured around a small number of **stable interaction components**.

These components form the **interaction shell of the platform**:

* a **Workspace Tree**
* a **Tabbed Workspace Host**
* a **dynamic Content Host**
* a **Global Contextual Input Bar**
* a **collapsible Log Panel**

The interface is not limited to chat interaction.

The UI supports multiple project object types, with **chat** and **notebook** as the initial MVP views.

Future views may include workflow visualization, artifact inspection and run analysis.

---


# 2. Global Layout

```text
+-------------------------------------------------------------------+
| TOP MENU                                                          |
+-------------------------------------------------------------------+
| Project Selector                                                  |
+-------------------------------------------------------------------+
| Project Tree      | Notebook Viewer                               |
|                   |-----------------------------------------------|
|                   | Selected Object Content                       |
|                   | (Notebook / Source / Artifact / Output / ...) |
+-------------------------------------------------------------------+
| GLOBAL CONTEXTUAL INPUT BAR                                       |
+-------------------------------------------------------------------+
| COLLAPSIBLE LOG PANEL                                             |
+-------------------------------------------------------------------+
```

This layout defines the **canonical workspace shell** of the platform.

The global layout is centered on **project navigation and notebook-based content presentation**.

The left side of the interface is responsible for **project selection and structural navigation**.
The main workspace is responsible for rendering the **selected project object**.

This layout must remain stable across versions in order to preserve **interaction consistency** and alignment with the workflow-centric project model.

---

## 2.1 Layout Distribution

Recommended visual distribution:

* Left region:

  * **Project Selector** → fixed header zone
  * **Project Tree** → **20–28%**
* Central region (**Notebook Viewer**) → **72–80%**

Additional layout properties:

* Global Input Bar → fixed height
* Bottom Log Panel → variable height and collapsible

The central region must remain the **dominant interaction space**, ensuring that the selected project object is always the primary focus of the interface.

The Context Panel is **not part of the minimal canonical layout** and should be treated, if implemented, as an optional future extension rather than as a mandatory structural region. This is more consistent with the current project model centered on:

```text
Project
 ├ Workflow
 ├ Notebooks
 ├ Sources
 ├ Prompts
 ├ Artifacts
 ├ Outputs
 └ Runs
```

---

# 3. Top Menu

The top menu provides access to **global system operations**.

Menu actions must only trigger:

* registered runtime tools
* UI controllers
* system dialogs

Direct execution of domain logic from the UI layer is forbidden.

---

## 3.1 File

The File menu provides workspace and project lifecycle actions.

MVP functions:

* New Area
* New Project
* New Chat
* New Notebook
* Register document
* Export
* Close application

Excluded from MVP:

* RDF management
* embedding management
* advanced runtime management

These capabilities appear only when the corresponding runtime layers become available.

---

## 3.2 Preferences

The Preferences dialog provides **core system configuration**.

Preferences are part of the **core platform configuration** and must not be owned by plugins.

Subsections:

### General

* UI language
* Theme (light / dark)
* Remember workspace layout
* Restore previously open tabs

### LLM (if provider active)

* AUTO / MANUAL execution mode
* Target response length
* Creativity parameter (if supported)

Restrictions:

* No model identifiers may be hardcoded in the UI
* If runtime capabilities are not declared → only basic chat interaction may appear

Preferences remain a **core dialog** managed by the platform.

---

## 3.3 Tools

The Tools menu provides access to **registered system tools**.

Examples may include:

* structured drafting
* agent management
* runtime healthcheck
* structural audit

All menu actions must execute registered tools through the **runtime/tool registry**.

The UI must not implement operational logic.

---

## 3.4 Help

The Help section provides informational resources.

Typical entries:

* User manual
* Current platform version
* Technical diagnostics

The Help section must not expose development tooling directly.

---

# 4. Left Region — WorkspaceTree

The Workspace Tree represents the **persistent project structure**.

The tree reflects the conceptual model of the platform but does not execute system logic.

Tree navigation must only control **workspace selection and tab activation**.

---

### Hierarchical structure

```text
Area
 └── Project
      ├── Workflow
      ├── Notebooks
      │    ├── Notebook
      │    └── Notebook
      ├── Sources
      ├── Prompts
      ├── Data
      ├── Artifacts
      ├── Outputs
      └── Runs
```

This structure mirrors the **internal model of a project** used by the platform.

---

### MVP visible nodes

During the MVP stage the tree may initially expose only:

* Areas
* Projects
* Notebooks
* Chats

Additional nodes such as Sources, Artifacts or Runs may appear when the corresponding runtime layers are activated.

---

### Rules

The Workspace Tree must follow strict behavioral constraints:

* Selecting a node updates the **active workspace selection**
* The tree must never execute domain logic
* Tree navigation must not generate runs automatically
* The tree only opens or activates workspace tabs
* The tree does not own content state

Tab behavior rules:

* Only **one tab** may be active at a time
* Multiple tabs may remain open simultaneously

This guarantees separation between **navigation state and content state**.

---

### Allowed actions

The tree may allow a limited set of structural actions:

* Create Area
* Create Project
* Create Chat
* Create Notebook
* Rename entities
* Delete entities (with confirmation)

These actions must be executed through **registered controllers**, not through direct UI state mutation.

---

## 5. Central Region — Tabbed Workspace Host

The central region is no longer a single ChatView.

It is composed of:

* `TabHost`
* `ContentHost`

### Purpose

Allow simultaneous work on multiple open project objects.

### Allowed tab types (MVP)

* Chat
* Notebook

### Future tab types

* Document
* Output
* Specialized plugin views

### Rules

* Clicking a tree item:

  * activates the existing tab if already open
  * otherwise opens a new tab
* Closing a tab closes only the **view**, not the underlying object
* Tabs are a workspace mechanism, not a persistence mechanism
* Tabs must remain lightweight and reversible

### Visual guidance

Each tab should display:

* object title
* object type icon
* modified indicator if applicable

Suggested icons:

* 💬 Chat
* 📓 Notebook
* 📄 Document
* 📦 Output

---

## 6. Content Host

The `ContentHost` is the dynamic central container.

It renders the view corresponding to the active tab type.

### Initial supported views

* `ChatView`
* `NotebookView`

### Future supported views

* `DocumentView`
* `OutputView`

### Dispatch rule

The content host resolves the active view from the selected/open object type.

Example logic:

* chat → `ChatView`
* notebook → `NotebookView`

The content host owns **presentation switching**, not domain execution.

---

## 7. ChatView

`ChatView` is the conversational view for a project chat.

### Functions

* conversation with the active agent
* chat history display
* runtime status indicator
* contextual tool access

### Visible indicators

* effective agent
* active collection (if applicable)
* associated sources count
* runtime state

### Restrictions

* no direct RAG logic
* no OCR logic
* no direct database access

All operations are delegated to the execution layer.

### Notes

A chat remains an independent project object.
It is not reduced to a notebook block.

---

## 8. NotebookView

`NotebookView` is the structured editing workspace for notebooks.

A notebook is a **project object**, not a popup tool.

### Initial block types (MVP)

* `heading`
* `paragraph`
* `snippet`
* `tool_result`

### Future block types

* `table`
* `list`
* `figure`
* `callout`
* `chat_ref`

### Rules

* one block may be active at a time
* operations target the active block
* notebooks are persistent objects under the project tree
* notebook state is edited in the central workspace, not in detached windows

### Chat reference block

Later, notebooks may support:

* `chat_ref`

This block references a project chat without destroying chat identity.

---

## 9. Global Contextual Input Bar

The application uses a **single shared input bar** below the active workspace.

This input belongs to the **workspace shell**, not to a specific chat or notebook.

### Behavior by active tab type

#### If active tab = Chat

The input sends a user message to the active chat.

#### If active tab = Notebook

The input sends an instruction to the active block of the notebook.

Typical actions:

* rewrite
* summarize
* expand
* convert
* create block below

### UI behavior

The input should adapt:

* placeholder text
* icon
* mode hint

Examples:

* `> Write a message...`
* `> Instruction for active block...`

### Restrictions

The input is a contextual interaction surface, not a free execution console.

---

## 10. Context Resolution

## 10.1 Context Identifier

For chats, the classical operational context remains:

```text
context_id = (area_id, project_id, chat_id)
```

For notebooks, the operational context extends to:

```text
context_id = (area_id, project_id, notebook_id, active_block_id)
```

### Rule

The UI must always resolve context deterministically.

---

## 10.2 Effective Agent

Agent resolution follows deterministic hierarchy:

1. Agent assigned to the active object
2. Agent assigned to the Project
3. Agent assigned to the Area
4. System default agent

This mechanism must always produce a deterministic result.

---

## 11. Context Panel (Collapsible)

Purpose:

Display the structural state of the **active workspace object**.

### For active Chat

* active Area
* active Project
* active Chat
* effective Agent
* active Collection
* number of associated Sources
* recent runs related to the chat

### For active Notebook

* active Area
* active Project
* active Notebook
* active block type
* active block id
* effective Agent
* recent runs related to notebook actions

### Allowed actions

* inspect context
* inspect recent runs
* manage lightweight associations

### Not allowed

* deep structural editing
* direct model/runtime mutation
* direct registry changes
* execution of complex tools from inside the panel

---

## 12. Bottom Log Panel

Possible states:

* IDLE
* LLM_RUNNING
* TOOL_RUNNING
* ERROR
* CANCELLED

Characteristics:

* collapsible
* scrollable
* linked to `run_id`
* compatible with release mode

Release mode behavior:

* log collapsed by default
* debug messages hidden

The log panel is distinct from the contextual input bar.

---

## 13. Dialog Windows

Dialogs are used for:

* preferences
* creation flows
* registration flows
* management tasks

They are not the main editing surface for notebooks or chats.

### 13.1 Preferences Dialog

Core configuration dialog.

### 13.2 Registration Dialog

Functions:

* file selection
* automatic OCR when required

### 13.3 Library Dialog

Functions:

* display registered documents
* manual selection
* association workflows

### 13.4 Agent Management Dialog

Defines:

* base prompt
* interaction parameters
* declared capabilities

Only activated capabilities may appear in the UI.

---

## 14. Runtime Transition States

Execution flow:

1. user submits prompt or notebook instruction
2. state → `LLM_RUNNING` or `TOOL_RUNNING`
3. run is registered
4. result linked to `run_id`
5. state returns to `IDLE`, `ERROR`, or `CANCELLED`

All executions must follow the run traceability system.

---

## 15. Global UI Restrictions

The UI must not introduce:

* duplicated persistent application state
* direct access to `system/`
* direct execution of business logic
* hidden capabilities not declared by configuration
* notebook replacement by popup-only editing
* chat replacement by notebook-only embedding

Tabs are allowed as a **workspace mechanism**, but they must not replace the project tree.

The UI remains a presentation and orchestration layer only.

---

## 16. Evolution Readiness

The layout allows future integration without redesign:

* document tabs
* output tabs
* split views
* command palette
* slash-like notebook commands
* plugin-provided views
* knowledge graph tools
* local LLM runtime
* causal reasoning tools

Future capabilities remain hidden until explicit activation in configuration.

---


# 17. Evolution Readiness

The UI architecture is designed to remain compatible with future platform capabilities defined in the **Platform Roadmap Rev05**.

The interface must support the following future architectural layers without structural redesign.

---

## 17.1 Self-Diagnosis Integration

Future versions of the platform may include **structural diagnostic tools** capable of analyzing:

* filesystem structure
* registry integrity
* plugin validity
* invariant compliance
* runtime configuration

The UI must support:

* visualization of diagnostic reports
* navigation to diagnostic artifacts
* contextual display of system health indicators

Diagnostic outputs are expected under:

```
user/runs/system_diagnostics/
```

The UI must treat these reports as **read-only analytical artifacts**.

---

## 17.2 Knowledge Workflow Visualization

Future workflow-driven knowledge operations may include:

```
document ingestion
semantic retrieval
reasoning pipelines
AI-assisted generation
structured output creation
```

The UI workspace model must therefore support:

* workflow object visualization
* step execution monitoring
* artifact inspection
* traceable run history

These capabilities must remain aligned with the **Run traceability system** defined in the platform architecture.

---

## 17.3 Human-in-the-Loop Governance

Even when semi-autonomous workflows are introduced, the UI must ensure:

* explicit user approval for workflow execution
* clear run traceability
* visible artifact provenance
* reversible workflow steps when possible

The UI therefore remains an **orchestration and inspection layer**, never the authority for execution.

---


