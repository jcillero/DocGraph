from __future__ import annotations

from typing import Any, Callable, Mapping, Protocol, Sequence

try:
    from system.app.ui.dialogs.base.tabbed_dialog_base import TabbedDialogBase
    from system.app.ui.dialogs.preferences.prefs_field_classifier import bucket_fields
    from system.app.ui.dialogs.preferences.tabs.tab_from_catalog import TabFromCatalog
except ImportError:  # pragma: no cover - fallback for local harness/testing
    from tabbed_dialog_base import TabbedDialogBase  # type: ignore
    from prefs_field_classifier import bucket_fields  # type: ignore
    from tab_from_catalog import TabFromCatalog  # type: ignore


TAB_ORDER: tuple[str, ...] = ("general", "ui", "llm", "advanced", "other")


class PreferencesControllerProtocol(Protocol):
    def get_preferences_fields_all(self) -> Sequence[Mapping[str, Any]]: ...
    def get_preferences_snapshot(self) -> Mapping[str, Any]: ...
    def apply_preferences_patch(self, data: Mapping[str, Any]) -> None: ...
    def get_ui_resolver(self) -> Callable[..., str]: ...
    def get_window_icon_path(self) -> str | None: ...
    def get_header_icon_path(self) -> str | None: ...


class PreferencesDialog(TabbedDialogBase):
    """
    Preferences dialog generated dynamically from a declarative catalog.

    Expected controller contract:
        - get_preferences_fields_all()
        - get_preferences_snapshot()
        - apply_preferences_patch(data)
        - get_ui_resolver()
        - get_window_icon_path()   [optional]
        - get_header_icon_path()   [optional]

    Responsibilities:
        - compose tabs from exposed preference fields
        - aggregate flat dotted-path patch data from tabs
        - delegate read/apply operations to controller
        - keep UI state and dialog lifecycle local

    Non-responsibilities:
        - domain validation
        - persistence details
        - business rules
    """

    def __init__(self, parent: Any, controller: PreferencesControllerProtocol) -> None:
        self.controller = controller
        self.ui: Callable[..., str] = controller.get_ui_resolver()
        self._tabs_by_bucket: dict[str, TabFromCatalog] = {}

        super().__init__(
            parent,
            title_text=self.ui("dialog.preferences.title"),
            subtitle_text=self.ui("dialog.preferences.subtitle"),
            modal=True,
            min_width=760,
            min_height=560,
            window_icon_path=self._safe_optional_path("get_window_icon_path"),
            header_icon_path=self._safe_optional_path("get_header_icon_path"),
            confirm_discard_on_cancel=True,
            ok_text=self.ui("common.button.ok"),
            apply_text=self.ui("common.button.apply"),
            cancel_text=self.ui("common.button.cancel"),
            ui=self.ui,
        )

    # ------------------------------------------------------------------
    # Controller helpers
    # ------------------------------------------------------------------
    def _safe_optional_path(self, attr_name: str) -> str | None:
        getter = getattr(self.controller, attr_name, None)
        if not callable(getter):
            return None
        try:
            value = getter()
        except Exception:
            return None
        return str(value) if value else None

    def _get_visible_fields(self) -> list[dict[str, Any]]:
        raw_fields = self.controller.get_preferences_fields_all()

        if not isinstance(raw_fields, Sequence):
            return []

        visible_fields: list[dict[str, Any]] = []
        for item in raw_fields:
            if not isinstance(item, Mapping):
                continue
            if not bool(item.get("exposed", False)):
                continue
            visible_fields.append(dict(item))

        visible_fields.sort(key=lambda item: str(item.get("key", "")).lower())
        return visible_fields

    def _build_empty_tab(self) -> None:
        empty_tab = TabFromCatalog(
            self.notebook,
            fields=[],
            on_dirty=self.mark_tab_dirty,
            ui=self.ui,
            unknown_bucket=True,
        )
        self._tabs_by_bucket["other"] = empty_tab
        self.add_tab("other", empty_tab, title=self.ui("prefs.tab.other"))

    # ------------------------------------------------------------------
    # Aggregation helpers
    # ------------------------------------------------------------------
    def _collect_flat_patch(self) -> dict[str, Any]:
        """
        Collect a flat preferences patch using dotted keys, for example:
            {
                "ui.language": "en",
                "ui.theme": "light",
                "llm.mode": "manual"
            }
        """
        patch: dict[str, Any] = {}

        for _bucket, tab in self._tabs_by_bucket.items():
            collector = getattr(tab, "collect_flat_data", None)
            if not callable(collector):
                continue

            try:
                tab_data = collector()
            except Exception:
                continue

            if not isinstance(tab_data, dict):
                continue

            for key, value in tab_data.items():
                if isinstance(key, str) and "." in key:
                    patch[key] = value

        return patch

    # ------------------------------------------------------------------
    # TabbedDialogBase hooks
    # ------------------------------------------------------------------
    def build_tabs(self) -> None:
        self._tabs_by_bucket.clear()

        visible_fields = self._get_visible_fields()
        buckets = bucket_fields(visible_fields)

        for bucket in TAB_ORDER:
            fields = buckets.get(bucket, [])
            if not fields:
                continue

            tab = TabFromCatalog(
                self.notebook,
                fields=fields,
                on_dirty=self.mark_tab_dirty,
                ui=self.ui,
                unknown_bucket=(bucket == "other"),
            )
            self._tabs_by_bucket[bucket] = tab

            self.add_tab(
                bucket,
                tab,
                title=self.ui(f"prefs.tab.{bucket}"),
            )

        if not self._tabs_by_bucket:
            self._build_empty_tab()

    def get_form_snapshot(self) -> dict[str, Any]:
        snapshot = self.controller.get_preferences_snapshot()
        return dict(snapshot) if isinstance(snapshot, Mapping) else {}

    def apply_form_data(self, data: dict[str, Any]) -> None:
        """
        Ignore generic aggregated input from the base dialog and send the
        explicit flat patch expected by PreferencesManager.
        """
        patch = self._collect_flat_patch()
        self.controller.apply_preferences_patch(patch)

    def validate_aggregated_form_data(self, data: Any) -> tuple[bool, str]:
        del data

        patch = self._collect_flat_patch()
        if not isinstance(patch, dict):
            return False, self.ui("prefs.validation.invalid_format")

        return True, ""

    def after_tab_changed(self, tab_name: str, tab_widget: Any) -> None:
        del tab_name, tab_widget
        self.clear_status()

    # ------------------------------------------------------------------
    # Modal lifecycle
    # ------------------------------------------------------------------
    def show_modal(self) -> None:
        self.transient(self.master)
        self.grab_set()
        try:
            self.wait_window()
        finally:
            try:
                self.grab_release()
            except Exception:
                pass