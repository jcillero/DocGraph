# system/app/ui/dialogs/preferences/prefs_field_classifier.py
from __future__ import annotations


KNOWN_BUCKETS = {"general", "ui", "llm", "advanced", "other", "otros"}


def _normalize_tag(value: str) -> str:
    return (value or "").strip().lower().replace("-", "_")


def classify_field(field: dict) -> str:
    """
    Devuelve uno de:
    - general
    - ui
    - llm
    - advanced
    - other
    """
    key = str(field.get("key", "")).strip()
    key_l = key.lower()

    group = _normalize_tag(str(field.get("group", "")))
    if group in KNOWN_BUCKETS:
        return "other" if group == "otros" else group

    tags = field.get("tags") or []
    tags_n = {_normalize_tag(str(tag)) for tag in tags}

    for bucket in ("general", "ui", "llm", "advanced"):
        if bucket in tags_n:
            return bucket

    if key_l.startswith("ui."):
        return "ui"

    if key_l.startswith(("llm.", "openai.", "anthropic.", "gemini.", "azure_openai.")):
        return "llm"

    if key_l.startswith(("runner.", "logging.", "telemetry.", "registry.", "runtime.")):
        return "advanced"

    if key_l.startswith("general."):
        return "general"

    return "other"


def bucket_fields(fields: list[dict]) -> dict[str, list[dict]]:
    buckets = {
        "general": [],
        "ui": [],
        "llm": [],
        "advanced": [],
        "other": [],
    }

    for field in fields:
        bucket = classify_field(field)
        buckets[bucket].append(field)

    # útil para UX
    buckets["other"].sort(key=lambda item: str(item.get("key", "")).lower())
    return buckets