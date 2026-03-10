# UI-FORM-BUILDER-SPEC

### Declarative Form Builder Specification

**Version:** 1.0 (Premium)
**Status:** Stable Proposal

---

# 1. Purpose

The **FormBuilder** is responsible for converting a **Declarative Form Specification** into a working UI form composed of concrete field widgets.

The builder interprets the form specification and creates the appropriate UI elements using the system architecture:

```
FormSpec
   ↓
FormBuilder
   ↓
FieldFactory
   ↓
FieldRegistry
   ↓
BaseField subclasses
```

The builder does **not implement domain logic**.
It is strictly a **UI composition layer**.

---

# 2. Responsibilities

The FormBuilder must:

1. Interpret form specifications.
2. Normalize fields and sections.
3. Create UI widgets through `FieldFactory`.
4. Maintain references to created fields.
5. Load values from a snapshot.
6. Collect values into a structured object.
7. Support readonly and visibility states.
8. Provide minimal validation hooks.

The FormBuilder must **not**:

* persist data
* execute actions
* perform domain validation
* modify system configuration

---

# 3. Architecture Position

The builder sits between specification and widgets.

```
Declarative Spec
      ↓
FormBuilder
      ↓
FieldFactory
      ↓
Field Widgets
```

Dialogs and panels should **not build fields directly**.

Instead they should delegate:

```
PreferencesDialog
      ↓
FormBuilder
```

---

# 4. FormBuilder Object

The builder is typically instantiated with:

```
FormBuilder(
    parent,
    form_spec,
    on_dirty=None,
    ui=None
)
```

### Parameters

| Parameter | Description                        |
| --------- | ---------------------------------- |
| parent    | container widget                   |
| form_spec | normalized form specification      |
| on_dirty  | callback when field changes        |
| ui        | UI resolver / translation resolver |

---

# 5. Internal State

The builder maintains several internal structures.

### Field Map

```
fields_by_key
```

Mapping:

```
key → BaseField instance
```

Example:

```
{
 "ui.language": <SelectField>,
 "llm.temperature": <SliderField>
}
```

---

### Section List

Ordered list of sections created.

```
sections = [
  section_widget,
  section_widget
]
```

---

### Field Order

Ordered list for traversal and validation.

```
field_widgets = []
```

---

# 6. Building the Form

The build process occurs in phases.

---

## Phase 1 — Normalize Spec

The builder must normalize the specification:

```
sections
fields
defaults
visibility
types
```

Normalization rules come from the form spec.

Example:

```
missing type → entry
missing label → humanized key
missing key → fallback to id
```

---

## Phase 2 — Create Sections

For each section:

```
create SectionFrame
apply title
apply description
store reference
```

Sections are rendered in order.

---

## Phase 3 — Create Fields

For each field:

```
field_widget = FieldFactory.create_field(...)
```

Parameters passed:

```
parent
field spec
ui resolver
dirty callback
```

Then:

```
field_widget.grid(...)
```

The builder records:

```
fields_by_key[key] = field_widget
field_widgets.append(field_widget)
```

---

# 7. Snapshot Loading

The builder supports loading values from a snapshot.

Example snapshot:

```
{
 "ui": {
   "language": "en"
 },
 "llm": {
   "temperature": 0.8
 }
}
```

Fields use dotted path resolution.

Example:

```
llm.temperature
```

Load process:

```
value = snapshot_lookup(snapshot, key)
field.set_value(value)
```

Errors must not crash the builder.

---

# 8. Data Collection

Collected data must follow the same structure as the snapshot.

Example:

```
{
 "ui": {
   "language": "es"
 },
 "llm": {
   "temperature": 0.9
 }
}
```

Process:

```
for each field:
    value = field.get_value()
    assign via dotted path
```

---

# 9. Dirty Tracking

Fields notify changes via callback.

```
on_dirty()
```

The builder should:

```
ignore events while loading
propagate change to dialog
```

This allows dialogs to enable:

```
Apply
Save
```

buttons.

---

# 10. Readonly Mode

Readonly can be applied at three levels:

### Form

```
form.readonly
```

### Section

```
section.readonly
```

### Field

```
field.readonly
```

Resolution priority:

```
field > section > form
```

The builder calls:

```
field.set_readonly(True)
```

---

# 11. Validation

The builder supports lightweight validation.

Process:

```
for field in fields:
    result = field.validate_value()
```

Return format:

```
(valid, message)
```

Domain validation remains outside the builder.

---

# 12. Error Handling

The builder must behave **tolerantly**.

| Situation             | Behavior          |
| --------------------- | ----------------- |
| unknown field type    | fallback to entry |
| missing label         | generate label    |
| invalid option format | ignore field      |
| invalid default       | ignore            |

Strict mode may optionally exist.

---

# 13. Public Builder API

Typical methods:

### Build

```
build()
```

Creates sections and fields.

---

### Load Snapshot

```
load_data(snapshot)
```

---

### Collect Data

```
collect_data()
```

Returns nested structure.

---

### Flat Data (optional)

```
collect_flat_data()
```

Returns:

```
{
 "ui.language": "en"
}
```

---

### Validation

```
validate()
```

Returns:

```
(True, "")
(False, "error message")
```

---

### Readonly

```
set_readonly(True)
```

---

# 14. Example Usage

Example dialog code:

```
builder = FormBuilder(
    parent=self.frame,
    form_spec=spec,
    on_dirty=self.mark_dirty,
    ui=self.ui
)

builder.build()

builder.load_data(snapshot)
```

On apply:

```
data = builder.collect_data()
```

Controller receives:

```
controller.update_preferences(data)
```

---

# 15. Benefits

Using FormBuilder provides:

### Reduced UI boilerplate

Dialogs stop creating widgets manually.

### Consistent UI behavior

All forms behave the same.

### Plugin extensibility

Plugins can provide their own form specs.

### LLM compatibility

LLMs can generate valid UI structures.

---

# 16. Relationship with Existing Components

| Component     | Role              |
| ------------- | ----------------- |
| FormSpec      | describe form     |
| FormBuilder   | build form        |
| FieldFactory  | resolve field     |
| FieldRegistry | map control names |
| BaseField     | widget behavior   |
| Dialog        | UI container      |
| Controller    | domain logic      |

---

# 17. Future Extensions

Possible future improvements include:

* multi-column layouts
* dynamic visibility rules
* async option loading
* conditional sections
* field dependencies

These are **intentionally excluded** from version 1.0.

---

# 18. Summary

The **FormBuilder** transforms a declarative form specification into a working UI form using the existing widget architecture.

It guarantees:

* clean separation of layers
* reusable dialogs
* plugin compatibility
* safe UI generation

without introducing a complex layout engine.

---

## Siguiente paso recomendado (muy interesante)

Ahora mismo tu arquitectura UI queda **muy clara**:

```
FormSpec
   ↓
FormBuilder
   ↓
FieldFactory
   ↓
FieldRegistry
   ↓
Fields
```

Si quieres, puedo darte también algo **muy potente para tu proyecto**:

### 📦 `system/app/ui/forms/`

Una **arquitectura completa de formularios declarativos** con:

```
forms/
   form_builder.py
   form_normalizer.py
   form_section.py
   form_spec_validator.py
```

Eso deja tu sistema listo para:

* **paneles de preferencias**
* **paneles plugin**
* **formularios generados por LLM**
* **reducción brutal de código UI**

