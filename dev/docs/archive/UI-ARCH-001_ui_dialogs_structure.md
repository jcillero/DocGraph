# UI Architecture — Dialog Folder Plan (UI-ARCH-001)

**Status:** Draft (ready to adopt)  
**Updated:** 2026-03-04  
**Scope:** Tkinter UI dialogs + reusable widgets + controllers (UI wiring only)

---

## Objetivo

Consolidar una **estructura de carpetas para dialogs** que:

- escale sin caos (muchos dialogs, múltiples módulos)
- separe UI de lógica de dominio (managers/config/storage fuera de UI)
- sea compatible con los ficheros actuales (preferences/credentials managers + tools)
- permita pruebas tempranas (harness CLI) y migración progresiva a UI integrada

---

## Compatibilidad con el estado actual

Este plan es compatible porque **no cambia**:

- `preferences_manager.py` / `preferences_catalog.json` (core/config)
- `credentials_manager.py` / `credentials.json` (core/config)
- tools `tool_get_*`, `tool_set_*`, `tool_reset_*`, `tool_validate_*` (CLI)

Los scripts `tool_ui_preferences_dialog.py` y `tool_ui_credentials_dialog.py` pasan a ser **harness**:
- hoy: ejecutables desde CLI
- mañana: base para `dialogs/preferences/...` y `dialogs/credentials/...` (mínimos cambios: quitar argparse y usar `Toplevel`)

---

## Árbol propuesto

> **Nota:** El árbol está pensado para crecer por “dominios” (features) y reutilizar bases.

```
app/
  ui/
    app_ui.py                      # Entry point UI (wiring, sin lógica de dominio)
    menu/
      menu_builder.py              # Construcción de barra superior/acciones

    dialogs/
      __init__.py

      base/
        dialog_base.py             # Toplevel + helpers (center, modal, esc)
        form_dialog_base.py        # OK/Apply/Cancel + dirty state + status line
        tabbed_dialog_base.py      # ttk.Notebook + helpers de tabs

      preferences/
        preferences_dialog.py      # Dialog principal "Preferencias" con tabs
        tabs/
          tab_general.py
          tab_llm.py
          tab_ui.py
          tab_advanced.py

      credentials/
        credentials_dialog.py      # Editor de credenciales (provider selector + fields)

      registry/
        register_document_dialog.py
        register_collection_dialog.py

      library/
        manage_collections_dialog.py

      chat/
        new_chat_dialog.py
        export_chat_dialog.py

      reports/
        export_report_dialog.py

      common/
        about_dialog.py
        confirm_dialog.py
        error_dialog.py

    widgets/
      __init__.py
      scrollframe.py               # Frame scrollable reutilizable
      masked_entry.py              # Entry con mostrar/ocultar (api_key)
      labeled_input.py             # Label + entry/combobox estándar
      pref_field_factory.py        # Generación de widgets desde catalog (select/checkbox/slider)
      status_bar.py                # Línea de estado reusable
      toast.py                     # (Opcional) notificaciones no bloqueantes

    controllers/
      __init__.py
      preferences_controller.py    # UI → PreferencesManager
      credentials_controller.py    # UI → CredentialsManager
      registry_controller.py
      library_controller.py
      chat_controller.py
      reports_controller.py

    state/
      ui_state.py                  # Estado efímero UI (geometry, tab activa, flags)
```

---

## Principios de diseño

### 1) Separación UI / dominio

**Los dialogs NO acceden directamente a JSON ni a storage.**  
Flujo recomendado:

```
Dialog (Tk widgets)
  → Controller (orquesta acciones)
    → Manager (PreferencesManager / CredentialsManager)
      → Storage (user/registry/*.json)
```

### 2) Entry points sin lógica de dominio

`app_ui.py` y menús solo hacen **wiring**:

- crear ventana principal
- crear menú superior
- abrir dialogs vía controllers

### 3) Reutilización primero

Cualquier patrón repetido va a `widgets/` o `dialogs/base/`:

- scroll + forms
- masking de secretos
- OK / Apply / Cancel
- Notebook (tabs)

### 4) Organización por dominios

Un dialog “pertenece” a un módulo (registry, library, chat…), no a una colección global plana.
Esto evita carpetas tipo `dialogs/` con 80 ficheros sin orden.

### 5) Convención de ficheros

- **1 dialog = 1 fichero** (salvo tabs).
- Si el dialog tiene tabs: subcarpeta `tabs/` (un fichero por tab).
- Nombres explícitos: `export_chat_dialog.py` mejor que `export.py`.

---

## Diseño del dialog de Preferencias (referencia)

**Preferencias** se implementa como `TabbedDialogBase` con tabs:

- General: opciones frecuentes (modo LLM, provider cloud, creatividad, longitud)
- LLM: políticas (cloud_allowed, fallback)
- Credenciales: botón “Editar…” que abre `credentials_dialog.py`
- Interfaz: theme/language/layout
- Avanzado: opciones expuestas raras + futuras

Regla: el contenido se genera desde `preferences_catalog.json` filtrando por:
- `exposed: true`
- `ui_control`: `select|checkbox|slider`
y agrupando por `tags` (llm/ui/runner/etc.).

---

## Plan de migración progresiva (sin romper nada)

### Fase 0 — Mantener harness CLI (ya)
- `tool_ui_preferences_dialog.py`
- `tool_ui_credentials_dialog.py`

### Fase 1 — Integración UI real
- mover/copiar a `dialogs/preferences/...` y `dialogs/credentials/...`
- convertir `tk.Tk` → `tk.Toplevel`
- eliminar `argparse` y recibir `parent` + `controller` por inyección

### Fase 2 — Menú superior
- `menu_builder.py` añade acción: **Preferencias…**
- handler: `PreferencesController.open_dialog(parent=root)`

### Fase 3 — Refinos UX
- status line no bloqueante
- confirmaciones solo cuando hay error real
- “dirty state” (Apply habilitado solo si hay cambios)

---

## Checklist de validación

- [ ] ningún dialog lee/escribe JSON directamente
- [ ] rutas resueltas vía resolver (sin hardcode)
- [ ] `app_ui.py` sin lógica de dominio
- [ ] `PreferencesDialog` genera UI desde catálogo, no “a mano”
- [ ] credenciales ocultas por defecto; toggle “mostrar”
- [ ] Apply/OK/Cancel coherentes en todos los dialogs (base común)

---

## Notas

- Este documento define estructura y convenciones.  
- Las decisiones funcionales (qué hace cada dialog) van en specs o docs de feature.

---
**Fin — UI-ARCH-001**
