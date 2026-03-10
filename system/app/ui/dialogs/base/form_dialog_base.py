from __future__ import annotations

import tkinter as tk
from tkinter import messagebox, ttk
from typing import Any, Callable, Optional

try:
    from system.app.ui.dialogs.base.dialog_base import DialogBase, PathLike
except ImportError:  # pragma: no cover - fallback for local harness/testing
    from dialog_base import DialogBase, PathLike  # type: ignore


UIResolver = Callable[..., str]


class FormDialogBase(DialogBase):
    """
    Base común para diálogos tipo formulario.

    Aporta:
    - línea de estado no bloqueante
    - botones OK / Apply / Cancel
    - dirty state con Apply habilitado solo si hay cambios
    - hooks de carga / recogida / validación / aplicación
    - confirmación opcional al cancelar con cambios sin guardar
    """

    def __init__(
        self,
        parent: tk.Misc,
        *,
        title_text: str,
        subtitle_text: str = "",
        app_name: str = "DocGraph",
        modal: bool = True,
        min_width: int = 640,
        min_height: int = 480,
        resizable_x: bool = True,
        resizable_y: bool = True,
        window_icon_path: Optional[PathLike] = None,
        header_icon_path: Optional[PathLike] = None,
        ok_text: Optional[str] = None,
        apply_text: Optional[str] = None,
        cancel_text: Optional[str] = None,
        show_apply: bool = True,
        enter_confirms: bool = True,
        confirm_discard_on_cancel: bool = False,
        ui: Optional[UIResolver] = None,
    ) -> None:
        super().__init__(
            parent,
            title_text=title_text,
            subtitle_text=subtitle_text,
            app_name=app_name,
            modal=modal,
            min_width=min_width,
            min_height=min_height,
            resizable_x=resizable_x,
            resizable_y=resizable_y,
            window_icon_path=window_icon_path,
            header_icon_path=header_icon_path,
        )

        self.ui = ui
        self.show_apply = show_apply
        self.enter_confirms = enter_confirms
        self.confirm_discard_on_cancel = confirm_discard_on_cancel
        self._dirty = False
        self._dirty_trace_tokens: list[tuple[tk.Variable, str]] = []

        self.content_frame.columnconfigure(0, weight=1)
        self.content_frame.rowconfigure(0, weight=1)

        self.footer_host.columnconfigure(0, weight=1)
        self.footer_host.rowconfigure(0, weight=0)
        self.footer_host.rowconfigure(1, weight=0)
        self.footer_host.rowconfigure(2, weight=0)

        self.status_var = tk.StringVar(value="")
        self.status_label = ttk.Label(
            self.footer_host,
            textvariable=self.status_var,
            anchor="w",
        )
        self.status_label.grid(row=0, column=0, sticky="ew", pady=(0, 8))

        self.footer_separator = ttk.Separator(self.footer_host, orient="horizontal")
        self.footer_separator.grid(row=1, column=0, sticky="ew", pady=(0, 8))

        self.buttons_frame = ttk.Frame(self.footer_host)
        self.buttons_frame.grid(row=2, column=0, sticky="e")

        resolved_ok = ok_text or self._ui("common.button.ok")
        resolved_apply = apply_text or self._ui("common.button.apply")
        resolved_cancel = cancel_text or self._ui("common.button.cancel")

        self.ok_button = ttk.Button(
            self.buttons_frame,
            text=resolved_ok,
            command=self.on_ok_clicked,
        )
        self.ok_button.grid(row=0, column=0, padx=(0, 8))

        self.apply_button: Optional[ttk.Button]
        if self.show_apply:
            self.apply_button = ttk.Button(
                self.buttons_frame,
                text=resolved_apply,
                command=self.on_apply_clicked,
            )
            self.apply_button.grid(row=0, column=1, padx=(0, 8))
        else:
            self.apply_button = None

        cancel_column = 2 if self.show_apply else 1
        self.cancel_button = ttk.Button(
            self.buttons_frame,
            text=resolved_cancel,
            command=self.on_cancel_clicked,
        )
        self.cancel_button.grid(row=0, column=cancel_column)

        self._refresh_apply_button_state()

        if self.enter_confirms:
            self.bind("<Return>", self._on_enter_confirm)

    # ------------------------------------------------------------------
    # UI helpers
    # ------------------------------------------------------------------
    def _ui(self, key: str, **kwargs: Any) -> str:
        if self.ui is not None:
            try:
                return self.ui(key, **kwargs)
            except Exception:
                pass

        fallback = {
            "common.button.ok": "Aceptar",
            "common.button.apply": "Aplicar",
            "common.button.cancel": "Cancelar",
            "common.status.info": "Info: {message}",
            "common.status.warning": "Aviso: {message}",
            "common.status.error": "Error: {message}",
            "common.confirm.discard_changes.title": "Cambios sin guardar",
            "common.dialog.confirm_discard_unsaved": "Hay cambios sin guardar. ¿Quieres descartarlos?",
            "form.validation.review_data": "Revisa los datos del formulario.",
            "form.status.changes_applied": "Cambios aplicados.",
            "form.error.initialize_failed": "Error inicializando formulario: {exc}",
            "form.error.apply_failed": "Error al aplicar cambios: {exc}",
            "dialog.warning.title": "Aviso",
            "dialog.info.title": "Información",
            "dialog.error.title": "Error",
        }

        text = fallback.get(key, key)
        if kwargs:
            try:
                text = text.format(**kwargs)
            except Exception:
                pass
        return text

    # ------------------------------------------------------------------
    # Estado / UX no bloqueante
    # ------------------------------------------------------------------
    def clear_status(self) -> None:
        self.status_var.set("")

    def set_status(self, text: str) -> None:
        self.status_var.set(text)

    def set_info(self, message: str) -> None:
        self.set_status(self._ui("common.status.info", message=message))

    def set_warning(self, message: str) -> None:
        self.set_status(self._ui("common.status.warning", message=message))

    def set_error(self, message: str) -> None:
        self.set_status(self._ui("common.status.error", message=message))

    # ------------------------------------------------------------------
    # Dirty state
    # ------------------------------------------------------------------
    def is_dirty(self) -> bool:
        return self._dirty

    def mark_dirty(self) -> None:
        self._dirty = True
        self._refresh_apply_button_state()

    def clear_dirty(self) -> None:
        self._dirty = False
        self._refresh_apply_button_state()

    def bind_dirty_var(self, variable: tk.Variable) -> None:
        token = variable.trace_add("write", lambda *_args: self.mark_dirty())
        self._dirty_trace_tokens.append((variable, token))

    def _refresh_apply_button_state(self) -> None:
        if self.apply_button is not None:
            self.apply_button.configure(state=("normal" if self._dirty else "disabled"))

    # ------------------------------------------------------------------
    # Hooks para subclases
    # ------------------------------------------------------------------
    def initialize_form(self) -> Any:
        """
        Carga snapshot inicial y deja el diálogo en estado limpio.
        Debe llamarse una vez la subclase haya construido su contenido.
        """
        try:
            snapshot = self.load_form_data()
        except Exception as exc:
            self.set_error(self._ui("form.error.initialize_failed", exc=exc))
            return None

        self.clear_dirty()
        self.clear_status()
        return snapshot

    def load_form_data(self) -> Any:
        return {}

    def collect_form_data(self) -> Any:
        return {}

    def validate_form_data(self, data: Any) -> tuple[bool, str]:
        return True, ""

    def apply_form_data(self, data: Any) -> None:
        raise NotImplementedError("Subclasses must implement apply_form_data().")

    # ------------------------------------------------------------------
    # Acciones estándar
    # ------------------------------------------------------------------
    def on_apply_clicked(self) -> bool:
        try:
            data = self.collect_form_data()
            ok, message = self.validate_form_data(data)
        except Exception as exc:
            self.set_error(self._ui("form.error.apply_failed", exc=exc))
            return False

        if not ok:
            self.set_warning(message or self._ui("form.validation.review_data"))
            return False

        try:
            self.apply_form_data(data)
        except Exception as exc:
            self.set_error(self._ui("form.error.apply_failed", exc=exc))
            return False

        self.clear_dirty()
        self.set_info(self._ui("form.status.changes_applied"))
        return True

    def on_ok_clicked(self) -> None:
        if not self.is_dirty():
            self.close()
            return

        if self.on_apply_clicked():
            self.close()

    def on_cancel_clicked(self) -> None:
        if not self._can_close_with_dirty_state():
            return
        self.close()

    def on_close_requested(self) -> bool:
        return self._can_close_with_dirty_state()

    def _can_close_with_dirty_state(self) -> bool:
        if not self._dirty or not self.confirm_discard_on_cancel:
            return True

        return bool(
            messagebox.askyesno(
                self._ui("common.confirm.discard_changes.title"),
                self._ui("common.dialog.confirm_discard_unsaved"),
                parent=self,
            )
        )

    def _on_enter_confirm(self, _event: tk.Event) -> str:
        self.on_ok_clicked()
        return "break"
