"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_ui_preferences_dialog.py",
  "tool_id": "tool_ui_preferences_dialog",
  "version": "1.0.0",
  "type": "runtime_tool",
  "category": "ui",
  "description": "Harness UI: abre el diálogo de Preferencias (tabs por categorías) para pruebas manuales.",
  "location_expected": "system/bin/tools/runtime",
  "entry_point": "main",
  "runtime_required": true,
  "modifies_system": false,
  "output_location": null,
  "outputs": [],
  "depends_on": [],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "DEV_RUNBOOK",
    "UI-ARCH-001_ui_dialogs_structure"
  ]
}
================================================================================
"""
from __future__ import annotations

import argparse
import tkinter as tk
from tkinter import ttk, messagebox
from pathlib import Path
from typing import Any, Dict, Tuple, Optional

from tool_common import resolve_user_registry_file, resolve_system_spec_file
from preferences_manager import PreferencesManager


SUPPORTED_CONTROLS = {"select", "checkbox", "slider"}


def _make_mgr(prefs_path: Path, catalog_path: Optional[Path]) -> PreferencesManager:
    if catalog_path is not None:
        return PreferencesManager(preferences_path=prefs_path, catalog_path=catalog_path)
    return PreferencesManager(
        preferences_path=prefs_path,
        catalog_resolver=lambda: resolve_system_spec_file("preferences_catalog.json"),
    )


def _load_catalog(path: Path) -> Dict[str, Any]:
    import json
    return json.loads(path.read_text(encoding="utf-8"))


def _get_spec_entries(catalog: Dict[str, Any]) -> Dict[str, Dict[str, Any]]:
    prefs = catalog.get("preferences", {})
    if not isinstance(prefs, dict):
        return {}
    out: Dict[str, Dict[str, Any]] = {}
    for k, spec in prefs.items():
        if not isinstance(k, str) or not isinstance(spec, dict):
            continue
        if not bool(spec.get("exposed", False)):
            continue
        ui_control = spec.get("ui_control", "none")
        if ui_control not in SUPPORTED_CONTROLS:
            continue
        out[k] = spec
    return out


def _pretty_label(key: str, spec: Dict[str, Any]) -> str:
    desc = spec.get("description")
    return desc if isinstance(desc, str) and desc.strip() else key


class PreferencesDialog(tk.Tk):
    def __init__(self, mgr: PreferencesManager, catalog: Dict[str, Any]) -> None:
        super().__init__()
        self.title("Preferencias (harness CLI)")
        self.geometry("760x520")
        self.minsize(680, 420)

        self._mgr = mgr
        self._catalog = catalog
        self._specs = _get_spec_entries(catalog)

        payload, status = self._mgr.load_or_repair()
        self._effective = payload.values
        self._status = status

        self._vars: Dict[str, Any] = {}  # key -> tk.Variable / (tk.DoubleVar, ttk.Scale)
        self._build_ui()
        self._populate_from_effective()

        if status.warnings:
            messagebox.showwarning("Avisos al cargar", "\n".join(status.warnings))

    def _build_ui(self) -> None:
        top = ttk.Frame(self, padding=12)
        top.pack(fill="both", expand=True)

        title = ttk.Label(top, text="Preferencias", font=("TkDefaultFont", 14, "bold"))
        title.pack(anchor="w")

        subtitle = ttk.Label(
            top,
            text="Este diálogo se lanza desde CLI para probar el pipeline catálogo → manager → storage.",
            wraplength=720,
        )
        subtitle.pack(anchor="w", pady=(4, 10))

        # Scrollable area
        container = ttk.Frame(top)
        container.pack(fill="both", expand=True)

        canvas = tk.Canvas(container, highlightthickness=0)
        scrollbar = ttk.Scrollbar(container, orient="vertical", command=canvas.yview)
        self._scroll_frame = ttk.Frame(canvas)

        self._scroll_frame.bind(
            "<Configure>",
            lambda e: canvas.configure(scrollregion=canvas.bbox("all")),
        )

        canvas.create_window((0, 0), window=self._scroll_frame, anchor="nw")
        canvas.configure(yscrollcommand=scrollbar.set)

        canvas.pack(side="left", fill="both", expand=True)
        scrollbar.pack(side="right", fill="y")

        # Build controls
        for key, spec in sorted(self._specs.items()):
            self._add_control(key, spec)

        # Buttons
        btns = ttk.Frame(top)
        btns.pack(fill="x", pady=(10, 0))

        ttk.Button(btns, text="Validar", command=self._on_validate).pack(side="left")
        ttk.Button(btns, text="Reset (backup)", command=self._on_reset).pack(side="left", padx=(8, 0))

        ttk.Button(btns, text="Cancelar", command=self.destroy).pack(side="right")
        ttk.Button(btns, text="Guardar", command=self._on_save).pack(side="right", padx=(0, 8))

        # Status line
        self._status_var = tk.StringVar(value="")
        ttk.Label(top, textvariable=self._status_var, foreground="#666666").pack(anchor="w", pady=(8, 0))

    def _add_control(self, key: str, spec: Dict[str, Any]) -> None:
        ui_control = spec.get("ui_control")
        row = ttk.Frame(self._scroll_frame, padding=(0, 6))
        row.pack(fill="x", expand=True)

        label = ttk.Label(row, text=_pretty_label(key, spec), wraplength=520)
        label.grid(row=0, column=0, sticky="w")

        help_txt = f"[{key}]"
        ttk.Label(row, text=help_txt, foreground="#777777").grid(row=1, column=0, sticky="w", pady=(2, 0))

        ctrl = ttk.Frame(row)
        ctrl.grid(row=0, column=1, rowspan=2, sticky="e", padx=(12, 0))

        if ui_control == "checkbox":
            var = tk.BooleanVar(value=False)
            self._vars[key] = var
            ttk.Checkbutton(ctrl, variable=var).pack(anchor="e")

        elif ui_control == "select":
            allowed = spec.get("allowed", [])
            if not isinstance(allowed, list):
                allowed = []
            var = tk.StringVar(value="")
            self._vars[key] = var
            combo = ttk.Combobox(ctrl, textvariable=var, values=allowed, state="readonly", width=18)
            combo.pack(anchor="e")

        elif ui_control == "slider":
            # For float/int ranges. We'll use DoubleVar and show value.
            r = spec.get("range", [0, 1])
            try:
                lo, hi = float(r[0]), float(r[1])
            except Exception:
                lo, hi = 0.0, 1.0

            var = tk.DoubleVar(value=0.0)
            self._vars[key] = var

            val_label = ttk.Label(ctrl, textvariable=var, width=6)
            val_label.pack(side="right", padx=(8, 0))

            scale = ttk.Scale(ctrl, from_=lo, to=hi, orient="horizontal", variable=var, length=180)
            scale.pack(side="right")

        else:
            # Shouldn't happen due to filtering.
            pass

        row.columnconfigure(0, weight=1)

    def _populate_from_effective(self) -> None:
        for key, spec in self._specs.items():
            default = spec.get("default")
            current = self._get_effective_value(key, default)
            v = self._vars.get(key)

            if isinstance(v, tk.BooleanVar):
                v.set(bool(current))
            elif isinstance(v, tk.StringVar):
                v.set("" if current is None else str(current))
            elif isinstance(v, tk.DoubleVar):
                try:
                    v.set(float(current))
                except Exception:
                    # fallback to default/range min
                    r = spec.get("range", [0, 1])
                    try:
                        v.set(float(r[0]))
                    except Exception:
                        v.set(0.0)

        self._status_var.set(f"Usando: {self._mgr.preferences_path()}")

    def _get_effective_value(self, dotted: str, fallback: Any) -> Any:
        cur: Any = self._effective
        for part in dotted.split("."):
            if not isinstance(cur, dict) or part not in cur:
                return fallback
            cur = cur[part]
        return cur

    def _collect_patch(self) -> Dict[str, Any]:
        patch: Dict[str, Any] = {}
        for key, var in self._vars.items():
            spec = self._specs[key]
            t = spec.get("type")

            if isinstance(var, tk.BooleanVar):
                patch[key] = bool(var.get())
            elif isinstance(var, tk.StringVar):
                patch[key] = str(var.get())
            elif isinstance(var, tk.DoubleVar):
                val = float(var.get())
                if t == "int":
                    patch[key] = int(round(val))
                else:
                    patch[key] = float(val)
            else:
                continue
        return patch

    def _on_validate(self) -> None:
        status = self._mgr.validate_only()
        if status.ok and not status.errors:
            msg = "OK ✅\n" + ("\n".join(status.warnings) if status.warnings else "Sin avisos.")
            messagebox.showinfo("Validación", msg)
        else:
            msg = "Errores:\n" + ("\n".join(status.errors) if status.errors else "(sin detalle)")
            if status.warnings:
                msg += "\n\nAvisos:\n" + "\n".join(status.warnings)
            messagebox.showerror("Validación", msg)

    def _on_reset(self) -> None:
        if not messagebox.askyesno("Reset", "¿Resetear preferencias (con backup)?"):
            return
        payload, status = self._mgr.reset_preferences(keep_backup=True)
        self._effective = payload.values
        self._populate_from_effective()
        if status.warnings:
            messagebox.showwarning("Reset", "\n".join(status.warnings))
        else:
            messagebox.showinfo("Reset", "Preferencias reseteadas (overrides vacíos).")

    def _on_save(self) -> None:
        patch = self._collect_patch()
        payload, status = self._mgr.set_preferences(patch)
        if status.ok and not status.errors:
            self._effective = payload.values
            self._populate_from_effective()
            if status.warnings:
                messagebox.showwarning("Guardado con avisos", "\n".join(status.warnings))
            else:
                messagebox.showinfo("Guardado", "Preferencias guardadas ✅")
        else:
            msg = "No se pudo guardar.\n\n" + ("\n".join(status.errors) if status.errors else "(sin detalle)")
            messagebox.showerror("Error", msg)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--prefs-path", type=str, default=None)
    ap.add_argument("--catalog-path", type=str, default=None)
    args = ap.parse_args()

    prefs_path = Path(args.prefs_path) if args.prefs_path else resolve_user_registry_file("preferences.json")
    catalog_path = Path(args.catalog_path) if args.catalog_path else resolve_system_spec_file("preferences_catalog.json")

    mgr = _make_mgr(prefs_path, catalog_path)
    catalog = _load_catalog(catalog_path)

    app = PreferencesDialog(mgr, catalog)
    app.mainloop()


if __name__ == "__main__":
    main()
