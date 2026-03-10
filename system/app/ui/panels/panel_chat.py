"""Panel Chat (stub): árbol a la izquierda + chat a la derecha (placeholder)."""
from tkinter import ttk

def build(parent):
    root = ttk.Frame(parent)

    left = ttk.Frame(root)
    right = ttk.Frame(root)

    left.pack(side="left", fill="y")
    right.pack(side="right", fill="both", expand=True)

    ttk.Label(left, text="Árbol Área → Proyecto → Chat").pack(anchor="w", padx=12, pady=12)
    ttk.Label(right, text="Chat activo").pack(anchor="w", padx=12, pady=12)

    return root
