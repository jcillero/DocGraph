# UI_LAYOUT — DocGraph

Version: **Rev07 — Governance Grade**
Nature: **Declarative Specification**
Status: **Active**
Aligned with: **Operational Definition Rev09 / Invariants Guide Rev05**

---

# 1. Purpose

This document defines the **canonical user interface layout** of the DocGraph platform.

The objective is to maintain a **stable interaction shell** that:

* preserves portability
* guarantees architectural separation
* enables plugin extensibility
* supports AI-assisted UI analysis
* avoids layout drift across versions

The UI layer is strictly a **presentation and orchestration layer** and must not contain domain logic.

---

# 2. Architectural Principles

The UI must follow these core rules:

1. Workspace-centric design
2. Minimal and deterministic layout
3. Separation between navigation, content and execution
4. Compatibility with structural invariants
5. Stable interaction model across releases

The UI reflects system state but **never controls system behavior directly**.

---

# 3. Workspace Shell Components

The platform defines a **canonical workspace shell** composed of five core components.

These components must always exist in the UI architecture.

```
WorkspaceTree
TabHost
ContentHost
GlobalInputBar
LogPanel
```

These components represent the **minimum interaction infrastructure** required by the platform.

They allow the UI to remain stable while new capabilities are added through plugins.

---

# 4. Global Layout

The canonical UI layout is defined as follows:

```
+-------------------------------------------------------------------+
| TOP MENU                                                          |
+-------------------------------------------------------------------+
| Project Selector                                                  |
+-------------------------------------------------------------------+
| Project Tree      | Workspace Tabs                                |
|                   |-----------------------------------------------|
|                   | Active Content View                           |
|                   | (Notebook / Chat / Document / Output / ...)   |
+-------------------------------------------------------------------+
| GLOBAL CONTEXTUAL INPUT BAR                                       |
+-------------------------------------------------------------------+
| COLLAPSIBLE LOG PANEL                                             |
+-------------------------------------------------------------------+
```

This layout represents the **stable interaction shell of the platform**.

The UI is centered on **project navigation and content interaction**.

---

# 5. Layout Distribution

Recommended visual proportions:

Left region:

Project Selector → fixed height
Project Tree → **20–28% width**

Central region:

Workspace Tabs + Content Host → **72–80% width**

Bottom elements:

Global Input Bar → fixed height
Log Panel → collapsible with variable height

The central region must always remain the **dominant visual workspace**.

---

# 6. Top Menu

The top menu provides access to **global system actions**.

Menus must only trigger:

* registered runtime tools
* UI controllers
* system dialogs

Direct execution of domain logic from the UI is forbidden.

Menu groups:

```
File
Preferences
Tools
Help
```

---

# 7. Workspace Tree

The Workspace Tree represents the **project navigation structure**.

It mirrors the conceptual project model:

```
Area
 └ Project
     ├ Workflow
     ├ Notebooks
     ├ Sources
     ├ Prompts
     ├ Data
     ├ Artifacts
     ├ Outputs
     └ Runs
```

Rules:

* navigation only
* no execution logic
* opening objects activates tabs

---

# 8. TabHost

The TabHost manages open workspace views.

Tabs represent **UI views**, not persistent objects.

Allowed MVP tab types:

```
Chat
Notebook
```

Future tab types:

```
Document
Output
PluginView
```

Tabs must remain **lightweight and reversible**.

---

# 9. ContentHost

The ContentHost renders the view corresponding to the active tab.

Example dispatch:

```
chat → ChatView
notebook → NotebookView
document → DocumentView
output → OutputView
```

The content host handles **view switching only**.

---

# 10. Global Contextual Input Bar

The platform uses **one global input bar** shared by all views.

Behavior depends on active tab.

### Chat tab

Send user message to chat.

### Notebook tab

Send instruction to active notebook block.

Examples:

```
Rewrite block
Summarize text
Generate section
```

The input bar is **context-aware but not a command console**.

---

# 11. Context Resolution

Operational context must always be deterministic.

For chats:

```
context_id = (area_id, project_id, chat_id)
```

For notebooks:

```
context_id = (area_id, project_id, notebook_id, active_block_id)
```

---

# 12. Log Panel

The bottom panel displays runtime execution status.

Possible states:

```
IDLE
LLM_RUNNING
TOOL_RUNNING
ERROR
CANCELLED
```

Characteristics:

* collapsible
* scrollable
* linked to `run_id`
* compatible with release mode

---

# 13. Dialog Windows

Dialogs handle configuration and management tasks.

Examples:

```
Preferences
Document Registration
Library Management
Agent Management
```

Dialogs are **auxiliary surfaces**, not primary workspaces.

---

# 14. UI Restrictions

The UI must **never**:

* write artifacts directly to `user/output`
* write run records directly to `user/runs`
* modify `system/`
* execute domain logic
* bypass runtime registries

All operational actions must go through **registered tools or controllers**.

---

# 15. Evolution Readiness

The layout supports future extensions without redesign.

Possible integrations:

```
Document viewer
Workflow visualization
Run inspector
Plugin panels
```

Because the **workspace shell remains stable**, the platform can evolve without breaking the interaction model.

---

# 16. Summary

The DocGraph UI architecture is defined by a **stable workspace shell** composed of:

```
WorkspaceTree
TabHost
ContentHost
GlobalInputBar
LogPanel
```

This structure guarantees:

* UI stability
* plugin compatibility
* architectural governance
* AI-assisted analysis capability
* long-term maintainability of the platform

---

