# security_sanitization_policy

## Status

DECLARED_ONLY / F9-F10_PREP governance.

This document does not implement runtime sanitization, encryption, secret scanning, key management, provider execution, tool execution, filesystem mutation, or RDF runtime.

## Purpose

Define governed security and sanitization rules for metadata, derivatives, semantic quads, relations, RDF projection, and trace records.

The goal is to prevent leakage of sensitive data across all declarative and future-derived layers.

## Sensitive data definition

Sensitive data includes, at minimum:

- credentials
- access tokens
- API keys
- bearer tokens
- secret values embedded in configuration or prompts
- private absolute host paths
- personal data when present

Personal data may include, when applicable:

- personal email addresses
- phone numbers
- government or institutional identifiers
- home addresses
- other directly identifying personal fields not required for governed traceability

## Forbidden locations

Sensitive data must not appear in:

- `StoredObject.metadata.semantic`
- `StoredObject.metadata.technical`
- `StoredObject.metadata.operational`
- deterministic derivatives
- semantic quads
- semantic relations
- RDF projections
- trace records
- graph-analysis artifacts
- storage indexes

The only allowed representation is a governed non-secret reference, redacted value, masked value, or stable hash when a trace need exists.

## Sanitization rules

Sanitization must be governed by the following strategies:

- redact full secret values when the field meaning must remain visible
- mask partial values only when the residual fragment does not reconstruct the secret
- hash values only when stable correlation is required without exposing the underlying value
- replace private absolute host paths with portable governed refs or sanitized placeholders
- minimize personal data to the least information necessary for governed traceability

### Redaction patterns

Allowed conceptual redaction patterns include:

- `[REDACTED_CREDENTIAL]`
- `[REDACTED_TOKEN]`
- `[REDACTED_API_KEY]`
- `[REDACTED_PERSONAL_DATA]`
- `[REDACTED_PRIVATE_PATH]`

Redaction must preserve type meaning without preserving the secret.

### Hashing rules

Hashing may be used when:

- a stable comparison key is required
- a duplicate-detection or trace-correlation purpose exists
- the unhashed value must remain hidden

Hashing must not be treated as semantic authority, approval, or execution permission.

### Masking rules

Masking may be used when:

- a human needs to distinguish multiple governed references
- the remaining visible fragment is non-sensitive

Masking must not expose enough material to reconstruct the original secret.

## Layer rules

### Storage and metadata

- metadata must not store credentials, token values, API keys, private absolute paths, or personal data beyond governed minimum need
- storage indexes must not leak secrets through duplicated metadata or path capture
- `file_ref`, `object_ref`, and governed ids remain preferred over raw host-specific path truth

### Deterministic derivatives

- derivatives must not duplicate secret-bearing source fragments unless a future governed exception explicitly opens that behavior
- structured derivative metadata must be sanitized before becoming portable declarative state

### Semantic layer

- semantic quads and relations must not contain secret values as subject, predicate, object, evidence payload, metadata, or trace material
- semantic evidence must reference governed sources rather than duplicate sensitive raw text
- no semantic proposal may treat secret-bearing material as approved knowledge

### RDF projection

- RDF projections must remain sanitized derived views only
- no secrets may appear in IRIs, literals, named graphs, provenance links, or governance graphs
- RDF must not re-expand previously sanitized material

### Traceability

- traceability is required, but traceability must not leak secrets
- trace refs, prompt refs, model refs, tool refs, and source refs are allowed only as non-secret identifiers
- human-review refs such as `reviewer_ref` are allowed only as non-secret identifiers and must not expose credentials or personal data beyond governed minimum need
- trace metadata must remain sufficient for audit without exposing underlying secret values
- future readonly presentation surfaces such as `docs/specs/system_view.md` must expose only sanitized references, summaries, and explanation state
- `FULL_TRACE` review presentation must remain sanitized and must not expose raw payloads, secrets, private absolute host paths, or redacted content
- `AuthorizedExecutionRequest.safety_snapshot` and owner-requirement metadata must remain sanitized summaries and must not include raw payloads, secrets, private absolute host paths, or credential material
- `SingleToolExecution.input_refs`, `output_plan`, result placeholders, trace refs, and future manifest metadata must remain sanitized summaries and must not include raw payloads, secrets, private absolute host paths, or credential material
- future `ToolRunManifest.inputs`, `configuration`, `outputs`, and future `TraceRecord.links` must remain sanitized and must not include raw payloads, secrets, or private absolute host paths

## F12.1 text.measure sanitization gate

`F12.1` is gate-only and creates no runtime sanitization.

For the later first `text.measure` implementation slice:

- accepted input must be a governed text input ref
- raw payloads are forbidden
- secrets are forbidden
- private absolute host paths are forbidden
- unsafe input must block future execution
- stale input must block future execution
- manifest metadata must remain sanitized
- trace metadata must remain sanitized

Future-only error codes include:

- `unsafe_input`
- `stale_input`

## F12.2A text.measure sanitization implementation plan

`F12.2A` is plan-only and creates no runtime sanitization.

Future `F12.2B` must enforce:

- governed text input only
- no raw payload persistence
- no secrets in input-derived metadata
- no private absolute host paths in manifest, result, or trace metadata
- unsafe input blocks execution
- stale input blocks execution if staleness is present
- `owner_ref` is mandatory
- `trace_ref` is mandatory
- manifest metadata remains sanitized
- trace metadata remains sanitized

Future `F12.2B` must not call providers, access network, invoke external binaries, run LLM tools, run agent tools, or introduce document/semantic/graph mutation.

## Non-goals

This policy does not open:

- encryption runtime
- secret vault runtime
- active credential resolution
- secret scanning runtime
- RDF runtime
- graph analysis runtime
- execution authority

## Security and sanitization invariants

- `INV-SEC-001`: no secrets may appear in metadata.
- `INV-SEC-002`: no secrets may appear in deterministic derivatives.
- `INV-SEC-003`: no secrets may appear in semantic quads or relations.
- `INV-SEC-004`: no secrets may appear in RDF projections.
- `INV-SEC-005`: no secrets may appear in trace records or storage indexes.
- `INV-SEC-006`: private absolute host paths must not become portable truth.
- `INV-SEC-007`: sanitization must preserve traceability without exposing secret values.
- `INV-SEC-008`: hashing, masking, and redaction do not imply authority, approval, or execution permission.
- `INV-SEC-009`: semantic and RDF layers must not re-expand previously sanitized material.
- `INV-SEC-010`: personal data must be minimized and sanitized when not strictly required for governed traceability.

## Related specs

- `docs/specs/storage_policy.md`
- `docs/specs/semantic_quad_model.md`
- `docs/specs/rdf_projection_policy.md`
- `docs/specs/graph_analysis_policy.md`
- `docs/specs/system_view.md`

## F12.4 file intake sanitization alignment

`docs/specs/file_intake.md` owns governed file intake semantics.

Future intake metadata must be sanitized and must not contain secrets, raw payloads, credentials, unsanitized user paths, or private absolute host paths as portable truth.

Original filenames must be sanitized before durable metadata use.

Source paths may be transient future runtime inputs only.

F12.4 intake comments are governed metadata and must be sanitized before durable persistence.

Comment sanitization must:

- remove or flag secrets
- remove or flag credentials
- remove or flag tokens
- remove or flag private absolute host paths
- reject or flag raw sensitive data according to active policy
- enforce maximum length
- enforce safe encoding
- reject executable content

Valid `comment_sanitization_state` values are:

- `safe`
- `flagged`
- `rejected`

Rejected comments block ingestion only when the active policy explicitly requires blocking.

## F12.5 file intake sanitization plan

`F12.5` is plan-only and adds no sanitization runtime.

Future `F12.6` must sanitize:

- original filenames
- source display labels
- intake metadata
- optional `user_comment`

Future `F12.6` metadata must not contain secrets, credentials, tokens, raw sensitive data, private absolute host paths as portable truth, or unsanitized user paths.

Future `F12.6` must use typed sanitization or blocked reason codes including `sanitization_failed`, `comment_contains_secrets`, `comment_contains_private_path`, `comment_too_large`, and `comment_sanitization_failed`.
