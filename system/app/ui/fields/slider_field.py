"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "slider_field.py",
  "version": "0.3.0",
  "type": "tool_module",
  "category": "ui",
  "description": "Reusable slider field widget generated from catalog for declarative UI forms.",
  "location_expected": "system/app/ui/fields/",
  "entry_point": "none",
  "inputs": [],
  "outputs": [],
  "depends_on": [],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "DEV_RUNBOOK",
    "UI_LAYOUT",
    "UI-ARCH-001",
    "UI-FORM-BUILDER-SPEC",
    "UI_DECLARATIVE_FORM_SPEC"
  ]
}
================================================================================
"""

from __future__ import annotations

from tkinter import StringVar, ttk
from typing import Any

from system.app.ui.fields.base_field import BaseField
from system.app.ui.fields.field_utils import (
    coerce_value_for_field,
    format_numeric_value,
    get_numeric_bounds,
    safe_float,
)


class SliderField(BaseField):
    """
    Reusable slider field based on ttk.Scale.

    Responsibilities:
    - render a slider from declarative catalog metadata
    - show current numeric value
    - clamp values to declared bounds
    - align visually with the common form grid
    - expose common field API through BaseField

    Non-responsibilities:
    - persistence
    - direct snapshot traversal
    - domain validation
    """

    def __init__(self, parent, *, field: dict[str, Any], on_dirty=None, ui=None) -> None:
        self.min_value, self.max_value = get_numeric_bounds(field)

        default_value = safe_float(
            coerce_value_for_field(field, field.get("default")),
            default=self.min_value,
        )
        self._resolved_default = max(self.min_value, min(self.max_value, default_value))

        self.value_var = StringVar(value=format_numeric_value(self._resolved_default))

        super().__init__(
            parent,
            field=field,
            on_dirty=on_dirty,
            ui=ui,
        )

        try:
            self.var.set(self._resolved_default)
        except Exception:
            pass

    # ------------------------------------------------------------------
    # Build
    # ------------------------------------------------------------------
    def _build(self) -> None:
        # fila 0: label izquierda + valor derecha
        self.label_widget = self.build_label(
            self,
            row=0,
            column=0,
        )

        self.value_widget = ttk.Label(
            self,
            textvariable=self.value_var,
            width=7,
            anchor="e",
        )
        self.value_widget.grid(
            row=0,
            column=1,
            sticky="e",
            pady=self.ROW_PADY,
        )

        # fila 1: slider alineado con la columna de control
        slider_row = ttk.Frame(self)
        slider_row.grid(
            row=1,
            column=1,
            sticky="ew",
            pady=(4, 0),
        )
        slider_row.columnconfigure(0, weight=1)

        self.scale = ttk.Scale(
            slider_row,
            variable=self.var,
            from_=self.min_value,
            to=self.max_value,
            orient="horizontal",
            command=self._on_scale_change,
        )
        self.scale.grid(
            row=0,
            column=0,
            sticky="ew",
        )

        self.set_primary_control(self.scale)

        # fila 2: límites alineados bajo la barra
        footer = ttk.Frame(self)
        footer.grid(
            row=2,
            column=1,
            sticky="ew",
            pady=(2, 0),
        )
        footer.columnconfigure(0, weight=1)

        self.min_label = ttk.Label(
            footer,
            text=format_numeric_value(self.min_value),
        )
        self.min_label.grid(row=0, column=0, sticky="w")

        self.max_label = ttk.Label(
            footer,
            text=format_numeric_value(self.max_value),
        )
        self.max_label.grid(row=0, column=1, sticky="e")

        # fila 3: help alineado con columna de control
        self.help_widget = self.build_help_label(
            self,
            row=3,
            column=1,
            columnspan=1,
        )

    # ------------------------------------------------------------------
    # Events
    # ------------------------------------------------------------------
    def _on_scale_change(self, raw: str) -> None:
        self.value_var.set(format_numeric_value(raw))

    def _on_var_write(self, *_args: Any) -> None:
        self.value_var.set(format_numeric_value(self.var.get()))
        super()._on_var_write(*_args)

    # ------------------------------------------------------------------
    # Value API
    # ------------------------------------------------------------------
    def get_value(self) -> float:
        numeric = safe_float(self.var.get(), default=self._resolved_default)
        return max(self.min_value, min(self.max_value, numeric))

    def set_value(self, value: Any) -> None:
        numeric = safe_float(
            coerce_value_for_field(self.field, value),
            default=self._resolved_default,
        )
        numeric = max(self.min_value, min(self.max_value, numeric))

        self._loading = True
        try:
            self.var.set(numeric)
            self.value_var.set(format_numeric_value(numeric))
        finally:
            self._loading = False

    # ------------------------------------------------------------------
    # UI state
    # ------------------------------------------------------------------
    def set_readonly(self, readonly: bool = True) -> None:
        self._readonly = bool(readonly)

        try:
            self.scale.configure(
                state="disabled" if self._readonly else "normal"
            )
        except Exception:
            pass

    # ------------------------------------------------------------------
    # Validation
    # ------------------------------------------------------------------
    def validate_value(self):
        value = self.get_value()
        if value < self.min_value or value > self.max_value:
            return False, f"Value out of range for field: {self.key}"
        return True