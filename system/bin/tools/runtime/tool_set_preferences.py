"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_set_preferences.py",
  "tool_id": "tool_set_preferences",
  "version": "1.0.0",
  "type": "runtime_tool",
  "category": "runtime",
  "description": "Actualiza preferencias del usuario (user/) con validación básica y persistencia. No modifica system/.",
  "location_expected": "system/bin/tools/runtime",
  "entry_point": "main",
  "runtime_required": true,
  "modifies_system": false,
  "output_location": null,
  "outputs": [],
  "depends_on": [],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "DEV_RUNBOOK",
    "MANUAL DE TIPOS DE DOCUMENTO"
  ]
}
================================================================================
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from tool_common import emit, resolve_user_registry_file, resolve_system_spec_file

from preferences_manager import PreferencesManager


def _load_patch(args: argparse.Namespace) -> dict:
    if args.patch_json:
        return json.loads(args.patch_json)
    if args.patch_file:
        return json.loads(Path(args.patch_file).read_text(encoding="utf-8"))
    if not sys.stdin.isatty():
        return json.loads(sys.stdin.read())
    raise ValueError("No patch provided. Use --patch-json, --patch-file, or stdin.")


def _make_mgr(prefs_path: Path, args: argparse.Namespace) -> PreferencesManager:
    if args.catalog_path:
        return PreferencesManager(preferences_path=prefs_path, catalog_path=Path(args.catalog_path))
    return PreferencesManager(
        preferences_path=prefs_path,
        catalog_resolver=lambda: resolve_system_spec_file("preferences_catalog.json"),
    )


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--prefs-path", type=str, default=None)
    ap.add_argument("--catalog-path", type=str, default=None)
    ap.add_argument("--patch-json", type=str, default=None, help='JSON dict like {"ui.theme":"light"}')
    ap.add_argument("--patch-file", type=str, default=None, help="Path to JSON file containing patch dict.")
    args = ap.parse_args()

    prefs_path = Path(args.prefs_path) if args.prefs_path else resolve_user_registry_file("preferences.json")
    mgr = _make_mgr(prefs_path, args)

    try:
        patch = _load_patch(args)
        if not isinstance(patch, dict):
            raise ValueError("Patch must be a JSON object/dict.")
    except Exception as e:
        emit({"ok": False, "error": f"Invalid patch: {e}"}, exit_code=2)

    payload, status = mgr.set_preferences(patch)

    emit(
        {
            "ok": status.ok and (not status.errors),
            "status": {
                "created_new": status.created_new,
                "repaired": status.repaired,
                "backed_up_path": status.backed_up_path,
                "warnings": status.warnings,
                "errors": status.errors,
                "message": status.message,
            },
            "preferences_path": str(mgr.preferences_path()),
            "catalog_path": str(mgr.catalog_path()) if mgr.catalog_path() else None,
            "preferences": {
                "schema_version": payload.schema_version,
                "updated_at": payload.updated_at,
                "values": payload.values,
            },
        },
        exit_code=0 if status.ok and (not status.errors) else 1,
    )


if __name__ == "__main__":
    main()
