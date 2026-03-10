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
class DialogTabProtocol(Protocol):
    def load_data(self, snapshot: Any) -> None:
        ...

    def collect_data(self) -> Optional[dict[str, Any]]:
        ...

    def validate_data(self) -> tuple[bool, str]:
        ...

    def set_readonly(self, readonly: bool) -> None:
        ...


class TabbedDialogBase(FormDialogBase):
    """
    Base común para formularios con ttk.Notebook.
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
            ok_text=ok_text,
            apply_text=apply_text,
            cancel_text=cancel_text,
            show_apply=True,
            enter_confirms=enter_confirms,
            confirm_discard_on_cancel=confirm_discard_on_cancel,
            ui=ui,
        )

        self.content_frame.columnconfigure(0, weight=1)
        self.content_frame.rowconfigure(0, weight=1)

        self.notebook = ttk.Notebook(self.content_frame)
        self.notebook.grid(row=0, column=0, sticky="nsew")

        self._tabs: dict[str, dict[str, Any]] = {}
        self._tab_order: list[str] = []

        self.notebook.bind("<<NotebookTabChanged>>", self._on_tab_changed)

        self.build_tabs()
        self.initialize_form()

    # ------------------------------------------------------------------
    # Hooks de extensión
    # ------------------------------------------------------------------
    def build_tabs(self) -> None:
        return None

    def after_tab_changed(self, tab_name: str, tab_widget: tk.Widget) -> None:
        return None

    def get_form_snapshot(self) -> Any:
        return {}

    def validate_aggregated_form_data(self, data: Any) -> tuple[bool, str]:
        return True, ""

    # ------------------------------------------------------------------
    # Registro y acceso a tabs
    # ------------------------------------------------------------------
    def add_tab(self, name: str, widget: tk.Widget, *, title: Optional[str] = None) -> None:
        if not name:
            raise ValueError("Tab name must be non-empty.")
        if name in self._tabs:
            raise ValueError(f"Duplicated tab name: {name!r}")

        visible_title = title or name
        self.notebook.add(widget, text=visible_title)

        self._tabs[name] = {
            "name": name,
            "title": visible_title,
            "widget": widget,
        }
        self._tab_order.append(name)

    def add_tabs(self, tabs: list[tuple[str, tk.Widget, Optional[str]]]) -> None:
        for name, widget, title in tabs:
            self.add_tab(name, widget, title=title)

    def get_tab(self, name: str) -> Optional[tk.Widget]:
        meta = self._tabs.get(name)
        return None if meta is None else meta["widget"]

    def get_tab_names(self) -> list[str]:
        return list(self._tab_order)

    def get_current_tab_name(self) -> Optional[str]:
        current_id = self.notebook.select()
        if not current_id:
            return None

        for name in self._tab_order:
            widget = self._tabs[name]["widget"]
            if str(widget) == current_id:
                return name
        return None

    def get_current_tab_widget(self) -> Optional[tk.Widget]:
        name = self.get_current_tab_name()
        return self.get_tab(name) if name else None

    def select_tab(self, name: str) -> None:
        meta = self._tabs.get(name)
        if meta is None:
            raise KeyError(f"Unknown tab: {name}")
        self.notebook.select(meta["widget"])

    # ------------------------------------------------------------------
    # Helpers internos capacidades opcionales de tab
    # ------------------------------------------------------------------
    def _tab_loader(self, widget: tk.Widget):
        fn = getattr(widget, "load_data", None)
        return fn if callable(fn) else None

    def _tab_collector(self, widget: tk.Widget):
        fn = getattr(widget, "collect_data", None)
        return fn if callable(fn) else None

    def _tab_validator(self, widget: tk.Widget):
        fn = getattr(widget, "validate_data", None)
        return fn if callable(fn) else None

    def _tab_readonly_setter(self, widget: tk.Widget):
        fn = getattr(widget, "set_readonly", None)
        return fn if callable(fn) else None

    # ------------------------------------------------------------------
    # Lifecycle de formulario
    # ------------------------------------------------------------------
    def load_form_data(self) -> Any:
        snapshot = self.get_form_snapshot()

        for name in self._tab_order:
            widget = self._tabs[name]["widget"]
            loader = self._tab_loader(widget)
            if loader is not None:
                loader(snapshot)

        current_name = self.get_current_tab_name()
        if current_name:
            self.after_tab_changed(current_name, self._tabs[current_name]["widget"])

        return snapshot

    def collect_form_data(self) -> Any:
        payload: dict[str, Any] = {}

        for name in self._tab_order:
            widget = self._tabs[name]["widget"]
            collector = self._tab_collector(widget)
            if collector is None:
                continue

            data = collector()
            if data is None:
                continue
            if not isinstance(data, dict):
                raise TypeError(
                    f"Tab {name!r} returned {type(data).__name__}; expected dict or None."
                )
            payload.update(data)

        return payload

    # ------------------------------------------------------------------
    # Dirty helpers
    # ------------------------------------------------------------------
    def mark_tab_dirty(self, _tab_name: Optional[str] = None) -> None:
        self.mark_dirty()

    def bind_tab_dirty_vars(self, *variables: tk.Variable) -> None:
        for variable in variables:
            self.bind_dirty_var(variable)

    # ------------------------------------------------------------------
    # Validación
    # ------------------------------------------------------------------
    def validate_form_data(self, data: Any) -> tuple[bool, str]:
        for name in self._tab_order:
            widget = self._tabs[name]["widget"]
            validator = self._tab_validator(widget)
            if validator is None:
                continue

            result = validator()
            if not isinstance(result, tuple) or len(result) != 2:
                raise TypeError(
                    f"Tab {name!r} validate_data() must return tuple[bool, str]."
                )

            ok, message = result
            if not ok:
                self.select_tab(name)
                return (
                    False,
                    message or self._ui("tab.validation.review_tab", title=self._tabs[name]["title"]),
                )

        return self.validate_aggregated_form_data(data)

    # ------------------------------------------------------------------
    # Solo lectura
    # ------------------------------------------------------------------
    def set_tabs_readonly(self, readonly: bool) -> None:
        for name in self._tab_order:
            widget = self._tabs[name]["widget"]
            setter = self._tab_readonly_setter(widget)
            if setter is not None:
                setter(readonly)

    # ------------------------------------------------------------------
    # Eventos
    # ------------------------------------------------------------------
    def _on_tab_changed(self, _event: tk.Event) -> None:
        name = self.get_current_tab_name()
        if not name:
            return
        self.after_tab_changed(name, self._tabs[name]["widget"])
