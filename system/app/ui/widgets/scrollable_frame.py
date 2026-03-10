import tkinter as tk
from tkinter import ttk


class ScrollableFrame(ttk.Frame):

    def __init__(self, parent):
        super().__init__(parent)

        canvas = tk.Canvas(self, borderwidth=0)
        vscroll = ttk.Scrollbar(self, orient="vertical", command=canvas.yview)
        hscroll = ttk.Scrollbar(self, orient="horizontal", command=canvas.xview)

        self.scrollable_frame = ttk.Frame(canvas)

        self.scrollable_frame.bind(
            "<Configure>",
            lambda e: canvas.configure(
                scrollregion=canvas.bbox("all")
            )
        )

        canvas.create_window((0, 0), window=self.scrollable_frame, anchor="nw")

        canvas.configure(
            yscrollcommand=vscroll.set,
            xscrollcommand=hscroll.set
        )

        canvas.grid(row=0, column=0, sticky="nsew")
        vscroll.grid(row=0, column=1, sticky="ns")
        hscroll.grid(row=1, column=0, sticky="ew")

        self.grid_rowconfigure(0, weight=1)
        self.grid_columnconfigure(0, weight=1)