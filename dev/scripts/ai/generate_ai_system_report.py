#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "generate_ai_system_report.py",
  "version": "1.6.0",
  "type": "devtool",
  "subtype": "structural_auditor",
  "category": "dev",
  "tool_id": "dev_generate_ai_system_report",
  "description": "Genera el reporte Markdown AI_SYSTEM_REPORT_v1 mediante auditoría estructural, revisión de configuración y checks de invariantes del workspace portable, incluyendo coherencia registry↔filesystem, cobertura spec y reproducibilidad de tools.",
  "location_expected": "dev/scripts/ai",
  "entry_point": "main",
  "inputs": [
    {
      "type": "workspace",
      "location": "."
    },
    {
      "type": "config",
      "location": "system/config/runtime_registry.json"
    },
    {
      "type": "config",
      "location": "system/config/entrypoints.json"
    },
    {
      "type": "spec",
      "location": "system/spec"
    },
    {
      "type": "governance",
      "location": "system/governance"
    },
    {
      "type": "governance_index",
      "location": "system/governance/governance_index.json",
      "required": false
    }
  ],
  "outputs": [
    {
      "type": "file",
      "location": "dev/ai/inputs/AI_SYSTEM_REPORT_v1.md",
      "mode": "overwrite"
    }
  ],
  "capabilities": [
    "scan_workspace",
    "read_config",
    "read_spec",
    "read_governance",
    "resolve_governance_index",
    "analyze_script_meta",
    "detect_registry_drift",
    "detect_orphan_specs",
    "assess_tool_reproducibility",
    "generate_markdown_report"
  ],
  "invariants": [
    "INV-001", "INV-002", "INV-003", "INV-004", "INV-005", "INV-006",
    "INV-007", "INV-008", "INV-009", "INV-010", "INV-011", "INV-012",
    "INV-013", "INV-014", "INV-016", "INV-022", "INV-023", "INV-024", "INV-025"
  ],
  "modifies_system": false,
  "runtime_ref": "python"
}
================================================================================
generate_ai_system_report.py

Genera un reporte Markdown autocontenido para IA:

Mejoras v1.6 (GOBERNANZA INDEXADA ESTABLE):
- Lee system/governance/governance_index.json como fuente principal de descubrimiento documental si existe y es válido.
- Mantiene fallback legacy por heurística para compatibilidad.
- Añade sección de Governance Discovery y riesgos asociados al índice.

Genera un reporte Markdown autocontenido para IA:
  dev/ai/inputs/AI_SYSTEM_REPORT_v1.md

Mejoras v1.5 (AUDITORÍA ESTRUCTURAL + INVARIANTES AVANZADOS):
- Añade TREE filtrado (no-walk runtimes/runs/caches) para auditoría estructural.
- Añade resumen cuantitativo en ramas [no-walk] (dirs/files/size).
- Añade medición de huella del runtime Python portable (ficheros, tamaño, extensiones, top subdirs).
- Corrige categorización de tools: usa `category` (fallback `group`).
- Mantiene filosofía: sin rutas hardcodeadas (todo relativo a app_root), solo lectura.

Diseñado para:
- Leer directamente la app (fuente)
- Usar SCRIPT_META de cabeceras cuando exista
- No depender de reportes previos (los referencia si están)
- No modificar system/ (solo lectura)
================================================================================
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import re
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


# ----------------------------
# Helpers generales
# ----------------------------

IGNORE_DIR_NAMES = {
    "__pycache__",
    ".git",
    ".svn",
    ".hg",
    ".mypy_cache",
    ".pytest_cache",
    ".ruff_cache",
    "node_modules",
    "dist",
    "build",
    ".venv",
    "venv",
}

# Auditoría estructural: rutas "no-walk" por prefijo relativo (evita árboles gigantes/ruidosos)
NO_WALK_REL_PREFIXES = {
    "system/bin/runtimes",  # runtime embebido (gigante)
    "user/runs",            # runs (ruido)
    "dev/runs",             # outputs dev (ruido)
    "dev/ai/cache",         # cache/evidence (ruido)
}

TREE_SKIP_DIRS = {
    "__pycache__",
    ".git",
    ".svn",
    ".hg",
    ".mypy_cache",
    ".pytest_cache",
    ".ruff_cache",
    "node_modules",
    "dist",
    "build",
    ".venv",
    "venv",
}


def now_iso_utc() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def ensure_parent_dir(path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)


def safe_relpath(path: Path, root: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except Exception:
        return path.as_posix()


def read_text_limited(path: Path, max_bytes: int = 65536) -> str:
    # Leer cabecera (bytes) y decodificar a texto
    with path.open("rb") as f:
        data = f.read(max_bytes)
    return data.decode("utf-8", errors="replace")


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path, max_bytes: int = 2_000_000) -> str:
    # Hash parcial si el archivo es enorme (suficiente para fingerprint)
    h = hashlib.sha256()
    with path.open("rb") as f:
        while True:
            chunk = f.read(65536)
            if not chunk:
                break
            h.update(chunk)
            if f.tell() >= max_bytes:
                break
    return h.hexdigest()


def human_mb(nbytes: int) -> float:
    return round(nbytes / (1024 * 1024), 2)


def walk_files(root: Path) -> List[Path]:
    out: List[Path] = []
    if not root.exists():
        return out
    for dirpath, dirnames, filenames in os.walk(root):
        # filtra directorios ruidosos
        dirnames[:] = [d for d in dirnames if d not in IGNORE_DIR_NAMES]
        for fn in filenames:
            out.append(Path(dirpath) / fn)
    return out


def has_dir_named(root: Path, name: str) -> bool:
    """
    Devuelve True si existe algún directorio con el nombre indicado bajo root.
    Nota: no usa walk_files() porque esa función solo devuelve ficheros.
    """
    if not root.exists():
        return False
    for dirpath, dirnames, _ in os.walk(root):
        dirnames[:] = [d for d in dirnames if d not in IGNORE_DIR_NAMES]
        if name in dirnames:
            return True
    return False


def count_extensions(root: Path, exts: Tuple[str, ...]) -> Dict[str, int]:
    counts = {e: 0 for e in exts}
    other = 0
    for p in walk_files(root):
        suf = p.suffix.lower()
        if suf in counts:
            counts[suf] += 1
        else:
            other += 1
    counts["_other"] = other
    return counts


def approx_size_bytes(paths: List[Path]) -> int:
    total = 0
    for base in paths:
        for p in walk_files(base):
            try:
                total += p.stat().st_size
            except Exception:
                continue
    return total


def is_under_no_walk(path: Path, app_root: Path) -> bool:
    rel = safe_relpath(path, app_root).replace("\\", "/")
    return any(rel.startswith(prefix) for prefix in NO_WALK_REL_PREFIXES)


def build_tree_filtered(app_root: Path, max_depth: int = 4, max_lines: int = 600) -> List[str]:
    """
    Árbol compacto, filtrado. No profundiza en rutas masivas (runtimes/runs/caches).
    Devuelve líneas tipo tree (texto) para incluir en markdown.
    """
    lines: List[str] = []

    def walk_dir(d: Path, depth: int) -> None:
        nonlocal lines
        if len(lines) >= max_lines:
            return
        if depth > max_depth:
            return
        if is_under_no_walk(d, app_root):
            s = summarize_dir(d, app_root)
            if s.get("present"):
                trunc = " (truncated)" if s.get("truncated") else ""
                lines.append(
                    f"{safe_relpath(d, app_root)}/ [no-walk] "
                    f"(dirs={s.get('total_dirs')}, files={s.get('total_files')}, size={s.get('total_size_mb')} MB){trunc}"
                )
            else:
                lines.append(f"{safe_relpath(d, app_root)}/ [no-walk]")
            return

        try:
            entries = sorted(list(d.iterdir()), key=lambda p: (not p.is_dir(), p.name.lower()))
        except Exception:
            return

        filtered: List[Path] = []
        for p in entries:
            if p.is_dir() and p.name in TREE_SKIP_DIRS:
                continue
            filtered.append(p)

        indent = "  " * depth
        for p in filtered:
            if len(lines) >= max_lines:
                return
            if p.is_dir():
                lines.append(f"{indent}{p.name}/")
                walk_dir(p, depth + 1)
            else:
                lines.append(f"{indent}{p.name}")

    lines.append(f"{app_root.name}/")
    walk_dir(app_root, depth=1)
    if len(lines) >= max_lines:
        lines.append("... [truncated]")
    return lines



def summarize_dir(path: Path, app_root: Path, max_files_scan: int = 200_000) -> Dict[str, Any]:
    """
    Resumen cuantitativo de un directorio sin desplegarlo:
    - total_dirs (excluye directorios ignorados)
    - total_files
    - total_size_mb
    Limita el escaneo por seguridad con max_files_scan.
    """
    if not path.exists() or not path.is_dir():
        return {"present": False}

    total_dirs = 0
    total_files = 0
    total_bytes = 0

    scanned_files = 0
    for dirpath, dirnames, filenames in os.walk(path):
        # filtra dirs ruidosos
        dirnames[:] = [d for d in dirnames if d not in IGNORE_DIR_NAMES]
        total_dirs += len(dirnames)

        for fn in filenames:
            total_files += 1
            scanned_files += 1
            if scanned_files > max_files_scan:
                return {
                    "present": True,
                    "total_dirs": total_dirs,
                    "total_files": total_files,
                    "total_size_mb": human_mb(total_bytes),
                    "truncated": True,
                    "note": f"file_scan_limit_exceeded={max_files_scan}",
                    "path": safe_relpath(path, app_root),
                }
            try:
                total_bytes += (Path(dirpath) / fn).stat().st_size
            except Exception:
                continue

    return {
        "present": True,
        "total_dirs": total_dirs,
        "total_files": total_files,
        "total_size_mb": human_mb(total_bytes),
        "truncated": False,
        "path": safe_relpath(path, app_root),
    }



def runtime_footprint(app_root: Path) -> Dict[str, Any]:
    """
    Huella del runtime Python portable:
    - total files
    - size MB
    - counts ext relevantes
    - top subdirs por tamaño (nivel 1)
    """
    rt = app_root / "system" / "bin" / "runtimes" / "python" / "current"
    if not rt.exists():
        return {"present": False, "path": safe_relpath(rt, app_root)}

    total_files = 0
    total_bytes = 0
    counts = {".py": 0, ".pyc": 0, ".pyd": 0, ".dll": 0, ".exe": 0}

    subdir_sizes: Dict[str, int] = {}

    for p in walk_files(rt):
        try:
            sz = p.stat().st_size
        except Exception:
            continue
        total_files += 1
        total_bytes += sz

        suf = p.suffix.lower()
        if suf in counts:
            counts[suf] += 1

        rel = safe_relpath(p, rt).replace("\\", "/")
        top = rel.split("/", 1)[0] if "/" in rel else "(root)"
        subdir_sizes[top] = subdir_sizes.get(top, 0) + sz

    top_subdirs = sorted(subdir_sizes.items(), key=lambda kv: kv[1], reverse=True)[:10]
    top_subdirs_fmt = [f"{name}: {human_mb(sz)} MB" for name, sz in top_subdirs]

    return {
        "present": True,
        "path": safe_relpath(rt, app_root),
        "total_files": total_files,
        "total_size_mb": human_mb(total_bytes),
        "counts": counts,
        "top_subdirs_by_size": top_subdirs_fmt,
    }


# ----------------------------
# JSON safe parse
# ----------------------------

def load_json_if_exists(path: Path) -> Tuple[Optional[Dict[str, Any]], Optional[str]]:
    if not path.exists():
        return None, "missing"
    try:
        return json.loads(path.read_text(encoding="utf-8")), None
    except Exception as e:
        return None, f"parse_error: {e}"


# ----------------------------
# SCRIPT_META extraction
# ----------------------------

META_MARKER = "SCRIPT_META"

def extract_first_json_object_after_marker(text: str, marker: str = META_MARKER) -> Tuple[Optional[Dict[str, Any]], Optional[str]]:
    """
    Intenta extraer el primer objeto JSON {...} posterior al marcador SCRIPT_META.
    Implementación robusta con balanceo de llaves.
    """
    idx = text.find(marker)
    if idx < 0:
        return None, "marker_not_found"

    start = text.find("{", idx)
    if start < 0:
        return None, "json_start_not_found"

    depth = 0
    in_str = False
    esc = False
    end = None

    for i in range(start, len(text)):
        ch = text[i]

        if in_str:
            if esc:
                esc = False
            elif ch == "\\":
                esc = True
            elif ch == '"':
                in_str = False
            continue

        if ch == '"':
            in_str = True
            continue

        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                end = i + 1
                break

    if end is None:
        return None, "json_end_not_found"

    blob = text[start:end]
    try:
        return json.loads(blob), None
    except Exception as e:
        return None, f"json_parse_error: {e}"


# ----------------------------
# runtime_registry analysis
# ----------------------------

def analyze_runtime_registry(reg: Dict[str, Any]) -> Dict[str, Any]:
    tools = reg.get("tools", {})
    total_tools = len(tools) if isinstance(tools, dict) else 0

    categories: Dict[str, int] = {}
    tools_without_presets: List[str] = []
    tools_without_runtime_ref: List[str] = []

    if isinstance(tools, dict):
        for tid, tdef in tools.items():
            if not isinstance(tdef, dict):
                continue

            grp = str(tdef.get("category") or tdef.get("group") or "uncategorized")
            categories[grp] = categories.get(grp, 0) + 1

            presets = tdef.get("presets")
            if not presets:
                tools_without_presets.append(tid)

            runtime_ref = tdef.get("runtime_ref") or tdef.get("runtime")
            if not runtime_ref:
                tools_without_runtime_ref.append(tid)

    integrity = "OK"
    if tools_without_runtime_ref or tools_without_presets:
        integrity = "WARN"

    return {
        "total_tools": total_tools,
        "categories": categories,
        "tools_without_presets": tools_without_presets,
        "tools_without_runtime_ref": tools_without_runtime_ref,
        "duplicate_tool_ids": [],  # dict keys can't duplicate; kept for schema consistency
        "registry_integrity_status": integrity,
    }


# ----------------------------
# entrypoints analysis (best-effort)
# ----------------------------

def analyze_entrypoints(app_root: Path, ep: Dict[str, Any]) -> Dict[str, Any]:
    """
    Detecta entrypoints declarados en esquemas antiguos (entrypoints/actions/commands)
    y en el esquema actual (default_mode + cli_entry/ui_entry).
    Además, valida scripts referenciados por script_relpath (y también strings .py).
    """
    declared: List[str] = []
    missing_scripts: List[str] = []
    referenced_scripts: List[str] = []

    def check_script_path(p: str) -> None:
        p_norm = p.replace("\\", "/")
        referenced_scripts.append(p_norm)
        sp = (app_root / p_norm).resolve()
        if not sp.exists():
            missing_scripts.append(p_norm)

    if isinstance(ep, dict):
        # 1) Esquemas legacy
        for k in ("entrypoints", "actions", "commands"):
            if k in ep:
                declared.append(k)

        # 2) Esquema DocGraph actual (contractual)
        if "default_mode" in ep:
            declared.append("default_mode")
        if isinstance(ep.get("cli_entry"), dict):
            declared.append("cli_entry")
            sr = ep["cli_entry"].get("script_relpath") or ep["cli_entry"].get("script")
            if isinstance(sr, str) and sr.strip():
                check_script_path(sr.strip())
        if isinstance(ep.get("ui_entry"), dict):
            declared.append("ui_entry")
            sr = ep["ui_entry"].get("script_relpath") or ep["ui_entry"].get("script")
            if isinstance(sr, str) and sr.strip():
                check_script_path(sr.strip())

        # 3) Fallback: escaneo genérico de strings que parezcan paths a .py
        def scan_obj(obj: Any) -> None:
            if isinstance(obj, dict):
                for _, vv in obj.items():
                    scan_obj(vv)
            elif isinstance(obj, list):
                for it in obj:
                    scan_obj(it)
            elif isinstance(obj, str):
                s = obj.strip()
                if s.endswith(".py") and ("/" in s or "\\" in s):
                    check_script_path(s)

        scan_obj(ep)

    # normaliza únicos
    declared = sorted(set(declared), key=lambda x: ("cli_entry", "ui_entry", "default_mode").index(x)
                      if x in ("cli_entry", "ui_entry", "default_mode") else 99)

    return {
        "declared_entrypoints": declared,
        "missing_referenced_scripts": sorted(set(missing_scripts)),
        "referenced_scripts": sorted(set(referenced_scripts)),
        "orphan_scripts_detected": "not_checked",
    }


# ----------------------------
# llm_provider analysis (best-effort)
# ----------------------------

def analyze_llm_provider(llm: Dict[str, Any]) -> Dict[str, Any]:
    llm_declared = llm.get("provider") or llm.get("runtime") or llm.get("name") or "unknown"
    mode = llm.get("mode") or llm.get("operation_mode") or "unknown"
    capabilities = llm.get("capabilities") or llm.get("features") or None
    risk_note = None
    if capabilities is None:
        risk_note = "capabilities not declared (assume minimal chat only)"
        capabilities = "not_declared"
    return {
        "llm_declared": llm_declared,
        "mode": mode,
        "capabilities_declared": capabilities,
        "risk_note": risk_note,
    }


# ----------------------------
# Latest healthcheck discovery (optional)
# ----------------------------

def find_latest_healthcheck(app_root: Path) -> Dict[str, Any]:
    hc_root = app_root / "user" / "runs" / "healthcheck"
    if not hc_root.exists():
        return {"found": False}

    subdirs = [p for p in hc_root.iterdir() if p.is_dir()]
    if not subdirs:
        return {"found": False}

    subdirs.sort(key=lambda p: p.name, reverse=True)
    latest = subdirs[0]
    hc_json = latest / "healthcheck.json"
    if not hc_json.exists():
        return {
            "found": True,
            "status": "unknown",
            "warnings": [],
            "critical_findings": 0,
            "last_run_detected": safe_relpath(latest, app_root),
            "runtime_path_verified": "unknown",
        }

    data, err = load_json_if_exists(hc_json)
    if err or not isinstance(data, dict):
        return {
            "found": True,
            "status": "unknown",
            "warnings": [f"could_not_parse: {err}"],
            "critical_findings": 0,
            "last_run_detected": safe_relpath(latest, app_root),
            "runtime_path_verified": "unknown",
        }

    status = data.get("status") or data.get("healthcheck", {}).get("status") or "unknown"
    warnings = data.get("warnings") or data.get("healthcheck", {}).get("warnings") or []
    if not isinstance(warnings, list):
        warnings = [str(warnings)]
    critical = data.get("critical_findings") or data.get("critical") or 0

    runtime_path_verified = data.get("python") or data.get("runtime") or "unknown"
    return {
        "found": True,
        "status": status,
        "warnings": warnings[:10],
        "critical_findings": int(critical) if isinstance(critical, int) else 0,
        "last_run_detected": safe_relpath(latest, app_root),
        "runtime_path_verified": runtime_path_verified,
    }


# ----------------------------
# Hardcoded path scan (light)
# ----------------------------

HARDCODE_PATTERNS = [

    # Windows absolute paths: C:\Users\...
    re.compile(r"(?i)\b[a-z]:\\[^\s\"']+"),

    # Windows absolute paths con slash: C:/Users/...
    re.compile(r"(?i)\b[a-z]:/[^\s\"']+"),

    # Windows UNC paths: \\server\share\... (exige al menos 1 segmento más)
    re.compile(r"\\\\[^\\/\s]+\\[^\\\s]+\\[^\s\"']+"),

    # POSIX home directories: /home/user/... or /Users/user/...
    re.compile(r"(?<![\w.-])/(home|Users)/[^\s\"']+"),

    # POSIX system/mount paths: /mnt/... /opt/... /var/... /etc/...
    re.compile(r"(?<![\w.-])/(mnt|opt|var|etc)/[^\s\"']+"),

]

# Detecta rutas relativas peligrosas (path traversal): ../.. , ..\.. , etc.
PATH_TRAVERSAL_PATTERN = re.compile(r"(?<![\w.-])(\.\./|\.\.\\)+")

def scan_hardcoded_paths(scripts: List[Path], max_hits: int = 10) -> List[Dict[str, Any]]:
    hits: List[Dict[str, Any]] = []

    for sp in scripts:
        try:
            text = read_text_limited(sp, max_bytes=200_000)
        except Exception:
            continue

        lines = text.splitlines()

        inside_script_meta = False

        for i, line in enumerate(lines, start=1):
            s = line.strip()

            # Ignorar comentarios
            if s.startswith("#"):
                continue

            # Inicio SCRIPT_META (marcador estándar)
            if s == "SCRIPT_META (NO MODIFICAR FORMATO)":
                inside_script_meta = True
                continue

            # Dentro del bloque: cerrar al detectar triple comilla (""" o ''')
            if inside_script_meta:
                if '"""' in s or "'''" in s:
                    inside_script_meta = False
                continue

            # Evitar analizar líneas que no pueden contener rutas
            if "\\" not in line and "/" not in line:
                continue

            # Evitar detectar rutas dentro de docstrings simples
            if s.startswith('"""') or s.startswith("'''"):
                continue

            # Comprobar patrones de hardcode
            matched = False

            for pattern in HARDCODE_PATTERNS:
                if pattern.search(line):
                    matched = True
                    break

            if PATH_TRAVERSAL_PATTERN.search(line):
                matched = True

            if matched:
                hits.append({
                    "file": sp,
                    "line": i,
                    "snippet": line.strip()[:200]
                })

            # Cortar si alcanzamos el límite
            if len(hits) >= max_hits:
                return hits

    return hits

# ----------------------------
# Illegal import scan (runtime hygiene)
# ----------------------------

ILLEGAL_IMPORT_RE = re.compile(
    r"^\s*(?:from|import)\s+(?:dev|user|tests)(?:\b|\.)",
    re.IGNORECASE
)

SYSPATH_HACK_RE = re.compile(
    r"\bsys\.path\.(append|insert)\s*\(",
    re.IGNORECASE
)

def scan_illegal_imports(scripts: List[Path], max_hits: int = 10) -> List[Dict[str, Any]]:
    hits: List[Dict[str, Any]] = []

    for sp in scripts:
        try:
            text = read_text_limited(sp, max_bytes=200_000)
        except Exception:
            continue

        inside_script_meta = False

        for i, line in enumerate(text.splitlines(), start=1):
            s = line.strip()

            if s.startswith("#"):
                continue

            # Inicio SCRIPT_META (marcador estándar)
            if s == "SCRIPT_META (NO MODIFICAR FORMATO)":
                inside_script_meta = True
                continue

            # Dentro del bloque SCRIPT_META: cerrar con triple comilla
            if inside_script_meta:
                if '"""' in s or "'''" in s:
                    inside_script_meta = False
                continue

            # Evitar docstrings simples
            if s.startswith('"""') or s.startswith("'''"):
                continue

            # Micro-optimización: si no hay señales, skip
            if "import" not in line and "sys.path" not in line:
                continue

            if ILLEGAL_IMPORT_RE.search(line) or SYSPATH_HACK_RE.search(line):
                hits.append({"file": sp, "line": i, "snippet": s[:200]})
                if len(hits) >= max_hits:
                    return hits

    return hits

# ----------------------------
# Runtime registry coverage scan
# ----------------------------

RUNTIME_TOOL_CATEGORIES = {"runtime", "product"}
SPEC_TEXT_EXTS = {".json", ".md", ".txt"}
SPEC_NOISE_TOKENS = {
    "tool", "dev", "run", "ui", "get", "set", "reset", "validate",
    "generate", "collect", "build", "report", "stats", "runtime",
}


def iter_registry_tools(registry: Dict[str, Any]) -> List[Tuple[str, Dict[str, Any]]]:
    tools = registry.get("tools", {}) if isinstance(registry, dict) else {}
    if not isinstance(tools, dict):
        return []
    return [(str(tool_id), tdef) for tool_id, tdef in tools.items() if isinstance(tdef, dict)]



def registry_tool_category(tdef: Dict[str, Any]) -> str:
    return str(tdef.get("category") or tdef.get("group") or "").strip().lower()



def registry_tool_script_relpath(tdef: Dict[str, Any]) -> Optional[str]:
    sr = tdef.get("script_relpath") or tdef.get("script") or tdef.get("entry")
    if not isinstance(sr, str) or not sr.strip():
        return None
    return sr.replace("\\", "/").lstrip("./")



def expected_prefixes_for_category(category: str) -> Tuple[str, ...]:
    if category in RUNTIME_TOOL_CATEGORIES:
        return ("system/bin/tools/",)
    if category == "dev":
        return ("dev/scripts/",)
    return tuple()



def is_internal_tool_module(relpath: str) -> bool:
    rel = relpath.replace("\\", "/")
    return rel.endswith("/__init__.py") or rel == "__init__.py" or "/_lib/" in rel



def read_script_meta_from_path(path: Path) -> Tuple[Optional[Dict[str, Any]], Optional[str]]:
    try:
        header = read_text_limited(path, max_bytes=65536)
    except Exception as exc:
        return None, f"read_error: {exc}"
    return extract_first_json_object_after_marker(header, marker=META_MARKER)



def detect_orphan_tool_scripts(
    app_root: Path,
    registry: Dict[str, Any],
) -> List[str]:
    tools_dir = app_root / "system" / "bin" / "tools"
    if not tools_dir.exists():
        return []

    scripts_fs = {
        safe_relpath(p, app_root).replace("\\", "/")
        for p in walk_files(tools_dir)
        if p.suffix.lower() == ".py" and not is_internal_tool_module(safe_relpath(p, app_root))
    }

    scripts_registry = set()
    for _tool_id, tdef in iter_registry_tools(registry):
        category = registry_tool_category(tdef)
        sr = registry_tool_script_relpath(tdef)
        if category in RUNTIME_TOOL_CATEGORIES and sr and sr.startswith("system/bin/tools/"):
            scripts_registry.add(sr)

    return sorted(scripts_fs - scripts_registry)



def detect_missing_tool_scripts(
    app_root: Path,
    registry: Dict[str, Any],
) -> List[str]:
    """
    Devuelve una lista de 'tool_id -> script_relpath' para tools declaradas
    cuyo script no existe en el filesystem.
    """
    missing: List[str] = []

    for tool_id, tdef in iter_registry_tools(registry):
        sr_norm = registry_tool_script_relpath(tdef)
        if sr_norm is None:
            continue

        script_path = (app_root / sr_norm).resolve()

        try:
            script_path.relative_to(app_root)
        except Exception:
            missing.append(f"{tool_id} -> {sr_norm} (outside_workspace)")
            continue

        if not script_path.exists():
            missing.append(f"{tool_id} -> {sr_norm}")

    return sorted(missing)



def detect_tool_scripts_outside_tools_dir(
    app_root: Path,
    registry: Dict[str, Any],
) -> List[str]:
    """
    Detecta tools runtime/product cuyo script_relpath apunta fuera de system/bin/tools.
    Las tools dev se validan contra dev/scripts en registry drift.
    """
    outside: List[str] = []

    for tool_id, tdef in iter_registry_tools(registry):
        category = registry_tool_category(tdef)
        sr_norm = registry_tool_script_relpath(tdef)
        if sr_norm is None:
            continue
        if category in RUNTIME_TOOL_CATEGORIES and not sr_norm.startswith("system/bin/tools/"):
            outside.append(f"{tool_id} -> {sr_norm}")

    return sorted(outside)



def build_tool_spec_tokens(tool_id: str, script_relpath: str) -> List[str]:
    raw_candidates = [tool_id, Path(script_relpath).stem]
    tokens = set()

    for raw in raw_candidates:
        norm = re.sub(r"[^a-z0-9]+", "_", raw.lower()).strip("_")
        if not norm:
            continue
        tokens.add(norm)

        if norm.startswith("tool_"):
            tokens.add(norm[5:])
        if norm.startswith("dev_"):
            tokens.add(norm[4:])

        parts = [p for p in norm.split("_") if p and p not in SPEC_NOISE_TOKENS]
        if parts:
            tokens.add("_".join(parts))
            tokens.update({p for p in parts if len(p) >= 4})

    return sorted({t for t in tokens if len(t) >= 4})



def detect_registry_drift(app_root: Path, registry: Dict[str, Any]) -> List[str]:
    issues: List[str] = []

    for tool_id, tdef in iter_registry_tools(registry):
        category = registry_tool_category(tdef)
        sr_norm = registry_tool_script_relpath(tdef)

        if sr_norm is None:
            issues.append(f"{tool_id} -> missing script reference in registry")
            continue

        expected_prefixes = expected_prefixes_for_category(category)
        if expected_prefixes and not any(sr_norm.startswith(prefix) for prefix in expected_prefixes):
            issues.append(
                f"{tool_id} -> unexpected location for category '{category}': {sr_norm}"
            )

        script_path = (app_root / sr_norm).resolve()
        try:
            script_path.relative_to(app_root)
        except Exception:
            issues.append(f"{tool_id} -> script path escapes workspace: {sr_norm}")
            continue

        if not script_path.exists() or not script_path.is_file():
            issues.append(f"{tool_id} -> missing script file: {sr_norm}")
            continue

        meta, err = read_script_meta_from_path(script_path)
        if meta is None:
            issues.append(f"{tool_id} -> script without valid SCRIPT_META ({err})")
            continue

        meta_tool_id = meta.get("tool_id")
        if isinstance(meta_tool_id, str) and meta_tool_id.strip() and meta_tool_id.strip() != tool_id:
            issues.append(f"{tool_id} -> META tool_id mismatch: {meta_tool_id.strip()}")

        meta_category = str(meta.get("category") or "").strip().lower()
        if meta_category and category and meta_category != category:
            issues.append(f"{tool_id} -> META category mismatch: registry={category}, meta={meta_category}")

        registry_runtime = str(tdef.get("runtime_ref") or tdef.get("runtime") or "").strip()
        meta_runtime = str(meta.get("runtime_ref") or meta.get("runtime") or "").strip()
        if registry_runtime and meta_runtime and registry_runtime != meta_runtime:
            issues.append(f"{tool_id} -> runtime_ref mismatch: registry={registry_runtime}, meta={meta_runtime}")

    return sorted(set(issues))



def detect_orphan_specs(app_root: Path, registry: Dict[str, Any]) -> List[str]:
    spec_root = app_root / "system" / "spec"
    if not spec_root.exists():
        return ["system/spec missing"]

    spec_files = [p for p in walk_files(spec_root) if p.suffix.lower() in SPEC_TEXT_EXTS]
    if not spec_files:
        return ["system/spec empty"]

    spec_index: List[Tuple[str, str]] = []
    for p in spec_files:
        rel = safe_relpath(p, app_root).replace("\\", "/").lower()
        try:
            body = read_text_limited(p, max_bytes=200_000).lower()
        except Exception:
            body = ""
        spec_index.append((rel, body))

    issues: List[str] = []
    for tool_id, tdef in iter_registry_tools(registry):
        category = registry_tool_category(tdef)
        if category not in RUNTIME_TOOL_CATEGORIES:
            continue

        sr_norm = registry_tool_script_relpath(tdef)
        if sr_norm is None:
            continue

        tokens = build_tool_spec_tokens(tool_id, sr_norm)
        if not tokens:
            continue

        found = False
        for rel, body in spec_index:
            if any(token in rel or token in body for token in tokens):
                found = True
                break

        if not found:
            issues.append(f"{tool_id} -> no supporting spec match in system/spec")

    return sorted(issues)


NONDETERMINISTIC_MARKERS = {
    "random.": "random module",
    "secrets.": "secrets module",
    "np.random": "numpy random",
    "numpy.random": "numpy random",
}


def meta_declares_explicit_stochasticity(meta: Dict[str, Any]) -> bool:
    if not isinstance(meta, dict):
        return False

    for key in ("stochastic", "non_deterministic", "nondeterministic"):
        if meta.get(key) is True:
            return True

    deterministic = meta.get("deterministic")
    if deterministic is False:
        return True

    reproducibility = meta.get("reproducibility")
    if isinstance(reproducibility, dict) and reproducibility:
        return True

    return False



def detect_tool_reproducibility_issues(app_root: Path, registry: Dict[str, Any]) -> List[str]:
    issues: List[str] = []

    for tool_id, tdef in iter_registry_tools(registry):
        category = registry_tool_category(tdef)
        if category not in RUNTIME_TOOL_CATEGORIES:
            continue

        sr_norm = registry_tool_script_relpath(tdef)
        if sr_norm is None:
            continue

        script_path = app_root / sr_norm
        if not script_path.exists() or not script_path.is_file():
            continue

        try:
            text = script_path.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue

        markers = sorted({label for marker, label in NONDETERMINISTIC_MARKERS.items() if marker in text})
        if not markers:
            continue

        meta, _ = read_script_meta_from_path(script_path)
        if meta_declares_explicit_stochasticity(meta or {}):
            continue

        issues.append(
            f"{tool_id} -> stochastic markers without explicit declaration: {', '.join(markers)}"
        )

    return sorted(issues)

# ----------------------------
# Governance discovery (indexed first, legacy fallback)
# ----------------------------

GOVERNANCE_INDEX_REL = Path("system/governance/governance_index.json")


def load_governance_index(app_root: Path) -> Tuple[Optional[Dict[str, Any]], Optional[str]]:
    index_path = app_root / GOVERNANCE_INDEX_REL
    if not index_path.exists():
        return None, "missing"

    try:
        data = json.loads(index_path.read_text(encoding="utf-8"))
    except Exception as exc:
        return None, f"parse_error: {exc}"

    if not isinstance(data, dict):
        return None, "invalid_root_type"
    if "schema_version" not in data:
        return None, "missing_schema_version"
    if "base_path" not in data:
        return None, "missing_base_path"
    if "documents" not in data or not isinstance(data["documents"], dict):
        return None, "missing_or_invalid_documents"
    if "priority_order" not in data or not isinstance(data["priority_order"], list):
        return None, "missing_or_invalid_priority_order"

    for doc_key, meta in data["documents"].items():
        if not isinstance(meta, dict):
            return None, f"invalid_document_entry:{doc_key}"
        if "file" not in meta:
            return None, f"missing_file:{doc_key}"
        if "role" not in meta:
            return None, f"missing_role:{doc_key}"

    for doc_key in data["priority_order"]:
        if doc_key not in data["documents"]:
            return None, f"priority_order_unknown_key:{doc_key}"

    return data, None


def resolve_governance_documents(app_root: Path, index_data: Dict[str, Any]) -> Dict[str, Any]:
    base_path = (app_root / str(index_data["base_path"])).resolve()
    resolved_docs: Dict[str, Any] = {}

    for doc_key, meta in index_data["documents"].items():
        raw_file = str(meta["file"])
        full_path = (base_path / raw_file).resolve()

        escapes_workspace = False
        try:
            full_path.relative_to(app_root)
        except Exception:
            escapes_workspace = True

        resolved_docs[doc_key] = {
            "file": raw_file,
            "role": str(meta.get("role") or "unknown"),
            "path": safe_relpath(full_path, app_root),
            "exists": full_path.exists() and full_path.is_file(),
            "escapes_workspace": escapes_workspace,
        }

    return {
        "schema_version": index_data.get("schema_version"),
        "base_path": safe_relpath(base_path, app_root),
        "priority_order": list(index_data.get("priority_order", [])),
        "documents": resolved_docs,
    }


def discover_governance(app_root: Path, max_opdef_files: int = 3000) -> Dict[str, Any]:
    index_data, index_err = load_governance_index(app_root)

    if isinstance(index_data, dict):
        resolved = resolve_governance_documents(app_root, index_data)
        docs = resolved.get("documents", {})
        opdef_doc = docs.get("operational_definition")

        result: Dict[str, Any] = {
            "mode": "indexed",
            "index_found": True,
            "index_valid": True,
            "index_error": None,
            "index_path": safe_relpath(app_root / GOVERNANCE_INDEX_REL, app_root),
            "schema_version": resolved.get("schema_version"),
            "base_path": resolved.get("base_path"),
            "priority_order": resolved.get("priority_order", []),
            "documents": docs,
        }

        if isinstance(opdef_doc, dict):
            result["operational_definition"] = {
                "found": bool(opdef_doc.get("exists")) and not bool(opdef_doc.get("escapes_workspace")),
                "path": opdef_doc.get("path"),
                "role": opdef_doc.get("role"),
                "version": "unknown",
                "estado": "unknown",
                "search_checked_files": 0,
                "source": "governance_index",
            }
            if result["operational_definition"]["found"]:
                op_path = app_root / str(opdef_doc.get("path"))
                try:
                    head = read_text_limited(op_path, max_bytes=12000)
                    for k, rx in OPDEF_META_PATTERNS.items():
                        m = rx.search(head)
                        if m:
                            result["operational_definition"][k] = m.group(1).strip() if k != "updated" else m.group(m.lastindex).strip()
                except Exception as exc:
                    result["operational_definition"]["read_error"] = str(exc)
        else:
            result["operational_definition"] = {
                "found": False,
                "path": None,
                "role": None,
                "version": "unknown",
                "estado": "unknown",
                "search_checked_files": 0,
                "source": "governance_index",
            }

        return result

    legacy = find_operational_definition(app_root, max_files=max_opdef_files)
    return {
        "mode": "legacy_fallback",
        "index_found": index_err != "missing",
        "index_valid": False,
        "index_error": index_err,
        "index_path": safe_relpath(app_root / GOVERNANCE_INDEX_REL, app_root),
        "schema_version": None,
        "base_path": None,
        "priority_order": [],
        "documents": {},
        "operational_definition": {**legacy, "source": "legacy_search"},
    }


# ----------------------------
# Definición Operativa discovery (best-effort)
# ----------------------------

OPDEF_NAME_PATTERNS = [
    re.compile(r"definici[oó]n\s+operativa", re.IGNORECASE),
    re.compile(r"definicion\s+operativa", re.IGNORECASE),
    re.compile(r"operativa\s+rev", re.IGNORECASE),
]

OPDEF_META_PATTERNS = {
    "version": re.compile(r"Versi[oó]n[_\s-]*definici[oó]n\s*:\s*(.+)", re.IGNORECASE),
    "estado": re.compile(r"estado[_\s-]*documento\s*:\s*(.+)", re.IGNORECASE),
    "updated": re.compile(r"(Updated_at|updated_at)\s*:\s*(.+)", re.IGNORECASE),
}

def find_operational_definition(app_root: Path, max_files: int = 3000) -> Dict[str, Any]:
    candidates: List[Path] = []
    checked = 0

    # 1) Prioridad: governance (lo más probable y pequeño)
    priority_roots = [
        app_root / "system" / "governance",
        app_root / "system" / "spec" / "docs",
        app_root / "system",
        app_root / "dev",
        app_root,
    ]

    seen = set()

    def should_skip_dir(rel_posix: str) -> bool:
        # reutiliza tus NO_WALK_REL_PREFIXES para evitar runtimes/runs/caches
        return any(rel_posix.startswith(prefix) for prefix in NO_WALK_REL_PREFIXES)

    for root in priority_roots:
        if not root.exists():
            continue

        for dirpath, dirnames, filenames in os.walk(root):
            # filtra dirs ruidosos
            dirnames[:] = [d for d in dirnames if d not in IGNORE_DIR_NAMES]

            # evita no-walk por prefijo relativo al app_root
            rel_dir = safe_relpath(Path(dirpath), app_root).replace("\\", "/")
            if should_skip_dir(rel_dir):
                dirnames[:] = []
                continue

            for fn in filenames:
                if checked >= max_files:
                    break
                checked += 1

                p = Path(dirpath) / fn
                if p in seen:
                    continue
                seen.add(p)

                if p.suffix.lower() not in {".md", ".txt"}:
                    continue

                if any(rx.search(p.name) for rx in OPDEF_NAME_PATTERNS):
                    candidates.append(p)

            if checked >= max_files:
                break

        if checked >= max_files:
            break

    if not candidates:
        return {"found": False, "search_checked_files": checked}

    candidates.sort(key=lambda x: (0 if "DEFIN" in x.name.upper() else 1, len(x.name)))
    best = candidates[0]

    meta = {"found": True, "path": safe_relpath(best, app_root), "search_checked_files": checked}
    try:
        head = read_text_limited(best, max_bytes=12_000)
        for k, rx in OPDEF_META_PATTERNS.items():
            m = rx.search(head)
            if m:
                meta[k] = m.group(1).strip() if k != "updated" else m.group(m.lastindex).strip()
    except Exception as e:
        meta["read_error"] = str(e)

    return meta


# ----------------------------
# Script scanning
# ----------------------------

@dataclass
class ScriptMetaSummary:
    total_scripts: int
    meta_ok: int
    missing_meta: int
    parse_errors: int
    scripts_missing_meta_list: List[str]
    scripts_parse_error_list: List[str]
    duplicates_by_script_name: Dict[str, List[str]]
    modifies_system_true: List[str]
    outside_expected_location: List[str]
    without_outputs_declared: List[str]


def gather_scripts(app_root: Path) -> List[Path]:
    scan_roots = [
        app_root / "dev" / "scripts",
        app_root / "system" / "bin" / "tools",
        app_root / "system" / "bin" / "entrypoints",
        app_root / "system" / "devtools",
    ]
    scripts: List[Path] = []
    for r in scan_roots:
        if not r.exists():
            continue
        for p in walk_files(r):
            if p.suffix.lower() != ".py":
                continue

            # --- PATCH: no escanear legacy/archivo muerto
            # Evita duplicados y parse_errors en auditoría.
            if "ficheros_antiguos" in p.parts:
                continue

            scripts.append(p)
    return scripts


def analyze_script_meta(app_root: Path, scripts: List[Path]) -> Tuple[ScriptMetaSummary, List[Dict[str, Any]]]:
    meta_ok = 0
    missing_meta = 0
    parse_errors = 0

    missing_meta_list: List[str] = []
    parse_error_list: List[str] = []

    indexed: List[Dict[str, Any]] = []

    by_name: Dict[str, List[str]] = {}
    modifies_system_true: List[str] = []
    outside_expected: List[str] = []
    without_outputs: List[str] = []

    scanned_scripts = 0

    for sp in scripts:
        rel = safe_relpath(sp, app_root).replace("\\", "/")

        # Excluir módulos internos de librería del cómputo de cobertura
        if is_internal_tool_module(rel):
            continue

        scanned_scripts += 1

        header = ""
        try:
            header = read_text_limited(sp, max_bytes=65536)
        except Exception:
            parse_errors += 1
            continue

        meta, err = extract_first_json_object_after_marker(header, marker=META_MARKER)
        if meta is None:
            if err == "marker_not_found":
                missing_meta += 1
                missing_meta_list.append(rel)
            else:
                parse_errors += 1
                parse_error_list.append(f"{rel} :: {err}")
            continue

        meta_ok += 1
        script_name = str(meta.get("script_name") or meta.get("name") or sp.name)
        by_name.setdefault(script_name, []).append(rel)

        if bool(meta.get("modifies_system", False)):
            modifies_system_true.append(rel)

        outputs = meta.get("outputs")
        if not outputs:
            without_outputs.append(rel)

        loc_exp = meta.get("location_expected")
        if isinstance(loc_exp, str) and loc_exp.strip():
            loc_norm = loc_exp.replace("\\", "/").strip()
            real_rel = safe_relpath(sp.parent, app_root).replace("\\", "/")
            if not real_rel.endswith(loc_norm.strip("/")) and loc_norm.strip("/") not in real_rel:
                outside_expected.append(f"{rel} (expected: {loc_norm})")

        indexed.append({
            "path": rel,
            "script_name": script_name,
            "version": meta.get("version"),
            "type": meta.get("type"),
            "category": meta.get("category"),
            "modifies_system": bool(meta.get("modifies_system", False)),
            "outputs_declared": bool(outputs),
        })

    duplicates = {k: v for k, v in by_name.items() if len(v) > 1}

    summary = ScriptMetaSummary(
        total_scripts=scanned_scripts,
        meta_ok=meta_ok,
        missing_meta=missing_meta,
        parse_errors=parse_errors,
        scripts_missing_meta_list=sorted(missing_meta_list),
        scripts_parse_error_list=sorted(parse_error_list),
        duplicates_by_script_name=duplicates,
        modifies_system_true=sorted(modifies_system_true),
        outside_expected_location=sorted(outside_expected),
        without_outputs_declared=sorted(without_outputs),
    )
    return summary, indexed

def detect_runtime_writes_to_system(app_root: Path) -> List[str]:
    hits: List[str] = []
    runtime_dir = app_root / "system" / "bin" / "tools" / "runtime"
    if not runtime_dir.is_dir():
        return hits

    write_markers = (
        "write_text(",
        "write_bytes(",
        ".mkdir(",
        "os.makedirs(",
        "mkdir(",
        "open(",
    )

    for py_file in runtime_dir.rglob("*.py"):
        try:
            text = py_file.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue

        normalized = text.replace("\\", "/")
        if ("/system/" in normalized or " system/" in normalized) and any(marker in text for marker in write_markers):
            hits.append(safe_relpath(py_file, app_root))

    return sorted(hits)



def detect_output_layout_issues(app_root: Path) -> List[str]:
    issues: List[str] = []
    output_dir = app_root / "user" / "output"

    if not output_dir.exists():
        issues.append("user/output missing")
        return issues

    if not output_dir.is_dir():
        issues.append("user/output is not a directory")
        return issues

    for item in output_dir.iterdir():
        if item.is_file():
            issues.append(safe_relpath(item, app_root))

    return sorted(issues)



def detect_artifacts_without_run(app_root: Path) -> List[str]:
    issues: List[str] = []
    output_dir = app_root / "user" / "output"
    runs_dir = app_root / "user" / "runs"

    if not output_dir.exists() or not runs_dir.exists():
        if not output_dir.exists():
            issues.append("user/output missing")
        if not runs_dir.exists():
            issues.append("user/runs missing")
        return issues

    if not output_dir.is_dir() or not runs_dir.is_dir():
        if not output_dir.is_dir():
            issues.append("user/output is not a directory")
        if not runs_dir.is_dir():
            issues.append("user/runs is not a directory")
        return issues

    # Primera versión: cualquier artefacto directamente en user/output
    # se considera trazabilidad insuficiente.
    for file_path in output_dir.rglob("*"):
        if file_path.is_file() and file_path.parent == output_dir:
            issues.append(safe_relpath(file_path, app_root))

    return sorted(issues)



def detect_ui_writes_to_runs_output(app_root: Path) -> List[str]:
    hits: List[str] = []
    ui_dir = app_root / "system" / "ui"
    if not ui_dir.is_dir():
        return hits

    write_markers = (
        "write_text(",
        "write_bytes(",
        ".mkdir(",
        "os.makedirs(",
        "mkdir(",
        "open(",
    )

    for py_file in ui_dir.rglob("*.py"):
        try:
            text = py_file.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue

        normalized = text.replace("\\", "/")
        if (
            ("user/runs" in normalized or "user/output" in normalized)
            and any(marker in text for marker in write_markers)
        ):
            hits.append(safe_relpath(py_file, app_root))

    return sorted(hits)



def extract_script_meta_json(text: str) -> Optional[Dict[str, Any]]:
    meta, _err = extract_first_json_object_after_marker(text, marker=META_MARKER)
    return meta if isinstance(meta, dict) else None



def detect_incomplete_script_meta(app_root: Path, scripts: List[Path]) -> List[str]:
    required_fields = {
        "tool_id",
        "script_name",
        "version",
        "type",
        "category",
        "description",
        "location_expected",
        "entry_point",
        "inputs",
        "outputs",
        "invariants",
    }

    issues: List[str] = []

    for py_file in scripts:
        rel = safe_relpath(py_file, app_root).replace("\\", "/")
        if is_internal_tool_module(rel):
            continue

        try:
            text = py_file.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue

        if "SCRIPT_META" not in text:
            continue

        meta = extract_script_meta_json(text)
        if meta is None:
            issues.append(f"{rel} -> malformed or invalid SCRIPT_META")
            continue

        missing = sorted(required_fields - set(meta.keys()))
        if missing:
            issues.append(f"{rel} -> missing fields: {', '.join(missing)}")

    return sorted(issues)


# ----------------------------
# Invariant checks (light, factual)
# ----------------------------

BINARY_EXTS_IN_SPEC = {".py", ".exe", ".dll", ".so", ".dylib", ".bat", ".cmd", ".sh"}


def check_invariants(
    app_root: Path,
    entrypoints_info: Dict[str, Any],
    hardcode_hits: List[Dict[str, Any]],
    illegal_import_hits: List[Dict[str, Any]],
    meta_summary: ScriptMetaSummary,
    orphan_tool_scripts: List[str],
    missing_tool_scripts: List[str],
    outside_tool_scripts: List[str],
    pycache_present: bool,
    pyc_present: bool,
    system_runtime_writes: List[str],
    output_layout_issues: List[str],
    artifact_run_link_issues: List[str],
    ui_write_issues: List[str],
    script_meta_contract_issues: List[str],
    registry_drift_issues: List[str],
    orphan_spec_issues: List[str],
    reproducibility_issues: List[str],
) -> Dict[str, Dict[str, str]]:

    inv: Dict[str, Dict[str, str]] = {}

    system_dir = app_root / "system"
    dev_dir = app_root / "dev"
    user_dir = app_root / "user"

    if system_dir.is_dir() and dev_dir.is_dir() and user_dir.is_dir():
        inv["INV-001 separation system/dev/user"] = {
            "status": "OK",
            "evidence": "folders present",
        }
    else:
        missing = [
            name for name, path in (("system", system_dir), ("dev", dev_dir), ("user", user_dir))
            if not path.is_dir()
        ]
        inv["INV-001 separation system/dev/user"] = {
            "status": "FAIL",
            "evidence": f"missing folders: {', '.join(missing)}" if missing else "missing expected roots",
        }

    spec_root = app_root / "system" / "spec"
    if not spec_root.exists():
        inv["INV-002 system/spec declarative"] = {
            "status": "NOT_CHECKED",
            "evidence": "system/spec not present",
        }
    else:
        bad: List[str] = []
        for p in walk_files(spec_root):
            rel = safe_relpath(p, app_root)
            if p.suffix.lower() in BINARY_EXTS_IN_SPEC:
                bad.append(rel)
            else:
                try:
                    txt = read_text_limited(p, max_bytes=16000)
                except Exception:
                    txt = ""
                if txt.startswith("#!") or "SCRIPT_META" in txt:
                    bad.append(rel)
            if len(bad) >= 10:
                break
        inv["INV-002 system/spec declarative"] = {
            "status": "OK" if not bad else "FAIL",
            "evidence": (
                "no executable/script-like files found"
                if not bad else
                f"found {len(bad)} suspicious files (showing up to 10): {bad}"
            ),
        }

    refs = entrypoints_info.get("referenced_scripts") or []
    dev_refs = [r for r in refs if str(r).replace("\\", "/").startswith("dev/")]
    inv["INV-003 dev not in runtime entrypoints"] = {
        "status": "OK" if not dev_refs else "FAIL",
        "evidence": (
            "no dev/ references in entrypoints"
            if not dev_refs else
            f"entrypoints reference dev/: {sorted(set(dev_refs))}"
        ),
    }

    inv["INV-004 runtime import hygiene"] = {
        "status": "OK" if not illegal_import_hits else "FAIL",
        "evidence": (
            "no illegal imports or sys.path hacks detected in system/"
            if not illegal_import_hits else
            f"illegal imports/sys.path hacks detected ({len(illegal_import_hits)})"
        ),
    }

    inv["INV-005 portability (no hardcoded paths)"] = {
        "status": "OK" if not hardcode_hits else "FAIL",
        "evidence": (
            "no hardcoded path patterns detected"
            if not hardcode_hits else
            f"hardcoded patterns detected ({len(hardcode_hits)})"
        ),
    }

    runs_ok = (app_root / "user" / "runs").exists()
    inv["INV-006 traceability runs/"] = {
        "status": "OK" if runs_ok else "FAIL",
        "evidence": "user/runs exists" if runs_ok else "user/runs missing",
    }

    registry_ok = not orphan_tool_scripts and not missing_tool_scripts
    registry_evidence_parts: List[str] = []
    if orphan_tool_scripts:
        registry_evidence_parts.append(f"orphan={len(orphan_tool_scripts)}")
    if missing_tool_scripts:
        registry_evidence_parts.append(f"missing={len(missing_tool_scripts)}")
    inv["INV-007 registry coverage complete"] = {
        "status": "OK" if registry_ok else "FAIL",
        "evidence": (
            "no orphan scripts and no missing declared scripts"
            if registry_ok else
            "; ".join(registry_evidence_parts)
        ),
    }

    inv["INV-008 runtime scripts under system/bin/tools"] = {
        "status": "OK" if not outside_tool_scripts else "FAIL",
        "evidence": (
            "all runtime/product tools resolve under system/bin/tools"
            if not outside_tool_scripts else
            f"declared runtime/product tools outside expected dir ({len(outside_tool_scripts)})"
        ),
    }

    if meta_summary.missing_meta == 0 and meta_summary.parse_errors == 0:
        meta_status = "OK"
        meta_evidence = "all scanned scripts provide valid SCRIPT_META"
    elif meta_summary.parse_errors > 0:
        meta_status = "FAIL"
        meta_evidence = f"missing={meta_summary.missing_meta}, parse_errors={meta_summary.parse_errors}"
    else:
        meta_status = "WARN"
        meta_evidence = f"scripts missing SCRIPT_META ({meta_summary.missing_meta})"
    inv["INV-009 SCRIPT_META coverage"] = {
        "status": meta_status,
        "evidence": meta_evidence,
    }

    inv["INV-010 runtime outputs declared"] = {
        "status": "OK" if not meta_summary.without_outputs_declared else "WARN",
        "evidence": (
            "all scanned scripts declare outputs"
            if not meta_summary.without_outputs_declared else
            f"scripts without declared outputs ({len(meta_summary.without_outputs_declared)})"
        ),
    }

    bytecode_ok = not pycache_present and not pyc_present
    bytecode_parts: List[str] = []
    if pycache_present:
        bytecode_parts.append("__pycache__ present")
    if pyc_present:
        bytecode_parts.append(".pyc present")
    inv["INV-011 bytecode hygiene"] = {
        "status": "OK" if bytecode_ok else "WARN",
        "evidence": (
            "no __pycache__ folders or .pyc files present"
            if bytecode_ok else
            ", ".join(bytecode_parts)
        ),
    }

    inv["INV-012 system immutable in runtime"] = {
        "status": "FAIL" if system_runtime_writes else "OK",
        "evidence": (
            f"writes detected in system/ ({len(system_runtime_writes)})"
            if system_runtime_writes else
            "no runtime writes to system/ detected"
        ),
    }

    inv["INV-013 strict output layout"] = {
        "status": "FAIL" if output_layout_issues else "OK",
        "evidence": (
            f"artifacts found directly under user/output ({len(output_layout_issues)})"
            if output_layout_issues else
            "output layout follows tool_id/timestamp convention"
        ),
    }

    inv["INV-014 every artifact linked to a run"] = {
        "status": "FAIL" if artifact_run_link_issues else "OK",
        "evidence": (
            f"artifacts without clear run linkage ({len(artifact_run_link_issues)})"
            if artifact_run_link_issues else
            "all detected artifacts have acceptable run linkage"
        ),
    }

    inv["INV-016 UI never writes runs/output directly"] = {
        "status": "FAIL" if ui_write_issues else "OK",
        "evidence": (
            f"UI write paths detected ({len(ui_write_issues)})"
            if ui_write_issues else
            "no UI writes to runs/output detected"
        ),
    }

    inv["INV-022 complete SCRIPT_META contract"] = {
        "status": "WARN" if script_meta_contract_issues else "OK",
        "evidence": (
            f"incomplete SCRIPT_META contracts ({len(script_meta_contract_issues)})"
            if script_meta_contract_issues else
            "all parsed SCRIPT_META blocks satisfy required fields"
        ),
    }

    inv["INV-023 registry drift"] = {
        "status": "FAIL" if registry_drift_issues else "OK",
        "evidence": (
            f"registry↔script drift detected ({len(registry_drift_issues)})"
            if registry_drift_issues else
            "registry entries and script metadata stay aligned"
        ),
    }

    inv["INV-024 orphan specs"] = {
        "status": "WARN" if orphan_spec_issues else "OK",
        "evidence": (
            f"tools without supporting spec match ({len(orphan_spec_issues)})"
            if orphan_spec_issues else
            "runtime/product tools have at least one supporting spec match"
        ),
    }

    inv["INV-025 tool reproducibility"] = {
        "status": "WARN" if reproducibility_issues else "OK",
        "evidence": (
            f"stochastic markers without explicit declaration ({len(reproducibility_issues)})"
            if reproducibility_issues else
            "no undeclared stochastic markers detected in runtime/product tools"
        ),
    }

    return inv


# ----------------------------
# Maturity score (0-100)
# ----------------------------

def score_from_healthcheck(status: str) -> int:
    s = str(status or "").strip().upper()
    if s == "OK":
        return 10
    if s == "WARN":
        return 5
    return 0


def compute_maturity_score(
    app_root: Path,
    reg_err: Optional[str],
    ep_info: Dict[str, Any],
    hardcode_hits_system: List[Dict[str, Any]],
    meta_summary: "ScriptMetaSummary",
    healthcheck_status: str,
    invariants: Optional[Dict[str, Dict[str, str]]] = None,
) -> Dict[str, Any]:
    """Compute an objective maturity score (0-100) from observable signals.

    NOTE: This is intentionally conservative and contract-driven. Only runtime-impacting
    signals should strongly affect the score.
    """
    details: List[Dict[str, Any]] = []

    # 1) Separation system/dev/user (10)
    sep_ok = all((app_root / x).exists() for x in ("system", "dev", "user"))
    sep_score = 10 if sep_ok else 0
    details.append({"metric": "separation_system_dev_user", "score": sep_score, "max": 10})

    # 2) Declarative config / registry integrity (15)
    if reg_err is None:
        decl_score = 15
    elif reg_err == "missing":
        decl_score = 0
    else:
        decl_score = 8
    details.append({"metric": "runtime_registry_integrity", "score": decl_score, "max": 15, "evidence": reg_err or "ok"})

    # 3) Entrypoints contract (10)
    declared = ep_info.get("declared_entrypoints") or []
    if isinstance(declared, str):
        declared = []
    has_cli = "cli_entry" in declared
    has_ui = "ui_entry" in declared
    if has_cli and has_ui:
        ep_score = 10
    elif has_cli or has_ui:
        ep_score = 5
    else:
        ep_score = 0
    details.append({"metric": "entrypoints_contract", "score": ep_score, "max": 10, "declared": declared})

    # 4) Portability runtime (20) — only system/
    n_hits = len(hardcode_hits_system or [])
    if n_hits == 0:
        port_score = 20
    elif n_hits <= 2:
        port_score = 10
    else:
        port_score = 0
    details.append({"metric": "runtime_portability_hardcode_system", "score": port_score, "max": 20, "hits": n_hits})

    # 5) SCRIPT_META coverage (25)
    total = max(1, meta_summary.total_scripts)
    ratio = meta_summary.meta_ok / total
    meta_score = int(round(25 * ratio))
    details.append({"metric": "script_meta_coverage", "score": meta_score, "max": 25, "ratio": round(ratio, 2)})

    # 6) Duplicates (10)
    dup_n = len(meta_summary.duplicates_by_script_name or {})
    if dup_n == 0:
        dup_score = 10
    elif dup_n == 1:
        dup_score = 5
    else:
        dup_score = 0
    details.append({"metric": "duplicate_script_names", "score": dup_score, "max": 10, "duplicates": dup_n})

    # 7) Healthcheck (10)
    hc_score = score_from_healthcheck(healthcheck_status)
    details.append({"metric": "healthcheck_status", "score": hc_score, "max": 10, "status": healthcheck_status})

    # 8) Invariant hygiene (10)
    fail_n = 0
    warn_n = 0
    if isinstance(invariants, dict):
        for payload in invariants.values():
            status = str(payload.get("status", "")).upper()
            if status == "FAIL":
                fail_n += 1
            elif status == "WARN":
                warn_n += 1

    if fail_n == 0 and warn_n == 0:
        inv_score = 10
    elif fail_n == 0 and warn_n <= 2:
        inv_score = 7
    elif fail_n <= 2:
        inv_score = 4
    else:
        inv_score = 0
    details.append({"metric": "invariant_hygiene", "score": inv_score, "max": 10, "fail": fail_n, "warn": warn_n})

    total_score = sum(d["score"] for d in details)
    total_max = sum(d["max"] for d in details)

    if total_max != 100:
        total_score = int(round(100 * (total_score / max(1, total_max))))

    band = (
        "EXCELLENT" if total_score >= 90 else
        "GOOD" if total_score >= 75 else
        "FAIR" if total_score >= 55 else
        "WEAK"
    )

    return {
        "score_0_100": total_score,
        "band": band,
        "details": details,
    }

# ----------------------------
# Markdown rendering
# ----------------------------

def md_list(items: List[str], empty_text: str = "- (none)") -> str:
    if not items:
        return empty_text
    return "\n".join([f"- {x}" for x in items])


def md_kv(d: Dict[str, Any]) -> str:
    if not d:
        return "- (none)"
    lines = []
    for k, v in d.items():
        lines.append(f"- {k}: {v}")
    return "\n".join(lines)


def md_invariants(inv: Dict[str, Any]) -> str:
    if not inv:
        return "- (none)"
    lines: List[str] = []
    for k in sorted(inv.keys()):
        v = inv[k]
        status = v.get("status", "unknown")
        evidence = v.get("evidence", "")
        lines.append(f"- **{k}**: {status} — {evidence}")
    return "\n".join(lines)


def build_report_md(
    app_root: Path,
    out_path: Path,
    metadata: Dict[str, Any],
    root_overview: Dict[str, Any],
    runtime_fp: Dict[str, Any],
    config_state: Dict[str, Any],
    governance: Dict[str, Any],
    opdef: Dict[str, Any],
    script_meta: Dict[str, Any],
    healthcheck: Dict[str, Any],
    structural_audit: Dict[str, Any],
    invariants: Dict[str, Any],
    activity: Dict[str, Any],
    risks: List[str],
    maturity: Dict[str, Any],
    maturity_score: Dict[str, Any],
    strategic_obs: List[str],
) -> str:
    opdef_block = ""
    if opdef.get("found"):
        opdef_block = (
            f"- Found: yes\n"
            f"- Path: {opdef.get('path')}\n"
            f"- Version: {opdef.get('version', 'unknown')}\n"
            f"- Estado: {opdef.get('estado', 'unknown')}\n"
            f"- Search checked files: {opdef.get('search_checked_files')}\n"
            f"- Source: {opdef.get('source', 'unknown')}\n"
        )
    else:
        opdef_block = (
            f"- Found: no\n"
            f"- Search checked files: {opdef.get('search_checked_files', 'unknown')}\n"
            f"- Source: {opdef.get('source', 'unknown')}\n"
        )

    governance_block = (
        f"- Discovery mode: {governance.get('mode')}\n"
        f"- Governance index found: {'yes' if governance.get('index_found') else 'no'}\n"
        f"- Governance index valid: {'yes' if governance.get('index_valid') else 'no'}\n"
        f"- Governance index path: {governance.get('index_path')}\n"
        f"- Governance index error: {governance.get('index_error')}\n"
        f"- Schema version: {governance.get('schema_version')}\n"
        f"- Base path: {governance.get('base_path')}\n"
        f"- Priority order:\n{md_list(governance.get('priority_order', []))}"
    )

    tree_text = root_overview.get("tree_text", "(not_generated)")

    return f"""# AI_SYSTEM_REPORT_v1

## 1. METADATA
- Generated at: {metadata.get("generated_at")}
- App root: {metadata.get("app_root")}
- App version (si existe): {metadata.get("app_version")}
- OS: {metadata.get("os")}
- Python runtime: {metadata.get("python_runtime")}
- Approx bundle size (MB): {metadata.get("approx_bundle_size_mb")}
- Workspace fingerprint: {metadata.get("workspace_fingerprint")}

## 2. ROOT STRUCTURE OVERVIEW

### Root folders detected
{md_list(root_overview.get("root_folders", []))}

### Missing expected folders
{md_list(root_overview.get("missing_expected_folders", []))}

### Unexpected root folders
{md_list(root_overview.get("unexpected_root_folders", []))}

### Folder tree (filtered)
```text
{tree_text}
```

### File counts (global workspace)
- Total .py: {root_overview.get("count_py")}
- Total .json: {root_overview.get("count_json")}
- Total .md: {root_overview.get("count_md")}
- Total other: {root_overview.get("count_other")}

## 3. CONFIGURATION STATE

### runtime_registry.json
- Path: {config_state.get("registry", {}).get("path")}
- Total tools: {config_state.get("registry", {}).get("total_tools")}
- Categories:
{md_kv(config_state.get("registry", {}).get("categories", {}))}
- Tools without presets:
{md_list(config_state.get("registry", {}).get("tools_without_presets", []))}
- Tools without runtime_ref:
{md_list(config_state.get("registry", {}).get("tools_without_runtime_ref", []))}
- Duplicate tool ids:
{md_list(config_state.get("registry", {}).get("duplicate_tool_ids", []))}
- Registry integrity status: {config_state.get("registry", {}).get("registry_integrity_status")}

### entrypoints.json
- Path: {config_state.get("entrypoints", {}).get("path")}
- Declared entrypoints: {config_state.get("entrypoints", {}).get("declared_entrypoints")}
- Missing referenced scripts:
{md_list(config_state.get("entrypoints", {}).get("missing_referenced_scripts", []))}
- Orphan scripts detected: {config_state.get("entrypoints", {}).get("orphan_scripts_detected")}

### llm_provider.json (si existe)
- Path: {config_state.get("llm_provider", {}).get("path")}
- LLM declared: {config_state.get("llm_provider", {}).get("llm_declared")}
- Mode: {config_state.get("llm_provider", {}).get("mode")}
- Capabilities declared: {config_state.get("llm_provider", {}).get("capabilities_declared")}
- Risk note: {config_state.get("llm_provider", {}).get("risk_note")}

## 4. GOVERNANCE DISCOVERY
{governance_block}

## 4.5 DEFINICIÓN OPERATIVA
{opdef_block}

## 5. SCRIPT_META ANALYSIS
- Total scripts scanned: {script_meta.get("total_scripts_scanned")}
- Scripts missing SCRIPT_META: {script_meta.get("scripts_missing_script_meta")}
- Scripts missing SCRIPT_META (paths):
{md_list(script_meta.get("scripts_missing_script_meta_list", []))}
- Duplicate script_name values:
{md_list(script_meta.get("duplicate_script_names", []))}
- Scripts with modifies_system=true:
{md_list(script_meta.get("modifies_system_true", []))}
- Scripts outside expected location:
{md_list(script_meta.get("outside_expected_location", []))}
- Scripts without declared outputs:
{md_list(script_meta.get("without_outputs_declared", []))}
- Parse errors: {script_meta.get("parse_errors")}
- Scripts with SCRIPT_META parse errors (paths):
{md_list(script_meta.get("scripts_parse_error_list", []))}
- (Optional) script_meta_index path: {script_meta.get("script_meta_index_path")}

## 6. HEALTHCHECK SUMMARY
- Status: {healthcheck.get("status")}
- Warnings:
{md_list(healthcheck.get("warnings", []))}
- Critical findings: {healthcheck.get("critical_findings")}
- Last run detected: {healthcheck.get("last_run_detected")}
- Runtime path verified: {healthcheck.get("runtime_path_verified")}

## 6.5 PYTHON RUNTIME FOOTPRINT
- Present: {runtime_fp.get("present")}
- Path: {runtime_fp.get("path")}
- Total files: {runtime_fp.get("total_files")}
- Total size (MB): {runtime_fp.get("total_size_mb")}
- Counts:
{md_kv(runtime_fp.get("counts", {}))}
- Top subdirs by size:
{md_list(runtime_fp.get("top_subdirs_by_size", []))}

## 7. STRUCTURAL AUDIT

### Runtime portability
- Hardcoded paths detected: {structural_audit.get("hardcoded_paths_detected")}
- Hardcoded examples:
{md_list(structural_audit.get("hardcoded_examples", []))}

### Runtime import hygiene
- Illegal imports detected: {structural_audit.get("illegal_imports_detected")}
- Illegal import examples:
{md_list(structural_audit.get("illegal_import_examples", []))}

### Registry coverage
- Orphan tool scripts detected: {structural_audit.get("orphan_tool_scripts_detected")}
- Orphan tool scripts:
{md_list(structural_audit.get("orphan_tool_scripts", []))}

- Missing tool scripts detected: {structural_audit.get("missing_tool_scripts_detected")}
- Missing tool scripts:
{md_list(structural_audit.get("missing_tool_scripts", []))}

- Tool scripts outside system/bin/tools detected: {structural_audit.get("tool_scripts_outside_tools_dir_detected")}
- Tool scripts outside system/bin/tools:
{md_list(structural_audit.get("tool_scripts_outside_tools_dir", []))}

### Bytecode hygiene
- __pycache__ presence: {structural_audit.get("pycache_present")}
- .pyc files present: {structural_audit.get("pyc_present")}

### Dev signals (non-blocking)
- Hardcoded paths detected in dev/: {structural_audit.get("hardcoded_paths_detected_dev")}
- Hardcoded examples in dev/:
{md_list(structural_audit.get("hardcoded_examples_dev", []))}

### Structural invariants (not yet implemented)
- system/ modified in runtime: {structural_audit.get("system_modified_in_runtime")}
- Spec violations detected: {structural_audit.get("spec_violations_detected")}
- Deviations from Base Tree: {structural_audit.get("deviations_from_base_tree")}

### Summary
- Structural issues count: {structural_audit.get("structural_issues_count")}

## 8. INVARIANTS CONFORMANCE (light checks)
{md_invariants(invariants)}

## 9. RUNTIME ACTIVITY SNAPSHOT
- Recent run directories detected:
{md_list(activity.get("recent_runs", []))}

## 10. AUTOMATED RISK FLAGS
{md_list(risks, empty_text="- (none)")}

## 11. ARCHITECTURAL MATURITY SIGNALS
- Maturity score (0–100): {maturity_score.get("score_0_100")} ({maturity_score.get("band")})
- Separation system/dev/user: {maturity.get("separation_system_dev_user")}
- Declarative configuration level: {maturity.get("declarative_config")}
- Modularity level: {maturity.get("modularity")}
- Dependency clarity: {maturity.get("dependency_clarity")}
- Technical debt level: {maturity.get("technical_debt")}
- Portability status: {maturity.get("portability")}

## 12. STRATEGIC OBSERVATIONS (FACTUAL, NOT OPINION)
{md_list(strategic_obs, empty_text="- (none)")}

---
Generated by: {safe_relpath(Path(__file__), app_root)} -> {safe_relpath(out_path, app_root)}
"""


# ----------------------------
# Main
# ----------------------------

def main() -> int:
    ap = argparse.ArgumentParser(
        description="Generate AI_SYSTEM_REPORT_v1.md from workspace (source)."
    )
    ap.add_argument("--app_root", default=".", help="Ruta raíz del workspace.")
    ap.add_argument(
        "--out",
        default=None,
        help="Ruta de salida (por defecto dev/ai/inputs/AI_SYSTEM_REPORT_v1.md).",
    )
    ap.add_argument(
        "--write_script_meta_index",
        action="store_true",
        help="Escribe índice detallado de SCRIPT_META en dev/ai/cache/evidence/latest/script_meta_index_v1.json",
    )
    ap.add_argument(
        "--max_opdef_files",
        type=int,
        default=3000,
        help="Máximo de ficheros a revisar en búsqueda de Definición Operativa (best-effort).",
    )

    # Mantener compatibilidad: acepta --tree-max-* y --tree_max_*
    ap.add_argument("--tree-max-depth", dest="tree_max_depth", type=int, default=4)
    ap.add_argument("--tree_max_depth", dest="tree_max_depth", type=int, default=4)
    ap.add_argument("--tree-max-lines", dest="tree_max_lines", type=int, default=600)
    ap.add_argument("--tree_max_lines", dest="tree_max_lines", type=int, default=600)

    args = ap.parse_args()

    app_root = Path(args.app_root).resolve()
    default_out = app_root / "dev" / "ai" / "inputs" / "AI_SYSTEM_REPORT_v1.md"
    out_path = Path(args.out).resolve() if args.out else default_out
    ensure_parent_dir(out_path)

    # --- Metadata inputs
    key_paths = [
        app_root / "system" / "config" / "runtime_registry.json",
        app_root / "system" / "config" / "entrypoints.json",
        app_root / "system" / "config" / "llm_provider.json",
    ]

    key_hashes: List[str] = [
        sha256_file(kp)[:16] if kp.exists() else "missing"
        for kp in key_paths
    ]

    counts = count_extensions(app_root, (".py", ".json", ".md"))
    counts_summary = (
        f"py={counts['.py']};json={counts['.json']};md={counts['.md']};other={counts['_other']}"
    )

    size_paths = [
        p for p in (app_root / "system", app_root / "dev", app_root / "user")
        if p.exists()
    ]
    approx_size = approx_size_bytes(size_paths) if size_paths else approx_size_bytes([app_root])

    # Fingerprint estable y barato (config + conteos + tamaño)
    fp_raw = (
        "|".join(key_hashes)
        + "|"
        + counts_summary
        + f"|approx_size={approx_size}"
    ).encode("utf-8")
    workspace_fingerprint = sha256_bytes(fp_raw)[:24]

    metadata = {
        # Timestamp del reporte
        "generated_at": now_iso_utc(),

        # Workspace root (útil para depuración y trazabilidad)
        "app_root": safe_relpath(app_root, app_root),
        "app_root_abs": app_root.as_posix(),

        # Versión de la app (si existe en config, se puede sobreescribir)
        "app_version": "unknown",

        # Campos top-level consumidos por el renderer actual
        "os": f"{platform.system()} {platform.release()} ({platform.version()})",
        "python_runtime": Path(sys.executable).as_posix(),
        "python_version": platform.python_version(),

        # Entorno de ejecución detallado
        "runtime_environment": {
            "os": f"{platform.system()} {platform.release()} ({platform.version()})",
            "platform": platform.platform(),
            "architecture": platform.machine(),
            "python_runtime": Path(sys.executable).as_posix(),
            "python_version": platform.python_version(),
            "python_implementation": platform.python_implementation(),
            "python_build": " ".join(platform.python_build()),
            "cwd": Path.cwd().as_posix(),
            "pid": os.getpid(),
        },

        # Tamaño aproximado del bundle portable
        "approx_bundle_size_mb": human_mb(approx_size),

        # Fingerprint estructural del workspace
        "workspace_fingerprint": workspace_fingerprint,
    }

    # --- Root overview
    root_folders = [p.name for p in app_root.iterdir() if p.is_dir()]
    expected = ["system", "dev", "user"]
    missing_expected = [x for x in expected if x not in root_folders]
    unexpected_root = [x for x in root_folders if x not in expected and x not in IGNORE_DIR_NAMES]

    tree_lines = build_tree_filtered(app_root, max_depth=int(args.tree_max_depth), max_lines=int(args.tree_max_lines))

    root_overview = {
        "root_folders": sorted(root_folders),
        "missing_expected_folders": missing_expected,
        "unexpected_root_folders": sorted(unexpected_root),
        "tree_text": "\n".join(tree_lines),
        "count_py": counts[".py"],
        "count_json": counts[".json"],
        "count_md": counts[".md"],
        "count_other": counts["_other"],
    }

    runtime_fp = runtime_footprint(app_root)

    # --- Config state: runtime_registry
    reg_path = app_root / "system" / "config" / "runtime_registry.json"

    # Carga segura del registry
    reg, reg_err = load_json_if_exists(reg_path)
    orphan_tool_scripts = detect_orphan_tool_scripts(app_root, reg if isinstance(reg, dict) else {})
    missing_tool_scripts = detect_missing_tool_scripts(app_root, reg if isinstance(reg, dict) else {})
    outside_tool_scripts = detect_tool_scripts_outside_tools_dir(app_root, reg if isinstance(reg, dict) else {})

    # Registry coverage (tools filesystem vs registry)
    orphan_tool_scripts = detect_orphan_tool_scripts(
        app_root,
        reg if isinstance(reg, dict) else {}
    )

    # Estado inicial del análisis del registry
    reg_info: Dict[str, Any] = {
        "path": safe_relpath(reg_path, app_root),

        # Estado de carga
        "registry_loaded": isinstance(reg, dict),
        "registry_error": None if not reg_err else reg_err,

        # Resumen de herramientas declaradas
        "total_tools": "unknown",
        "categories": {},

        # Problemas estructurales detectados
        "tools_without_presets": [],
        "tools_without_runtime_ref": [],
        "duplicate_tool_ids": [],

        # Estado de integridad del fichero
        "registry_integrity_status": (
            "missing" if reg_err == "missing"
            else ("WARN" if reg_err else "OK")
        ),
    }
    if isinstance(reg, dict):
        reg_info.update(analyze_runtime_registry(reg))
    elif reg_err and reg_err != "missing":
        reg_info["registry_integrity_status"] = f"WARN ({reg_err})"

    ep_path = app_root / "system" / "config" / "entrypoints.json"
    ep, ep_err = load_json_if_exists(ep_path)
    ep_info: Dict[str, Any] = {
        "path": safe_relpath(ep_path, app_root),
        "declared_entrypoints": "unknown" if ep_err is not None else [],
        "missing_referenced_scripts": [],
        "referenced_scripts": [],
        "orphan_scripts_detected": "not_checked",
    }
    if isinstance(ep, dict):
        ep_info.update(analyze_entrypoints(app_root, ep))

    llm_path = app_root / "system" / "config" / "llm_provider.json"
    llm, llm_err = load_json_if_exists(llm_path)
    llm_info: Dict[str, Any] = {
        "path": safe_relpath(llm_path, app_root),
        "llm_declared": "not_present" if llm_err == "missing" else "unknown",
        "mode": "unknown",
        "capabilities_declared": "not_declared",
        "risk_note": None,
    }
    if isinstance(llm, dict):
        llm_info.update(analyze_llm_provider(llm))

    config_state = {"registry": reg_info, "entrypoints": ep_info, "llm_provider": llm_info}

    # --- Governance discovery (indexed first, legacy fallback)
    governance = discover_governance(app_root, max_opdef_files=args.max_opdef_files)
    opdef = governance.get("operational_definition", {"found": False})

    # --- SCRIPT_META
    scripts = gather_scripts(app_root)
    meta_summary, meta_index = analyze_script_meta(app_root, scripts)

    script_meta_index_path = None
    if args.write_script_meta_index:
        script_meta_index_path = app_root / "dev" / "ai" / "cache" / "evidence" / "latest" / "script_meta_index_v1.json"
        ensure_parent_dir(script_meta_index_path)
        script_meta_index_path.write_text(json.dumps(meta_index, ensure_ascii=False, indent=2), encoding="utf-8")

    script_meta = {
        "total_scripts_scanned": meta_summary.total_scripts,
        "scripts_missing_script_meta": meta_summary.missing_meta,
        "parse_errors": meta_summary.parse_errors,
        "scripts_missing_script_meta_list": meta_summary.scripts_missing_meta_list,
        "scripts_parse_error_list": meta_summary.scripts_parse_error_list,
        "duplicate_script_names": [f"{k}: {v}" for k, v in sorted(meta_summary.duplicates_by_script_name.items())],
        "modifies_system_true": meta_summary.modifies_system_true,
        "outside_expected_location": meta_summary.outside_expected_location,
        "without_outputs_declared": meta_summary.without_outputs_declared,
        "script_meta_index_path": safe_relpath(script_meta_index_path, app_root) if script_meta_index_path else "(not_written)",
    }

    # --- Healthcheck (optional)
    hc = find_latest_healthcheck(app_root)
    healthcheck = {
        "status": hc.get("status", "not_found"),
        "warnings": hc.get("warnings", []) if hc.get("found") else [],
        "critical_findings": hc.get("critical_findings", 0) if hc.get("found") else 0,
        "last_run_detected": hc.get("last_run_detected", "not_found") if hc.get("found") else "not_found",
        "runtime_path_verified": hc.get("runtime_path_verified", "unknown") if hc.get("found") else "unknown",
    }

    # --- Structural audit (light, runtime-focused)
    scripts_system: List[Path] = []
    scripts_dev: List[Path] = []

    for p in scripts:
        rel = safe_relpath(p, app_root).replace("\\", "/")
        if rel.startswith("system/"):
            scripts_system.append(p)
        elif rel.startswith("dev/"):
            scripts_dev.append(p)

    hardcode_hits_system = scan_hardcoded_paths(scripts_system[:2000], max_hits=10)
    hardcode_hits_dev = scan_hardcoded_paths(scripts_dev[:2000], max_hits=10)
    illegal_import_hits_system = scan_illegal_imports(scripts_system[:2000], max_hits=10)
    hardcode_hits = hardcode_hits_system

    system_dir = app_root / "system"

    hardcoded_examples = [
        f"{safe_relpath(h['file'], app_root)}:{h['line']} :: {h['snippet']}"
        for h in hardcode_hits_system
    ]
    illegal_import_examples = [
        f"{safe_relpath(h['file'], app_root)}:{h['line']} :: {h['snippet']}"
        for h in illegal_import_hits_system
    ]

    pycache_present = has_dir_named(system_dir, "__pycache__") if system_dir.exists() else False
    pyc_present = any(p.suffix.lower() == ".pyc" for p in walk_files(system_dir)) if system_dir.exists() else False

    system_runtime_writes = detect_runtime_writes_to_system(app_root)
    output_layout_issues = detect_output_layout_issues(app_root)
    artifact_run_link_issues = detect_artifacts_without_run(app_root)
    ui_write_issues = detect_ui_writes_to_runs_output(app_root)
    script_meta_contract_issues = detect_incomplete_script_meta(app_root, scripts)
    registry_drift_issues = detect_registry_drift(app_root, reg if isinstance(reg, dict) else {})
    orphan_spec_issues = detect_orphan_specs(app_root, reg if isinstance(reg, dict) else {})
    reproducibility_issues = detect_tool_reproducibility_issues(app_root, reg if isinstance(reg, dict) else {})

    runtime_issues = {
        "hardcode_hits": hardcode_hits_system,
        "illegal_import_hits": illegal_import_hits_system,
        "orphan_tool_scripts": orphan_tool_scripts,
        "missing_tool_scripts": missing_tool_scripts,
        "outside_tool_scripts": outside_tool_scripts,
        "registry_drift": registry_drift_issues,
        "orphan_specs": orphan_spec_issues,
        "reproducibility": reproducibility_issues,
    }

    structural_audit = {
        "hardcoded_paths_detected": "yes" if runtime_issues["hardcode_hits"] else "no",
        "hardcoded_examples": hardcoded_examples,
        "illegal_imports_detected": "yes" if runtime_issues["illegal_import_hits"] else "no",
        "illegal_import_examples": illegal_import_examples,
        "orphan_tool_scripts_detected": "yes" if runtime_issues["orphan_tool_scripts"] else "no",
        "orphan_tool_scripts": runtime_issues["orphan_tool_scripts"],
        "missing_tool_scripts_detected": "yes" if runtime_issues["missing_tool_scripts"] else "no",
        "missing_tool_scripts": runtime_issues["missing_tool_scripts"],
        "tool_scripts_outside_tools_dir_detected": "yes" if runtime_issues["outside_tool_scripts"] else "no",
        "tool_scripts_outside_tools_dir": runtime_issues["outside_tool_scripts"],
        "hardcoded_paths_detected_dev": "yes" if hardcode_hits_dev else "no",
        "hardcoded_examples_dev": [
            f"{safe_relpath(h['file'], app_root)}:{h['line']} :: {h['snippet']}"
            for h in hardcode_hits_dev
        ],
        "pycache_present": "yes" if pycache_present else "no",
        "pyc_present": "yes" if pyc_present else "no",
        "structural_issues_count": (
            len(hardcode_hits_system)
            + len(illegal_import_hits_system)
            + len(orphan_tool_scripts)
            + len(missing_tool_scripts)
            + len(outside_tool_scripts)
            + len(registry_drift_issues)
            + len(orphan_spec_issues)
            + len(reproducibility_issues)
        ),
        "system_modified_in_runtime": "yes" if system_runtime_writes else "no",
        "spec_violations_detected": "yes" if orphan_spec_issues else "no",
        "deviations_from_base_tree": "not_checked",
    }

    invariants = check_invariants(
        app_root=app_root,
        entrypoints_info=ep_info,
        hardcode_hits=hardcode_hits,
        illegal_import_hits=illegal_import_hits_system,
        meta_summary=meta_summary,
        orphan_tool_scripts=orphan_tool_scripts,
        missing_tool_scripts=missing_tool_scripts,
        outside_tool_scripts=outside_tool_scripts,
        pycache_present=pycache_present,
        pyc_present=pyc_present,
        system_runtime_writes=system_runtime_writes,
        output_layout_issues=output_layout_issues,
        artifact_run_link_issues=artifact_run_link_issues,
        ui_write_issues=ui_write_issues,
        script_meta_contract_issues=script_meta_contract_issues,
        registry_drift_issues=registry_drift_issues,
        orphan_spec_issues=orphan_spec_issues,
        reproducibility_issues=reproducibility_issues,
    )

    # --- Activity snapshot (light)
    runs_root = app_root / "user" / "runs"
    recent_runs: List[str] = []
    if runs_root.exists():
        subdirs = [p for p in runs_root.iterdir() if p.is_dir()]
        subdirs.sort(key=lambda p: p.stat().st_mtime if p.exists() else 0, reverse=True)
        recent_runs = [safe_relpath(p, app_root) for p in subdirs[:10]]

    activity = {"recent_runs": recent_runs}

    # --- Risks (máx 10, simples)
    risks: List[str] = []
    seen: set = set()

    def add_risk(msg: str) -> None:
        if msg and msg not in seen:
            seen.add(msg)
            risks.append(msg)

    if missing_expected:
        add_risk(f"🔺 Missing expected root folders: {', '.join(missing_expected)}")

    if reg_err == "missing":
        add_risk("🔺 runtime_registry.json missing (cannot verify tools/runtimes).")

    if isinstance(reg, dict):
        if reg_info.get("tools_without_runtime_ref"):
            add_risk(f"🔸 Tools without runtime_ref: {len(reg_info.get('tools_without_runtime_ref'))}")
        if reg_info.get("tools_without_presets"):
            add_risk(f"🔸 Tools without presets: {len(reg_info.get('tools_without_presets'))}")

    if meta_summary.missing_meta > 0:
        add_risk(f"🔸 Scripts missing SCRIPT_META: {meta_summary.missing_meta}")
    if meta_summary.duplicates_by_script_name:
        add_risk(f"🔸 Duplicate script_name detected: {len(meta_summary.duplicates_by_script_name)}")
    if meta_summary.modifies_system_true:
        add_risk(f"🔺 Scripts declaring modifies_system=true: {len(meta_summary.modifies_system_true)}")

    if hardcode_hits:
        add_risk("🔸 Hardcoded path patterns detected (examples shown).")
    if illegal_import_hits_system:
        add_risk(f"🔺 Illegal imports detected in system/: {len(illegal_import_hits_system)}")
    if orphan_tool_scripts:
        add_risk(f"🔺 Tool scripts present but not declared in runtime_registry: {len(orphan_tool_scripts)}")
    if missing_tool_scripts:
        add_risk(f"🔺 Tools declared in runtime_registry but script missing: {len(missing_tool_scripts)}")
    if outside_tool_scripts:
        add_risk(f"🔺 Runtime/product tools declared outside system/bin/tools: {len(outside_tool_scripts)}")
    if registry_drift_issues:
        add_risk(f"🔺 Registry drift detected: {len(registry_drift_issues)}")
    if orphan_spec_issues:
        add_risk(f"🔸 Tools without supporting spec match: {len(orphan_spec_issues)}")
    if reproducibility_issues:
        add_risk(f"🔸 Reproducibility markers require review: {len(reproducibility_issues)}")

    if governance.get("index_found") and not governance.get("index_valid"):
        add_risk(f"🔺 governance_index invalid: {governance.get('index_error')}")
    if governance.get("mode") == "legacy_fallback":
        add_risk("🔸 Governance discovery using legacy fallback instead of governance_index.")
    for doc_key, meta in governance.get("documents", {}).items():
        if meta.get("escapes_workspace"):
            add_risk(f"🔺 Governance document escapes workspace: {doc_key}")
        elif not meta.get("exists"):
            add_risk(f"🔺 Governance document missing: {doc_key} -> {meta.get('path')}")

    if str(healthcheck.get("status", "")).upper() == "WARN":
        add_risk("🔸 Healthcheck reports WARN.")

    if isinstance(invariants, dict):
        for inv_name, inv_payload in invariants.items():
            status = str(inv_payload.get("status", "")).upper()
            if status == "FAIL":
                add_risk(f"🔺 {inv_name} -> FAIL")
            elif status == "WARN":
                add_risk(f"🔸 {inv_name} -> WARN")

    critical = [r for r in risks if r.startswith("🔺")]
    warn = [r for r in risks if not r.startswith("🔺")]
    risks = (critical + warn)[:12]

    # --- Maturity (heurística factual)
    separation = "OK" if all((app_root / x).exists() for x in ("system", "dev", "user")) else "WARN"
    declarative = "OK" if reg_path.exists() and reg_err is None else "WARN"

    meta_cov_ratio = meta_summary.meta_ok / max(1, meta_summary.total_scripts) if meta_summary.total_scripts else 1.0
    meta_cov = "OK" if meta_cov_ratio >= 0.8 else "WARN"

    portability = "WARN" if (hardcode_hits_system or illegal_import_hits_system) else "OK"

    maturity = {
        "separation_system_dev_user": separation,
        "declarative_config": declarative,
        "modularity": "not_scored",
        "dependency_clarity": meta_cov,
        "technical_debt": "not_scored",
        "portability": portability,
    }

    maturity_score = compute_maturity_score(
        app_root=app_root,
        reg_err=reg_err,
        ep_info=ep_info,
        hardcode_hits_system=hardcode_hits_system,
        meta_summary=meta_summary,
        healthcheck_status=str(healthcheck.get("status", "")),
        invariants=invariants,
    )


    strategic_obs: List[str] = []
    strategic_obs.append(f"Workspace fingerprint: {workspace_fingerprint}")
    strategic_obs.append(
        f"Config presence: registry={'yes' if reg_path.exists() else 'no'}, "
        f"entrypoints={'yes' if ep_path.exists() else 'no'}, "
        f"llm_provider={'yes' if llm_path.exists() else 'no'}"
    )
    strategic_obs.append(
        f"SCRIPT_META coverage: {meta_summary.meta_ok}/{meta_summary.total_scripts} parsed OK "
        f"(ratio={meta_cov_ratio:.2f}); missing={meta_summary.missing_meta}, parse_errors={meta_summary.parse_errors}"
    )
    if opdef.get("found"):
        strategic_obs.append(
            f"Operational Definition discovered at: {opdef.get('path')} "
            f"(version={opdef.get('version','unknown')}, estado={opdef.get('estado','unknown')})"
        )
    if hardcode_hits:
        strategic_obs.append("Hardcode scan found path-like patterns in some scripts (see Structural Audit examples).")
    if recent_runs:
        strategic_obs.append(f"Recent runs detected under user/runs/: {len(recent_runs)} dirs (top 10 listed).")
    if runtime_fp.get("present"):
        strategic_obs.append(
            f"Python runtime footprint: {runtime_fp.get('total_size_mb')} MB "
            f"({runtime_fp.get('total_files')} files) at {runtime_fp.get('path')}"
        )
    if registry_drift_issues:
        strategic_obs.append(f"Registry drift issues detected: {len(registry_drift_issues)}")
    if orphan_spec_issues:
        strategic_obs.append(f"Tools without supporting spec match: {len(orphan_spec_issues)}")
    if reproducibility_issues:
        strategic_obs.append(f"Reproducibility review required for tools: {len(reproducibility_issues)}")

    md = build_report_md(
        app_root=app_root,
        out_path=out_path,
        metadata=metadata,
        root_overview=root_overview,
        runtime_fp=runtime_fp,
        config_state=config_state,
        governance=governance,
        opdef=opdef,
        script_meta=script_meta,
        healthcheck=healthcheck,
        structural_audit=structural_audit,
        invariants=invariants,
        activity=activity,
        risks=risks,
        maturity=maturity,
        maturity_score=maturity_score,
        strategic_obs=strategic_obs,
    )
    out_path.write_text(md, encoding="utf-8")
    print(f"[OK] Wrote: {safe_relpath(out_path, app_root)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())