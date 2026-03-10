from __future__ import annotations

import tkinter as tk
from tkinter import ttk
from typing import Any, Optional, Protocol, runtime_checkable

try:
    from system.app.ui.dialogs.base.dialog_base import PathLike
    from system.app.ui.dialogs.base.form_dialog_base import FormDialogBase, UIResolver
except ImportError:  # pragma: no cover - fallback for local harness/testing
    from dialog_base import PathLike  # type: ignore
    from form_dialog_base import FormDialogBase, UIResolver  # type: ignore


@runtime_checkable
class WizardStepProtocol(Protocol):
    def load_data(self, snapshot: Any) -> None:
        ...

    def collect_data(self) -> Optional[dict[str, Any]]:
        ...

    def validate_data(self) -> tuple[bool, str]:
        ...

    def on_enter_step(self) -> None:
        ...

    def on_leave_step(self) -> None:
        ...

    def set_readonly(self, readonly: bool) -> None:
        ...


class WizardDialogBase(FormDialogBase):
    """
    Base común para asistentes por pasos.

    Aporta:
    - navegación Back / Next / Finish / Cancel
    - indicador de paso actual
    - validación por paso + validación agregada final
    - integración con el ciclo de vida de FormDialogBase

    Cada paso puede ser un widget simple o uno con capacidades opcionales:
    - load_data(snapshot)
    - collect_data() -> dict | None
    - validate_data() -> tuple[bool, str]
    - on_enter_step()
    - on_leave_step()
    - set_readonly(flag)
    """

    def __init__(
        self,
        parent: tk.Misc,
        *,
        title_text: str,
        subtitle_text: str = "",
        app_name: str = "DocGraph",
        modal: bool = True,
        min_width: int = 680,
        min_height: int = 500,
        resizable_x: bool = True,
        resizable_y: bool = True,
        window_icon_path: Optional[PathLike] = None,
        header_icon_path: Optional[PathLike] = None,
        back_text: Optional[str] = None,
        next_text: Optional[str] = None,
        finish_text: Optional[str] = None,
        cancel_text: Optional[str] = None,
        enter_confirms: bool = False,
        confirm_discard_on_cancel: bool = False,
        ui: Optional[UIResolver] = None,
    ) -> None:
        resolved_finish = finish_text or self._resolve_ui(ui, "common.button.finish", "Finalizar")
        resolved_cancel = cancel_text or self._resolve_ui(ui, "common.button.cancel", "Cancelar")
        self._back_text = back_text or self._resolve_ui(ui, "common.button.back", "Anterior")
        self._next_text = next_text or self._resolve_ui(ui, "common.button.next", "Siguiente")

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
            ok_text=resolved_finish,
            cancel_text=resolved_cancel,
            show_apply=False,
            enter_confirms=enter_confirms,
            confirm_discard_on_cancel=confirm_discard_on_cancel,
            ui=ui,
        )

        self.step_indicator_var = tk.StringVar(value="")
        self.step_indicator_label = ttk.Label(
            self.footer_host,
            textvariable=self.step_indicator_var,
            anchor="w",
        )
        self.step_indicator_label.grid(row=0, column=0, sticky="ew", pady=(0, 6))
        self.status_label.grid_configure(row=1, column=0, sticky="ew", pady=(0, 8))
        self.footer_separator.grid_configure(row=2, column=0, sticky="ew", pady=(0, 8))
        self.buttons_frame.grid_configure(row=3, column=0, sticky="e")

        self.back_button = ttk.Button(
            self.buttons_frame,
            text=self._back_text,
            command=self.on_back_clicked,
        )
        self.next_button = ttk.Button(
            self.buttons_frame,
            text=self._next_text,
            command=self.on_next_clicked,
        )

        self.back_button.grid(row=0, column=0, padx=(0, 8))
        self.next_button.grid(row=0, column=1, padx=(0, 8))
        self.ok_button.grid_configure(row=0, column=2, padx=(0, 8))
        self.cancel_button.grid_configure(row=0, column=3)

        self.content_frame.columnconfigure(0, weight=1)
        self.content_frame.rowconfigure(0, weight=1)

        self.steps_container = ttk.Frame(self.content_frame)
        self.steps_container.grid(row=0, column=0, sticky="nsew")
        self.steps_container.columnconfigure(0, weight=1)
        self.steps_container.rowconfigure(0, weight=1)

        self._steps: list[dict[str, Any]] = []
        self._current_step_index: int = -1

        self.build_steps()
        self._ensure_steps_registered()
        self.initialize_form()
        self._refresh_navigation_buttons()

    @staticmethod
    def _resolve_ui(ui: Optional[UIResolver], key: str, default: str) -> str:
        if ui is not None:
            try:
                return ui(key)
            except Exception:
                pass
        return default

    # ------------------------------------------------------------------
    # Hooks de extensión
    # ------------------------------------------------------------------
    def build_steps(self) -> None:
        return None

    def get_form_snapshot(self) -> Any:
        return {}

    def validate_aggregated_form_data(self, data: Any) -> tuple[bool, str]:
        return True, ""

    def after_step_changed(self, step_name: str, step_widget: tk.Widget, step_index: int) -> None:
        return None

    # ------------------------------------------------------------------
    # Registro y acceso a pasos
    # ------------------------------------------------------------------
    def add_step(self, name: str, widget: tk.Widget, *, title: Optional[str] = None) -> None:
        if not name:
            raise ValueError("Step name must be non-empty.")
        if any(step["name"] == name for step in self._steps):
            raise ValueError(f"Duplicated step name: {name!r}")

        widget.grid(in_=self.steps_container, row=0, column=0, sticky="nsew")
        widget.grid_remove()

        self._steps.append(
            {
                "name": name,
                "title": title or name,
                "widget": widget,
            }
        )

    def add_steps(self, steps: list[tuple[str, tk.Widget, Optional[str]]]) -> None:
        for name, widget, title in steps:
            self.add_step(name, widget, title=title)

    def get_step_names(self) -> list[str]:
        return [step["name"] for step in self._steps]

    def get_step(self, name: str) -> Optional[tk.Widget]:
        for step in self._steps:
            if step["name"] == name:
                return step["widget"]
        return None

    def get_current_step_index(self) -> int:
        return self._current_step_index

    def get_current_step_name(self) -> Optional[str]:
        if self._current_step_index < 0 or self._current_step_index >= len(self._steps):
            return None
        return self._steps[self._current_step_index]["name"]

    def get_current_step_widget(self) -> Optional[tk.Widget]:
        if self._current_step_index < 0 or self._current_step_index >= len(self._steps):
            return None
        return self._steps[self._current_step_index]["widget"]

    def select_step(self, name: str) -> None:
        for idx, step in enumerate(self._steps):
            if step["name"] == name:
                self._show_step(idx)
                return
        raise KeyError(f"Unknown step: {name}")

    def _ensure_steps_registered(self) -> None:
        if self._steps:
            return
        raise RuntimeError(
            "WizardDialogBase requires at least one registered step. "
            "Call add_step() from build_steps()."
        )

    # ------------------------------------------------------------------
    # Helpers internos de capacidades opcionales
    # ------------------------------------------------------------------
    def _step_loader(self, widget: tk.Widget):
        fn = getattr(widget, "load_data", None)
        return fn if callable(fn) else None

    def _step_collector(self, widget: tk.Widget):
        fn = getattr(widget, "collect_data", None)
        return fn if callable(fn) else None

    def _step_validator(self, widget: tk.Widget):
        fn = getattr(widget, "validate_data", None)
        return fn if callable(fn) else None

    def _step_on_enter(self, widget: tk.Widget):
        fn = getattr(widget, "on_enter_step", None)
        return fn if callable(fn) else None

    def _step_on_leave(self, widget: tk.Widget):
        fn = getattr(widget, "on_leave_step", None)
        return fn if callable(fn) else None

    def _step_readonly_setter(self, widget: tk.Widget):
        fn = getattr(widget, "set_readonly", None)
        return fn if callable(fn) else None

    # ------------------------------------------------------------------
    # Lifecycle formulario
    # ------------------------------------------------------------------
    def load_form_data(self) -> Any:
        snapshot = self.get_form_snapshot()

        for step in self._steps:
            widget = step["widget"]
            loader = self._step_loader(widget)
            if loader is not None:
                loader(snapshot)

        if self._current_step_index == -1:
            self._show_step(0)

        return snapshot

    def collect_form_data(self) -> Any:
        payload: dict[str, Any] = {}

        for step in self._steps:
            widget = step["widget"]
            collector = self._step_collector(widget)
            if collector is None:
                continue

            data = collector()
            if data is None:
                continue
            if not isinstance(data, dict):
                raise TypeError(
                    f"Step {step['name']!r} returned {type(data).__name__}; expected dict or None."
                )
            payload.update(data)

        return payload

    def validate_form_data(self, data: Any) -> tuple[bool, str]:
        for idx, step in enumerate(self._steps):
            widget = step["widget"]
            validator = self._step_validator(widget)
            if validator is None:
                continue

            result = validator()
            if not isinstance(result, tuple) or len(result) != 2:
                raise TypeError(
                    f"Step {step['name']!r} validate_data() must return tuple[bool, str]."
                )

            ok, message = result
            if not ok:
                self._show_step(idx)
                return (
                    False,
                    message or self._ui("wizard.validation.review_step", title=step["title"]),
                )

        return self.validate_aggregated_form_data(data)

    # ------------------------------------------------------------------
    # Navegación
    # ------------------------------------------------------------------
    def _show_step(self, index: int) -> None:
        if not (0 <= index < len(self._steps)):
            return

        previous_widget = self.get_current_step_widget()
        if previous_widget is not None:
            on_leave = self._step_on_leave(previous_widget)
            if on_leave is not None:
                on_leave()
            previous_widget.grid_remove()

        self._current_step_index = index
        step = self._steps[index]
        widget = step["widget"]
        widget.grid()

        on_enter = self._step_on_enter(widget)
        if on_enter is not None:
            on_enter()

        self.clear_status()
        self._refresh_step_indicator()
        self._refresh_navigation_buttons()
        self.after_step_changed(step["name"], widget, index)

    def _refresh_step_indicator(self) -> None:
        if self._current_step_index < 0:
            self.step_indicator_var.set("")
            return

        step = self._steps[self._current_step_index]
        total = len(self._steps)
        current = self._current_step_index + 1
        self.step_indicator_var.set(
            self._ui("wizard.step_indicator", current=current, total=total, title=step["title"])
        )

    def _refresh_navigation_buttons(self) -> None:
        total = len(self._steps)
        idx = self._current_step_index

        is_first = idx <= 0
        is_last = (idx == total - 1) if total > 0 else True

        self.back_button.configure(state=("disabled" if is_first else "normal"))
        self.next_button.configure(state=("disabled" if is_last else "normal"))
        self.ok_button.configure(state=("normal" if is_last else "disabled"))

    def can_leave_current_step(self) -> tuple[bool, str]:
        widget = self.get_current_step_widget()
        if widget is None:
            return True, ""

        validator = self._step_validator(widget)
        if validator is None:
            return True, ""

        result = validator()
        if not isinstance(result, tuple) or len(result) != 2:
            raise TypeError("Current step validate_data() must return tuple[bool, str].")

        return result

    def on_back_clicked(self) -> None:
        if self._current_step_index > 0:
            self._show_step(self._current_step_index - 1)

    def on_next_clicked(self) -> None:
        ok, message = self.can_leave_current_step()
        if not ok:
            self.set_warning(message or self._ui("wizard.validation.review_step_data"))
            return

        if self._current_step_index < len(self._steps) - 1:
            self._show_step(self._current_step_index + 1)

    def on_ok_clicked(self) -> None:
        if self._current_step_index != len(self._steps) - 1:
            return
        super().on_ok_clicked()

    # ------------------------------------------------------------------
    # Solo lectura / dirty
    # ------------------------------------------------------------------
    def set_steps_readonly(self, readonly: bool) -> None:
        for step in self._steps:
            widget = step["widget"]
            setter = self._step_readonly_setter(widget)
            if setter is not None:
                setter(readonly)

    def mark_step_dirty(self, _step_name: Optional[str] = None) -> None:
        self.mark_dirty()

    def bind_step_dirty_vars(self, *variables: tk.Variable) -> None:
        for variable in variables:
            self.bind_dirty_var(variable)
