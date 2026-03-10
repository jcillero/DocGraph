# UI-DECLARATIVE-FORM-SPEC

### Declarative UI Form Specification

**Version:** 1.1 (Premium)
**Status:** Stable Proposal

---

# 1. Purpose

This specification defines a **declarative structure for UI forms** used by dialogs, panels, and configuration views.

The goal is to allow UI forms to be **defined as data structures instead of hard-coded widgets**, enabling:

* reusable UI composition
* dynamic dialogs
* plugin-defined panels
* LLM-generated forms
* reduction of UI boilerplate code

Forms described by this specification are interpreted by a **Form Builder**, which uses the existing architecture:

```
form_spec
   ↓
form_builder
   ↓
field_factory
   ↓
field_registry
   ↓
concrete field widgets
   ↓
dialog / panel
   ↓
controller
```

The specification only defines **UI structure and presentation hints**.

It **does not define**:

* domain logic
* persistence
* runtime execution
* artifact generation
* system configuration changes

---

# 2. Architectural Principles

The design follows several core principles.

### 2.1 Declarative UI

Forms describe **what fields exist**, not **how they are built**.

Concrete widgets are resolved through the UI architecture:

```
FieldFactory
FieldRegistry
BaseField subclasses
```

---

### 2.2 Separation of Responsibilities

| Layer          | Responsibility               |
| -------------- | ---------------------------- |
| Form Spec      | describe form structure      |
| Form Builder   | interpret spec               |
| Field Factory  | resolve widget types         |
| Field Registry | map control names to classes |
| Field Widgets  | implement UI behavior        |
| Dialog / Panel | compose UI                   |
| Controller     | read/write domain state      |

---

### 2.3 Domain Isolation

Form specifications must **never contain domain logic**.

The following are explicitly forbidden inside form specs:

* business rules
* system mutations
* execution instructions
* external API calls
* file operations

---

### 2.4 Tolerant Interpretation

Unknown properties in the specification **must be ignored unless explicitly supported**.

This guarantees:

* forward compatibility
* plugin extensibility
* safe LLM-generated specs

---

# 3. Top-Level Form Structure

A form specification is a JSON object.

Example:

```json
{
  "id": "preferences.general",
  "title": "General Preferences",
  "description": "Basic configuration options",
  "sections": []
}
```

## Fields

| Field       | Type    | Required | Description                   |
| ----------- | ------- | -------- | ----------------------------- |
| id          | string  | yes      | unique identifier of the form |
| title       | string  | optional | display title                 |
| description | string  | optional | explanatory text              |
| readonly    | boolean | optional | entire form is readonly       |
| sections    | array   | yes      | ordered list of sections      |
| metadata    | object  | optional | extension container           |

---

# 4. Sections

Forms are composed of **sections** grouping related fields.

Example:

```json
{
  "key": "language",
  "title": "Language",
  "fields": []
}
```

## Section Fields

| Field       | Type    | Required | Description               |
| ----------- | ------- | -------- | ------------------------- |
| key         | string  | yes      | unique section identifier |
| title       | string  | optional | section title             |
| description | string  | optional | explanatory text          |
| visible     | boolean | optional | default true              |
| collapsed   | boolean | optional | UI hint                   |
| fields      | array   | yes      | list of field objects     |
| metadata    | object  | optional | extension container       |

---

# 5. Field Definition

A field describes a **single user-editable value**.

Example:

```json
{
  "id": "ui_language",
  "key": "ui.language",
  "type": "select",
  "label": "Language",
  "options": ["en", "es", "fr"],
  "default": "en"
}
```

---

# 6. Field Properties

## Identity

| Field | Description                       |
| ----- | --------------------------------- |
| id    | unique identifier inside the form |
| key   | dotted path used to bind values   |

### Key Rules

* `key` should normally exist.
* if missing, builder may fallback to `id`.
* dotted paths are allowed.

Example:

```
ui.language
llm.temperature
editor.font_size
```

---

## Presentation

| Field       | Type    | Description   |
| ----------- | ------- | ------------- |
| label       | string  | display label |
| description | string  | help text     |
| placeholder | string  | hint text     |
| visible     | boolean | default true  |
| enabled     | boolean | default true  |
| readonly    | boolean | default false |
| required    | boolean | default false |

---

## Type

### Canonical Field Property

```
"type"
```

Example:

```
entry
checkbox
select
slider
textarea
password
```

### Compatibility Alias

For backward compatibility:

```
ui_control
```

may also appear.

Rule:

```
type overrides ui_control
```

---

## Options (for select)

Two formats are allowed.

### Simple

```json
"options": ["en", "es", "fr"]
```

Normalized internally to:

```
value = option
label = option
```

### Rich

```json
"options": [
  { "value": "en", "label": "English" },
  { "value": "es", "label": "Spanish" }
]
```

Builder must always return **value**.

---

## Numeric Range

Used by sliders or numeric inputs.

| Field | Description   |
| ----- | ------------- |
| min   | minimum value |
| max   | maximum value |
| step  | increment     |

Example:

```json
{
  "type": "slider",
  "key": "llm.temperature",
  "min": 0,
  "max": 2,
  "step": 0.1
}
```

---

## Validation (UI Only)

Validation is limited to **light UI feedback**.

Example:

```json
"validation": {
  "min_length": 3
}
```

Validation **must not contain domain rules**.

Controllers remain responsible for final validation.

---

## Tags

Tags are optional labels used for grouping or filtering.

Example:

```json
"tags": ["advanced", "llm"]
```

---

## UI Extensions

The `ui` field allows widget-specific hints.

Example:

```json
"ui": {
  "width": 300
}
```

Builders may ignore unsupported hints.

---

## Metadata

Metadata is reserved for future extension.

Example:

```json
"metadata": {
  "plugin": "translation"
}
```

---

# 7. Field Normalization Rules

Form builders must apply the following normalization rules.

| Property | Default          |
| -------- | ---------------- |
| visible  | true             |
| enabled  | true             |
| readonly | false            |
| required | false            |
| type     | entry            |
| label    | derived from key |
| key      | fallback to id   |

Example label derivation:

```
llm.temperature → Llm Temperature
editor_font_size → Editor Font Size
```

---

# 8. Data Binding

Field keys map to a **snapshot object**.

Example snapshot:

```json
{
  "ui": {
    "language": "en"
  },
  "llm": {
    "temperature": 0.8
  }
}
```

Builders must support dotted paths.

Example:

```
llm.temperature
```

---

# 9. Data Collection

Collected form output must follow the same key structure.

Example:

```json
{
  "ui": {
    "language": "es"
  },
  "llm": {
    "temperature": 0.9
  }
}
```

Builders may optionally support **flat output mode**.

---

# 10. Error Handling

Builders should behave **tolerantly**.

| Situation                | Behavior          |
| ------------------------ | ----------------- |
| unknown field property   | ignore            |
| unknown section property | ignore            |
| unknown field type       | fallback to entry |
| invalid option format    | ignore field      |

Strict mode may optionally be implemented.

---

# 11. Extensibility

The spec supports extension through:

```
metadata
ui
tags
```

Plugins may add fields without modifying the core system.

---

# 12. Example Complete Form

```json
{
  "id": "preferences.general",
  "title": "General Preferences",
  "sections": [
    {
      "key": "ui",
      "title": "Interface",
      "fields": [
        {
          "id": "language",
          "key": "ui.language",
          "type": "select",
          "label": "Language",
          "options": ["en", "es", "fr"],
          "default": "en"
        },
        {
          "id": "dark_mode",
          "key": "ui.dark_mode",
          "type": "checkbox",
          "label": "Dark Mode",
          "default": false
        }
      ]
    },
    {
      "key": "llm",
      "title": "LLM Settings",
      "fields": [
        {
          "id": "temperature",
          "key": "llm.temperature",
          "type": "slider",
          "label": "Temperature",
          "min": 0,
          "max": 2,
          "step": 0.1,
          "default": 0.7
        }
      ]
    }
  ]
}
```

---

# 13. Future Extensions (Non-normative)

Possible future improvements include:

* grid layouts
* conditional visibility
* async data sources
* multi-column forms
* nested sections

These are **intentionally excluded** from the current specification.

---

# 14. Summary

This specification defines a **simple, extensible, declarative form model** that integrates with the UI architecture:

```
Form Spec
↓
Form Builder
↓
FieldFactory
↓
FieldRegistry
↓
Widget classes
```

It provides:

* strong separation of concerns
* compatibility with dynamic UI generation
* safe extensibility
* clear integration with controllers

while deliberately avoiding the complexity of a full layout engine.

---
