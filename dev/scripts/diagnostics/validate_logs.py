#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "validate_logs.py",
  "version": "1.0.0",
  "type": "devtool",
  "category": "dev",
  "description": "Valida estructura y coherencia de logs (Logging v1) generados en user/runs/**.",
  "location_expected": "system/devtools/logging",
  "entry_point": "main",
  "inputs": [
    {
      "type": "log_directory",
      "location": "user/runs/**",
      "via_argument": "--runs_dir"
    },
    {
      "type": "catalog",
      "location": "system/config/logging/logging_event_types_v1.json"
    },
    {
      "type": "catalog",
      "location": "system/config/logging/work_units_catalog_v1.json"
    }
  ],
  "outputs": [
    {
      "type": "stdout_report"
    }
  ],
  "modifies_system": false,
  "runtime": "python_portable",
  "logging": {
    "spec": "logging_v1"
  }
}
================================================================================
validate_logs.py

Devtool: valida artefactos de logging (Logging v1) en user/runs/**.

Valida:
- Estructura de metadata y eventos
- Coherencia run_start / run_end
- duration_ms y work_units obligatorios donde corresponda
- stage, level y event_type contra catálogo
- work_units contra catálogo
- Regla metric
- Cierre coherente de run_metadata si status != running

Contrato:
- No modifica system/
- Lectura desde user/runs/**
- Reporta por stdout (y opcionalmente puede escribir un json de diagnóstico en user/runs/ si se habilita)
================================================================================
"""

from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List, Optional


RUN_METADATA_SCHEMA_VERSION = "logging.run_metadata.v1"
EVENT_SCHEMA_VERSION = "logging.event.v1"
EVENT_TYPES_SCHEMA_VERSION = "logging.event_types.v1"
WORK_UNITS_SCHEMA_VERSION = "logging.work_units_catalog.v1"


@dataclass
class ValidationIssue:
    severity: str  # "ERROR" | "WARNING"
    message: str
    path: str


# ---------------------------------------------------------
# Utilidades
# ---------------------------------------------------------

def _is_iso8601_like(s: Any) -> bool:
    return isinstance(s, str) and "T" in s and len(s) >= 19


def _require_field(obj: Dict[str, Any], field: str, issues: List[ValidationIssue], path: str) -> None:
    if field not in obj:
        issues.append(ValidationIssue("ERROR", f"Falta campo obligatorio: {field}", path))


def _require_int_ge0(value: Any, field: str, issues: List[ValidationIssue], path: str) -> None:
    if not isinstance(value, int) or value < 0:
        issues.append(ValidationIssue("ERROR", f"{field} debe ser int >= 0", path))


# ---------------------------------------------------------
# Carga de catálogos
# ---------------------------------------------------------

def _load_catalog(path: Path) -> Dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def _validate_catalog_versions(
    event_catalog: Dict[str, Any],
    work_units_catalog: Dict[str, Any],
    issues: List[ValidationIssue],
    path: str
) -> None:
    if event_catalog.get("schema_version") != EVENT_TYPES_SCHEMA_VERSION:
        issues.append(ValidationIssue("ERROR", f"event_catalog schema_version incorrecta (esperada {EVENT_TYPES_SCHEMA_VERSION})", path))
    if work_units_catalog.get("schema_version") != WORK_UNITS_SCHEMA_VERSION:
        issues.append(ValidationIssue("ERROR", f"work_units_catalog schema_version incorrecta (esperada {WORK_UNITS_SCHEMA_VERSION})", path))


# ---------------------------------------------------------
# Validación work_units
# ---------------------------------------------------------

def _validate_work_units(
    work_units: Any,
    units_catalog: set,
    issues: List[ValidationIssue],
    path: str
) -> None:
    if not isinstance(work_units, dict):
        issues.append(ValidationIssue("ERROR", "work_units debe ser objeto JSON", path))
        return

    primary = work_units.get("primary")
    if not isinstance(primary, dict):
        issues.append(ValidationIssue("ERROR", "work_units.primary obligatorio", path))
        return

    unit = primary.get("unit")
    count = primary.get("count")

    if unit not in units_catalog:
        issues.append(ValidationIssue("ERROR", f"Unidad no permitida: {unit}", path))

    _require_int_ge0(count, "work_units.primary.count", issues, path)

    # Secondary
    secondary = work_units.get("secondary", [])
    if not isinstance(secondary, list):
        issues.append(ValidationIssue("ERROR", "work_units.secondary debe ser lista", path))
        return

    seen_units = set()
    for idx, item in enumerate(secondary):
        if not isinstance(item, dict):
            issues.append(ValidationIssue("ERROR", f"work_units.secondary[{idx}] inválido (debe ser objeto)", path))
            continue

        su = item.get("unit")
        sc = item.get("count")

        if su not in units_catalog:
            issues.append(ValidationIssue("ERROR", f"Unidad secundaria no permitida: {su}", path))

        if su in seen_units:
            issues.append(ValidationIssue("ERROR", f"Unidad repetida en secondary: {su}", path))
        seen_units.add(su)

        _require_int_ge0(sc, f"work_units.secondary[{idx}].count", issues, path)


# ---------------------------------------------------------
# Validación principal
# ---------------------------------------------------------

def validate_run_dir(
    run_dir: Path,
    event_catalog: Dict[str, Any],
    work_units_catalog: Dict[str, Any]
) -> List[ValidationIssue]:
    issues: List[ValidationIssue] = []

    meta_path = run_dir / "run_metadata.json"
    events_path = run_dir / "execution_events.jsonl"

    if not meta_path.exists():
        issues.append(ValidationIssue("ERROR", "No existe run_metadata.json", str(meta_path)))
        return issues
    if not events_path.exists():
        issues.append(ValidationIssue("ERROR", "No existe execution_events.jsonl", str(events_path)))
        return issues

    # Validar versiones de catálogos
    _validate_catalog_versions(event_catalog, work_units_catalog, issues, str(run_dir))

    # Cargar catálogos
    levels = set(event_catalog.get("levels", []))
    stages = set(event_catalog.get("stages", []))
    event_types = set(event_catalog.get("event_types", []))
    allowed_units = set(work_units_catalog.get("units", []))

    # -------------------------
    # Validar metadata
    # -------------------------
    try:
        meta = json.loads(meta_path.read_text(encoding="utf-8"))
    except Exception as e:
        issues.append(ValidationIssue("ERROR", f"run_metadata.json no es JSON válido: {e}", str(meta_path)))
        return issues

    if meta.get("schema_version") != RUN_METADATA_SCHEMA_VERSION:
        issues.append(ValidationIssue("ERROR", f"schema_version metadata incorrecto (esperado {RUN_METADATA_SCHEMA_VERSION})", str(meta_path)))

    for f in ["run_id", "tool_id", "started_at", "status", "inputs", "work_units"]:
        _require_field(meta, f, issues, str(meta_path))

    if "started_at" in meta and not _is_iso8601_like(meta["started_at"]):
        issues.append(ValidationIssue("WARNING", "started_at no parece ISO8601", str(meta_path)))

    if "work_units" in meta:
        _validate_work_units(meta["work_units"], allowed_units, issues, str(meta_path))

    status = meta.get("status")
    if status not in ("running", "completed", "failed", "aborted"):
        issues.append(ValidationIssue("ERROR", f"status inválido: {status}", str(meta_path)))

    if status in ("completed", "failed", "aborted"):
        for f in ["ended_at", "duration_ms", "exit_code"]:
            _require_field(meta, f, issues, str(meta_path))
        if "ended_at" in meta and not _is_iso8601_like(meta["ended_at"]):
            issues.append(ValidationIssue("WARNING", "ended_at no parece ISO8601", str(meta_path)))
        if "duration_ms" in meta:
            _require_int_ge0(meta["duration_ms"], "run_metadata.duration_ms", issues, str(meta_path))
        if "exit_code" in meta and not isinstance(meta["exit_code"], int):
            issues.append(ValidationIssue("ERROR", "exit_code debe ser int", str(meta_path)))

    # -------------------------
    # Validar eventos JSONL
    # -------------------------
    try:
        lines = events_path.read_text(encoding="utf-8").splitlines()
    except Exception as e:
        issues.append(ValidationIssue("ERROR", f"No se pudo leer execution_events.jsonl: {e}", str(events_path)))
        return issues

    if len(lines) == 0:
        issues.append(ValidationIssue("ERROR", "execution_events.jsonl está vacío", str(events_path)))
        return issues

    found_run_start = False
    found_run_end = False
    previous_ts: Optional[str] = None

    for idx, line in enumerate(lines, start=1):
        path = f"{events_path}:{idx}"

        if not line.strip():
            issues.append(ValidationIssue("ERROR", "Línea vacía en JSONL (no permitida)", path))
            continue

        try:
            ev = json.loads(line)
        except Exception as e:
            issues.append(ValidationIssue("ERROR", f"Línea JSON inválida: {e}", path))
            continue

        if ev.get("schema_version") != EVENT_SCHEMA_VERSION:
            issues.append(ValidationIssue("ERROR", f"schema_version evento incorrecto (esperado {EVENT_SCHEMA_VERSION})", path))

        # Campos obligatorios
        for f in ["timestamp", "level", "stage", "event_type", "message"]:
            _require_field(ev, f, issues, path)

        # timestamp y orden temporal
        ts = ev.get("timestamp")
        if ts is not None and not _is_iso8601_like(ts):
            issues.append(ValidationIssue("WARNING", "timestamp no parece ISO8601", path))
        if previous_ts and isinstance(ts, str) and ts < previous_ts:
            issues.append(ValidationIssue("WARNING", "Eventos fuera de orden cronológico", path))
        if isinstance(ts, str):
            previous_ts = ts

        # level / stage / event_type contra catálogo
        if "level" in ev and ev.get("level") not in levels:
            issues.append(ValidationIssue("ERROR", f"Level no permitido: {ev.get('level')}", path))

        if "stage" in ev and ev.get("stage") not in stages:
            issues.append(ValidationIssue("ERROR", f"Stage no permitido: {ev.get('stage')}", path))

        et = ev.get("event_type")
        if et is not None and et not in event_types:
            issues.append(ValidationIssue("ERROR", f"Event_type no permitido: {et}", path))

        # Metric rule
        if et == "metric":
            data = ev.get("data", {})
            if not isinstance(data, dict):
                issues.append(ValidationIssue("ERROR", "Metric requiere data como objeto", path))
            else:
                if "metric_name" not in data or "metric_value" not in data:
                    issues.append(ValidationIssue("ERROR", "Metric requiere metric_name y metric_value", path))

        # stage_end y run_end
        if et in ("stage_end", "run_end"):
            if "duration_ms" not in ev:
                issues.append(ValidationIssue("ERROR", f"{et} requiere duration_ms", path))
            else:
                _require_int_ge0(ev.get("duration_ms"), "duration_ms", issues, path)

            if "work_units" not in ev:
                issues.append(ValidationIssue("ERROR", f"{et} requiere work_units", path))
            else:
                _validate_work_units(ev.get("work_units"), allowed_units, issues, path)

        if et == "run_start":
            found_run_start = True
        if et == "run_end":
            found_run_end = True

    if not found_run_start:
        issues.append(ValidationIssue("ERROR", "No existe run_start", str(events_path)))

    if status in ("completed", "failed", "aborted") and not found_run_end:
        issues.append(ValidationIssue("ERROR", "Run cerrado sin run_end", str(events_path)))

    return issues


def validate_tree(app_root: Path, event_catalog_path: Path, work_units_catalog_path: Path, tool_id: Optional[str] = None) -> List[ValidationIssue]:
    issues: List[ValidationIssue] = []

    event_catalog = _load_catalog(event_catalog_path)
    work_units_catalog = _load_catalog(work_units_catalog_path)

    runs_root = app_root / "user" / "runs"
    if not runs_root.exists():
        issues.append(ValidationIssue("ERROR", "No existe user/runs", str(runs_root)))
        return issues

    if tool_id:
        tool_dir = runs_root / tool_id
        if not tool_dir.exists():
            issues.append(ValidationIssue("ERROR", f"No existe carpeta para tool_id={tool_id}", str(tool_dir)))
            return issues
        for run_dir in sorted([p for p in tool_dir.iterdir() if p.is_dir()]):
            issues.extend(validate_run_dir(run_dir, event_catalog, work_units_catalog))
        return issues

    for tool_dir in sorted([p for p in runs_root.iterdir() if p.is_dir()]):
        for run_dir in sorted([p for p in tool_dir.iterdir() if p.is_dir()]):
            issues.extend(validate_run_dir(run_dir, event_catalog, work_units_catalog))

    return issues


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser(description="Validador de logs (Logging v1)")
    parser.add_argument("app_root", help="Ruta a la raíz de la app portable")
    parser.add_argument("--tool_id", default=None, help="Valida solo runs de un tool_id")
    parser.add_argument("--event_catalog", required=True, help="Ruta a logging_event_types_v1.json")
    parser.add_argument("--work_units_catalog", required=True, help="Ruta a work_units_catalog_v1.json")
    args = parser.parse_args()

    app_root = Path(args.app_root)
    event_catalog_path = Path(args.event_catalog)
    work_units_catalog_path = Path(args.work_units_catalog)

    issues = validate_tree(app_root, event_catalog_path, work_units_catalog_path, tool_id=args.tool_id)

    if not issues:
        print("OK: No se han encontrado problemas.")
        return 0

    err_count = sum(1 for x in issues if x.severity == "ERROR")
    warn_count = sum(1 for x in issues if x.severity == "WARNING")

    for it in issues:
        print(f"[{it.severity}] {it.path} :: {it.message}")

    print(f"\nResumen: errores={err_count} warnings={warn_count}")
    return 2 if err_count > 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())