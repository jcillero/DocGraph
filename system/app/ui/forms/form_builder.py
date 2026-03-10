"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "form_builder.py",
  "version": "0.2.0",
  "type": "tool_module",
  "category": "ui",
  "description": "Declarative form builder aligned with UI_DECLARATIVE_FORM_SPEC and integrated with FieldFactory.",
  "location_expected": "system/app/ui/forms/",
  "entry_point": "FormBuilder",
  "runtime_required": false,
  "modifies_system": false,
  "inputs": [
    {
      "type": "mapping",
      "location": "in-memory",
      "description": "form_spec mapping"
    },
    {
      "type": "mapping",
      "location": "in-memory",
      "description": "optional snapshot mapping",
      "required": false
    }
  ],
  "outputs": [
    {
      "type": "widget",
      "location": "in-memory",
      "description": "ttk.Frame-based declarative form"
    }
  ],
  "depends_on": [
    "tkinter",
    "system/app/ui/fields/field_factory.py",
    "system/app/ui/fields/field_utils.py"
  ],
  "alignment_required": [
    "UI_DECLARATIVE_FORM_SPEC",
    "UI-FORM-BUILDER-SPEC",
    "UI_LAYOUT",
    "UI-ARCH-001",
    "CONTRATO_UI_CORE"
  ]
}
================================================================================
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Callable, Mapping, Optional, Protocol

import tkinter as tk
from tkinter import ttk

from system.app.ui.fields.field_factory import FieldFactory
from system.app.ui.fields.field_utils import get_snapshot_value, set_dotted_value


UIResolver = Callable[..., str]
DirtyCallback = Callable[[], None]


class FieldWidgetProtocol(Protocol):
    def grid(self, *args: Any, **kwargs: Any) -> Any: ...
    def set_value(self, value: Any) -> None: ...
    def get_value(self) -> Any: ...
    def set_readonly(self, readonly: bool = True) -> None: ...
    def set_enabled(self, enabled: bool = True) -> None: ...
    def set_visible(self, visible: bool = True) -> None: ...
    def focus_control(self) -> None: ...
    def validate_value(self) -> Any: ...


@dataclass(slots=True)
class NormalizedFieldSpec:
    id: str
    key: str
    control: str
    label: str
    description: str = ""
    default: Any = None
    visible: bool = True
    enabled: bool = True
    readonly: bool = False
    required: bool = False
    placeholder: str = ""
    options: list[Any] = field(default_factory=list)
    min: Any = None
    max: Any = None
    step: Any = None
    ui: dict[str, Any] = field(default_factory=dict)
    validation: dict[str, Any] = field(default_factory=dict)
    tags: list[str] = field(default_factory=list)
    metadata: dict[str, Any] = field(default_factory=dict)
    raw: dict[str, Any] = field(default_factory=dict)


@dataclass(slots=True)
class NormalizedSectionSpec:
    key: str
    title: str
    description: str = ""
    visible: bool = True
    collapsed: bool = False
    readonly: bool = False
    enabled: bool = True
    metadata: dict[str, Any] = field(default_factory=dict)
    fields: list[NormalizedFieldSpec] = field(default_factory=list)


@dataclass(slots=True)
class NormalizedFormSpec:
    id: str
    title: str
    description: str = ""
    mode: str = "form"
    readonly: bool = False
    enabled: bool = True
    metadata: dict[str, Any] = field(default_factory=dict)
    sections: list[NormalizedSectionSpec] = field(default_factory=list)


class FormBuilder(ttk.Frame):
    """
    Declarative form builder aligned with UI_DECLARATIVE_FORM_SPEC.

    Responsibilities:
    - validate and normalize a declarative form spec
    - build visible sections in declared order
    - create fields through FieldFactory
    - load snapshot/default values
    - collect current values
    - perform lightweight UI validation

    Non-responsibilities:
    - domain logic
    - persistence
    - runtime execution
    - business validation
    """

    def __init__(
        self,
        parent: tk.Misc,
        *,
        form_spec: Mapping[str, Any],
        snapshot: Optional[Mapping[str, Any]] = None,
        on_dirty: Optional[DirtyCallback] = None,
        ui: Optional[UIResolver] = None,
        strict_factory: bool = False,
        padding: int = 10,
        auto_build: bool = True,
    ) -> None:
        super().__init__(parent, padding=padding)

        self.ui = ui
        self.on_dirty = on_dirty
        self.strict_factory = bool(strict_factory)

        self._loading = False
        self._built = False

        self._form_spec = self._normalize_form_spec(form_spec)
        self._snapshot = dict(snapshot or {})

        self._field_widgets_by_id: dict[str, FieldWidgetProtocol] = {}
        self._field_widgets_by_key: dict[str, FieldWidgetProtocol] = {}
        self._field_specs_by_id: dict[str, NormalizedFieldSpec] = {}
        self._field_specs_by_key: dict[str, NormalizedFieldSpec] = {}
        self._section_frames: dict[str, ttk.Frame] = {}
        self._field_order: list[str] = []

        self.columnconfigure(0, weight=1)

        if auto_build:
            self.build()

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def build(self) -> None:
        if self._built:
            return

        self._build()
        self._built = True

        if self._snapshot:
            self.load_snapshot(self._snapshot)
        else:
            self.apply_defaults()

        if self._form_spec.readonly:
            self.set_readonly(True)

        if not self._form_spec.enabled:
            self.set_enabled(False)

    def load_data(self, snapshot: Mapping[str, Any] | None) -> None:
        self.load_snapshot(snapshot)

    def validate(self) -> tuple[bool, str]:
        return self.validate_form()

    def collect(self) -> dict[str, Any]:
        return self.collect_data()

    @property
    def form_id(self) -> str:
        return self._form_spec.id

    def has_fields(self) -> bool:
        return bool(self._field_order)

    # ------------------------------------------------------------------
    # UI resolver
    # ------------------------------------------------------------------

    def _ui(self, key: str, **kwargs: Any) -> str:
        if self.ui is not None:
            try:
                return self.ui(key, **kwargs)
            except Exception:
                pass

        text = key
        if kwargs:
            try:
                text = text.format(**kwargs)
            except Exception:
                pass
        return text

    # ------------------------------------------------------------------
    # Build
    # ------------------------------------------------------------------

    def _build(self) -> None:
        row = 0

        if self._form_spec.title:
            title_label = ttk.Label(self, text=self._form_spec.title)
            title_label.grid(row=row, column=0, sticky="w", pady=(0, 6))
            row += 1

        if self._form_spec.description:
            desc_label = ttk.Label(
                self,
                text=self._form_spec.description,
                justify="left",
                wraplength=900,
            )
            desc_label.grid(row=row, column=0, sticky="ew", pady=(0, 10))
            row += 1

        body = ttk.Frame(self)
        body.grid(row=row, column=0, sticky="nsew")
        body.columnconfigure(0, weight=1)

        visible_section_index = 0
        for section in self._form_spec.sections:
            if not section.visible:
                continue

            section_frame = self._build_section(body, section)
            section_frame.grid(
                row=visible_section_index,
                column=0,
                sticky="ew",
                pady=(0, 12),
            )
            self._section_frames[section.key] = section_frame
            visible_section_index += 1

    def _build_section(
        self,
        parent: ttk.Frame,
        section: NormalizedSectionSpec,
    ) -> ttk.Frame:
        frame = ttk.LabelFrame(
            parent,
            text=section.title or self._humanize_key(section.key),
        )
        frame.columnconfigure(0, weight=1)

        row = 0

        if section.description:
            desc = ttk.Label(
                frame,
                text=section.description,
                justify="left",
                wraplength=900,
            )
            desc.grid(row=row, column=0, sticky="ew", padx=10, pady=(8, 6))
            row += 1

        content = ttk.Frame(frame)
        content.grid(row=row, column=0, sticky="ew", padx=10, pady=(4, 10))
        content.columnconfigure(0, weight=1)

        visible_field_index = 0
        for field_spec in section.fields:
            if not field_spec.visible:
                continue

            host = ttk.Frame(content)
            host.grid(row=visible_field_index, column=0, sticky="ew", pady=(0, 8))
            host.columnconfigure(0, weight=1)

            widget = FieldFactory.create_field(
                host,
                field=self._to_factory_field_dict(field_spec),
                on_dirty=self._handle_dirty,
                ui=self.ui,
                strict=self.strict_factory,
            )

            widget.grid(row=0, column=0, sticky="ew")

            self._field_widgets_by_id[field_spec.id] = widget
            self._field_widgets_by_key[field_spec.key] = widget
            self._field_specs_by_id[field_spec.id] = field_spec
            self._field_specs_by_key[field_spec.key] = field_spec
            self._field_order.append(field_spec.id)

            self._apply_initial_widget_state(widget, field_spec, section)

            visible_field_index += 1

        return frame

    def _apply_initial_widget_state(
        self,
        widget: FieldWidgetProtocol,
        field_spec: NormalizedFieldSpec,
        section_spec: NormalizedSectionSpec,
    ) -> None:
        effective_enabled = bool(self._form_spec.enabled and section_spec.enabled and field_spec.enabled)
        effective_readonly = bool(field_spec.readonly or section_spec.readonly or self._form_spec.readonly)
        effective_visible = bool(field_spec.visible and section_spec.visible)

        setter = getattr(widget, "set_enabled", None)
        if callable(setter):
            try:
                setter(effective_enabled)
            except Exception:
                pass

        setter = getattr(widget, "set_visible", None)
        if callable(setter):
            try:
                setter(effective_visible)
            except Exception:
                pass

        setter = getattr(widget, "set_readonly", None)
        if callable(setter):
            try:
                setter(effective_readonly)
            except Exception:
                pass

    # ------------------------------------------------------------------
    # Snapshot / defaults
    # ------------------------------------------------------------------

    def apply_defaults(self) -> None:
        self._loading = True
        try:
            for field_id in self._field_order:
                widget = self._field_widgets_by_id.get(field_id)
                spec = self._field_specs_by_id.get(field_id)
                if widget is None or spec is None:
                    continue

                try:
                    widget.set_value(spec.default)
                except Exception:
                    pass
        finally:
            self._loading = False

    def load_snapshot(self, snapshot: Mapping[str, Any] | None) -> None:
        self._loading = True
        try:
            source = dict(snapshot or {})
            self._snapshot = source

            for field_id in self._field_order:
                widget = self._field_widgets_by_id.get(field_id)
                spec = self._field_specs_by_id.get(field_id)
                if widget is None or spec is None:
                    continue

                value = get_snapshot_value(source, spec.key, spec.default)

                try:
                    widget.set_value(value)
                except Exception:
                    try:
                        widget.set_value(spec.default)
                    except Exception:
                        pass
        finally:
            self._loading = False

    # ------------------------------------------------------------------
    # Collection
    # ------------------------------------------------------------------

    def collect_data(self) -> dict[str, Any]:
        """
        Collect values using dotted keys into nested dictionaries.
        """
        data: dict[str, Any] = {}

        for field_id in self._field_order:
            widget = self._field_widgets_by_id.get(field_id)
            spec = self._field_specs_by_id.get(field_id)
            if widget is None or spec is None:
                continue

            try:
                value = widget.get_value()
            except Exception:
                continue

            set_dotted_value(data, spec.key, value)

        return data

    def collect_flat_data(self) -> dict[str, Any]:
        """
        Collect values as flat key -> value mapping.
        """
        data: dict[str, Any] = {}

        for field_id in self._field_order:
            widget = self._field_widgets_by_id.get(field_id)
            spec = self._field_specs_by_id.get(field_id)
            if widget is None or spec is None:
                continue

            try:
                data[spec.key] = widget.get_value()
            except Exception:
                continue

        return data

    # ------------------------------------------------------------------
    # Validation
    # ------------------------------------------------------------------

    def validate_form(self) -> tuple[bool, str]:
        for field_id in self._field_order:
            widget = self._field_widgets_by_id.get(field_id)
            spec = self._field_specs_by_id.get(field_id)
            if widget is None or spec is None:
                continue

            validator = getattr(widget, "validate_value", None)
            if not callable(validator):
                continue

            try:
                result = validator()
            except Exception:
                return False, f"Validation error in field: {spec.key}"

            if result is True or result is None:
                continue

            if result is False:
                return False, f"Invalid value for field: {spec.key}"

            if isinstance(result, str):
                return False, result or f"Invalid value for field: {spec.key}"

            if isinstance(result, tuple) and len(result) == 2:
                ok, message = result
                if not ok:
                    return False, str(message or f"Invalid value for field: {spec.key}")
                continue

            return False, f"Invalid value for field: {spec.key}"

        return True, ""

    # ------------------------------------------------------------------
    # State helpers
    # ------------------------------------------------------------------

    def set_readonly(self, readonly: bool = True) -> None:
        for field_id in self._field_order:
            widget = self._field_widgets_by_id.get(field_id)
            if widget is None:
                continue

            setter = getattr(widget, "set_readonly", None)
            if callable(setter):
                try:
                    setter(bool(readonly))
                except Exception:
                    pass

    def set_enabled(self, enabled: bool = True) -> None:
        for field_id in self._field_order:
            widget = self._field_widgets_by_id.get(field_id)
            if widget is None:
                continue

            setter = getattr(widget, "set_enabled", None)
            if callable(setter):
                try:
                    setter(bool(enabled))
                except Exception:
                    pass

    def focus_first_field(self) -> None:
        for field_id in self._field_order:
            widget = self._field_widgets_by_id.get(field_id)
            if widget is None:
                continue

            focus_method = getattr(widget, "focus_control", None)
            if callable(focus_method):
                try:
                    focus_method()
                    return
                except Exception:
                    continue

    def get_field_widget_by_id(self, field_id: str) -> FieldWidgetProtocol | None:
        return self._field_widgets_by_id.get(str(field_id).strip())

    def get_field_widget_by_key(self, key: str) -> FieldWidgetProtocol | None:
        return self._field_widgets_by_key.get(str(key).strip())

    def get_section_frame(self, section_key: str) -> ttk.Frame | None:
        return self._section_frames.get(str(section_key).strip())

    # ------------------------------------------------------------------
    # Dirty handling
    # ------------------------------------------------------------------

    def _handle_dirty(self) -> None:
        if self._loading:
            return

        if callable(self.on_dirty):
            try:
                self.on_dirty()
            except Exception:
                pass

    # ------------------------------------------------------------------
    # Normalization
    # ------------------------------------------------------------------

    def _normalize_form_spec(self, form_spec: Mapping[str, Any]) -> NormalizedFormSpec:
        if not isinstance(form_spec, Mapping):
            raise TypeError("form_spec must be a mapping")

        form_id = str(form_spec.get("id", "")).strip()
        if not form_id:
            raise ValueError("form_spec must include a non-empty 'id'")

        sections_raw = form_spec.get("sections")
        if not isinstance(sections_raw, list):
            raise ValueError("form_spec must include 'sections' as a list")

        normalized_sections: list[NormalizedSectionSpec] = []
        seen_section_keys: set[str] = set()
        seen_field_ids: set[str] = set()
        seen_field_keys: set[str] = set()

        for section_raw in sections_raw:
            if not isinstance(section_raw, Mapping):
                raise TypeError("each section must be a mapping")

            section_key = str(section_raw.get("key", "")).strip()
            if not section_key:
                raise ValueError("each section must include a non-empty 'key'")

            if section_key in seen_section_keys:
                raise ValueError(f"duplicate section key: '{section_key}'")
            seen_section_keys.add(section_key)

            fields_raw = section_raw.get("fields")
            if not isinstance(fields_raw, list):
                raise ValueError(f"section '{section_key}' must include 'fields' as a list")

            normalized_fields: list[NormalizedFieldSpec] = []

            for field_raw in fields_raw:
                if not isinstance(field_raw, Mapping):
                    raise TypeError(f"field in section '{section_key}' must be a mapping")

                field_id = str(field_raw.get("id", "")).strip()
                if not field_id:
                    raise ValueError(f"field in section '{section_key}' must include non-empty 'id'")

                if field_id in seen_field_ids:
                    raise ValueError(f"duplicate field id: '{field_id}'")
                seen_field_ids.add(field_id)

                field_key = str(field_raw.get("key", field_id)).strip()
                if not field_key:
                    raise ValueError(f"field '{field_id}' must include non-empty 'key' or 'id'")

                if field_key in seen_field_keys:
                    raise ValueError(f"duplicate field key: '{field_key}'")
                seen_field_keys.add(field_key)

                field_type = str(
                    field_raw.get("type", field_raw.get("ui_control", "entry"))
                ).strip().lower()
                if not field_type:
                    field_type = "entry"

                label = str(field_raw.get("label", self._humanize_key(field_key))).strip()

                normalized_fields.append(
                    NormalizedFieldSpec(
                        id=field_id,
                        key=field_key,
                        control=field_type,
                        label=label,
                        description=str(field_raw.get("description", "")).strip(),
                        default=field_raw.get("default"),
                        visible=bool(field_raw.get("visible", True)),
                        enabled=bool(field_raw.get("enabled", True)),
                        readonly=bool(field_raw.get("readonly", False)),
                        required=bool(field_raw.get("required", False)),
                        placeholder=str(field_raw.get("placeholder", "")).strip(),
                        options=list(field_raw.get("options", []) or []),
                        min=field_raw.get("min"),
                        max=field_raw.get("max"),
                        step=field_raw.get("step"),
                        ui=dict(field_raw.get("ui", {}) or {}),
                        validation=dict(field_raw.get("validation", {}) or {}),
                        tags=[str(tag) for tag in list(field_raw.get("tags", []) or [])],
                        metadata=dict(field_raw.get("metadata", {}) or {}),
                        raw=dict(field_raw),
                    )
                )

            normalized_sections.append(
                NormalizedSectionSpec(
                    key=section_key,
                    title=str(section_raw.get("title", self._humanize_key(section_key))).strip(),
                    description=str(section_raw.get("description", "")).strip(),
                    visible=bool(section_raw.get("visible", True)),
                    collapsed=bool(section_raw.get("collapsed", False)),
                    readonly=bool(section_raw.get("readonly", False)),
                    enabled=bool(section_raw.get("enabled", True)),
                    metadata=dict(section_raw.get("metadata", {}) or {}),
                    fields=normalized_fields,
                )
            )

        return NormalizedFormSpec(
            id=form_id,
            title=str(form_spec.get("title", self._humanize_key(form_id))).strip(),
            description=str(form_spec.get("description", "")).strip(),
            mode=str(form_spec.get("mode", "form")).strip() or "form",
            readonly=bool(form_spec.get("readonly", False)),
            enabled=bool(form_spec.get("enabled", True)),
            metadata=dict(form_spec.get("metadata", {}) or {}),
            sections=normalized_sections,
        )

    # ------------------------------------------------------------------
    # Adapters / helpers
    # ------------------------------------------------------------------

    def _to_factory_field_dict(self, spec: NormalizedFieldSpec) -> dict[str, Any]:
        """
        Adapt declarative form spec field -> FieldFactory field dict.
        """
        field_dict = dict(spec.raw)

        field_dict["key"] = spec.key
        field_dict["label"] = spec.label
        field_dict["help"] = spec.description
        field_dict["default"] = spec.default
        field_dict["readonly"] = spec.readonly
        field_dict["visible"] = spec.visible
        field_dict["enabled"] = spec.enabled
        field_dict["required"] = spec.required
        field_dict["choices"] = list(spec.options)
        field_dict["min"] = spec.min
        field_dict["max"] = spec.max
        field_dict["step"] = spec.step

        # Declarative form spec uses type/ui_control.
        field_dict["ui_control"] = spec.control
        field_dict["type"] = spec.control

        return field_dict

    @staticmethod
    def _humanize_key(value: str) -> str:
        cleaned = str(value or "").replace(".", " ").replace("_", " ").strip()
        return " ".join(part.capitalize() for part in cleaned.split())