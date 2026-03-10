"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "field_registry.py",
  "version": "0.2.0",
  "type": "tool_module",
  "category": "ui",
  "description": "Central registry for declarative UI field controls and their concrete widget classes.",
  "location_expected": "system/app/ui/fields/",
  "entry_point": "FieldRegistry.register",
  "runtime_required": false,
  "modifies_system": false,
  "inputs": [
    "control name",
    "field class"
  ],
  "outputs": [],
  "depends_on": [
    "system/app/ui/fields/base_field.py"
  ],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "UI_LAYOUT",
    "UI-ARCH-001",
    "CONTRATO_UI_CORE",
    "SCRIPT_META STANDARD"
  ]
}
================================================================================
"""

from __future__ import annotations

from typing import Any

from .base_field import BaseField


class FieldRegistry:
    """
    Central registry for declarative UI field controls.

    Responsibilities:
    - map canonical control names to concrete BaseField subclasses
    - validate registered classes
    - provide lookup and inspection helpers

    Non-responsibilities:
    - widget instantiation
    - layout building
    - persistence
    - business logic
    """

    _registry: dict[str, type[BaseField]] = {}

    # ------------------------------------------------------------------
    # Internal helpers
    # ------------------------------------------------------------------
    @classmethod
    def _normalize_control_name(cls, control_name: Any) -> str:
        value = str(control_name or "").strip().lower()
        if not value:
            raise ValueError("control_name must not be empty")
        return value

    @classmethod
    def _validate_field_class(cls, field_class: type[BaseField]) -> None:
        try:
            is_valid = issubclass(field_class, BaseField)
        except TypeError:
            is_valid = False

        if not is_valid:
            raise TypeError(
                "field_class must inherit from BaseField"
            )

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------
    @classmethod
    def register(
        cls,
        control_name: str,
        field_class: type[BaseField],
        *,
        replace: bool = True,
    ) -> None:
        """
        Register a field class for a canonical control name.

        Parameters:
            control_name:
                Control identifier, e.g. 'entry', 'checkbox', 'select', 'slider'.

            field_class:
                Concrete BaseField subclass.

            replace:
                If False, attempting to overwrite an existing registration
                raises ValueError.
        """
        key = cls._normalize_control_name(control_name)
        cls._validate_field_class(field_class)

        if not replace and key in cls._registry:
            raise ValueError(
                f"Field control already registered: {key!r}"
            )

        cls._registry[key] = field_class

    @classmethod
    def unregister(cls, control_name: str) -> bool:
        """
        Remove a control registration if present.
        Returns True if removed, False otherwise.
        """
        key = cls._normalize_control_name(control_name)
        return cls._registry.pop(key, None) is not None

    @classmethod
    def clear(cls) -> None:
        """
        Clear the whole registry.

        Useful for isolated tests, but should be used sparingly in app runtime.
        """
        cls._registry.clear()

    @classmethod
    def get(cls, control_name: str) -> type[BaseField] | None:
        key = cls._normalize_control_name(control_name)
        return cls._registry.get(key)

    @classmethod
    def require(cls, control_name: str) -> type[BaseField]:
        key = cls._normalize_control_name(control_name)
        field_class = cls._registry.get(key)
        if field_class is None:
            available = ", ".join(cls.list_controls())
            raise KeyError(
                f"Field control not registered: {key!r}. "
                f"Available controls: [{available}]"
            )
        return field_class

    @classmethod
    def is_registered(cls, control_name: str) -> bool:
        key = cls._normalize_control_name(control_name)
        return key in cls._registry

    @classmethod
    def list_controls(cls) -> list[str]:
        return sorted(cls._registry.keys())

    @classmethod
    def list_items(cls) -> list[tuple[str, type[BaseField]]]:
        return sorted(cls._registry.items(), key=lambda item: item[0])

    @classmethod
    def snapshot(cls) -> dict[str, str]:
        """
        Return a lightweight debug snapshot:
        control -> class name
        """
        return {
            control: field_class.__name__
            for control, field_class in cls.list_items()
        }

    @classmethod
    def count(cls) -> int:
        return len(cls._registry)