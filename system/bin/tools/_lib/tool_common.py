"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_common",
  "version": "1.0.0",
  "type": "tool_module",
  "category": "tools_lib",
  "description": "Utilidades compartidas para tools: emisión JSON, resolución de rutas del árbol portable y helpers de salida.",
  "location_expected": "system/bin/tools/_lib/",
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

import json
import sys
from pathlib import Path
from typing import Optional


def emit(obj: dict, exit_code: int = 0) -> None:
    sys.stdout.write(json.dumps(obj, ensure_ascii=False, indent=2) + "\n")
    raise SystemExit(exit_code)


def find_project_root(start: Optional[Path] = None) -> Path:
    """Walk parents until both 'system' and 'user' directories exist."""
    p = (start or Path(__file__)).resolve()
    for cand in [p] + list(p.parents):
        if (cand / "system").is_dir() and (cand / "user").is_dir():
            return cand
    cwd = Path.cwd().resolve()
    if (cwd / "system").is_dir() and (cwd / "user").is_dir():
        return cwd
    raise FileNotFoundError("Project root not found (expected 'system/' and 'user/' directories).")


def resolve_user_registry_file(filename: str, *, start: Optional[Path] = None) -> Path:
    root = find_project_root(start)
    return root / "user" / "registry" / filename


def resolve_system_spec_file(filename: str, *, start: Optional[Path] = None) -> Path:
    root = find_project_root(start)
    return root / "system" / "spec" / filename
