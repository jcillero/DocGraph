"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_validate_preferences.py",
  "tool_id": "tool_validate_preferences",
  "version": "1.0.0",
  "type": "runtime_tool",
  "category": "runtime",
  "description": "Valida preferencias actuales del usuario contra el catálogo de preferencias del sistema; reporta errores y avisos.",
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
from pathlib import Path

from tool_common import emit, resolve_user_registry_file, resolve_system_spec_file
from preferences_manager import PreferencesManager


def _make_mgr(prefs_path: Path, args: argparse.Namespace) -> PreferencesManager:
    if args.catalog_path:
        return PreferencesManager(
            preferences_path=prefs_path,
            catalog_path=Path(args.catalog_path),
        )

    return PreferencesManager(
        preferences_path=prefs_path,
        catalog_resolver=lambda: resolve_system_spec_file("preferences_catalog.json"),
    )


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--prefs-path", type=str, default=None)
    ap.add_argument("--catalog-path", type=str, default=None)
    args = ap.parse_args()

    prefs_path = (
        Path(args.prefs_path)
        if args.prefs_path
        else resolve_user_registry_file("preferences.json")
    )

    mgr = _make_mgr(prefs_path, args)

    status = mgr.validate_only()

    emit(
        {
            "ok": status.ok and (not status.errors),
            "preferences_path": str(prefs_path),
            "catalog_path": str(mgr.catalog_path()) if mgr.catalog_path() else None,
            "validation": {
                "warnings": status.warnings,
                "errors": status.errors,
                "message": status.message,
            },
        },
        exit_code=0 if status.ok and (not status.errors) else 1,
    )


if __name__ == "__main__":
    main()
