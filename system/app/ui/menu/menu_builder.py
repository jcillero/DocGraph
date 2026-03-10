from __future__ import annotations

import tkinter as tk
from tkinter import messagebox


def build_main_menu(root, *, preferences_controller) -> tk.Menu:
    menu_bar = tk.Menu(root)

    # ---------------------------------------------------------
    # File
    # ---------------------------------------------------------
    file_menu = tk.Menu(menu_bar, tearoff=0)
    file_menu.add_command(label="Nueva área")
    file_menu.add_command(label="Nuevo proyecto")
    file_menu.add_separator()
    file_menu.add_command(label="Salir", command=root.destroy)
    menu_bar.add_cascade(label="Archivo", menu=file_menu)

    # ---------------------------------------------------------
    # Preferences
    # ---------------------------------------------------------
    preferences_menu = tk.Menu(menu_bar, tearoff=0)
    preferences_menu.add_command(
        label="Abrir preferencias...",
        command=lambda: preferences_controller.open_dialog(root),
    )
    menu_bar.add_cascade(label="Preferencias", menu=preferences_menu)

    # ---------------------------------------------------------
    # Tools
    # ---------------------------------------------------------
    tools_menu = tk.Menu(menu_bar, tearoff=0)
    tools_menu.add_command(
        label="Diagnóstico estructural",
        command=lambda: messagebox.showinfo("Herramientas", "Pendiente de integrar."),
    )
    menu_bar.add_cascade(label="Herramientas", menu=tools_menu)

    # ---------------------------------------------------------
    # Help
    # ---------------------------------------------------------
    help_menu = tk.Menu(menu_bar, tearoff=0)
    help_menu.add_command(
        label="Acerca de",
        command=lambda: messagebox.showinfo("DocGraph", "DocGraph — Portable workspace"),
    )
    menu_bar.add_cascade(label="Ayuda", menu=help_menu)

    return menu_bar