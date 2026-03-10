"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "base_field.py",
  "version": "0.3.0",
  "type": "tool_module",
  "category": "ui",
  "description": "Base reusable field class for declarative UI forms generated from catalog.",
  "location_expected": "system/app/ui/fields/",
  "entry_point": "none",
  "inputs": [
    {
      "type": "field_spec",
      "location": "in-memory"
    },
    {
      "type": "tk_parent",
      "location": "in-memory"
    }
  ],
  "outputs": [],
  "depends_on": [
    "tkinter"
  ],
  "alignment_required": [
    "OPERATIONAL DEFINITION Rev08",
    "UI-ARCH-001",
    "UI-FORM-BUILDER-SPEC",
    "UI_DECLARATIVE_FORM_SPEC"
  ]
}
================================================================================
"""

from __future__ import annotations

import tkinter as tk
from tkinter import ttk
from typing import Any, Callable, Optional


UIResolver = Callable[..., str]


class BaseField(ttk.Frame):
    """
    Base reusable field for declarative UI forms.

    Responsibilities:
    - store normalized field metadata
    - create and keep the Tk variable
    - expose a common API for value loading/reading
    - notify dirty state through on_dirty
    - provide shared UI helpers for consistent form layout

    Non-responsibilities:
    - persistence
    - domain rules
    - catalog classification
    - direct JSON read/write
    """

    # ------------------------------------------------------------------
    # Shared layout constants
    # ------------------------------------------------------------------
    LABEL_WIDTH = 34
    LABEL_PADX = (0, 12)
    ROW_PADY = 3
    HELP_PADY = (4, 0)
    HELP_WRAP = 620

    def __init__(
        self,
        parent: Any,
        *,
        field: dict[str, Any],
        on_dirty: Optional[Callable[[], None]] = None,
        ui: Optional[UIResolver] = None,
    ) -> None:
        super().__init__(parent)

        self.field = dict(field)
        self.on_dirty = on_dirty
        self.ui = ui

        self.key = str(self.field.get("key", "")).strip()
        self.label = str(self.field.get("label") or self.key)
        self.help_text = str(self.field.get("help") or "")
        self.default = self.field.get("default")
        self.field_type = str(self.field.get("type", "string")).strip().lower()
        self.ui_control = str(self.field.get("ui_control", "entry")).strip().lower()

        self._loading = False
        self._readonly = False
        self._primary_control: tk.Widget | None = None

        self.var: tk.Variable = self._build_variable()

        # Retícula canónica de formulario:
        # col 0 -> label
        # col 1 -> control
        self.columnconfigure(0, weight=0, minsize=220)
        self.columnconfigure(1, weight=1)

        self._build()

        try:
            self.var.trace_add("write", self._on_var_write)
        except Exception:
            pass

    # ------------------------------------------------------------------
    # Build
    # ------------------------------------------------------------------
    def _build_variable(self) -> tk.Variable:
        if self.ui_control == "checkbox" or self.field_type == "bool":
            return tk.BooleanVar(value=bool(self.default))

        if self.field_type == "int":
            try:
                return tk.IntVar(value=int(self.default))
            except Exception:
                return tk.IntVar(value=0)

        if self.field_type == "float" or self.ui_control == "slider":
            try:
                return tk.DoubleVar(value=float(self.default))
            except Exception:
                return tk.DoubleVar(value=0.0)

        return tk.StringVar(value="" if self.default is None else str(self.default))

    def _build(self) -> None:
        raise NotImplementedError("Subclasses must implement _build().")

    # ------------------------------------------------------------------
    # UI helpers
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

    def build_label(
        self,
        parent: Any,
        *,
        row: int,
        column: int = 0,
        width: int | None = None,
        padx: tuple[int, int] | None = None,
        pady: int | tuple[int, int] | None = None,
    ) -> ttk.Label:
        label_widget = ttk.Label(
            parent,
            text=self.label,
            width=self.LABEL_WIDTH if width is None else width,
            anchor="w",
            justify="left",
        )
        label_widget.grid(
            row=row,
            column=column,
            sticky="w",
            padx=self.LABEL_PADX if padx is None else padx,
            pady=self.ROW_PADY if pady is None else pady,
        )
        return label_widget

    def build_help_label(
        self,
        parent: Any,
        *,
        row: int,
        column: int = 0,
        columnspan: int = 2,
        wraplength: int | None = None,
        padx: tuple[int, int] = (0, 0),
    ) -> ttk.Label | None:
        if not self.help_text:
            return None

        help_widget = ttk.Label(
            parent,
            text=self.help_text,
            justify="left",
            wraplength=self.HELP_WRAP if wraplength is None else wraplength,
        )
        help_widget.grid(
            row=row,
            column=column,
            columnspan=columnspan,
            sticky="w",
            padx=padx,
            pady=self.HELP_PADY,
        )
        return help_widget

    def build_standard_control(
        self,
        control: tk.Widget,
        *,
        row: int,
        column: int = 1,
        pady: int | tuple[int, int] | None = None,
    ) -> tk.Widget:
        control.grid(
            row=row,
            column=column,
            sticky="ew",
            pady=self.ROW_PADY if pady is None else pady,
        )
        return control

    def build_full_width_control(
        self,
        control: tk.Widget,
        *,
        row: int,
        column: int = 0,
        columnspan: int = 2,
        pady: int | tuple[int, int] | None = None,
    ) -> tk.Widget:
        control.grid(
            row=row,
            column=column,
            columnspan=columnspan,
            sticky="ew",
            pady=self.ROW_PADY if pady is None else pady,
        )
        return control

    def set_primary_control(self, widget: tk.Widget) -> None:
        self._primary_control = widget

    # ------------------------------------------------------------------
    # Common events / state
    # ------------------------------------------------------------------
    def _on_var_write(self, *_args: Any) -> None:
        if self._loading:
            return

        if callable(self.on_dirty):
            try:
                self.on_dirty()
            except Exception:
                pass

    def set_loading(self, value: bool) -> None:
        self._loading = bool(value)

    def set_readonly(self, readonly: bool = True) -> None:
        self._readonly = bool(readonly)

        if self._primary_control is None:
            return

        try:
            self._primary_control.configure(
                state="disabled" if self._readonly else "normal"
            )
        except Exception:
            pass

    def focus_control(self) -> None:
        if self._primary_control is None:
            return

        try:
            self._primary_control.focus_set()
        except Exception:
            pass

    # ------------------------------------------------------------------
    # Value API
    # ------------------------------------------------------------------
    def get_value(self) -> Any:
        raw_value = self.var.get()

        if self.field_type == "bool":
            return bool(raw_value)

        if self.field_type == "int":
            try:
                return int(raw_value)
            except Exception:
                try:
                    return int(self.default)
                except Exception:
                    return 0

        if self.field_type == "float" or self.ui_control == "slider":
            try:
                return float(raw_value)
            except Exception:
                try:
                    return float(self.default)
                except Exception:
                    return 0.0

        return raw_value

    def set_value(self, value: Any) -> None:
        self._loading = True

        try:
            if self.field_type == "bool":
                self.var.set(bool(value))
                return

            if self.field_type == "int":
                try:
                    self.var.set(int(value))
                except Exception:
                    try:
                        self.var.set(int(self.default))
                    except Exception:
                        self.var.set(0)
                return

            if self.field_type == "float" or self.ui_control == "slider":
                try:
                    self.var.set(float(value))
                except Exception:
                    try:
                        self.var.set(float(self.default))
                    except Exception:
                        self.var.set(0.0)
                return

            self.var.set("" if value is None else str(value))

        finally:
            self._loading = False

    def load_from_snapshot(self, snapshot_value: Any) -> None:
        self.set_value(snapshot_value)

    def collect(self) -> tuple[str, Any]:
        return self.key, self.get_value()

    # ------------------------------------------------------------------
    # Validation
    # ------------------------------------------------------------------
    def validate_value(self) -> bool | str | tuple[bool, str]:
        return True