# APP ANALYSIS BUNDLE
- Generated at: 2026-02-28 19:51:44
- App root: `App_Tool`
- Scripts included: 8

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
│   │   └── runtime_registry.json
│   └── spec/
│       ├── docs/
│       │   ├── GUI_FRAMEWORK_INTEGRATION_GUIDE.md
│       │   └── WORKSPACE_STRUCTURE_EXPLAINED.md
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

### system/bin/tools/csv_to_report_pdf.py
- size: 12279 bytes
- sha256: `f054c46c3c89a94e6d33726d55e0099c6c3c5fa0341ac73303861d10ca506104`

```text
"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "csv_to_report_pdf.py",
  "version": "1.0.0",
  "type": "product_tool",
  "category": "product",
  "description": "Genera informe PDF a partir de CSV estructurado.",
  "location_expected": "system/bin/tools/",
  "runtime_required": true,
  "modifies_system": false,
  "output_location": "user/output/",
  "outputs": ["report.pdf"],
  "depends_on": ["reportlab"],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "MANUAL DE TIPOS DE DOCUMENTO"
  ]
}
================================================================================
"""

from __future__ import annotations

import csv
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple

from reportlab.lib import colors
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.lib.units import mm
from reportlab.platypus import (
    PageBreak,
    Paragraph,
    SimpleDocTemplate,
    Spacer,
    Table,
    TableStyle,
)

"""
===============================================================================
TOOL META
===============================================================================
Tool: csv_to_report_pdf
Objetivo: Leer un CSV con cabecera fija (líneas superiores) y generar un informe PDF:
  - Portada con resumen
  - Una página por fila (registro)
Entrada: user/input/*.csv
Salida: user/output/*.pdf y registro en user/runs/csv_report/<timestamp>/
Notas:
  - Detecta automáticamente la fila de cabeceras real (por ejemplo: 'ID;FECHA;CÓDIGO;...')
  - Usa separador ';' y respeta comillas (campos con saltos de línea).
  - Wrapping real en tablas usando Paragraph.
  - Control dinámico de tamaño de letra en campos largos (ej. DESCRIPCIÓN).
===============================================================================
"""

# -----------------------------
# Page numbering (Página X de Y)
# -----------------------------
from reportlab.pdfgen import canvas as rl_canvas
from reportlab.pdfbase.pdfmetrics import stringWidth
from reportlab.lib.pagesizes import A4
from reportlab.lib import colors


class NumberedCanvas(rl_canvas.Canvas):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._saved_page_states = []

    def showPage(self):
        # Guardar estado de la página actual
        self._saved_page_states.append(dict(self.__dict__))
        self._startPage()  # ⚠️ NO llamar a super().showPage()

    def save(self):
        total_pages = len(self._saved_page_states)

        for state in self._saved_page_states:
            self.__dict__.update(state)
            self.draw_page_number(total_pages)
            rl_canvas.Canvas.showPage(self)

        rl_canvas.Canvas.save(self)

    def draw_page_number(self, total_pages):
        page_num = self._pageNumber
        text = f"Página {page_num} de {total_pages}"

        page_width, _ = A4
        font_name = "Helvetica"
        font_size = 9

        self.setFillColor(colors.black)
        self.setFont(font_name, font_size)

        text_width = stringWidth(text, font_name, font_size)
        x = (page_width - text_width) / 2
        y = 12

        self.drawString(x, y, text)


# -----------------------------
# Paths / util
# -----------------------------
def app_root() -> Path:
    # system/bin/tools/csv_to_report_pdf.py -> App_Tool
    return Path(__file__).resolve().parents[3]


def now_stamp() -> str:
    return datetime.now().strftime("%Y%m%d_%H%M%S")


def ensure_dirs(root: Path) -> Tuple[Path, Path, Path]:
    input_dir = root / "user" / "input"
    output_dir = root / "user" / "output"
    runs_dir = root / "user" / "runs" / "csv_report"
    input_dir.mkdir(parents=True, exist_ok=True)
    output_dir.mkdir(parents=True, exist_ok=True)
    runs_dir.mkdir(parents=True, exist_ok=True)
    return input_dir, output_dir, runs_dir


def pick_csv_interactive(input_dir: Path) -> Optional[Path]:
    files = sorted(input_dir.glob("*.csv"))
    if not files:
        print("No hay CSVs en user/input/")
        return None

    print("CSVs disponibles en user/input/:")
    for i, f in enumerate(files, 1):
        print(f"  {i}. {f.name}")

    s = input("Selecciona número: ").strip()
    if not s.isdigit():
        print("Selección inválida.")
        return None
    idx = int(s)
    if idx < 1 or idx > len(files):
        print("Fuera de rango.")
        return None
    return files[idx - 1]


# -----------------------------
# CSV parsing (cabecera fija)
# -----------------------------
def _first_cell(line: str, delim: str = ";") -> str:
    return line.split(delim, 1)[0].strip().strip('"').strip()


def detect_header_line_index(lines: List[str]) -> int:
    """
    Encuentra la línea que contiene la cabecera real.
    En tu formato, suele empezar por 'ID;FECHA;CÓDIGO;...'
    """
    for i, ln in enumerate(lines):
        if _first_cell(ln).upper() == "ID":
            return i

    for i, ln in enumerate(lines):
        up = ln.upper()
        if ln.count(";") >= 5 and "ID" in up and "FECHA" in up:
            return i

    raise ValueError("No se pudo detectar la fila de cabecera (ID;FECHA;...).")


def read_csv_records(path: Path) -> Tuple[List[str], List[Dict[str, str]]]:
    """
    Lee el archivo completo y parsea desde la cabecera real.
    Usa ';' como separador y soporta comillas + saltos de línea dentro de campos.
    """
    raw = path.read_text(encoding="utf-8-sig", errors="replace")
    lines = raw.splitlines()
    header_idx = detect_header_line_index(lines)

    tail_text = "\n".join(lines[header_idx:])

    reader = csv.DictReader(
        tail_text.splitlines(True),
        delimiter=";",
        quotechar='"',
        skipinitialspace=True,
    )

    fieldnames = reader.fieldnames or []
    records: List[Dict[str, str]] = []
    for row in reader:
        records.append({k: (v if v is not None else "") for k, v in row.items()})

    return fieldnames, records


# -----------------------------
# PDF rendering (ReportLab)
# -----------------------------
def _p(text: str, styles, base_size: int = 9) -> Paragraph:
    """
    Paragraph con wrapping + control dinámico de tamaño.
    Reduce tamaño automáticamente si el texto es muy largo.
    """
    if text is None:
        text = ""

    text = str(text).replace("\r\n", "\n").replace("\r", "\n")
    text = text.replace("\n", "<br/>")

    length = len(text)
    if length > 2000:
        size = base_size - 2
    elif length > 1000:
        size = base_size - 1
    else:
        size = base_size

    style = styles["BodyText"].clone("dynamic")
    style.fontSize = max(7, size)
    style.leading = style.fontSize + 2

    return Paragraph(text if text.strip() else "&nbsp;", style)


def _kv_table(data: List[Tuple[str, str]], styles) -> Table:
    """
    Tabla Key/Value con wrapping real (valores como Paragraph)
    y control de tamaño por campo.
    """
    rows = [[_p("<b>Campo</b>", styles, 9), _p("<b>Valor</b>", styles, 9)]]

    for k, v in data:
        k_up = k.upper().strip()
        if k_up in {"DESCRIPCIÓN", "DESCRIPCION", "OBSERVACIÓN", "OBSERVACION", "COMENTARIOS"}:
            # Campos típicamente largos -> base más compacta
            rows.append([_p(f"<b>{k}</b>", styles, 9), _p(v, styles, 8)])
        else:
            rows.append([_p(f"<b>{k}</b>", styles, 9), _p(v, styles, 9)])

    table = Table(
        rows,
        colWidths=[55 * mm, 125 * mm],
        repeatRows=1,
    )
    table.setStyle(
        TableStyle(
            [
                ("BACKGROUND", (0, 0), (-1, 0), colors.lightgrey),
                ("GRID", (0, 0), (-1, -1), 0.25, colors.grey),
                ("VALIGN", (0, 0), (-1, -1), "TOP"),
                ("LEFTPADDING", (0, 0), (-1, -1), 6),
                ("RIGHTPADDING", (0, 0), (-1, -1), 6),
                ("TOPPADDING", (0, 0), (-1, -1), 4),
                ("BOTTOMPADDING", (0, 0), (-1, -1), 4),
            ]
        )
    )
    return table


def build_pdf(
    out_path: Path,
    source_csv: Path,
    fieldnames: List[str],
    records: List[Dict[str, str]],
) -> None:
    styles = getSampleStyleSheet()
    story = []

    # --- Portada ---
    story.append(Paragraph("<b>INFORME DE REGISTROS (CSV → PDF)</b>", styles["Title"]))
    story.append(Spacer(1, 6 * mm))

    story.append(_p(f"<b>Fuente:</b> {source_csv.name}", styles, 10))
    story.append(_p(f"<b>Generado:</b> {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}", styles, 10))
    story.append(_p(f"<b>Nº de registros:</b> {len(records)}", styles, 10))
    story.append(_p(f"<b>Nº de campos:</b> {len(fieldnames)}", styles, 10))
    story.append(Spacer(1, 6 * mm))

    preview_cols = fieldnames[:8]
    if preview_cols:
        story.append(_p("<b>Campos detectados (vista previa):</b>", styles, 10))
        story.append(_p(", ".join(preview_cols) + ("…" if len(fieldnames) > len(preview_cols) else ""), styles, 10))
        story.append(Spacer(1, 6 * mm))

    # Resumen rápido por ORIGEN/EQUIPO si existen
    for key in ["ORIGEN", "EQUIPO"]:
        if key in fieldnames:
            counts: Dict[str, int] = {}
            for r in records:
                val = (r.get(key) or "").strip() or "(vacío)"
                counts[val] = counts.get(val, 0) + 1
            top = sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))[:8]
            story.append(_p(f"<b>Resumen por {key} (top 8):</b>", styles, 10))
            story.append(_p("; ".join([f"{k}={v}" for k, v in top]), styles, 10))
            story.append(Spacer(1, 4 * mm))

    story.append(PageBreak())

    # --- Páginas por registro ---
    for idx, row in enumerate(records, 1):
        story.append(Paragraph(f"<b>Registro {idx} / {len(records)}</b>", styles["Heading2"]))
        story.append(Spacer(1, 3 * mm))

        kv: List[Tuple[str, str]] = []
        for k in fieldnames:
            v = (row.get(k) or "").strip()
            kv.append((k, v))

        story.append(_kv_table(kv, styles))

        if idx != len(records):
            story.append(PageBreak())

    doc = SimpleDocTemplate(
        str(out_path),
        pagesize=A4,
        leftMargin=15 * mm,
        rightMargin=15 * mm,
        topMargin=15 * mm,
        bottomMargin=15 * mm,
        title="CSV Report",
    )
    doc.build(story,canvasmaker=NumberedCanvas)


def write_run_log(runs_dir: Path, csv_path: Path, out_path: Path, n: int) -> None:
    run_dir = runs_dir / now_stamp()
    run_dir.mkdir(parents=True, exist_ok=True)
    (run_dir / "run.txt").write_text(
        "\n".join(
            [
                "CSV_TO_REPORT_PDF RUN",
                f"CSV: {csv_path}",
                f"OUTPUT: {out_path}",
                f"RECORDS: {n}",
                f"TIMESTAMP: {datetime.now().isoformat()}",
            ]
        ),
        encoding="utf-8",
    )


def main() -> int:
    root = app_root()
    input_dir, output_dir, runs_dir = ensure_dirs(root)

    csv_path = pick_csv_interactive(input_dir)
    if not csv_path:
        return 1

    try:
        fieldnames, records = read_csv_records(csv_path)
    except Exception as e:
        print(f"ERROR leyendo CSV: {e}")
        return 1

    if not fieldnames:
        print("ERROR: CSV sin cabeceras detectables.")
        return 1

    out_name = f"csv_report_{csv_path.stem}_{now_stamp()}.pdf"
    out_path = output_dir / out_name

    try:
        build_pdf(out_path, csv_path, fieldnames, records)
    except Exception as e:
        print(f"ERROR generando PDF: {e}")
        return 1

    write_run_log(runs_dir, csv_path, out_path, len(records))

    print(f"OK -> {out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

### system/bin/tools/pdf_merge.py
- size: 4364 bytes
- sha256: `50bfdc25fe1089e6c823f7341aebda970b63f06b5077d72bda384bc9bc055dcf`

```text
"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "pdf_merge.py",
  "version": "1.0.0",
  "type": "product_tool",
  "category": "product",
  "description": "Fusiona múltiples PDFs en un único documento.",
  "location_expected": "system/bin/tools/",
  "runtime_required": true,
  "modifies_system": false,
  "output_location": "user/output/",
  "outputs": ["merged.pdf"],
  "depends_on": ["pypdf"],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "MANUAL DE TIPOS DE DOCUMENTO"
  ]
}
================================================================================
"""

from __future__ import annotations

import sys
import shutil
from datetime import datetime
from pathlib import Path
from typing import List

try:
    from pypdf import PdfReader, PdfWriter
except ImportError:
    print("ERROR: pypdf no está instalado en el runtime.")
    sys.exit(1)


# ============================
# Utilidades base
# ============================

def app_root() -> Path:
    # system/bin/tools/pdf_merge.py -> subir 3 niveles
    return Path(__file__).resolve().parents[3]


def now_stamp() -> str:
    return datetime.now().strftime("%Y%m%d_%H%M%S")


def list_pdfs(input_dir: Path) -> List[Path]:
    return sorted(input_dir.glob("*.pdf"))


def print_list(files: List[Path]):
    print("\nArchivos seleccionados:")
    for i, f in enumerate(files, 1):
        print(f"  {i}. {f.name}")
    print()


def reorder_interactive(files: List[Path]) -> List[Path]:
    while True:
        print_list(files)
        print("Comandos: swap i j | move i j | rm i | done")
        cmd = input(">> ").strip().lower()

        if cmd == "done":
            return files

        parts = cmd.split()
        try:
            if parts[0] == "swap" and len(parts) == 3:
                i, j = int(parts[1]) - 1, int(parts[2]) - 1
                files[i], files[j] = files[j], files[i]

            elif parts[0] == "move" and len(parts) == 3:
                i, j = int(parts[1]) - 1, int(parts[2]) - 1
                item = files.pop(i)
                files.insert(j, item)

            elif parts[0] == "rm" and len(parts) == 2:
                i = int(parts[1]) - 1
                files.pop(i)

        except Exception:
            print("Comando inválido.")

    return files


def merge_pdfs(files: List[Path], output_path: Path):
    writer = PdfWriter()
    for file in files:
        reader = PdfReader(str(file))
        for page in reader.pages:
            writer.add_page(page)

    with open(output_path, "wb") as f:
        writer.write(f)


def register_run(root: Path, files: List[Path], output: Path):
    run_dir = root / "user" / "runs" / "pdf_merge" / now_stamp()
    run_dir.mkdir(parents=True, exist_ok=True)

    log_file = run_dir / "run.txt"
    with open(log_file, "w", encoding="utf-8") as f:
        f.write("PDF MERGE RUN\n")
        f.write(f"Output: {output}\n")
        f.write("Inputs:\n")
        for file in files:
            f.write(f" - {file}\n")


# ============================
# Main
# ============================

def main():
    root = app_root()
    input_dir = root / "user" / "input"
    output_dir = root / "user" / "output"

    input_dir.mkdir(exist_ok=True, parents=True)
    output_dir.mkdir(exist_ok=True, parents=True)

    pdfs = list_pdfs(input_dir)

    if not pdfs:
        print("No hay PDFs en user/input/")
        return 0

    print("PDFs disponibles en user/input/:")
    print_list(pdfs)

    selection = input("Selecciona números separados por espacio: ")
    try:
        indices = [int(i) - 1 for i in selection.split()]
        selected = [pdfs[i] for i in indices]
    except Exception:
        print("Selección inválida.")
        return 1

    ordered = reorder_interactive(selected)

    output_name = f"merged_{now_stamp()}.pdf"
    output_path = output_dir / output_name

    merge_pdfs(ordered, output_path)
    register_run(root, ordered, output_path)

    print(f"\nOK -> {output_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

### system/bin/tools/tool_healthcheck_runtime.py
- size: 10998 bytes
- sha256: `14dbd38a0c673a0a4313fbc45b8004eb2023ea9971edb898d78996841e5d2d9a`

```text
"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_healthcheck_runtime.py",
  "version": "2.0.0",
  "type": "system_tool",
  "category": "product",
  "description": "Verifica integridad portable, runtime, dependencias y estructura.",
  "location_expected": "system/bin/tools/",
  "runtime_required": true,
  "modifies_system": false,
  "output_location": "user/runs/healthcheck/<timestamp>/",
  "outputs": [
    "healthcheck.json"
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

import hashlib
import json
import platform
import re
import subprocess
import sys
from datetime import datetime
from pathlib import Path
from typing import Any


HEALTHCHECK_VERSION = "2.0.0"


def get_app_root() -> Path:
    # system/bin/tools/tool_healthcheck_runtime.py -> App_Tool (subir 3)
    return Path(__file__).resolve().parents[3]


def read_text_bom_safe(path: Path) -> str:
    return path.read_text(encoding="utf-8-sig")


def load_registry(app_root: Path) -> dict[str, Any]:
    registry_path = app_root / "system" / "config" / "runtime_registry.json"
    return json.loads(read_text_bom_safe(registry_path))


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def normalize_slashes(s: str) -> str:
    return s.replace("\\", "/")


def check_import(module_name: str) -> dict[str, Any]:
    try:
        __import__(module_name)
        return {"module": module_name, "ok": True, "error": None}
    except Exception as e:
        return {"module": module_name, "ok": False, "error": str(e)}


def run_cmd(args: list[str]) -> tuple[int, str]:
    try:
        r = subprocess.run(args, capture_output=True, text=True)
        out = (r.stdout or "").strip()
        err = (r.stderr or "").strip()
        if out:
            return r.returncode, out
        return r.returncode, err
    except Exception as e:
        return 1, f"ERROR: {e}"


def scan_pycache_in_system(app_root: Path) -> dict[str, Any]:
    system_dir = app_root / "system"
    paths: list[str] = []

    if system_dir.exists():
        for p in system_dir.rglob("__pycache__"):
            if p.is_dir():
                paths.append(normalize_slashes(str(p.relative_to(app_root))))

    sample = paths[:10]
    return {"count": len(paths), "paths_sample": sample}


def registry_integrity_checks(registry_text: str) -> dict[str, Any]:
    # Rutas absolutas Windows: "C:\"
    has_abs_win = bool(re.search(r"[A-Za-z]:\\", registry_text))
    # Rutas absolutas tipo "C:/"
    has_abs_posixish = bool(re.search(r"[A-Za-z]:/", registry_text))

    no_absolute_paths = not (has_abs_win or has_abs_posixish)

    # Prohibiciones conocidas
    no_venv_python = "system/bin/runtimes/python/current/Scripts/python.exe" not in registry_text.replace("\\", "/")
    no_wpy64_refs = "WPy64-" not in registry_text

    return {
        "no_absolute_paths_in_registry": no_absolute_paths,
        "no_venv_python": no_venv_python,
        "no_wpy64_refs": no_wpy64_refs,
    }


def validate_registry_with_local_validator(app_root: Path, reg: dict[str, Any]) -> tuple[bool, list[str]]:
    """
    Usa system/bin/entrypoints/registry_validate.py sin asumir que el cwd está bien.
    """
    entrypoints_dir = app_root / "system" / "bin" / "entrypoints"
    sys.path.insert(0, str(entrypoints_dir))

    try:
        from registry_validate import validate_registry, RegistryValidationError  # type: ignore

        try:
            validate_registry(reg, app_root)
            return True, []
        except RegistryValidationError as e:
            # El validador ya formatea el mensaje con líneas "- ..."
            msg = str(e)
            # extraer líneas de error
            lines = [ln.strip() for ln in msg.splitlines() if ln.strip().startswith("-")]
            if not lines:
                lines = [msg]
            return False, lines
    except Exception as e:
        return False, [f"- No se pudo cargar registry_validate.py: {e}"]


def main() -> int:
    app_root = get_app_root()
    registry_path_rel = "system/config/runtime_registry.json"
    registry_path = app_root / "system" / "config" / "runtime_registry.json"

    reg = load_registry(app_root)

    # --- Run folder
    ts_folder = datetime.now().strftime("%Y%m%d_%H%M%S")
    out_dir = app_root / "user" / "runs" / "healthcheck" / ts_folder
    out_dir.mkdir(parents=True, exist_ok=True)

    # --- App meta
    meta = reg.get("app_meta", {}) or {}
    app_name = str(meta.get("name") or app_root.name)
    app_vendor = str(meta.get("vendor") or "")
    app_version = str(meta.get("version") or "")

    # --- Registry validation
    reg_valid, reg_errors = validate_registry_with_local_validator(app_root, reg)

    tools = reg.get("tools", {}) or {}
    runtimes = reg.get("runtimes", {}) or {}

    tools_list: list[dict[str, Any]] = []
    for tid in sorted(tools.keys()):
        t = tools[tid] or {}
        entry = str(t.get("entry") or "")
        runtime_ref = str(t.get("runtime_ref") or "")
        entry_path = (app_root / entry) if entry else None
        tools_list.append(
            {
                "tool_id": tid,
                "runtime_ref": runtime_ref,
                "entry": entry,
                "entry_exists": bool(entry_path and entry_path.exists()),
            }
        )

    # --- Runtime info (python)
    runtime_ref = "python"
    runtime_obj = (runtimes.get(runtime_ref) or {}) if isinstance(runtimes, dict) else {}
    console_executable = str(runtime_obj.get("console_executable") or "")
    resolved_console = app_root / console_executable if console_executable else None

    console_exists = bool(resolved_console and resolved_console.exists())
    sys_executable = normalize_slashes(sys.executable)
    resolved_console_norm = normalize_slashes(str(resolved_console)) if resolved_console else ""

    # pip version via current interpreter (no dependemos de Scripts en PATH)
    pip_rc, pip_out = run_cmd([sys.executable, "-m", "pip", "--version"])

    # --- Dependencies
    lock_path = app_root / "system" / "bin" / "runtimes" / "python" / "current" / "requirements.lock.txt"
    lock_rel = "system/bin/runtimes/python/current/requirements.lock.txt"
    lock_exists = lock_path.exists()
    lock_sha = sha256_file(lock_path) if lock_exists else None

    imports = [
        check_import("reportlab"),
        check_import("pypdf"),
        check_import("PIL"),
    ]

    # --- Portable integrity
    registry_text = read_text_bom_safe(registry_path)
    integrity = registry_integrity_checks(registry_text)
    pycache_info = scan_pycache_in_system(app_root)
    integrity["pycache_in_system"] = pycache_info

    # --- Status decision
    fail_reasons: list[str] = []
    warn_reasons: list[str] = []

    if not reg_valid:
        fail_reasons.append("Registry inválido (validate_registry).")
        # añadir detalle resumido
        for e in reg_errors[:10]:
            fail_reasons.append(e.lstrip("- ").strip())

    if not console_exists:
        fail_reasons.append("console_executable no existe en disco.")

    # sys.executable debería coincidir con el runtime declarado (tolerando slash)
    if console_exists and resolved_console_norm and sys_executable.lower() != resolved_console_norm.lower():
        fail_reasons.append("sys.executable no coincide con console_executable declarado.")

    if pip_rc != 0:
        warn_reasons.append("pip no respondió correctamente (se recomienda revisar).")

    if any(not it["ok"] for it in imports):
        fail_reasons.append("Falló import de dependencias mínimas.")

    if not lock_exists:
        warn_reasons.append("No existe requirements.lock.txt (reproducibilidad reducida).")

    # Integridad portable
    if not integrity["no_absolute_paths_in_registry"]:
        fail_reasons.append("Registry contiene rutas absolutas (no portable).")
    if not integrity["no_venv_python"]:
        fail_reasons.append("Registry referencia venv (current/Scripts/python.exe).")
    if not integrity["no_wpy64_refs"]:
        warn_reasons.append("Registry contiene referencia a WPy64- (posible deriva de runtime).")

    if pycache_info["count"] > 0:
        warn_reasons.append("__pycache__ dentro de system/ (recomendado limpiar en releases).")

    status = "OK"
    if fail_reasons:
        status = "FAIL"
    elif warn_reasons:
        status = "WARN"

    report: dict[str, Any] = {
        "healthcheck_version": HEALTHCHECK_VERSION,
        "timestamp": datetime.now().isoformat(timespec="seconds"),
        "status": status,
        "app": {
            "app_root": normalize_slashes(str(app_root)),
            "name": app_name,
            "vendor": app_vendor,
            "version": app_version,
        },
        "registry": {
            "path": registry_path_rel,
            "validated": reg_valid,
            "errors": reg_errors,
            "tools_count": len(tools_list),
            "runtimes_count": len(runtimes) if isinstance(runtimes, dict) else 0,
            "tools": tools_list,
        },
        "runtime": {
            "runtime_ref": runtime_ref,
            "console_executable": console_executable,
            "console_exists": console_exists,
            "sys_executable": sys_executable,
            "python_version": sys.version.splitlines()[0],
            "arch": platform.architecture()[0],
            "pip_version": pip_out,
        },
        "dependencies": {
            "lock_path": lock_rel,
            "lock_exists": lock_exists,
            "lock_sha256": lock_sha,
            "imports": imports,
        },
        "portable_integrity": integrity,
        "summary": {
            "fail_reasons": fail_reasons,
            "warn_reasons": warn_reasons,
        },
    }

    out_path = out_dir / "healthcheck.json"
    out_path.write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8")

    # Console output (compact)
    rel_out = normalize_slashes(str(out_path.relative_to(app_root)))
    print(f"[HEALTHCHECK] status={status}")
    print(f"[HEALTHCHECK] python={sys_executable}")
    print(f"[HEALTHCHECK] report={rel_out}")

    return 0 if status != "FAIL" else 1


if __name__ == "__main__":
    raise SystemExit(main())
```

