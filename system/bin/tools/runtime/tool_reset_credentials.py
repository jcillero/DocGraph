"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_reset_credentials.py",
  "tool_id": "tool_reset_credentials",
  "version": "1.0.0",
  "type": "runtime_tool",
  "category": "runtime",
  "description": "Resetea credenciales a plantilla vacía en user/registry/credentials.json; opcionalmente crea backup .bak antes del reset.",
  "location_expected": "system/bin/tools/runtime",
  "entry_point": "main",
  "runtime_required": true,
  "modifies_system": false,
  "outputs": [
    {
      "type": "stdout_json",
      "role": "reset_report"
    }
  ],
  "capabilities": [
    "credentials_reset",
    "registry_repair",
    "backup_creation"
  ],
  "depends_on": [
    "credentials_manager"
  ],
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

# PROPOSAL: adjust import to your final location
from credentials_manager import CredentialsManager


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--creds-path", type=str, default=None)
    ap.add_argument("--no-backup", action="store_true", help="Do not create a .bak-* backup before reset.")
    args = ap.parse_args()

    creds_path = Path(args.creds_path) if args.creds_path else resolve_user_registry_file("credentials.json")
    mgr = CredentialsManager(credentials_path=creds_path)

    payload, status = mgr.reset_credentials(keep_backup=not args.no_backup)

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
                "providers": payload.providers,
            },
        },
        exit_code=0 if status.ok and (not status.errors) else 1,
    )


if __name__ == "__main__":
    main()
