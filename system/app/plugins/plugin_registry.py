"""
===============================================================================
SCRIPT_META
===============================================================================
{
  "script_name": "plugin_registry.py",
  "version": "1.1.0",
  "type": "core_module",
  "category": "plugins",
  "description": "In-memory registry for discovered plugins across system, user and dev layers using unique scope-aware keys.",
  "location_expected": "system/app/plugins/",
  "entry_point": "PluginRegistry",
  "inputs": ["plugin metadata dictionaries"],
  "outputs": ["registered plugin catalogue", "filtered plugin views"]
}
===============================================================================
"""

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, List, Optional


@dataclass(frozen=True)
class PluginEntrypoint:
    """
    Declarative description of a plugin entrypoint.
    """

    type: str
    module: str
    entry_function: str
    description: str = ""


@dataclass
class PluginRecord:
    """
    In-memory normalized plugin representation.
    """

    plugin_id: str
    name: str
    version: str
    description: str
    plugin_type: str
    scope: str
    path: Path
    manifest_path: Path
    enabled: bool = True
    platform_compatibility: Optional[str] = None
    outputs: List[str] = field(default_factory=list)
    requires: List[str] = field(default_factory=list)
    tags: List[str] = field(default_factory=list)
    author: Optional[str] = None
    homepage: Optional[str] = None
    entrypoints: List[PluginEntrypoint] = field(default_factory=list)
    raw_manifest: Dict[str, Any] = field(default_factory=dict)

    @property
    def key(self) -> str:
        """
        Unique technical key used internally by the registry.

        Example:
            system:personal_snippets
            user:personal_snippets
            dev:personal_snippets
        """
        return f"{self.scope}:{self.plugin_id}"

    def to_dict(self) -> Dict[str, Any]:
        return {
            "key": self.key,
            "plugin_id": self.plugin_id,
            "name": self.name,
            "version": self.version,
            "description": self.description,
            "plugin_type": self.plugin_type,
            "scope": self.scope,
            "path": str(self.path),
            "manifest_path": str(self.manifest_path),
            "enabled": self.enabled,
            "platform_compatibility": self.platform_compatibility,
            "outputs": list(self.outputs),
            "requires": list(self.requires),
            "tags": list(self.tags),
            "author": self.author,
            "homepage": self.homepage,
            "entrypoints": [
                {
                    "type": ep.type,
                    "module": ep.module,
                    "entry_function": ep.entry_function,
                    "description": ep.description,
                }
                for ep in self.entrypoints
            ],
        }


class PluginRegistryError(Exception):
    """Base registry exception."""


class DuplicatePluginError(PluginRegistryError):
    """Raised when a duplicated scope-aware plugin key is registered."""


class PluginNotFoundError(PluginRegistryError):
    """Raised when a requested plugin does not exist."""


class AmbiguousPluginIdError(PluginRegistryError):
    """Raised when a plain plugin_id matches more than one plugin."""


class PluginRegistry:
    """
    Registry of discovered plugins.

    Responsibilities:
    - normalize plugin manifests into consistent records
    - prevent duplicated scope-aware plugin keys
    - expose query/filter helpers
    - provide stable plugin catalogue for loader/manager/UI

    Non-responsibilities:
    - filesystem discovery
    - JSON schema validation
    - module import/execution
    """

    def __init__(self) -> None:
        self._plugins: Dict[str, PluginRecord] = {}

    # ------------------------------------------------------------------
    # Registration
    # ------------------------------------------------------------------

    def register(
        self,
        *,
        scope: str,
        path: Path,
        manifest: Dict[str, Any],
        manifest_path: Optional[Path] = None,
    ) -> PluginRecord:
        """
        Register a plugin from a validated manifest.

        Parameters
        ----------
        scope:
            Discovery scope. Usually one of: system, user, dev.
        path:
            Root directory of the plugin.
        manifest:
            Validated manifest dictionary.
        manifest_path:
            Optional explicit manifest path.
        """

        record = self._build_record(
            scope=scope,
            path=path,
            manifest=manifest,
            manifest_path=manifest_path,
        )

        if record.key in self._plugins:
            existing = self._plugins[record.key]
            raise DuplicatePluginError(
                f"Duplicated plugin key '{record.key}'. "
                f"Existing: {existing.path} | New: {record.path}"
            )

        self._plugins[record.key] = record
        return record

    def unregister(self, key: str) -> None:
        """
        Remove a plugin from the registry by unique key.

        Example:
            user:personal_snippets
        """

        if key not in self._plugins:
            raise PluginNotFoundError(f"Plugin not found: {key}")

        del self._plugins[key]

    def clear(self) -> None:
        """
        Remove all registered plugins.
        """
        self._plugins.clear()

    # ------------------------------------------------------------------
    # Queries
    # ------------------------------------------------------------------

    def has_plugin(self, key: str) -> bool:
        return key in self._plugins

    def get(self, key: str) -> PluginRecord:
        """
        Get plugin by unique registry key.

        Example:
            registry.get("user:personal_snippets")
        """
        if key not in self._plugins:
            raise PluginNotFoundError(f"Plugin not found: {key}")
        return self._plugins[key]

    def get_optional(self, key: str) -> Optional[PluginRecord]:
        return self._plugins.get(key)

    def get_by_plugin_id(self, plugin_id: str) -> PluginRecord:
        """
        Get a plugin by functional plugin_id only.

        Raises:
            PluginNotFoundError if none exists
            AmbiguousPluginIdError if more than one exists
        """
        matches = self.find_by_plugin_id(plugin_id)

        if not matches:
            raise PluginNotFoundError(f"Plugin id not found: {plugin_id}")

        if len(matches) > 1:
            keys = ", ".join(plugin.key for plugin in matches)
            raise AmbiguousPluginIdError(
                f"Ambiguous plugin_id '{plugin_id}'. Matches: {keys}"
            )

        return matches[0]

    def all(self) -> List[PluginRecord]:
        """
        Return all registered plugins sorted by unique key.
        """
        return [self._plugins[k] for k in sorted(self._plugins)]

    def keys(self) -> List[str]:
        return sorted(self._plugins.keys())

    def ids(self) -> List[str]:
        return sorted({plugin.plugin_id for plugin in self._plugins.values()})

    def count(self) -> int:
        return len(self._plugins)

    def by_scope(self, scope: str) -> List[PluginRecord]:
        return sorted(
            (p for p in self._plugins.values() if p.scope == scope),
            key=lambda p: p.key,
        )

    def by_type(self, plugin_type: str) -> List[PluginRecord]:
        return sorted(
            (p for p in self._plugins.values() if p.plugin_type == plugin_type),
            key=lambda p: p.key,
        )

    def enabled(self) -> List[PluginRecord]:
        return sorted(
            (p for p in self._plugins.values() if p.enabled),
            key=lambda p: p.key,
        )

    def disabled(self) -> List[PluginRecord]:
        return sorted(
            (p for p in self._plugins.values() if not p.enabled),
            key=lambda p: p.key,
        )

    def with_tag(self, tag: str) -> List[PluginRecord]:
        return sorted(
            (p for p in self._plugins.values() if tag in p.tags),
            key=lambda p: p.key,
        )

    def requiring(self, dependency: str) -> List[PluginRecord]:
        return sorted(
            (p for p in self._plugins.values() if dependency in p.requires),
            key=lambda p: p.key,
        )

    def find_by_plugin_id(self, plugin_id: str) -> List[PluginRecord]:
        """
        Find all plugins sharing the same functional plugin_id across scopes.
        """
        return sorted(
            (p for p in self._plugins.values() if p.plugin_id == plugin_id),
            key=lambda p: p.key,
        )

    def find_entrypoint(
        self,
        key: str,
        *,
        entry_function: Optional[str] = None,
        module: Optional[str] = None,
    ) -> List[PluginEntrypoint]:
        """
        Find entrypoints inside a plugin using optional filters.
        """
        plugin = self.get(key)
        result = plugin.entrypoints

        if entry_function is not None:
            result = [ep for ep in result if ep.entry_function == entry_function]

        if module is not None:
            result = [ep for ep in result if ep.module == module]

        return result

    # ------------------------------------------------------------------
    # Export helpers
    # ------------------------------------------------------------------

    def to_dict(self) -> Dict[str, Any]:
        return {
            "plugin_count": self.count(),
            "plugins": [plugin.to_dict() for plugin in self.all()],
        }

    def summary(self) -> Dict[str, Any]:
        return {
            "total": self.count(),
            "system": len(self.by_scope("system")),
            "user": len(self.by_scope("user")),
            "dev": len(self.by_scope("dev")),
            "enabled": len(self.enabled()),
            "disabled": len(self.disabled()),
        }

    # ------------------------------------------------------------------
    # Internal helpers
    # ------------------------------------------------------------------

    def _build_record(
        self,
        *,
        scope: str,
        path: Path,
        manifest: Dict[str, Any],
        manifest_path: Optional[Path],
    ) -> PluginRecord:
        entrypoints = [
            PluginEntrypoint(
                type=entry["type"],
                module=entry["module"],
                entry_function=entry["entry_function"],
                description=entry.get("description", ""),
            )
            for entry in manifest.get("entrypoints", [])
        ]

        return PluginRecord(
            plugin_id=manifest["plugin_id"],
            name=manifest["name"],
            version=manifest["version"],
            description=manifest["description"],
            plugin_type=manifest["plugin_type"],
            scope=scope,
            path=path,
            manifest_path=manifest_path or (path / "plugin_manifest.json"),
            enabled=manifest.get("enabled", True),
            platform_compatibility=manifest.get("platform_compatibility"),
            outputs=list(manifest.get("outputs", [])),
            requires=list(manifest.get("requires", [])),
            tags=list(manifest.get("tags", [])),
            author=manifest.get("author"),
            homepage=manifest.get("homepage"),
            entrypoints=entrypoints,
            raw_manifest=dict(manifest),
        )