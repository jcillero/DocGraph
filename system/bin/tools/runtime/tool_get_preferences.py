"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_get_preferences.py",
  "tool_id": "tool_get_preferences",
  "version": "1.0.0",
  "type": "runtime_tool",
  "category": "runtime",
  "description": "Devuelve preferencias del usuario (almacenadas en user/) y/o valores por defecto desde catálogo de system/spec. Soporta reparación si falta el fichero.",
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


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--prefs-path", type=str, default=None, help="Explicit path to preferences.json (no hardcode).")
    ap.add_argument("--catalog-path", type=str, default=None, help="Explicit path to preferences_catalog.json (optional).")
    args = ap.parse_args()

    prefs_path = Path(args.prefs_path) if args.prefs_path else resolve_user_registry_file("preferences.json")

    if args.catalog_path:
        catalog_path = Path(args.catalog_path)
        mgr = PreferencesManager(preferences_path=prefs_path, catalog_path=catalog_path)
    else:
        mgr = PreferencesManager(
            preferences_path=prefs_path,
            catalog_resolver=lambda: resolve_system_spec_file("preferences_catalog.json"),
        )

    payload, status = mgr.load_or_repair()

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
