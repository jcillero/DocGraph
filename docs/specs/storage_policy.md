# storage_policy

## Purpose

Define the F9-ready storage model for user data, project data, and regenerable indexes.

The current model is file-based.

## Scope F9

F9 declares:

- file-based storage
- checksum deduplication
- regenerable JSONL indexes
- `user/file_store/blobs/<sha256_hash>/` as the only physical blob authority
- governed JSON schemas for `file_ref`, `BlobRecord`, `StoredObject`, `UsageRef`, and `DerivationManifest` as declarative contracts only
- source files plus manifests as source of truth
- SQLite disabled
- Oxigraph disabled

Indexes are derived data and may be rebuilt.

## file_store as physical authority

`file_store` is the physical storage authority for governed blob content:

- blobs live under `user/file_store/blobs/<sha256_hash>/`
- blobs are content-addressed by `sha256`
- blobs are immutable
- blob content is not duplicated by usage kind
- blob content is not stored under project folders

The blob store is the only physical source of truth for governed persisted content.

`file_store` belongs to the governed `SANDBOX` domain and does not imply `HOST` or `EXTERNAL` authority.

## StoredObject as logical abstraction

`StoredObject` is the canonical logical abstraction above physical blob storage.

It must include:

- `object_ref`
- `object_kind`
- `content_ref`
- `metadata.semantic`
- `metadata.technical`
- `metadata.operational`
- `lifecycle_state`
- `derivation_capabilities`
- `quad_flags`

`StoredObject` is logical, not physical:

- one blob may map to multiple `StoredObject` declarations
- `StoredObject` does not imply project exposure
- metadata is mandatory even when its subobjects are empty
- metadata must not contain secrets
- metadata must be sanitized before becoming portable governed state
- portable truth must not depend on absolute host paths

## refs vs storage

Refs declare usage; they do not duplicate content and they do not define storage authority.

- `file_ref` is a stable content identifier, not a filename or path
- `usage_ref` declares usage, not ownership of physical content
- schema validity does not imply project exposure
- `project_manifest` remains the exposure authority
- storage indexes and refs must not leak secrets or private absolute paths

## Intake dedup identity

F12.8 defines storage dedup hardening for governed file intake.

The storage layer treats content hash as physical identity evidence:

- same governed byte-level hash means same physical blob candidate
- different governed byte-level hash means different physical content
- missing or unverifiable hash means physical sameness is unknown

Identity relationship:

- `content_hash` is hash evidence
- `file_ref` is content identity
- `StoredObject` or `stored_object_candidate_ref` is logical usage context
- `intake_item_id` is intake selection identity

Storage reuse policy:

- physical blob reuse is preferred when content hash matches and the existing blob is governed and immutable
- logical stored object candidates may be distinct while sharing the same `file_ref`
- duplicate intake selections must not duplicate blob bytes when reuse is allowed
- duplicate intake selections must not collapse metadata, comments, traces, owners, or blocking reasons
- blob reuse does not imply project exposure
- blob reuse does not imply derivative generation
- blob reuse does not update `project_manifest`

## derivatives and semantic boundary

Derivatives are:

- deterministic
- regenerable
- non-semantic

Examples include text extraction, pages, chunks, previews, and structured metadata.

Derivative presence or folder presence alone is not authority.

The semantic layer remains separate:

- semantic quads are not generated here
- semantic runtime is not active here
- RDF, Oxigraph, and SPARQL are not enabled here
- storage invalidation flows downstream only through derivatives, semantic, and graph layers
- no downstream semantic, graph, or analysis layer may mutate storage
- downstream semantic or RDF layers must not re-expand sanitized storage material

## Invariants

- `INV-STORAGE-001`: the same physical file SHOULD NOT be duplicated across `knowledge/files`, chats, or documents; stable references SHOULD be preferred.
- `INV-STORAGE-002`: `file_store/blobs` is the only physical blob authority.
- `INV-STORAGE-003`: `StoredObject` is logical, not storage.
- `INV-STORAGE-004`: schema presence does not imply runtime validation.
- `INV-STORAGE-005`: schema validity does not imply project exposure.
- `INV-STORAGE-006`: schema validity does not imply execution authority.
- `INV-STORAGE-007`: `file_ref` is content-addressed and path-independent.
- `INV-STORAGE-008`: `usage_ref` declares usage and does not duplicate physical content.
- `INV-STORAGE-009`: derivation-manifest state must be explicit and must not be inferred from folder presence.
- `INV-STORAGE-010`: derivatives are deterministic and non-semantic.
- `INV-STORAGE-011`: semantic preparation remains separate and inactive at this layer.
- `INV-STORAGE-012`: storage is root authority for downstream invalidation flow.
- `INV-STORAGE-013`: no downstream layer may mutate physical storage authority.
- `INV-STORAGE-014`: storage metadata and indexes must not expose secrets or private absolute host paths.
- `INV-STORAGE-015`: duplicate intake content should reuse physical blob identity when hash and policy allow.
- `INV-STORAGE-016`: physical blob reuse must not merge logical `StoredObject` or intake item metadata.
- `INV-STORAGE-017`: missing or unverifiable content hash must not trigger assumed deduplication.
- Source files plus governed manifests remain the source of truth.
- Regenerable indexes MUST NOT become authoritative state.
- Selected external folders are not project truth.
- Sandbox working copies are project-owned working artifacts when future runtime creates them.
- Host absolute paths must not become portable truth.
- Project storage must not contain secret values.
- Credential references are not credentials.
- Logs, manifests, graph data, and tool outputs must not contain secret values.

## Forbidden responsibilities

Storage policy must not:

- introduce SQLite now
- introduce Oxigraph now
- make JSONL indexes authoritative
- hide source truth inside chat
- create onboarding projects
- bypass governed manifests

## Future F10/F11 notes

F10 may rely on file-based policy for LLM/tool context selection.

Future compatibility:

- `StoredObject` may feed future semantic quad generation
- `StoredObject` may feed future RDF projection
- `StoredObject` may feed future graph analysis
- approved semantic relation edges may reference governed `StoredObject` evidence or source refs in the future
- semantic quad sources must align with governed `StoredObject`, `file_ref`, or equivalent governed source references
- semantic quad material remains separate from deterministic derivatives and from physical blob authority
- future retained semantic material must remain bounded by governed semantic storage limits rather than unrestricted accumulation

These future consumers do not activate semantic runtime, RDF/Oxigraph, or execution here.

Stored content and derivatives may support future quad generation only through explicit governed triggers and bounded scope.

Stored content and deterministic derivatives may support future governed reuse when content identity and governed context match.

Any future RDF dataset remains a projection target only and does not replace physical blob authority, `StoredObject`, or `project_manifest`.

## F11.7 manifest and trace storage alignment

Future `ToolRunManifest` and `TraceRecord` artifacts must follow governed storage discipline:

- outputs must be owner-scoped
- no project-root `outputs/`
- manifest path remains future
- trace path remains future
- filesystem presence does not imply project exposure
- `project_manifest` remains exposure authority

Rules:

- `F11.7` creates no files or folders
- `F11.7` does not update `project_manifest`
- `F11.7` does not update graph
- `F11.7` does not update registry

F11 should audit checksum behavior, index regeneration, and optional database decisions before any database-backed storage is introduced.

## F12.0 / F11.RUNTIME-0 output discipline proposal

`F12.0 / F11.RUNTIME-0` is proposal-only and creates no files.

A future first runtime opening must produce, when explicitly implemented later:

- owner-scoped result
- `tool_run_manifest.json`
- trace record or trace metadata

It must not produce:

- project-root `outputs/`
- raw private host paths
- secrets
- source-artifact mutation
- registry mutation
- graph mutation
- `project_manifest` mutation

Until a later implementation slice opens runtime, `ToolRunManifest` remains future-required and `TraceRecord` remains future-required.

## F12.1 text.measure storage gate

`F12.1` is gate-only and creates no files.

For the future `text.measure` implementation slice, the output contract must be:

- `result.json`
- owner-scoped location only
- `owner_ref` mandatory
- no project-root `outputs/`
- no raw private host paths
- no secrets
- no source-artifact mutation

`ToolRunManifest` is mandatory for future execution, but manifest path policy is declared only and no `tool_run_manifest.json` is created in `F12.1`.

`TraceRecord` is mandatory for future execution, but trace path policy is declared only and no `TraceRecord` is persisted in `F12.1`.

## F12.2A text.measure storage implementation plan

`F12.2A` is plan-only and creates no files.

Future `F12.2B` storage scope is limited to `text.measure` and `crates/tool_runtime`.

Future allowed storage effects:

- create an owner-scoped output directory
- create owner-scoped `result.json` after successful execution
- create `tool_run_manifest.json` after successful execution or governed failed run if explicitly allowed
- create `TraceRecord`-compatible metadata only if already declared by `F12.1` and `F11.7`

Future storage acceptance checklist:

- `owner_ref` is mandatory
- `trace_ref` is mandatory
- missing `owner_ref` blocks execution
- missing `trace_ref` blocks execution
- output path cannot escape owner-scoped sandbox
- no project-root outputs
- no `HOST` write
- no source-artifact mutation
- no registry, graph, or `project_manifest` mutation
- manifest contains inputs, configuration, `owner_ref`, outputs, status, and trace data
- manifest contains no private absolute host paths
- result, manifest, and trace metadata contain no secrets

## F12.4 governed file intake storage alignment

`docs/specs/file_intake.md` owns file intake semantics.

F12.4 declares future intake storage only. It creates no files, blobs, manifests, registry entries, graph entries, indexes, or derivatives.

Future intake storage must preserve these storage rules:

- physical storage must be governed
- blobs are canonical physical storage
- refs declare usage
- types classify/navigation only
- indexes are derivable accelerators
- no project-root `outputs/`
- no `assets/` as runtime
- no passive filesystem exposure
- `project_manifest.json` remains exposure authority
- source host paths must not be persisted as portable truth

F12.4 intake comments are future governed metadata only.

Comment storage rules:

- comments belong to `IntakeItem -> StoredObject metadata`
- comments are not separate documents
- comments are not chat logs
- comments are not filesystem annotations
- comments are not blobs
- comments must be sanitized before durable storage
- comments must not contain secrets, credentials, tokens, raw sensitive data, or private absolute host paths
- comments do not create project exposure
- comments do not authorize derivation

## F12.5 file intake storage plan

`F12.5` is plan-only and creates no files.

Future `F12.6` may write only to an owner-scoped intake directory or governed file-store location when the storage policy path is explicit.

Future `F12.6` may create metadata JSON for an intake item or stored object candidate, and may create an optional batch manifest only if explicitly declared before implementation.

Future `F12.6` must not create project-root outputs, write to source folders, use `assets/` as runtime storage, mutate `project_manifest.json`, generate `registry.json`, write graph entries, or generate derivatives.

Future intake metadata must not persist private absolute host paths as portable truth.

## F12.8 intake dedup hardening storage strategy

`F12.8` is SPEC-ONLY / HARDENING-STRATEGY-ONLY.

It opens no new runtime and creates no files.

Future dedup hardening must preserve:

- `file_store/blobs/<sha256_hash>/` as the only physical blob authority
- `file_ref` as content identity
- `StoredObject` as logical metadata and usage context
- `intake_item_id` as the explicit selection event identity
- `project_manifest` as the only project exposure authority

Future dedup hardening must not create project-root outputs, write to source folders, use `assets/` as runtime storage, mutate `project_manifest.json`, generate `registry.json`, write graph entries, generate derivatives, call `document_text_runtime`, or use `tool_runtime` orchestration.

## F13.0 project exposure storage alignment

`F13.0` is SPEC-ONLY / GATE-ONLY and creates no files.

Project exposure remains separate from storage:

- `file_store/blobs/<sha256_hash>/` is physical blob authority
- `StoredObject` is logical metadata and usage context
- `file_ref` is content identity
- `project_manifest.json` is the only project exposure authority

Storage presence must not create exposure:

- blob presence does not expose a file to a project
- `StoredObject` metadata presence does not expose a file to a project
- intake metadata presence does not expose a file to a project
- registry presence does not expose a file to a project
- graph presence does not expose a file to a project

Future project exposure may reference governed storage objects only through sanitized refs and after request, candidate, and human confirmation defined by `docs/specs/file_intake.md`.

F13.0 must not create project-root outputs, write source folders, use `assets/` as runtime storage, mutate `project_manifest.json`, generate `registry.json`, write graph entries, generate derivatives, call `document_text_runtime`, call `tool_runtime`, or create rollback/revocation storage effects.

## F13.1 legacy storage-bypass hardening

`F13.1` is SPEC-only and creates no files.

Storage and copied files remain non-authoritative even when legacy helpers exist:

- a file copied by `import_project_document` must not become exposed by storage presence
- chat-resource storage must not become project storage authority
- derivative files must not become exposure authority
- any legacy copy path must remain separate from manifest-governed exposure

## F13.4 manifest exposure storage hardening

`F13.4` is SPEC-only and creates no files.

Future manifest exposure remains storage-aware but storage-non-authoritative.

Rules:

- `ManifestExposureEntry` may reference governed stored objects only through portable refs
- blob reuse is allowed but must not collapse logical exposure identity, request identity, candidate identity, confirmation identity, comments, `owner_ref`, or `trace_ref`
- manifest exposure entries must not persist private absolute host paths, raw payloads, or secrets
- manifest exposure entries must not imply `registry.json` generation, graph writes, or derivative creation
- storage presence, checksum presence, or stored-object metadata presence must remain insufficient for project exposure

## F13.5A manifest exposure storage checklist

`F13.5A` is SPEC-only and creates no files.

Future storage checklist for `F13.5B`:

- manifest exposure may reference existing governed storage only through portable refs
- no raw host absolute path may appear in manifest exposure outputs
- duplicate physical blob reuse must remain storage-level only and must not collapse logical exposure identity
- exposure runtime must not create registries, graph artifacts, or derivatives as storage side effects
