#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_run_stats.py",
  "tool_id": "tool_run_stats",
  "version": "0.3.1",
  "type": "runtime_tool",
  "category": "runtime",
  "description": "Calcula estadísticas básicas a partir de user/runs/**/run_metadata.json.",
  "location_expected": "system/bin/tools/runtime",
  "entry_point": "main",
  "runtime_required": true,
  "modifies_system": false,
  "output_location": "user/runs/tool_run_stats/<run_id>/",
  "outputs": ["run_stats.json"],
  "depends_on": [
    "system/core/runtime/tool_runtime.py"
  ],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "DEV_RUNBOOK"
  ]
}
================================================================================
"""

from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from datetime import datetime, timedelta
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

from system.core.runtime.tool_runtime import (
    close_tool_runtime,
    init_tool_runtime,
    mark_artifact_written,
)
from system.core.logging.logging_runtime import WorkUnit, WorkUnits


TOOL_VERSION = "0.3.0"


# -----------------------------
# Base utils
# -----------------------------
def iso_to_dt(s: str) -> Optional[datetime]:
    try:
        return datetime.fromisoformat(s)
    except Exception:
        return None


def dt_to_iso_seconds(dt: datetime) -> str:
    return dt.isoformat(timespec="seconds")


def safe_read_json(path: Path) -> Tuple[Optional[Dict[str, Any]], Optional[str]]:
    try:
        obj = json.loads(path.read_text(encoding="utf-8"))
        if isinstance(obj, dict):
            return obj, None
        return None, "json_not_object"
    except Exception as e:
        return None, f"json_read_error: {e}"


def fmt_int(n: Optional[float]) -> str:
    if n is None:
        return "-"
    try:
        return str(int(round(n)))
    except Exception:
        return "-"


def pad(s: str, w: int) -> str:
    if len(s) >= w:
        return s[: w - 1] + "…" if w >= 2 else s[:w]
    return s + " " * (w - len(s))


# -----------------------------
# Stats structures
# -----------------------------
@dataclass
class ToolAgg:
    tool_id: str
    runs: int = 0
    completed: int = 0
    failed: int = 0
    avg_duration_ms: Optional[float] = None
    last_run_dt: Optional[datetime] = None


def update_avg(current_avg: Optional[float], current_n: int, new_value: float) -> float:
    if current_n <= 0 or current_avg is None:
        return new_value
    return (current_avg * current_n + new_value) / float(current_n + 1)


# -----------------------------
# Core logic
# -----------------------------
def collect_run_metadata_paths(app_root: Path) -> List[Path]:
    runs_root = app_root / "user" / "runs"
    if not runs_root.exists():
        return []
    return list(runs_root.rglob("run_metadata.json"))


def should_include(meta: Dict[str, Any], tool_filter: Optional[str], since_dt: Optional[datetime]) -> bool:
    if tool_filter and str(meta.get("tool_id") or "") != tool_filter:
        return False
    if since_dt:
        started = iso_to_dt(str(meta.get("started_at") or ""))
        if started is None or started < since_dt:
            return False
    return True


def build_stats(
    app_root: Path,
    tool_filter: Optional[str],
    since_days: Optional[int],
    limit_tools: Optional[int],
) -> Tuple[Dict[str, ToolAgg], Dict[str, Any]]:
    paths = collect_run_metadata_paths(app_root)

    since_dt = None
    if since_days is not None and since_days >= 0:
        since_dt = datetime.now() - timedelta(days=since_days)

    aggs: Dict[str, ToolAgg] = {}
    errors: List[Dict[str, Any]] = []

    scanned = 0
    included = 0

    for p in paths:
        scanned += 1
        meta, err = safe_read_json(p)
        if meta is None:
            errors.append({"path": str(p).replace("\\", "/"), "error": err})
            continue

        if not should_include(meta, tool_filter, since_dt):
            continue

        included += 1

        tid = str(meta.get("tool_id") or "UNKNOWN")
        agg = aggs.get(tid) or ToolAgg(tool_id=tid)
        agg.runs += 1

        status = str(meta.get("status") or "").lower()
        exit_code = meta.get("exit_code", None)

        if status == "completed" and (exit_code == 0 or exit_code is None):
            agg.completed += 1
        else:
            agg.failed += 1

        dur = meta.get("duration_ms", None)
        if isinstance(dur, int) and dur >= 0:
            agg.avg_duration_ms = update_avg(agg.avg_duration_ms, agg.runs - 1, float(dur))

        started_dt = iso_to_dt(str(meta.get("started_at") or ""))
        if started_dt is not None:
            if agg.last_run_dt is None or started_dt > agg.last_run_dt:
                agg.last_run_dt = started_dt

        aggs[tid] = agg

    sorted_items = sorted(
        aggs.items(),
        key=lambda kv: (kv[1].runs, kv[0]),
        reverse=True,
    )

    if limit_tools is not None and limit_tools > 0:
        sorted_items = sorted_items[:limit_tools]

    aggs_limited = {k: v for k, v in sorted_items}

    summary = {
        "scanned_run_metadata_files": scanned,
        "included_runs": included,
        "tools_count": len(aggs_limited),
        "errors_count": len(errors),
        "errors_sample": errors[:10],
        "since_days": since_days,
        "tool_filter": tool_filter,
        "limit_tools": limit_tools,
    }
    return aggs_limited, summary


def print_table(aggs: Dict[str, ToolAgg]) -> None:
    rows = list(aggs.values())
    w_tool = max([len(r.tool_id) for r in rows] + [7])
    w_tool = min(max(w_tool, 12), 36)

    header = (
        pad("tool_id", w_tool) + "  " +
        pad("runs", 6) + "  " +
        pad("ok", 4) + "  " +
        pad("fail", 6) + "  " +
        pad("avg_ms", 8) + "  " +
        "last_run"
    )
    print(header)
    print("-" * len(header))

    for r in rows:
        last = dt_to_iso_seconds(r.last_run_dt) if r.last_run_dt else "-"
        line = (
            pad(r.tool_id, w_tool) + "  " +
            pad(str(r.runs), 6) + "  " +
            pad(str(r.completed), 4) + "  " +
            pad(str(r.failed), 6) + "  " +
            pad(fmt_int(r.avg_duration_ms), 8) + "  " +
            last
        )
        print(line)


# -----------------------------
# Entry
# -----------------------------
def parse_args(argv: List[str]) -> argparse.Namespace:
    ap = argparse.ArgumentParser(
        prog="tool_run_stats",
        description="Estadísticas básicas a partir de user/runs/**/run_metadata.json",
    )
    ap.add_argument("--tool_id", default=None, help="Filtra por tool_id exacto (opcional).")
    ap.add_argument("--since_days", type=int, default=None, help="Solo runs desde hace N días (opcional).")
    ap.add_argument("--limit_tools", type=int, default=20, help="Máximo de tools a mostrar (default 20).")
    ap.add_argument("--json", action="store_true", help="Imprime también el JSON completo por stdout.")
    return ap.parse_args(argv)


def main(argv: Optional[List[str]] = None) -> int:
    argv = argv or []

    rt = init_tool_runtime(
        tool_id="tool_run_stats",
        tool_version=TOOL_VERSION,
        primary_work_unit_name="actions",
        primary_work_unit_count=1,
    )

    app_root = rt.app.app_root
    logger = rt.logger
    args = parse_args(argv)

    exit_code = 1
    run_status = "failed"

    try:
        t_ingest = logger.stage_start("ingest", "Scanning run_metadata.json files")

        aggs, summary = build_stats(
            app_root=app_root,
            tool_filter=args.tool_id,
            since_days=args.since_days,
            limit_tools=args.limit_tools,
        )

        ingest_ms = t_ingest.elapsed_ms()
        logger.stage_end(
            "ingest",
            "Scan completed",
            timer=t_ingest,
            work_units=WorkUnits(
                primary=WorkUnit(unit="runs", count=int(summary["included_runs"])),
                secondary=[
                    WorkUnit(unit="files", count=int(summary["scanned_run_metadata_files"])),
                    WorkUnit(unit="tools", count=int(summary["tools_count"])),
                ],
            ),
            data=summary,
        )
        logger.metric("ingest", "compute_duration_ms", int(ingest_ms), unit="ms", message="Stats scan time")

        t_proc = logger.stage_start("process", "Building report + printing table")

        report = {
            "schema_version": "tool.run_stats.v0",
            "generated_at": datetime.now().isoformat(timespec="seconds"),
            "app_root": str(app_root).replace("\\", "/"),
            "filters": {
                "tool_id": args.tool_id,
                "since_days": args.since_days,
                "limit_tools": args.limit_tools,
            },
            "summary": summary,
            "tools": [
                {
                    "tool_id": a.tool_id,
                    "runs": a.runs,
                    "completed": a.completed,
                    "failed": a.failed,
                    "avg_duration_ms": int(round(a.avg_duration_ms)) if a.avg_duration_ms is not None else None,
                    "last_run": dt_to_iso_seconds(a.last_run_dt) if a.last_run_dt else None,
                }
                for a in aggs.values()
            ],
        }

        print_table(aggs)
        if args.json:
            print("\n--- JSON ---")
            print(json.dumps(report, ensure_ascii=False, indent=2))

        out_path = rt.run_dir / "run_stats.json"
        out_path.write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8")

        proc_ms = t_proc.elapsed_ms()
        logger.stage_end(
            "process",
            "Report generated",
            timer=t_proc,
            work_units=WorkUnits(primary=WorkUnit(unit="tools", count=len(aggs))),
        )
        logger.metric("process", "compute_duration_ms", int(proc_ms), unit="ms", message="Report generation time")

        mark_artifact_written(rt, "export", out_path)

        exit_code = 0
        run_status = "completed"
        return exit_code

    except Exception as e:
        logger.exception("process", "tool_run_stats crashed", e, exception_type="tool_run_stats_error")
        exit_code = 99
        run_status = "failed"
        return exit_code

    finally:
        close_tool_runtime(
            rt,
            status=run_status,
            exit_code=exit_code,
            summary="Run stats execution",
            primary_work_unit_name="actions",
            primary_work_unit_count=1,
        )


if __name__ == "__main__":
    import sys
    raise SystemExit(main(sys.argv[1:]))
