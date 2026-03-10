# PLUGIN_FORMAT_SPEC.md

```markdown
# PLUGIN FORMAT SPECIFICATION
DocGraph Platform

Version: 1.0  
Status: Stable  
Scope: system / user / dev plugins

---

# 1. Purpose

This document defines the **standard structure and contract for plugins** used in the DocGraph platform.

The goal is to ensure that all plugins:

- follow a predictable structure
- are portable
- can be validated automatically
- can be loaded dynamically
- do not break system invariants

This specification applies to plugins located in:

system/plugins/  
user/plugins/  
dev/plugins/

---

# 2. Plugin Types

Plugins are classified by **scope**.

| Type | Location | Purpose |
|-----|-----|-----|
| system | system/plugins | Extensions shipped with the platform |
| user | user/plugins | User-installed functional extensions |
| dev | dev/plugins | Developer tooling and diagnostics |

The **plugin format is identical for all types**.

Only the **execution context and allowed outputs differ**.

---

# 3. Standard Plugin Directory Structure

Each plugin must follow the structure below.

```

<plugin_name>/

```
plugin_manifest.json
README.md

tools/
    <tool_scripts>.py

docs/
    <optional documentation>

assets/
    <optional assets>

tests/
    <optional tests>
```

````

Minimum required files:

- `plugin_manifest.json`
- `tools/` directory

Recommended:

- `README.md`

---

# 4. plugin_manifest.json

Each plugin must provide a manifest describing its capabilities.

Example:

```json
{
  "plugin_id": "example_plugin",
  "name": "Example Plugin",
  "version": "0.1.0",
  "description": "Example plugin demonstrating plugin format.",
  "plugin_type": "dev",

  "entrypoints": [
    {
      "type": "tool",
      "module": "tools.example_tool",
      "entry_function": "main"
    }
  ],

  "outputs": [
    "dev/runs/example_plugin/"
  ]
}
````

---

# 5. Manifest Fields

| Field       | Required    | Description            |
| ----------- | ----------- | ---------------------- |
| plugin_id   | yes         | Unique identifier      |
| name        | yes         | Human-readable name    |
| version     | yes         | Plugin version         |
| description | yes         | Short description      |
| plugin_type | yes         | system / user / dev    |
| entrypoints | yes         | Executable entrypoints |
| outputs     | recommended | Declared output paths  |

---

# 6. Entrypoints

Entrypoints define executable capabilities provided by the plugin.

Example:

```json
{
  "type": "tool",
  "module": "tools.generate_report",
  "entry_function": "main"
}
```

Fields:

| Field          | Description                                |
| -------------- | ------------------------------------------ |
| type           | execution type (tool, service, etc.)       |
| module         | Python module path relative to plugin root |
| entry_function | callable function                          |

---

# 7. Execution Rules

Plugins must follow these rules:

1. Plugins must not modify `system/` files.
2. Plugins must declare all output locations.
3. Plugins must avoid hardcoded absolute paths.
4. Plugins must be portable.

---

# 8. Output Locations

Outputs must follow scope rules.

| Plugin Type | Allowed Output Location |
| ----------- | ----------------------- |
| system      | user/output/            |
| user        | user/output/            |
| dev         | dev/runs/               |

Plugins should never write outside declared paths.

---

# 9. Security Restrictions

Plugins must not:

* overwrite system configuration
* modify runtime registry
* alter plugin registry files
* access restricted directories

---

# 10. Optional Components

Plugins may optionally include:

### assets/

Static resources:

```
assets/
    icons/
    templates/
```

### docs/

Plugin documentation.

```
docs/
    usage.md
```

### tests/

Unit tests.

```
tests/
    test_plugin.py
```

---

# 11. Plugin Discovery

Plugins are discovered through a **registry file**.

Examples:

```
system/config/runtime_registry.json
dev/config/dev_plugins_registry.json
```

The registry determines:

* which plugins are available
* whether they are enabled
* how they are executed

---

# 12. Version Compatibility

Plugins should declare compatibility with platform versions when required.

Example:

```
"platform_compatibility": ">=1.0"
```

---

# 13. Validation

Plugins may be validated by automated tools checking:

* manifest correctness
* structure compliance
* entrypoint availability
* output declarations

---

# 14. Design Principles

The plugin system follows these principles:

* portability
* declarative configuration
* explicit outputs
* separation of runtime and development tooling
* predictable structure

---

# 15. Summary

A valid plugin must provide:

* a plugin directory
* a plugin manifest
* at least one tool entrypoint
* declared outputs
* portable paths

This specification guarantees that plugins can be safely integrated into the platform.

````

---
