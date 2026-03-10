"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "dialog_registry.py",
  "version": "1.0.1",
  "type": "module",
  "category": "ui",
  "description": "DialogRegistry: carga un catálogo declarativo de diálogos (system/spec/ui/dialogs_registry_v1.json) y los instancia/abre dinámicamente.",
  "location_expected": "system/app/ui/dialogs/",
  "runtime_required": true,
  "modifies_system": false,
  "output_location": null,
  "outputs": [],
  "depends_on": [],
  "alignment_required": ["UI-ARCH-001_ui_dialogs_structure"]
}
================================================================================
"""
from __future__ import annotations

import importlib
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Optional, Tuple


@dataclass(frozen=True)
class DialogSpec:
    dialog_id: str
    label: str
    module: str
    class_name: str
    modal: bool = True
    default_size: Tuple[int, int] = (700, 500)
    resizable: bool = True
    requires_context: bool = False
    icon: str = ""
    category: str = ""


class DialogRegistry:
    """
    Registry declarativo para abrir dialogs por id.

    Uso:
      DialogRegistry.open("preferences", parent=..., context=...)
      DialogRegistry.create("credentials", parent=..., context=..., ...)
    """

    _cache: Optional[Dict[str, DialogSpec]] = None

    # ----------------------------
    # Paths / loading
    # ----------------------------

    @staticmethod
    def _default_registry_path() -> Path:
        # Portable: se asume que el cwd es la raíz del workspace.
        # (START_APP.bat / runner ya lo hace)
        return Path("system/spec/ui/dialogs_registry_v1.json")

    @classmethod
    def resolve_path(cls, registry_path: Optional[Path] = None) -> Path:
        """Resuelve la ruta final del registry (permitiendo override)."""
        return registry_path or cls._default_registry_path()

    @staticmethod
    def _coerce_size(value: Any, fallback: Tuple[int, int] = (700, 500)) -> Tuple[int, int]:
        """
        Convierte default_size a (w, h) con ints. Acepta:
          - [800, 600], (800, 600)
          - {"w": 800, "h": 600}
          - "800x600"
        """
        try:
            if isinstance(value, (list, tuple)) and len(value) >= 2:
                w, h = int(value[0]), int(value[1])
                return (max(1, w), max(1, h))

            if isinstance(value, dict) and ("w" in value and "h" in value):
                w, h = int(value["w"]), int(value["h"])
                return (max(1, w), max(1, h))

            if isinstance(value, str) and "x" in value.lower():
                parts = value.lower().split("x", 1)
                w, h = int(parts[0].strip()), int(parts[1].strip())
                return (max(1, w), max(1, h))
        except Exception:
            pass

        return fallback

    @classmethod
    def load(cls, registry_path: Optional[Path] = None, force_reload: bool = False) -> Dict[str, DialogSpec]:
        """Carga y cachea el catálogo declarativo de dialogs."""
        if cls._cache is not None and not force_reload:
            return cls._cache

        path = cls.resolve_path(registry_path)
        if not path.exists():
            raise FileNotFoundError(f"Dialog registry not found: {path.as_posix()}")

        data = json.loads(path.read_text(encoding="utf-8"))
        dialogs = data.get("dialogs", {})
        cache: Dict[str, DialogSpec] = {}

        if not isinstance(dialogs, dict):
            raise ValueError("Invalid dialogs registry: 'dialogs' must be an object/dict.")

        for dialog_id, spec in dialogs.items():
            if not isinstance(spec, dict):
                raise ValueError(f"Invalid dialog spec for '{dialog_id}': expected object/dict.")

            module = spec.get("module")
            class_name = spec.get("class")
            if not module or not class_name:
                raise ValueError(f"Invalid dialog spec for '{dialog_id}': missing 'module' or 'class'.")

            cache[str(dialog_id)] = DialogSpec(
                dialog_id=str(dialog_id),
                label=str(spec.get("label", dialog_id)),
                module=str(module),
                class_name=str(class_name),
                modal=bool(spec.get("modal", True)),
                default_size=cls._coerce_size(spec.get("default_size"), fallback=(700, 500)),
                resizable=bool(spec.get("resizable", True)),
                requires_context=bool(spec.get("requires_context", False)),
                icon=str(spec.get("icon", "")),
                category=str(spec.get("category", "")),
            )

        cls._cache = cache
        return cache

    @classmethod
    def reload(cls, registry_path: Optional[Path] = None) -> Dict[str, DialogSpec]:
        """Fuerza recarga del registry."""
        return cls.load(registry_path=registry_path, force_reload=True)

    @classmethod
    def list_dialogs(cls) -> Tuple[str, ...]:
        """Devuelve dialog_ids conocidos."""
        return tuple(sorted(cls.load().keys()))

    # ----------------------------
    # Spec resolution / instantiation
    # ----------------------------

    @classmethod
    def get_spec(cls, dialog_id: str) -> DialogSpec:
        registry = cls.load()
        if dialog_id not in registry:
            known = ", ".join(sorted(registry.keys()))
            raise KeyError(f"Unknown dialog_id='{dialog_id}'. Known: {known}")
        return registry[dialog_id]

    @staticmethod
    def _apply_spec_best_effort(dlg: Any, spec: DialogSpec) -> None:
        """
        Aplica modal/size/resizable si el framework lo soporta.
        Best-effort: no falla si el objeto no expone métodos.
        """
        # Size
        w, h = spec.default_size
        for meth_name in ("resize", "setFixedSize", "setMinimumSize"):
            meth = getattr(dlg, meth_name, None)
            if callable(meth):
                try:
                    meth(w, h)
                except TypeError:
                    # Algunos frameworks aceptan tuplas/objetos
                    try:
                        meth((w, h))
                    except Exception:
                        pass
                except Exception:
                    pass
                break

        # Modal (Qt suele tener setModal / setWindowModality)
        if spec.modal:
            for meth_name in ("setModal", "setWindowModality"):
                meth = getattr(dlg, meth_name, None)
                if callable(meth):
                    try:
                        meth(True)
                    except Exception:
                        pass
                    break

        # Resizable (framework-dependent)
        # Tkinter: resizable(width, height)
        # Qt: flags; aquí solo intentamos si existe.
        if not spec.resizable:
            meth = getattr(dlg, "resizable", None)
            if callable(meth):
                try:
                    meth(False, False)
                except Exception:
                    pass

    @classmethod
    def create(cls, dialog_id: str, parent: Any = None, context: Any = None, **kwargs: Any) -> Any:
        spec = cls.get_spec(dialog_id)
        if spec.requires_context and context is None:
            raise ValueError(f"Dialog '{dialog_id}' requires_context=true but context=None")

        try:
            mod = importlib.import_module(spec.module)
        except Exception as e:
            raise ImportError(
                f"Failed to import dialog module for '{dialog_id}': module='{spec.module}'. Error={e}"
            ) from e

        try:
            DialogCls = getattr(mod, spec.class_name)
        except AttributeError as e:
            raise AttributeError(
                f"Dialog class not found for '{dialog_id}': module='{spec.module}', class='{spec.class_name}'."
            ) from e

        # Intento de firma flexible: (parent, context, **kwargs) o (parent, **kwargs) o (**kwargs)
        try:
            dlg = DialogCls(parent=parent, context=context, **kwargs)
        except TypeError:
            try:
                dlg = DialogCls(parent=parent, **kwargs)
            except TypeError:
                dlg = DialogCls(**kwargs)

        cls._apply_spec_best_effort(dlg, spec)
        return dlg

    @classmethod
    def open(cls, dialog_id: str, parent: Any = None, context: Any = None, **kwargs: Any) -> Any:
        dlg = cls.create(dialog_id, parent=parent, context=context, **kwargs)

        # Apertura compatible con frameworks típicos (Qt/Tk/custom)
        for method_name in ("exec_", "exec", "open", "show"):
            method = getattr(dlg, method_name, None)
            if callable(method):
                method()
                break

        return dlg