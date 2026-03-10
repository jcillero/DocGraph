"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_get_credentials.py",
  "tool_id": "tool_get_credentials",
  "version": "1.0.1",
  "type": "runtime_tool",
  "category": "runtime",
  "description": "Lee credenciales desde user/registry/credentials.json; repara o regenera plantilla si falta o está corrupto. Opción de salida enmascarada.",
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

from tool_common import emit, resolve_user_registry_file

# PROPOSAL: adjust import path to match your tree (system/core/...)
from credentials_manager import CredentialsManager


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--creds-path", type=str, default=None, help="Explicit path to credentials.json (no hardcode).")
    ap.add_argument("--masked", action="store_true", help="Mask common secret fields in output.")
    args = ap.parse_args()

    creds_path = Path(args.creds_path) if args.creds_path else resolve_user_registry_file("credentials.json")
    mgr = CredentialsManager(credentials_path=creds_path)

    payload, status = mgr.load_or_repair()

    providers = payload.providers
    if args.masked:
        providers = {}
        for prov in payload.providers.keys():
            providers[prov] = mgr.get_provider(prov, masked=True)

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
            "credentials_path": str(mgr.credentials_path()),
            "credentials": {
                "schema_version": payload.schema_version,
                "credentials_id": payload.credentials_id,
                "updated_at": payload.updated_at,
                "providers": providers,
            },
        },
        exit_code=0 if status.ok and (not status.errors) else 1,
    )


if __name__ == "__main__":
    main()
