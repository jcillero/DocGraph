"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "tool_ui_credentials_dialog.py",
  "tool_id": "tool_ui_credentials_dialog",
  "version": "1.0.0",
  "type": "runtime_tool",
  "category": "ui",
  "description": "Harness UI: abre el diálogo de Credenciales para pruebas manuales (sin integrar aún en menú principal).",
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
from typing import Any, Dict, Optional, Tuple

from tool_common import resolve_user_registry_file
from credentials_manager import CredentialsManager, KNOWN_FIELDS, KNOWN_PROVIDERS


# Heuristic: fields to hide by default in UI
SECRET_FIELD_NAMES = {"api_key", "token", "secret", "password"}


def _make_mgr(credentials_path: Path) -> CredentialsManager:
    return CredentialsManager(credentials_path=credentials_path)


def _pretty_provider(p: str) -> str:
    return p.replace("_", " ").title()


class CredentialsDialog(tk.Tk):
    def __init__(self, mgr: CredentialsManager) -> None:
        super().__init__()
        self.title("Credenciales (harness CLI)")
        self.geometry("760x460")
        self.minsize(680, 420)

        self._mgr = mgr
        payload, status = self._mgr.load_or_repair()
        self._payload = payload
        self._status = status

        self._provider_var = tk.StringVar(value="openai")
        self._show_secrets_var = tk.BooleanVar(value=False)

        # key -> (StringVar, EntryWidget, is_secret)
        self._field_vars: Dict[str, Tuple[tk.StringVar, ttk.Entry, bool]] = {}

        self._build_ui()

        if status.warnings:
            messagebox.showwarning("Avisos al cargar", "\n".join(status.warnings))

        # Populate initial provider
        if self._provider_var.get() not in self._payload.providers:
            # pick first known provider present
            for p in KNOWN_PROVIDERS:
                if p in self._payload.providers:
                    self._provider_var.set(p)
                    break
        self._load_provider_into_form(self._provider_var.get())

    def _build_ui(self) -> None:
        root = ttk.Frame(self, padding=12)
        root.pack(fill="both", expand=True)

        title = ttk.Label(root, text="Credenciales", font=("TkDefaultFont", 14, "bold"))
        title.pack(anchor="w")

        subtitle = ttk.Label(
            root,
            text="Diálogo lanzado desde CLI para probar credentials_manager y el storage en user/registry.",
            wraplength=720,
        )
        subtitle.pack(anchor="w", pady=(4, 10))

        # Top row: provider select + show secrets
        top = ttk.Frame(root)
        top.pack(fill="x")

        ttk.Label(top, text="Proveedor:").pack(side="left")

        providers = [p for p in KNOWN_PROVIDERS if p in self._payload.providers]
        if not providers:
            providers = sorted(list(self._payload.providers.keys()))

        self._provider_combo = ttk.Combobox(top, textvariable=self._provider_var, values=providers, state="readonly", width=22)
        self._provider_combo.pack(side="left", padx=(8, 18))
        self._provider_combo.bind("<<ComboboxSelected>>", lambda e: self._load_provider_into_form(self._provider_var.get()))

        ttk.Checkbutton(
            top,
            text="Mostrar secretos",
            variable=self._show_secrets_var,
            command=self._apply_secret_visibility,
        ).pack(side="left")

        # Form area (scrollable)
        container = ttk.Frame(root)
        container.pack(fill="both", expand=True, pady=(10, 0))

        canvas = tk.Canvas(container, highlightthickness=0)
        scrollbar = ttk.Scrollbar(container, orient="vertical", command=canvas.yview)

        self._form = ttk.Frame(canvas, padding=(0, 0, 6, 0))
        self._form.bind("<Configure>", lambda e: canvas.configure(scrollregion=canvas.bbox("all")))

        canvas.create_window((0, 0), window=self._form, anchor="nw")
        canvas.configure(yscrollcommand=scrollbar.set)

        canvas.pack(side="left", fill="both", expand=True)
        scrollbar.pack(side="right", fill="y")

        # Buttons
        btns = ttk.Frame(root)
        btns.pack(fill="x", pady=(12, 0))

        ttk.Button(btns, text="Reset (backup)", command=self._on_reset_all).pack(side="left")
        ttk.Button(btns, text="Recargar", command=self._on_reload).pack(side="left", padx=(8, 0))

        ttk.Button(btns, text="Cancelar", command=self.destroy).pack(side="right")
        ttk.Button(btns, text="Guardar", command=self._on_save).pack(side="right", padx=(0, 8))

        self._status_var = tk.StringVar(value="")
        ttk.Label(root, textvariable=self._status_var, foreground="#666666").pack(anchor="w", pady=(8, 0))
        self._status_var.set(f"Usando: {self._mgr.credentials_path()}")

    def _clear_form(self) -> None:
        for child in list(self._form.winfo_children()):
            child.destroy()
        self._field_vars.clear()

    def _fields_for_provider(self, provider: str) -> Tuple[str, ...]:
        # Prefer KNOWN_FIELDS ordering, otherwise show keys present in payload
        known = KNOWN_FIELDS.get(provider)
        if known:
            return tuple(known)
        p = self._payload.providers.get(provider, {})
        if isinstance(p, dict):
            return tuple(sorted(p.keys()))
        return tuple()

    def _load_provider_into_form(self, provider: str) -> None:
        self._clear_form()

        data = self._mgr.get_provider(provider, masked=False)
        fields = self._fields_for_provider(provider)

        # If provider dict has extra keys (even if unknown), include them too
        extra = [k for k in data.keys() if k not in fields]
        all_fields = list(fields) + sorted(extra)

        if not all_fields:
            ttk.Label(self._form, text=f"No hay campos para provider '{provider}'.").grid(row=0, column=0, sticky="w")
            return

        for r, field in enumerate(all_fields):
            is_secret = field in SECRET_FIELD_NAMES
            label = ttk.Label(self._form, text=field)
            label.grid(row=r, column=0, sticky="w", padx=(0, 12), pady=6)

            var = tk.StringVar(value=str(data.get(field, "")) if data.get(field) is not None else "")
            entry = ttk.Entry(self._form, textvariable=var, width=48)
            entry.grid(row=r, column=1, sticky="we", pady=6)

            # secret masking via 'show' config
            if is_secret and not self._show_secrets_var.get():
                entry.configure(show="•")

            self._field_vars[field] = (var, entry, is_secret)

        self._form.columnconfigure(1, weight=1)
        self._apply_secret_visibility()

    def _apply_secret_visibility(self) -> None:
        show = self._show_secrets_var.get()
        for _, (var, entry, is_secret) in self._field_vars.items():
            if is_secret:
                entry.configure(show="" if show else "•")

    def _collect_fields(self) -> Dict[str, str]:
        out: Dict[str, str] = {}
        for field, (var, _entry, _is_secret) in self._field_vars.items():
            out[field] = str(var.get())
        return out

    def _on_reload(self) -> None:
        payload, status = self._mgr.load_or_repair()
        self._payload = payload
        self._status = status
        self._load_provider_into_form(self._provider_var.get())
        if status.warnings:
            messagebox.showwarning("Avisos", "\n".join(status.warnings))

    def _on_save(self) -> None:
        provider = self._provider_var.get()
        fields = self._collect_fields()

        payload, status = self._mgr.set_provider_fields(provider, fields)

        self._payload = payload
        self._status = status

        if status.ok and not status.errors:
            if status.warnings:
                messagebox.showwarning("Guardado con avisos", "\n".join(status.warnings))
            else:
                messagebox.showinfo("Guardado", "Credenciales guardadas ✅")
        else:
            msg = "No se pudo guardar.\n\n" + ("\n".join(status.errors) if status.errors else "(sin detalle)")
            messagebox.showerror("Error", msg)

    def _on_reset_all(self) -> None:
        if not messagebox.askyesno("Reset", "¿Resetear TODAS las credenciales (con backup)?"):
            return
        payload, status = self._mgr.reset_credentials(keep_backup=True)
        self._payload = payload
        self._status = status
        self._load_provider_into_form(self._provider_var.get())
        if status.warnings:
            messagebox.showwarning("Reset", "\n".join(status.warnings))
        else:
            messagebox.showinfo("Reset", "Credenciales reseteadas (plantilla vacía).")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--credentials-path", type=str, default=None)
    args = ap.parse_args()

    cred_path = Path(args.credentials_path) if args.credentials_path else resolve_user_registry_file("credentials.json")
    mgr = _make_mgr(cred_path)

    app = CredentialsDialog(mgr)
    app.mainloop()


if __name__ == "__main__":
    main()
