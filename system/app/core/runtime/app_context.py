#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "app_context.py",
  "version": "1.0.0",
  "type": "runtime_core",
  "category": "runtime",
  "description": "Construye y expone el contexto global de la aplicación portable (rutas, runtime y configuraciones).",
  "location_expected": "system/core/runtime/",
  "runtime_required": true,
  "modifies_system": false,
  "outputs": [],
  "depends_on": [
    "system/core/runtime/paths.py"
  ],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "DEV_RUNBOOK"
  ]
}
================================================================================
"""

from __future__ import annotations

import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Optional, Dict, Any

from system.core.runtime.paths import resolve_app_root, resolve_system_root


# -----------------------------------------------------------------------------
# Data model
# -----------------------------------------------------------------------------

@dataclass(frozen=True)
class AppContext:
    """Estructura inmutable con el contexto completo de la app."""

    app_root: Path
    system_root: Path
    user_root: Path

    config_dir: Path
    runs_dir: Path
    input_dir: Path
    output_dir: Path

    runtime_registry_path: Path
    entrypoints_path: Path

    python_executable: Path
    is_portable_runtime: bool


# -----------------------------------------------------------------------------
# Context builder
# -----------------------------------------------------------------------------

def build_app_context() -> AppContext:
    """Construye el contexto completo de la app portable."""

    app_root = resolve_app_root()
    system_root = resolve_system_root(app_root)

    user_root = app_root / "user"

    config_dir = system_root / "config"
    runs_dir = user_root / "runs"
    input_dir = user_root / "input"
    output_dir = user_root / "output"

    runtime_registry_path = config_dir / "runtime_registry.json"
    entrypoints_path = config_dir / "entrypoints.json"

    python_executable = Path(sys.executable)

    portable_runtime = "system/bin/runtimes/python" in str(python_executable).replace("\\", "/")

    return AppContext(
        app_root=app_root,
        system_root=system_root,
        user_root=user_root,
        config_dir=config_dir,
        runs_dir=runs_dir,
        input_dir=input_dir,
        output_dir=output_dir,
        runtime_registry_path=runtime_registry_path,
        entrypoints_path=entrypoints_path,
        python_executable=python_executable,
        is_portable_runtime=portable_runtime,
    )


# -----------------------------------------------------------------------------
# Helper accessors
# -----------------------------------------------------------------------------

_CONTEXT_CACHE: Optional[AppContext] = None


def get_app_context() -> AppContext:
    """Devuelve un contexto singleton para evitar recalcular rutas."""

    global _CONTEXT_CACHE

    if _CONTEXT_CACHE is None:
        _CONTEXT_CACHE = build_app_context()

    return _CONTEXT_CACHE


# -----------------------------------------------------------------------------
# Diagnostics
# -----------------------------------------------------------------------------

def context_to_dict(ctx: Optional[AppContext] = None) -> Dict[str, Any]:
    """Devuelve el contexto en formato serializable."""

    ctx = ctx or get_app_context()

    return {
        "app_root": str(ctx.app_root),
        "system_root": str(ctx.system_root),
        "user_root": str(ctx.user_root),
        "config_dir": str(ctx.config_dir),
        "runs_dir": str(ctx.runs_dir),
        "input_dir": str(ctx.input_dir),
        "output_dir": str(ctx.output_dir),
        "runtime_registry_path": str(ctx.runtime_registry_path),
        "entrypoints_path": str(ctx.entrypoints_path),
        "python_executable": str(ctx.python_executable),
        "is_portable_runtime": ctx.is_portable_runtime,
    }


# -----------------------------------------------------------------------------
# CLI diagnostic mode
# -----------------------------------------------------------------------------

def _main() -> int:
    ctx = get_app_context()
    data = context_to_dict(ctx)

    import json
    print(json.dumps(data, indent=2, ensure_ascii=False))

    return 0


if __name__ == "__main__":
    raise SystemExit(_main())
