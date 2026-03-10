#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_telemetry_product_audit.py",
  "tool_id": "tool_telemetry_product_audit",
  "version": "1.1.1",
  "type": "product_tool",
  "category": "product",
  "description": "Ejecuta la auditoría de telemetría del producto delegando en la capa wiring y emite un resultado JSON trazable.",
  "location_expected": "system/bin/tools/product",
  "entry_point": "main",
  "inputs": [
    {
      "type": "optional_spec",
      "location": "system/spec/telemetry/*.json",
      "via_argument": "--spec_path",
      "required": false
    }
  ],
  "outputs": [
    {
      "type": "run_logs",
      "location": "user/runs/telemetry/product/<timestamp>/"
    },
    {
      "type": "file",
      "location": "user/runs/telemetry/product/<timestamp>/report.json",
      "role": "canonical_run_artifact"
    },
    {
      "type": "file",
      "location": "user/runs/telemetry/product/latest/report.json",
      "role": "mutable_alias",
      "optional": true
    }
  ],
  "invariants_relevant": [
    "INV-001 separation system/user/dev",
    "INV-004 ui_cli_without_domain_logic",
    "INV-005 self_contained_workspace",
    "INV-006 traceability_required",
    "INV-007 no_document_contradiction",
    "INV-009 structural_determinism"
  ],
  "modifies_system": false,
  "runtime_ref": "python",
  "runtime": "python_portable",
  "notes": [
    "This wrapper validates arguments and delegates domain execution to system.tools.telemetry.product_telemetry_wiring.",
    "Run creation and canonical telemetry artifact emission are expected to be handled by the delegated wiring layer."
  ]
}
================================================================================
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any, Dict, Optional

TOOL_ID = "telemetry_product_audit"
TOOL_VERSION = "1.1.0"
ALLOWED_SPEC_BASE = Path("system/spec/telemetry")
ALLOWED_SPEC_SUFFIXES = {".json"}


def resolve_app_root() -> Path:
    return Path(__file__).resolve().parents[3]


def to_rel_posix(root: Path, path: Path) -> str:
    try:
        return str(path.relative_to(root)).replace("\\", "/")
    except Exception:
        return str(path).replace("\\", "/")


def emit_json(payload: Dict[str, Any], pretty: bool = False) -> None:
    print(json.dumps(payload, indent=2 if pretty else None, ensure_ascii=False))


def error_payload(
    *,
    app_root: Path,
    error_type: str,
    message: str,
    hint: Optional[str] = None,
    spec_path_arg: Optional[str] = None,
    spec_path_resolved: Optional[Path] = None,
    details: Optional[Dict[str, Any]] = None,
) -> Dict[str, Any]:
    payload: Dict[str, Any] = {
        "status": "ERROR",
        "tool_id": TOOL_ID,
        "tool_version": TOOL_VERSION,
        "error_type": error_type,
        "message": message,
        "app_root": to_rel_posix(app_root, app_root),
    }

    if hint:
        payload["hint"] = hint

    if spec_path_arg is not None:
        payload["spec_path_arg"] = spec_path_arg

    if spec_path_resolved is not None:
        payload["spec_path_resolved"] = to_rel_posix(app_root, spec_path_resolved)

    if details:
        payload["details"] = details

    return payload


def validate_runtime_structure(app_root: Path) -> None:
    required_paths = [
        app_root / "system",
        app_root / "user",
        app_root / "system" / "tools" / "telemetry" / "product_telemetry_wiring.py",
    ]
    missing = [p for p in required_paths if not p.exists()]
    if missing:
        missing_rel = [to_rel_posix(app_root, p) for p in missing]
        raise FileNotFoundError(
            f"Required runtime paths are missing: {', '.join(missing_rel)}"
        )


def resolve_and_validate_spec_path(app_root: Path, spec_path_arg: Optional[str]) -> Optional[Path]:
    if not spec_path_arg:
        return None

    candidate = Path(spec_path_arg)

    if candidate.is_absolute():
        raise ValueError("Absolute paths are not allowed for --spec_path.")

    resolved = (app_root / candidate).resolve()
    allowed_base = (app_root / ALLOWED_SPEC_BASE).resolve()

    try:
        resolved.relative_to(app_root)
    except ValueError as exc:
        raise ValueError("Resolved spec path escapes app_root.") from exc

    try:
        resolved.relative_to(allowed_base)
    except ValueError as exc:
        raise ValueError("Resolved spec path must be inside system/spec/telemetry/.") from exc

    if resolved.suffix.lower() not in ALLOWED_SPEC_SUFFIXES:
        allowed = ", ".join(sorted(ALLOWED_SPEC_SUFFIXES))
        raise ValueError(f"Spec file must use one of the allowed suffixes: {allowed}")

    if not resolved.exists():
        raise FileNotFoundError(f"Spec file does not exist: {to_rel_posix(app_root, resolved)}")

    if not resolved.is_file():
        raise ValueError("Resolved spec path is not a file.")

    return resolved


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Auditar producto (telemetría) delegando en la capa wiring."
    )
    parser.add_argument(
        "--spec_path",
        type=str,
        default=None,
        help="Ruta relativa opcional dentro de system/spec/telemetry/ hacia un spec JSON.",
    )
    parser.add_argument(
        "--pretty",
        action="store_true",
        help="Imprime la salida en JSON legible.",
    )
    return parser


def import_wiring_run(app_root: Path):
    try:
        from system.tools.telemetry.product_telemetry_wiring import run as run_telemetry
        return run_telemetry
    except ModuleNotFoundError as exc:
        payload = error_payload(
            app_root=app_root,
            error_type=type(exc).__name__,
            message=f"Failed to import telemetry wiring: {exc}",
            hint=(
                "Ejecuta este tool desde la raíz de la app o mediante la consola/launcher. "
                "Verifica también que exista system/tools/telemetry/product_telemetry_wiring.py."
            ),
            details={
                "import_target": "system.tools.telemetry.product_telemetry_wiring.run",
            },
        )
        raise RuntimeError(json.dumps(payload, ensure_ascii=False)) from exc
    except Exception as exc:
        payload = error_payload(
            app_root=app_root,
            error_type=type(exc).__name__,
            message=f"Failed to import telemetry wiring: {exc}",
            hint="Verifica que el módulo de telemetría no tenga errores internos.",
            details={
                "import_target": "system.tools.telemetry.product_telemetry_wiring.run",
            },
        )
        raise RuntimeError(json.dumps(payload, ensure_ascii=False)) from exc


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()

    app_root = resolve_app_root()

    try:
        validate_runtime_structure(app_root)
    except Exception as exc:
        emit_json(
            error_payload(
                app_root=app_root,
                error_type=type(exc).__name__,
                message=str(exc),
                hint=(
                    "La estructura runtime no es válida o faltan rutas requeridas. "
                    "Revisa la integridad del bundle portable."
                ),
                spec_path_arg=args.spec_path,
            ),
            pretty=True,
        )
        return 2

    try:
        spec_path = resolve_and_validate_spec_path(app_root, args.spec_path)
    except FileNotFoundError as exc:
        emit_json(
            error_payload(
                app_root=app_root,
                error_type="InvalidSpecPath",
                message=str(exc),
                hint="Proporciona un fichero existente dentro de system/spec/telemetry/.",
                spec_path_arg=args.spec_path,
                spec_path_resolved=(app_root / args.spec_path).resolve() if args.spec_path else None,
            ),
            pretty=True,
        )
        return 3
    except ValueError as exc:
        emit_json(
            error_payload(
                app_root=app_root,
                error_type="InvalidSpecPath",
                message=str(exc),
                hint=(
                    "La ruta debe ser relativa, permanecer dentro de system/spec/telemetry/ "
                    "y apuntar a un .json."
                ),
                spec_path_arg=args.spec_path,
                spec_path_resolved=(app_root / args.spec_path).resolve() if args.spec_path else None,
            ),
            pretty=True,
        )
        return 3

    try:
        run_telemetry = import_wiring_run(app_root)
    except RuntimeError as exc:
        try:
            payload = json.loads(str(exc))
        except Exception:
            payload = error_payload(
                app_root=app_root,
                error_type=type(exc).__name__,
                message=str(exc),
                hint="Error inesperado importando la capa wiring.",
                spec_path_arg=args.spec_path,
                spec_path_resolved=spec_path,
            )
        emit_json(payload, pretty=True)
        return 4

    try:
        result = run_telemetry(app_root=app_root, spec_path=spec_path)

        if not isinstance(result, dict):
            result = {
                "status": "OK",
                "tool_id": TOOL_ID,
                "tool_version": TOOL_VERSION,
                "result": result,
            }
        else:
            result.setdefault("tool_id", TOOL_ID)
            result.setdefault("tool_version", TOOL_VERSION)

        emit_json(result, pretty=args.pretty)
        status = str(result.get("status", "OK")).lower()

        if status in {"ok", "success", "completed"}:
            return 0
        if status in {"warn", "warning"}:
            return 0
        if status in {"cancelled", "canceled"}:
            return 5
        return 4

    except Exception as exc:
        emit_json(
            error_payload(
                app_root=app_root,
                error_type=type(exc).__name__,
                message=str(exc),
                hint=(
                    "La ejecución de la auditoría falló dentro de la capa wiring o una dependencia "
                    "interna devolvió un error no controlado."
                ),
                spec_path_arg=args.spec_path,
                spec_path_resolved=spec_path,
            ),
            pretty=True,
        )
        return 99


if __name__ == "__main__":
    raise SystemExit(main())