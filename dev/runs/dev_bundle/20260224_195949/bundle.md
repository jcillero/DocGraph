# APP ANALYSIS BUNDLE
- Generated at: 2026-02-24 19:59:49
- App root: `App_Tool`
- Scripts included: 4

## Folder tree (filtered)
```text
App_Tool/
├── dev/
│   └── scripts/
│       └── reports/
│           ├── collect_app_bundle.py
│           └── generate_structure_report.py
├── system/
│   ├── assets/
│   │   ├── images/
│   │   │   ├── logos/
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
│   │   │   └── app_cli.py
│   │   ├── runtimes/ [no-walk]
│   │   └── tools/
│   │       ├── csv_to_report_pdf.py
│   │       └── pdf_merge.py
│   ├── config/
│   │   └── runtime_registry.json
│   └── spec/
│       └── docs/
│           ├── GUI_FRAMEWORK_INTEGRATION_GUIDE.md
│           └── WORKSPACE_STRUCTURE_EXPLAINED.md
├── user/
│   ├── input/
│   │   └── RE-PD-OBS-GESTION DE OBSOLESCENCIAS_R06.5 simple.csv
│   └── projects/
├── START_APP.bat
└── USER_MANUAL.md
```

## Scripts
> Nota: contenido incluido tal cual (UTF-8 con reemplazo en caracteres no decodificables).

### dev/scripts/reports/collect_app_bundle.py
- size: 11656 bytes
- sha256: `64a05f1642dcc5db71d17a05bdbd6d336e0dca5b69ea0175fd2f92a2cf9bbc2d`

```text
#!/usr/bin/env python3
"""
DEV TOOL — collect_app_bundle.py

Genera un "bundle" de análisis del proyecto:
- Árbol de carpetas (con exclusiones para evitar binarios/runtimes gigantes)
- Un único fichero de texto/markdown con el contenido de todos los scripts (para análisis posterior)
- Un manifest.json con hashes y tamaños

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
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Iterable, List, Dict, Optional, Tuple


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
    "dev/runs",   # evitamos auto-incluir bundles anteriores
    ".git",
    "__pycache__",
]


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


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def detect_app_root(start: Path) -> Path:
    """
    Detecta la raíz de la app subiendo desde este script hasta encontrar system/config/runtime_registry.json
    (o, como fallback, system/).
    """
    p = start.resolve()
    for _ in range(10):
        if (p / "system" / "config" / "runtime_registry.json").exists():
            return p
        if (p / "system").exists() and (p / "dev").exists() and (p / "user").exists():
            # fallback razonable si ya existe estructura base
            return p
        p = p.parent
    # último recurso: el padre del script
    return start.resolve().parent


def iter_tree(root: Path, *, max_depth: int, include_no_walk_children: bool) -> List[str]:
    """
    Devuelve líneas estilo árbol. No profundiza en NO_WALK_REL_DIR_PREFIXES.
    """
    lines: List[str] = []
    root_name = root.name + "/"
    lines.append(root_name)

    def walk(dir_path: Path, prefix: str, depth: int) -> None:
        if depth > max_depth:
            return

        try:
            entries = sorted(dir_path.iterdir(), key=lambda x: (not x.is_dir(), x.name.lower()))
        except PermissionError:
            lines.append(prefix + "└── [perm-denied]/")
            return

        # Filtrar exclusions
        filtered: List[Path] = []
        for e in entries:
            rp = rel_posix(root, e)
            if starts_with_any_prefix(rp, EXCLUDED_REL_DIR_PREFIXES):
                continue
            filtered.append(e)

        count = len(filtered)
        for i, e in enumerate(filtered):
            is_last = (i == count - 1)
            branch = "└── " if is_last else "├── "
            next_prefix = prefix + ("    " if is_last else "│   ")

            rp = rel_posix(root, e)
            if e.is_dir():
                # No-walk folders
                if starts_with_any_prefix(rp, NO_WALK_REL_DIR_PREFIXES):
                    lines.append(prefix + branch + e.name + "/ [no-walk]")
                    if include_no_walk_children:
                        # Solo listar primer nivel (sin recursión) para indicar presencia
                        try:
                            kids = sorted(e.iterdir(), key=lambda x: (not x.is_dir(), x.name.lower()))
                            # limitamos a 20 para no hacer ruido
                            kids = kids[:20]
                            for k in kids:
                                k_branch = "└── "
                                lines.append(next_prefix + k_branch + (k.name + "/" if k.is_dir() else k.name))
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

    Excluye EXCLUDED_REL_DIR_PREFIXES y no-walk (runtimes masivos).
    """
    targets: List[Path] = [
        root / "dev" / "scripts",
        root / "system" / "bin" / "entrypoints",
    ]
    if include_system_tools:
        targets.append(root / "system" / "bin" / "tools")

    # include root launcher(s)
    launchers = [root / "START_APP.bat", root / "START_APP.cmd"]
    files: List[Path] = []
    for l in launchers:
        if l.exists():
            files.append(l)

    # Walk targets
    for base in targets:
        if not base.exists():
            continue
        for p in base.rglob("*"):
            if not p.is_file():
                continue
            rp = rel_posix(root, p)
            if starts_with_any_prefix(rp, EXCLUDED_REL_DIR_PREFIXES):
                continue
            if starts_with_any_prefix(rp, NO_WALK_REL_DIR_PREFIXES):
                continue
            if p.suffix.lower() in [e.lower() for e in script_exts]:
                files.append(p)

    # Unique by absolute
    uniq = {}
    for p in files:
        uniq[str(p.resolve())] = p
    files = sorted(uniq.values(), key=lambda x: rel_posix(root, x).lower())

    items: List[ScriptItem] = []
    for p in files:
        try:
            st = p.stat()
            items.append(
                ScriptItem(
                    rel_path=rel_posix(root, p),
                    abs_path=p.resolve(),
                    size_bytes=st.st_size,
                    sha256=sha256_file(p),
                )
            )
        except Exception:
            # si falla, omitimos silenciosamente (dev tool)
            continue
    return items


def read_text_safe(path: Path) -> str:
    # Intento razonable de lectura. No usamos chardet para evitar dependencias.
    # UTF-8 con reemplazo suele ser suficiente para scripts.
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

    args = parser.parse_args()

    app_root = detect_app_root(Path(__file__).parent)
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
        "scripts": [
            {"path": s.rel_path, "size_bytes": s.size_bytes, "sha256": s.sha256}
            for s in scripts
        ],
    }
    manifest_json.write_text(json.dumps(manifest, indent=2, ensure_ascii=False), encoding="utf-8")

    print(f"OK: {tree_txt}")
    print(f"OK: {bundle_md}")
    print(f"OK: {manifest_json}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

### dev/scripts/reports/generate_structure_report.py
- size: 16555 bytes
- sha256: `6d40b1365bdf4748a5eeda8f3ec8935cad7268dd2d940c28161b62aa382e9e51`

```text
from __future__ import annotations

import argparse
import json
import os
import re
import sys
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


"""
===============================================================================
DEV TOOL META
===============================================================================
Tool: dev_generate_structure_report
Objetivo:
  - Reporte de estructura (tree.txt / tree.json)
  - Modo extended: auditoría de arquitectura (runtime_registry, tools, runtimes) con warnings
Entrada:
  - App_Tool (raíz detectada automáticamente)
Salida:
  - user/runs/dev_reports/<timestamp>/
      tree.txt
      tree.json
      architecture_report.txt            (solo extended)
      architecture_summary.json          (solo extended)
Notas:
  - No profundiza en binarios (runtimes, motores externos, etc.)
  - Solo extrae docstring inicial de scripts para identificar propósito
===============================================================================
"""


# -----------------------------------------------------------------------------
# Configuración de walking (NO profundizar en binarios)
# -----------------------------------------------------------------------------
NO_WALK_REL_DIR_PREFIXES = [
    "system/bin/runtimes",
    "system/bin/engines",
    "system/bin/third_party",
    "system/bin/vendor",
]

# Excluir outputs/autogenerados
EXCLUDED_REL_DIR_PREFIXES = [
    "user/runs/dev_reports",
    "user/runs/csv_report",
    "user/runs/pdf_merge",
]

# Para docstrings: cuántas líneas como máximo leemos
MAX_DOCSTRING_LINES = 40


# -----------------------------------------------------------------------------
# Utilidades base
# -----------------------------------------------------------------------------
def now_stamp() -> str:
    return datetime.now().strftime("%Y%m%d_%H%M%S")


def app_root_from_this_file() -> Path:
    # dev/scripts/reports/generate_structure_report.py -> subir 3 -> App_Tool
    return Path(__file__).resolve().parents[3]


def rel_posix(root: Path, p: Path) -> str:
    return p.resolve().relative_to(root.resolve()).as_posix()


def starts_with_any_prefix(rel_posix_path: str, prefixes: List[str]) -> bool:
    return any(rel_posix_path == pref or rel_posix_path.startswith(pref + "/") for pref in prefixes)


def safe_read_text(path: Path, encoding: str = "utf-8") -> str:
    try:
        return path.read_text(encoding=encoding)
    except UnicodeDecodeError:
        # fallback típico en Windows
        return path.read_text(encoding="utf-8", errors="replace")


# -----------------------------------------------------------------------------
# Docstring extractor (solo descripción breve)
# -----------------------------------------------------------------------------
_DOCSTRING_RE = re.compile(r'^[ \t]*([ruRU]{0,2}["\']{3})(.*)$')


def extract_top_docstring(script_path: Path) -> str:
    """
    Extrae el docstring inicial (triple comilla) si existe.
    Solo lee un número limitado de líneas.
    """
    try:
        text = safe_read_text(script_path)
    except Exception:
        return ""

    lines = text.splitlines()
    if not lines:
        return ""

    # Saltar shebang/encoding comments
    i = 0
    while i < len(lines) and (lines[i].startswith("#!") or "coding" in lines[i]):
        i += 1
    if i >= len(lines):
        return ""

    m = _DOCSTRING_RE.match(lines[i])
    if not m:
        return ""

    quote = m.group(1)
    # reconstruimos hasta cerrar las triples
    doc_lines = []
    # línea actual sin el prefijo de comillas
    first = lines[i]
    # cortamos desde la primera ocurrencia de """ o '''
    q = '"""' if '"""' in first else "'''"
    after = first.split(q, 1)[1]
    # si cierra en la misma línea
    if q in after:
        inside = after.split(q, 1)[0]
        return inside.strip()

    doc_lines.append(after)
    i += 1
    count = 1
    while i < len(lines) and count < MAX_DOCSTRING_LINES:
        ln = lines[i]
        if q in ln:
            doc_lines.append(ln.split(q, 1)[0])
            break
        doc_lines.append(ln)
        i += 1
        count += 1

    doc = "\n".join(doc_lines).strip()
    return doc


# -----------------------------------------------------------------------------
# Tree builder
# -----------------------------------------------------------------------------
@dataclass
class TreeNode:
    name: str
    rel: str
    kind: str  # "dir" | "file"
    note: str = ""
    children: Optional[List["TreeNode"]] = None


def build_tree(root: Path, current: Path) -> Optional[TreeNode]:
    rel = rel_posix(root, current)

    # Exclusiones
    if rel != "." and starts_with_any_prefix(rel, EXCLUDED_REL_DIR_PREFIXES):
        return TreeNode(name=current.name, rel=rel, kind="dir", note="[excluded]", children=[])

    if current.is_dir():
        # No-walk en binarios
        if rel != "." and starts_with_any_prefix(rel, NO_WALK_REL_DIR_PREFIXES):
            return TreeNode(name=current.name, rel=rel, kind="dir", note="[no-walk]", children=[])

        children: List[TreeNode] = []
        try:
            entries = sorted(current.iterdir(), key=lambda p: (not p.is_dir(), p.name.lower()))
        except Exception:
            return TreeNode(name=current.name, rel=rel, kind="dir", note="[unreadable]", children=[])

        for e in entries:
            node = build_tree(root, e)
            if node is not None:
                children.append(node)

        return TreeNode(name=current.name, rel=rel, kind="dir", children=children)

    # file
    note = ""
    if current.suffix.lower() == ".py":
        doc = extract_top_docstring(current)
        if doc:
            # primera línea
            first_line = doc.splitlines()[0].strip()
            note = f"[doc: {first_line[:120]}]"
    return TreeNode(name=current.name, rel=rel, kind="file", note=note, children=None)


def tree_to_dict(node: TreeNode) -> Dict[str, Any]:
    out = {
        "name": node.name,
        "rel": node.rel,
        "kind": node.kind,
    }
    if node.note:
        out["note"] = node.note
    if node.children is not None:
        out["children"] = [tree_to_dict(c) for c in node.children]
    return out


def tree_to_text(node: TreeNode, indent: str = "") -> str:
    line = f"{indent}{node.name}"
    if node.note:
        line += f"  {node.note}"
    lines = [line]
    if node.children is not None:
        for c in node.children:
            lines.append(tree_to_text(c, indent + "  "))
    return "\n".join(lines)


# -----------------------------------------------------------------------------
# Auditoría (extended)
# -----------------------------------------------------------------------------
@dataclass
class WarningItem:
    code: str
    message: str
    ref: str = ""


def load_json_if_exists(path: Path) -> Optional[dict]:
    if not path.exists():
        return None
    try:
        return json.loads(safe_read_text(path))
    except Exception:
        return None


def audit_registry(root: Path) -> Tuple[Dict[str, Any], List[WarningItem]]:
    warnings: List[WarningItem] = []
    registry_path = root / "system" / "config" / "runtime_registry.json"
    reg = load_json_if_exists(registry_path)

    summary: Dict[str, Any] = {
        "registry_path": str(registry_path),
        "registry_loaded": bool(reg),
        "runtimes_declared": 0,
        "tools_declared": 0,
        "presets_total": 0,
        "tools_missing_label": [],
        "tools_missing_presets": [],
        "tools_entry_missing": [],
        "runtimes_unused": [],
        "runtime_executable_missing": [],
        "unregistered_tool_scripts": [],
    }

    if not reg:
        warnings.append(WarningItem("REGISTRY_MISSING_OR_INVALID", "No se pudo cargar runtime_registry.json", ref=str(registry_path)))


    # ---------------------------------------------------------------------
    # Check: .py.txt accidental (Windows extensions hidden)
    # ---------------------------------------------------------------------
    for p in root.rglob("*.py.txt"):
        rel = rel_posix(root, p)
        warnings.append(
            WarningItem(
                "PY_TXT_DETECTED",
                "Archivo .py.txt detectado. Probable extensión oculta en Windows.",
                ref=rel,
            )
        )

    # ---------------------------------------------------------------------
    # Check: duplicate script basenames
    # ---------------------------------------------------------------------
    from collections import defaultdict

    script_map = defaultdict(list)
    for p in root.rglob("*.py"):
        rp = rel_posix(root, p)

        # Evitar ruido y no-walk
        if starts_with_any_prefix(rp, NO_WALK_REL_DIR_PREFIXES):
            continue
        if "/site-packages/" in rp or rp.startswith("system/bin/runtimes/"):
            continue

        script_map[p.name].append(rp)

    for name, rels in script_map.items():
        if len(rels) > 1:
            warnings.append(
                WarningItem(
                    "DUPLICATE_SCRIPT_NAME",
                    f"Mismo nombre de script detectado en múltiples ubicaciones: {name}",
                    ref=" | ".join(rels),
                )
            )

    runtimes = reg.get("runtimes", {})
    tools = reg.get("tools", {})
    summary["runtimes_declared"] = len(runtimes)
    summary["tools_declared"] = len(tools)

    # uso de runtimes
    runtime_used: Dict[str, int] = {k: 0 for k in runtimes.keys()}

    # Validaciones de runtimes
    for rid, r in runtimes.items():
        exe_rel = r.get("console_executable")
        if not exe_rel:
            warnings.append(WarningItem("RUNTIME_NO_CONSOLE_EXE", f"Runtime '{rid}' no define console_executable", ref=f"runtimes.{rid}"))
            continue
        exe_path = root / str(exe_rel)
        if not exe_path.exists():
            warnings.append(WarningItem("RUNTIME_EXE_MISSING", f"Ejecutable no existe para runtime '{rid}'", ref=str(exe_path)))
            summary["runtime_executable_missing"].append({"runtime": rid, "path": str(exe_path)})

    # Validaciones de tools
    for tid, t in tools.items():
        if not t.get("label"):
            warnings.append(WarningItem("TOOL_MISSING_LABEL", f"Tool '{tid}' no tiene label (se mostrará el id)", ref=f"tools.{tid}"))
            summary["tools_missing_label"].append(tid)

        runtime_ref = t.get("runtime_ref")
        if runtime_ref in runtime_used:
            runtime_used[runtime_ref] += 1
        else:
            warnings.append(WarningItem("TOOL_RUNTIME_REF_INVALID", f"Tool '{tid}' referencia runtime inexistente '{runtime_ref}'", ref=f"tools.{tid}.runtime_ref"))

        entry_rel = t.get("entry")
        if not entry_rel:
            warnings.append(WarningItem("TOOL_MISSING_ENTRY", f"Tool '{tid}' no define entry", ref=f"tools.{tid}"))
        else:
            entry_path = root / str(entry_rel)
            if not entry_path.exists():
                warnings.append(WarningItem("TOOL_ENTRY_MISSING", f"Entry no existe para tool '{tid}'", ref=str(entry_path)))
                summary["tools_entry_missing"].append({"tool": tid, "path": str(entry_path)})

        presets = t.get("presets")
        if not isinstance(presets, list) or len(presets) == 0:
            warnings.append(WarningItem("TOOL_NO_PRESETS", f"Tool '{tid}' no tiene presets (menú queda pobre)", ref=f"tools.{tid}.presets"))
            summary["tools_missing_presets"].append(tid)
        else:
            summary["presets_total"] += len(presets)
            for i, p in enumerate(presets, 1):
                if not isinstance(p, dict) or not p.get("label"):
                    warnings.append(WarningItem("PRESET_MISSING_LABEL", f"Tool '{tid}' preset #{i} sin label", ref=f"tools.{tid}.presets[{i}]"))

    # runtimes sin uso
    for rid, count in runtime_used.items():
        if count == 0:
            warnings.append(WarningItem("RUNTIME_UNUSED", f"Runtime '{rid}' declarado pero no usado por ninguna tool", ref=f"runtimes.{rid}"))
            summary["runtimes_unused"].append(rid)

    # scripts físicos en system/bin/tools que no estén registrados
    tools_dir = root / "system" / "bin" / "tools"
    if tools_dir.exists():
        physical = sorted(p for p in tools_dir.glob("*.py") if p.is_file())
        registered_entries = set(str((root / str(t.get("entry", ""))).resolve()) for t in tools.values() if t.get("entry"))
        for p in physical:
            if str(p.resolve()) not in registered_entries:
                rel = rel_posix(root, p)
                warnings.append(WarningItem("UNREGISTERED_TOOL_SCRIPT", "Script en system/bin/tools no registrado en runtime_registry.json", ref=rel))
                summary["unregistered_tool_scripts"].append(rel)

    return summary, warnings


def format_warnings_text(warnings: List[WarningItem]) -> str:
    if not warnings:
        return "Sin warnings.\n"

    lines = []
    lines.append(f"WARNINGS: {len(warnings)}")
    lines.append("-" * 80)
    for w in warnings:
        ref = f" [{w.ref}]" if w.ref else ""
        lines.append(f"- {w.code}: {w.message}{ref}")
    lines.append("")
    return "\n".join(lines)


def build_architecture_report(root: Path) -> Tuple[str, Dict[str, Any]]:
    summary, warnings = audit_registry(root)

    report_lines = []
    report_lines.append("ARCHITECTURE REPORT (extended)")
    report_lines.append("=" * 80)
    report_lines.append(f"App root: {root}")
    report_lines.append(f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    report_lines.append("")
    report_lines.append("SUMMARY")
    report_lines.append("-" * 80)
    report_lines.append(json.dumps(summary, ensure_ascii=False, indent=2))
    report_lines.append("")
    report_lines.append(format_warnings_text(warnings))

    report_text = "\n".join(report_lines)

    out_summary = {
        "summary": summary,
        "warnings": [w.__dict__ for w in warnings],
    }
    return report_text, out_summary



# -----------------------------------------------------------------------------
# Main
# -----------------------------------------------------------------------------
def main() -> int:
    parser = argparse.ArgumentParser(description="Developer Structure Report (basic/extended)")
    parser.add_argument("--print-root", action="store_true", help="Imprime la raíz detectada y sale.")
    parser.add_argument("--mode", choices=["basic", "extended"], default="basic", help="Modo de reporte.")
    parser.add_argument("--max-depth", type=int, default=50, help="Profundidad máxima de impresión en tree.txt (visual).")
    args = parser.parse_args()

    root = app_root_from_this_file()

    if args.print_root:
        print(root)
        return 0

    # output
    out_dir = root / "user" / "runs" / "dev_reports" / now_stamp()
    out_dir.mkdir(parents=True, exist_ok=True)

    # tree
    node = build_tree(root, root)
    if not node:
        print("ERROR: no se pudo construir árbol.")
        return 1

    tree_txt = tree_to_text(node)
    (out_dir / "tree.txt").write_text(tree_txt, encoding="utf-8")
    (out_dir / "tree.json").write_text(json.dumps(tree_to_dict(node), ensure_ascii=False, indent=2), encoding="utf-8")

    print(f"OK: {out_dir / 'tree.txt'}")
    print(f"OK: {out_dir / 'tree.json'}")

    # extended auditoría
    if args.mode == "extended":
        rep_txt, rep_json = build_architecture_report(root)
        (out_dir / "architecture_report.txt").write_text(rep_txt, encoding="utf-8")
        (out_dir / "architecture_summary.json").write_text(json.dumps(rep_json, ensure_ascii=False, indent=2), encoding="utf-8")
        print(f"OK: {out_dir / 'architecture_report.txt'}")
        print(f"OK: {out_dir / 'architecture_summary.json'}")

        # avisar en consola si hay warnings
        n_warn = len(rep_json.get("warnings", []))
        if n_warn:
            print(f"\nWARNINGS: {n_warn} (ver architecture_report.txt)")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

### START_APP.bat
- size: 309 bytes
- sha256: `7080780ecc03e9bc1d7da5900a0441c0ffb8efa34a7d6fe935b32ddceed9a49b`

```text
@echo off
setlocal
set ROOT=%~dp0

set PY=%ROOT%system\bin\runtimes\python\WPy64-312101\python\python.exe
set CLI=%ROOT%system\bin\entrypoints\app_cli.py

REM Si no se pasan argumentos, lanzar shell
if "%~1"=="" (
    "%PY%" "%CLI%" shell
) else (
    "%PY%" "%CLI%" %*
)

echo.
pause
endlocal
```

### system/bin/entrypoints/app_cli.py
- size: 8398 bytes
- sha256: `0d018bf344872a51f51aa665add3bfb857f9ba487846ddfa16d91554dbeec0ec`

```text
from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any


# -----------------------------
# Core wiring
# -----------------------------
def get_app_root() -> Path:
    # system/bin/entrypoints/app_cli.py -> App_Tool (subir 3)
    return Path(__file__).resolve().parents[3]


def load_registry(app_root: Path) -> dict:
    registry_path = app_root / "system" / "config" / "runtime_registry.json"
    return json.loads(registry_path.read_text(encoding="utf-8"))


def build_env(runtime: dict) -> dict:
    env = os.environ.copy()
    for k, v in runtime.get("env", {}).items():
        env[str(k)] = str(v)
    return env


def run_tool(app_root: Path, reg: dict, tool_id: str, pass_args: list[str]) -> int:
    tool = reg.get("tools", {}).get(tool_id)
    if not tool:
        print(f"Tool no encontrada: {tool_id}")
        return 2

    runtime = reg.get("runtimes", {}).get(tool["runtime_ref"])
    if not runtime:
        print(f"Runtime no encontrado: {tool.get('runtime_ref')}")
        return 2

    py_exe = app_root / runtime["console_executable"]
    entry = app_root / tool["entry"]

    if not py_exe.exists():
        print(f"No existe el ejecutable del runtime: {py_exe}")
        return 1
    if not entry.exists():
        print(f"No existe el script del tool: {entry}")
        return 1

    env = build_env(runtime)
    return subprocess.call([str(py_exe), str(entry), *pass_args], env=env)


# -----------------------------
# UI helpers (compact shell)
# -----------------------------
def clear_screen() -> None:
    os.system("cls" if os.name == "nt" else "clear")


def banner(title: str, subtitle: str = "") -> str:
    top = "╔" + "═" * 46 + "╗"
    mid = f"║ {title.center(44)} ║"
    bot = "╚" + "═" * 46 + "╝"
    if subtitle:
        return f"{top}\n{mid}\n{bot}\n{subtitle}\n"
    return f"{top}\n{mid}\n{bot}\n"


def tool_display_name(tool_id: str, tool: dict) -> str:
    # Preferimos un nombre bonito; fallback al id
    return str(tool.get("label") or tool_id)


def preset_list(tool: dict) -> list[dict[str, Any]]:
    presets = tool.get("presets")
    out: list[dict[str, Any]] = []
    if isinstance(presets, list) and presets:
        for p in presets:
            if not isinstance(p, dict):
                continue
            label = str(p.get("label", "Preset"))
            args = p.get("args", [])
            if not isinstance(args, list):
                args = []
            out.append({"label": label, "args": [str(a) for a in args]})
        if out:
            return out
    return [{"label": "Ejecutar", "args": []}]


def build_compact_menu(reg: dict) -> tuple[list[dict[str, Any]], list[str]]:
    """
    Devuelve:
      - items: lista con {key, tool_id, tool_name, preset_label, args, category}
      - lines: representación de líneas para imprimir
    """
    tools = reg.get("tools", {})
    tool_ids = sorted(tools.keys())

    items: list[dict[str, Any]] = []
    lines: list[str] = []

    # Agrupamos por categoría si existe (DEV/PRODUCT/etc.), pero imprimimos compacto
    # Orden fijo deseable:
    cat_order = ["dev", "product", "system", "other"]
    buckets: dict[str, list[str]] = {c: [] for c in cat_order}
    extra: dict[str, list[str]] = {}

    for tid in tool_ids:
        cat = str(tools[tid].get("category", "other")).lower()
        if cat in buckets:
            buckets[cat].append(tid)
        else:
            extra.setdefault(cat, []).append(tid)

    ordered_cats = [c for c in cat_order if buckets[c]] + sorted(extra.keys())

    tool_index = 0
    for cat in ordered_cats:
        tids = buckets.get(cat, []) if cat in buckets else extra.get(cat, [])
        if not tids:
            continue

        # Encabezado de categoría
        cat_label = cat.upper()
        lines.append(f"[{cat_label}]")

        for tid in tids:
            tool_index += 1
            tool = tools[tid]
            tname = tool_display_name(tid, tool)
            presets = preset_list(tool)

            for pi, p in enumerate(presets, 1):
                key = f"{tool_index}.{pi}"
                items.append(
                    {
                        "key": key,
                        "tool_id": tid,
                        "tool_name": tname,
                        "preset_label": p["label"],
                        "args": p["args"],
                        "category": cat,
                    }
                )
                # Línea compacta: "2.1  CSV → Informe PDF — Ejecutar"
                # Usamos em dash si soporta; si no, queda bien igualmente.
                lines.append(f" {key:<4} {tname} — {p['label']}")
        lines.append("")  # línea en blanco entre categorías

    return items, lines


def help_text() -> str:
    return "\n".join(
        [
            "Comandos:",
            "  X.Y   Ejecutar preset (ej: 2.1)",
            "  r     Refrescar (releer runtime_registry.json)",
            "  h     Ayuda",
            "  q     Salir",
            "",
            "Notas:",
            "  - Los presets se definen en system/config/runtime_registry.json (tools -> presets).",
            "  - Si una tool no tiene presets, aparece como 'Ejecutar'.",
        ]
    )


def shell_mode(app_root: Path) -> int:
    while True:
        reg = load_registry(app_root)
        items, lines = build_compact_menu(reg)

        clear_screen()
        total_tools = len(reg.get("tools", {}))
        print(banner("APP_TOOL — CONSOLA ICOFER DE TOOLS", f"Tools registradas: {total_tools}\n"))
        print("\n".join(lines))
        print("──────────────────────────────────────────────")
        choice = input("Selecciona (X.Y) │ r refrescar │ h ayuda │ q salir: ").strip().lower()

        if choice in {"q", "quit", "exit"}:
            return 0
        if choice == "r":
            continue
        if choice == "h":
            clear_screen()
            print(banner("AYUDA", ""))
            print(help_text())
            input("\nEnter para volver…")
            continue

        match = next((it for it in items if it["key"] == choice), None)
        if not match:
            print("Entrada inválida. Usa X.Y (ej: 2.1), o 'h'.")
            input("Enter para continuar…")
            continue

        clear_screen()
        print(banner("EJECUCIÓN", ""))
        print(f">> {match['key']}  {match['tool_name']} — {match['preset_label']}")
        if match["args"]:
            print(f"   args: {' '.join(match['args'])}")
        else:
            print("   args: (none)")
        print("")

        rc = run_tool(app_root, reg, match["tool_id"], match["args"])

        print(f"\n(Exit code: {rc})")
        input("Enter para volver al menú…")


# -----------------------------
# CLI commands (list/run/shell)
# -----------------------------
def main() -> int:
    if len(sys.argv) < 2:
        print("Uso:")
        print("  app_cli.py list")
        print("  app_cli.py run <tool_id> [-- tool_args...]")
        print("  app_cli.py shell")
        return 2

    cmd = sys.argv[1].lower()
    app_root = get_app_root()
    reg = load_registry(app_root)

    if cmd == "list":
        tools = reg.get("tools", {})
        print("Tools registradas:")
        for tool_id in sorted(tools.keys()):
            desc = tools[tool_id].get("description", "")
            print(f"  - {tool_id}" + (f"  | {desc}" if desc else ""))
        return 0

    if cmd == "shell":
        return shell_mode(app_root)

    if cmd == "run":
        if len(sys.argv) < 3:
            print("Uso: app_cli.py run <tool_id> [-- tool_args...]")
            return 2
        tool_id = sys.argv[2]

        pass_args: list[str] = []
        if "--" in sys.argv:
            idx = sys.argv.index("--")
            pass_args = sys.argv[idx + 1 :]

        return run_tool(app_root, reg, tool_id, pass_args)

    print("Comando inválido. Usa: list | run | shell")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
```

