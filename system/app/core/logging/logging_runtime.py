# logging_runtime.py
"""
Logging runtime (Logging v1).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Optional, List
import json
import traceback

from .run_context import RunContext, MonotonicTimer, now_iso_ms


RUN_METADATA_SCHEMA_VERSION = "logging.run_metadata.v1"
EVENT_SCHEMA_VERSION = "logging.event.v1"

EVENT_TYPE_WARNING = "warning_raised"
EVENT_TYPE_EXCEPTION = "exception_raised"
EVENT_TYPE_METRIC = "metric"
EVENT_TYPE_ARTIFACT_WRITTEN = "artifact_written"


@dataclass(frozen=True)
class WorkUnit:
    unit: str
    count: int


@dataclass(frozen=True)
class WorkUnits:
    primary: WorkUnit
    secondary: Optional[List[WorkUnit]] = None

    def to_dict(self) -> Dict[str, Any]:
        d: Dict[str, Any] = {
            "primary": {"unit": self.primary.unit, "count": int(self.primary.count)}
        }
        if self.secondary:
            d["secondary"] = [
                {"unit": wu.unit, "count": int(wu.count)} for wu in self.secondary
            ]
        return d


def _require_non_negative_int(name: str, value: int) -> None:
    if not isinstance(value, int) or value < 0:
        raise ValueError(f"{name} debe ser entero >= 0.")


def _validate_work_units(work_units: WorkUnits) -> None:
    if not isinstance(work_units, WorkUnits):
        raise ValueError("work_units debe ser WorkUnits.")
    if not work_units.primary:
        raise ValueError("work_units.primary es obligatorio.")
    _require_non_negative_int("work_units.primary.count", int(work_units.primary.count))


class RuntimeLogger:
    def __init__(
        self,
        ctx: RunContext,
        *,
        tool_version: Optional[str] = None,
        runtime_version: Optional[str] = None,
        project_id: Optional[str] = None,
        config_snapshot: Optional[Dict[str, Any]] = None,
        environment: Optional[Dict[str, Any]] = None,
        summary: Optional[str] = None,
        inputs: Optional[List[Dict[str, Any]]] = None,
        work_units: Optional[WorkUnits] = None,
    ) -> None:
        self.ctx = ctx
        self._run_timer = MonotonicTimer()
        self._is_closed = False

        self._warnings_count = 0
        self._errors_count = 0

        # Inputs mutables (se completan durante el run)
        self._inputs: List[Dict[str, Any]] = inputs if inputs is not None else []

        if work_units is None:
            work_units = WorkUnits(primary=WorkUnit(unit="files", count=0))
        _validate_work_units(work_units)
        self._work_units = work_units

        meta: Dict[str, Any] = {
            "schema_version": RUN_METADATA_SCHEMA_VERSION,
            "run_id": ctx.run_id,
            "tool_id": ctx.tool_id,
            "started_at": ctx.started_at_iso,
            "status": "running",
            "inputs": self._inputs,
            "work_units": work_units.to_dict(),
        }
        if tool_version is not None:
            meta["tool_version"] = tool_version
        if runtime_version is not None:
            meta["runtime_version"] = runtime_version
        if project_id is not None:
            meta["project_id"] = project_id
        if config_snapshot is not None:
            meta["config_snapshot"] = config_snapshot
        if environment is not None:
            meta["environment"] = environment
        if summary is not None:
            meta["summary"] = summary

        self._write_json(self.ctx.run_metadata_path, meta)

        self.event(
            level="INFO",
            stage="run",
            event_type="run_start",
            message="Execution started",
        )

    # -----------------------------
    # Helpers públicos
    # -----------------------------
    def set_inputs(self, inputs: List[Dict[str, Any]]) -> None:
        if self._is_closed:
            return
        self._inputs = inputs

    def warning(self, stage: str, message: str, *, warning_type: str, data: Optional[Dict[str, Any]] = None) -> None:
        payload = {"warning_type": warning_type}
        if data:
            payload.update(data)
        self.event(
            level="WARNING",
            stage=stage,
            event_type=EVENT_TYPE_WARNING,
            message=message,
            data=payload,
        )

    def exception(
        self,
        stage: str,
        message: str,
        exc: BaseException,
        *,
        exception_type: str,
        data: Optional[Dict[str, Any]] = None,
    ) -> None:
        tb = traceback.format_exc()
        payload = {
            "exception_type": exception_type,
            "exception_class": exc.__class__.__name__,
            "exception_message": str(exc),
            "traceback": tb,
        }
        if data:
            payload.update(data)
        self.event(
            level="ERROR",
            stage=stage,
            event_type=EVENT_TYPE_EXCEPTION,
            message=message,
            data=payload,
        )

    def metric(
        self,
        stage: str,
        metric_name: str,
        metric_value: Any,
        *,
        unit: Optional[str] = None,
        message: str = "Metric recorded",
        data: Optional[Dict[str, Any]] = None,
    ) -> None:
        payload: Dict[str, Any] = {
            "metric_name": metric_name,
            "metric_value": metric_value,
        }
        if unit is not None:
            payload["metric_unit"] = unit
        if data:
            payload.update(data)
        self.event(
            level="INFO",
            stage=stage,
            event_type=EVENT_TYPE_METRIC,
            message=message,
            data=payload,
        )

    def artifact_written(self, stage: str, artifact_path: str, *, data: Optional[Dict[str, Any]] = None) -> None:
        payload = {"artifact_path": artifact_path}
        if data:
            payload.update(data)
        self.event(
            level="INFO",
            stage=stage,
            event_type=EVENT_TYPE_ARTIFACT_WRITTEN,
            message="Artifact written",
            data=payload,
        )

    # -----------------------------
    # IO interno
    # -----------------------------
    def _write_json(self, path: Path, obj: Dict[str, Any]) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(obj, ensure_ascii=False, indent=2), encoding="utf-8")

    def _append_jsonl(self, path: Path, obj: Dict[str, Any]) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open("a", encoding="utf-8") as f:
            f.write(json.dumps(obj, ensure_ascii=False) + "\n")

    # -----------------------------
    # Evento base + stages
    # -----------------------------
    def event(
        self,
        *,
        level: str,
        stage: str,
        event_type: str,
        message: str,
        data: Optional[Dict[str, Any]] = None,
        duration_ms: Optional[int] = None,
        work_units: Optional[WorkUnits] = None,
    ) -> None:
        if self._is_closed:
            return

        ev: Dict[str, Any] = {
            "schema_version": EVENT_SCHEMA_VERSION,
            "timestamp": now_iso_ms(),
            "level": level,
            "stage": stage,
            "event_type": event_type,
            "message": message,
        }
        if data:
            ev["data"] = data
        if duration_ms is not None:
            ev["duration_ms"] = int(duration_ms)
        if work_units is not None:
            ev["work_units"] = work_units.to_dict()

        if level == "WARNING":
            self._warnings_count += 1
        if level in ("ERROR", "CRITICAL"):
            self._errors_count += 1

        self._append_jsonl(self.ctx.execution_events_path, ev)

    def stage_start(self, stage: str, message: str) -> MonotonicTimer:
        self.event(level="INFO", stage=stage, event_type="stage_start", message=message)
        return MonotonicTimer()

    def stage_end(self, stage: str, message: str, timer: MonotonicTimer, *, work_units: WorkUnits, data: Optional[Dict[str, Any]] = None):
        duration_ms = timer.elapsed_ms()
        self.event(
            level="INFO",
            stage=stage,
            event_type="stage_end",
            message=message,
            duration_ms=duration_ms,
            work_units=work_units,
            data=data,
        )

    # -----------------------------
    # Cierre del run
    # -----------------------------
    def close(
        self,
        *,
        status: str,
        exit_code: int,
        work_units: Optional[WorkUnits] = None,
        summary: Optional[str] = None,
        data: Optional[Dict[str, Any]] = None,
    ) -> None:
        if self._is_closed:
            return

        ended_at = now_iso_ms()
        duration_ms = self._run_timer.elapsed_ms()

        if work_units is None:
            work_units = self._work_units

        # run_end
        run_end_data = data or {}
        run_end_data.setdefault("status", status)
        run_end_data.setdefault("exit_code", int(exit_code))
        run_end_data.setdefault("warnings_count", int(self._warnings_count))
        run_end_data.setdefault("errors_count", int(self._errors_count))

        self.event(
            level="INFO" if status == "completed" else "ERROR",
            stage="run",
            event_type="run_end",
            message="Execution finished",
            duration_ms=duration_ms,
            work_units=work_units,
            data=run_end_data,
        )

        meta = json.loads(self.ctx.run_metadata_path.read_text(encoding="utf-8"))
        meta["ended_at"] = ended_at
        meta["duration_ms"] = int(duration_ms)
        meta["status"] = status
        meta["exit_code"] = int(exit_code)
        meta["warnings_count"] = int(self._warnings_count)
        meta["errors_count"] = int(self._errors_count)
        meta["work_units"] = work_units.to_dict()

        # 🔥 inputs finales
        meta["inputs"] = self._inputs

        if summary:
            meta["summary"] = summary

        self._write_json(self.ctx.run_metadata_path, meta)
        self._is_closed = True


def init_logger(ctx: RunContext, **kwargs) -> RuntimeLogger:
    return RuntimeLogger(ctx, **kwargs)