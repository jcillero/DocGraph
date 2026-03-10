"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "registry_validate.py",
  "version": "1.0.0",
  "type": "system_tool",
  "category": "system",
  "description": "Valida coherencia estructural de runtime_registry.json.",
  "location_expected": "system/bin/entrypoints/",
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

from pathlib import Path


class RegistryValidationError(Exception):
    pass


def _is_rel_posix_path(s: str) -> bool:
    # Queremos rutas tipo "system/bin/..." (no absolutas, no "..", no backslashes)
    if not isinstance(s, str) or not s.strip():
        return False
    s = s.strip()
    if ":\\" in s or ":/" in s:
        return False
    if s.startswith(("/", "\\")):
        return False
    if "\\" in s:
        return False
    parts = s.split("/")
    if any(p == ".." for p in parts):
        return False
    return True


def validate_registry(reg: dict, app_root: Path) -> None:
    errors: list[str] = []

    # Top-level contract
    for k in ("app_meta", "runtimes", "tools"):
        if k not in reg:
            errors.append(f"Falta clave top-level: {k}")

    app_meta = reg.get("app_meta") or {}
    for k in ("name", "vendor", "version"):
        if not (isinstance(app_meta.get(k), str) and app_meta.get(k).strip()):
            errors.append(f"app_meta.{k} es obligatorio (string no vacío)")

    runtimes = reg.get("runtimes") or {}
    tools = reg.get("tools") or {}
    if not isinstance(runtimes, dict) or not runtimes:
        errors.append("runtimes debe ser un dict no vacío")
    if not isinstance(tools, dict) or not tools:
        errors.append("tools debe ser un dict no vacío")

    # Runtimes
    if isinstance(runtimes, dict):
        for rname, robj in runtimes.items():
            if not isinstance(robj, dict):
                errors.append(f"runtime '{rname}' debe ser objeto")
                continue

            exe = robj.get("console_executable")
            if not (isinstance(exe, str) and exe.strip()):
                errors.append(f"runtime '{rname}': falta console_executable")
                continue

            exe = exe.strip().replace("\\", "/")
            if not _is_rel_posix_path(exe):
                errors.append(f"runtime '{rname}': console_executable debe ser ruta relativa con '/' (actual: {exe!r})")
            else:
                if not (app_root / exe).exists():
                    errors.append(f"runtime '{rname}': console_executable no existe en disco: {exe}")

            # Regla crítica: prohibido venv
            if "system/bin/runtimes/python/current/Scripts/python.exe" in exe:
                errors.append(
                    f"runtime '{rname}': PROHIBIDO usar venv (current/Scripts/python.exe). Usa current/python/python.exe"
                )

    # Tools
    if isinstance(tools, dict):
        for tid, tobj in tools.items():
            if not isinstance(tobj, dict):
                errors.append(f"tool '{tid}' debe ser objeto")
                continue

            for k in ("entry", "runtime_ref", "category"):
                if k not in tobj:
                    errors.append(f"tool '{tid}': falta campo obligatorio '{k}'")

            entry = tobj.get("entry")
            if isinstance(entry, str) and entry.strip():
                entry = entry.strip().replace("\\", "/")
                if not _is_rel_posix_path(entry):
                    errors.append(f"tool '{tid}': entry debe ser ruta relativa con '/' (actual: {entry!r})")
                else:
                    if not (app_root / entry).exists():
                        errors.append(f"tool '{tid}': entry no existe en disco: {entry}")

            rr = tobj.get("runtime_ref")
            if isinstance(rr, str) and rr.strip():
                if rr not in runtimes:
                    errors.append(f"tool '{tid}': runtime_ref '{rr}' no existe en runtimes")

    if errors:
        raise RegistryValidationError("REGISTRY_VALIDATION_FAIL\n" + "\n".join(f"- {e}" for e in errors))