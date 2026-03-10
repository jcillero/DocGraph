"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_set_credentials.py",
  "tool_id": "tool_set_credentials",
  "version": "1.0.0",
  "type": "runtime_tool",
  "category": "runtime",
  "description": "Actualiza credenciales en user/registry/credentials.json con valores suministrados por CLI, preservando estructura y validando campos básicos.",
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

from tool_common import emit, resolve_user_registry_file

# PROPOSAL: adjust import to your final location
from credentials_manager import CredentialsManager


def _load_patch(args: argparse.Namespace) -> dict:
    if args.patch_json:
        return json.loads(args.patch_json)
    if args.patch_file:
        return json.loads(Path(args.patch_file).read_text(encoding="utf-8"))
    if not sys.stdin.isatty():
        return json.loads(sys.stdin.read())
    raise ValueError("No patch provided. Use --patch-json, --patch-file, or stdin.")


def _patch_to_fields(patch: dict, provider: str) -> dict:
    # Accept either {"api_key":"..."} OR {"provider.api_key":"..."}
    if not isinstance(patch, dict):
        raise ValueError("Patch must be a JSON object/dict.")

    dotted = [k for k in patch.keys() if isinstance(k, str) and "." in k]
    if dotted:
        fields = {}
        for k, v in patch.items():
            if not isinstance(k, str) or "." not in k:
                continue
            prov, field = k.split(".", 1)
            if prov != provider:
                continue
            fields[field] = v
        if not fields:
            raise ValueError("Dotted patch did not include any fields for the selected provider.")
        return fields

    return patch


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--creds-path", type=str, default=None, help="Explicit path to credentials.json (no hardcode).")
    ap.add_argument("--provider", type=str, required=True, help="Provider id, e.g. openai|anthropic|azure_openai")
    ap.add_argument("--patch-json", type=str, default=None, help='JSON dict like {"api_key":"..."}')
    ap.add_argument("--patch-file", type=str, default=None, help="Path to JSON file containing patch dict.")
    ap.add_argument("--masked", action="store_true", help="Mask secrets in returned provider data.")
    args = ap.parse_args()

    creds_path = Path(args.creds_path) if args.creds_path else resolve_user_registry_file("credentials.json")
    mgr = CredentialsManager(credentials_path=creds_path)

    try:
        patch = _load_patch(args)
        fields = _patch_to_fields(patch, args.provider)
    except Exception as e:
        emit({"ok": False, "error": f"Invalid patch: {e}"}, exit_code=2)

    payload, status = mgr.set_provider_fields(args.provider, fields)

    providers = payload.providers
    if args.masked:
        providers = dict(payload.providers)
        providers[args.provider] = mgr.get_provider(args.provider, masked=True)

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
