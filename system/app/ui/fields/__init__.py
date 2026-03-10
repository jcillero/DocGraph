"""
================================================================================
SCRIPT_META
================================================================================
{
  "script_name": "__init__.py",
  "version": "1.0.0",
  "type": "ui_module_bootstrap",
  "category": "ui",
  "description": "Bootstrap module that registers built-in UI field widgets.",
  "location_expected": "system/app/ui/fields/",
  "entry_point": "auto_import"
}
================================================================================
"""

from __future__ import annotations

from .field_registry import FieldRegistry

from .entry_field import EntryField
from .checkbox_field import CheckboxField
from .select_field import SelectField
from .slider_field import SliderField


def register_builtin_fields() -> None:
    """
    Register all built-in UI field widgets.
    """

    FieldRegistry.register("entry", EntryField)
    FieldRegistry.register("checkbox", CheckboxField)
    FieldRegistry.register("select", SelectField)
    FieldRegistry.register("slider", SliderField)


# -----------------------------------------------------------------------------
# Automatic bootstrap on package import
# -----------------------------------------------------------------------------

register_builtin_fields()