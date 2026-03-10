"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "app_cli.py",
  "version": "1.0.0",
  "type": "entrypoint",
  "category": "system",
  "description": "CLI principal de la aplicación portable.",
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

import argparse
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any


# -----------------------------
# Paths / registry
# -----------------------------


def get_app_root() -> Path:
    """
    app_cli.py vive en:
      system/bin/entrypoints/app_cli.py
    Por tanto:
      app_root = parents[3]
    """
    return Path(__file__).resolve().parents[3]


def registry_path(app_root: Path) -> Path:
    return app_root / "system" / "config" / "runtime_registry.json"


def load_registry(app_root: Path) -> dict:
    rp = registry_path(app_root)
    if not rp.exists():
        raise FileNotFoundError(f"No existe runtime_registry.json en: {rp}")
    return json.loads(rp.read_text(encoding="utf-8-sig"))  # BOM-safe


def validate_registry(reg: dict) -> list[str]:
    errors: list[str] = []
    if not isinstance(reg, dict):
        return ["Registry no es un objeto JSON."]

    if "runtimes" not in reg or not isinstance(reg.get("runtimes"), dict):
        errors.append("Falta 'runtimes' o no es un objeto.")
    if "tools" not in reg or not isinstance(reg.get("tools"), dict):
        errors.append("Falta 'tools' o no es un objeto.")

    runtimes = reg.get("runtimes", {}) or {}
    tools = reg.get("tools", {}) or {}

    for tool_id, tool in tools.items():
        if not isinstance(tool, dict):
            errors.append(f"Tool '{tool_id}' no es un objeto.")
            continue
        rr = tool.get("runtime_ref")
        if not rr:
            errors.append(f"Tool '{tool_id}' no define runtime_ref.")
            continue
        if rr not in runtimes:
            errors.append(f"Tool '{tool_id}' referencia runtime_ref inexistente: {rr}")

    py = runtimes.get("python")
    if py is None:
        errors.append("Falta runtime 'python' (recomendado).")
    elif isinstance(py, dict):
        if not py.get("console_executable"):
            errors.append("Runtime 'python' no define console_executable.")
    else:
        errors.append("Runtime 'python' no es un objeto.")

    return errors


# -----------------------------
# Execution helpers (BLINDAJE)
# -----------------------------


def _looks_like_rel_path(v: str) -> bool:
    """Heurística conservadora para detectar rutas relativas del árbol portable."""
    if not v:
        return False
    v_norm = v.replace("\\", "/")

    # Absolutas (POSIX/UNC) o con letra de unidad (Windows)
    if v_norm.startswith(("/", "\\")):
        return False
    if len(v_norm) >= 2 and v_norm[1] == ":":
        return False

    return (
        v_norm.startswith("user/")
        or v_norm.startswith("system/")
        or v_norm.startswith("dev/")
        or ("/" in v_norm)
    )


def _resolve_env_value(app_root: Path, value: str) -> str:
    """Convierte valores que parecen rutas relativas a rutas absolutas (desde app_root)."""
    if _looks_like_rel_path(value):
        return str((app_root / value).resolve())
    return value


def _resolve_path(app_root: Path, p: str) -> Path:
    """Resuelve un path que puede venir absoluto o relativo al app_root."""
    pp = Path(p)
    if pp.is_absolute():
        return pp
    return (app_root / p).resolve()


def build_env(app_root: Path, runtime: dict) -> dict:
    env = os.environ.copy()
    for k, v in (runtime.get("env", {}) or {}).items():
        key = str(k)
        val = "" if v is None else str(v)
        env[key] = _resolve_env_value(app_root, val)
    return env


def run_tool(app_root: Path, reg: dict, tool_id: str, pass_args: list[str]) -> int:
    tool = (reg.get("tools", {}) or {}).get(tool_id)
    if not tool:
        print(f"Tool no encontrada: {tool_id}")
        return 2

    runtime_ref = str(tool.get("runtime_ref") or "")
    runtime = (reg.get("runtimes", {}) or {}).get(runtime_ref)
    if not runtime:
        print("Runtime no encontrado para la tool.")
        print(f"  tool_id:      {tool_id}")
        print(f"  runtime_ref:  {runtime_ref!r}")
        print(f"  runtimes:     {sorted((reg.get('runtimes', {}) or {}).keys())}")
        return 2

    console_exe = str(runtime.get("console_executable") or "")
    entry_rel = str(tool.get("entry") or "")

    py_exe = _resolve_path(app_root, console_exe)
    entry = _resolve_path(app_root, entry_rel)

    if not py_exe.exists():
        print("No existe el ejecutable del runtime declarado:")
        print(f"  runtime_ref: {runtime_ref}")
        print(f"  exe:         {console_exe}")
        print(f"  resolved:    {py_exe}")
        return 1

    if not entry.exists():
        print("No existe el script del tool declarado:")
        print(f"  tool_id:     {tool_id}")
        print(f"  entry:       {entry_rel}")
        print(f"  resolved:    {entry}")
        return 1

    env = build_env(app_root, runtime)
    return subprocess.call(
        [str(py_exe), str(entry), *pass_args],
        env=env,
        cwd=str(app_root),
    )


# -----------------------------
# UI helpers (compact shell)
# -----------------------------


def _terminal_columns(default: int = 80) -> int:
    try:
        return shutil.get_terminal_size((default, 20)).columns
    except Exception:
        return default


def _fit_banner_text(text: str, width: int) -> str:
    if len(text) <= width:
        return text.ljust(width)
    if width <= 3:
        return text[:width]
    return (text[: width - 3] + "...").ljust(width)


def banner(title: str, subtitle: str = "", *, min_inner: int = 44, max_inner: int = 140) -> str:
    cols = _terminal_columns()
    inner_cap = max(min_inner, min(max_inner, cols - 4))

    title_norm = " ".join((title or "").split())
    desired = max(min_inner, len(title_norm))
    inner = min(desired, inner_cap)

    top = "╔" + ("═" * (inner + 2)) + "╗"
    mid = f"║ {_fit_banner_text(title_norm, inner)} ║"
    bot = "╚" + ("═" * (inner + 2)) + "╝"
    return "\n".join([top, mid, bot]) + ("\n" + subtitle if subtitle else "")


def tool_display_name(tool_id: str, tool: dict) -> str:
    return str(tool.get("label") or tool_id)


def preset_list(tool: dict) -> list[dict[str, Any]]:
    presets = tool.get("presets")
    out: list[dict[str, Any]] = []
    if isinstance(presets, list) and presets:
        for p in presets:
            if not isinstance(p, dict):
                continue
            label = str(p.get("label") or "").strip()
            args = p.get("args")
            if not isinstance(args, list):
                args = []
            out.append({"label": label, "args": [str(a) for a in args]})
    return out


def _console_title_from_registry(app_root: Path, reg: dict) -> str:
    app = reg.get("app", {}) or {}
    name = str(app.get("name") or app_root.name)
    customer = str(app.get("customer") or "").strip()
    ver = str(app.get("version") or "").strip()

    left = name
    if customer:
        left += f" — {customer}"
    if ver:
        left += f" ({ver})"
    return f"{left} — CONSOLA DE TOOLS"


def build_compact_menu(reg: dict) -> tuple[list[dict], list[str]]:
    tools = reg.get("tools", {}) or {}

    cats: dict[str, list[tuple[str, dict]]] = {}
    for tool_id, tool in tools.items():
        cat = str((tool or {}).get("category") or "tools").strip().upper()
        cats.setdefault(cat, []).append((tool_id, tool))

    cat_names = sorted(cats.keys())

    items: list[dict] = []
    lines: list[str] = []

    tool_index = 1
    for cat in cat_names:
        lines.append(f"[{cat}]")

        for tool_id, tool in sorted(cats[cat], key=lambda x: x[0]):
            tool = tool or {}
            presets = preset_list(tool)
            if presets:
                for pi, p in enumerate(presets, start=1):
                    key = f"{tool_index}.{pi}"
                    mode = str(p.get("label") or tool.get("menu_mode") or "Ejecutar (default)")
                    items.append({"key": key, "tool_id": tool_id, "args": p.get("args") or []})
                    lines.append(f"{key}  {tool_display_name(tool_id, tool)} — {mode}")
            else:
                key = f"{tool_index}.1"
                mode = str(tool.get("menu_mode") or "Ejecutar (default)")
                items.append({"key": key, "tool_id": tool_id, "args": []})
                lines.append(f"{key}  {tool_display_name(tool_id, tool)} — {mode}")

            tool_index += 1

        lines.append("")

    while lines and lines[-1] == "":
        lines.pop()

    return items, lines


# -----------------------------
# Shell mode
# -----------------------------


def clear_screen() -> None:
    os.system("cls" if os.name == "nt" else "clear")


def shell_mode(app_root: Path) -> int:
    while True:
        reg = load_registry(app_root)
        items, lines = build_compact_menu(reg)

        clear_screen()
        total_tools = len(reg.get("tools", {}))
        title = _console_title_from_registry(app_root, reg)

        # sin \n extra (evita línea fantasma bajo el banner)
        print(banner(title, f"Tools registradas: {total_tools}", min_inner=44, max_inner=140))
        print()  # ← línea en blanco para dar aire
        print("\n".join(lines))
        print("──────────────────────────────────────────────")
        choice = input("Selecciona (X.Y) | r refrescar | h ayuda | q salir: ").strip().lower()

        if choice in {"q", "quit", "exit"}:
            return 0
        if choice == "r":
            continue
        if choice == "h":
            clear_screen()
            print(banner("AYUDA", "", min_inner=44, max_inner=140))
            print("Comandos:")
            print("  X.Y    Ejecuta una tool/preset (ej: 3.1)")
            print("  r      Refrescar (releer registry)")
            print("  h      Ayuda")
            print("  q      Salir\n")
            input("Enter para volver…")
            continue

        match = next((it for it in items if it["key"] == choice), None)
        if not match:
            print("Opción inválida.")
            input("Enter para continuar…")
            continue

        tool_id = match["tool_id"]
        tool_args = match.get("args") or []
        code = run_tool(app_root, reg, tool_id, tool_args)

        print(f"\n(Exit code: {code})")
        input("Enter para volver al menú…")


# -----------------------------
# CLI args
# -----------------------------


def parse_args(argv: list[str]) -> argparse.Namespace:
    p = argparse.ArgumentParser(prog="app_cli.py", add_help=True)
    sub = p.add_subparsers(dest="cmd")

    sub.add_parser("shell", help="Modo menú interactivo (default)")

    run = sub.add_parser("run", help="Ejecutar una tool por ID")
    run.add_argument("tool_id", help="ID de la tool (p.ej. tool_healthcheck_runtime)")
    run.add_argument("tool_args", nargs=argparse.REMAINDER, help="Args para la tool")

    return p.parse_args(argv)


def main(argv: list[str]) -> int:
    app_root = get_app_root()

    try:
        reg = load_registry(app_root)
    except Exception as e:
        print(f"ERROR cargando registry: {e}")
        return 2

    errs = validate_registry(reg)
    if errs:
        print("Registry inválido:")
        for e in errs:
            print(f"  - {e}")
        return 2

    args = parse_args(argv)

    if not args.cmd or args.cmd == "shell":
        return shell_mode(app_root)

    if args.cmd == "run":
        tool_id = str(args.tool_id)
        tool_args = [a for a in (args.tool_args or []) if a is not None]
        return run_tool(app_root, reg, tool_id, tool_args)

    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))