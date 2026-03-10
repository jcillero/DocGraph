"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "dev_tool_launcher.py",
  "version": "1.0.0",
  "type": "dev_tool",
  "category": "dev_launcher",
  "description": "Launcher interactivo para ejecutar herramientas de desarrollo declaradas en dev/config/dev_tools_registry.json.",
  "location_expected": "dev/scripts/",
  "entry_point": "main",
  "inputs": [
    {
      "type": "argument",
      "name": "workspace_root",
      "description": "Ruta raíz del workspace de la aplicación portable."
    },
    {
      "type": "file",
      "name": "dev_tools_registry",
      "location": "dev/config/dev_tools_registry.json",
      "description": "Registro declarativo de herramientas de desarrollo disponibles."
    }
  ],
  "outputs": [
    {
      "type": "console",
      "description": "Menú interactivo para selección de herramientas dev."
    },
    {
      "type": "tool_execution",
      "description": "Invocación de scripts o wrappers declarados en el registry."
    }
  ],
  "side_effects": [
    "launch_dev_scripts",
    "execute_wrapper_bat"
  ],
  "modifies_system": false,
  "modifies_user": false,
  "modifies_dev": false,
  "requires_runtime": "python",
  "runtime_version": ">=3.10",
  "deterministic": true,
  "idempotent": true
}
================================================================================
"""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

def load_registry(registry_path: Path) -> list[dict[str, Any]]:
    with registry_path.open("r", encoding="utf-8") as f:
        data = json.load(f)

    tools = data.get("dev_tools", [])
    if not isinstance(tools, list):
        raise ValueError("Registry field 'dev_tools' must be a list.")

    filtered: list[dict[str, Any]] = []
    for tool in tools:
        if not isinstance(tool, dict):
            continue

        enabled = tool.get("enabled", True)
        launch = tool.get("launch", {})
        show = isinstance(launch, dict) and launch.get("show_in_dev_launcher", False)

        if enabled and show:
            filtered.append(tool)

    filtered.sort(
        key=lambda t: (
            str(t.get("category", "other")).lower(),
            str((t.get("launch") or {}).get("launch_label") or t.get("name") or t.get("tool_id", "")).lower(),
        )
    )
    return filtered


def display_menu(tools: list[dict[str, Any]]) -> None:
    print("==================================================")
    print("           DEV TOOL LAUNCHER - PYTHON")
    print("==================================================")
    print()

    current_category = None
    for idx, tool in enumerate(tools, start=1):
        category = str(tool.get("category", "other"))
        launch = tool.get("launch", {}) or {}
        label = str(launch.get("launch_label") or tool.get("name") or tool.get("tool_id") or f"tool_{idx}")
        mode = str(launch.get("mode", "direct"))

        if category != current_category:
            current_category = category
            print(f"[{category}]")

        print(f"  {idx}. {label} <{mode}>")

    print()
    print("  0. Salir")
    print()


def run_direct(root: Path, py: Path, tool: dict[str, Any]) -> int:
    script_relpath = tool.get("script_relpath")

    if not script_relpath:
        print("ERROR: tool does not declare script_relpath.")
        return 1

    script_path = root / "dev" / "scripts" / str(script_relpath)

    if not script_path.exists():
        print("ERROR: script not found:")
        print(f"  {script_path}")
        return 1

    cmd = [str(py), str(script_path)]

    # Caso especial: scripts que necesitan root del workspace
    if tool.get("tool_id") == "generate_ai_system_report":
        cmd += ["--app_root", str(root)]

    print(f"SCRIPT: {script_path}")
    print("CMD:    " + " ".join(cmd))
    print()

    return subprocess.call(cmd)


def run_wrapper(root: Path, tool: dict[str, Any]) -> int:
    launch = tool.get("launch", {}) or {}
    wrapper_bat = launch.get("wrapper_bat", "")
    wrapper_path = root / str(wrapper_bat)

    if not wrapper_path.exists():
        print("ERROR: wrapper BAT not found:")
        print(f"  {wrapper_path}")
        return 1

    print(f"WRAPPER: {wrapper_path}")
    print()
    return subprocess.call([str(wrapper_path)], shell=True)


def main() -> int:
    if len(sys.argv) < 3:
        print("Usage: dev_tool_launcher.py <root> <registry>")
        return 1

    root = Path(sys.argv[1]).resolve()
    registry_path = Path(sys.argv[2]).resolve()
    py = root / "system" / "bin" / "runtimes" / "python" / "current" / "python" / "python.exe"

    if not py.exists():
        print("ERROR: portable Python runtime not found:")
        print(f"  {py}")
        return 1

    if not registry_path.exists():
        print("ERROR: registry not found:")
        print(f"  {registry_path}")
        return 1

    try:
        tools = load_registry(registry_path)
    except Exception as exc:
        print("ERROR: could not load registry.")
        print(exc)
        return 1

    if not tools:
        print("No launchable dev tools found in registry.")
        input("\nPresione Intro para continuar . . . ")
        return 0

    while True:
        print("\n" * 2)
        display_menu(tools)

        choice = input("Selecciona numero y pulsa Intro: ").strip()

        if choice == "0":
            return 0

        if not choice.isdigit():
            print("\nSeleccion no valida.")
            input("Presione Intro para continuar . . . ")
            continue

        idx = int(choice)
        if idx < 1 or idx > len(tools):
            print("\nSeleccion fuera de rango.")
            input("Presione Intro para continuar . . . ")
            continue

        tool = tools[idx - 1]
        launch = tool.get("launch", {}) or {}
        mode = str(launch.get("mode", "direct"))

        print()
        print("==================================================")
        print("Running dev tool")
        print("==================================================")
        print()
        print(f"TOOL ID:   {tool.get('tool_id', '')}")
        print(f"LABEL:     {launch.get('launch_label') or tool.get('name') or tool.get('tool_id', '')}")
        print(f"CATEGORY:  {tool.get('category', 'other')}")
        print(f"MODE:      {mode}")
        print()

        if mode == "direct":
            err = run_direct(root, py, tool)
        elif mode == "wrapper":
            err = run_wrapper(root, tool)
        elif mode == "args_required":
            print("Esta tool requiere argumentos y no esta expuesta como ejecucion directa.")
            err = 0
        elif mode == "hidden":
            print("Esta tool esta marcada como hidden.")
            err = 0
        else:
            print(f"ERROR: unsupported launch mode: {mode}")
            err = 1

        print()
        print("==================================================")
        if err == 0:
            print("Finished OK")
        else:
            print(f"Finished with errors. Exit code: {err}")
        print("==================================================")
        input("\nPresione Intro para continuar . . . ")


if __name__ == "__main__":
    raise SystemExit(main())