from __future__ import annotations

import json
from pathlib import Path
from typing import Any, Callable

from system.app.core.preferences.preferences_manager import PreferencesManager
from system.app.ui.dialogs.preferences.preferences_dialog import PreferencesDialog
from system.app.ui.i18n.ui_strings_resolver import UIStringsResolver


class PreferencesController:
    """
    Bridge between PreferencesDialog and PreferencesManager.

    Responsibilities:
    - resolve runtime/user/spec paths
    - expose UI strings
    - expose catalog-driven fields
    - load effective preferences snapshot
    - persist patches through PreferencesManager
    - resolve effective UI language from persisted preferences when no external
      language getter is provided
    """

    def __init__(
        self,
        *,
        app_root: str | Path,
        preferences_manager: PreferencesManager | None = None,
        language_getter: Callable[[], str] | None = None,
    ) -> None:
        self.app_root = Path(app_root)

        self._ui_strings_path = (
            self.app_root / "system" / "spec" / "ui" / "ui_strings.json"
        )
        self._catalog_path = (
            self.app_root / "system" / "spec" / "preferences_catalog.json"
        )
        self._preferences_path = (
            self.app_root / "user" / "registry" / "preferences.json"
        )

        self._window_icon_path = (
            self.app_root / "system" / "assets" / "images" / "logos" / "app_icon.ico"
        )
        self._header_icon_path = (
            self.app_root / "system" / "assets" / "images" / "logos" / "app_header.png"
        )

        self.manager = preferences_manager or PreferencesManager(
            preferences_path=self._preferences_path,
            catalog_path=self._catalog_path,
        )

        self._language_getter = language_getter or self._get_language_from_preferences

    # ------------------------------------------------------------------
    # Internal language resolution
    # ------------------------------------------------------------------
    def _get_language_from_preferences(self) -> str:
        """
        Resolve UI language from effective preferences.

        Fallback order:
        1. effective preferences from PreferencesManager
        2. raw preferences.json if needed
        3. hard fallback: "es"
        """
        # Preferred path: effective values from manager
        try:
            payload, status = self.manager.load_or_repair()
            if status.ok and isinstance(payload.values, dict):
                ui_block = payload.values.get("ui", {})
                if isinstance(ui_block, dict):
                    language = ui_block.get("language")
                    if isinstance(language, str) and language.strip():
                        return language.strip()
        except Exception:
            pass

        # Defensive fallback: raw JSON read
        try:
            if self._preferences_path.exists():
                data = json.loads(self._preferences_path.read_text(encoding="utf-8"))
                values = data.get("values", {})
                if isinstance(values, dict):
                    ui_block = values.get("ui", {})
                    if isinstance(ui_block, dict):
                        language = ui_block.get("language")
                        if isinstance(language, str) and language.strip():
                            return language.strip()
        except Exception:
            pass

        return "es"

    # ------------------------------------------------------------------
    # UI / i18n
    # ------------------------------------------------------------------
    def get_ui_resolver(self) -> UIStringsResolver:
        return UIStringsResolver(
            filepath=self._ui_strings_path,
            language=self._language_getter(),
            fallback_language="en",
        )

    def get_window_icon_path(self) -> Path:
        return self._window_icon_path

    def get_header_icon_path(self) -> Path:
        return self._header_icon_path

    # ------------------------------------------------------------------
    # Catalog
    # ------------------------------------------------------------------
    def get_preferences_fields_all(self) -> list[dict[str, Any]]:
        if not self._catalog_path.exists():
            return []

        data = json.loads(self._catalog_path.read_text(encoding="utf-8"))
        prefs = data.get("preferences", {})
        if not isinstance(prefs, dict):
            return []

        ui = self.get_ui_resolver()
        fields: list[dict[str, Any]] = []

        for key, spec in prefs.items():
            if not isinstance(key, str) or not isinstance(spec, dict):
                continue

            field_type = str(spec.get("type", "string")).strip().lower()
            ui_control = str(
                spec.get("ui_control") or self._map_type_to_ui_control(field_type, spec)
            ).strip().lower()

            label = self._resolve_localized_value(
                spec.get("label"),
                ui,
                fallback=str(spec.get("description") or key),
            )

            help_text = self._resolve_localized_value(
                spec.get("help"),
                ui,
                fallback="",
            )

            field: dict[str, Any] = {
                "key": key,
                "type": field_type,
                "label": label,
                "help": help_text,
                "default": spec.get("default"),
                "group": spec.get("group", ""),
                "tags": spec.get("tags", []),
                "ui_control": ui_control,
                "exposed": bool(spec.get("exposed", False)),
            }

            if field_type == "enum":
                allowed = spec.get("allowed", [])
                field["choices"] = allowed if isinstance(allowed, list) else []

            if field_type in ("int", "float"):
                rng = spec.get("range")
                if isinstance(rng, list) and len(rng) == 2:
                    field["min"] = rng[0]
                    field["max"] = rng[1]

            fields.append(field)

        return fields

    def _map_type_to_ui_control(self, field_type: str, spec: dict[str, Any]) -> str:
        if field_type == "bool":
            return "checkbox"
        if field_type == "enum":
            return "select"
        if field_type in ("int", "float") and isinstance(spec.get("range"), list):
            return "slider"
        return "entry"

    def _resolve_localized_value(
        self,
        value: Any,
        ui: UIStringsResolver,
        *,
        fallback: str = "",
    ) -> str:
        if isinstance(value, dict):
            lang = self._language_getter()
            return str(value.get(lang) or value.get("en") or fallback)

        if isinstance(value, str) and value.startswith("ui:"):
            return ui(value[3:])

        if isinstance(value, str):
            return value

        return fallback

    # ------------------------------------------------------------------
    # Preferences data
    # ------------------------------------------------------------------
    def get_preferences_snapshot(self) -> dict[str, Any]:
        payload, _status = self.manager.load_or_repair()
        return payload.values

    def apply_preferences_patch(self, patch: dict[str, Any]) -> None:
        _payload, status = self.manager.set_preferences(patch)
        if not status.ok:
            raise ValueError("; ".join(status.errors) or "Failed to save preferences.")

    # ------------------------------------------------------------------
    # Dialog launching
    # ------------------------------------------------------------------
    def open_dialog(self, parent) -> None:
        dialog = PreferencesDialog(parent, controller=self)
        dialog.show_modal()