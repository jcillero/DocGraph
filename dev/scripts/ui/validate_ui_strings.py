#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "validate_ui_strings.py",
  "version": "1.0.0",
  "type": "devtool",
  "subtype": "catalog_validator",
  "category": "dev",
  "description": "Valida el catálogo ui_strings.json y detecta inconsistencias entre claves definidas y usadas en el código Python, incluyendo placeholders, idiomas requeridos y claves deprecated.",
  "location_expected": "dev/scripts/ui",
  "entry_point": "main",
  "inputs": [
    {
      "type": "catalog",
      "location": "system/spec/ui/ui_strings.json"
    },
    {
      "type": "source_code",
      "location": "system/ui"
    }
  ],
  "outputs": [
    {
      "type": "stdout",
      "description": "Reporte de validación UI strings"
    },
    {
      "type": "exit_code",
      "description": "0=ok, 1=validation errors"
    }
  ],
  "capabilities": [
    "validate_catalog_schema",
    "scan_python_ui_usage",
    "detect_missing_keys",
    "detect_unused_keys",
    "validate_placeholders",
    "detect_deprecated_usage"
  ],
  "modifies_system": false,
  "runtime_ref": "python"
}
================================================================================
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, Iterable, List, Set, Tuple


DEFAULT_REQUIRED_LANGS: Set[str] = {"es", "en"}

DEFAULT_ALLOWED_PREFIXES: Tuple[str, ...] = (
    "common.",
    "dialog.",
    "menu.",
    "prefs.",
    "form.",
    "wizard.",
    "tab.",
)

DEFAULT_ALLOW_UNUSED: Set[str] = {
    "dialog.info.title",
    "dialog.warning.title",
}

DEFAULT_EXCLUDED_DIRS: Set[str] = {
    "__pycache__",
    ".git",
    ".hg",
    ".svn",
    ".mypy_cache",
    ".pytest_cache",
    ".ruff_cache",
    "venv",
    ".venv",
    "env",
    ".env",
    "build",
    "dist",
    "node_modules",
}

VALID_STATUSES: Set[str] = {
    "active",
    "deprecated",
    "draft",
    "reserved",
}

UI_CALL_PATTERNS: Tuple[re.Pattern[str], ...] = (
    re.compile(r"""\bself\.ui\(\s*['"]([a-zA-Z0-9_.]+)['"]"""),
    re.compile(r"""\bself\._ui\(\s*['"]([a-zA-Z0-9_.]+)['"]"""),
    re.compile(r"""\bui\(\s*['"]([a-zA-Z0-9_.]+)['"]"""),
    re.compile(r"""\bresolver\(\s*['"]([a-zA-Z0-9_.]+)['"]"""),
)

PLACEHOLDER_PATTERN = re.compile(r"\{([a-zA-Z_][a-zA-Z0-9_]*)\}")


@dataclass
class Report:
    errors: List[str] = field(default_factory=list)
    warnings: List[str] = field(default_factory=list)
    info: List[str] = field(default_factory=list)

    defined_keys: Set[str] = field(default_factory=set)
    used_keys: Set[str] = field(default_factory=set)
    missing_keys: Set[str] = field(default_factory=set)
    unused_keys: Set[str] = field(default_factory=set)

    deprecated_used_keys: Set[str] = field(default_factory=set)
    used_by_file: Dict[str, Set[str]] = field(default_factory=dict)

    def has_errors(self) -> bool:
        return bool(self.errors)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate ui_strings.json structure and usage in Python source files."
    )
    parser.add_argument(
        "--catalog",
        required=True,
        help="Path to ui_strings.json",
    )
    parser.add_argument(
        "--src",
        nargs="+",
        required=True,
        help="One or more Python source files or directories to scan.",
    )
    parser.add_argument(
        "--strict-unused",
        action="store_true",
        help="Treat unused catalog keys as errors instead of warnings.",
    )
    parser.add_argument(
        "--ignore-unused",
        nargs="*",
        default=[],
        help="Additional keys allowed to remain unused.",
    )
    parser.add_argument(
        "--verbose-files",
        action="store_true",
        help="Show keys detected in each file.",
    )
    return parser.parse_args()


def load_ui_strings(path: Path) -> Dict[str, Dict[str, Any]]:
    if not path.exists():
        raise FileNotFoundError(f"Catalog file not found: {path}")

    try:
        raw = path.read_text(encoding="utf-8")
    except OSError as exc:
        raise OSError(f"Could not read catalog file '{path}': {exc}") from exc

    try:
        data = json.loads(raw)
    except json.JSONDecodeError as exc:
        raise ValueError(
            f"Invalid JSON in '{path}' at line {exc.lineno}, col {exc.colno}: {exc.msg}"
        ) from exc

    if not isinstance(data, dict):
        raise TypeError("Catalog root must be a JSON object.")

    return data


def get_meta(data: Dict[str, Dict[str, Any]]) -> Dict[str, Any]:
    meta = data.get("_meta", {})
    return meta if isinstance(meta, dict) else {}


def iter_catalog_entries(data: Dict[str, Dict[str, Any]]) -> Iterable[Tuple[str, Dict[str, Any]]]:
    for key, value in data.items():
        if key == "_meta":
            continue
        yield key, value


def extract_placeholders(text: str) -> Set[str]:
    return set(PLACEHOLDER_PATTERN.findall(text))


def get_language_map(entry: Dict[str, Any]) -> Dict[str, str]:
    if not isinstance(entry, dict):
        return {}

    lang_map: Dict[str, str] = {}
    for key, value in entry.items():
        if key.startswith("_"):
            continue
        if isinstance(value, str):
            lang_map[key] = value
        else:
            lang_map[key] = value
    return lang_map


def get_required_langs(meta: Dict[str, Any]) -> Set[str]:
    raw = meta.get("required_languages")
    if isinstance(raw, list) and all(isinstance(x, str) and x.strip() for x in raw):
        return {x.strip() for x in raw}
    return set(DEFAULT_REQUIRED_LANGS)


def get_allowed_prefixes(meta: Dict[str, Any]) -> Tuple[str, ...]:
    raw = meta.get("allowed_prefixes")
    if isinstance(raw, list):
        prefixes = tuple(x for x in raw if isinstance(x, str) and x.strip())
        if prefixes:
            return prefixes
    return DEFAULT_ALLOWED_PREFIXES


def get_key_status(entry: Dict[str, Any]) -> str:
    if not isinstance(entry, dict):
        return "active"

    status = entry.get("_status", "active")
    if isinstance(status, str) and status.strip():
        return status.strip()
    return "active"


def validate_meta(meta: Dict[str, Any]) -> Tuple[List[str], List[str]]:
    errors: List[str] = []
    warnings: List[str] = []

    if not meta:
        warnings.append("Catalog does not define '_meta'. Using default validator configuration.")
        return errors, warnings

    schema_version = meta.get("schema_version")
    if schema_version is not None and not isinstance(schema_version, str):
        errors.append("Catalog '_meta.schema_version' must be a string.")

    default_language = meta.get("default_language")
    if default_language is not None and not isinstance(default_language, str):
        errors.append("Catalog '_meta.default_language' must be a string.")

    required_languages = meta.get("required_languages")
    if required_languages is not None:
        if not isinstance(required_languages, list) or not all(
            isinstance(x, str) and x.strip() for x in required_languages
        ):
            errors.append("Catalog '_meta.required_languages' must be a list of non-empty strings.")

    allowed_prefixes = meta.get("allowed_prefixes")
    if allowed_prefixes is not None:
        if not isinstance(allowed_prefixes, list) or not all(
            isinstance(x, str) and x.strip() for x in allowed_prefixes
        ):
            errors.append("Catalog '_meta.allowed_prefixes' must be a list of non-empty strings.")

    return errors, warnings


def validate_catalog_schema(
    data: Dict[str, Dict[str, Any]],
    required_langs: Set[str],
    allowed_prefixes: Tuple[str, ...],
) -> Tuple[List[str], List[str]]:
    errors: List[str] = []
    warnings: List[str] = []

    for key, value in iter_catalog_entries(data):
        if not isinstance(key, str):
            errors.append(f"Non-string catalog key detected: {key!r}")
            continue

        if not any(key.startswith(prefix) for prefix in allowed_prefixes):
            warnings.append(
                f"Key outside allowed prefixes: {key} "
                f"(allowed: {', '.join(allowed_prefixes)})"
            )

        if not isinstance(value, dict):
            errors.append(f"Key '{key}' must map to an object with language entries.")
            continue

        status = get_key_status(value)
        if status not in VALID_STATUSES:
            warnings.append(
                f"Key '{key}' has unknown _status='{status}'. "
                f"Expected one of: {', '.join(sorted(VALID_STATUSES))}"
            )

        lang_map = get_language_map(value)
        missing_langs = required_langs - set(lang_map.keys())
        if missing_langs:
            errors.append(
                f"Key '{key}' is missing required languages: {', '.join(sorted(missing_langs))}"
            )

        for lang in sorted(required_langs & set(lang_map.keys())):
            lang_value = lang_map.get(lang)

            if not isinstance(lang_value, str):
                errors.append(
                    f"Key '{key}' language '{lang}' must be a string, "
                    f"got {type(lang_value).__name__}."
                )
                continue

            if not lang_value.strip():
                errors.append(
                    f"Key '{key}' language '{lang}' is empty or whitespace only."
                )

            if lang_value != lang_value.strip():
                warnings.append(
                    f"Key '{key}' language '{lang}' has leading/trailing whitespace."
                )

    return errors, warnings


def validate_catalog_entries(
    data: Dict[str, Dict[str, Any]],
    required_langs: Set[str],
) -> Tuple[List[str], List[str]]:
    errors: List[str] = []
    warnings: List[str] = []

    for key, entry in iter_catalog_entries(data):
        if not isinstance(entry, dict):
            continue

        lang_map = get_language_map(entry)
        if not required_langs.issubset(lang_map.keys()):
            continue

        placeholder_map: Dict[str, Set[str]] = {}
        for lang in sorted(required_langs):
            text = lang_map.get(lang)
            if isinstance(text, str):
                placeholder_map[lang] = extract_placeholders(text)

        base_lang = "es" if "es" in placeholder_map else next(iter(placeholder_map), None)
        if base_lang is None:
            continue

        base_placeholders = placeholder_map[base_lang]

        for lang, placeholders in placeholder_map.items():
            if placeholders != base_placeholders:
                errors.append(
                    f"Placeholder mismatch in key '{key}': "
                    f"{base_lang}={sorted(base_placeholders)} vs "
                    f"{lang}={sorted(placeholders)}"
                )

    return errors, warnings


def iter_python_files(paths: Iterable[Path]) -> Iterable[Path]:
    for path in paths:
        if not path.exists():
            continue

        if path.is_file():
            if path.suffix == ".py":
                yield path
            continue

        for subpath in path.rglob("*.py"):
            if any(part in DEFAULT_EXCLUDED_DIRS for part in subpath.parts):
                continue
            yield subpath


def extract_ui_keys_from_source(source_text: str) -> Set[str]:
    found: Set[str] = set()
    for pattern in UI_CALL_PATTERNS:
        for match in pattern.finditer(source_text):
            found.add(match.group(1))
    return found


def scan_python_files(
    paths: Iterable[Path],
) -> Tuple[Set[str], Dict[str, Set[str]], List[str]]:
    used_keys: Set[str] = set()
    used_by_file: Dict[str, Set[str]] = {}
    warnings: List[str] = []

    for py_file in iter_python_files(paths):
        try:
            text = py_file.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            warnings.append(f"Skipped non-UTF8 file: {py_file}")
            continue
        except OSError as exc:
            warnings.append(f"Could not read source file '{py_file}': {exc}")
            continue

        file_keys = extract_ui_keys_from_source(text)
        if file_keys:
            used_by_file[str(py_file)] = file_keys
            used_keys.update(file_keys)

    return used_keys, used_by_file, warnings


def compare_defined_vs_used(
    defined_keys: Set[str],
    used_keys: Set[str],
    allow_unused: Set[str],
    data: Dict[str, Dict[str, Any]],
) -> Tuple[Set[str], Set[str]]:
    missing_keys = used_keys - defined_keys

    ignored_unused: Set[str] = set(allow_unused)
    for key in defined_keys:
        entry = data.get(key)
        if isinstance(entry, dict):
            status = get_key_status(entry)
            if status in {"draft", "reserved"}:
                ignored_unused.add(key)

    unused_keys = defined_keys - used_keys - ignored_unused
    return missing_keys, unused_keys


def detect_redundant_families(data: Dict[str, Dict[str, Any]]) -> List[str]:
    warnings: List[str] = []

    redundant_pairs = [
        ("common.status.info_prefix", "common.status.info"),
        ("common.status.warning_prefix", "common.status.warning"),
        ("common.status.error_prefix", "common.status.error"),
        (
            "common.confirm.discard_changes.message",
            "common.dialog.confirm_discard_unsaved",
        ),
        ("wizard.validation.step_named", "wizard.validation.review_step"),
        ("wizard.validation.step_review", "wizard.validation.review_step_data"),
    ]

    for a, b in redundant_pairs:
        if a in data and b in data:
            warnings.append(
                f"Potential redundant keys detected: '{a}' and '{b}'. "
                f"Consider keeping only one naming convention."
            )

    return warnings


def detect_deprecated_usage(
    data: Dict[str, Dict[str, Any]],
    used_keys: Set[str],
) -> List[str]:
    warnings: List[str] = []

    for key in sorted(used_keys):
        entry = data.get(key)
        if isinstance(entry, dict) and get_key_status(entry) == "deprecated":
            warnings.append(f"Deprecated key used in code: {key}")

    return warnings


def print_section(title: str, items: List[str]) -> None:
    print()
    print(title)
    print("-" * len(title))
    if not items:
        print("(none)")
        return
    for item in items:
        print(f"- {item}")


def print_report(
    report: Report,
    catalog_path: Path,
    strict_unused: bool,
    verbose_files: bool,
) -> None:
    print("UI STRINGS VALIDATION")
    print("=====================")
    print(f"Catalog file: {catalog_path}")
    print(f"Defined keys: {len(report.defined_keys)}")
    print(f"Used keys in code: {len(report.used_keys)}")
    print(f"Strict unused mode: {'ON' if strict_unused else 'OFF'}")

    print_section("INFO", report.info)
    print_section("ERRORS", report.errors)
    print_section("WARNINGS", report.warnings)

    if verbose_files and report.used_by_file:
        print()
        print("USED KEYS BY FILE")
        print("-----------------")
        for filename in sorted(report.used_by_file):
            keys = sorted(report.used_by_file[filename])
            print(f"{filename}: {len(keys)} key(s)")
            for key in keys:
                print(f"  - {key}")

    print()
    print("SUMMARY")
    print("-------")
    print(f"Errors: {len(report.errors)}")
    print(f"Warnings: {len(report.warnings)}")
    print(f"Missing keys: {len(report.missing_keys)}")
    print(f"Unused keys: {len(report.unused_keys)}")
    print(f"Deprecated keys used: {len(report.deprecated_used_keys)}")


def main() -> int:
    args = parse_args()

    catalog_path = Path(args.catalog).resolve()
    src_paths = [Path(p).resolve() for p in args.src]
    allow_unused = set(DEFAULT_ALLOW_UNUSED) | set(args.ignore_unused)

    report = Report()

    try:
        data = load_ui_strings(catalog_path)
    except Exception as exc:
        print(f"[FATAL] {exc}", file=sys.stderr)
        return 1

    meta = get_meta(data)
    required_langs = get_required_langs(meta)
    allowed_prefixes = get_allowed_prefixes(meta)

    meta_errors, meta_warnings = validate_meta(meta)
    report.errors.extend(meta_errors)
    report.warnings.extend(meta_warnings)

    report.defined_keys = {key for key, _value in iter_catalog_entries(data)}
    report.info.append("Catalog JSON loaded successfully.")
    report.info.append(
        f"Required languages: {', '.join(sorted(required_langs))}"
    )
    report.info.append(
        f"Allowed prefixes: {', '.join(allowed_prefixes)}"
    )
    if meta:
        report.info.append(
            f"Schema version: {meta.get('schema_version', '(not set)')}"
        )

    schema_errors, schema_warnings = validate_catalog_schema(
        data=data,
        required_langs=required_langs,
        allowed_prefixes=allowed_prefixes,
    )
    report.errors.extend(schema_errors)
    report.warnings.extend(schema_warnings)

    entry_errors, entry_warnings = validate_catalog_entries(
        data=data,
        required_langs=required_langs,
    )
    report.errors.extend(entry_errors)
    report.warnings.extend(entry_warnings)

    report.warnings.extend(detect_redundant_families(data))

    used_keys, used_by_file, scan_warnings = scan_python_files(src_paths)
    report.used_keys = used_keys
    report.used_by_file = used_by_file
    report.warnings.extend(scan_warnings)

    report.warnings.extend(detect_deprecated_usage(data, used_keys))
    report.deprecated_used_keys = {
        key
        for key in used_keys
        if isinstance(data.get(key), dict) and get_key_status(data[key]) == "deprecated"
    }

    missing_keys, unused_keys = compare_defined_vs_used(
        defined_keys=report.defined_keys,
        used_keys=report.used_keys,
        allow_unused=allow_unused,
        data=data,
    )
    report.missing_keys = missing_keys
    report.unused_keys = unused_keys

    for key in sorted(missing_keys):
        report.errors.append(f"Missing key used in code: {key}")

    for key in sorted(unused_keys):
        message = f"Unused key in catalog: {key}"
        if args.strict_unused:
            report.errors.append(message)
        else:
            report.warnings.append(message)

    print_report(
        report=report,
        catalog_path=catalog_path,
        strict_unused=args.strict_unused,
        verbose_files=args.verbose_files,
    )

    return 1 if report.has_errors() else 0


if __name__ == "__main__":
    raise SystemExit(main())