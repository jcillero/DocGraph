import json
import time
from pathlib import Path
from typing import Optional, Dict, Any

from system.core.runtime.paths import resolve_app_root, resolve_system_root
from system.tools.telemetry.product_telemetry import run_product_telemetry
from system.tools.telemetry.product_reporter import save_report


def load_spec(spec_path: Path) -> dict:
    with spec_path.open("r", encoding="utf-8") as f:
        return json.load(f)


def run(app_root: Path, spec_path: Optional[Path] = None) -> Dict[str, Any]:
    t0 = time.perf_counter()

    # normaliza raíz real de la app
    app_root = resolve_app_root(app_root)
    system_root = resolve_system_root(app_root)

    if spec_path is None:
        spec_path = system_root / "spec" / "telemetry" / "product_telemetry_spec.json"

    if not spec_path.exists():
        raise FileNotFoundError(f"Telemetry spec not found: {spec_path}")

    spec = load_spec(spec_path)
    t_spec = time.perf_counter()

    report = run_product_telemetry(app_root=app_root, spec_path=spec_path)
    t_telemetry = time.perf_counter()

    history_cfg = spec.get("history_policy", {})
    retain_last_n = int(history_cfg.get("retain_last_n_snapshots", 200))
    auto_prune = bool(history_cfg.get("auto_prune", True))

    save_info = save_report(
        app_root=app_root,
        report=report,
        retain_last_n=retain_last_n,
        auto_prune=auto_prune,
        include_profile=True,
    )
    t_save = time.perf_counter()

    profile_ms = {
        "load_spec": round((t_spec - t0) * 1000, 3),
        "run_telemetry": round((t_telemetry - t_spec) * 1000, 3),
        "save_report_total": round((t_save - t_telemetry) * 1000, 3),
        "total": round((t_save - t0) * 1000, 3),
    }

    reporter_profile = save_info.get("profile_ms")

    return {
        "status": "OK",
        "spec_path": str(spec_path),
        "snapshot_path": save_info.get("snapshot_path"),
        "latest_path": save_info.get("latest_path"),
        "retain_last_n_snapshots": retain_last_n,
        "auto_prune": auto_prune,
        "generated_at_utc": report.get("generated_at_utc"),
        "summary": report.get("summary", {}),
        "alerts_count": len(report.get("alerts", [])),
        "profile_ms": profile_ms,
        "reporter_profile_ms": reporter_profile,
    }