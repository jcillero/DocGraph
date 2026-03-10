# =============================================================================
# SCRIPT METADATA
# =============================================================================
# script_id: credentials_manager
# version: 1.0.2
# script_type: core_module
# owner: system
# status: stable
#
# description:
#   Core module responsible for loading, repairing and saving user credentials
#   in a UI-agnostic way.
#
# script_location:
#   system/core/credentials/credentials_manager.py
#
# canonical_storage:
#   user/registry/credentials.json
#
# runtime_path_policy:
#   The path MUST be resolved dynamically using the runtime configuration
#   layer (runtime_registry / path resolver). Hardcoded paths are forbidden.
#
# behavior:
#   - Missing file → create empty template (defaults)
#   - Corrupt JSON → backup as *.bad-YYYYMMDD_HHMMSS and regenerate defaults
#   - Atomic writes (tmp → replace)
# =============================================================================

from __future__ import annotations

import json
import os
import time
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable, Dict, Optional, Tuple, List


# -----------------------------
# Defaults + schema (MVP)
# -----------------------------
SCHEMA_VERSION = "1.0.0"

# Known providers & fields (soft validation / tooling)
KNOWN_PROVIDERS: Tuple[str, ...] = ("openai", "anthropic", "azure_openai", "gemini")

KNOWN_FIELDS: Dict[str, Tuple[str, ...]] = {
    "openai": ("api_key", "base_url", "organization"),
    "anthropic": ("api_key",),
    "azure_openai": ("api_key", "endpoint", "deployment", "api_version"),
    "gemini": ("api_key", "project_id", "location"),
}

# NOTE:
# - This file is intended for PERSONAL use; values are kept in user/ (borrable).
# - Secrets are stored as plain text here (no encryption in MVP).
DEFAULT_CREDENTIALS: Dict[str, Any] = {
    "schema_version": SCHEMA_VERSION,
    "credentials_id": "user_credentials",
    "updated_at": "1970-01-01T00:00:00Z",
    "providers": {

        "openai": {
            "api_key": "",
            "base_url": "",
            "organization": "",
        },

        "anthropic": {
            "api_key": "",
        },

        "gemini": {
            "api_key": "",
            "project_id": "",
            "location": "",
        },

        "azure_openai": {
            "api_key": "",
            "endpoint": "",
            "deployment": "",
            "api_version": "",
        },

    },
}


# -----------------------------
# Status objects
# -----------------------------
@dataclass
class CredentialsStatus:
    ok: bool = True
    created_new: bool = False
    repaired: bool = False
    backed_up_path: Optional[str] = None
    warnings: List[str] = field(default_factory=list)
    errors: List[str] = field(default_factory=list)
    message: str = ""


@dataclass
class CredentialsPayload:
    schema_version: str
    credentials_id: str
    updated_at: str
    providers: Dict[str, Any]


# -----------------------------
# Helpers
# -----------------------------
def _utc_iso_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def _timestamp_compact_local() -> str:
    return datetime.now().strftime("%Y%m%d_%H%M%S")


def _deep_copy(obj: Any) -> Any:
    return json.loads(json.dumps(obj))


def _merge_defaults(defaults: Dict[str, Any], overrides: Dict[str, Any]) -> Dict[str, Any]:
    out = _deep_copy(defaults)

    def rec(dst: Dict[str, Any], src: Dict[str, Any]) -> None:
        for k, v in src.items():
            if isinstance(v, dict) and isinstance(dst.get(k), dict):
                rec(dst[k], v)
            else:
                dst[k] = v

    rec(out, overrides)
    return out


def _mask_secret(value: str) -> str:
    if not value:
        return ""
    # Keep a tiny tail so user can distinguish keys; still safe for logs/UI.
    if len(value) <= 8:
        return "********"
    return "****" + value[-4:]


# -----------------------------
# CredentialsManager
# -----------------------------
class CredentialsManager:
    """
    UI-agnostic credentials manager.

    Provide ONE of:
      - credentials_path: explicit file path
      - path_resolver: callable returning a Path to credentials.json

    Notes:
      - For personal use, this stores secrets in plain text under user/.
      - A future "scrub" tool can delete user/ to transfer the app cleanly.
    """

    def __init__(
        self,
        *,
        credentials_path: Optional[Path] = None,
        path_resolver: Optional[Callable[[], Path]] = None,
        auto_persist_on_warnings: bool = True,
        replace_retries: int = 2,
        replace_retry_sleep_s: float = 0.05,
    ) -> None:
        if credentials_path is None and path_resolver is None:
            raise ValueError(
                "CredentialsManager requires credentials_path or path_resolver "
                "(do not hardcode; resolve from your runtime/config layer)."
            )
        self._credentials_path = credentials_path
        self._path_resolver = path_resolver

        self._auto_persist_on_warnings = bool(auto_persist_on_warnings)
        self._replace_retries = int(replace_retries)
        self._replace_retry_sleep_s = float(replace_retry_sleep_s)

    def credentials_path(self) -> Path:
        p = self._credentials_path if self._credentials_path is not None else self._path_resolver()
        return Path(p)

    # ---------
    # Public API
    # ---------
    def load_or_repair(self) -> Tuple[CredentialsPayload, CredentialsStatus]:
        """
        Load credentials. If missing -> create empty defaults.
        If corrupt -> backup bad file and regenerate empty defaults.
        Always returns a valid payload (defaults merged with stored values).
        """
        path = self.credentials_path()
        status = CredentialsStatus(ok=True)

        path.parent.mkdir(parents=True, exist_ok=True)

        if not path.exists():
            payload = self._new_default_payload()
            self._save_payload_atomic(path, payload)
            status.created_new = True
            status.message = "Credentials file did not exist; created empty template."
            return payload, status

        try:
            raw = path.read_text(encoding="utf-8")
            data = json.loads(raw)
        except json.JSONDecodeError:
            backup = self._backup_corrupt(path)
            payload = self._new_default_payload()
            self._save_payload_atomic(path, payload)
            status.repaired = True
            status.backed_up_path = str(backup)
            status.warnings.append("Credentials JSON was corrupt and has been regenerated as empty template.")
            status.message = f"Corrupt credentials detected; backed up to {backup.name} and regenerated."
            return payload, status
        except OSError as e:
            status.ok = False
            status.errors.append(f"Failed to read credentials: {e}")
            status.message = "Failed to read credentials file."
            return self._new_default_payload(), status

        payload = self._normalize_payload(data, status)
        self._validate_payload(payload, status)

        # Keep OVERRIDES for storage; compute EFFECTIVE for UI/consumers
        overrides_for_storage = self._payload_to_dict(payload)
        effective = _merge_defaults(DEFAULT_CREDENTIALS, overrides_for_storage)
        payload = self._dict_to_payload(effective)

        # Persist cleaned version if configured (keeps user file well-formed) — but store only overrides
        if status.warnings and self._auto_persist_on_warnings:
            try:
                # Update timestamp in stored file, but don't force-default-fill every field
                stored = dict(overrides_for_storage)
                stored["updated_at"] = _utc_iso_now()
                stored_payload = self._dict_to_payload(stored)
                self._save_payload_atomic(path, stored_payload)
            except OSError as e:
                status.ok = False
                status.errors.append(f"Failed to persist normalized credentials: {e}")

        return payload, status

    def set_provider_fields(self, provider: str, fields: Dict[str, Any]) -> Tuple[CredentialsPayload, CredentialsStatus]:
        """
        Update a provider sub-dict (e.g. {"api_key": "..."}). Writes atomically.
        """
        payload, status = self.load_or_repair()
        if not status.ok:
            return payload, status

        if provider not in payload.providers or not isinstance(payload.providers.get(provider), dict):
            status.ok = False
            status.errors.append(f"Unknown provider '{provider}'.")
            status.message = "Credentials not saved due to errors."
            return payload, status

        if not isinstance(fields, dict):
            status.ok = False
            status.errors.append("fields must be a dict.")
            status.message = "Credentials not saved due to errors."
            return payload, status

        known_fields = set(KNOWN_FIELDS.get(provider, ()))
        for k, v in fields.items():
            if not isinstance(k, str):
                status.ok = False
                status.errors.append(f"Invalid field name type for provider '{provider}'.")
                continue
            if not isinstance(v, str):
                status.ok = False
                status.errors.append(f"Field '{provider}.{k}' must be a string.")
                continue

            if known_fields and k not in known_fields:
                status.warnings.append(f"Unknown field '{provider}.{k}' stored as-is.")

            payload.providers[provider][k] = v

        if status.errors:
            status.message = "Credentials not saved due to validation errors."
            return payload, status

        payload.schema_version = SCHEMA_VERSION
        payload.updated_at = _utc_iso_now()

        try:
            self._save_payload_atomic(self.credentials_path(), payload)
            status.message = "Credentials updated successfully."
        except OSError as e:
            status.ok = False
            status.errors.append(f"Failed to save credentials: {e}")
            status.message = "Failed to save credentials."

        return payload, status

    def get_provider(self, provider: str, *, masked: bool = False) -> Dict[str, Any]:
        """
        Convenience read for a single provider. If masked=True, masks common secret fields.
        """
        payload, _ = self.load_or_repair()
        p = payload.providers.get(provider, {})
        if not isinstance(p, dict):
            return {}

        if not masked:
            return _deep_copy(p)

        out = _deep_copy(p)
        # Mask typical secret fields
        for secret_key in ("api_key", "token", "secret"):
            if secret_key in out and isinstance(out[secret_key], str):
                out[secret_key] = _mask_secret(out[secret_key])
        return out

    def reset_credentials(self, *, keep_backup: bool = True) -> Tuple[CredentialsPayload, CredentialsStatus]:
        """
        Reset credentials to empty template. Optionally backs up existing file as *.bak-...
        """
        path = self.credentials_path()
        status = CredentialsStatus(ok=True)

        path.parent.mkdir(parents=True, exist_ok=True)

        if keep_backup and path.exists():
            try:
                backup = path.with_name(f"{path.name}.bak-{_timestamp_compact_local()}")
                path.replace(backup)
                status.backed_up_path = str(backup)
                status.warnings.append(f"Backed up previous credentials to {backup.name}.")
            except OSError as e:
                status.ok = False
                status.errors.append(f"Failed to backup existing credentials: {e}")
                status.message = "Reset aborted due to backup failure."
                return self._new_default_payload(), status

        payload = self._new_default_payload()
        try:
            self._save_payload_atomic(path, payload)
            status.message = "Credentials reset to empty template."
        except OSError as e:
            status.ok = False
            status.errors.append(f"Failed to write empty credentials: {e}")
            status.message = "Failed to reset credentials."

        return payload, status

    # ---------
    # Internal
    # ---------
    def _new_default_payload(self) -> CredentialsPayload:
        d = _deep_copy(DEFAULT_CREDENTIALS)
        d["updated_at"] = _utc_iso_now()
        return self._dict_to_payload(d)

    def _normalize_payload(self, data: Any, status: CredentialsStatus) -> CredentialsPayload:
        if not isinstance(data, dict):
            status.warnings.append("Credentials root was not an object; replacing with empty template.")
            return self._new_default_payload()

        schema_version = data.get("schema_version")
        credentials_id = data.get("credentials_id")
        updated_at = data.get("updated_at")
        providers = data.get("providers")

        if not isinstance(schema_version, str):
            status.warnings.append("Missing/invalid schema_version; normalizing.")
            schema_version = SCHEMA_VERSION

        if not isinstance(credentials_id, str):
            status.warnings.append("Missing/invalid credentials_id; normalizing.")
            credentials_id = "user_credentials"

        if not isinstance(updated_at, str):
            status.warnings.append("Missing/invalid updated_at; normalizing.")
            updated_at = _utc_iso_now()

        if not isinstance(providers, dict):
            status.warnings.append("Missing/invalid providers; replacing with empty template providers.")
            providers = _deep_copy(DEFAULT_CREDENTIALS["providers"])

        return CredentialsPayload(
            schema_version=str(schema_version),
            credentials_id=str(credentials_id),
            updated_at=str(updated_at),
            providers=providers,
        )

    def _validate_payload(self, payload: CredentialsPayload, status: CredentialsStatus) -> None:
        if not isinstance(payload.providers, dict):
            status.warnings.append("providers is not a dict; resetting.")
            payload.providers = _deep_copy(DEFAULT_CREDENTIALS["providers"])
            return

        # Ensure known providers are dicts with at least known fields
        for p in KNOWN_PROVIDERS:
            if p not in payload.providers or not isinstance(payload.providers.get(p), dict):
                status.warnings.append(f"Provider '{p}' missing/invalid; adding empty defaults.")
                payload.providers[p] = _deep_copy(DEFAULT_CREDENTIALS["providers"][p])
                continue

            # Ensure known fields exist and are strings
            for f in KNOWN_FIELDS.get(p, ()):
                if f not in payload.providers[p]:
                    status.warnings.append(f"Field '{p}.{f}' missing; adding default empty string.")
                    payload.providers[p][f] = ""
                elif not isinstance(payload.providers[p][f], str):
                    status.warnings.append(f"Field '{p}.{f}' invalid type; resetting to empty string.")
                    payload.providers[p][f] = ""

        unknown_providers = [k for k in payload.providers.keys() if k not in KNOWN_PROVIDERS]
        if unknown_providers:
            status.warnings.append(f"Unknown providers present: {sorted(unknown_providers)}")

    def _payload_to_dict(self, payload: CredentialsPayload) -> Dict[str, Any]:
        return {
            "schema_version": payload.schema_version,
            "credentials_id": payload.credentials_id,
            "updated_at": payload.updated_at,
            "providers": payload.providers,
        }

    def _dict_to_payload(self, d: Dict[str, Any]) -> CredentialsPayload:
        return CredentialsPayload(
            schema_version=str(d.get("schema_version", SCHEMA_VERSION)),
            credentials_id=str(d.get("credentials_id", "user_credentials")),
            updated_at=str(d.get("updated_at", _utc_iso_now())),
            providers=d.get("providers", {}),
        )

    def _backup_corrupt(self, path: Path) -> Path:
        backup = path.with_name(f"{path.name}.bad-{_timestamp_compact_local()}")
        try:
            path.replace(backup)
        except OSError:
            try:
                backup.write_bytes(path.read_bytes())
            except OSError:
                pass
        return backup

    def _save_payload_atomic(self, path: Path, payload: CredentialsPayload) -> None:
        tmp = path.with_name(f"{path.name}.tmp")
        data = self._payload_to_dict(payload)
        text = json.dumps(data, ensure_ascii=False, indent=2, sort_keys=True)

        with open(tmp, "w", encoding="utf-8", newline="\n") as f:
            f.write(text)
            f.flush()
            os.fsync(f.fileno())

        # Atomic replace with small retry (Windows can momentarily lock files)
        last_exc: Optional[OSError] = None
        for attempt in range(self._replace_retries + 1):
            try:
                os.replace(tmp, path)
                return
            except OSError as e:
                last_exc = e
                if attempt < self._replace_retries:
                    time.sleep(self._replace_retry_sleep_s)
                else:
                    raise OSError(f"os.replace failed after {self._replace_retries + 1} attempts: {e}") from e

