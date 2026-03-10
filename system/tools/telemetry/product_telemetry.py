import json
import os
import fnmatch
from pathlib import Path
from datetime import datetime, timezone
from collections import defaultdict
from typing import Dict, List, Tuple, Any, Optional


def load_spec(spec_path: Path) -> dict:
    with spec_path.open("r", encoding="utf-8") as f:
        return json.load(f)


def _normalize_patterns(patterns: Optional[List[str]]) -> List[str]:
    if not patterns:
        return []
    return [str(p).strip() for p in patterns if str(p).strip()]


def _should_exclude(name: str, patterns: List[str]) -> bool:
    """Match using fnmatch patterns (e.g., '*.tmp', '__pycache__', '.git')."""
    for pat in patterns:
        if fnmatch.fnmatch(name, pat):
            return True
    return False


def scan_zone(
    app_root: Path,
    zone_path: Path,
    exclude_dir_patterns: Optional[List[str]] = None,
    exclude_file_patterns: Optional[List[str]] = None
) -> Tuple[int, int, List[dict], Dict[str, int], Dict[str, dict]]:
    """
    Scan a zone folder and compute:
      - file_count
      - total_size_bytes
      - files: [{relative_path, size_bytes}]
      - folders: {relative_folder_path: size_bytes}
      - by_extension: {'.pdf': {file_count, size_bytes}, ...}
    """
    exclude_dir_patterns = _normalize_patterns(exclude_dir_patterns)
    exclude_file_patterns = _normalize_patterns(exclude_file_patterns)

    file_count = 0
    total_size = 0
    files: List[dict] = []
    folders: Dict[str, int] = defaultdict(int)
    by_extension: Dict[str, dict] = defaultdict(lambda: {"file_count": 0, "size_bytes": 0})

    if not zone_path.exists():
        return file_count, total_size, files, folders, by_extension

    for root, dirnames, filenames in os.walk(zone_path):
        # Prune excluded directories in-place to avoid walking them
        dirnames[:] = [d for d in dirnames if not _should_exclude(d, exclude_dir_patterns)]

        root_path = Path(root)

        for name in filenames:
            if _should_exclude(name, exclude_file_patterns):
                continue

            file_path = root_path / name
            try:
                st = file_path.stat()
            except OSError:
                continue

            size = st.st_size
            try:
                rel_path = file_path.relative_to(app_root)
            except ValueError:
                rel_path = file_path

            file_count += 1
            total_size += size

            files.append({
                "relative_path": str(rel_path).replace("\\", "/"),
                "size_bytes": size
            })

            try:
                rel_folder = root_path.relative_to(app_root)
                folders[str(rel_folder).replace("\\", "/")] += size
            except ValueError:
                folders[str(root_path).replace("\\", "/")] += size

            ext = file_path.suffix.lower() or "no_ext"
            by_extension[ext]["file_count"] += 1
            by_extension[ext]["size_bytes"] += size

    return file_count, total_size, files, folders, by_extension


def _count_registry_documents(app_root: Path, registry_cfg: dict) -> int:
    """
    Count registered documents as number of subfolders under the configured documents path.
    Default convention: user/registry/documents/<doc_id>/
    """
    rel_docs_path = registry_cfg.get("documents_path") or "user/registry/documents"
    docs_path = app_root / rel_docs_path

    if not docs_path.exists():
        return 0

    try:
        return sum(1 for p in docs_path.iterdir() if p.is_dir())
    except OSError:
        return 0


def _count_outputs(outputs_cfg: dict, zone_path: Path) -> int:
    """
    Count outputs.
    Default: number of files in user/outputs (recursive).
    Optional: restrict by extensions via outputs_cfg['final_extensions'] (list like ['.pdf','.docx']).
    """
    final_exts = outputs_cfg.get("final_extensions")
    if not final_exts:
        count = 0
        for _, _, filenames in os.walk(zone_path):
            count += len(filenames)
        return count

    final_exts = [e.lower() for e in final_exts]
    count = 0
    for _, _, filenames in os.walk(zone_path):
        for name in filenames:
            if Path(name).suffix.lower() in final_exts:
                count += 1
    return count


def _build_alert(code: str, message: str, severity: str = "warning") -> dict:
    return {"code": code, "message": message, "severity": severity}


def analyze_workspace(app_root: Path, spec: dict) -> dict:
    zones_spec = spec.get("scan_zones", {})
    metrics_spec = spec.get("metrics", {})
    alerts_spec = spec.get("alerts", {})
    exclude_cfg = spec.get("exclude", {})

    exclude_dir_patterns = exclude_cfg.get(
        "exclude_dir_patterns",
        ["__pycache__", ".git", ".svn", ".hg"]
    )
    exclude_file_patterns = exclude_cfg.get(
        "exclude_file_patterns",
        ["*.tmp", "*.bak", "*.swp", "Thumbs.db", ".DS_Store"]
    )

    # Schema expects these zones to exist in the report (even if disabled)
    zone_names = ["runs", "outputs", "registry"]

    summary_total_files = 0
    summary_total_size = 0

    zones_report: Dict[str, dict] = {zn: {"file_count": 0, "size_bytes": 0} for zn in zone_names}

    all_files: List[dict] = []
    extension_accumulator: Dict[str, dict] = defaultdict(lambda: {"file_count": 0, "size_bytes": 0})
    folder_accumulator: Dict[str, int] = defaultdict(int)

    for zone_name in zone_names:
        zone_cfg = zones_spec.get(zone_name, {})
        if not zone_cfg.get("enabled", False):
            continue

        zone_path = app_root / zone_cfg.get("relative_path", f"user/{zone_name}")

        file_count, size_bytes, files, folders, by_ext = scan_zone(
            app_root=app_root,
            zone_path=zone_path,
            exclude_dir_patterns=exclude_dir_patterns,
            exclude_file_patterns=exclude_file_patterns
        )

        zones_report[zone_name] = {"file_count": file_count, "size_bytes": size_bytes}

        summary_total_files += file_count
        summary_total_size += size_bytes
        all_files.extend(files)

        for ext, data in by_ext.items():
            extension_accumulator[ext]["file_count"] += int(data.get("file_count", 0))
            extension_accumulator[ext]["size_bytes"] += int(data.get("size_bytes", 0))

        for folder, size in folders.items():
            folder_accumulator[folder] += int(size)

    # Domain counts
    registry_cfg = spec.get("registry", {})
    outputs_cfg = spec.get("outputs", {})

    document_count = _count_registry_documents(app_root, registry_cfg)

    outputs_zone_cfg = zones_spec.get("outputs", {})
    outputs_zone_path = app_root / outputs_zone_cfg.get("relative_path", "user/outputs")
    outputs_count = _count_outputs(outputs_cfg, outputs_zone_path) if outputs_zone_cfg.get("enabled", False) else 0

    # Optional sections, governed by spec flags
    by_extension_list: List[dict] = []
    if metrics_spec.get("compute_by_extension", True):
        by_extension_list = [
            {"extension": ext, "file_count": data["file_count"], "size_bytes": data["size_bytes"]}
            for ext, data in extension_accumulator.items()
        ]
        by_extension_list.sort(key=lambda x: (x["extension"] or ""))

    largest_files: List[dict] = []
    if metrics_spec.get("compute_largest_files", True):
        n = int(metrics_spec.get("top_n_largest_files", 20))
        largest_files = sorted(all_files, key=lambda x: x["size_bytes"], reverse=True)[:n]

    largest_folders: List[dict] = []
    if metrics_spec.get("compute_largest_folders", True):
        n = int(metrics_spec.get("top_n_largest_folders", 20))
        largest_folders = sorted(
            [{"relative_path": k, "size_bytes": v} for k, v in folder_accumulator.items()],
            key=lambda x: x["size_bytes"],
            reverse=True
        )[:n]

    # Alerts (minimum viable)
    alerts: List[dict] = []

    if alerts_spec.get("detect_outputs_inside_runs", False):
        suspicious = app_root / "user" / "runs" / "outputs"
        if suspicious.exists():
            alerts.append(_build_alert(
                code="OUTPUTS_INSIDE_RUNS",
                message="Found 'user/runs/outputs'. Outputs should live in 'user/outputs'.",
                severity="warning"
            ))

    threshold = alerts_spec.get("detect_large_single_file_threshold_bytes")
    if isinstance(threshold, int) and threshold > 0:
        over = [f for f in all_files if int(f.get("size_bytes", 0)) >= threshold]
        if over:
            top = sorted(over, key=lambda x: x["size_bytes"], reverse=True)[:5]
            msg = "Found very large file(s) (>= threshold). Top offenders: " + ", ".join(
                f"{t['relative_path']} ({t['size_bytes']} bytes)" for t in top
            )
            alerts.append(_build_alert(code="LARGE_SINGLE_FILE", message=msg, severity="warning"))

    report: Dict[str, Any] = {
        "schema_version": "1.0.0",
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "summary": {
            "total_files": summary_total_files,
            "total_size_bytes": summary_total_size,
            "document_count": document_count,
            "outputs_count": outputs_count
        },
        "zones": zones_report,
        "alerts": alerts
    }

    if metrics_spec.get("compute_by_extension", True):
        report["by_extension"] = by_extension_list
    if metrics_spec.get("compute_largest_files", True):
        report["largest_files"] = largest_files
    if metrics_spec.get("compute_largest_folders", True):
        report["largest_folders"] = largest_folders

    return report


def run_product_telemetry(app_root: Path, spec_path: Path) -> dict:
    spec = load_spec(spec_path)
    return analyze_workspace(app_root, spec)
