"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "entry_field.py",
  "version": "0.2.0",
  "type": "tool_module",
  "category": "ui",
  "description": "Reusable entry field widget generated from catalog for declarative UI forms.",
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
from system.app.ui.fields.field_utils import coerce_value_for_field


class EntryField(BaseField):
    """
    Reusable text entry field.

    Responsibilities:
    - render a label + entry pair from declarative catalog metadata
    - expose common field API through BaseField
    - support readonly/focus behavior for entry controls

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
        self.columnconfigure(1, weight=1)

        self.label_widget = self.build_label(
            self,
            row=0,
            column=0,
            padx=(0, 8),
            pady=2,
        )

        self.entry = ttk.Entry(
            self,
            textvariable=self.var,
        )
        self.entry.grid(
            row=0,
            column=1,
            sticky="ew",
            pady=2,
        )

        self.set_primary_control(self.entry)

        self.help_widget = self.build_help_label(
            self,
            row=1,
            column=0,
            columnspan=2,
            wraplength=520,
        )

    # ------------------------------------------------------------------
    # Value API
    # ------------------------------------------------------------------
    def get_value(self) -> str:
        return str(self.var.get())

    def set_value(self, value: Any) -> None:
        text = coerce_value_for_field(self.field, value)
        text = "" if text is None else str(text)
        super().set_value(text)

    # ------------------------------------------------------------------
    # UI state
    # ------------------------------------------------------------------
    def set_readonly(self, readonly: bool = True) -> None:
        self._readonly = bool(readonly)

        try:
            self.entry.configure(
                state="readonly" if self._readonly else "normal"
            )
        except Exception:
            pass

    # ------------------------------------------------------------------
    # Validation
    # ------------------------------------------------------------------
    def validate_value(self):
        return True