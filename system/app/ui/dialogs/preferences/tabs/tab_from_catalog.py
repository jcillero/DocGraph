"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tab_from_catalog.py",
  "version": "0.3.0",
  "type": "tool_module",
  "category": "ui",
  "description": "Reusable preferences tab generated from catalog using FieldFactory.",
  "location_expected": "system/app/ui/dialogs/preferences/tabs/",
  "entry_point": "none",
  "inputs": [
    {
      "type": "field_specs",
      "location": "in-memory"
    },
    {
      "type": "snapshot",
      "location": "in-memory"
    }
  ],
  "outputs": [],
  "depends_on": [
    "tkinter",
    "system/app/ui/fields/field_factory.py"
  ],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "UI_LAYOUT",
    "UI-ARCH-001",
    "CONTRATO_UI_CORE"
  ]
}
================================================================================
"""

from __future__ import annotations

from typing import Any, Callable, Mapping, Optional, Protocol

import tkinter as tk
from tkinter import ttk

from system.app.ui.fields.field_factory import FieldFactory
from system.app.ui.fields.field_utils import get_snapshot_value, set_dotted_value


UIResolver = Callable[..., str]


class FieldWidgetProtocol(Protocol):
    def grid(self, *args: Any, **kwargs: Any) -> Any: ...
    def set_value(self, value: Any) -> None: ...
    def get_value(self) -> Any: ...
    def set_readonly(self, readonly: bool = True) -> None: ...
    def focus_control(self) -> None: ...
    def validate_value(self) -> Any: ...


class TabFromCatalog(ttk.Frame):
    """
    Dynamic preferences tab generated from a field catalog.

    Responsibilities:
    - create a visual container for fields described by catalog
    - delegate widget creation to FieldFactory
    - load snapshot data into fields
    - collect field values in flat or dotted-path form
    - notify dirty state to parent dialog

    Non-responsibilities:
    - direct JSON read/write
    - domain logic
    - persistence orchestration
    - runtime operations
    """

    def __init__(
        self,
        parent: tk.Misc,
        *,
        fields: list[dict[str, Any]],
        on_dirty: Optional[Callable[[], None]] = None,
        ui: Optional[UIResolver] = None,
        unknown_bucket: bool = False,
    ) -> None:
        super().__init__(parent)

        self.on_dirty = on_dirty
        self.ui = ui
        self.unknown_bucket = bool(unknown_bucket)

        self._loading = False
        self._readonly = False

        self._fields: list[dict[str, Any]] = self._normalize_fields(fields)
        self._field_widgets: list[FieldWidgetProtocol] = []
        self._field_by_key: dict[str, FieldWidgetProtocol] = {}

        self.columnconfigure(0, weight=1)
        self.rowconfigure(0, weight=1)

        self._build()

    # ------------------------------------------------------------------
    # UI resolver
    # ------------------------------------------------------------------
    def _ui(self, key: str, **kwargs: Any) -> str:
        if self.ui is not None:
            try:
                return self.ui(key, **kwargs)
            except Exception:
                pass

        text = key
        if kwargs:
            try:
                text = text.format(**kwargs)
            except Exception:
                pass
        return text

    # ------------------------------------------------------------------
    # Normalization
    # ------------------------------------------------------------------
    def _normalize_fields(self, fields: list[dict[str, Any]] | None) -> list[dict[str, Any]]:
        normalized: list[dict[str, Any]] = []
        seen_keys: set[str] = set()

        for item in fields or []:
            if not isinstance(item, dict):
                continue

            key = str(item.get("key", "")).strip()
            if not key:
                continue

            if key in seen_keys:
                continue

            seen_keys.add(key)
            normalized.append(dict(item))

        return normalized

    # ------------------------------------------------------------------
    # Build
    # ------------------------------------------------------------------
    def _build(self) -> None:
        outer = ttk.Frame(self, padding=8)
        outer.grid(row=0, column=0, sticky="nsew")
        outer.columnconfigure(0, weight=1)

        if not self._fields:
            self._build_empty_state(outer)
            return

        for row_index, field in enumerate(self._fields):
            key = str(field.get("key", "")).strip()
            if not key:
                continue

            widget = FieldFactory.create_field(
                outer,
                field=field,
                on_dirty=self._on_field_changed,
                ui=self.ui,
            )

            widget.grid(
                row=row_index,
                column=0,
                sticky="ew",
                pady=(0, 10),
            )

            self._field_widgets.append(widget)
            self._field_by_key[key] = widget

        if self._readonly:
            self.set_readonly(True)

    def _build_empty_state(self, parent: ttk.Frame) -> None:
        message_key = (
            "prefs.tab.empty.unknown_bucket"
            if self.unknown_bucket
            else "prefs.tab.empty"
        )

        message = self._ui(message_key)
        if message == message_key:
            message = "No preferences available in this section."

        ttk.Label(
            parent,
            text=message,
            justify="left",
            wraplength=700,
        ).grid(
            row=0,
            column=0,
            sticky="w",
            padx=4,
            pady=8,
        )

    # ------------------------------------------------------------------
    # Events
    # ------------------------------------------------------------------
    def _on_field_changed(self) -> None:
        if self._loading:
            return

        if callable(self.on_dirty):
            try:
                self.on_dirty()
            except Exception:
                pass

    # ------------------------------------------------------------------
    # Snapshot loading
    # ------------------------------------------------------------------
    def load_data(self, snapshot: Mapping[str, Any] | None) -> None:
        """
        Load effective snapshot values.

        Supports nested values through dotted keys, for example:
        - ui.language
        - llm.creativity
        """
        self._loading = True

        try:
            source = dict(snapshot or {})

            for field in self._fields:
                key = str(field.get("key", "")).strip()
                if not key:
                    continue

                widget = self._field_by_key.get(key)
                if widget is None:
                    continue

                default = field.get("default")
                value = get_snapshot_value(source, key, default)

                try:
                    widget.set_value(value)
                except Exception:
                    try:
                        widget.set_value(default)
                    except Exception:
                        pass

        finally:
            self._loading = False

    # ------------------------------------------------------------------
    # Data collection
    # ------------------------------------------------------------------
    def collect_data(self) -> dict[str, Any]:
        """
        Return nested data using dotted keys.

        Example:
            ui.language -> {"ui": {"language": "..."}}
        """
        data: dict[str, Any] = {}

        for field in self._fields:
            key = str(field.get("key", "")).strip()
            if not key:
                continue

            widget = self._field_by_key.get(key)
            if widget is None:
                continue

            try:
                value = widget.get_value()
            except Exception:
                continue

            set_dotted_value(data, key, value)

        return data

    def collect_flat_data(self) -> dict[str, Any]:
        """
        Optional variant: return flat key -> value.
        Useful for debugging or change comparison.
        """
        data: dict[str, Any] = {}

        for field in self._fields:
            key = str(field.get("key", "")).strip()
            if not key:
                continue

            widget = self._field_by_key.get(key)
            if widget is None:
                continue

            try:
                data[key] = widget.get_value()
            except Exception:
                continue

        return data

    # ------------------------------------------------------------------
    # Validation
    # ------------------------------------------------------------------
    def validate_data(self) -> tuple[bool, str]:
        """
        Minimal shared validation.

        Prepared to grow with field-specific validation
        without introducing domain rules into the tab.
        """
        for field in self._fields:
            key = str(field.get("key", "")).strip()
            if not key:
                continue

            widget = self._field_by_key.get(key)
            if widget is None:
                continue

            validator = getattr(widget, "validate_value", None)
            if not callable(validator):
                continue

            try:
                result = validator()
            except Exception:
                return False, f"Validation error in field: {key}"

            if result is True or result is None:
                continue

            if result is False:
                return False, f"Invalid value for field: {key}"

            if isinstance(result, str):
                return False, result or f"Invalid value for field: {key}"

            if isinstance(result, tuple) and len(result) == 2:
                ok, message = result
                if not ok:
                    return False, str(message or f"Invalid value for field: {key}")
                continue

            return False, f"Invalid value for field: {key}"

        return True, ""

    # ------------------------------------------------------------------
    # Readonly / focus / helpers
    # ------------------------------------------------------------------
    def set_readonly(self, readonly: bool = True) -> None:
        self._readonly = bool(readonly)

        for widget in self._field_widgets:
            setter = getattr(widget, "set_readonly", None)
            if callable(setter):
                try:
                    setter(self._readonly)
                except Exception:
                    pass

    def focus_first_field(self) -> None:
        for widget in self._field_widgets:
            focus_method = getattr(widget, "focus_control", None)
            if callable(focus_method):
                try:
                    focus_method()
                    return
                except Exception:
                    continue

    def has_fields(self) -> bool:
        return bool(self._field_widgets)

    def get_field_widget(self, key: str) -> FieldWidgetProtocol | None:
        return self._field_by_key.get(str(key).strip())