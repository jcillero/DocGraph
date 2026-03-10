# APP ANALYSIS BUNDLE
- Generated at: 2026-02-24 22:55:33
- App root: `App_Tool`
- Scripts included: 7

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
│   │       ├── pdf_merge.py
│   │       └── tool_healthcheck_runtime.py
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
├── BASELINE_1.0.0.md.txt
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
- size: 12045 bytes
- sha256: `1f56889e9269521caa477381aa67b09a2a16009fe8721b2dc4efd70344dd5f7d`

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
    - Busca triple comillas al principio (''' o """)
    """
    try:
        head = py_file.read_text(encoding="utf-8", errors="ignore")[: max_chars]
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

    # 5) scripts sin SCRIPT_META (solo en system/bin y dev/scripts)
    script_issues: List[str] = []
    for rel_base in ["system/bin", "dev/scripts"]:
        base = app_root / rel_base
        if not base.exists():
            continue
        for py_file in base.rglob("*.py"):
            if not _has_script_meta(py_file):
                script_issues.append(_safe_rel(app_root, py_file))
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
- size: 10489 bytes
- sha256: `400bfa3be6157a15773d3968216366d4b3d124f81bf7e7b079df94974ce0fa23`

```text
from __future__ import annotations

import json
import os
import shutil
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
    # BOM-safe
    return json.loads(registry_path.read_text(encoding="utf-8-sig"))


def build_env(runtime: dict) -> dict:
    env = os.environ.copy()
    for k, v in (runtime.get("env", {}) or {}).items():
        env[str(k)] = str(v)
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

    py_exe = app_root / console_exe
    entry = app_root / entry_rel

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

    env = build_env(runtime)
    return subprocess.call([str(py_exe), str(entry), *pass_args], env=env)


# -----------------------------
# UI helpers (compact shell)
# -----------------------------
def clear_screen() -> None:
    os.system("cls" if os.name == "nt" else "clear")


def _terminal_columns(default: int = 100) -> int:
    try:
        return shutil.get_terminal_size((default, 20)).columns
    except Exception:
        return default


def _fit_banner_text(text: str, width: int) -> str:
    text = " ".join((text or "").split())
    if width <= 0:
        return ""
    if len(text) <= width:
        return text.center(width)
    return (text[: width - 1] + "…")


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
            label = str(p.get("label", "Preset"))
            args = p.get("args", [])
            if not isinstance(args, list):
                args = []
            out.append({"label": label, "args": [str(a) for a in args]})
        if out:
            return out
    return [{"label": "Ejecutar", "args": []}]


def build_compact_menu(reg: dict) -> tuple[list[dict[str, Any]], list[str]]:
    tools = reg.get("tools", {}) or {}
    tool_ids = sorted(tools.keys())

    items: list[dict[str, Any]] = []
    lines: list[str] = []

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

        lines.append(f"[{cat.upper()}]")

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
                lines.append(f" {key:<4} {tname} — {p['label']}")
        lines.append("")

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


def _console_title_from_registry(app_root: Path, reg: dict) -> str:
    meta = reg.get("app_meta", {}) or {}
    name = (meta.get("name") or app_root.name).strip()
    vendor = (meta.get("vendor") or "").strip()
    version = (meta.get("version") or "").strip()

    left = name.upper()
    if vendor:
        left += f" — {vendor.upper()}"
    if version:
        left += f" (v{version})"
    left += " — CONSOLA DE TOOLS"
    return left


def shell_mode(app_root: Path) -> int:
    while True:
        reg = load_registry(app_root)
        items, lines = build_compact_menu(reg)

        clear_screen()
        total_tools = len(reg.get("tools", {}) or {})
        title = _console_title_from_registry(app_root, reg)

        print(banner(title, f"Tools registradas: {total_tools}\n", min_inner=44, max_inner=140))
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
        print(f"   args: {' '.join(match['args'])}" if match["args"] else "   args: (none)")
        print("")

        rc = run_tool(app_root, reg, match["tool_id"], match["args"])

        print(f"\n(Exit code: {rc})")
        input("Enter para volver al menú…")


# -----------------------------
# CLI commands (list/run/shell/diag)
# -----------------------------
def diag(app_root: Path, reg: dict) -> int:
    print("=== DIAG ===")
    print(f"app_root:      {app_root}")
    print(f"sys.executable:{sys.executable}")
    print(f"sys.version:   {sys.version.splitlines()[0]}")
    print("")
    runtimes = reg.get("runtimes", {}) or {}
    print("Runtimes declarados:")
    for k in sorted(runtimes.keys()):
        exe = runtimes[k].get("console_executable", "")
        print(f"  - {k}: {exe}")
    print("")
    tools = reg.get("tools", {}) or {}
    print("Tools registradas:")
    for tid in sorted(tools.keys()):
        rr = tools[tid].get("runtime_ref", "")
        entry = tools[tid].get("entry", "")
        print(f"  - {tid}: runtime_ref={rr} entry={entry}")
    return 0


def main() -> int:
    if len(sys.argv) < 2:
        print("Uso:")
        print("  app_cli.py list")
        print("  app_cli.py run <tool_id> [-- tool_args...]")
        print("  app_cli.py shell")
        print("  app_cli.py diag")
        return 2

    cmd = sys.argv[1].lower()
    app_root = get_app_root()
    reg = load_registry(app_root)

    if cmd == "list":
        tools = reg.get("tools", {}) or {}
        print("Tools registradas:")
        for tool_id in sorted(tools.keys()):
            desc = tools[tool_id].get("description", "")
            print(f"  - {tool_id}" + (f"  | {desc}" if desc else ""))
        return 0

    if cmd == "diag":
        return diag(app_root, reg)

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

    print("Comando inválido. Usa: list | run | shell | diag")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
```

### system/bin/tools/csv_to_report_pdf.py
- size: 11490 bytes
- sha256: `192bb798edda2bb95c4e8c7ec71c9e9d19aac7cf4c69920b66eb9bddf3f3f84f`

```text
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
- size: 3588 bytes
- sha256: `5ca5a008de021666bb8dd8e3e8452e0da860e74646909385bdf494acec57166f`

```text
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
- size: 2814 bytes
- sha256: `e907f83cf61561ed0b40bec6cee10d380f344f0b3b331f77f4a4f8cb79a7893c`

```text
"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_healthcheck_runtime.py",
  "version": "1.0.0",
  "type": "runtime_tool",
  "category": "maintenance",
  "description": "Verifica integridad del runtime (python executable, imports críticos) y genera traza.",
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
    "DEV_RUNBOOK"
  ]
}
================================================================================
"""

from __future__ import annotations

import json
import platform
import sys
from datetime import datetime
from pathlib import Path


def app_root() -> Path:
    # system/bin/tools/*.py -> subir 3 hasta la raíz de la app
    return Path(__file__).resolve().parents[3]


def make_run_dir(root: Path) -> Path:
    ts = datetime.now().strftime("%Y%m%d_%H%M%S")
    out = root / "user" / "runs" / "healthcheck" / ts
    out.mkdir(parents=True, exist_ok=True)
    return out


def check_import(name: str) -> dict:
    try:
        mod = __import__(name)
        ver = getattr(mod, "__version__", None)
        return {"module": name, "ok": True, "version": ver}
    except Exception as e:
        return {"module": name, "ok": False, "error": str(e)}


def main() -> int:
    root = app_root()
    run_dir = make_run_dir(root)
    out_path = run_dir / "healthcheck.json"

    checks = [
        check_import("reportlab"),
        check_import("pypdf"),
    ]

    ok = all(c["ok"] for c in checks)

    payload = {
        "timestamp": datetime.now().isoformat(timespec="seconds"),
        "status": "OK" if ok else "FAIL",
        "python": {
            "executable": sys.executable,
            "version": sys.version,
        },
        "platform": {
            "system": platform.system(),
            "release": platform.release(),
            "machine": platform.machine(),
        },
        "checks": checks,
        "artifacts": {
            "output": str(out_path.relative_to(root)).replace("\\", "/")
        }
    }

    out_path.write_text(json.dumps(payload, indent=2, ensure_ascii=False), encoding="utf-8")

    print(f"[HEALTHCHECK] status={payload['status']}")
    print(f"[HEALTHCHECK] python={payload['python']['executable']}")
    print(f"[HEALTHCHECK] report={payload['artifacts']['output']}")

    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
```

