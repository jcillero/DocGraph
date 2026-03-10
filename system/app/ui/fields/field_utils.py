"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "field_utils.py",
  "version": "0.2.0",
  "type": "tool_module",
  "category": "ui",
  "description": "Common utilities for reusable UI fields generated from declarative catalog.",
  "location_expected": "system/app/ui/fields/",
  "entry_point": "none",
  "inputs": [
    {
      "type": "field_spec",
      "location": "in-memory"
    },
    {
      "type": "snapshot_value",
      "location": "in-memory"
    }
  ],
  "outputs": [],
  "depends_on": [],
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

from typing import Any


# ----------------------------------------------------------------------
# Basic normalization
# ----------------------------------------------------------------------
def normalize_key(value: Any) -> str:
    return str(value or "").strip()


def normalize_label(field: dict[str, Any]) -> str:
    key = normalize_key(field.get("key"))
    label = field.get("label")

    if label is None or str(label).strip() == "":
        return key

    return str(label).strip()


def normalize_help(field: dict[str, Any]) -> str:
    help_text = field.get("help")
    if help_text is None:
        return ""
    return str(help_text).strip()


def normalize_ui_control(field: dict[str, Any]) -> str:
    return str(field.get("ui_control", "entry")).strip().lower()


def normalize_field_type(field: dict[str, Any]) -> str:
    return str(field.get("type", "string")).strip().lower()


def normalize_choices(field: dict[str, Any]) -> list[str]:
    """
    Normalize selectable choices to a clean list[str].

    Accepts:
    - ["a", "b", "c"]
    - [1, 2, 3]
    - None -> []
    """
    choices = field.get("choices", [])

    if not isinstance(choices, list):
        return []

    normalized: list[str] = []
    for item in choices:
        text = str(item).strip()
        if text == "":
            continue
        normalized.append(text)

    return normalized


# ----------------------------------------------------------------------
# Numeric helpers
# ----------------------------------------------------------------------
def safe_bool(value: Any, default: bool = False) -> bool:
    if isinstance(value, bool):
        return value

    if value is None:
        return default

    if isinstance(value, (int, float)):
        return bool(value)

    text = str(value).strip().lower()

    if text in {"1", "true", "yes", "y", "on"}:
        return True

    if text in {"0", "false", "no", "n", "off", ""}:
        return False

    return default


def safe_int(value: Any, default: int = 0) -> int:
    try:
        return int(value)
    except Exception:
        return default


def safe_float(value: Any, default: float = 0.0) -> float:
    try:
        return float(value)
    except Exception:
        return default


def get_numeric_bounds(field: dict[str, Any]) -> tuple[float, float]:
    min_value = safe_float(field.get("min", 0.0), default=0.0)
    max_value = safe_float(field.get("max", 1.0), default=1.0)

    if max_value < min_value:
        min_value, max_value = max_value, min_value

    return min_value, max_value


def clamp_numeric_value(value: Any, *, minimum: float, maximum: float, default: float) -> float:
    numeric = safe_float(value, default=default)
    return max(minimum, min(maximum, numeric))


def format_numeric_value(value: Any, *, decimals: int = 2) -> str:
    decimals = safe_int(decimals, default=2)
    if decimals < 0:
        decimals = 0

    try:
        return f"{float(value):.{decimals}f}"
    except Exception:
        return f"{0.0:.{decimals}f}"


# ----------------------------------------------------------------------
# Field coercion
# ----------------------------------------------------------------------
def coerce_value_for_field(field: dict[str, Any], value: Any) -> Any:
    field_type = normalize_field_type(field)
    control = normalize_ui_control(field)
    default = field.get("default")

    if control == "checkbox" or field_type == "bool":
        return safe_bool(value, default=safe_bool(default, default=False))

    if field_type == "int":
        return safe_int(value, default=safe_int(default, default=0))

    if field_type == "float" or control == "slider":
        return safe_float(value, default=safe_float(default, default=0.0))

    if field_type == "enum" or control == "select":
        choices = normalize_choices(field)
        text = "" if value is None else str(value).strip()

        if choices:
            if text in choices:
                return text

            default_text = "" if default is None else str(default).strip()
            if default_text in choices:
                return default_text

            return choices[0]

        return text

    if value is None:
        return ""

    return str(value)


# ----------------------------------------------------------------------
# Snapshot helpers
# ----------------------------------------------------------------------
def get_snapshot_value(snapshot: dict[str, Any], dotted_key: str, default: Any) -> Any:
    if not isinstance(snapshot, dict):
        return default

    parts = [p for p in str(dotted_key).split(".") if p]
    if not parts:
        return default

    current: Any = snapshot

    for part in parts:
        if not isinstance(current, dict) or part not in current:
            return default
        current = current[part]

    return current


def set_dotted_value(target: dict[str, Any], dotted_key: str, value: Any) -> dict[str, Any]:
    if not isinstance(target, dict):
        raise TypeError("target must be a dict")

    parts = [p for p in str(dotted_key).split(".") if p]
    if not parts:
        return target

    current = target
    for part in parts[:-1]:
        node = current.get(part)
        if not isinstance(node, dict):
            node = {}
            current[part] = node
        current = node

    current[parts[-1]] = value
    return target


# ----------------------------------------------------------------------
# Optional convenience helpers
# ----------------------------------------------------------------------
def build_flat_patch(field_key: str, value: Any) -> dict[str, Any]:
    patch: dict[str, Any] = {}
    set_dotted_value(patch, field_key, value)
    return patch