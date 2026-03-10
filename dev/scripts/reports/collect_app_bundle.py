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