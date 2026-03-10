"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "checkbox_field.py",
  "version": "0.3.0",
  "type": "tool_module",
  "category": "ui",
  "description": "Reusable checkbox field widget generated from catalog for declarative UI forms.",
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

from tkinter import ttk
from typing import Any

from system.app.ui.fields.base_field import BaseField
from system.app.ui.fields.field_utils import coerce_value_for_field, safe_bool


class CheckboxField(BaseField):
    """
    Reusable checkbox field.

    Responsibilities:
    - render a checkbutton from declarative catalog metadata
    - expose common field API through BaseField
    - support readonly/focus behavior for checkbox controls
    - use a full-width row layout suitable for long checkbox labels

    Non-responsibilities:
    - persistence
    - direct snapshot traversal
    - domain validation
    """

    def __init__(self, parent, *, field: dict[str, Any], on_dirty=None, ui=None) -> None:
        super().__init__(
            parent,
            field=field,
            on_dirty=on_dirty,
            ui=ui,
        )

    # ------------------------------------------------------------------
    # Build
    # ------------------------------------------------------------------
    def _build(self) -> None:
        # El checkbox ocupa el ancho completo del field, no la retícula label+control.
        self.columnconfigure(0, weight=1)
        self.columnconfigure(1, weight=0)

        self.checkbox = ttk.Checkbutton(
            self,
            text=self.label,
            variable=self.var,
        )
        self.build_full_width_control(
            self.checkbox,
            row=0,
            column=0,
            columnspan=2,
        )

        self.set_primary_control(self.checkbox)

        self.help_widget = self.build_help_label(
            self,
            row=1,
            column=0,
            columnspan=2,
            wraplength=self.HELP_WRAP,
            padx=(24, 0),
        )

    # ------------------------------------------------------------------
    # Value API
    # ------------------------------------------------------------------
    def get_value(self) -> bool:
        return safe_bool(self.var.get(), default=bool(self.default))

    def set_value(self, value: Any) -> None:
        checked = safe_bool(
            coerce_value_for_field(self.field, value),
            default=bool(self.default),
        )
        super().set_value(checked)

    # ------------------------------------------------------------------
    # UI state
    # ------------------------------------------------------------------
    def set_readonly(self, readonly: bool = True) -> None:
        self._readonly = bool(readonly)

        try:
            if self._readonly:
                self.checkbox.state(["disabled"])
            else:
                self.checkbox.state(["!disabled"])
        except Exception:
            pass

    # ------------------------------------------------------------------
    # Validation
    # ------------------------------------------------------------------
    def validate_value(self):
        return True