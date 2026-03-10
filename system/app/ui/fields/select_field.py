"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "select_field.py",
  "version": "0.3.0",
  "type": "tool_module",
  "category": "ui",
  "description": "Reusable select/combobox field widget generated from catalog for declarative UI forms.",
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
from system.app.ui.fields.field_utils import coerce_value_for_field, normalize_choices


class SelectField(BaseField):
    """
    Reusable select field based on ttk.Combobox (readonly).

    Responsibilities:
    - render a label + readonly combobox from declarative catalog metadata
    - normalize allowed choices
    - expose common field API through BaseField
    - support readonly/focus behavior for combobox controls

    Non-responsibilities:
    - persistence
    - direct snapshot traversal
    - domain validation
    """

    def __init__(self, parent, *, field: dict[str, Any], on_dirty=None, ui=None) -> None:
        self.choices: list[str] = [str(x) for x in normalize_choices(field)]

        super().__init__(
            parent,
            field=field,
            on_dirty=on_dirty,
            ui=ui,
        )

        if self.choices:
            current = str(self.var.get())
            if current not in self.choices:
                try:
                    self.var.set(self.choices[0])
                except Exception:
                    pass

    # ------------------------------------------------------------------
    # Build
    # ------------------------------------------------------------------
    def _build(self) -> None:
        self.label_widget = self.build_label(
            self,
            row=0,
            column=0,
        )

        self.combobox = ttk.Combobox(
            self,
            textvariable=self.var,
            state="readonly",
            values=self.choices,
        )
        self.build_standard_control(
            self.combobox,
            row=0,
            column=1,
        )

        self.set_primary_control(self.combobox)

        self.help_widget = self.build_help_label(
            self,
            row=1,
            column=0,
            columnspan=2,
        )

    # ------------------------------------------------------------------
    # Value API
    # ------------------------------------------------------------------
    def get_value(self) -> str:
        value = str(self.var.get())

        if self.choices and value not in self.choices:
            return self.choices[0]

        return value

    def set_value(self, value: Any) -> None:
        text = coerce_value_for_field(self.field, value)
        text = "" if text is None else str(text)

        if self.choices and text not in self.choices:
            text = self.choices[0]

        super().set_value(text)

    # ------------------------------------------------------------------
    # UI state
    # ------------------------------------------------------------------
    def set_readonly(self, readonly: bool = True) -> None:
        self._readonly = bool(readonly)

        try:
            self.combobox.configure(
                state="disabled" if self._readonly else "readonly"
            )
        except Exception:
            pass

    # ------------------------------------------------------------------
    # Validation
    # ------------------------------------------------------------------
    def validate_value(self):
        if self.choices and self.get_value() not in self.choices:
            return False, f"Invalid value for field: {self.key}"
        return True