#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_csv_to_report_pdf.py",
  "version": "1.3.1",
  "type": "product_tool",
  "category": "product",
  "tool_id": "tool_csv_to_report_pdf",
  "description": "Genera un reporte PDF a partir de un CSV seleccionado interactivamente desde user/input, guardando el artefacto canónico en user/runs y publicando opcionalmente una copia visible en user/output.",
  "location_expected": "system/bin/tools/product",
  "entry_point": "main",
  "inputs": [
    {
      "type": "file",
      "location": "user/input/*.csv",
      "interactive_selection": true
    }
  ],
  "outputs": [
    {
      "type": "file",
      "location": "user/runs/csv_to_report_pdf/<run_id>/outputs/csv_report_<name>_<timestamp>.pdf",
      "role": "canonical_run_artifact"
    },
    {
      "type": "file",
      "location": "user/output/csv_report_<name>_<timestamp>.pdf",
      "role": "published_export",
      "optional": true
    },
    {
      "type": "run_logs",
      "location": "user/runs/csv_to_report_pdf/<run_id>/"
    }
  ],
  "capabilities": [
    "interactive_selection",
    "csv_read",
    "pdf_generation",
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

import csv
import shutil
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Tuple


# -----------------------------------------------------------------------------
# APPLICATION ROOT RESOLUTION
# -----------------------------------------------------------------------------

def app_root() -> Path:
    """
    Resolve application root directory.

    Tool location (expected):
        system/bin/tools/product/csv_to_report_pdf.py

    parents[0] -> product
    parents[1] -> tools
    parents[2] -> bin
    parents[3] -> system
    parents[4] -> application root (DocGraph)

    Using parents[4] ensures user/, dev/ and system/ remain sibling folders
    as defined by the architecture invariants.
    """
    return Path(__file__).resolve().parents[4]


# -----------------------------------------------------------------------------
# UTILITIES
# -----------------------------------------------------------------------------

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


# -----------------------------------------------------------------------------
# CSV READING
# -----------------------------------------------------------------------------

def read_csv_records(path: Path) -> Tuple[List[str], List[Dict[str, str]]]:

    raw = path.read_text(encoding="utf-8-sig", errors="replace")

    reader = csv.DictReader(
        raw.splitlines(True),
        delimiter=";",
        quotechar='"'
    )

    fieldnames = reader.fieldnames or []

    records = [
        {k: (v or "") for k, v in row.items()}
        for row in reader
    ]

    return fieldnames, records


# -----------------------------------------------------------------------------
# PDF GENERATION
# -----------------------------------------------------------------------------

def build_pdf(
    out_path: Path,
    source_csv: Path,
    fieldnames: List[str],
    records: List[Dict[str, str]]
) -> None:

    from reportlab.lib.pagesizes import A4
    from reportlab.lib.styles import getSampleStyleSheet
    from reportlab.lib.units import mm
    from reportlab.platypus import Paragraph, SimpleDocTemplate, Spacer

    styles = getSampleStyleSheet()

    story = []

    story.append(Paragraph("<b>CSV REPORT</b>", styles["Title"]))
    story.append(Spacer(1, 5 * mm))

    story.append(
        Paragraph(f"Source file: {source_csv.name}", styles["Normal"])
    )

    story.append(
        Paragraph(f"Rows: {len(records)}", styles["Normal"])
    )

    story.append(
        Paragraph(f"Columns: {len(fieldnames)}", styles["Normal"])
    )

    doc = SimpleDocTemplate(
        str(out_path),
        pagesize=A4
    )

    doc.build(story)


# -----------------------------------------------------------------------------
# MAIN EXECUTION
# -----------------------------------------------------------------------------

def main() -> int:

    root = app_root()

    try:
        from system.core.logging.run_context import create_run_context
        from system.core.logging.logging_runtime import (
            init_logger,
            WorkUnit,
            WorkUnits,
        )
    except ModuleNotFoundError as exc:

        print(
            "[ERROR] Could not import 'system.core.logging'. "
            "Execute this tool from the application root "
            "or via the launcher."
        )

        print(f"Detail: {exc}")
        return 2

    ctx = create_run_context(root, tool_id="tool_csv_to_report_pdf")

    inputs_meta: List[Dict[str, object]] = []

    logger = init_logger(
        ctx,
        tool_version="1.3.1",
        runtime_version="python_portable",
        inputs=inputs_meta,
        work_units=WorkUnits(primary=WorkUnit("actions", 1)),
    )

    input_dir = root / "user" / "input"
    output_dir = ctx.run_dir / "outputs"

    input_dir.mkdir(parents=True, exist_ok=True)
    output_dir.mkdir(parents=True, exist_ok=True)

    exit_code = 1
    run_status = "failed"

    try:

        # ---------------------------------------------------------------------
        # INPUT SELECTION
        # ---------------------------------------------------------------------

        t_ingest = logger.stage_start("ingest", "Selecting CSV")

        files = sorted(input_dir.glob("*.csv"))

        if not files:

            logger.warning(
                "ingest",
                "No CSV files found",
                warning_type="no_inputs"
            )

            logger.stage_end(
                "ingest",
                "No inputs",
                timer=t_ingest,
                work_units=WorkUnits(
                    primary=WorkUnit("files", 0)
                ),
            )

            return 1

        print("Available CSV files:")

        for i, f in enumerate(files, 1):
            print(f"{i}. {f.name}")

        sel = input("Select number: ").strip()

        if not sel.isdigit():
            return 1

        idx = int(sel) - 1

        if idx < 0 or idx >= len(files):
            return 1

        csv_path = files[idx]

        # ---------------------------------------------------------------------
        # INPUT METADATA
        # ---------------------------------------------------------------------

        inputs_meta.append(
            {
                "path": to_rel_posix(root, csv_path),
                "content_hash": f"sha256_{sha256_file(csv_path)}",
                "size_bytes": file_size_bytes(csv_path),
                "mime_type": "text/csv",
            }
        )

        logger.set_inputs(inputs_meta)

        logger.stage_end(
            "ingest",
            "CSV selected",
            timer=t_ingest,
            work_units=WorkUnits(primary=WorkUnit("files", 1)),
        )

        # ---------------------------------------------------------------------
        # CSV VALIDATION
        # ---------------------------------------------------------------------

        t_val = logger.stage_start("validation", "Reading CSV")

        fieldnames, records = read_csv_records(csv_path)

        logger.stage_end(
            "validation",
            "CSV parsed",
            timer=t_val,
            work_units=WorkUnits(
                primary=WorkUnit("rows", len(records)),
                secondary=[WorkUnit("columns", len(fieldnames))]
            ),
        )

        # ---------------------------------------------------------------------
        # PDF GENERATION
        # ---------------------------------------------------------------------

        t_proc = logger.stage_start("process", "Generating PDF")

        out_path = (
            output_dir
            / f"csv_report_{csv_path.stem}_{now_stamp()}.pdf"
        )

        build_pdf(
            out_path,
            csv_path,
            fieldnames,
            records
        )

        public_dir = root / "user" / "output"
        public_dir.mkdir(parents=True, exist_ok=True)

        public_path = public_dir / out_path.name

        shutil.copy2(out_path, public_path)

        logger.artifact_written(
            "output",
            to_rel_posix(root, out_path)
        )

        logger.artifact_written(
            "export",
            to_rel_posix(root, public_path)
        )

        exit_code = 0
        run_status = "completed"

        print(f"OK (run) -> {out_path}")
        print(f"OK (export) -> {public_path}")

        return 0

    except Exception as e:

        logger.exception(
            "process",
            "CSV report failed",
            e,
            exception_type="csv_report_error"
        )

        return 99

    finally:

        logger.close(
            status=run_status,
            exit_code=exit_code,
            work_units=WorkUnits(
                primary=WorkUnit("actions", 1)
            ),
            summary="CSV to PDF report execution",
        )


if __name__ == "__main__":
    raise SystemExit(main())