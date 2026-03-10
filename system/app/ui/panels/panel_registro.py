"""Panel Registro (stub)."""
from tkinter import ttk

def build(parent):
    frame = ttk.Frame(parent)
    ttk.Label(frame, text="Registro").pack(anchor="w", padx=12, pady=12)
    return frame
