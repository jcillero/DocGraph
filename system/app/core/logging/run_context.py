# run_context.py
"""
Run context utilities (Logging v1).

Responsabilidad:
- Generar run_id (RUN_YYYYMMDD_HHMMSS_mmm) con sufijo incremental si colisiona.
- Crear la carpeta de run en user/runs/<tool_id>/<run_id>/
- Exponer rutas a:
  - run_metadata.json
  - execution_events.jsonl
- Proveer timestamps ISO8601 con milisegundos.

No hace:
- Logging de eventos (eso es responsabilidad de logging_runtime.py)
- Lógica de dominio de ninguna tool
"""

from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
import re
import time


@dataclass(frozen=True)
class RunContext:
    tool_id: str
    run_id: str
    run_dir: Path
    run_metadata_path: Path
    execution_events_path: Path
    started_at_iso: str


_TOOL_ID_SAFE_RE = re.compile(r"[^a-zA-Z0-9_]+")


def sanitize_tool_id(tool_id: str) -> str:
    """
    Asegura que tool_id sea seguro como nombre de carpeta.
    Regla: solo A-Z a-z 0-9 y guion bajo.
    """
    if not isinstance(tool_id, str) or not tool_id.strip():
        raise ValueError("tool_id debe ser string no vacío.")

    tool_id = tool_id.strip()
    tool_id = tool_id.replace("-", "_")  # opcional: homogeneiza
    tool_id = _TOOL_ID_SAFE_RE.sub("_", tool_id)
    tool_id = re.sub(r"_+", "_", tool_id).strip("_")

    if not tool_id:
        raise ValueError("tool_id quedó vacío tras sanitización.")

    return tool_id


def now_iso_ms(dt: datetime | None = None) -> str:
    """
    ISO8601 naive con milisegundos.
    Ej: 2026-02-28T23:15:02.417
    """
    if dt is None:
        dt = datetime.now()
    return dt.strftime("%Y-%m-%dT%H:%M:%S.") + f"{dt.microsecond // 1000:03d}"


def build_run_id(dt: datetime | None = None) -> str:
    """
    Formato: RUN_YYYYMMDD_HHMMSS_mmm
    """
    if dt is None:
        dt = datetime.now()
    mmm = dt.microsecond // 1000
    return f"RUN_{dt:%Y%m%d}_{dt:%H%M%S}_{mmm:03d}"


def _try_mkdir_unique(base_dir: Path, run_id_base: str, max_suffix: int = 9999) -> tuple[str, Path]:
    """
    Intenta crear base_dir/<run_id> y, si existe, prueba con sufijo _01, _02...
    Devuelve (run_id_final, run_dir_final) ya creado.
    Maneja carrera entre procesos: si mkdir falla por existir, sigue probando.
    """
    # 0 = sin sufijo
    candidates = [(run_id_base, base_dir / run_id_base)]
    for i in range(1, max_suffix + 1):
        candidates.append((f"{run_id_base}_{i:02d}", base_dir / f"{run_id_base}_{i:02d}"))

    for run_id, run_dir in candidates:
        try:
            run_dir.mkdir(parents=True, exist_ok=False)
            return run_id, run_dir
        except FileExistsError:
            continue

    raise RuntimeError(f"No se pudo generar un run_id único tras {max_suffix} sufijos.")


def create_run_context(app_root: Path, tool_id: str) -> RunContext:
    """
    Crea el contexto de ejecución para una tool.

    app_root: raíz de la app portable.
    tool_id: id estable de la tool (ej: tool_pdf_ocr_extract)

    Estructura:
      user/runs/<tool_id>/<run_id>/
        run_metadata.json
        execution_events.jsonl
    """
    tool_id_safe = sanitize_tool_id(tool_id)

    runs_root = app_root / "user" / "runs" / tool_id_safe
    runs_root.mkdir(parents=True, exist_ok=True)

    dt = datetime.now()
    run_id_base = build_run_id(dt)
    run_id_final, run_dir = _try_mkdir_unique(runs_root, run_id_base)

    run_metadata_path = run_dir / "run_metadata.json"
    execution_events_path = run_dir / "execution_events.jsonl"

    started_at_iso = now_iso_ms(dt)

    return RunContext(
        tool_id=tool_id_safe,
        run_id=run_id_final,
        run_dir=run_dir,
        run_metadata_path=run_metadata_path,
        execution_events_path=execution_events_path,
        started_at_iso=started_at_iso,
    )


class MonotonicTimer:
    """
    Temporizador monotónico para medir duration_ms sin depender del reloj del sistema.
    """
    __slots__ = ("_t0",)

    def __init__(self) -> None:
        self._t0 = time.perf_counter()

    def elapsed_ms(self) -> int:
        return int((time.perf_counter() - self._t0) * 1000)