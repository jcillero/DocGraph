# =============================================================================
# SCRIPT METADATA
# =============================================================================
# script_id: preferences_manager
# version: 1.1.2
# script_type: core_module
# owner: system
# status: stable
#
# description:
#   Core module responsible for loading, validating, repairing and saving
#   user preferences in a UI-agnostic way.
#
# script_location: system/app/core/preferences/preferences_manager.py
# canonical_storage: user/registry/preferences.json
#
# runtime_path_policy:
#   The preferences path MUST be resolved dynamically using the runtime
#   configuration layer (runtime_registry / path resolver). Hardcoded paths
#   are forbidden.
#
# behavior:
#   - Missing file → create an overrides-only preferences file (values:{})
#   - Corrupt JSON → backup as *.bad-YYYYMMDD_HHMMSS and regenerate overrides-only
#   - Atomic writes (tmp → replace) with small retry to be Windows-friendly
#   - Validation + patch application using dotted paths (e.g., "ui.theme")
#
# catalog:
#   Optionally loads a read-only preferences catalog (system/spec) as the
#   single source of truth for:
#     - defaults
#     - types
#     - enums
#     - ranges
#     - patterns
#     - list item validation rules (items.*)
#
# notes:
#   - Disk storage is kept small: only user overrides are persisted.
#   - Returned payload.values are EFFECTIVE values (defaults merged with overrides),
#     suitable for UI consumption.
# =============================================================================
from __future__ import annotations

import json
import os
import re
import time
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable, Dict, Optional, Tuple, List


# -----------------------------
# Schema
# -----------------------------
SCHEMA_VERSION = "1.0.0"


# -----------------------------
# Fallback defaults (only used if no catalog is provided)
# -----------------------------
FALLBACK_DEFAULTS: Dict[str, Any] = {
    "ui": {
        "language": "es",
        "theme": "dark",
        "remember_layout": True,
    },
    "llm": {
        "mode": "manual",
        "target_length": "medium",
        "creativity": 0.7,
    },
}

FALLBACK_TYPES: Dict[str, Any] = {
    "ui.language": str,
    "ui.theme": str,
    "ui.remember_layout": bool,
    "llm.mode": str,
    "llm.target_length": str,
    "llm.creativity": (int, float),
}

FALLBACK_ALLOWED_ENUMS: Dict[str, Tuple[str, ...]] = {
    "ui.language": ("es", "en", "gl", "ca"),
    "ui.theme": ("dark", "light"),
    "llm.mode": ("manual", "auto"),
    "llm.target_length": ("short", "medium", "long"),
}

FALLBACK_RANGES: Dict[str, Tuple[float, float]] = {
    "llm.creativity": (0.0, 1.0),
}

FALLBACK_PATTERNS: Dict[str, str] = {}

# For list preferences, optional item validation rules, keyed by dotted preference key.
# Example:
#   {"llm.fallback_order": {"type":"enum", "allowed":["local","cloud"]}}
FALLBACK_ITEMS: Dict[str, Dict[str, Any]] = {}


# -----------------------------
# Status objects
# -----------------------------
@dataclass
class PreferencesStatus:
    ok: bool = True
    created_new: bool = False
    repaired: bool = False
    backed_up_path: Optional[str] = None
    warnings: List[str] = field(default_factory=list)
    errors: List[str] = field(default_factory=list)
    message: str = ""


@dataclass
class PreferencesPayload:
    schema_version: str
    updated_at: str
    values: Dict[str, Any]  # EFFECTIVE (defaults merged with overrides)


# -----------------------------
# Helpers
# -----------------------------
def _utc_iso_now_z() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def _timestamp_compact_local() -> str:
    return datetime.now().strftime("%Y%m%d_%H%M%S")


def _deep_copy(obj: Any) -> Any:
    return json.loads(json.dumps(obj))


def _get_by_path(d: Dict[str, Any], dotted_path: str) -> Any:
    cur: Any = d
    for part in dotted_path.split("."):
        if not isinstance(cur, dict) or part not in cur:
            raise KeyError(dotted_path)
        cur = cur[part]
    return cur


def _set_by_path(d: Dict[str, Any], dotted_path: str, value: Any) -> None:
    parts = dotted_path.split(".")
    cur: Any = d
    for part in parts[:-1]:
        if part not in cur or not isinstance(cur[part], dict):
            cur[part] = {}
        cur = cur[part]
    cur[parts[-1]] = value


def _delete_by_path(d: Dict[str, Any], dotted_path: str) -> None:
    parts = dotted_path.split(".")
    cur: Any = d
    for part in parts[:-1]:
        if not isinstance(cur, dict) or part not in cur:
            return
        cur = cur[part]
    if isinstance(cur, dict):
        cur.pop(parts[-1], None)


def _prune_empty_dicts(d: Dict[str, Any]) -> None:
    def rec(obj: Any) -> bool:
        if isinstance(obj, dict):
            to_del = []
            for k, v in obj.items():
                empty = rec(v)
                if empty:
                    to_del.append(k)
            for k in to_del:
                obj.pop(k, None)
            return len(obj) == 0
        return False

    rec(d)


def _merge_defaults(defaults: Dict[str, Any], overrides: Dict[str, Any]) -> Dict[str, Any]:
    out = _deep_copy(defaults)

    def rec(dst: Dict[str, Any], src: Dict[str, Any]) -> None:
        for k, v in src.items():
            if isinstance(v, dict) and isinstance(dst.get(k), dict):
                rec(dst[k], v)
            else:
                dst[k] = v

    rec(out, overrides)
    return out


def _load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def _catalog_to_rules(
    catalog: Dict[str, Any],
) -> Tuple[
    Dict[str, Any],
    Dict[str, Any],
    Dict[str, Tuple[str, ...]],
    Dict[str, Tuple[float, float]],
    Dict[str, str],
    Dict[str, Dict[str, Any]],
]:
    prefs = catalog.get("preferences", {})
    defaults_tree: Dict[str, Any] = {}
    types_map: Dict[str, Any] = {}
    enums_map: Dict[str, Tuple[str, ...]] = {}
    ranges_map: Dict[str, Tuple[float, float]] = {}
    patterns_map: Dict[str, str] = {}
    items_map: Dict[str, Dict[str, Any]] = {}

    type_map = {
        "bool": bool,
        "int": int,
        "float": (int, float),
        "string": str,
        "enum": str,
        "list": list,
    }

    for key, spec in prefs.items():
        if not isinstance(key, str) or not isinstance(spec, dict):
            continue

        t = spec.get("type")
        default = spec.get("default", None)

        if "." in key:
            _set_by_path(defaults_tree, key, default)

        if t in type_map:
            types_map[key] = type_map[t]

        if t == "enum":
            allowed = spec.get("allowed", [])
            if isinstance(allowed, list) and all(isinstance(x, str) for x in allowed):
                enums_map[key] = tuple(allowed)

        if t in ("int", "float"):
            r = spec.get("range")
            if isinstance(r, list) and len(r) == 2:
                try:
                    lo, hi = float(r[0]), float(r[1])
                    ranges_map[key] = (lo, hi)
                except Exception:
                    pass

        pattern = spec.get("pattern")
        if isinstance(pattern, str) and pattern:
            patterns_map[key] = pattern

        # list item rules (optional)
        if t == "list":
            items = spec.get("items")
            if isinstance(items, dict):
                ir: Dict[str, Any] = {}
                it = items.get("type")
                if isinstance(it, str):
                    ir["type"] = it
                allowed = items.get("allowed")
                if isinstance(allowed, list) and all(isinstance(x, str) for x in allowed):
                    ir["allowed"] = allowed
                r = items.get("range")
                if isinstance(r, list) and len(r) == 2:
                    try:
                        ir["range"] = (float(r[0]), float(r[1]))
                    except Exception:
                        pass
                pat = items.get("pattern")
                if isinstance(pat, str) and pat:
                    ir["pattern"] = pat
                if ir:
                    items_map[key] = ir

    return defaults_tree, types_map, enums_map, ranges_map, patterns_map, items_map


def _find_leaf_dotted_keys(values: Dict[str, Any]) -> List[str]:
    found: List[str] = []

    def walk(prefix: str, obj: Any) -> None:
        if isinstance(obj, dict):
            for k, v in obj.items():
                p = f"{prefix}.{k}" if prefix else k
                if isinstance(v, dict):
                    walk(p, v)
                else:
                    found.append(p)

    walk("", values)
    return found


# -----------------------------
# PreferencesManager
# -----------------------------
class PreferencesManager:
    def __init__(
        self,
        *,
        preferences_path: Optional[Path] = None,
        path_resolver: Optional[Callable[[], Path]] = None,
        catalog_path: Optional[Path] = None,
        catalog_resolver: Optional[Callable[[], Path]] = None,
        unknown_keys_policy: str = "warn",  # "warn" | "drop" | "error"
        auto_persist_on_warnings: bool = True,
        replace_retries: int = 2,
        replace_retry_sleep_s: float = 0.05,
    ) -> None:
        if preferences_path is None and path_resolver is None:
            raise ValueError(
                "PreferencesManager requires preferences_path or path_resolver "
                "(do not hardcode; resolve from your runtime/config layer)."
            )
        if unknown_keys_policy not in ("warn", "drop", "error"):
            raise ValueError("unknown_keys_policy must be one of: warn, drop, error")

        self._preferences_path = preferences_path
        self._path_resolver = path_resolver
        self._catalog_path = catalog_path
        self._catalog_resolver = catalog_resolver
        self._unknown_keys_policy = unknown_keys_policy
        self._auto_persist_on_warnings = bool(auto_persist_on_warnings)
        self._replace_retries = int(replace_retries)
        self._replace_retry_sleep_s = float(replace_retry_sleep_s)

        self._rules_loaded = False
        self._defaults_tree = FALLBACK_DEFAULTS
        self._types_map = FALLBACK_TYPES
        self._enums_map = FALLBACK_ALLOWED_ENUMS
        self._ranges_map = FALLBACK_RANGES
        self._patterns_map = FALLBACK_PATTERNS
        self._items_map = FALLBACK_ITEMS

    def preferences_path(self) -> Path:
        p = self._preferences_path if self._preferences_path is not None else self._path_resolver()
        return Path(p)

    def catalog_path(self) -> Optional[Path]:
        if self._catalog_path is not None:
            return Path(self._catalog_path)
        if self._catalog_resolver is not None:
            return Path(self._catalog_resolver())
        return None

    # ---------
    # Public API
    # ---------
    def load_or_repair(self) -> Tuple[PreferencesPayload, PreferencesStatus]:
        path = self.preferences_path()
        status = PreferencesStatus(ok=True)
        self._ensure_rules_loaded(status)

        path.parent.mkdir(parents=True, exist_ok=True)

        if not path.exists():
            overrides_payload = self._new_overrides_payload(values={})
            self._save_payload_atomic(path, overrides_payload)
            status.created_new = True
            status.message = "Preferences file did not exist; created overrides-only file."
            effective = _merge_defaults(self._defaults_tree, overrides_payload["values"])
            return PreferencesPayload(schema_version=SCHEMA_VERSION, updated_at=overrides_payload["updated_at"], values=effective), status

        try:
            data = _load_json(path)
        except json.JSONDecodeError:
            backup = self._backup_corrupt(path)
            overrides_payload = self._new_overrides_payload(values={})
            self._save_payload_atomic(path, overrides_payload)
            status.repaired = True
            status.backed_up_path = str(backup)
            status.warnings.append("Preferences JSON was corrupt and has been regenerated (overrides-only).")
            status.message = f"Corrupt preferences detected; backed up to {backup.name} and regenerated."
            effective = _merge_defaults(self._defaults_tree, overrides_payload["values"])
            return PreferencesPayload(schema_version=SCHEMA_VERSION, updated_at=overrides_payload["updated_at"], values=effective), status
        except OSError as e:
            status.ok = False
            status.errors.append(f"Failed to read preferences: {e}")
            status.message = "Failed to read preferences file."
            effective = _merge_defaults(self._defaults_tree, {})
            return PreferencesPayload(schema_version=SCHEMA_VERSION, updated_at=_utc_iso_now_z(), values=effective), status

        overrides_payload = self._normalize_overrides_payload(data, status)
        self._validate_overrides(overrides_payload["values"], status)

        effective = _merge_defaults(self._defaults_tree, overrides_payload["values"])

        if status.warnings and self._auto_persist_on_warnings:
            try:
                overrides_payload["updated_at"] = _utc_iso_now_z()
                self._save_payload_atomic(path, overrides_payload)
            except OSError as e:
                status.ok = False
                status.errors.append(f"Failed to persist normalized preferences: {e}")

        return PreferencesPayload(schema_version=SCHEMA_VERSION, updated_at=overrides_payload["updated_at"], values=effective), status

    def set_preferences(self, patch: Dict[str, Any]) -> Tuple[PreferencesPayload, PreferencesStatus]:
        path = self.preferences_path()
        status = PreferencesStatus(ok=True)
        self._ensure_rules_loaded(status)

        overrides_payload, status2 = self._load_overrides_or_repair()
        status.warnings.extend(status2.warnings)
        status.errors.extend(status2.errors)
        status.ok = status.ok and status2.ok
        status.created_new = status.created_new or status2.created_new
        status.repaired = status.repaired or status2.repaired
        status.backed_up_path = status.backed_up_path or status2.backed_up_path

        overrides = overrides_payload["values"]

        if not isinstance(patch, dict):
            status.ok = False
            status.errors.append("Patch must be a dict of dotted keys -> values.")
            status.message = "Preferences not saved due to validation errors."
            effective = _merge_defaults(self._defaults_tree, overrides)
            return PreferencesPayload(schema_version=SCHEMA_VERSION, updated_at=overrides_payload["updated_at"], values=effective), status

        for k, v in patch.items():
            if not isinstance(k, str) or "." not in k:
                status.ok = False
                status.errors.append(f"Invalid patch key '{k}'. Expected dotted path like 'ui.theme'.")
                continue

            if k not in self._types_map:
                status.ok = False
                status.errors.append(f"Unknown preference key '{k}'.")
                continue

            expected = self._types_map[k]
            if not isinstance(v, expected):
                status.ok = False
                status.errors.append(f"Key '{k}' expects {expected}, got {type(v)}.")
                continue

            # list items (strict)
            if expected == list:
                if not self._validate_list_value(k, v, status, strict=True):
                    continue

            if k in self._enums_map and v not in self._enums_map[k]:
                status.ok = False
                status.errors.append(f"Key '{k}' must be one of {self._enums_map[k]}, got '{v}'.")
                continue

            if k in self._ranges_map:
                lo, hi = self._ranges_map[k]
                fv = float(v)
                if fv < lo or fv > hi:
                    status.ok = False
                    status.errors.append(f"Key '{k}' must be in range [{lo}, {hi}], got {fv}.")
                    continue
                v = fv

            pat = self._patterns_map.get(k)
            if pat and isinstance(v, str):
                if re.fullmatch(pat, v) is None:
                    status.ok = False
                    status.errors.append(f"Key '{k}' does not match pattern '{pat}'.")
                    continue

            _set_by_path(overrides, k, v)

        if status.errors:
            status.message = "Preferences not saved due to validation errors."
            effective = _merge_defaults(self._defaults_tree, overrides)
            return PreferencesPayload(schema_version=SCHEMA_VERSION, updated_at=overrides_payload["updated_at"], values=effective), status

        self._validate_overrides(overrides, status)

        overrides_payload["schema_version"] = SCHEMA_VERSION
        overrides_payload["updated_at"] = _utc_iso_now_z()
        overrides_payload["values"] = overrides

        try:
            self._save_payload_atomic(path, overrides_payload)
            status.message = "Preferences updated successfully."
        except OSError as e:
            status.ok = False
            status.errors.append(f"Failed to save preferences: {e}")
            status.message = "Failed to save preferences."

        effective = _merge_defaults(self._defaults_tree, overrides)
        return PreferencesPayload(schema_version=SCHEMA_VERSION, updated_at=overrides_payload["updated_at"], values=effective), status

    def reset_preferences(self, *, keep_backup: bool = True) -> Tuple[PreferencesPayload, PreferencesStatus]:
        path = self.preferences_path()
        status = PreferencesStatus(ok=True)
        self._ensure_rules_loaded(status)

        path.parent.mkdir(parents=True, exist_ok=True)

        if keep_backup and path.exists():
            try:
                backup = path.with_name(f"{path.name}.bak-{_timestamp_compact_local()}")
                path.replace(backup)
                status.backed_up_path = str(backup)
                status.warnings.append(f"Backed up previous preferences to {backup.name}.")
            except OSError as e:
                status.ok = False
                status.errors.append(f"Failed to backup existing preferences: {e}")
                status.message = "Reset aborted due to backup failure."
                effective = _merge_defaults(self._defaults_tree, {})
                return PreferencesPayload(schema_version=SCHEMA_VERSION, updated_at=_utc_iso_now_z(), values=effective), status

        overrides_payload = self._new_overrides_payload(values={})
        try:
            self._save_payload_atomic(path, overrides_payload)
            status.message = "Preferences reset to defaults (overrides cleared)."
        except OSError as e:
            status.ok = False
            status.errors.append(f"Failed to write preferences: {e}")
            status.message = "Failed to reset preferences."

        effective = _merge_defaults(self._defaults_tree, {})
        return PreferencesPayload(schema_version=SCHEMA_VERSION, updated_at=overrides_payload["updated_at"], values=effective), status

    def validate_only(self) -> PreferencesStatus:
        status = PreferencesStatus(ok=True)
        self._ensure_rules_loaded(status)
        path = self.preferences_path()

        if not path.exists():
            status.ok = False
            status.errors.append("Preferences file is missing.")
            status.message = "Missing preferences file."
            return status

        try:
            data = _load_json(path)
        except json.JSONDecodeError:
            status.ok = False
            status.errors.append("Preferences file is corrupt JSON.")
            status.message = "Corrupt preferences JSON."
            return status
        except OSError as e:
            status.ok = False
            status.errors.append(f"Failed to read preferences: {e}")
            status.message = "Failed to read preferences."
            return status

        overrides_payload = self._normalize_overrides_payload(data, status)
        self._validate_overrides(overrides_payload["values"], status)
        status.message = "Validation complete."
        return status

    # ---------
    # Internal
    # ---------
    def _ensure_rules_loaded(self, status: Optional[PreferencesStatus] = None) -> None:
        if self._rules_loaded:
            return

        cp = self.catalog_path()
        if cp is None:
            self._rules_loaded = True
            return

        try:
            catalog = _load_json(cp)
            defaults_tree, types_map, enums_map, ranges_map, patterns_map, items_map = _catalog_to_rules(catalog)

            if defaults_tree:
                self._defaults_tree = defaults_tree
            if types_map:
                self._types_map = types_map
            self._enums_map = enums_map
            self._ranges_map = ranges_map
            self._patterns_map = patterns_map
            self._items_map = items_map

        except Exception as e:
            if status is not None:
                status.warnings.append(f"Failed to load preferences catalog ({cp}): {e}. Using fallback rules.")
        finally:
            self._rules_loaded = True

    def _new_overrides_payload(self, *, values: Dict[str, Any]) -> Dict[str, Any]:
        return {
            "schema_version": SCHEMA_VERSION,
            "updated_at": _utc_iso_now_z(),
            "values": _deep_copy(values),
        }

    def _normalize_overrides_payload(self, data: Any, status: PreferencesStatus) -> Dict[str, Any]:
        if not isinstance(data, dict):
            status.warnings.append("Preferences root was not an object; replacing with overrides-only defaults.")
            return self._new_overrides_payload(values={})

        schema_version = data.get("schema_version")
        updated_at = data.get("updated_at")
        values = data.get("values")

        if not isinstance(schema_version, str):
            status.warnings.append("Missing/invalid schema_version; normalizing.")
            schema_version = SCHEMA_VERSION

        if not isinstance(updated_at, str):
            status.warnings.append("Missing/invalid updated_at; normalizing.")
            updated_at = _utc_iso_now_z()

        if not isinstance(values, dict):
            status.warnings.append("Missing/invalid values; replacing with empty overrides.")
            values = {}

        if schema_version != SCHEMA_VERSION:
            status.warnings.append(
                f"schema_version mismatch (found {schema_version}, expected {SCHEMA_VERSION}). "
                "MVP keeps values; consider migrations if needed."
            )

        return {
            "schema_version": str(schema_version),
            "updated_at": str(updated_at),
            "values": values,
        }

    def _validate_list_value(self, key: str, value: Any, status: PreferencesStatus, *, strict: bool) -> bool:
        if not isinstance(value, list):
            msg = f"Key '{key}' expects a list."
            if strict:
                status.ok = False
                status.errors.append(msg)
            else:
                status.warnings.append(msg)
            return False

        rules = self._items_map.get(key) or {}
        if not rules:
            return True

        it = rules.get("type")
        allowed = rules.get("allowed")
        rng = rules.get("range")
        pat = rules.get("pattern")

        type_check: Any = None
        if it in ("string", "enum"):
            type_check = str
        elif it == "int":
            type_check = int
        elif it == "float":
            type_check = (int, float)
        elif it == "bool":
            type_check = bool

        for i, elem in enumerate(value):
            if type_check is not None and not isinstance(elem, type_check):
                msg = f"Key '{key}' has invalid item type at index {i}."
                if strict:
                    status.ok = False
                    status.errors.append(msg)
                else:
                    status.warnings.append(msg)
                return False

            if isinstance(allowed, list) and isinstance(elem, str) and elem not in allowed:
                msg = f"Key '{key}' has invalid item value '{elem}' at index {i}."
                if strict:
                    status.ok = False
                    status.errors.append(msg)
                else:
                    status.warnings.append(msg)
                return False

            if rng is not None and isinstance(elem, (int, float)):
                lo, hi = float(rng[0]), float(rng[1])
                fv = float(elem)
                if fv < lo or fv > hi:
                    msg = f"Key '{key}' has out-of-range item at index {i} ({fv} not in [{lo}, {hi}])."
                    if strict:
                        status.ok = False
                        status.errors.append(msg)
                    else:
                        status.warnings.append(msg)
                    return False

            if isinstance(pat, str) and isinstance(elem, str):
                if re.fullmatch(pat, elem) is None:
                    msg = f"Key '{key}' has item that does not match pattern at index {i}."
                    if strict:
                        status.ok = False
                        status.errors.append(msg)
                    else:
                        status.warnings.append(msg)
                    return False

        return True

    def _validate_overrides(self, overrides: Dict[str, Any], status: PreferencesStatus) -> None:
        if not isinstance(overrides, dict):
            status.warnings.append("values is not a dict; resetting to empty overrides.")
            overrides.clear()
            return

        leaf_keys = _find_leaf_dotted_keys(overrides)

        unknown = [k for k in leaf_keys if k not in self._types_map]
        if unknown:
            msg = f"Unknown preference keys present: {sorted(unknown)}"
            if self._unknown_keys_policy == "warn":
                status.warnings.append(msg)
            elif self._unknown_keys_policy == "drop":
                status.warnings.append(msg + " (dropped)")
                for k in unknown:
                    _delete_by_path(overrides, k)
                _prune_empty_dicts(overrides)
            elif self._unknown_keys_policy == "error":
                status.ok = False
                status.errors.append(msg)

        for k in [kk for kk in leaf_keys if kk in self._types_map]:
            try:
                v = _get_by_path(overrides, k)
            except KeyError:
                continue

            expected = self._types_map[k]
            if not isinstance(v, expected):
                status.warnings.append(f"Invalid type for '{k}', dropping override.")
                _delete_by_path(overrides, k)
                continue

            if expected == list and (not self._validate_list_value(k, v, status, strict=False)):
                status.warnings.append(f"Dropping invalid list override for '{k}'.")
                _delete_by_path(overrides, k)
                continue

            if k in self._enums_map and v not in self._enums_map[k]:
                status.warnings.append(f"Invalid value for '{k}', dropping override.")
                _delete_by_path(overrides, k)
                continue

            if k in self._ranges_map:
                lo, hi = self._ranges_map[k]
                fv = float(v)
                if fv < lo or fv > hi:
                    status.warnings.append(f"Out-of-range '{k}', dropping override.")
                    _delete_by_path(overrides, k)
                    continue
                _set_by_path(overrides, k, fv)

            pat = self._patterns_map.get(k)
            if pat and isinstance(v, str):
                if re.fullmatch(pat, v) is None:
                    status.warnings.append(f"Pattern mismatch '{k}', dropping override.")
                    _delete_by_path(overrides, k)
                    continue

        _prune_empty_dicts(overrides)

    def _backup_corrupt(self, path: Path) -> Path:
        backup = path.with_name(f"{path.name}.bad-{_timestamp_compact_local()}")
        try:
            path.replace(backup)
        except OSError:
            try:
                backup.write_bytes(path.read_bytes())
            except OSError:
                pass
        return backup

    def _save_payload_atomic(self, path: Path, payload: Dict[str, Any]) -> None:
        tmp = path.with_name(f"{path.name}.tmp")
        text = json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True)

        with open(tmp, "w", encoding="utf-8", newline="\n") as f:
            f.write(text)
            f.flush()
            os.fsync(f.fileno())

        for attempt in range(self._replace_retries + 1):
            try:
                os.replace(tmp, path)
                return
            except OSError as e:
                if attempt < self._replace_retries:
                    time.sleep(self._replace_retry_sleep_s)
                else:
                    raise OSError(f"os.replace failed after {self._replace_retries + 1} attempts: {e}") from e

    def _load_overrides_or_repair(self) -> Tuple[Dict[str, Any], PreferencesStatus]:
        path = self.preferences_path()
        status = PreferencesStatus(ok=True)

        path.parent.mkdir(parents=True, exist_ok=True)

        if not path.exists():
            payload = self._new_overrides_payload(values={})
            self._save_payload_atomic(path, payload)
            status.created_new = True
            status.message = "Preferences file did not exist; created overrides-only file."
            return payload, status

        try:
            data = _load_json(path)
        except json.JSONDecodeError:
            backup = self._backup_corrupt(path)
            payload = self._new_overrides_payload(values={})
            self._save_payload_atomic(path, payload)
            status.repaired = True
            status.backed_up_path = str(backup)
            status.warnings.append("Preferences JSON was corrupt and has been regenerated (overrides-only).")
            status.message = f"Corrupt preferences detected; backed up to {backup.name} and regenerated."
            return payload, status
        except OSError as e:
            status.ok = False
            status.errors.append(f"Failed to read preferences: {e}")
            status.message = "Failed to read preferences file."
            return self._new_overrides_payload(values={}), status

        payload = self._normalize_overrides_payload(data, status)
        self._validate_overrides(payload["values"], status)

        if status.warnings and self._auto_persist_on_warnings:
            try:
                payload["updated_at"] = _utc_iso_now_z()
                self._save_payload_atomic(path, payload)
            except OSError as e:
                status.ok = False
                status.errors.append(f"Failed to persist normalized preferences: {e}")

        return payload, status


def make_manager_from_resolver(path_resolver: Callable[[], Path]) -> PreferencesManager:
    return PreferencesManager(path_resolver=path_resolver)
