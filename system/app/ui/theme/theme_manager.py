"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "theme_manager.py",
  "version": "0.1.0",
  "type": "core_module",
  "category": "ui",
  "description": "Minimal theme manager for applying light/dark ttk themes in DocGraph.",
  "location_expected": "system/app/ui/theme/",
  "entry_point": "ThemeManager",
  "inputs": [
    "root widget",
    "theme name"
  ],
  "outputs": [
    "configured ttk.Style"
  ]
}
================================================================================
"""

from __future__ import annotations

import tkinter as tk
from tkinter import ttk
from typing import Any


class ThemeManager:
    """
    Minimal runtime theme manager for DocGraph.

    Responsibilities:
    - apply light/dark theme tokens to ttk widgets
    - configure a stable base style for the application
    - expose current theme state

    Non-responsibilities:
    - persistence
    - preferences loading
    - advanced token merging from external specs
    """

    THEMES: dict[str, dict[str, str]] = {
        "light": {
            "bg": "#F2F2F2",
            "panel": "#FFFFFF",
            "fg": "#1A1A1A",
            "muted": "#5F6368",
            "border": "#CFCFCF",
            "accent": "#2F6FED",
            "input_bg": "#FFFFFF",
            "input_fg": "#111111",
            "button_bg": "#EAEAEA",
            "button_fg": "#111111",
            "tab_bg": "#E8E8E8",
            "tab_active_bg": "#FFFFFF",
            "tab_fg": "#111111",
        },
        "dark": {
            "bg": "#1E1E1E",
            "panel": "#252526",
            "fg": "#F3F3F3",
            "muted": "#B8B8B8",
            "border": "#3A3A3A",
            "accent": "#4C8DFF",
            "input_bg": "#2D2D30",
            "input_fg": "#F3F3F3",
            "button_bg": "#2D2D30",
            "button_fg": "#F3F3F3",
            "tab_bg": "#2A2A2A",
            "tab_active_bg": "#252526",
            "tab_fg": "#F3F3F3",
        },
    }

    def __init__(self, root: tk.Misc) -> None:
        self.root = root
        self.style = ttk.Style(root)
        self.current_theme = "light"

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------
    def apply_theme(self, theme_name: str | None) -> str:
        """
        Apply a named theme to the application.

        Returns the effective theme name actually applied.
        """
        theme = str(theme_name or "light").strip().lower()
        if theme not in self.THEMES:
            theme = "light"

        self._use_best_base_theme()
        self._apply_tokens(self.THEMES[theme])
        self.current_theme = theme
        return theme

    def get_current_theme(self) -> str:
        return self.current_theme

    # ------------------------------------------------------------------
    # Internal helpers
    # ------------------------------------------------------------------
    def _use_best_base_theme(self) -> None:
        """
        Select a stable ttk base theme before overriding style tokens.
        """
        available = set(self.style.theme_names())

        # vista gives decent native widgets on Windows
        for candidate in ("vista", "clam", "default"):
            if candidate in available:
                self.style.theme_use(candidate)
                return

    def _apply_tokens(self, t: dict[str, str]) -> None:
        # Root / classic widgets
        try:
            self.root.configure(bg=t["bg"])
        except Exception:
            pass

        # Generic ttk defaults
        self.style.configure(
            ".",
            background=t["bg"],
            foreground=t["fg"],
            fieldbackground=t["input_bg"],
            bordercolor=t["border"],
            lightcolor=t["border"],
            darkcolor=t["border"],
            troughcolor=t["panel"],
            focuscolor=t["accent"],
        )

        # Frames / containers
        self.style.configure("TFrame", background=t["bg"])
        self.style.configure("Card.TFrame", background=t["panel"])
        self.style.configure("Panel.TFrame", background=t["panel"])

        # Labels
        self.style.configure("TLabel", background=t["bg"], foreground=t["fg"])
        self.style.configure("Muted.TLabel", background=t["bg"], foreground=t["muted"])
        self.style.configure("Header.TLabel", background=t["bg"], foreground=t["fg"])

        # Buttons
        self.style.configure(
            "TButton",
            background=t["button_bg"],
            foreground=t["button_fg"],
            bordercolor=t["border"],
            focusthickness=1,
            focuscolor=t["accent"],
            padding=(10, 4),
        )
        self.style.map(
            "TButton",
            background=[
                ("disabled", t["panel"]),
                ("pressed", t["accent"]),
                ("active", t["panel"]),
            ],
            foreground=[
                ("disabled", t["muted"]),
                ("pressed", "#FFFFFF" if self.current_theme == "dark" else "#FFFFFF"),
                ("active", t["fg"]),
            ],
        )

        # Entries / Comboboxes
        self.style.configure(
            "TEntry",
            fieldbackground=t["input_bg"],
            foreground=t["input_fg"],
            insertcolor=t["input_fg"] if "insertcolor" in self.style.configure("TEntry") else t["input_fg"],
            bordercolor=t["border"],
            padding=4,
        )

        self.style.configure(
            "TCombobox",
            fieldbackground=t["input_bg"],
            foreground=t["input_fg"],
            background=t["input_bg"],
            bordercolor=t["border"],
            arrowcolor=t["fg"],
            padding=4,
        )
        self.style.map(
            "TCombobox",
            fieldbackground=[
                ("readonly", t["input_bg"]),
                ("disabled", t["panel"]),
            ],
            foreground=[
                ("readonly", t["input_fg"]),
                ("disabled", t["muted"]),
            ],
            arrowcolor=[
                ("disabled", t["muted"]),
                ("readonly", t["fg"]),
            ],
        )

        # Checkbutton
        self.style.configure(
            "TCheckbutton",
            background=t["bg"],
            foreground=t["fg"],
            focuscolor=t["accent"],
        )
        self.style.map(
            "TCheckbutton",
            background=[("active", t["bg"])],
            foreground=[("disabled", t["muted"])],
        )

        # Notebook
        self.style.configure(
            "TNotebook",
            background=t["bg"],
            bordercolor=t["border"],
            tabmargins=(2, 2, 2, 0),
        )
        self.style.configure(
            "TNotebook.Tab",
            background=t["tab_bg"],
            foreground=t["tab_fg"],
            padding=(10, 4),
            bordercolor=t["border"],
        )
        self.style.map(
            "TNotebook.Tab",
            background=[
                ("selected", t["tab_active_bg"]),
                ("active", t["panel"]),
            ],
            foreground=[
                ("selected", t["fg"]),
                ("active", t["fg"]),
            ],
        )

        # Separator
        self.style.configure("TSeparator", background=t["border"])

        # Treeview
        self.style.configure(
            "Treeview",
            background=t["panel"],
            fieldbackground=t["panel"],
            foreground=t["fg"],
            bordercolor=t["border"],
        )
        self.style.map(
            "Treeview",
            background=[("selected", t["accent"])],
            foreground=[("selected", "#FFFFFF")],
        )
        self.style.configure(
            "Treeview.Heading",
            background=t["tab_bg"],
            foreground=t["fg"],
            bordercolor=t["border"],
        )

        # Scale
        self.style.configure(
            "Horizontal.TScale",
            background=t["bg"],
            troughcolor=t["panel"],
            bordercolor=t["border"],
        )

        # Labelframe
        self.style.configure(
            "TLabelframe",
            background=t["bg"],
            bordercolor=t["border"],
        )
        self.style.configure(
            "TLabelframe.Label",
            background=t["bg"],
            foreground=t["fg"],
        )

        # Scrollbar
        self.style.configure(
            "Vertical.TScrollbar",
            background=t["panel"],
            troughcolor=t["bg"],
            bordercolor=t["border"],
            arrowcolor=t["fg"],
        )
        self.style.configure(
            "Horizontal.TScrollbar",
            background=t["panel"],
            troughcolor=t["bg"],
            bordercolor=t["border"],
            arrowcolor=t["fg"],
        )