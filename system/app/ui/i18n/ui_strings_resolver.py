from __future__ import annotations

import json
from pathlib import Path
from typing import Any, Dict


class UIStringsResolver:
    """
    Resuelve textos UI desde un catálogo multilenguaje.

    Soporta metadatos de catálogo tipo:
        - _meta en la raíz
        - _status u otros campos auxiliares dentro de cada clave
    """

    def __init__(
        self,
        filepath: str | Path,
        language: str = "es",
        fallback_language: str = "en",
    ) -> None:
        self.filepath = Path(filepath)
        self.language = language
        self.fallback_language = fallback_language
        self._data: Dict[str, Dict[str, Any]] = {}
        self._load()

    # ---------------------------------------------------------
    # Load
    # ---------------------------------------------------------
    def _load(self) -> None:
        if not self.filepath.exists():
            raise FileNotFoundError(f"UI strings file not found: {self.filepath}")

        with open(self.filepath, "r", encoding="utf-8") as f:
            data = json.load(f)

        if not isinstance(data, dict):
            raise ValueError("UI strings file must contain a dictionary")

        self._data = data

    # ---------------------------------------------------------
    # Helpers
    # ---------------------------------------------------------
    def _get_entry_translations(self, entry: Dict[str, Any]) -> Dict[str, str]:
        translations: Dict[str, str] = {}
        for key, value in entry.items():
            if key.startswith("_"):
                continue
            if isinstance(value, str):
                translations[key] = value
        return translations

    # ---------------------------------------------------------
    # Public API
    # ---------------------------------------------------------
    def get(self, key: str, **kwargs: Any) -> str:
        entry = self._data.get(key)

        if not isinstance(entry, dict):
            text = key
        else:
            translations = self._get_entry_translations(entry)
            text = (
                translations.get(self.language)
                or translations.get(self.fallback_language)
                or next(iter(translations.values()), key)
            )

        if kwargs:
            try:
                text = text.format(**kwargs)
            except Exception:
                pass

        return text

    # ---------------------------------------------------------
    # Callable
    # ---------------------------------------------------------
    def __call__(self, key: str, **kwargs: Any) -> str:
        return self.get(key, **kwargs)

    # ---------------------------------------------------------
    # Runtime language switch
    # ---------------------------------------------------------
    def set_language(self, language: str) -> None:
        self.language = language

    # ---------------------------------------------------------
    # Debug helpers
    # ---------------------------------------------------------
    def keys(self) -> list[str]:
        return [key for key in self._data.keys() if key != "_meta"]

    def raw(self) -> Dict[str, Dict[str, Any]]:
        return self._data
