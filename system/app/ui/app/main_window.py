from __future__ import annotations

import tkinter as tk
from pathlib import Path
from tkinter import ttk

from system.app.core.preferences.preferences_controller import PreferencesController
from system.app.ui.menu.menu_builder import build_main_menu


class MainWindow(tk.Tk):
    def __init__(self, *, app_root: str | Path) -> None:
        super().__init__()

        self.app_root = Path(app_root)

        self.title("DocGraph")
        self.geometry("1200x800")
        self.minsize(960, 640)

        self._configure_layout()

        # ---------------------------------------------------------
        # Controllers
        # ---------------------------------------------------------
        self.preferences_controller = PreferencesController(
            app_root=self.app_root,
        )

        # ---------------------------------------------------------
        # Menu
        # ---------------------------------------------------------
        menu_bar = build_main_menu(
            self,
            preferences_controller=self.preferences_controller,
        )
        self.config(menu=menu_bar)

        # ---------------------------------------------------------
        # MVP shell placeholders
        # ---------------------------------------------------------
        self._build_shell_placeholders()

    def _configure_layout(self) -> None:
        self.grid_rowconfigure(0, weight=1)
        self.grid_columnconfigure(0, weight=0)  # tree
        self.grid_columnconfigure(1, weight=1)  # center
        self.grid_columnconfigure(2, weight=0)  # context

    def _build_shell_placeholders(self) -> None:
        left = ttk.Frame(self, padding=8)
        center = ttk.Frame(self, padding=8)
        right = ttk.Frame(self, padding=8)
        bottom = ttk.Frame(self, padding=8)

        left.grid(row=0, column=0, sticky="nsew")
        center.grid(row=0, column=1, sticky="nsew")
        right.grid(row=0, column=2, sticky="nsew")
        bottom.grid(row=1, column=0, columnspan=3, sticky="ew")

        left.configure(width=220)
        right.configure(width=240)

        ttk.Label(left, text="WorkspaceTree").pack(anchor="nw")
        ttk.Label(center, text="Tabbed Workspace Host").pack(anchor="nw")
        ttk.Label(right, text="Context Panel").pack(anchor="nw")
        ttk.Label(bottom, text="Global Input / Log").pack(anchor="nw")