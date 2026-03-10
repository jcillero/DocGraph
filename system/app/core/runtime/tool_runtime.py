#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_runtime.py",
  "version": "1.0.0",
  "type": "runtime_core",
  "category": "runtime",
  "description": "Bootstrap premium para tools: app_context + run_context + logger estructurado.",
  "location_expected": "system/core/runtime/",
  "runtime_required": true,
  "modifies_system": false,
  "outputs": [],
  "depends_on": [
    "system/core/runtime/app_context.py",
    "system/core/logging/run_context.py",
    "system/core/logging/logging_runtime.py"
  ],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "DEV_RUNBOOK"
  ]
}
================================================================================
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Optional, Tuple

from system.core.runtime.app_context import AppContext, get_app_context
from system.core.logging.run_context import create_run_context
from system.core.logging.logging_runtime import init_logger, WorkUnit, WorkUnits


@dataclass(frozen=True)
class ToolRuntime:
    """Contexto operativo estándar para una tool."""

    tool_id: str
    tool_version: str
    runtime_version: str
    app: AppContext
    run_dir: Path
    logger: Any


def init_tool_runtime(
    *,
    tool_id: str,
    tool_version: str,
    runtime_version: str = "python_portable",
    inputs: Optional[list[dict[str, Any]]] = None,
    primary_work_unit_name: str = "actions",
    primary_work_unit_count: int = 1,
) -> ToolRuntime:
    """Inicializa contexto + run_context + logger para una tool."""

    app = get_app_context()

    run_ctx = create_run_context(app.app_root, tool_id=tool_id)
    logger = init_logger(
        run_ctx,
        tool_version=tool_version,
        runtime_version=runtime_version,
        inputs=inputs or [],
        work_units=WorkUnits(
            primary=WorkUnit(primary_work_unit_name, primary_work_unit_count)
        ),
    )

    return ToolRuntime(
        tool_id=tool_id,
        tool_version=tool_version,
        runtime_version=runtime_version,
        app=app,
        run_dir=run_ctx.run_dir,
        logger=logger,
    )


def close_tool_runtime(
    runtime: ToolRuntime,
    *,
    status: str,
    exit_code: int,
    summary: str,
    primary_work_unit_name: str = "actions",
    primary_work_unit_count: int = 1,
) -> None:
    """Cierre estándar del logger de una tool."""

    runtime.logger.close(
        status=status,
        exit_code=exit_code,
        work_units=WorkUnits(
            primary=WorkUnit(primary_work_unit_name, primary_work_unit_count)
        ),
        summary=summary,
    )


def artifact_relpath(runtime: ToolRuntime, path: Path) -> str:
    """Convierte una ruta absoluta a ruta relativa POSIX respecto a app_root."""

    try:
        return str(path.relative_to(runtime.app.app_root)).replace("\\", "/")
    except Exception:
        return str(path).replace("\\", "/")


def mark_artifact_written(runtime: ToolRuntime, category: str, path: Path) -> str:
    """Registra un artefacto escrito y devuelve su ruta relativa POSIX."""

    rel = artifact_relpath(runtime, path)
    runtime.logger.artifact_written(category, rel)
    return rel


def runtime_info(runtime: ToolRuntime) -> Dict[str, Any]:
    """Devuelve metadatos serializables del runtime actual."""

    return {
        "tool_id": runtime.tool_id,
        "tool_version": runtime.tool_version,
        "runtime_version": runtime.runtime_version,
        "app_root": str(runtime.app.app_root),
        "system_root": str(runtime.app.system_root),
        "user_root": str(runtime.app.user_root),
        "run_dir": str(runtime.run_dir),
        "python_executable": str(runtime.app.python_executable),
        "is_portable_runtime": runtime.app.is_portable_runtime,
    }


def _main() -> int:
    rt = init_tool_runtime(
        tool_id="tool_runtime_selftest",
        tool_version="1.0.0",
        primary_work_unit_name="checks",
        primary_work_unit_count=1,
    )

    exit_code = 1
    status = "failed"

    try:
        info = runtime_info(rt)
        print(info)

        exit_code = 0
        status = "completed"
        return 0

    finally:
        close_tool_runtime(
            rt,
            status=status,
            exit_code=exit_code,
            summary="tool_runtime selftest",
            primary_work_unit_name="checks",
            primary_work_unit_count=1,
        )


if __name__ == "__main__":
    raise SystemExit(_main())
