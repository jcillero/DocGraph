# APP ANALYSIS BUNDLE
- Generated at: 2026-03-09 20:36:10
- App root: `App_DocGraph`
- Scripts included: 11

## Folder tree (filtered)
```text
App_DocGraph/
├── dev/
│   ├── ai/
│   │   └── inputs/
│   │       └── AI_SYSTEM_REPORT_v1.md
│   ├── config/
│   │   ├── dev_plugins_registry.json
│   │   └── dev_tools_registry.json
│   ├── dev/
│   │   └── ai/
│   │       └── inputs/
│   │           └── AI_SYSTEM_REPORT_v1.md
│   ├── docs/
│   │   └── archive/
│   │       ├── Propuesta arbol UI mínimo Rev01.txt
│   │       └── UI-ARCH-001_ui_dialogs_structure.md
│   ├── plugins/
│   │   └── PLUGIN_FORMAT_SPEC.md
│   ├── scripts/
│   │   ├── ai/
│   │   │   └── generate_ai_system_report.py
│   │   ├── diagnostics/
│   │   │   └── validate_logs.py
│   │   ├── reports/
│   │   │   ├── collect_app_bundle.py
│   │   │   └── generate_structure_report.py
│   │   ├── ui/
│   │   │   └── validate_ui_strings.py
│   │   ├── utils/
│   │   ├── build_icon_ico.py
│   │   └── dev_tool_launcher.py
│   ├── test/
│   │   └── test_product_telemetry.py
│   └── run_dev_tool.bat
├── system/
│   ├── app/
│   │   ├── core/
│   │   │   ├── credentials/
│   │   │   │   └── credentials_manager.py
│   │   │   ├── logging/
│   │   │   │   ├── logging_runtime.py
│   │   │   │   └── run_context.py
│   │   │   ├── plugins/
│   │   │   ├── preferences/
│   │   │   │   ├── preferences_controller.py
│   │   │   │   └── preferences_manager.py
│   │   │   ├── runner/
│   │   │   ├── runtime/
│   │   │   │   ├── app_context.py
│   │   │   │   ├── paths.py
│   │   │   │   └── tool_runtime.py
│   │   │   └── workspace/
│   │   ├── plugins/
│   │   │   ├── __init__.py
│   │   │   ├── plugin_loader.py
│   │   │   └── plugin_registry.py
│   │   └── ui/
│   │       ├── app/
│   │       │   ├── __init__.py
│   │       │   └── main_window.py
│   │       ├── controllers/
│   │       ├── dialogs/
│   │       │   ├── base/
│   │       │   │   ├── dialog_base.py
│   │       │   │   ├── form_dialog_base.py
│   │       │   │   ├── tabbed_dialog_base.py
│   │       │   │   └── wizard_dialog_base.py
│   │       │   ├── credentials/
│   │       │   ├── preferences/
│   │       │   │   ├── tabs/
│   │       │   │   │   └── tab_from_catalog.py
│   │       │   │   ├── preferences_dialog.py
│   │       │   │   └── prefs_field_classifier.py
│   │       │   └── dialog_registry.py
│   │       ├── fields/
│   │       │   ├── __init__.py
│   │       │   ├── base_field.py
│   │       │   ├── checkbox_field.py
│   │       │   ├── entry_field.py
│   │       │   ├── field_factory.py
│   │       │   ├── field_registry.py
│   │       │   ├── field_utils.py
│   │       │   ├── select_field.py
│   │       │   └── slider_field.py
│   │       ├── forms/
│   │       │   └── form_builder.py
│   │       ├── i18n/
│   │       │   ├── __init__.py
│   │       │   └── ui_strings_resolver.py
│   │       ├── menu/
│   │       │   └── menu_builder.py
│   │       ├── panels/
│   │       │   ├── __init__.py
│   │       │   ├── panel_biblioteca.py
│   │       │   ├── panel_chat.py
│   │       │   ├── panel_informes.py
│   │       │   ├── panel_inicio.py
│   │       │   └── panel_registro.py
│   │       ├── theme/
│   │       │   └── __init__.py
│   │       ├── views/
│   │       ├── widgets/
│   │       │   ├── __init__.py
│   │       │   └── scrollable_frame.py
│   │       ├── __init__.py
│   │       ├── UI-FORM-BUILDER-SPEC.md
│   │       └── UI_DECLARATIVE_FORM_SPEC.md
│   ├── assets/
│   │   ├── images/
│   │   │   ├── logos/
│   │   │   │   ├── ficheros_antiguos/
│   │   │   │   │   ├── docgraph_icon_active.png
│   │   │   │   │   ├── imagen_128v.png
│   │   │   │   │   ├── imagen_16v.png
│   │   │   │   │   ├── imagen_256v.png
│   │   │   │   │   ├── imagen_32v.png
│   │   │   │   │   └── imagen_64v.png
│   │   │   │   ├── docgraph_icon_active.ico
│   │   │   │   └── docgraph_icon_active_1024.png
│   │   │   ├── reports/
│   │   │   │   ├── escudo_arfer.png
│   │   │   │   ├── escudo_armada.png
│   │   │   │   └── escudo_jicofer.png
│   │   │   └── ui/
│   │   │       ├── backgrounds/
│   │   │       └── icons/
│   │   └── templates/
│   ├── bin/
│   │   ├── entrypoints/
│   │   │   ├── app_cli.py
│   │   │   ├── app_ui.py
│   │   │   └── registry_validate.py
│   │   ├── runtimes/ [no-walk]
│   │   └── tools/
│   │       ├── _lib/
│   │       │   ├── __init__.py
│   │       │   └── tool_common.py
│   │       ├── product/
│   │       │   ├── tool_csv_to_report_pdf.py
│   │       │   ├── tool_pdf_merge.py
│   │       │   └── tool_telemetry_product_audit.py
│   │       └── runtime/
│   │           ├── tool_get_credentials.py
│   │           ├── tool_get_preferences.py
│   │           ├── tool_healthcheck_runtime.py
│   │           ├── tool_reset_credentials.py
│   │           ├── tool_reset_preferences.py
│   │           ├── tool_run_stats.py
│   │           ├── tool_set_credentials.py
│   │           ├── tool_set_preferences.py
│   │           ├── tool_ui_credentials_dialog.py
│   │           ├── tool_ui_preferences_dialog.py
│   │           └── tool_validate_preferences.py
│   ├── bootstrap/
│   ├── config/
│   │   ├── i18n/
│   │   ├── logging/
│   │   │   ├── logging_event_types_v1.json
│   │   │   └── work_units_catalog_v1.json
│   │   ├── plugin_format/
│   │   │   └── PLUGIN_FORMAT_SPEC.md.txt
│   │   ├── entrypoints.json
│   │   ├── plugin_registry.json
│   │   ├── runtime_bootstrap.json
│   │   └── runtime_registry.json
│   ├── governance/
│   │   ├── DEV_RUNBOOK_Rev03.txt
│   │   ├── GOVERNANCE_INDEX.json
│   │   ├── INVARIANTS_GUIDE.md
│   │   ├── OPERATIONAL_DEFINITION_Rev08.txt
│   │   ├── PLATFORM_ROADMAP_Rev03.txt
│   │   ├── UI_DESIGN_USER_GUIDE.md
│   │   └── UI_LAYOUT_Rev06.md
│   ├── plugins/
│   │   └── plugin_designer_core/
│   ├── snippets/
│   │   ├── configs/
│   │   ├── prompts/
│   │   └── templates/
│   ├── spec/
│   │   ├── docs/
│   │   │   ├── GUI_FRAMEWORK_INTEGRATION_GUIDE.md
│   │   │   └── WORKSPACE_STRUCTURE_EXPLAINED.md
│   │   ├── logging/
│   │   │   └── logging_spec_v1.md
│   │   ├── telemetry/
│   │   │   ├── product_report_schema.json
│   │   │   └── product_telemetry_spec.json
│   │   ├── ui/
│   │   │   ├── dialogs/
│   │   │   ├── dialogs_registry_v1.json
│   │   │   ├── ui_catalog.json
│   │   │   ├── ui_strings.json
│   │   │   ├── ui_strings_contract.json
│   │   │   ├── ui_theme.json
│   │   │   └── ui_theme_contract.json
│   │   ├── CONTRATO_UI_CORE.json
│   │   ├── DATA_MODEL_MIN.json
│   │   ├── plugin_manifest_schema.json
│   │   ├── preferences_catalog.json
│   │   └── runtime_registry_contract.json
│   └── tools/
│       └── telemetry/
│           ├── product_reporter.py
│           ├── product_telemetry.py
│           └── product_telemetry_wiring.py
├── user/
│   ├── data/
│   ├── input/
│   │   ├── Capítulo 4_ Ecosistema técnico del MDPS.pdf
│   │   ├── cobroDocumentoPago.pdf
│   │   └── RE-PD-OBS-GESTION DE OBSOLESCENCIAS_R06.5 simple.csv
│   ├── plugins/
│   │   └── personal_snippets/
│   ├── projects/
│   └── registry/
│       ├── documents/
│       ├── credentials.json
│       └── preferences.json
├── .gitignore
├── BASELINE_PLATFORM.md
├── CHANGELOG.md
├── CITATION.cff
├── CONTRIBUTING.md
├── LICENSE
├── README.md
├── START_APP.bat
├── START_APP_UI.bat
└── USER_MANUAL.md
```

## Scripts
> Nota: contenido incluido tal cual (UTF-8 con reemplazo en caracteres no decodificables).

### dev/scripts/ai/generate_ai_system_report.py
- size: 96205 bytes
- sha256: `413bd8692a105d18dc9dd456d7b5c7c2614fe58cb2cb62d65b9b708fe90f413b`

```text
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
```

### dev/scripts/build_icon_ico.py
- size: 2949 bytes
- sha256: `03c5145f411038f612619b3ef6cd47fd8c152ebe5e53fa6670aeb141f20ab267`

```text
"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "build_icon_ico.py",
  "version": "1.1.1",
  "type": "dev_tooling",
  "category": "assets",
  "description": "Genera un archivo .ico multi-resolución a partir de un PNG maestro.",
  "location_expected": "dev/scripts/",
  "runtime_required": false,
  "modifies_system": false,
  "output_location": "system/assets/images/logos/",
  "outputs": [
    "docgraph_icon_active.ico"
  ],
  "depends_on": [
    "Pillow"
  ],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "app_root BASE"
  ]
}
================================================================================
"""
import argparse
from pathlib import Path
from PIL import Image


DEFAULT_SIZES = [16, 24, 32, 48, 64, 128, 256, 512]


def parse_args():
    parser = argparse.ArgumentParser(description="Build multi-resolution ICO.")
    parser.add_argument("--input", required=True, help="Ruta PNG de entrada (relativa al root)")
    parser.add_argument("--output", required=True, help="Ruta ICO de salida (relativa al root)")
    parser.add_argument(
        "--sizes",
        default=",".join(str(s) for s in DEFAULT_SIZES),
        help="Lista de tamaños separados por coma (ej: 16,32,64)"
    )
    return parser.parse_args()


def _parse_sizes(sizes_str: str) -> list[tuple[int, int]]:
    raw = [s.strip() for s in sizes_str.split(",") if s.strip()]
    vals: list[int] = []
    for s in raw:
        try:
            v = int(s)
            if v > 0:
                vals.append(v)
            else:
                print(f"⚠️ Tamaño no válido (<=0) ignorado: {s}")
        except ValueError:
            print(f"⚠️ Tamaño inválido ignorado: {s}")

    vals = sorted(set(vals))
    return [(v, v) for v in vals]


def main():
    args = parse_args()
    root = Path(".").resolve()

    input_path = root / args.input
    output_path = root / args.output

    if not input_path.exists():
        print(f"❌ No se encuentra el archivo: {input_path}")
        return

    if output_path.suffix.lower() != ".ico":
        print(f"⚠️ El output no termina en .ico: {output_path.name}")

    sizes = _parse_sizes(args.sizes)
    if not sizes:
        print("❌ No hay tamaños válidos en --sizes")
        return

    # Asegura carpeta destino
    output_path.parent.mkdir(parents=True, exist_ok=True)

    # Asegura alpha estable
    img = Image.open(input_path).convert("RGBA")
    img.save(output_path, sizes=sizes)

    print("✅ ICO generado correctamente")
    print(f"   input : {input_path}")
    print(f"   output: {output_path}")
    print(f"   sizes : {', '.join(str(w) for (w, _h) in sizes)}")


if __name__ == "__main__":
    main()
```

### dev/scripts/dev_tool_launcher.py
- size: 5785 bytes
- sha256: `4d8621c2396af7f4b00601971657b01e9a45329363145df881f86b423028701d`

```text
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
    script_relpath = tool.get("script_relpath", "")
    script_path = root / "dev" / "scripts" / str(script_relpath)

    if not script_path.exists():
        print("ERROR: script not found:")
        print(f"  {script_path}")
        return 1

    print(f"SCRIPT: {script_path}")
    print()
    return subprocess.call([str(py), str(script_path)])


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
```

### dev/scripts/diagnostics/validate_logs.py
- size: 15002 bytes
- sha256: `49abcb8e80e2614db051bc6cfac02fdf7b73f93832b9bfbbdfa38f4313770a24`

```text
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
```

### dev/scripts/reports/collect_app_bundle.py
- size: 15448 bytes
- sha256: `f7385ee183a93505703840f0736bae1086e882bc11e97aaaf058b5946875fa61`

```text
"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "collect_app_bundle.py",
  "version": "1.0.0",
  "type": "dev_tooling",
  "category": "dev",
  "description": "Genera bundle portable para release (tree, manifest, limpieza pycache).",
  "location_expected": "dev/scripts/reports/",
  "runtime_required": true,
  "modifies_system": false,
  "output_location": "dev/runs/dev_bundle/<timestamp>/",
  "outputs": [
    "tree.txt",
    "bundle.md",
    "manifest.json"
  ],
  "depends_on": [],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "DEV_RUNBOOK",
    "MANUAL DE TIPOS DE DOCUMENTO"
  ]
}
================================================================================

DEV TOOL — collect_app_bundle.py

Genera un "bundle" de análisis del proyecto:
- Árbol de carpetas (con exclusiones para evitar binarios/runtimes gigantes)
- Un único fichero de texto/markdown con el contenido de scripts (para análisis posterior)
- Un manifest.json con hashes y tamaños

+ EXTRA (AUTO-LIMPIEZA):
- Limpia automáticamente __pycache__ y *.pyc dentro de system/ (por defecto)
  para que los bundles y el Healthcheck salgan limpios en releases.

Principios:
- Sin rutas hardcodeadas: todo relativo a la raíz de la app (detectada desde este script).
- No profundiza en carpetas de runtimes / binarios masivos.
- No incluye user/runs ni outputs generados.

Salida:
- dev/runs/dev_bundle/YYYYMMDD_HHMMSS/
  - tree.txt
  - bundle.md
  - manifest.json

Uso:
  python dev/scripts/reports/collect_app_bundle.py
  python dev/scripts/reports/collect_app_bundle.py --include-system-tools
  python dev/scripts/reports/collect_app_bundle.py --max-depth 12
  python dev/scripts/reports/collect_app_bundle.py --no-clean-system-pycaches
"""
from __future__ import annotations

import argparse
import hashlib
import json
import shutil
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Dict, Iterable, List


# -----------------------------
# Config (relative prefixes)
# -----------------------------
NO_WALK_REL_DIR_PREFIXES = [
    "system/bin/runtimes",
    "system/bin/engines",
    "system/bin/llm",
    "system/bin/ghostscript",
    "system/bin/pandoc",
    "system/bin/miktex",
    "system/bin/java",
    "system/bin/perl",
    "system/bin/tesseract",
    "system/bin/oxigraph",
    "system/bin/tika",
    "system/bin/VSCodium",
    "system/bin/wheels",
]

EXCLUDED_REL_DIR_PREFIXES = [
    "user/runs",
    "user/output",
    "dev/runs",  # evitamos auto-incluir bundles anteriores
    ".git",
]

EXCLUDED_PATH_SEGMENTS = {
    "__pycache__",
}

EXCLUDED_FILE_EXTS_ANYWHERE = {
    ".pyc",
    ".pyo",
}

SCRIPT_EXTS_DEFAULT = [".py", ".bat", ".cmd", ".ps1"]


@dataclass(frozen=True)
class ScriptItem:
    rel_path: str
    abs_path: Path
    size_bytes: int
    sha256: str


def rel_posix(root: Path, p: Path) -> str:
    return p.resolve().relative_to(root.resolve()).as_posix()


def starts_with_any_prefix(path_posix: str, prefixes: Iterable[str]) -> bool:
    for pref in prefixes:
        pref = pref.replace("\\", "/").rstrip("/")
        if path_posix == pref or path_posix.startswith(pref + "/"):
            return True
    return False


def path_has_any_segment(path_posix: str, segments: Iterable[str]) -> bool:
    parts = [p for p in path_posix.replace("\\", "/").split("/") if p]
    segset = set(segments)
    return any(p in segset for p in parts)


def is_excluded_path(root: Path, p: Path) -> bool:
    rp = rel_posix(root, p)
    if starts_with_any_prefix(rp, EXCLUDED_REL_DIR_PREFIXES):
        return True
    if path_has_any_segment(rp, EXCLUDED_PATH_SEGMENTS):
        return True
    if p.is_file() and p.suffix.lower() in EXCLUDED_FILE_EXTS_ANYWHERE:
        return True
    return False


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def detect_app_root(start: Path) -> Path:
    """
    Detecta la raíz de la app subiendo desde este script hasta encontrar system/config/runtime_registry.json
    (o, como fallback, system/ + dev/ + user/).
    """
    p = start.resolve()
    for _ in range(10):
        if (p / "system" / "config" / "runtime_registry.json").exists():
            return p
        if (p / "system").exists() and (p / "dev").exists() and (p / "user").exists():
            return p
        p = p.parent
    return start.resolve().parent


def clean_system_pycaches(app_root: Path) -> Dict[str, int]:
    """
    Borra:
    - system/**/__pycache__/
    - system/**/*.pyc
    - system/**/*.pyo

    Devuelve contadores para logging/manifest.
    """
    system_dir = app_root / "system"
    if not system_dir.exists():
        return {"pycache_dirs_removed": 0, "pyc_files_removed": 0}

    pycache_dirs = [p for p in system_dir.rglob("__pycache__") if p.is_dir()]
    pyc_files = [p for p in system_dir.rglob("*.pyc") if p.is_file()]
    pyo_files = [p for p in system_dir.rglob("*.pyo") if p.is_file()]

    dirs_removed = 0
    files_removed = 0

    # borrar bytecode primero
    for f in pyc_files + pyo_files:
        try:
            f.unlink()
            files_removed += 1
        except Exception:
            pass

    # borrar carpetas __pycache__ de forma fiable
    for d in pycache_dirs:
        try:
            shutil.rmtree(d, ignore_errors=False)
            dirs_removed += 1
        except Exception:
            # fallback tolerante
            try:
                shutil.rmtree(d, ignore_errors=True)
                dirs_removed += 1
            except Exception:
                pass

    return {"pycache_dirs_removed": dirs_removed, "pyc_files_removed": files_removed}


def iter_tree(root: Path, *, max_depth: int, include_no_walk_children: bool) -> List[str]:
    """
    Devuelve líneas estilo árbol. No profundiza en NO_WALK_REL_DIR_PREFIXES.
    Excluye rutas por prefijo y por segmentos (ej. __pycache__ en cualquier nivel).
    """
    lines: List[str] = []
    lines.append(root.name + "/")

    def walk(dir_path: Path, prefix: str, depth: int) -> None:
        if depth > max_depth:
            return

        try:
            entries = sorted(dir_path.iterdir(), key=lambda x: (not x.is_dir(), x.name.lower()))
        except PermissionError:
            lines.append(prefix + "└── [perm-denied]/")
            return

        filtered: List[Path] = []
        for e in entries:
            if is_excluded_path(root, e):
                continue
            rp = rel_posix(root, e)
            if starts_with_any_prefix(rp, NO_WALK_REL_DIR_PREFIXES):
                # la carpeta no se excluye (solo no se camina), salvo que contenga segmentos excluidos
                filtered.append(e)
            else:
                filtered.append(e)

        count = len(filtered)
        for i, e in enumerate(filtered):
            is_last = (i == count - 1)
            branch = "└── " if is_last else "├── "
            next_prefix = prefix + ("    " if is_last else "│   ")

            rp = rel_posix(root, e)

            if e.is_dir():
                # Segment exclusion: si la carpeta es __pycache__, ni se lista (ya filtrado)
                if starts_with_any_prefix(rp, NO_WALK_REL_DIR_PREFIXES):
                    lines.append(prefix + branch + e.name + "/ [no-walk]")
                    if include_no_walk_children:
                        try:
                            kids = sorted(e.iterdir(), key=lambda x: (not x.is_dir(), x.name.lower()))[:20]
                            for k in kids:
                                if is_excluded_path(root, k):
                                    continue
                                lines.append(next_prefix + "└── " + (k.name + "/" if k.is_dir() else k.name))
                        except Exception:
                            pass
                    continue

                lines.append(prefix + branch + e.name + "/")
                walk(e, next_prefix, depth + 1)
            else:
                lines.append(prefix + branch + e.name)

    walk(root, "", 1)
    return lines


def collect_scripts(
    root: Path,
    *,
    script_exts: List[str],
    include_system_tools: bool,
) -> List[ScriptItem]:
    """
    Recoge scripts del proyecto. Por defecto:
    - dev/scripts/**
    - system/bin/entrypoints/**
    - START_APP.bat (si existe)
    - (opcional) system/bin/tools/**

    Excluye rutas por prefijo y por segmentos (ej. __pycache__ en cualquier nivel)
    y excluye bytecode (*.pyc/*.pyo) aunque coincida con extensión.
    """
    targets: List[Path] = [
        root / "dev" / "scripts",
        root / "system" / "bin" / "entrypoints",
    ]
    if include_system_tools:
        targets.append(root / "system" / "bin" / "tools")

    include_files: List[Path] = []
    start_bat = root / "START_APP.bat"
    if start_bat.exists():
        include_files.append(start_bat)

    items: List[ScriptItem] = []

    def maybe_add_file(p: Path) -> None:
        try:
            if not p.exists() or not p.is_file():
                return
            if is_excluded_path(root, p):
                return

            rp = rel_posix(root, p)
            if starts_with_any_prefix(rp, NO_WALK_REL_DIR_PREFIXES):
                return

            size = p.stat().st_size
            items.append(ScriptItem(rel_path=rp, abs_path=p, size_bytes=size, sha256=sha256_file(p)))
        except Exception:
            return

    # archivos sueltos (START_APP.bat)
    for f in include_files:
        if f.suffix.lower() in [x.lower() for x in script_exts] or f.name.lower().endswith(".bat"):
            maybe_add_file(f)

    # carpetas objetivo
    for t in targets:
        if not t.exists():
            continue
        for p in t.rglob("*"):
            if not p.is_file():
                continue
            if is_excluded_path(root, p):
                continue
            if p.suffix.lower() not in [x.lower() for x in script_exts]:
                continue
            maybe_add_file(p)

    items.sort(key=lambda s: s.rel_path.lower())
    return items


def read_text_safe(path: Path) -> str:
    # No usamos chardet para evitar dependencias.
    return path.read_text(encoding="utf-8", errors="replace")


def write_bundle(
    out_bundle_md: Path,
    *,
    root: Path,
    scripts: List[ScriptItem],
    tree_lines: List[str],
) -> None:
    now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    out_bundle_md.parent.mkdir(parents=True, exist_ok=True)

    parts: List[str] = []
    parts.append("# APP ANALYSIS BUNDLE\n")
    parts.append(f"- Generated at: {now}\n")
    parts.append(f"- App root: `{root.name}`\n")
    parts.append(f"- Scripts included: {len(scripts)}\n\n")

    parts.append("## Folder tree (filtered)\n")
    parts.append("```text\n")
    parts.append("\n".join(tree_lines))
    parts.append("\n```\n\n")

    parts.append("## Scripts\n")
    parts.append("> Nota: contenido incluido tal cual (UTF-8 con reemplazo en caracteres no decodificables).\n\n")

    for s in scripts:
        parts.append(f"### {s.rel_path}\n")
        parts.append(f"- size: {s.size_bytes} bytes\n")
        parts.append(f"- sha256: `{s.sha256}`\n\n")
        parts.append("```text\n")
        try:
            parts.append(read_text_safe(s.abs_path))
        except Exception as e:
            parts.append(f"[ERROR reading file: {e}]\n")
        parts.append("\n```\n\n")

    out_bundle_md.write_text("".join(parts), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description="Dev bundle: tree + scripts content for analysis.")
    parser.add_argument("--max-depth", type=int, default=10, help="Profundidad máxima del árbol.")
    parser.add_argument(
        "--include-no-walk-children",
        action="store_true",
        help="Muestra primer nivel dentro de carpetas no-walk (sin recursión).",
    )
    parser.add_argument(
        "--include-system-tools",
        action="store_true",
        help="Incluye también system/bin/tools/*.py (por defecto solo dev/scripts y entrypoints).",
    )
    parser.add_argument(
        "--ext",
        action="append",
        default=None,
        help="Extensión adicional a incluir (p.ej. --ext .sh). Se puede repetir.",
    )
    parser.add_argument(
        "--no-clean-system-pycaches",
        action="store_true",
        help="No borra __pycache__ y *.pyc dentro de system/ antes de generar el bundle.",
    )

    args = parser.parse_args()

    app_root = detect_app_root(Path(__file__).parent)

    cleanup_stats = {"pycache_dirs_removed": 0, "pyc_files_removed": 0}
    if not args.no_clean_system_pycaches:
        cleanup_stats = clean_system_pycaches(app_root)

    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    out_dir = app_root / "dev" / "runs" / "dev_bundle" / timestamp
    out_dir.mkdir(parents=True, exist_ok=True)

    exts = SCRIPT_EXTS_DEFAULT[:]
    if args.ext:
        for e in args.ext:
            if not e.startswith("."):
                e = "." + e
            if e.lower() not in [x.lower() for x in exts]:
                exts.append(e)

    tree_lines = iter_tree(
        app_root,
        max_depth=int(args.max_depth),
        include_no_walk_children=bool(args.include_no_walk_children),
    )

    scripts = collect_scripts(
        app_root,
        script_exts=exts,
        include_system_tools=bool(args.include_system_tools),
    )

    # Outputs
    tree_txt = out_dir / "tree.txt"
    bundle_md = out_dir / "bundle.md"
    manifest_json = out_dir / "manifest.json"

    tree_txt.write_text("\n".join(tree_lines) + "\n", encoding="utf-8")
    write_bundle(bundle_md, root=app_root, scripts=scripts, tree_lines=tree_lines)

    manifest = {
        "generated_at": datetime.now().isoformat(timespec="seconds"),
        "app_root": str(app_root),
        "max_depth": int(args.max_depth),
        "include_system_tools": bool(args.include_system_tools),
        "script_exts": exts,
        "cleanup": {
            "system_pycaches_cleaned": (not bool(args.no_clean_system_pycaches)),
            **cleanup_stats,
        },
        "scripts": [{"path": s.rel_path, "size_bytes": s.size_bytes, "sha256": s.sha256} for s in scripts],
    }
    manifest_json.write_text(json.dumps(manifest, indent=2, ensure_ascii=False), encoding="utf-8")

    if not args.no_clean_system_pycaches:
        print(
            f"[CLEAN] system/: __pycache__ removed={cleanup_stats['pycache_dirs_removed']}, "
            f"*.pyc removed={cleanup_stats['pyc_files_removed']}"
        )

    print(f"OK: {tree_txt}")
    print(f"OK: {bundle_md}")
    print(f"OK: {manifest_json}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

### dev/scripts/reports/generate_structure_report.py
- size: 12269 bytes
- sha256: `5821991c2c12e77d2b7e7b03d935edd935d3a37e7c6ea5bc22a4c54caa9dc660`

```text
"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "generate_structure_report.py",
  "version": "1.1.0",
  "type": "dev_tooling",
  "category": "dev",
  "description": "Reporte estructural del árbol; no profundiza en binarios; extrae docstring inicial; incluye guards de baseline (runtime/lock/runtime_refs/WPy*/SCRIPT_META).",
  "location_expected": "dev/scripts/reports/",
  "runtime_required": true,
  "modifies_system": false,
  "output_location": "user/runs/dev_reports/<timestamp>/",
  "outputs": [
    "tree.txt",
    "tree.json",
    "audit_summary.json",
    "audit_summary.txt"
  ],
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

import fnmatch
import json
import os
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


# ---------------------------------------------------------------------
# App root + IO paths
# ---------------------------------------------------------------------
def get_app_root() -> Path:
    # dev/scripts/reports/generate_structure_report.py -> subir 3 niveles a raíz
    return Path(__file__).resolve().parents[3]


def make_out_dir(app_root: Path) -> Path:
    ts = datetime.now().strftime("%Y%m%d_%H%M%S")
    out = app_root / "user" / "runs" / "dev_reports" / ts
    out.mkdir(parents=True, exist_ok=True)
    return out


def load_registry(app_root: Path) -> dict:
    registry_path = app_root / "system" / "config" / "runtime_registry.json"
    # BOM-safe: si un editor/PS mete BOM, esto lo ignora sin romper.
    return json.loads(registry_path.read_text(encoding="utf-8-sig"))


# ---------------------------------------------------------------------
# Tree building (compact, readable)
# ---------------------------------------------------------------------
@dataclass
class TreeNode:
    name: str
    kind: str  # "dir" | "file"
    rel: str
    size: Optional[int] = None
    doc_head: Optional[str] = None
    children: Optional[List["TreeNode"]] = None


TEXT_EXTS = {".py", ".md", ".txt", ".json", ".bat", ".cmd"}
BINARY_EXTS = {".exe", ".dll", ".pyd", ".so", ".bin", ".dat", ".pyc"}


def _is_text_file(p: Path) -> bool:
    return p.suffix.lower() in TEXT_EXTS


def _is_binaryish(p: Path) -> bool:
    return p.suffix.lower() in BINARY_EXTS


def _safe_rel(app_root: Path, p: Path) -> str:
    return str(p.relative_to(app_root)).replace("\\", "/")


def _read_doc_head(py_file: Path, max_chars: int = 1200) -> Optional[str]:
    """
    Extrae el docstring inicial (si existe) de forma simple:
    - Lee el inicio del archivo (con errors='ignore')
    - Busca triple comillas al principio (''' o \"\"\")
    """
    try:
        head = py_file.read_text(encoding="utf-8", errors="ignore")[:max_chars]
    except Exception:
        return None

    s = head.lstrip()

    if s.startswith('"""'):
        end = s.find('"""', 3)
        if end != -1:
            return s[3:end].strip()

    if s.startswith("'''"):
        end = s.find("'''", 3)
        if end != -1:
            return s[3:end].strip()

    return None


def _should_skip_dir(name: str) -> bool:
    # Mantén esto conservador: no queremos ocultar cosas importantes, solo ruido típico.
    skip = {
        ".git",
        "__pycache__",
        ".pytest_cache",
        ".mypy_cache",
        ".ruff_cache",
        ".venv",
        "node_modules",
    }
    return name in skip


def build_tree(app_root: Path) -> TreeNode:
    def walk_dir(d: Path) -> TreeNode:
        node = TreeNode(
            name=d.name,
            kind="dir",
            rel=_safe_rel(app_root, d),
            children=[],
        )

        try:
            entries = sorted(d.iterdir(), key=lambda x: (not x.is_dir(), x.name.lower()))
        except Exception:
            entries = []

        for p in entries:
            if p.is_dir():
                if _should_skip_dir(p.name):
                    continue
                node.children.append(walk_dir(p))
            else:
                rel = _safe_rel(app_root, p)
                size = None
                try:
                    size = p.stat().st_size
                except Exception:
                    size = None

                doc_head = None
                if p.suffix.lower() == ".py":
                    doc_head = _read_doc_head(p)

                node.children.append(
                    TreeNode(
                        name=p.name,
                        kind="file",
                        rel=rel,
                        size=size,
                        doc_head=doc_head,
                        children=None,
                    )
                )

        return node

    return walk_dir(app_root)


def tree_to_txt(root: TreeNode) -> str:
    lines: List[str] = []

    def render(n: TreeNode, prefix: str = "") -> None:
        if n.kind == "dir":
            lines.append(f"{prefix}{n.name}/")
            if n.children:
                for ch in n.children:
                    render(ch, prefix + "  ")
        else:
            extra = ""
            if n.size is not None:
                extra = f" ({n.size} B)"
            lines.append(f"{prefix}{n.name}{extra}")
            if n.doc_head:
                # Dochead reducido para legibilidad
                snippet = " ".join(n.doc_head.split())
                if len(snippet) > 180:
                    snippet = snippet[:179] + "…"
                lines.append(f"{prefix}  doc: {snippet}")

    render(root, "")
    return "\n".join(lines)


def tree_to_json(root: TreeNode) -> dict:
    def pack(n: TreeNode) -> dict:
        out: Dict[str, Any] = {
            "name": n.name,
            "kind": n.kind,
            "rel": n.rel,
        }
        if n.size is not None:
            out["size"] = n.size
        if n.doc_head:
            out["doc_head"] = n.doc_head
        if n.children is not None:
            out["children"] = [pack(c) for c in n.children]
        return out

    return pack(root)


# ---------------------------------------------------------------------
# Baseline guards (anti-regresión)
# ---------------------------------------------------------------------
def _exists(app_root: Path, rel: str) -> bool:
    return (app_root / rel).exists()


def _glob_dirs(app_root: Path, rel_dir: str, pattern: str) -> List[str]:
    base = app_root / rel_dir
    if not base.exists():
        return []
    hits: List[str] = []
    for p in base.iterdir():
        if p.is_dir() and fnmatch.fnmatch(p.name, pattern):
            hits.append(_safe_rel(app_root, p))
    return sorted(hits)


def _collect_runtime_refs(registry: dict) -> Tuple[set[str], set[str]]:
    declared = set((registry.get("runtimes", {}) or {}).keys())
    used: set[str] = set()
    tools = (registry.get("tools", {}) or {})
    for _, t in tools.items():
        rr = t.get("runtime_ref")
        if rr:
            used.add(str(rr))
    return declared, used


def _has_script_meta(py_path: Path) -> bool:
    try:
        head = py_path.read_text(encoding="utf-8", errors="ignore")[:4000]
    except Exception:
        return False
    return "SCRIPT_META (NO MODIFICAR FORMATO)" in head


def run_baseline_guards(app_root: Path, registry: dict) -> dict:
    findings: List[dict] = []

    # 1) requirements.lock.txt
    lock_rel = "system/bin/runtimes/python/current/requirements.lock.txt"
    if not _exists(app_root, lock_rel):
        findings.append({"severity": "ERROR", "code": "LOCK_MISSING", "detail": lock_rel})

    # 2) runtime python apunta a current
    py = (registry.get("runtimes", {}) or {}).get("python", {})
    exe = str(py.get("console_executable", "") or "")
    exe_norm = exe.replace("\\", "/")
    if "system/bin/runtimes/python/current/" not in exe_norm:
        findings.append({"severity": "ERROR", "code": "RUNTIME_NOT_CURRENT", "detail": exe})

    # 3) detectar WPy* en runtimes/python/
    wpy = _glob_dirs(app_root, "system/bin/runtimes/python", "WPy*")
    if wpy:
        findings.append({"severity": "WARN", "code": "WPY_PRESENT", "detail": wpy})

    # 4) tools con runtime_ref inexistente
    declared, used = _collect_runtime_refs(registry)
    missing = sorted([r for r in used if r not in declared])
    if missing:
        findings.append({"severity": "ERROR", "code": "RUNTIME_REF_MISSING", "detail": missing})

    # 5) scripts sin SCRIPT_META (solo scripts del proyecto; NO runtime embebido)
    script_issues: List[str] = []
    for rel_base in ["system/bin", "dev/scripts"]:
        base = app_root / rel_base
        if not base.exists():
            continue

        for py_file in base.rglob("*.py"):
            rel = _safe_rel(app_root, py_file)

            # EXCLUSIONES: runtime embebido (stdlib/pip/etc.) no es "script del proyecto"
            if rel.startswith("system/bin/runtimes/"):
                continue

            if not _has_script_meta(py_file):
                script_issues.append(rel)

    if script_issues:
        findings.append(
            {"severity": "WARN", "code": "SCRIPT_META_MISSING", "detail": sorted(script_issues)}
        )

    status = "OK"
    if any(f["severity"] == "ERROR" for f in findings):
        status = "FAIL"
    elif findings:
        status = "WARN"

    return {"status": status, "findings": findings}


# ---------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------
def main() -> int:
    app_root = get_app_root()
    out_dir = make_out_dir(app_root)

    # Load registry (BOM-safe)
    try:
        registry = load_registry(app_root)
    except Exception as e:
        # Si el registry falla, aún generamos un reporte mínimo de error.
        (out_dir / "audit_summary.json").write_text(
            json.dumps(
                {
                    "status": "FAIL",
                    "findings": [
                        {"severity": "ERROR", "code": "REGISTRY_LOAD_FAIL", "detail": str(e)}
                    ],
                },
                indent=2,
                ensure_ascii=False,
            ),
            encoding="utf-8",
        )
        (out_dir / "audit_summary.txt").write_text(
            f"STATUS: FAIL\n- [ERROR] REGISTRY_LOAD_FAIL: {e}\n",
            encoding="utf-8",
        )
        print(f"OK: {out_dir / 'audit_summary.txt'}")
        return 2

    # Build tree
    tree = build_tree(app_root)
    tree_txt = tree_to_txt(tree)
    tree_json = tree_to_json(tree)

    (out_dir / "tree.txt").write_text(tree_txt, encoding="utf-8")
    (out_dir / "tree.json").write_text(
        json.dumps(tree_json, indent=2, ensure_ascii=False), encoding="utf-8"
    )

    # Guards
    guards = run_baseline_guards(app_root, registry)
    (out_dir / "audit_summary.json").write_text(
        json.dumps(guards, indent=2, ensure_ascii=False), encoding="utf-8"
    )

    lines = [f"STATUS: {guards['status']}"]
    for f in guards["findings"]:
        lines.append(f"- [{f['severity']}] {f['code']}: {f['detail']}")
    (out_dir / "audit_summary.txt").write_text("\n".join(lines) + "\n", encoding="utf-8")

    # Console OKs (mantén el estilo que ya tienes)
    print(f"OK: {out_dir / 'tree.txt'}")
    print(f"OK: {out_dir / 'tree.json'}")
    print(f"OK: {out_dir / 'audit_summary.txt'}")
    print(f"OK: {out_dir / 'audit_summary.json'}")

    # Exit code: 0 si OK/WARN, 1 si FAIL (para CI/manual gating)
    return 0 if guards["status"] in {"OK", "WARN"} else 1


if __name__ == "__main__":
    raise SystemExit(main())
```

### dev/scripts/ui/validate_ui_strings.py
- size: 19425 bytes
- sha256: `07a88a4dec91950aa9f83c193c179c7ff22a2907600daf288d1830425d818cf5`

```text
#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "validate_ui_strings.py",
  "version": "1.0.0",
  "type": "devtool",
  "subtype": "catalog_validator",
  "category": "dev",
  "description": "Valida el catálogo ui_strings.json y detecta inconsistencias entre claves definidas y usadas en el código Python, incluyendo placeholders, idiomas requeridos y claves deprecated.",
  "location_expected": "dev/scripts/ui",
  "entry_point": "main",
  "inputs": [
    {
      "type": "catalog",
      "location": "system/spec/ui/ui_strings.json"
    },
    {
      "type": "source_code",
      "location": "system/ui"
    }
  ],
  "outputs": [
    {
      "type": "stdout",
      "description": "Reporte de validación UI strings"
    },
    {
      "type": "exit_code",
      "description": "0=ok, 1=validation errors"
    }
  ],
  "capabilities": [
    "validate_catalog_schema",
    "scan_python_ui_usage",
    "detect_missing_keys",
    "detect_unused_keys",
    "validate_placeholders",
    "detect_deprecated_usage"
  ],
  "modifies_system": false,
  "runtime_ref": "python"
}
================================================================================
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, Iterable, List, Set, Tuple


DEFAULT_REQUIRED_LANGS: Set[str] = {"es", "en"}

DEFAULT_ALLOWED_PREFIXES: Tuple[str, ...] = (
    "common.",
    "dialog.",
    "menu.",
    "prefs.",
    "form.",
    "wizard.",
    "tab.",
)

DEFAULT_ALLOW_UNUSED: Set[str] = {
    "dialog.info.title",
    "dialog.warning.title",
}

DEFAULT_EXCLUDED_DIRS: Set[str] = {
    "__pycache__",
    ".git",
    ".hg",
    ".svn",
    ".mypy_cache",
    ".pytest_cache",
    ".ruff_cache",
    "venv",
    ".venv",
    "env",
    ".env",
    "build",
    "dist",
    "node_modules",
}

VALID_STATUSES: Set[str] = {
    "active",
    "deprecated",
    "draft",
    "reserved",
}

UI_CALL_PATTERNS: Tuple[re.Pattern[str], ...] = (
    re.compile(r"""\bself\.ui\(\s*['"]([a-zA-Z0-9_.]+)['"]"""),
    re.compile(r"""\bself\._ui\(\s*['"]([a-zA-Z0-9_.]+)['"]"""),
    re.compile(r"""\bui\(\s*['"]([a-zA-Z0-9_.]+)['"]"""),
    re.compile(r"""\bresolver\(\s*['"]([a-zA-Z0-9_.]+)['"]"""),
)

PLACEHOLDER_PATTERN = re.compile(r"\{([a-zA-Z_][a-zA-Z0-9_]*)\}")


@dataclass
class Report:
    errors: List[str] = field(default_factory=list)
    warnings: List[str] = field(default_factory=list)
    info: List[str] = field(default_factory=list)

    defined_keys: Set[str] = field(default_factory=set)
    used_keys: Set[str] = field(default_factory=set)
    missing_keys: Set[str] = field(default_factory=set)
    unused_keys: Set[str] = field(default_factory=set)

    deprecated_used_keys: Set[str] = field(default_factory=set)
    used_by_file: Dict[str, Set[str]] = field(default_factory=dict)

    def has_errors(self) -> bool:
        return bool(self.errors)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate ui_strings.json structure and usage in Python source files."
    )
    parser.add_argument(
        "--catalog",
        required=True,
        help="Path to ui_strings.json",
    )
    parser.add_argument(
        "--src",
        nargs="+",
        required=True,
        help="One or more Python source files or directories to scan.",
    )
    parser.add_argument(
        "--strict-unused",
        action="store_true",
        help="Treat unused catalog keys as errors instead of warnings.",
    )
    parser.add_argument(
        "--ignore-unused",
        nargs="*",
        default=[],
        help="Additional keys allowed to remain unused.",
    )
    parser.add_argument(
        "--verbose-files",
        action="store_true",
        help="Show keys detected in each file.",
    )
    return parser.parse_args()


def load_ui_strings(path: Path) -> Dict[str, Dict[str, Any]]:
    if not path.exists():
        raise FileNotFoundError(f"Catalog file not found: {path}")

    try:
        raw = path.read_text(encoding="utf-8")
    except OSError as exc:
        raise OSError(f"Could not read catalog file '{path}': {exc}") from exc

    try:
        data = json.loads(raw)
    except json.JSONDecodeError as exc:
        raise ValueError(
            f"Invalid JSON in '{path}' at line {exc.lineno}, col {exc.colno}: {exc.msg}"
        ) from exc

    if not isinstance(data, dict):
        raise TypeError("Catalog root must be a JSON object.")

    return data


def get_meta(data: Dict[str, Dict[str, Any]]) -> Dict[str, Any]:
    meta = data.get("_meta", {})
    return meta if isinstance(meta, dict) else {}


def iter_catalog_entries(data: Dict[str, Dict[str, Any]]) -> Iterable[Tuple[str, Dict[str, Any]]]:
    for key, value in data.items():
        if key == "_meta":
            continue
        yield key, value


def extract_placeholders(text: str) -> Set[str]:
    return set(PLACEHOLDER_PATTERN.findall(text))


def get_language_map(entry: Dict[str, Any]) -> Dict[str, str]:
    if not isinstance(entry, dict):
        return {}

    lang_map: Dict[str, str] = {}
    for key, value in entry.items():
        if key.startswith("_"):
            continue
        if isinstance(value, str):
            lang_map[key] = value
        else:
            lang_map[key] = value
    return lang_map


def get_required_langs(meta: Dict[str, Any]) -> Set[str]:
    raw = meta.get("required_languages")
    if isinstance(raw, list) and all(isinstance(x, str) and x.strip() for x in raw):
        return {x.strip() for x in raw}
    return set(DEFAULT_REQUIRED_LANGS)


def get_allowed_prefixes(meta: Dict[str, Any]) -> Tuple[str, ...]:
    raw = meta.get("allowed_prefixes")
    if isinstance(raw, list):
        prefixes = tuple(x for x in raw if isinstance(x, str) and x.strip())
        if prefixes:
            return prefixes
    return DEFAULT_ALLOWED_PREFIXES


def get_key_status(entry: Dict[str, Any]) -> str:
    if not isinstance(entry, dict):
        return "active"

    status = entry.get("_status", "active")
    if isinstance(status, str) and status.strip():
        return status.strip()
    return "active"


def validate_meta(meta: Dict[str, Any]) -> Tuple[List[str], List[str]]:
    errors: List[str] = []
    warnings: List[str] = []

    if not meta:
        warnings.append("Catalog does not define '_meta'. Using default validator configuration.")
        return errors, warnings

    schema_version = meta.get("schema_version")
    if schema_version is not None and not isinstance(schema_version, str):
        errors.append("Catalog '_meta.schema_version' must be a string.")

    default_language = meta.get("default_language")
    if default_language is not None and not isinstance(default_language, str):
        errors.append("Catalog '_meta.default_language' must be a string.")

    required_languages = meta.get("required_languages")
    if required_languages is not None:
        if not isinstance(required_languages, list) or not all(
            isinstance(x, str) and x.strip() for x in required_languages
        ):
            errors.append("Catalog '_meta.required_languages' must be a list of non-empty strings.")

    allowed_prefixes = meta.get("allowed_prefixes")
    if allowed_prefixes is not None:
        if not isinstance(allowed_prefixes, list) or not all(
            isinstance(x, str) and x.strip() for x in allowed_prefixes
        ):
            errors.append("Catalog '_meta.allowed_prefixes' must be a list of non-empty strings.")

    return errors, warnings


def validate_catalog_schema(
    data: Dict[str, Dict[str, Any]],
    required_langs: Set[str],
    allowed_prefixes: Tuple[str, ...],
) -> Tuple[List[str], List[str]]:
    errors: List[str] = []
    warnings: List[str] = []

    for key, value in iter_catalog_entries(data):
        if not isinstance(key, str):
            errors.append(f"Non-string catalog key detected: {key!r}")
            continue

        if not any(key.startswith(prefix) for prefix in allowed_prefixes):
            warnings.append(
                f"Key outside allowed prefixes: {key} "
                f"(allowed: {', '.join(allowed_prefixes)})"
            )

        if not isinstance(value, dict):
            errors.append(f"Key '{key}' must map to an object with language entries.")
            continue

        status = get_key_status(value)
        if status not in VALID_STATUSES:
            warnings.append(
                f"Key '{key}' has unknown _status='{status}'. "
                f"Expected one of: {', '.join(sorted(VALID_STATUSES))}"
            )

        lang_map = get_language_map(value)
        missing_langs = required_langs - set(lang_map.keys())
        if missing_langs:
            errors.append(
                f"Key '{key}' is missing required languages: {', '.join(sorted(missing_langs))}"
            )

        for lang in sorted(required_langs & set(lang_map.keys())):
            lang_value = lang_map.get(lang)

            if not isinstance(lang_value, str):
                errors.append(
                    f"Key '{key}' language '{lang}' must be a string, "
                    f"got {type(lang_value).__name__}."
                )
                continue

            if not lang_value.strip():
                errors.append(
                    f"Key '{key}' language '{lang}' is empty or whitespace only."
                )

            if lang_value != lang_value.strip():
                warnings.append(
                    f"Key '{key}' language '{lang}' has leading/trailing whitespace."
                )

    return errors, warnings


def validate_catalog_entries(
    data: Dict[str, Dict[str, Any]],
    required_langs: Set[str],
) -> Tuple[List[str], List[str]]:
    errors: List[str] = []
    warnings: List[str] = []

    for key, entry in iter_catalog_entries(data):
        if not isinstance(entry, dict):
            continue

        lang_map = get_language_map(entry)
        if not required_langs.issubset(lang_map.keys()):
            continue

        placeholder_map: Dict[str, Set[str]] = {}
        for lang in sorted(required_langs):
            text = lang_map.get(lang)
            if isinstance(text, str):
                placeholder_map[lang] = extract_placeholders(text)

        base_lang = "es" if "es" in placeholder_map else next(iter(placeholder_map), None)
        if base_lang is None:
            continue

        base_placeholders = placeholder_map[base_lang]

        for lang, placeholders in placeholder_map.items():
            if placeholders != base_placeholders:
                errors.append(
                    f"Placeholder mismatch in key '{key}': "
                    f"{base_lang}={sorted(base_placeholders)} vs "
                    f"{lang}={sorted(placeholders)}"
                )

    return errors, warnings


def iter_python_files(paths: Iterable[Path]) -> Iterable[Path]:
    for path in paths:
        if not path.exists():
            continue

        if path.is_file():
            if path.suffix == ".py":
                yield path
            continue

        for subpath in path.rglob("*.py"):
            if any(part in DEFAULT_EXCLUDED_DIRS for part in subpath.parts):
                continue
            yield subpath


def extract_ui_keys_from_source(source_text: str) -> Set[str]:
    found: Set[str] = set()
    for pattern in UI_CALL_PATTERNS:
        for match in pattern.finditer(source_text):
            found.add(match.group(1))
    return found


def scan_python_files(
    paths: Iterable[Path],
) -> Tuple[Set[str], Dict[str, Set[str]], List[str]]:
    used_keys: Set[str] = set()
    used_by_file: Dict[str, Set[str]] = {}
    warnings: List[str] = []

    for py_file in iter_python_files(paths):
        try:
            text = py_file.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            warnings.append(f"Skipped non-UTF8 file: {py_file}")
            continue
        except OSError as exc:
            warnings.append(f"Could not read source file '{py_file}': {exc}")
            continue

        file_keys = extract_ui_keys_from_source(text)
        if file_keys:
            used_by_file[str(py_file)] = file_keys
            used_keys.update(file_keys)

    return used_keys, used_by_file, warnings


def compare_defined_vs_used(
    defined_keys: Set[str],
    used_keys: Set[str],
    allow_unused: Set[str],
    data: Dict[str, Dict[str, Any]],
) -> Tuple[Set[str], Set[str]]:
    missing_keys = used_keys - defined_keys

    ignored_unused: Set[str] = set(allow_unused)
    for key in defined_keys:
        entry = data.get(key)
        if isinstance(entry, dict):
            status = get_key_status(entry)
            if status in {"draft", "reserved"}:
                ignored_unused.add(key)

    unused_keys = defined_keys - used_keys - ignored_unused
    return missing_keys, unused_keys


def detect_redundant_families(data: Dict[str, Dict[str, Any]]) -> List[str]:
    warnings: List[str] = []

    redundant_pairs = [
        ("common.status.info_prefix", "common.status.info"),
        ("common.status.warning_prefix", "common.status.warning"),
        ("common.status.error_prefix", "common.status.error"),
        (
            "common.confirm.discard_changes.message",
            "common.dialog.confirm_discard_unsaved",
        ),
        ("wizard.validation.step_named", "wizard.validation.review_step"),
        ("wizard.validation.step_review", "wizard.validation.review_step_data"),
    ]

    for a, b in redundant_pairs:
        if a in data and b in data:
            warnings.append(
                f"Potential redundant keys detected: '{a}' and '{b}'. "
                f"Consider keeping only one naming convention."
            )

    return warnings


def detect_deprecated_usage(
    data: Dict[str, Dict[str, Any]],
    used_keys: Set[str],
) -> List[str]:
    warnings: List[str] = []

    for key in sorted(used_keys):
        entry = data.get(key)
        if isinstance(entry, dict) and get_key_status(entry) == "deprecated":
            warnings.append(f"Deprecated key used in code: {key}")

    return warnings


def print_section(title: str, items: List[str]) -> None:
    print()
    print(title)
    print("-" * len(title))
    if not items:
        print("(none)")
        return
    for item in items:
        print(f"- {item}")


def print_report(
    report: Report,
    catalog_path: Path,
    strict_unused: bool,
    verbose_files: bool,
) -> None:
    print("UI STRINGS VALIDATION")
    print("=====================")
    print(f"Catalog file: {catalog_path}")
    print(f"Defined keys: {len(report.defined_keys)}")
    print(f"Used keys in code: {len(report.used_keys)}")
    print(f"Strict unused mode: {'ON' if strict_unused else 'OFF'}")

    print_section("INFO", report.info)
    print_section("ERRORS", report.errors)
    print_section("WARNINGS", report.warnings)

    if verbose_files and report.used_by_file:
        print()
        print("USED KEYS BY FILE")
        print("-----------------")
        for filename in sorted(report.used_by_file):
            keys = sorted(report.used_by_file[filename])
            print(f"{filename}: {len(keys)} key(s)")
            for key in keys:
                print(f"  - {key}")

    print()
    print("SUMMARY")
    print("-------")
    print(f"Errors: {len(report.errors)}")
    print(f"Warnings: {len(report.warnings)}")
    print(f"Missing keys: {len(report.missing_keys)}")
    print(f"Unused keys: {len(report.unused_keys)}")
    print(f"Deprecated keys used: {len(report.deprecated_used_keys)}")


def main() -> int:
    args = parse_args()

    catalog_path = Path(args.catalog).resolve()
    src_paths = [Path(p).resolve() for p in args.src]
    allow_unused = set(DEFAULT_ALLOW_UNUSED) | set(args.ignore_unused)

    report = Report()

    try:
        data = load_ui_strings(catalog_path)
    except Exception as exc:
        print(f"[FATAL] {exc}", file=sys.stderr)
        return 1

    meta = get_meta(data)
    required_langs = get_required_langs(meta)
    allowed_prefixes = get_allowed_prefixes(meta)

    meta_errors, meta_warnings = validate_meta(meta)
    report.errors.extend(meta_errors)
    report.warnings.extend(meta_warnings)

    report.defined_keys = {key for key, _value in iter_catalog_entries(data)}
    report.info.append("Catalog JSON loaded successfully.")
    report.info.append(
        f"Required languages: {', '.join(sorted(required_langs))}"
    )
    report.info.append(
        f"Allowed prefixes: {', '.join(allowed_prefixes)}"
    )
    if meta:
        report.info.append(
            f"Schema version: {meta.get('schema_version', '(not set)')}"
        )

    schema_errors, schema_warnings = validate_catalog_schema(
        data=data,
        required_langs=required_langs,
        allowed_prefixes=allowed_prefixes,
    )
    report.errors.extend(schema_errors)
    report.warnings.extend(schema_warnings)

    entry_errors, entry_warnings = validate_catalog_entries(
        data=data,
        required_langs=required_langs,
    )
    report.errors.extend(entry_errors)
    report.warnings.extend(entry_warnings)

    report.warnings.extend(detect_redundant_families(data))

    used_keys, used_by_file, scan_warnings = scan_python_files(src_paths)
    report.used_keys = used_keys
    report.used_by_file = used_by_file
    report.warnings.extend(scan_warnings)

    report.warnings.extend(detect_deprecated_usage(data, used_keys))
    report.deprecated_used_keys = {
        key
        for key in used_keys
        if isinstance(data.get(key), dict) and get_key_status(data[key]) == "deprecated"
    }

    missing_keys, unused_keys = compare_defined_vs_used(
        defined_keys=report.defined_keys,
        used_keys=report.used_keys,
        allow_unused=allow_unused,
        data=data,
    )
    report.missing_keys = missing_keys
    report.unused_keys = unused_keys

    for key in sorted(missing_keys):
        report.errors.append(f"Missing key used in code: {key}")

    for key in sorted(unused_keys):
        message = f"Unused key in catalog: {key}"
        if args.strict_unused:
            report.errors.append(message)
        else:
            report.warnings.append(message)

    print_report(
        report=report,
        catalog_path=catalog_path,
        strict_unused=args.strict_unused,
        verbose_files=args.verbose_files,
    )

    return 1 if report.has_errors() else 0


if __name__ == "__main__":
    raise SystemExit(main())
```

### START_APP.bat
- size: 745 bytes
- sha256: `2c35944a5e87fcee266fabe36cadcd6c799164f56fd2b578789dc4369e449d08`

```text
@echo off
setlocal EnableExtensions

set "ROOT=%~dp0"
set "PY=%ROOT%system\bin\runtimes\python\current\python\python.exe"
set "CLI=%ROOT%system\bin\entrypoints\app_cli.py"

REM Evitar generación de __pycache__ dentro de system/
set PYTHONDONTWRITEBYTECODE=1

echo ==========================================
echo   APP_TOOL LAUNCHER
echo ==========================================
echo ROOT: %ROOT%
echo PY:   %PY%
echo CLI:  %CLI%
echo.

if not exist "%PY%" (
  echo ERROR: No encuentro Python runtime:
  echo   "%PY%"
  echo.
  pause
  exit /b 1
)

if not exist "%CLI%" (
  echo ERROR: No encuentro entrypoint CLI:
  echo   "%CLI%"
  echo.
  pause
  exit /b 1
)

"%PY%" "%CLI%" shell

echo.
pause
endlocal
```

### system/bin/entrypoints/app_cli.py
- size: 12829 bytes
- sha256: `0a221ec61050adda3d4045a9e21838394af272983e2ac7a317dc336e816998e9`

```text
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
```

### system/bin/entrypoints/app_ui.py
- size: 595 bytes
- sha256: `ffcf3369c61a57a315e5e8ee37018e233bd274121b1760d64636164ed2bf81e6`

```text
"""
App_DocGraph UI Entry Point
===========================

Launcher for the graphical interface.

Responsibilities
----------------
- Create UI application
- Instantiate MainWindow
- Start event loop

No business logic allowed here.
"""

import sys
from pathlib import Path

from system.app.ui.app.main_window import MainWindow


def main() -> int:

    # raíz de la aplicación
    app_root = Path(__file__).resolve().parents[3]

    window = MainWindow(app_root=app_root)
    window.mainloop()

    return 0


if __name__ == "__main__":
    sys.exit(main())
```

### system/bin/entrypoints/registry_validate.py
- size: 4585 bytes
- sha256: `f0245b0884fab7086ffebd9b5538e7704af3b537350312079d9df6777d40b0f4`

```text
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
```

