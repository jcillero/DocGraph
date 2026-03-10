#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_pdf_merge.py",
  "version": "1.4.1",
  "type": "product_tool",
  "category": "product",
  "tool_id": "tool_pdf_merge",
  "description": "Merges multiple PDFs selected interactively from user/input, allows interactive reordering, stores the canonical artifact under user/runs/.../outputs, and optionally publishes a visible copy under user/output.",
  "location_expected": "system/bin/tools/product",
  "entry_point": "main",

  "inputs": [
    {
      "type": "file",
      "location": "user/input/*.pdf",
      "interactive_selection": true
    }
  ],

  "outputs": [
    {
      "type": "file",
      "location": "user/runs/pdf_merge/<run_id>/outputs/merged_<timestamp>.pdf",
      "role": "canonical_run_artifact"
    },
    {
      "type": "file",
      "location": "user/output/merged_<timestamp>.pdf",
      "role": "published_export",
      "optional": true
    },
    {
      "type": "run_logs",
      "location": "user/runs/pdf_merge/<run_id>/"
    }
  ],

  "capabilities": [
    "interactive_selection",
    "interactive_reorder",
    "pdf_merge",
    "input_traceability",
    "run_artifact_tracking",
    "published_export"
  ],

  "invariants_relevant": [
    "INV-001 separation system/user/dev",
    "INV-004 ui_cli_without_domain_logic",
    "INV-005 self_contained_workspace",
    "INV-006 traceability_required",
    "INV-009 structural_determinism",
    "INV-013 strict_output_layout",
    "INV-014 artifact_linked_to_run"
  ],

  "modifies_system": false,
  "runtime_ref": "python",
  "runtime": "python_portable",

  "logging": {
    "spec": "logging_v1",
    "catalog_event_types": "system/config/logging/logging_event_types_v1.json",
    "catalog_work_units": "system/config/logging/work_units_catalog_v1.json"
  }
}
================================================================================
"""

from __future__ import annotations

import shutil
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Tuple

from pypdf import PdfReader, PdfWriter


TOOL_ID = "tool_pdf_merge"
TOOL_VERSION = "1.4.1"


def app_root() -> Path:
    """
    Resolve application root directory.

    Expected tool location:
        system/bin/tools/product/tool_pdf_merge.py

    parents[0] -> product
    parents[1] -> tools
    parents[2] -> bin
    parents[3] -> system
    parents[4] -> application root

    Using parents[4] ensures that system/, user/ and dev/
    remain sibling folders, as required by the platform architecture.
    """
    return Path(__file__).resolve().parents[4]


def now_stamp() -> str:
    return datetime.now().strftime("%Y%m%d_%H%M%S")


def sha256_file(path: Path) -> str:
    import hashlib

    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def file_size_bytes(path: Path) -> int:
    try:
        return int(path.stat().st_size)
    except Exception:
        return 0


def to_rel_posix(root: Path, p: Path) -> str:
    try:
        return str(p.relative_to(root)).replace("\\", "/")
    except Exception:
        return str(p).replace("\\", "/")


def pdf_page_count(path: Path) -> int:
    try:
        reader = PdfReader(str(path))
        return len(reader.pages)
    except Exception:
        return 0


def list_input_pdfs(input_dir: Path) -> List[Path]:
    return sorted(input_dir.glob("*.pdf"))


def prompt_selection(files: List[Path]) -> List[Path]:
    if not files:
        return []

    print("Available PDF files:")
    for i, p in enumerate(files, 1):
        print(f"{i}. {p.name}")

    raw = input("Select numbers separated by commas: ").strip()
    idxs = [int(x) - 1 for x in raw.split(",") if x.strip().isdigit()]

    selected: List[Path] = []
    for idx in idxs:
        if 0 <= idx < len(files):
            selected.append(files[idx])

    return selected


def interactive_reorder(paths: List[Path]) -> List[Path]:
    selected = list(paths)

    if len(selected) <= 1:
        return selected

    while True:
        print("\nCurrent order:")
        for i, p in enumerate(selected, 1):
            print(f"{i}. {p.name}")

        print("\nCommands: swap A B | move A B | rm A | ok")
        command = input("Action: ").strip()

        if command.lower() == "ok":
            return selected

        parts = command.split()
        if not parts:
            continue

        try:
            op = parts[0].lower()

            if op == "swap" and len(parts) == 3:
                a, b = int(parts[1]) - 1, int(parts[2]) - 1
                if 0 <= a < len(selected) and 0 <= b < len(selected):
                    selected[a], selected[b] = selected[b], selected[a]

            elif op == "move" and len(parts) == 3:
                a, b = int(parts[1]) - 1, int(parts[2]) - 1
                if 0 <= a < len(selected) and 0 <= b < len(selected):
                    item = selected.pop(a)
                    selected.insert(b, item)

            elif op == "rm" and len(parts) == 2:
                a = int(parts[1]) - 1
                if 0 <= a < len(selected):
                    selected.pop(a)

            else:
                print("Invalid command.")

        except ValueError:
            print("Invalid input.")

        if not selected:
            return []


def merge_pdfs(paths: List[Path], out_path: Path) -> Tuple[int, int]:
    writer = PdfWriter()
    total_pages = 0

    for p in paths:
        reader = PdfReader(str(p))
        total_pages += len(reader.pages)
        for page in reader.pages:
            writer.add_page(page)

    with out_path.open("wb") as f:
        writer.write(f)

    return len(paths), total_pages


def main() -> int:
    root = app_root()

    try:
        from system.core.logging.run_context import create_run_context
        from system.core.logging.logging_runtime import init_logger, WorkUnit, WorkUnits
    except ModuleNotFoundError as exc:
        print(
            "[ERROR] Could not import 'system.core.logging'. "
            "Execute this tool from the application root or via the launcher."
        )
        print(f"Detail: {exc}")
        return 2

    ctx = create_run_context(root, tool_id=TOOL_ID)
    inputs_meta: List[Dict[str, object]] = []

    logger = init_logger(
        ctx,
        tool_version=TOOL_VERSION,
        runtime_version="python_portable",
        inputs=inputs_meta,
        work_units=WorkUnits(primary=WorkUnit("actions", 1)),
    )

    input_dir = root / "user" / "input"
    output_dir = ctx.run_dir / "outputs"
    public_dir = root / "user" / "output"

    input_dir.mkdir(parents=True, exist_ok=True)
    output_dir.mkdir(parents=True, exist_ok=True)
    public_dir.mkdir(parents=True, exist_ok=True)

    exit_code = 1
    run_status = "failed"

    try:
        t_ingest = logger.stage_start("ingest", "Selecting PDFs")
        available = list_input_pdfs(input_dir)

        if not available:
            logger.warning("ingest", "No PDF files found", warning_type="no_inputs")
            logger.stage_end(
                "ingest",
                "No inputs",
                timer=t_ingest,
                work_units=WorkUnits(primary=WorkUnit("files", 0)),
            )
            print("No PDFs found in user/input")
            return 1

        selected = prompt_selection(available)
        if not selected:
            logger.warning("ingest", "No valid PDF selection", warning_type="invalid_selection")
            logger.stage_end(
                "ingest",
                "Invalid selection",
                timer=t_ingest,
                work_units=WorkUnits(primary=WorkUnit("files", 0)),
            )
            print("Empty or invalid selection.")
            return 1

        selected = interactive_reorder(selected)
        if not selected:
            logger.warning("ingest", "Selection emptied during reorder", warning_type="empty_selection")
            logger.stage_end(
                "ingest",
                "Selection emptied",
                timer=t_ingest,
                work_units=WorkUnits(primary=WorkUnit("files", 0)),
            )
            print("No PDFs remain after reordering.")
            return 1

        total_input_pages = 0
        for p in selected:
            pages = pdf_page_count(p)
            total_input_pages += pages
            try:
                inputs_meta.append(
                    {
                        "path": to_rel_posix(root, p),
                        "content_hash": f"sha256_{sha256_file(p)}",
                        "size_bytes": file_size_bytes(p),
                        "mime_type": "application/pdf",
                        "page_count": pages,
                    }
                )
            except Exception:
                inputs_meta.append(
                    {
                        "path": to_rel_posix(root, p),
                        "size_bytes": file_size_bytes(p),
                        "mime_type": "application/pdf",
                        "page_count": pages,
                    }
                )

        try:
            logger.set_inputs(inputs_meta)
        except Exception:
            pass

        ingest_ms = t_ingest.elapsed_ms()
        logger.stage_end(
            "ingest",
            "PDF selection completed",
            timer=t_ingest,
            work_units=WorkUnits(
                primary=WorkUnit("files", len(selected)),
                secondary=[WorkUnit("pages", total_input_pages)],
            ),
        )
        logger.metric("ingest", "interactive_duration_ms", int(ingest_ms), unit="ms")

        t_proc = logger.stage_start("process", "Merging PDFs")
        out_name = f"merged_{now_stamp()}.pdf"
        run_path = output_dir / out_name
        public_path = public_dir / out_name

        merged_files, merged_pages = merge_pdfs(selected, run_path)
        shutil.copy2(run_path, public_path)

        proc_ms = t_proc.elapsed_ms()
        logger.stage_end(
            "process",
            "PDF merged",
            timer=t_proc,
            work_units=WorkUnits(
                primary=WorkUnit("files", merged_files),
                secondary=[WorkUnit("pages", merged_pages)],
            ),
        )

        logger.metric("process", "compute_duration_ms", int(proc_ms), unit="ms")
        if proc_ms > 0:
            logger.metric(
                "process",
                "pages_per_second",
                round((merged_pages * 1000) / proc_ms, 3),
                unit="pages/s",
            )

        logger.artifact_written("output", to_rel_posix(root, run_path))
        logger.artifact_written("export", to_rel_posix(root, public_path))

        exit_code = 0
        run_status = "completed"
        print(f"\nOK (run) -> {run_path}")
        print(f"OK (export) -> {public_path}")
        return 0

    except ImportError as exc:
        logger.exception("process", "Missing PDF dependency", exc, exception_type="missing_dependency")
        print(
            "[ERROR] Missing dependency 'pypdf'. "
            "Check the portable runtime or requirements.lock.txt."
        )
        exit_code = 3
        run_status = "failed"
        return exit_code

    except Exception as exc:
        logger.exception("process", "PDF merge failed", exc, exception_type="pdf_merge_error")
        print(f"[ERROR] PDF merge failed: {exc}")
        exit_code = 99
        run_status = "failed"
        return exit_code

    finally:
        logger.close(
            status=run_status,
            exit_code=exit_code,
            work_units=WorkUnits(primary=WorkUnit("actions", 1)),
            summary="PDF merge execution",
        )


if __name__ == "__main__":
    raise SystemExit(main())
