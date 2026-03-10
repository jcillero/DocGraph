# APP ANALYSIS BUNDLE
- Generated at: 2026-02-28 20:15:30
- App root: `App_Tool`
- Scripts included: 5

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
│   │   │   ├── app_cli.py
│   │   │   └── registry_validate.py
│   │   ├── runtimes/ [no-walk]
│   │   └── tools/
│   │       ├── csv_to_report_pdf.py
│   │       ├── pdf_merge.py
│   │       └── tool_healthcheck_runtime.py
│   ├── config/
│   │   ├── logging/
│   │   │   ├── logging_event_types_v1.json
│   │   │   └── work_units_catalog_v1.json
│   │   └── runtime_registry.json
│   ├── core/
│   │   └── logging/
│   │       ├── logging_runtime.py
│   │       └── run_context.py
│   ├── devtools/
│   │   └── logging/
│   │       └── validate_logs.py
│   └── spec/
│       ├── docs/
│       │   ├── GUI_FRAMEWORK_INTEGRATION_GUIDE.md
│       │   └── WORKSPACE_STRUCTURE_EXPLAINED.md
│       ├── logging/
│       │   └── logging_spec_v1.md
│       └── runtime_registry_contract.json
├── user/
│   ├── input/
│   │   └── RE-PD-OBS-GESTION DE OBSOLESCENCIAS_R06.5 simple.csv
│   └── projects/
├── BASELINE_1.0.0.md
├── START_APP.bat
└── USER_MANUAL.md
```

## Scripts
> Nota: contenido incluido tal cual (UTF-8 con reemplazo en caracteres no decodificables).

### dev/scripts/reports/collect_app_bundle.py
- size: 15447 bytes
- sha256: `21d38393fb3c39218643babe5184c1110079a7b262463d71162e4a4aff0dff2f`

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
  "modifies_system": true,
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
- size: 12271 bytes
- sha256: `84d969d86e4de4c71fbbdc37ee7525d2a68dbd401292470ac50a26f76c32d743`

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

### START_APP.bat
- size: 655 bytes
- sha256: `ee2595eb1dba42efe70225c3fa9ae6b3a06b2701e9364694fe5aa00f1c637700`

```text
@echo off
setlocal EnableExtensions

set "ROOT=%~dp0"
set "PY=%ROOT%system\bin\runtimes\python\current\python\python.exe"
set "CLI=%ROOT%system\bin\entrypoints\app_cli.py"

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

