import json
import shutil
import time
import os
from pathlib import Path
from datetime import datetime, timezone


def ensure_directory(path: Path) -> None:
    path.mkdir(parents=True, exist_ok=True)


def write_json_atomic(path: Path, data: dict) -> None:
    """
    Atomic JSON write:
    - write to a temp file in the same directory
    - fsync
    - replace target
    """
    ensure_directory(path.parent)
    tmp_path = path.with_suffix(path.suffix + ".tmp")

    with tmp_path.open("w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)
        f.flush()
        try:
            # Best-effort durability on supported platforms
            os.fsync(f.fileno())
        except Exception:
            pass

    tmp_path.replace(path)


def basic_validate_report(report: dict) -> None:
    required_keys = [
        "schema_version",
        "generated_at_utc",
        "summary",
        "zones",
        "alerts"
    ]
    for key in required_keys:
        if key not in report:
            raise ValueError(f"Missing required key in report: {key}")


def _snapshot_dirs(base_dir: Path) -> list[Path]:
    # snapshot folders are timestamp-like; 'latest' is reserved
    return sorted(
        [p for p in base_dir.iterdir() if p.is_dir() and p.name != "latest"],
        key=lambda p: p.name
    )


def prune_old_snapshots(base_dir: Path, retain_last_n: int) -> int:
    """
    Delete old snapshot directories, keeping only the most recent N.
    Returns number of deleted snapshot folders.
    """
    if retain_last_n <= 0:
        return 0

    snapshots = _snapshot_dirs(base_dir)
    if len(snapshots) <= retain_last_n:
        return 0

    to_delete = snapshots[:-retain_last_n]
    deleted = 0
    for folder in to_delete:
        shutil.rmtree(folder, ignore_errors=True)
        deleted += 1
    return deleted


def _timestamp_dirname() -> str:
    # Include milliseconds to avoid collisions when running multiple times per second
    now = datetime.now(timezone.utc)
    return now.strftime("%Y-%m-%d_%H-%M-%S") + f"_{int(now.microsecond/1000):03d}"


def save_report(
    app_root: Path,
    report: dict,
    retain_last_n: int = 200,
    auto_prune: bool = True,
    include_profile: bool = True
) -> dict:
    """
    Persist telemetry report snapshots.

    Writes:
      - user/runs/telemetry/product/<timestamp>/report.json
      - user/runs/telemetry/product/latest/report.json

    Safety:
      - atomic writes for JSON files
      - optional pruning (retention)

    Profiling:
      - returns timing metrics (ms) if include_profile=True
    """
    t0 = time.perf_counter()
    basic_validate_report(report)
    t_validate = time.perf_counter()

    telemetry_root = app_root / "user" / "runs" / "telemetry" / "product"
    ensure_directory(telemetry_root)

    timestamp = _timestamp_dirname()
    snapshot_dir = telemetry_root / timestamp
    ensure_directory(snapshot_dir)

    report_path = snapshot_dir / "report.json"
    write_json_atomic(report_path, report)
    t_snapshot = time.perf_counter()

    latest_dir = telemetry_root / "latest"
    ensure_directory(latest_dir)
    latest_report_path = latest_dir / "report.json"
    write_json_atomic(latest_report_path, report)
    t_latest = time.perf_counter()

    deleted = 0
    if auto_prune:
        deleted = prune_old_snapshots(telemetry_root, retain_last_n)
    t_prune = time.perf_counter()

    out = {
        "snapshot_path": str(report_path),
        "latest_path": str(latest_report_path),
        "status": "OK",
        "snapshots_pruned": deleted
    }

    if include_profile:
        out["profile_ms"] = {
            "validate": round((t_validate - t0) * 1000, 3),
            "write_snapshot": round((t_snapshot - t_validate) * 1000, 3),
            "write_latest": round((t_latest - t_snapshot) * 1000, 3),
            "prune": round((t_prune - t_latest) * 1000, 3),
            "total": round((t_prune - t0) * 1000, 3)
        }

    return out
