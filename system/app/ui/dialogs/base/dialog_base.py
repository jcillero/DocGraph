# system/app/ui/dialogs/base/dialog_base.py
from __future__ import annotations

import tkinter as tk
from tkinter import ttk
from pathlib import Path
from typing import Optional, Union


PathLike = Union[str, Path]


class DialogBase(tk.Toplevel):
    """
    Base visual común para los diálogos de la aplicación.

    Responsabilidades:
    - Crear una ventana modal uniforme
    - Mostrar header con icono + título + subtítulo opcional
    - Exponer content_frame y footer_host reutilizables
    - Gestionar centrado, tamaño mínimo y cierre estándar

    No incluye:
    - lógica de dominio
    - acceso a managers / tools / storage
    - persistencia
    """

    DEFAULT_MIN_WIDTH = 560
    DEFAULT_MIN_HEIGHT = 380

    def __init__(
        self,
        parent: tk.Misc,
        *,
        title_text: str,
        subtitle_text: str = "",
        app_name: str = "DocGraph",
        modal: bool = True,
        min_width: int = DEFAULT_MIN_WIDTH,
        min_height: int = DEFAULT_MIN_HEIGHT,
        resizable_x: bool = True,
        resizable_y: bool = True,
        window_icon_path: Optional[PathLike] = None,
        header_icon_path: Optional[PathLike] = None,
        header_fallback_text: str = "◼",
        header_padding: tuple[int, int, int, int] = (12, 10, 12, 10),
        body_padding: tuple[int, int, int, int] = (12, 10, 12, 12),
    ) -> None:
        super().__init__(parent)

        self.parent = parent
        self.app_name = app_name
        self.title_text = title_text
        self.subtitle_text = subtitle_text
        self.modal = modal
        self.header_fallback_text = header_fallback_text

        self._closed = False
        self._header_image: Optional[tk.PhotoImage] = None

        # Configuración general de ventana
        self.title(self._compose_window_title())
        self.minsize(min_width, min_height)
        self.resizable(resizable_x, resizable_y)
        self.protocol("WM_DELETE_WINDOW", self._on_window_close_requested)

        self._set_window_icon(window_icon_path)

        # Atajos
        self.bind("<Escape>", self._on_escape)

        # Layout raíz
        self.columnconfigure(0, weight=1)
        self.rowconfigure(2, weight=1)

        # --------------------------------------------------------------
        # Header
        # --------------------------------------------------------------
        self.header_frame = ttk.Frame(self, padding=header_padding)
        self.header_frame.grid(row=0, column=0, sticky="ew")
        self.header_frame.columnconfigure(1, weight=1)

        self._build_header(header_icon_path=header_icon_path)

        # --------------------------------------------------------------
        # Separador
        # --------------------------------------------------------------
        self.header_separator = ttk.Separator(self, orient="horizontal")
        self.header_separator.grid(row=1, column=0, sticky="ew")

        # --------------------------------------------------------------
        # Contenedor principal
        # --------------------------------------------------------------
        self.body_container = ttk.Frame(self, padding=body_padding)
        self.body_container.grid(row=2, column=0, sticky="nsew")
        self.body_container.columnconfigure(0, weight=1)
        self.body_container.rowconfigure(0, weight=1)
        self.body_container.rowconfigure(1, weight=0)

        # Área pública principal para contenido
        self.content_frame = ttk.Frame(self.body_container)
        self.content_frame.grid(row=0, column=0, sticky="nsew")
        self.content_frame.columnconfigure(0, weight=1)
        self.content_frame.rowconfigure(0, weight=1)

        # Área pública para footer/botoneras/estado
        self.footer_host = ttk.Frame(self.body_container)
        self.footer_host.grid(row=1, column=0, sticky="ew")
        self.footer_host.columnconfigure(0, weight=1)

        self.update_idletasks()
        self._center_over_parent()

        if self.modal:
            self.transient(parent)
            self.grab_set()

    # ------------------------------------------------------------------
    # Header
    # ------------------------------------------------------------------
    def _build_header(self, *, header_icon_path: Optional[PathLike]) -> None:
        self.header_icon_label = ttk.Label(self.header_frame)
        self.header_icon_label.grid(row=0, column=0, rowspan=2, sticky="nw", padx=(0, 10))

        image = self._load_header_image(header_icon_path)
        if image is not None:
            self.header_icon_label.configure(image=image)
            self._header_image = image
        else:
            self.header_icon_label.configure(text=self.header_fallback_text, anchor="center")

        self.title_var = tk.StringVar(value=self.title_text)
        self.subtitle_var = tk.StringVar(value=self.subtitle_text)

        self.title_label = ttk.Label(
            self.header_frame,
            textvariable=self.title_var,
            font=("", 11, "bold"),
            anchor="w",
        )
        self.title_label.grid(row=0, column=1, sticky="ew")

        self.subtitle_label = ttk.Label(
            self.header_frame,
            textvariable=self.subtitle_var,
            anchor="w",
        )
        self.subtitle_label.grid(row=1, column=1, sticky="ew", pady=(2, 0))

        if not self.subtitle_text:
            self.subtitle_label.grid_remove()

    def set_header_title(self, text: str) -> None:
        self.title_text = text
        self.title_var.set(text)
        self.title(self._compose_window_title())

    def set_header_subtitle(self, text: str) -> None:
        self.subtitle_text = text
        self.subtitle_var.set(text)

        if text:
            self.subtitle_label.grid()
        else:
            self.subtitle_label.grid_remove()

    def set_header_icon(self, header_icon_path: Optional[PathLike]) -> None:
        image = self._load_header_image(header_icon_path)
        if image is not None:
            self.header_icon_label.configure(image=image, text="")
            self._header_image = image
        else:
            self.header_icon_label.configure(image="", text=self.header_fallback_text)
            self._header_image = None

    # ------------------------------------------------------------------
    # Comportamiento de ventana
    # ------------------------------------------------------------------
    def show(self) -> None:
        """
        Hace visible el diálogo y espera a su cierre si es modal.
        """
        self.deiconify()
        self.lift()
        self.focus_force()

        if self.modal:
            self.wait_window(self)

    def on_close_requested(self) -> bool:
        """
        Hook para subclases.

        Devuelve:
        - True  -> permitir cierre
        - False -> bloquear cierre
        """
        return True

    def close(self) -> None:
        if self._closed:
            return

        self._closed = True

        try:
            if self.modal:
                self.grab_release()
        except tk.TclError:
            pass

        self.destroy()

    def _on_window_close_requested(self) -> None:
        if not self.on_close_requested():
            return
        self.close()

    def _on_escape(self, _event: tk.Event) -> None:
        self._on_window_close_requested()

    # ------------------------------------------------------------------
    # Utilidades de layout / geometría
    # ------------------------------------------------------------------
    def _compose_window_title(self) -> str:
        return f"{self.app_name} — {self.title_text}"

    def _center_over_parent(self) -> None:
        """
        Centra el diálogo sobre la ventana padre si es posible.
        """
        try:
            self.update_idletasks()

            parent = self.parent

            pw = max(parent.winfo_width(), 1)
            ph = max(parent.winfo_height(), 1)
            px = parent.winfo_rootx()
            py = parent.winfo_rooty()

            w = max(self.winfo_reqwidth(), self.winfo_width(), self.DEFAULT_MIN_WIDTH)
            h = max(self.winfo_reqheight(), self.winfo_height(), self.DEFAULT_MIN_HEIGHT)

            x = px + max((pw - w) // 2, 0)
            y = py + max((ph - h) // 2, 0)

            self.geometry(f"{w}x{h}+{x}+{y}")
        except Exception:
            # No bloqueamos la UI por problemas de geometría.
            pass

    # ------------------------------------------------------------------
    # Iconos
    # ------------------------------------------------------------------
    def _set_window_icon(self, window_icon_path: Optional[PathLike]) -> None:
        """
        Establece el icono de la ventana si el fichero existe y es compatible.

        - .ico -> iconbitmap (Windows)
        - .png/.gif -> iconphoto (si Tk lo soporta bien en el entorno)
        """
        if not window_icon_path:
            return

        try:
            path = Path(window_icon_path)
            if not path.exists():
                return

            suffix = path.suffix.lower()

            if suffix == ".ico":
                self.iconbitmap(str(path))
                return

            if suffix in {".png", ".gif"}:
                image = tk.PhotoImage(file=str(path))
                self.iconphoto(True, image)
                # Mantener referencia por seguridad
                self._header_image = self._header_image or image
                return

        except (OSError, tk.TclError):
            # Nunca bloqueamos un diálogo por un icono.
            pass

    def _load_header_image(self, header_icon_path: Optional[PathLike]) -> Optional[tk.PhotoImage]:
        """
        Carga una imagen para el header si existe y es compatible con PhotoImage.
        """
        if not header_icon_path:
            return None

        try:
            path = Path(header_icon_path)
            if not path.exists():
                return None

            if path.suffix.lower() not in {".png", ".gif"}:
                return None

            return tk.PhotoImage(file=str(path))

        except (OSError, tk.TclError):
            return None