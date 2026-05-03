# Migration Baseline

The Python application remains the historical doctrinal and behavioral migration reference.

The Rust sandbox is now the active engineering center for the successor implementation.

Python is not the current execution path for this Rust sandbox and is not required to operate it.

## Reference areas in Python

- workspace separation and portability doctrine
- declarative loading of specs, config, and registries
- project runtime and manifest/ref-driven visibility
- IO and utility discipline
- LLM configuration, runtime policy, model catalog, tool policy, and observability
- UI controller boundaries, text externalization, and theme governance

## Rust responsibilities already opened

- workspace boundaries and path discipline
- declarative spec and registry interpretation
- governed project runtime pipeline
- thin application-service consumption above the project vertical
- declarative tool catalog modeling and selection
- local validation and engineering tooling inside the workspace

## What still remains primarily in Python

- the active product runtime
- mature operational workflows
- mature UI behavior
- existing LLM execution paths
- migration-support behavior that has not yet been ported under Rust governance

## Migration rule

Rust does not replace Python by imitation of incidental details.

It replaces Python by progressively absorbing governed responsibilities into explicit Rust contracts, crates, and validated phases.

## Asset migration note

Inherited visual assets may first be consolidated in `assets/` inside this sandbox as a temporary staging area.

That staging step is for:

- cleanup
- filename normalization
- selection of official candidates

`assets/` is not the canonical runtime resource location. Approved assets may later be promoted into `resources/` if governance requires it.
