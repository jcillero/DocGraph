#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_healthcheck_runtime.py",
  "tool_id": "tool_healthcheck_runtime",
  "version": "1.6.0",
  "type": "runtime_tool",
  "category": "runtime",
  "description": "Runtime healthcheck avanzado del sistema portable con deteccion de drift, runtime binding, sys.path hacks, output layout, specs huerfanas y chequeos de reproducibilidad.",
  "location_expected": "system/bin/tools/runtime",
  "entry_point": "main",
  "runtime_required": true,
  "modifies_system": false,
  "outputs": [
    {
      "type": "report",
      "location": "user/runs/tool_healthcheck_runtime/<run_id>/healthcheck_runtime_<timestamp>.json"
    }
  ],
  "capabilities": [
    "runtime_binding_check",
    "core_import_check",
    "sys_path_hack_detection",
    "registry_drift_detection",
    "output_layout_check",
    "system_user_mirror_detection",
    "orphan_spec_detection",
    "reproducibility_check"
  ]
}
================================================================================
"""

from __future__ import annotations

import ast
import importlib
import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Set, Tuple

from system.core.runtime.tool_runtime import (
    close_tool_runtime,
    init_tool_runtime,
    mark_artifact_written,
)

TOOL_VERSION = "1.6.0"

EXCLUDED_SCAN_ROOTS: Set[str] = {
    "user",
    "dev",
    ".venv",
    "venv",
    "__pycache__",
}

RUNTIME_TOOL_ROOT = Path("system/bin/tools/runtime")
PRODUCT_TOOL_ROOT = Path("system/bin/tools/product")
SPEC_ROOT = Path("system/spec")


# ------------------------------------------------------------
# Utilities
# ------------------------------------------------------------

def now_utc() -> str:
    return datetime.now(timezone.utc).isoformat()


def rel_posix(root: Path, p: Path) -> str:
    try:
        return str(p.relative_to(root)).replace("\\", "/")
    except Exception:
        return str(p).replace("\\", "/")


def run_cmd(cmd: List[str], cwd: Optional[Path] = None) -> Tuple[int, str, str]:
    proc = subprocess.run(
        cmd,
        cwd=str(cwd) if cwd else None,
        capture_output=True,
        text=True,
        shell=False,
    )
    return proc.returncode, proc.stdout.strip(), proc.stderr.strip()


def load_json(path: Path) -> Optional[Dict[str, Any]]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
        if isinstance(data, dict):
            return data
    except Exception:
        return None
    return None


def registry_tools(registry: Dict[str, Any]) -> Dict[str, Dict[str, Any]]:
    tools = registry.get("tools", {})
    return tools if isinstance(tools, dict) else {}


def iter_python_files(root: Path) -> Iterable[Path]:
    for py in root.rglob("*.py"):
        if any(part in EXCLUDED_SCAN_ROOTS for part in py.parts):
            continue
        yield py


def collect_tool_files(system_root: Path) -> Dict[str, Path]:
    tools_dir = system_root / "bin" / "tools"
    found: Dict[str, Path] = {}
    if not tools_dir.exists():
        return found
    for py in tools_dir.rglob("*.py"):
        rel = py.relative_to(system_root).as_posix()
        found[rel] = py
    return found


# ------------------------------------------------------------
# Runtime checks
# ------------------------------------------------------------

def check_python_runtime(system_root: Path) -> Dict[str, Any]:
    python_exec = Path(sys.executable).resolve()

    rc, out, err = run_cmd([sys.executable, "--version"])
    python_version = out or err

    rc_pip, out_pip, err_pip = run_cmd([sys.executable, "-m", "pip", "--version"])

    runtime_root = system_root / "bin" / "runtimes" / "python"
    inside_runtime = runtime_root in python_exec.parents

    return {
        "python_executable": str(python_exec),
        "python_version": python_version,
        "pip_version": out_pip or err_pip,
        "pip_ok": rc_pip == 0,
        "inside_portable_runtime": inside_runtime,
    }


# ------------------------------------------------------------
# Import checks
# ------------------------------------------------------------

def try_import(module: str) -> Tuple[bool, Optional[str]]:
    try:
        importlib.import_module(module)
        return True, None
    except Exception as e:
        return False, f"{type(e).__name__}: {e}"


def check_core_imports() -> Dict[str, Any]:
    modules = [
        "system.core.runtime.tool_runtime",
        "system.core.runtime.paths",
        "system.core.logging.logging_runtime",
        "system.bin.entrypoints.registry_validate",
    ]

    details: Dict[str, Any] = {}
    ok_all = True
    for mod in modules:
        ok, err = try_import(mod)
        details[mod] = {"ok": ok, "error": err}
        ok_all = ok_all and ok

    return {"ok": ok_all, "details": details}


# ------------------------------------------------------------
# sys.path.insert detection (AST based)
# ------------------------------------------------------------

class SysPathInsertVisitor(ast.NodeVisitor):
    def __init__(self) -> None:
        self.offenders: List[int] = []

    def visit_Call(self, node: ast.Call) -> Any:
        func = node.func
        if isinstance(func, ast.Attribute) and func.attr == "insert":
            value = func.value
            if isinstance(value, ast.Attribute) and value.attr == "path":
                root = value.value
                if isinstance(root, ast.Name) and root.id == "sys":
                    self.offenders.append(node.lineno)
        self.generic_visit(node)


def scan_sys_path_insert(app_root: Path) -> Dict[str, Any]:
    offenders: List[Dict[str, Any]] = []

    for py in iter_python_files(app_root / "system"):
        try:
            source = py.read_text(encoding="utf-8", errors="ignore")
            tree = ast.parse(source)
        except Exception:
            continue

        visitor = SysPathInsertVisitor()
        visitor.visit(tree)
        if visitor.offenders:
            offenders.append({
                "file": rel_posix(app_root, py),
                "lines": visitor.offenders,
            })

    return {"ok": len(offenders) == 0, "offenders": offenders}


# ------------------------------------------------------------
# Registry drift
# ------------------------------------------------------------

def load_registry(system_root: Path) -> Optional[Dict[str, Any]]:
    return load_json(system_root / "config" / "runtime_registry.json")


def check_registry_drift(app_root: Path, system_root: Path) -> Dict[str, Any]:
    registry = load_registry(system_root)
    if not registry:
        return {"ok": False, "error": "registry not found or invalid"}

    tools = registry_tools(registry)
    fs_files = collect_tool_files(system_root)

    declared: Dict[str, str] = {}
    for tool_id, meta in tools.items():
        entry = meta.get("entry")
        if isinstance(entry, str) and entry.endswith(".py"):
            declared[tool_id] = entry.replace("\\", "/")

    fs_set = set(fs_files.keys())
    declared_set = set(declared.values())

    missing_scripts = sorted(declared_set - fs_set)
    orphan_scripts = sorted(fs_set - declared_set)

    mismatches: List[Dict[str, Any]] = []
    for tool_id, entry in declared.items():
        path = app_root / entry
        if not path.exists():
            continue
        meta = extract_script_meta(path)
        if not meta:
            mismatches.append({"tool": tool_id, "reason": "missing_script_meta", "entry": entry})
            continue
        script_name = meta.get("script_name")
        tool_meta_id = meta.get("tool_id")
        loc = meta.get("location_expected")

        expected_name = Path(entry).name
        expected_loc = str(Path(entry).parent).replace("\\", "/")

        if script_name != expected_name:
            mismatches.append({
                "tool": tool_id,
                "reason": "script_name_mismatch",
                "entry": entry,
                "declared": script_name,
                "expected": expected_name,
            })
        if tool_meta_id is not None and tool_meta_id != tool_id:
            mismatches.append({
                "tool": tool_id,
                "reason": "tool_id_mismatch",
                "entry": entry,
                "declared": tool_meta_id,
                "expected": tool_id,
            })
        if loc is not None and str(loc).rstrip("/") != expected_loc.rstrip("/"):
            mismatches.append({
                "tool": tool_id,
                "reason": "location_expected_mismatch",
                "entry": entry,
                "declared": loc,
                "expected": expected_loc,
            })

    ok = not missing_scripts and not orphan_scripts and not mismatches
    return {
        "ok": ok,
        "missing_scripts": missing_scripts,
        "orphan_scripts": orphan_scripts,
        "meta_mismatches": mismatches,
    }


# ------------------------------------------------------------
# Output layout
# ------------------------------------------------------------

def check_output_layout(app_root: Path) -> Dict[str, Any]:
    output_root = app_root / "user" / "output"
    if not output_root.exists():
        return {"ok": True, "direct_artifacts": []}

    direct_files: List[str] = []
    malformed_paths: List[str] = []

    for item in output_root.iterdir():
        if item.is_file():
            direct_files.append(item.name)
        elif item.is_dir():
            subdirs = [p for p in item.iterdir() if p.is_dir()]
            files = [p for p in item.iterdir() if p.is_file()]
            if files:
                malformed_paths.append(rel_posix(app_root, item))
            elif not subdirs:
                malformed_paths.append(rel_posix(app_root, item))

    ok = not direct_files and not malformed_paths
    return {
        "ok": ok,
        "direct_artifacts": sorted(direct_files),
        "malformed_paths": sorted(malformed_paths),
    }


# ------------------------------------------------------------
# system/user detection
# ------------------------------------------------------------

def check_system_user_mirror(system_root: Path) -> Dict[str, Any]:
    suspect = system_root / "user"
    if suspect.exists():
        return {
            "ok": False,
            "path": str(suspect),
            "message": "system/user detected (possible historical mis-write)",
        }
    return {"ok": True}


# ------------------------------------------------------------
# Orphan specs
# ------------------------------------------------------------

def extract_script_meta(path: Path) -> Optional[Dict[str, Any]]:
    try:
        text = path.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return None

    marker = "SCRIPT_META (NO MODIFICAR FORMATO)"
    if marker not in text:
        return None

    start = text.find("{", text.find(marker))
    end = text.find("}\n", start)
    if start == -1 or end == -1:
        end = text.find("}\r\n", start)
    if start == -1:
        return None
    # fallback: find matching closing fence line by last brace before separator
    if end == -1:
        sep = text.find("================================================================================", start)
        block = text[start:sep if sep != -1 else None]
    else:
        block = text[start:end + 1]

    try:
        data = json.loads(block)
        return data if isinstance(data, dict) else None
    except Exception:
        return None


def check_orphan_specs(app_root: Path, system_root: Path) -> Dict[str, Any]:
    registry = load_registry(system_root)
    if not registry:
        return {"ok": False, "error": "registry not found or invalid"}

    tools = registry_tools(registry)
    spec_names: Set[str] = set()
    if SPEC_ROOT.exists():
        for path in (app_root / SPEC_ROOT).rglob("*.json"):
            spec_names.add(path.name)
        for path in (app_root / SPEC_ROOT).rglob("*.md"):
            spec_names.add(path.name)

    without_support: List[str] = []
    for tool_id, meta in tools.items():
        if meta.get("category") != "product":
            continue
        entry = meta.get("entry")
        if not isinstance(entry, str):
            continue
        script_path = app_root / entry
        smeta = extract_script_meta(script_path) or {}
        notes = json.dumps(smeta, ensure_ascii=False)
        base = tool_id.replace("tool_", "")
        candidates = {
            f"{base}.json",
            f"{base}.md",
            f"{base}_spec.json",
            f"{base}_spec.md",
        }
        if not any(c in spec_names for c in candidates) and base not in notes:
            without_support.append(tool_id)

    return {"ok": len(without_support) == 0, "tools_without_supporting_spec": sorted(without_support)}


# ------------------------------------------------------------
# Reproducibility
# ------------------------------------------------------------

def check_reproducibility(app_root: Path, system_root: Path) -> Dict[str, Any]:
    suspicious: List[Dict[str, Any]] = []
    roots = [system_root / "bin" / "tools" / "runtime", system_root / "bin" / "tools" / "product"]
    markers = ["random.random(", "uuid.uuid4(", "secrets.token_", "numpy.random", "time.time("]

    for root in roots:
        if not root.exists():
            continue
        for py in root.rglob("*.py"):
            try:
                text = py.read_text(encoding="utf-8", errors="ignore")
            except Exception:
                continue
            hits = [m for m in markers if m in text]
            if hits:
                suspicious.append({
                    "file": rel_posix(app_root, py),
                    "markers": hits,
                })

    return {"ok": len(suspicious) == 0, "suspicious_markers": suspicious}


# ------------------------------------------------------------
# Build report
# ------------------------------------------------------------

def build_report(app_root: Path, system_root: Path) -> Dict[str, Any]:
    runtime = check_python_runtime(system_root)
    imports = check_core_imports()
    sys_path = scan_sys_path_insert(app_root)
    registry = check_registry_drift(app_root, system_root)
    outputs = check_output_layout(app_root)
    mirror = check_system_user_mirror(system_root)
    orphan_specs = check_orphan_specs(app_root, system_root)
    reproducibility = check_reproducibility(app_root, system_root)

    overall_ok = all([
        runtime.get("inside_portable_runtime", False),
        imports.get("ok", False),
        sys_path.get("ok", False),
        registry.get("ok", False),
        outputs.get("ok", False),
        mirror.get("ok", False),
        reproducibility.get("ok", False),
    ])

    status = "OK" if overall_ok else "WARN"

    return {
        "status": status,
        "generated_at": now_utc(),
        "checks": {
            "runtime": runtime,
            "imports": imports,
            "sys_path": sys_path,
            "registry_drift": registry,
            "output_layout": outputs,
            "system_user_mirror": mirror,
            "orphan_specs": orphan_specs,
            "reproducibility": reproducibility,
        },
    }


# ------------------------------------------------------------
# Save report
# ------------------------------------------------------------

def save_report(run_dir: Path, report: Dict[str, Any]) -> Path:
    stamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    out_path = run_dir / f"healthcheck_runtime_{stamp}.json"
    out_path.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
    return out_path


# ------------------------------------------------------------
# Main
# ------------------------------------------------------------

def main() -> int:
    rt = init_tool_runtime(
        tool_id="tool_healthcheck_runtime",
        tool_version=TOOL_VERSION,
        primary_work_unit_name="checks",
        primary_work_unit_count=1,
    )

    app_root = rt.app.app_root
    system_root = rt.app.system_root

    exit_code = 1
    run_status = "failed"

    try:
        report = build_report(app_root, system_root)
        out_path = save_report(rt.run_dir, report)
        mark_artifact_written(rt, "export", out_path)

        print(json.dumps(report, indent=2, ensure_ascii=False))

        exit_code = 0 if report["status"] == "OK" else 1
        run_status = "completed"
        return exit_code

    except Exception as e:
        print(json.dumps({"status": "ERROR", "error": str(e)}, indent=2, ensure_ascii=False))
        return 99

    finally:
        close_tool_runtime(
            rt,
            status=run_status,
            exit_code=exit_code,
            summary="Runtime healthcheck execution",
            primary_work_unit_name="checks",
            primary_work_unit_count=1,
        )


if __name__ == "__main__":
    raise SystemExit(main())
