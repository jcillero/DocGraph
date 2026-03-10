"""
===============================================================================
SCRIPT_META
===============================================================================
{
  "script_name": "plugin_loader.py",
  "version": "1.1.0",
  "type": "core_module",
  "category": "plugins",
  "description": "Discovers, validates and loads plugins across system, user and dev layers using PluginRegistry.",
  "location_expected": "system/app/plugins/",
  "entry_point": "PluginLoader",
  "inputs": ["plugin directories", "plugin manifests", "plugin manifest schema"],
  "outputs": ["loaded plugin registry", "loaded entrypoints map"]
}
===============================================================================
"""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
from typing import Any, Callable, Dict, Optional

from system.app.plugins.plugin_registry import PluginRegistry, PluginRecord

try:
    import jsonschema
except ImportError:
    jsonschema = None


class PluginLoaderError(Exception):
    """Base plugin loader exception."""


class PluginSchemaNotFoundError(PluginLoaderError):
    """Raised when plugin manifest schema cannot be found."""


class PluginModuleLoadError(PluginLoaderError):
    """Raised when a plugin module cannot be loaded."""


class PluginEntrypointLoadError(PluginLoaderError):
    """Raised when a plugin entrypoint cannot be resolved."""


class PluginLoader:
    """
    Plugin discovery, validation and loading system.

    Responsibilities:
    - discover plugins from known plugin roots
    - load and validate plugin manifests
    - register plugins into PluginRegistry
    - import and expose plugin entrypoints

    Non-responsibilities:
    - plugin installation/uninstallation
    - persistence of enabled/disabled states outside manifests
    - UI management
    """

    def __init__(self, app_root: Path) -> None:
        self.app_root = app_root

        self.plugin_dirs = {
            "system": app_root / "system/plugins",
            "user": app_root / "user/plugins",
            "dev": app_root / "dev/plugins",
        }

        self.schema_path = app_root / "system/spec/plugin_manifest_schema.json"
        self.schema = self._load_schema()

        self.registry = PluginRegistry()

    # ------------------------------------------------------------------
    # Schema
    # ------------------------------------------------------------------

    def _load_schema(self) -> Dict[str, Any]:
        if not self.schema_path.exists():
            raise PluginSchemaNotFoundError(
                f"Plugin schema not found: {self.schema_path}"
            )

        with open(self.schema_path, "r", encoding="utf-8") as f:
            return json.load(f)

    def _validate_manifest(self, manifest: Dict[str, Any]) -> None:
        if jsonschema is None:
            return

        jsonschema.validate(manifest, self.schema)

    # ------------------------------------------------------------------
    # Discovery
    # ------------------------------------------------------------------

    def discover_plugins(self, clear_registry: bool = True) -> PluginRegistry:
        """
        Scan plugin directories, validate manifests and register plugins.

        Parameters
        ----------
        clear_registry:
            If True, clears the current registry before discovery.

        Returns
        -------
        PluginRegistry
            Populated registry instance.
        """
        if clear_registry:
            self.registry.clear()

        for scope, base_dir in self.plugin_dirs.items():
            if not base_dir.exists():
                continue

            for plugin_dir in sorted(base_dir.iterdir(), key=lambda p: p.name.lower()):
                if not plugin_dir.is_dir():
                    continue

                manifest_path = plugin_dir / "plugin_manifest.json"
                if not manifest_path.exists():
                    continue

                manifest = self._load_manifest(manifest_path)

                self.registry.register(
                    scope=scope,
                    path=plugin_dir,
                    manifest=manifest,
                    manifest_path=manifest_path,
                )

        return self.registry

    def _load_manifest(self, manifest_path: Path) -> Dict[str, Any]:
        with open(manifest_path, "r", encoding="utf-8") as f:
            manifest = json.load(f)

        self._validate_manifest(manifest)
        return manifest

    # ------------------------------------------------------------------
    # Entrypoint loading
    # ------------------------------------------------------------------

    def load_entrypoints(
        self,
        *,
        only_enabled: bool = True,
    ) -> Dict[str, Callable[..., Any]]:
        """
        Import plugin entrypoints and return a callable map.

        Key format:
            <scope>:<plugin_id>.<entry_function>

        Example:
            dev:validate_registry.main
            user:personal_snippets.run

        Parameters
        ----------
        only_enabled:
            If True, only enabled plugins are loaded.
        """
        plugins = self.registry.enabled() if only_enabled else self.registry.all()

        loaded_entrypoints: Dict[str, Callable[..., Any]] = {}

        for plugin in plugins:
            for entry in plugin.entrypoints:
                module = self._import_module(plugin, entry.module)

                try:
                    func = getattr(module, entry.entry_function)
                except AttributeError as exc:
                    raise PluginEntrypointLoadError(
                        f"Entrypoint '{entry.entry_function}' not found in "
                        f"module '{entry.module}' for plugin '{plugin.key}'."
                    ) from exc

                entry_key = f"{plugin.key}.{entry.entry_function}"
                loaded_entrypoints[entry_key] = func

        return loaded_entrypoints

    def load_plugin_entrypoints(
        self,
        plugin_key: str,
    ) -> Dict[str, Callable[..., Any]]:
        """
        Load entrypoints for a single plugin identified by unique key.

        Example:
            user:personal_snippets
        """
        plugin = self.registry.get(plugin_key)

        loaded_entrypoints: Dict[str, Callable[..., Any]] = {}

        for entry in plugin.entrypoints:
            module = self._import_module(plugin, entry.module)

            try:
                func = getattr(module, entry.entry_function)
            except AttributeError as exc:
                raise PluginEntrypointLoadError(
                    f"Entrypoint '{entry.entry_function}' not found in "
                    f"module '{entry.module}' for plugin '{plugin.key}'."
                ) from exc

            entry_key = f"{plugin.key}.{entry.entry_function}"
            loaded_entrypoints[entry_key] = func

        return loaded_entrypoints

    # ------------------------------------------------------------------
    # Module import
    # ------------------------------------------------------------------

    def _import_module(self, plugin: PluginRecord, module_path: str):
        module_file = plugin.path / (module_path.replace(".", "/") + ".py")

        if not module_file.exists():
            raise PluginModuleLoadError(
                f"Module file not found for plugin '{plugin.key}': {module_file}"
            )

        spec_name = f"{plugin.scope}_{plugin.plugin_id}_{module_path.replace('.', '_')}"
        spec = importlib.util.spec_from_file_location(spec_name, module_file)

        if spec is None or spec.loader is None:
            raise PluginModuleLoadError(
                f"Could not create import spec for plugin '{plugin.key}' "
                f"module '{module_path}'."
            )

        module = importlib.util.module_from_spec(spec)

        try:
            spec.loader.exec_module(module)
        except Exception as exc:
            raise PluginModuleLoadError(
                f"Failed to import module '{module_path}' for plugin '{plugin.key}'."
            ) from exc

        return module

    # ------------------------------------------------------------------
    # Helpers
    # ------------------------------------------------------------------

    def get_registry(self) -> PluginRegistry:
        return self.registry

    def get_plugin(self, plugin_key: str) -> PluginRecord:
        return self.registry.get(plugin_key)

    def find_plugins(self, plugin_id: str):
        return self.registry.find_by_plugin_id(plugin_id)

    def summary(self) -> Dict[str, Any]:
        return self.registry.summary()