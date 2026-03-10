# Contributing to DocGraph

Thank you for your interest in contributing to **DocGraph**.

DocGraph is a portable and reproducible document tooling platform designed with strong architectural constraints. Contributions are welcome, but changes should respect the core design principles of the system.

---

# 1. Before Contributing

Before submitting changes, please:

1. Read the project documentation:

   * `README.md`
   * `USER_MANUAL.md`
   * `BASELINE_PLATFORM.md`
2. Review the architectural structure of the project.
3. Open an Issue if the proposed change affects platform behavior or architecture.

This helps ensure that contributions align with the long-term design of the platform.

---

# 2. Types of Contributions

The following types of contributions are welcome:

### Bug fixes

Corrections that improve stability or correctness.

### Documentation improvements

Updates or clarifications to manuals and documentation.

### New tools

Additional tools that extend the capabilities of the platform without modifying core system behavior.

### Development utilities

Developer-oriented scripts that improve inspection, diagnostics, or testing.

---

# 3. Architectural Constraints

DocGraph follows a strict separation between operational domains.

```
system/   core platform configuration and tools
user/     user data, execution runs and outputs
dev/      development utilities and inspection scripts
```

Contributions must respect this separation.

In particular:

* The `system/` directory must remain stable and controlled.
* User-generated data must remain inside `user/`.
* Development utilities must be placed in `dev/`.

Changes that affect the platform architecture should be discussed before implementation.

---

# 4. Tool Integration

New tools should follow the platform conventions:

* Tools must be registered through the runtime configuration.
* Tools must not modify files inside `system/`.
* Generated artifacts should be stored under:

```
user/output/<tool_id>/<timestamp>/
```

Execution traces should be stored under:

```
user/runs/<tool_id>/<timestamp>/
```

This ensures traceability and reproducibility.

---

# 5. Code Style

When contributing code:

* Use clear and readable Python code.
* Prefer explicit logic over complex abstractions.
* Avoid introducing unnecessary dependencies.
* Maintain compatibility with the existing runtime environment.

---

# 6. Pull Requests

When submitting a pull request:

1. Fork the repository.
2. Create a feature branch.
3. Implement the proposed changes.
4. Provide a clear description of the modification.
5. Reference the related Issue if applicable.

Pull requests should remain focused and avoid mixing unrelated changes.

---

# 7. Reporting Issues

If you encounter a problem:

* open a GitHub Issue
* describe the problem clearly
* include steps to reproduce the issue
* provide relevant logs if available

Execution traces can usually be found under:

```
user/runs/
```

---

# 8. Project Philosophy

DocGraph prioritizes:

* portability
* reproducibility
* traceability
* architectural clarity

All contributions should aim to reinforce these principles.

---

# 9. License

By contributing to this project, you agree that your contributions will be licensed under the terms of the project's MIT License.

---
