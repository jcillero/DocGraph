"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "field_factory.py",
  "version": "0.3.0",
  "type": "tool_module",
  "category": "ui",
  "description": "Factory for creating UI field widgets from declarative catalog specifications.",
  "location_expected": "system/app/ui/fields/",
  "entry_point": "FieldFactory.create_field",
  "runtime_required": false,
  "modifies_system": false,
  "inputs": [
    {
      "type": "mapping",
      "location": "in-memory",
      "description": "field specification mapping"
    },
    {
      "type": "widget",
      "location": "in-memory",
      "description": "parent widget"
    },
    {
      "type": "callable",
      "location": "in-memory",
      "description": "optional ui resolver",
      "required": false
    },
    {
      "type": "callable",
      "location": "in-memory",
      "description": "optional on_dirty callback",
      "required": false
    }
  ],
  "outputs": [],
  "depends_on": [
    "system/app/ui/fields/base_field.py",
    "system/app/ui/fields/field_registry.py"
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

from typing import Any, Callable, Mapping, Optional

from .base_field import BaseField
from .field_registry import FieldRegistry


UIResolver = Callable[..., str]
DirtyCallback = Callable[[], None]


class FieldFactory:
    """
    Central factory for UI field widgets.

    Responsibilities:
    - normalize declarative field metadata
    - resolve concrete field class through FieldRegistry
    - instantiate field widgets consistently
    - provide strict or tolerant behavior for unknown controls

    Non-responsibilities:
    - business logic
    - persistence
    - runtime execution
    - domain validation
    """

    DEFAULT_CONTROL = "entry"

    CONTROL_ALIASES: dict[str, str] = {
        "text": "entry",
        "string": "entry",
        "input": "entry",
        "bool": "checkbox",
        "boolean": "checkbox",
        "choice": "select",
        "enum": "select",
        "range": "slider",
        "numeric_range": "slider",
    }

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------
    @classmethod
    def create_field(
        cls,
        parent: Any,
        *,
        field: Mapping[str, Any],
        on_dirty: Optional[DirtyCallback] = None,
        ui: Optional[UIResolver] = None,
        strict: bool = False,
        fallback_control: str | None = None,
    ) -> BaseField:
        """
        Create a concrete field widget from a declarative specification.

        Parameters:
            parent:
                Parent widget/container.

            field:
                Declarative field specification.

            on_dirty:
                Optional callback triggered by the field when its value changes.

            ui:
                Optional UI resolver / i18n resolver.

            strict:
                If True, unknown controls raise an error.
                If False, unknown controls fall back to a safe control.

            fallback_control:
                Optional fallback control name. Defaults to 'entry'.

        Returns:
            BaseField instance.

        Raises:
            TypeError:
                If the field spec is invalid.

            ValueError:
                If the field spec is missing required data or an unknown control
                is used in strict mode.
        """
        normalized = cls._normalize_field_spec(field)
        control = normalized["ui_control"]

        resolved_class = cls._resolve_field_class(
            control=control,
            strict=strict,
            fallback_control=fallback_control or cls.DEFAULT_CONTROL,
        )

        instance = resolved_class(
            parent,
            field=normalized,
            on_dirty=on_dirty,
            ui=ui,
        )

        if not isinstance(instance, BaseField):
            raise TypeError(
                f"Registered class for control '{normalized['ui_control']}' "
                f"did not create a BaseField instance."
            )

        return instance

    @classmethod
    def create(
        cls,
        parent: Any,
        *,
        field: Mapping[str, Any],
        on_dirty: Optional[DirtyCallback] = None,
        ui: Optional[UIResolver] = None,
        strict: bool = False,
        fallback_control: str | None = None,
    ) -> BaseField:
        """
        Convenience alias for create_field().
        """
        return cls.create_field(
            parent,
            field=field,
            on_dirty=on_dirty,
            ui=ui,
            strict=strict,
            fallback_control=fallback_control,
        )

    @classmethod
    def supports(cls, control: str) -> bool:
        canonical = cls.resolve_control(control)
        return FieldRegistry.get(canonical) is not None

    @classmethod
    def available_controls(cls) -> list[str]:
        return list(FieldRegistry.list_controls())

    @classmethod
    def resolve_control(cls, control: str | None) -> str:
        value = str(control or cls.DEFAULT_CONTROL).strip().lower()
        return cls.CONTROL_ALIASES.get(value, value)

    @classmethod
    def register(cls, control: str, field_class: type[BaseField]) -> None:
        """
        Optional convenience passthrough.

        Keeps external callers from importing FieldRegistry directly when the
        architectural entry point should remain the factory layer.
        """
        canonical = cls.resolve_control(control)
        FieldRegistry.register(canonical, field_class)

    @classmethod
    def get_registered_class(cls, control: str) -> type[BaseField] | None:
        canonical = cls.resolve_control(control)
        return FieldRegistry.get(canonical)

    # ------------------------------------------------------------------
    # Resolution
    # ------------------------------------------------------------------
    @classmethod
    def _resolve_field_class(
        cls,
        *,
        control: str,
        strict: bool,
        fallback_control: str,
    ) -> type[BaseField]:
        field_class = FieldRegistry.get(control)
        if field_class is not None:
            return field_class

        if strict:
            available = ", ".join(FieldRegistry.list_controls())
            raise ValueError(
                f"Unknown field type '{control}'. "
                f"Available controls: [{available}]"
            )

        fallback = cls.resolve_control(fallback_control)
        fallback_class = FieldRegistry.get(fallback)

        if fallback_class is None:
            available = ", ".join(FieldRegistry.list_controls())
            raise ValueError(
                f"Unknown field type '{control}', and fallback '{fallback}' "
                f"is not registered. Available controls: [{available}]"
            )

        return fallback_class

    # ------------------------------------------------------------------
    # Normalization
    # ------------------------------------------------------------------
    @classmethod
    def _normalize_field_spec(cls, field: Mapping[str, Any]) -> dict[str, Any]:
        if not isinstance(field, Mapping):
            raise TypeError("field spec must be a mapping")

        normalized = dict(field)

        key = str(normalized.get("key", "")).strip()
        if not key:
            raise ValueError("field spec must include a non-empty 'key'")

        raw_control = normalized.get("ui_control", cls.DEFAULT_CONTROL)
        canonical_control = cls.resolve_control(str(raw_control))

        normalized["key"] = key
        normalized["ui_control"] = canonical_control

        normalized.setdefault("label", cls._humanize_key(key))
        normalized.setdefault("visible", True)
        normalized.setdefault("enabled", True)
        normalized.setdefault("readonly", False)
        normalized.setdefault("required", False)

        return normalized

    @staticmethod
    def _humanize_key(key: str) -> str:
        """
        Convert technical keys into readable default labels.

        Examples:
            ui.language      -> Ui Language
            max_tokens       -> Max Tokens
            llm.temperature  -> Llm Temperature
        """
        cleaned = key.replace(".", " ").replace("_", " ").strip()
        parts = [part.capitalize() for part in cleaned.split() if part.strip()]
        return " ".join(parts)