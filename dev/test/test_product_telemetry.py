import json
import unittest
import tempfile
from pathlib import Path

from product_telemetry import run_product_telemetry
from product_reporter import save_report


def _write_bytes(path: Path, n: int) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(b"x" * n)


class TestProductTelemetry(unittest.TestCase):
    def setUp(self) -> None:
        self._tmpdir = tempfile.TemporaryDirectory()
        self.app_root = Path(self._tmpdir.name)

        # Create expected zone roots
        (self.app_root / "user" / "runs").mkdir(parents=True, exist_ok=True)
        (self.app_root / "user" / "outputs").mkdir(parents=True, exist_ok=True)
        (self.app_root / "user" / "registry").mkdir(parents=True, exist_ok=True)

        # Create spec file in a temp "system/spec/telemetry/" to mimic real layout
        spec_dir = self.app_root / "system" / "spec" / "telemetry"
        spec_dir.mkdir(parents=True, exist_ok=True)
        self.spec_path = spec_dir / "product_telemetry_spec.json"

        spec = {
            "spec_version": "1.0.0",
            "scan_zones": {
                "runs": {"relative_path": "user/runs", "enabled": True},
                "outputs": {"relative_path": "user/outputs", "enabled": True},
                "registry": {"relative_path": "user/registry", "enabled": True}
            },
            "metrics": {
                "compute_by_extension": True,
                "compute_largest_files": True,
                "compute_largest_folders": True,
                "top_n_largest_files": 20,
                "top_n_largest_folders": 20
            },
            "alerts": {
                "detect_outputs_inside_runs": True,
                "detect_files_outside_known_zones": True,
                "detect_empty_registry_documents": True,
                "detect_large_single_file_threshold_bytes": 500000000
            },
            "history_policy": {
                "retain_last_n_snapshots": 200,
                "auto_prune": False
            }
        }

        self.spec_path.write_text(json.dumps(spec, indent=2), encoding="utf-8")

    def tearDown(self) -> None:
        self._tmpdir.cleanup()

    def test_report_has_required_structure(self) -> None:
        # Arrange: create a few files with known sizes
        _write_bytes(self.app_root / "user" / "runs" / "a.log", 10)
        _write_bytes(self.app_root / "user" / "outputs" / "out.pdf", 200)
        _write_bytes(self.app_root / "user" / "registry" / "bibtex_snapshot.jsonl", 50)

        # Act
        report = run_product_telemetry(app_root=self.app_root, spec_path=self.spec_path)

        # Assert: required top-level keys
        for k in ("schema_version", "generated_at_utc", "summary", "zones", "alerts"):
            self.assertIn(k, report, f"Missing key '{k}' in report")

        # Assert: zones exist
        self.assertIn("runs", report["zones"])
        self.assertIn("outputs", report["zones"])
        self.assertIn("registry", report["zones"])

        # Assert: counts and sizes should match our test setup
        self.assertEqual(report["zones"]["runs"]["file_count"], 1)
        self.assertEqual(report["zones"]["outputs"]["file_count"], 1)
        self.assertEqual(report["zones"]["registry"]["file_count"], 1)

        self.assertEqual(report["zones"]["runs"]["size_bytes"], 10)
        self.assertEqual(report["zones"]["outputs"]["size_bytes"], 200)
        self.assertEqual(report["zones"]["registry"]["size_bytes"], 50)

        self.assertEqual(report["summary"]["total_files"], 3)
        self.assertEqual(report["summary"]["total_size_bytes"], 260)

    def test_largest_files_contains_biggest(self) -> None:
        # Arrange
        _write_bytes(self.app_root / "user" / "outputs" / "small.txt", 5)
        _write_bytes(self.app_root / "user" / "outputs" / "big.bin", 999)

        # Act
        report = run_product_telemetry(app_root=self.app_root, spec_path=self.spec_path)

        # Assert
        largest = report.get("largest_files", [])
        self.assertTrue(len(largest) >= 1, "largest_files should contain at least one entry")
        self.assertEqual(largest[0]["size_bytes"], 999, "Top largest file size should be 999 bytes")
        self.assertTrue(
            str(largest[0]["relative_path"]).endswith("user/outputs/big.bin"),
            "Top largest file path should point to user/outputs/big.bin"
        )

    def test_save_report_writes_snapshot_and_latest(self) -> None:
        # Arrange
        _write_bytes(self.app_root / "user" / "outputs" / "out.pdf", 123)
        report = run_product_telemetry(app_root=self.app_root, spec_path=self.spec_path)

        # Act
        info = save_report(app_root=self.app_root, report=report, retain_last_n=10)

        # Assert
        snapshot_path = Path(info["snapshot_path"])
        latest_path = Path(info["latest_path"])

        self.assertTrue(snapshot_path.exists(), "Snapshot report.json must exist")
        self.assertTrue(latest_path.exists(), "Latest report.json must exist")

        latest = json.loads(latest_path.read_text(encoding="utf-8"))
        self.assertIn("summary", latest)
        self.assertEqual(latest["summary"]["total_files"], report["summary"]["total_files"])
        self.assertEqual(latest["summary"]["total_size_bytes"], report["summary"]["total_size_bytes"])


if __name__ == "__main__":
    unittest.main()