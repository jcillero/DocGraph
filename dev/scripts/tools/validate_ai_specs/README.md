# validate_ai_specs

This helper validates F9.3/F9.4 declarative AI contracts under `resources/ai/`.

The active implementation lives in the Rust `verify_progress` crate and is invoked through `dev/scripts/validate_ai_specs.bat`.

It only reads JSON resources, checks structural governance rules, and writes a regenerable report to `user/output/validate_ai_specs_report.json`.

It does not:

- execute LLMs
- simulate LLMs
- invoke tools
- create runners
- create orchestration
- create operational AI traces
- mutate documents
- integrate with Rust crates or `spec_runtime`

`resources/ai/` is declarative only in the current phase.
