# File Intake

Status: F12.4 SPEC-ONLY / DECLARATIVE ONLY.

This spec owns governed file intake semantics for DocGraph.

`F12.4` defines the contract by which explicit user-selected external files may later become portable, traceable, project-owned inputs.

`F12.4` does not implement runtime, copy files, scan host folders, mutate `project_manifest.json`, create registry entries, create graph entries, derive text or chunks, execute tools, call providers, use network, invoke external binaries, open LLM tool calling, or open semantic/RDF/Oxigraph runtime.

## Governed intake chain

The canonical F12.4 chain is:

`user selection intent -> FileIntakeRequest -> FileIntakeDraft -> FileIntakeCandidate -> FileIntakePlan -> future governed runtime -> future StoredObject / project exposure / derivatives`

Filesystem presence is not project exposure.

`project_manifest.json` remains the exposure authority.

## Contract artifacts

`FileIntakeRequest`

- captures explicit user selection intent
- may reference one or multiple selected files
- carries `owner_ref` when known
- carries `trace_ref` when known
- does not classify, copy, hash, expose, derive, or register files

`FileIntakeDraft`

- normalizes the request into a reviewable draft
- records sanitized selection summaries
- may carry optional per-file `user_comment` values captured as text-only metadata proposals
- records missing preconditions
- remains non-mutating

`FileIntakeCandidate`

- represents a single candidate item after pre-runtime validation and classification
- carries `intake_item_id`
- carries sanitized metadata
- may be blocked
- does not imply project exposure

`FileIntakePlan`

- declares future storage, exposure, and derivation intent
- requires `owner_ref` and `trace_ref` before durable import
- remains declarative in F12.4

`FileIntakeResult`

- placeholder for future runtime result only
- must not exist as a durable result in F12.4

`IntakeBatch`

- groups one explicit selection flow
- carries an `intake_batch_ref`
- may contain one or more `IntakeItem` values

`IntakeItem`

- represents one selected source item in a batch
- carries `intake_item_id`
- links to source, classification, metadata, storage plan, exposure plan, derivation plan, and trace summary by refs or sanitized summaries
- may carry optional user comment metadata tied to that intake item
- comment fields are descriptive only and must not affect classification, capability, exposure authority, or derivation eligibility

`IntakeSource`

- describes external/host source input
- source paths are transient runtime inputs only
- source paths must not become portable identity

`IntakeClassification`

- records format classification as a hint
- does not imply exposure, derivation, or semantic eligibility

`IntakeMetadata`

- records sanitized portable metadata only
- must not include secrets, raw payloads, credentials, or unsanitized private absolute host paths

`IntakeStoragePlan`

- declares future storage intent aligned with `docs/specs/storage_policy.md`
- does not invent final layout

`IntakeExposurePlan`

- declares whether and how a stored object may later become visible to a project surface
- exposure requires `project_manifest.json` governance

`IntakeDerivationPlan`

- declares possible future deterministic derivatives
- does not generate derivatives in F12.4

`IntakeTraceSummary`

- records sanitized trace linkage
- carries `trace_ref`
- does not authorize import or exposure

## Selection intent

UI may capture explicit user selection.

Selection may include one or multiple files.

UI must not:

- classify files
- copy files
- hash files
- expose files
- derive text, chunks, thumbnails, or previews
- register files
- mutate storage
- authorize intake

UI may provide an optional comment field per selected file during intake.

UI comment capture rules:

- capture text only
- do not interpret comment meaning
- do not classify based on comment
- do not infer metadata from comment
- do not persist comment directly
- do not store anything directly
- do not run sanitization policy locally beyond basic input safety

UI emits intent only.

When a comment is present, UI emits a `FileIntakeDraft` value that includes the per-file `user_comment` as governed intake metadata awaiting sanitization and promotion.

## Source boundary

Source files are external/host inputs.

Source folders are readonly.

Original source files must not be written back to.

Absolute host paths may be used transiently by a future governed runtime, but they must not be persisted as canonical portable identity.

Source path is not identity.

Filename is not identity.

## Pre-runtime validation

F12.4 declares future validation requirements only.

Required validation checks:

- file exists
- file is readable
- file is not a directory unless future folder intake is explicitly opened
- file size policy
- extension and MIME classification as hints only
- unsupported format becomes blocked, not guessed
- duplicate candidate handling
- unsafe filename handling
- private absolute host path sanitization
- secret metadata minimization

Validation must not copy, import, expose, derive, register, or mutate.

## Classification

Classification fields:

- `original_filename_sanitized`
- `extension`
- `detected_kind`
- `declared_kind`
- `media_type_hint`
- `confidence`
- `classification_source`
- `supported_state`

Initial categories:

- `text`
- `markdown`
- `pdf`
- `docx`
- `image`
- `spreadsheet`
- `presentation`
- `unknown`
- `unsupported`

Classification does not imply project exposure.

Classification does not imply derivation.

Classification does not imply semantic eligibility.

## Identity

Identity rules:

- `file_ref` is content-based or a governed deterministic reference
- `object_ref` identifies a project-owned stored object candidate
- `intake_item_id` identifies an item in an intake batch
- source path must not be identity
- filename must not be identity
- duplicate content should reuse physical blob identity when policy allows
- usage refs declare why the file is used

Identity interpretation:

- two files are the same physical artifact when their canonical content hash matches after byte-level hashing
- two selected files are distinct intake events when they have different `intake_item_id` values, even if they point to the same physical artifact
- two stored object candidates may reference the same `file_ref` when the user selected duplicate content in different intake contexts
- `stored_object_id` or `stored_object_candidate_ref` identifies the logical stored object candidate, not the physical bytes
- `file_ref` identifies the content; it must not encode filename, source path, owner, batch, or project exposure
- `intake_item_id` identifies the selection event inside an `IntakeBatch`; it must not be reused as content identity
- `content_hash` is the hash evidence used to derive or verify `file_ref`

Duplicate handling:

- duplicate content in the same batch should create one `IntakeItem` per explicit user selection
- duplicate content should reuse the existing physical blob when hash and storage policy match
- duplicate content must not silently overwrite metadata, comments, owner refs, trace refs, or blocking reasons from another intake item
- duplicate content may produce a new logical `stored_object_candidate_ref` when the intake context, owner, comment, source label, or usage rationale differs
- exact duplicate selections may be marked as `duplicate_conflict` only when policy cannot safely disambiguate the user intent
- duplicate handling must preserve sanitized source labels so the user can understand which explicit selection produced each item
- reusing a blob does not imply project exposure, derivative readiness, registry generation, graph writes, or document-tree visibility

Reuse policy:

- prefer physical blob reuse when `content_hash` matches and the existing blob is governed, immutable, readable, and within the same storage authority
- create a new logical stored object candidate when metadata, owner, trace, comment, or intended usage differs
- create a new physical blob only when content differs, hash verification fails, or storage policy requires separation
- preserve one `IntakeItem` per explicit selection even when the same blob is reused
- never infer user intent from filename or source path alone

## Storage

Future storage must align with `docs/specs/storage_policy.md` and `docs/specs/project_folder_layout.md`.

Storage rules:

- physical storage must be governed
- blobs are canonical physical storage
- refs declare usage
- types classify/navigation only
- indexes are derivable accelerators
- no project-root `outputs/`
- no `assets/` as runtime
- no filesystem exposure by passive scanning
- `project_manifest.json` remains exposure authority

Comment storage rules:

- comments are stored as governed metadata only in a future durable intake step
- comments are not separate documents
- comments are not chat logs
- comments are not filesystem annotations
- comments are not blobs
- comments belong to `IntakeItem -> StoredObject metadata` in the future durable model

F12.4 does not define a final runtime storage layout.

## Mandatory metadata

Future durable intake metadata must include:

- `object_ref`
- `file_ref`
- `intake_batch_ref`
- `original_filename_sanitized`
- `detected_kind`
- `size_bytes`
- `content_hash`
- `created_at` or `imported_at` as policy allows
- `source_kind`
- `source_ref` sanitized
- `owner_ref`
- `trace_ref`
- `classification`
- `security_sanitization_state`
- `exposure_state`
- `derivation_state`

Optional per-file comment metadata may include:

- `user_comment`
- `comment_author_ref`
- `comment_created_at`
- `comment_sanitization_state`
- `comment_visibility`
- `trace_ref`

Comment metadata rules:

- `user_comment` is optional and empty by default
- `user_comment` is tied to one `IntakeItem`
- comments are descriptive metadata only
- comments are project-scoped
- comments may support human understanding, future search context, and trace explanation
- comments do not affect classification, capability, exposure authority, or derivation eligibility
- comments must be sanitized before durable persistence
- comments must not contain secrets, credentials, tokens, raw sensitive data, or private absolute host paths
- comments must not exceed the governed size limit
- rejected comments must block ingestion only when the active policy explicitly requires it

Metadata must not contain:

- secrets
- raw payloads
- private absolute host paths as portable truth
- credentials
- unsanitized user paths

## Chat comment proposals

`ChatCommentProposal` is a proposal-only chat artifact for intake-level annotations.

Chat may propose:

- adding a comment to an intake item
- editing a comment on an intake item

The governed flow is:

`user message -> ChatCommentProposal -> explicit promotion step -> FileIntakeDraft / IntakeItem update`

Rules:

- chat cannot directly persist comment metadata
- chat cannot mutate `IntakeItem`
- chat cannot mutate `FileIntakeDraft`
- chat output remains proposal-only
- promotion must be explicit and governed
- promotion must generate trace linkage
- promoted comments remain subject to sanitization and policy checks
- chat must not become a hidden comment store

## Exposure

Exposure states:

- `selected`
- `candidate`
- `planned`
- `imported_not_exposed`
- `exposed`
- `blocked`
- `rejected`
- `superseded`

Rules:

- `imported_not_exposed` is allowed
- `exposed` requires `project_manifest.json` governance
- filesystem presence alone does not expose
- document tree shows only exposed or prepared presentation state according to governed rules
- chat may reference intake items but must not become a hidden blob store

## Derivatives

Future derivation planning may include:

- text extraction
- page extraction
- chunking
- thumbnail or preview
- metadata index
- checksum index

Rules:

- derivatives are deterministic and regenerable
- derivatives are not source truth
- derivatives must not be generated in F12.4
- `document_text_runtime` may later consume exposed or eligible objects
- F12.4 does not implement derivation runtime

## Tool relationship

File intake is a governed pipeline, not merely a tool.

File intake may later use small operational/base tools.

`tool_runtime` must not become the intake orchestrator.

`project_runtime` must not be bypassed.

`app_services` remains thin.

UI remains intent/presentation only.

Future intake tools must be single-purpose, auditable, `owner_ref` aware, `trace_ref` aware, and not broad dispatchers.

Candidate future intake tools are declared only:

- `classify_file_format`
- `compute_file_hash`
- `copy_into_file_store`
- `create_stored_object_metadata`
- `plan_derivatives`
- `validate_intake_manifest`

These tools are not implemented or activated in F12.4.

## System View alignment

System View may present:

- selected files count
- intake batch status
- blocked reasons
- classification summary
- storage readiness
- exposure readiness
- derivation readiness
- `trace_ref`
- missing preconditions
- comment preview
- comment presence indicator
- comment sanitization warnings
- missing comment as optional information

System View must not:

- import
- copy
- classify by itself
- expose
- register
- derive
- execute tools
- authorize
- mutate storage
- edit comments
- persist comments
- generate comments
- infer comments

## Lume alignment

Lume may explain:

- why a file cannot be ingested
- what metadata is missing
- why a format is unsupported
- why source path is not portable truth
- why exposure requires manifest governance
- how intake comments are used
- how to improve a comment safely
- why comment content was flagged

Lume must not:

- import files
- authorize intake
- mutate manifests
- execute tools
- infer project exposure
- inspect raw payloads unless a later governed runtime provides sanitized summaries
- persist comments
- modify comment metadata
- bypass the explicit promotion flow

## Comment sanitization

Comment sanitization aligns with `docs/specs/security_sanitization_policy.md`.

Sanitization must remove or flag:

- secrets
- credentials
- tokens
- private absolute host paths
- raw sensitive data

Sanitization must enforce:

- maximum comment length
- safe encoding
- no executable content

`comment_sanitization_state` values:

- `safe`
- `flagged`
- `rejected`

## Blocking reason codes

- `missing_source`
- `unreadable_source`
- `source_is_directory`
- `unsupported_format`
- `unsafe_filename`
- `unsafe_path`
- `private_absolute_path`
- `missing_owner_ref`
- `missing_trace_ref`
- `missing_file_ref`
- `duplicate_conflict`
- `size_limit_exceeded`
- `sanitization_failed`
- `exposure_not_authorized`
- `derivation_not_available`
- `runtime_not_opened`
- `project_manifest_required`
- `policy_blocked`
- `comment_contains_secrets`
- `comment_contains_private_path`
- `comment_too_large`
- `comment_sanitization_failed`

Comment blocking reason codes are advisory or blocking according to the active policy. They must not block ingestion unless policy explicitly requires blocking.

## Audit preparation

Future advisory audit script:

- `dev/scripts/audits/audit_f12_file_intake_boundary.bat`

The future audit should check:

- no UI filesystem mutation
- no `app_services` intake orchestration
- no `project_runtime` bypass
- no raw host paths persisted as portable truth
- no passive filesystem exposure
- no `assets/` runtime use
- no broad `tool_runtime` orchestrator
- no derivation execution in the spec-only slice

F12.4 does not create this audit script.

## F12.5 implementation plan and audit checklist

Status: PLAN-ONLY / AUDIT-CHECKLIST-ONLY / NOT IMPLEMENTED.

`F12.5` prepares a future `F12.6` minimal governed file intake runtime.

`F12.5` does not implement runtime, copy files, hash files, classify files at runtime, persist metadata, mutate `project_manifest.json`, create registry entries, create graph entries, derive text, execute tools, modify runtime crates, call providers, use network, invoke external binaries, open LLM tool calling, or open semantic/RDF/Oxigraph runtime.

Runtime remains CLOSED after `F12.5`.

### Future crate boundaries

Preferred future `F12.6` ownership:

- `io_runtime`: source file validation, safe copy into governed storage, path normalization, and filesystem boundary checks
- `core_domain`: reusable identity, hash, or ref types only when already present or minimally extended
- `project_runtime`: must not be modified unless a later explicit audit approves project exposure
- `document_text_runtime`: no derivation in `F12.6`
- `tool_runtime`: no intake orchestration in `F12.6`
- `app_services`: no orchestration in `F12.6`; a later UI slice may add a thin adapter only if explicitly opened
- `ui_*`: no implementation in `F12.6`

### Future F12.6 allowed scope

Future `F12.6` may implement only:

- minimal governed file intake runtime
- explicit user-selected files only
- one intake batch per run
- governed `IntakeItem` candidates
- source file existence validation
- source file readability validation
- directory rejection
- safe extension or basic-kind classification only
- content hash only if existing `core_domain` or `io_runtime` support allows it without broad expansion
- copy into governed storage only when storage policy path is explicit
- sanitized metadata creation
- `owner_ref` preservation
- `trace_ref` preservation
- optional `user_comment` preservation after sanitization
- no private absolute host paths persisted as portable truth
- no automatic `project_manifest.json` exposure unless explicitly opened by `F12.6`
- no derivatives

### Future F12.6 forbidden scope

Future `F12.6` must not:

- scan host folders
- passively expose filesystem contents
- accept implicit files
- treat source path as identity
- write back to source folders
- mutate `project_manifest.json` unless separately opened
- generate `registry.json`
- write graph entries
- generate text, chunks, pages, thumbnails, previews, indexes, or derivatives
- use `tool_runtime` as intake orchestrator
- execute tools
- call providers
- use network
- invoke external binaries
- open LLM tool calling
- open semantic/RDF/Oxigraph runtime
- introduce UI or `app_services` authority

### Future accepted input shape

Future `F12.6` input must be explicit and governed:

- one `intake_batch_ref`
- one or more explicit selected source refs
- `owner_ref`
- `trace_ref`
- optional per-item `user_comment`
- policy references needed for size, extension, storage, and sanitization checks

Input must not depend on passive filesystem discovery.

### Future allowed outputs

Future `F12.6` may produce only:

- owner-scoped intake directory or governed file-store location according to existing storage policy
- metadata JSON for an intake item or stored object candidate
- optional batch manifest if explicitly declared before implementation

Future `F12.6` must not produce:

- `project_manifest.json` mutation unless separately opened
- `registry.json`
- graph files
- derivatives
- project-root outputs
- source-folder writes

### Future metadata shape

Future metadata must include:

- `intake_item_id`
- `intake_batch_ref`
- `object_ref` or `stored_object_candidate_ref`
- `file_ref` or `content_hash`
- `owner_ref`
- `trace_ref`
- `original_filename_sanitized`
- `detected_kind`
- `size_bytes`
- `source_kind`
- sanitized source display label only
- `classification`
- `security_sanitization_state`
- `exposure_state`
- `derivation_state`

Optional comment metadata:

- `text`
- `comment_author_ref`
- `comment_created_at`
- `comment_sanitization_state`
- `comment_visibility`

Comment rules:

- `user_comment` is optional
- comments are metadata, not authority
- comments must be sanitized
- comments must not contain secrets
- comments must not contain private absolute host paths
- comments must not affect classification, exposure, or derivation
- chat may propose comments only
- chat persistence is not opened in `F12.6`

### Future error model

Future `F12.6` must use typed error or blocked reason codes including:

- `missing_source`
- `unreadable_source`
- `source_is_directory`
- `unsupported_format`
- `unsafe_filename`
- `unsafe_path`
- `private_absolute_path`
- `missing_owner_ref`
- `missing_trace_ref`
- `duplicate_conflict`
- `size_limit_exceeded`
- `sanitization_failed`
- `comment_contains_secrets`
- `comment_contains_private_path`
- `comment_too_large`
- `comment_sanitization_failed`
- `storage_path_escape`
- `runtime_not_opened`
- `policy_blocked`
- `io_error`

### Required future F12.6 tests

Future `F12.6` must include tests proving:

- valid single file produces intake metadata
- valid batch produces one `IntakeItem` per file
- directory input is rejected
- missing source is rejected
- unreadable source is rejected when testable
- unsupported extension is blocked, not guessed
- unsafe filename is sanitized or rejected according to policy
- private absolute host path is not persisted as portable truth
- `owner_ref` is mandatory
- `trace_ref` is mandatory
- safe `user_comment` is preserved
- unsafe `user_comment` is flagged or rejected according to policy
- comment does not affect classification
- output path cannot escape governed storage root
- no `project_manifest.json` mutation occurs
- no `registry.json` generation occurs
- no graph write occurs
- no derivatives are generated
- no `tool_runtime` orchestration occurs
- no UI or `app_services` authority is introduced

### Required future audit

Future audit script:

- `dev/scripts/audits/audit_f12_file_intake_boundary.bat`

The audit is mandatory before closing `F12.6`.

The audit must check:

- no UI filesystem mutation
- no `app_services` intake orchestration
- no `project_runtime` bypass
- no `project_manifest.json` mutation unless explicitly opened
- no registry generation
- no graph writes
- no derivative generation
- no raw host absolute paths persisted
- no secrets in metadata
- no `assets/` runtime use
- no passive filesystem exposure
- no broad `tool_runtime` orchestrator
- no provider calls
- no network access
- no external binary invocation
- `owner_ref` required
- `trace_ref` required
- `user_comment` sanitized
- source folder remains readonly
- output or copy destination remains governed and owner/project scoped

Closure criteria for `F12.6`:

- implementation scope matches the allowed list
- forbidden scope remains absent
- required tests pass
- `audit_f12_file_intake_boundary.bat` returns acceptable closure status
- declarative validators pass
- Rust validation passes if crates are touched

## F12.5-F12.7 minimal governed file intake baseline closure

Closure state:

- `F12.5`: PLAN-ONLY COMPLETE
- `F12.6`: MINIMAL GOVERNED FILE INTAKE RUNTIME COMPLETE
- `F12.7`: READ-ONLY SYSTEM VIEW VISIBILITY COMPLETE

This baseline now includes:

- explicit user-selected intake batches only
- minimal governed intake runtime for text and markdown only
- mandatory `owner_ref`
- mandatory `trace_ref`
- sanitized per-item metadata
- optional sanitized per-item `user_comment`
- `imported_not_exposed` durable intake state
- blocked-item preservation with sanitized blocking reasons
- readonly `System View` visibility over prepared intake batches
- mandatory boundary audit coverage for the minimal intake slice

This closure does not open:

- project exposure through `project_manifest.json`
- registry generation
- graph writes
- derivatives
- `document_text_runtime` invocation
- `tool_runtime` orchestration
- UI execution authority
- LLM, provider, network, or external-binary execution

## Next allowed expansion candidates

Proposal-only. None of the following are opened by the minimal intake baseline:

1. project exposure gate, defined by `F13.0` as SPEC-only
2. derivatives gate
3. more file formats
4. intake history/index view, defined by `F12.9` as SPEC-only
5. storage dedup hardening, defined by `F12.8` as SPEC-only

## F12.8 storage dedup hardening / intake identity strategy

Status: SPEC-ONLY / HARDENING-STRATEGY-ONLY.

`F12.8` defines duplicate and identity semantics for the minimal governed file intake baseline.

It does not modify crates, copy files, hash files, mutate storage, expose projects, generate registries, write graph entries, create derivatives, invoke `document_text_runtime`, use `tool_runtime`, add UI authority, call providers, use network, invoke external binaries, or change `project_manifest.json`.

### Same artifact rule

Two files are the same physical artifact only when their governed byte-level content hash matches.

Filename equality, source path equality, extension equality, MIME hint equality, size equality, or user comment equality are not sufficient to prove sameness.

If the hash cannot be computed or verified, sameness must remain unknown and intake must not reuse physical storage by assumption.

### Identity relationship

- `content_hash`: hash evidence over file bytes
- `file_ref`: stable content reference derived from or verified by content identity
- `stored_object_id` or `stored_object_candidate_ref`: logical stored object candidate for a governed intake context
- `intake_item_id`: explicit selected-item event inside an intake batch

Relationship rules:

- one `content_hash` may map to one `file_ref`
- one `file_ref` may be referenced by multiple `stored_object_candidate_ref` values
- one `stored_object_candidate_ref` may be produced from one intake item context
- one `intake_item_id` belongs to exactly one `IntakeBatch`
- `intake_item_id` must not become `file_ref`
- `file_ref` must not become project exposure
- `stored_object_candidate_ref` must not imply `project_manifest.json` exposure

### Duplicate import policy

If a user imports duplicate content:

- keep every explicit selection as its own `IntakeItem`
- preserve the sanitized filename/source label for each item
- preserve item-level `owner_ref`, `trace_ref`, comment metadata, and blocking state
- reuse the existing physical blob when allowed by storage policy
- create a new logical stored object candidate when the intake context differs
- mark `duplicate_conflict` only when policy cannot safely represent or disambiguate the duplicate selection

Duplicate import must not:

- overwrite prior metadata
- merge comments
- infer exposure
- mutate `project_manifest.json`
- generate `registry.json`
- write graph entries
- generate derivatives
- call `document_text_runtime`
- call `tool_runtime`

### Future tests

Future implementation hardening should test:

- same bytes produce the same `content_hash`
- same bytes produce or resolve to the same `file_ref`
- different bytes produce different `content_hash` and `file_ref`
- duplicate selections in one batch produce distinct `intake_item_id` values
- duplicate selections reuse physical blob storage when allowed
- duplicate selections preserve distinct sanitized source labels
- duplicate selections preserve distinct user comments
- duplicate selections do not mutate `project_manifest.json`
- duplicate selections do not generate `registry.json`
- duplicate selections do not write graph entries
- duplicate selections do not generate derivatives
- hash unavailable blocks reuse rather than guessing
- `duplicate_conflict` is emitted only for ambiguous duplicate policy cases

### Future audit

Future intake boundary audit should check:

- duplicate content does not create duplicate physical blobs when reuse is allowed
- duplicate content does not collapse distinct `IntakeItem` records
- `file_ref` remains content-based and path-independent
- `stored_object_candidate_ref` remains logical and non-exposing
- `intake_item_id` remains batch-local selection identity
- no raw source path becomes portable identity
- no project exposure, registry, graph, derivative, `document_text_runtime`, or `tool_runtime` side effect is introduced by dedup hardening

## F12.9 intake history/index view

Status: SPEC-ONLY / HISTORY-INDEX-ONLY.

`F12.9` defines a readonly intake history/index view over governed intake metadata.

It does not modify crates, create a runtime index, expose projects, mutate `project_manifest.json`, generate `registry.json`, write graph entries, create derivatives, invoke `document_text_runtime`, use `tool_runtime`, add UI execution authority, call providers, use network, invoke external binaries, or change source storage.

The intake history/index is a derivable, rebuildable, non-authoritative presentation and lookup surface. It may help users understand prior intake batches, duplicate/reuse outcomes, blocked items, owner and trace linkage, and sanitized comments, but it must not become project exposure authority.

### Batch history model

A future intake history view may derive batch-level rows from already governed intake metadata.

Batch history fields may include:

- `intake_batch_ref`
- batch status summary
- item count
- imported item count
- blocked item count
- duplicate/reuse summary
- `owner_ref`
- `trace_ref`
- created or imported timestamp when already present in governed metadata
- sanitized batch comment preview when present
- sanitization summary

Batch history rules:

- batch history is readonly
- batch history is derivable from persisted governed intake metadata
- batch history may be rebuilt from intake records and stored object candidate metadata
- batch history must not be the only source of intake truth
- batch history must not infer project exposure
- missing history index entries must not delete or invalidate intake records

### Item history model

A future intake history view may derive item-level rows from `IntakeItem` metadata.

Item history fields may include:

- `intake_item_id`
- `intake_batch_ref`
- sanitized filename or sanitized source label
- item status: `candidate`, `blocked`, or `imported_not_exposed`
- `file_ref` when available
- `stored_object_candidate_ref` when available
- duplicate/reuse indicator
- blocking reason when present
- `owner_ref`
- `trace_ref`
- sanitization state
- sanitized `user_comment` preview when present

Item history rules:

- item history must preserve one row per explicit `IntakeItem`
- blocked items must remain visible with sanitized blocking reasons
- imported items remain `imported_not_exposed` unless a later project exposure gate explicitly opens exposure
- item history must not contain raw payloads, secrets, credentials, or private absolute host paths
- item history must not classify, reclassify, expose, derive, register, or execute

### Duplicate and reuse visibility

History/index presentation may show duplicate and reuse state as explanation only.

Allowed duplicate/reuse visibility:

- repeated `file_ref` across multiple `IntakeItem` records
- physical blob reuse indicator when governed metadata already records reuse
- distinct `stored_object_candidate_ref` values for distinct intake contexts
- `duplicate_conflict` blocking reason when policy could not disambiguate

Duplicate/reuse visibility must not:

- merge duplicate `IntakeItem` rows
- overwrite item comments or source labels
- infer that reused storage equals project exposure
- create or update dedup mappings
- scan storage to discover duplicates

### Blocked item visibility

Blocked items remain first-class history rows.

History/index presentation may show:

- sanitized blocking reason
- sanitized filename or source label
- owner and trace linkage
- sanitization state
- whether the item produced no stored object candidate

Blocked visibility must not authorize retry, import, exposure, derivation, or any corrective runtime action.

### Owner, trace, and comment visibility

History/index presentation may show `owner_ref` and `trace_ref` as governed references.

Rules:

- `owner_ref` display is not credential authority
- `trace_ref` display is not execution authority
- sanitized comments may be previewed only as metadata
- comments remain non-authoritative for classification, exposure, derivation, and duplicate resolution
- unsafe or rejected comments must be summarized by sanitized warning state only

### Future tests

Future implementation of an intake history/index view should test:

- batch history derives from existing intake metadata
- item history preserves one row per `IntakeItem`
- blocked items remain visible with sanitized blocking reasons
- duplicate selections remain distinct rows
- duplicate/reuse indicators do not merge item identity
- repeated `file_ref` can be displayed without implying project exposure
- `owner_ref` and `trace_ref` are visible as refs only
- sanitized comment previews are visible when safe
- raw source paths are not persisted or displayed as portable truth
- history can be rebuilt after index deletion
- deleting or rebuilding the index does not mutate intake records
- no `project_manifest.json`, `registry.json`, graph, derivative, `document_text_runtime`, `tool_runtime`, provider, network, LLM, or external-binary side effect occurs

### Future audit additions

Future intake boundary audit should also check:

- history/index files, if later implemented, are derivable and rebuildable
- history/index files are non-authoritative
- history/index loss does not imply data loss for intake records
- history/index presence does not imply project exposure
- duplicate/reuse visibility does not collapse distinct `IntakeItem` records
- blocked items remain visible without triggering retries
- sanitized comments are previews only and do not become classification or exposure authority
- no history/index code writes `project_manifest.json`, `registry.json`, graph files, derivatives, or source files
- no history/index code calls `tool_runtime`, `document_text_runtime`, providers, network, LLMs, or external binaries

## F13.0 Project Exposure Gate

Status: SPEC-ONLY / GATE-ONLY / NOT IMPLEMENTED.

`F13.0` defines the governed project exposure gate for promoting an already imported, supported, sanitized `IntakeItem` or `StoredObject` candidate from `imported_not_exposed` into project visibility.

It does not implement runtime, modify crates, create a manifest writer, create a registry generator, write graph entries, create derivatives, call `document_text_runtime`, call `tool_runtime`, add UI authority, expose blocked items, expose unsupported items, infer exposure from `file_store` presence, or scan the filesystem.

The canonical exposure chain is:

`ExposureRequest -> ExposureCandidate -> HumanConfirmation -> future project_manifest update -> exposed_to_project`

Only a later explicit implementation slice may perform the future `project_manifest.json` mutation.

### Exposure request

`ExposureRequest` is a governed request to make an already imported intake object visible to a project.

It must reference:

- `exposure_request_id`
- `intake_item_ref`
- `stored_object_candidate_ref` or `object_ref`
- `file_ref`
- `owner_ref`
- `trace_ref`
- requested project ref
- requested document tree placement or presentation hint
- sanitized display label
- requested exposure rationale

Rules:

- request input must be refs and sanitized metadata only
- request input must not contain raw payloads, private absolute host paths, or secrets
- the request does not mutate `project_manifest.json`
- the request does not expose the item
- the request does not create registry, graph, derivative, or document tree state

### Exposure candidate

`ExposureCandidate` is an inert evaluation artifact prepared from an `ExposureRequest`.

It may summarize:

- whether the intake item is `imported_not_exposed`
- whether the item is supported
- whether the item is not blocked
- whether required refs are present
- whether metadata is sanitized
- whether `owner_ref` and `trace_ref` are preserved
- whether duplicate exposure policy is satisfied
- whether `project_manifest.json` governance is required

Candidate states:

- `candidate`
- `blocked`
- `needs_confirmation`
- `superseded`
- `stale`

Blocked reasons may include:

- `item_not_imported`
- `item_already_exposed`
- `blocked_item`
- `unsupported_item`
- `missing_owner_ref`
- `missing_trace_ref`
- `missing_file_ref`
- `missing_stored_object_ref`
- `unsanitized_metadata`
- `duplicate_exposure_conflict`
- `project_manifest_required`
- `policy_blocked`
- `runtime_not_opened`

`ExposureCandidate` must not perform policy resolution, write manifests, generate registries, write graphs, derive text, execute tools, or expose items.

### Human confirmation requirement

Project exposure requires explicit human confirmation before any future `project_manifest.json` mutation.

Confirmation must preserve:

- `exposure_request_id`
- `exposure_candidate_id`
- `intake_item_ref`
- `stored_object_candidate_ref` or `object_ref`
- `owner_ref`
- `trace_ref`
- decision
- reviewer or confirmer ref
- timestamp when policy allows it

Accepted confirmation means the user approved the exposure proposal. It does not by itself mutate `project_manifest.json` and does not create document tree visibility until a later governed implementation writes the manifest.

Rejected, deferred, stale, blocked, or changes-requested confirmation must keep the item `imported_not_exposed`.

### Exposure transition

The only governed transition opened by this spec as a future possibility is:

`imported_not_exposed -> exposed_to_project`

Transition rules:

- source state must be `imported_not_exposed`
- blocked items must not transition
- unsupported items must not transition
- missing or unsafe metadata must block transition
- `owner_ref` must be preserved
- `trace_ref` must be preserved
- `file_ref` must remain content identity and must not become exposure identity
- `stored_object_candidate_ref` or `object_ref` must remain logical object identity
- project visibility starts only after a governed `project_manifest.json` declaration exists

`exposed_to_project` is the canonical exposure state for post-gate project visibility. Legacy `exposed` wording remains conceptual and must not be used to bypass this gate.

### Project manifest authority

`project_manifest.json` is the only project exposure authority.

Rules:

- filesystem presence is not exposure
- `file_store` blob presence is not exposure
- `StoredObject` metadata is not exposure
- registry presence is not exposure
- graph presence is not exposure
- document tree presentation is not exposure authority
- chat references are not exposure authority
- UI selection is not exposure authority

No project exposure may be inferred from scanning folders, reading `file_store`, seeing intake metadata, or viewing history/index rows.

### Duplicate exposure policy

Duplicate content may be exposed more than once only when the project manifest records distinct governed project entries or usages.

Rules:

- same `file_ref` may appear in multiple exposure candidates
- duplicate content must not collapse distinct `IntakeItem` or logical object identity
- duplicate exposure must preserve distinct display labels, comments, owners, traces, and usage rationale when present
- if the same logical object is already exposed in the same project context, a new exposure candidate must become `item_already_exposed` or `duplicate_exposure_conflict`
- exposing one duplicate does not expose all duplicates sharing the same `file_ref`
- physical blob reuse does not imply project exposure reuse

### Document tree visibility

After a future governed implementation writes the project manifest, `project_runtime` may prepare document tree state from that manifest.

Rules:

- document tree visibility follows `project_manifest.json`
- the document tree must not scan `file_store`
- the document tree must not scan intake metadata
- the document tree must not expose blocked or unsupported items
- document tree nodes must use sanitized display labels
- document tree nodes must remain readonly unless a later editing phase opens mutation

### Registry and graph relationship

`registry.json` remains derivable and non-authoritative.

Rules:

- registry may accelerate navigation only after manifest exposure exists
- registry must be rebuildable from authoritative sources
- registry absence must not remove manifest exposure
- registry presence must not create manifest exposure

Graph entries remain optional and future-only.

Rules:

- graph writes are not opened by `F13.0`
- graph presence must not create exposure
- graph absence must not block manifest-governed exposure unless a later policy explicitly requires it

### Rollback and revocation

Rollback and revocation are future-only.

Future revocation must be governed by a separate request, candidate, confirmation, and manifest update flow.

F13.0 does not define runtime revocation, delete blobs, delete intake metadata, rewrite history, remove registry entries, remove graph entries, or generate tombstones.

### Future tests

Future implementation must test:

- only `imported_not_exposed` supported items can become exposure candidates
- blocked items cannot become exposure candidates
- unsupported items cannot become exposure candidates
- missing `owner_ref` blocks exposure
- missing `trace_ref` blocks exposure
- missing `file_ref` blocks exposure
- accepted human confirmation is required before future manifest mutation
- accepted confirmation alone does not expose without manifest update
- future manifest update preserves `owner_ref` and `trace_ref`
- filesystem presence does not expose items
- `file_store` presence does not expose items
- duplicate `file_ref` exposure preserves distinct `IntakeItem` and object identity
- exposing one duplicate does not expose all duplicates
- document tree shows only manifest-exposed objects
- registry remains derivable and non-authoritative
- no graph write, derivative generation, `document_text_runtime`, `tool_runtime`, provider, network, LLM, or external-binary side effect occurs

### Future audit requirements

Future exposure-gate audit must check:

- no filesystem scanning creates exposure
- no automatic exposure from intake runtime
- no exposure of blocked items
- no exposure of unsupported items
- `project_manifest.json` is the only exposure authority
- registry generation is not required and remains non-authoritative
- graph writes are absent unless a later explicit graph slice opens them
- document tree visibility is manifest-driven only
- duplicate exposure policy preserves refs and does not collapse items
- `owner_ref` and `trace_ref` are preserved across request, candidate, confirmation, and future exposure state
- rollback or revocation code is absent unless separately opened
- no `document_text_runtime`, `tool_runtime`, provider, network, LLM, or external-binary side effect is introduced

## F13.1 Legacy Exposure/Import Bypass Hardening

Status: SPEC-ONLY / AUDIT-PLAN / NOT IMPLEMENTED.

`F13.1` hardens the boundary around `F12` intake and `F13.0` exposure by classifying legacy import, tree-scan, chat-resource, and direct-derivation flows that must not become an implicit project exposure pipeline.

The canonical governed exposure path is:

`explicit user-selected file -> F12 intake -> imported_not_exposed -> F13 exposure request -> F13 exposure candidate -> human confirmation -> future project_manifest update -> document tree visibility`

No legacy helper, direct UI path, chat-resource registration, derivative call, or passive filesystem scan may replace this chain.

### Legacy flow classification

The following legacy flows are classified for boundary hardening:

- `import_project_document`: deprecated for production project intake or project exposure; must be blocked before any `F13` runtime implementation; may remain only for fixtures, migration tests, or explicitly non-authoritative legacy tests until replaced
- `list_project_documents`: deprecated as a source of project visibility; must be blocked before any `F13` runtime implementation when used for document tree authority; may remain only for legacy fixture tests until tree state is manifest-driven
- `register_chat_resource`: legacy tolerated only for chat-local reference staging that does not create project exposure, document tree visibility, storage authority, or derivation authority; it must not be promoted into intake or exposure authority
- `derive_document_text`: allowed only as a separate derivative runtime for already governed eligible objects in a later explicit phase; deprecated as a direct UI-triggered import or exposure side effect and must be blocked before `F13` runtime
- any UI or Slint direct file import, direct project placement, direct exposure, direct chat-to-project promotion, direct tree refresh from filesystem, or direct derivation path: deprecated and must be blocked before `F13` runtime

### Forbidden bypasses

The following are forbidden as project exposure bypasses:

- importing a file into project folders and treating that copy as exposure
- scanning project folders, `file_store`, or intake metadata to infer project visibility
- registering a chat resource and treating the registration as intake, exposure, or project authorization
- deriving text directly from external files or newly copied files as part of intake or exposure
- populating document tree visibility from filesystem presence, copied files, registry rows, graph rows, or chat references
- allowing `app_services` to shadow exposure request, candidate, confirmation, or manifest authority
- allowing UI to import, expose, register, derive, or confirm directly

### Migration notes for legacy functions

Future migration must preserve narrow responsibilities:

- `import_project_document` must migrate behind `F12` intake plus `F13` exposure, or remain fixture-only
- `list_project_documents` must migrate to prepared tree state derived from manifest-governed exposure, or remain fixture-only
- `register_chat_resource` may remain chat-local only if clearly non-authoritative and non-promoting
- `derive_document_text` must remain separate from intake and exposure and may later consume only already governed eligible objects

### STOP conditions before F13 runtime

No `F13` runtime implementation may begin until all of the following are true:

- no production UI path directly calls legacy project import as an exposure substitute
- no production UI path directly refreshes document tree authority from filesystem scanning
- no production UI path directly calls `derive_document_text` as part of import or exposure
- no production UI path promotes chat resources into project visibility without `F12` intake plus `F13` exposure
- `app_services` does not shadow exposure request, candidate, confirmation, or manifest authority
- future `audit_f13_exposure_boundary.bat` exists and passes

### Future required tests

Future implementation and migration must test:

- legacy import does not create exposure
- copied files do not become exposed without manifest update
- chat resource registration does not create intake or exposure
- filesystem scanning does not create document tree exposure
- direct derivation does not run during intake or exposure
- blocked or unsupported items cannot bypass the exposure gate through legacy paths
- duplicate content cannot bypass request, candidate, confirmation, and manifest authority

### Future required audit

Future `audit_f13_exposure_boundary.bat` must check:

- no UI direct import-to-project path remains authoritative
- no UI direct derivation path remains authoritative
- no document tree authority is sourced from `list_project_documents`
- no chat-resource path becomes intake or exposure authority
- no `app_services` layer shadows exposure governance
- no `project_manifest.json` exposure is inferred from copied files, registry rows, graph rows, history rows, or filesystem presence
- no derivative generation is triggered by exposure
- no `tool_runtime`, provider, network, LLM, or external-binary side effect is introduced

## F13.4 Manifest Exposure Contract Hardening

Status: SPEC-ONLY / CONTRACT-HARDENING / NOT IMPLEMENTED.

`F13.4` closes the exact non-executing contract for promoting an already governed `imported_not_exposed` intake artifact into manifest-governed project visibility.

The canonical future lifecycle is:

`selected external file -> F12 intake -> imported_not_exposed -> ExposureRequest -> ExposureCandidate -> HumanConfirmation -> future ManifestExposureEntry -> exposed_to_project`

`F13.4` does not modify crates, implement runtime, create a `project_manifest.json` writer, generate `registry.json`, write graph entries, create derivatives, call `document_text_runtime`, call `tool_runtime`, or add UI or `app_services` authority.

### Canonical terminology

`exposed_to_project` is the canonical project exposure state.

Legacy `exposed` wording is deprecated shorthand only and must either be avoided or explicitly mapped to `exposed_to_project`.

The following never imply project exposure:

- filesystem presence
- `file_store` blob presence
- stored-object metadata presence
- `registry.json` presence
- graph presence
- `System View` visibility
- intake history/index visibility
- document tree visibility before manifest authority exists

### ExposureRequest contract

`ExposureRequest` is the formal non-executing request to make a governed imported artifact eligible for future manifest exposure.

Required fields:

- `exposure_request_id`
- `intake_item_ref` or `stored_object_candidate_ref`
- `requested_by_ref`
- `owner_ref`
- `trace_ref`
- `requested_at`
- `requested_exposure_kind`
- optional sanitized user intent or comment
- expected manifest target kind
- duplicate handling preference when policy allows it
- `request_status`

Rules:

- `ExposureRequest` is non-executing and does not write `project_manifest.json`
- request inputs must be portable refs and sanitized metadata only
- request inputs must not contain raw payloads, secrets, or private absolute host paths
- the request does not create `registry.json`, graph entries, derivatives, document tree entries, or viewer entries
- the request does not authorize exposure and does not imply confirmation

### ExposureCandidate contract

`ExposureCandidate` is the inert evaluation artifact prepared from an `ExposureRequest`.

Required fields:

- `exposure_candidate_id`
- `exposure_request_ref`
- `candidate_status`
- `eligibility_result`
- source intake item refs
- `file_ref` or `content_hash` evidence
- `stored_object_candidate_ref`
- sanitized display label
- blocking reasons
- duplicate assessment
- `owner_ref`
- `trace_ref`

Eligibility rules:

- only `imported_not_exposed` items may become candidates
- blocked items must never become exposure candidates
- unsupported items must never become exposure candidates
- missing or unreadable stored objects must block
- missing `owner_ref` or `trace_ref` must block
- unsafe metadata must block or require sanitized representation before any future runtime exposure
- candidate evaluation is descriptive only and must not resolve policy by side effect

### HumanConfirmation contract

`HumanConfirmation` is the explicit non-executing reviewer decision event required before any future manifest exposure.

Required fields:

- `confirmation_id`
- `exposure_candidate_ref`
- `reviewer_ref`
- decision: `accepted | rejected`
- `confirmed_at`
- `trace_ref`
- `stale_check_result`
- `risk_acknowledgement` when policy requires it

Rules:

- accepted confirmation is required before any future manifest exposure
- confirmation is not execution and does not mutate `project_manifest.json`
- rejected confirmation must not mutate any authority
- stale or unknown candidate state must block future runtime exposure
- confirmation must preserve `owner_ref`, `trace_ref`, and candidate linkage by ref

### ManifestExposureEntry contract

`ManifestExposureEntry` is the minimal future manifest entry shape that a later explicit runtime slice may write to `project_manifest.json`.

Required future fields:

- `manifest_entry_id`
- `exposed_object_ref`
- `file_ref`
- `stored_object_ref` or `stored_object_candidate_ref`, depending on canonical naming active at implementation time
- `source_intake_item_ref`
- `exposure_request_ref`
- `exposure_candidate_ref`
- `confirmation_ref`
- `owner_ref`
- `trace_ref`
- `exposed_to_project_at`
- `artifact_kind`
- `display_label`
- `content_kind` or `basic_kind`
- sanitized metadata refs only
- `exposure_state = exposed_to_project`

Rules:

- `project_manifest.json` is the only project exposure authority
- the manifest entry must use portable refs and sanitized labels only
- the manifest entry must not contain raw payloads, secrets, or private absolute host paths
- the manifest entry must not imply derivative generation
- the manifest entry must not imply graph writes or registry updates
- the manifest entry is a future runtime output and is not created by `F13.4`

### Duplicate exposure policy hardening

Duplicate content remains governed and explicit.

Rules:

- the same `content_hash` or `file_ref` may reuse one physical blob
- physical blob reuse must not collapse `intake_item_id`, `owner_ref`, `trace_ref`, comments, request identity, candidate identity, or confirmation identity
- the same physical file may produce distinct logical manifest entries only when explicitly confirmed and policy allows it
- duplicate exposure must be visible and traceable and must never be silent
- future runtime must either block or warn on duplicate exposure according to explicit policy, never by hidden convenience

### Failure model

Future runtime must report structured safe failures using typed categories such as:

- `source_not_imported`
- `blocked_intake_item`
- `unsupported_kind`
- `missing_owner_ref`
- `missing_trace_ref`
- `stale_candidate`
- `duplicate_policy_block`
- `unsafe_metadata`
- `missing_stored_object`
- `manifest_conflict`
- `unauthorized_transition`

Failure reporting must be safe for `System View` presentation and must not expose secrets, raw payloads, or private absolute host paths.

### System View alignment

`System View` may later present readonly summaries of:

- exposure requests
- exposure candidates
- blocking reasons
- confirmation state
- future manifest exposure status
- duplicate warnings
- `owner_ref`
- `trace_ref`

`System View` must not:

- confirm
- expose
- mutate
- write `project_manifest.json`
- resolve policy
- trigger runtime

### Document Tree alignment

The `Document Tree` may only show artifacts that are exposed through `project_manifest.json`.

The `Document Tree` must not:

- scan the filesystem for exposure authority
- consume intake history/index as authority
- expose items itself
- treat blob presence, registry rows, graph rows, or `System View` presence as project membership

### Future audit hardening

Future exposure audit requirements extend to:

- no `project_manifest.json` writes outside an explicit future exposure runtime slice
- no `list_project_documents` authority
- no filesystem scanning as exposure
- no registry or graph writes from exposure
- no derivatives from exposure
- no UI exposure authority
- no `tool_runtime` orchestration
- no `document_text_runtime` trigger during exposure
- no raw host paths in `ManifestExposureEntry`

### Closure note

`F13.4` is SPEC-only.

It changes no crates, introduces no runtime, adds no manifest writer, implements no project exposure, and only closes the contract required for a later `F13.5` audit or runtime-preparation slice.

## F13.5A Manifest Exposure Runtime Plan

Status: SPEC-ONLY / AUDIT-PLAN-ONLY / NOT IMPLEMENTED.

`F13.5A` converts the `F13.4` manifest exposure contract into the exact implementation checklist and mandatory audit plan for a later minimal runtime slice.

`F13.5A` does not implement runtime, modify crates, write `project_manifest.json`, generate `registry.json`, write graph entries, create derivatives, call `document_text_runtime`, call `tool_runtime`, add UI authority, add `app_services` authority, or change exposure state.

### Future runtime owner

Future minimal manifest exposure runtime ownership is:

- `project_runtime`: canonical owner of future manifest exposure runtime because `project_manifest.json` is project authority
- `io_runtime`: low-level safe file IO only if a later implementation needs it, without becoming exposure authority
- `app_services`: thin prepared call surface only in a later explicit slice
- UI: request intent and readonly prepared state only

### Allowed future F13.5B runtime scope

Future `F13.5B` may implement only:

- promotion of one already `imported_not_exposed` item
- required `ExposureRequest`
- required `ExposureCandidate`
- required accepted `HumanConfirmation`
- required non-stale candidate
- required `owner_ref`
- required `trace_ref`
- write of one `ManifestExposureEntry` to `project_manifest.json`
- preservation of portable refs only
- structured success or failure result

### Forbidden future F13.5B runtime scope

Future `F13.5B` must not:

- expose blocked items
- expose unsupported items
- scan the filesystem for exposure authority
- generate `registry.json`
- write graph entries
- create derivatives
- call `document_text_runtime`
- call `tool_runtime`
- invoke LLMs, providers, network access, or external binaries
- store raw host absolute paths
- collapse duplicate logical identity by blob reuse or convenience
- allow UI or `app_services` to become exposure authority

### Future input contract

Future runtime input must require:

- one `ExposureRequest`
- one `ExposureCandidate`
- one accepted `HumanConfirmation`
- one target `imported_not_exposed` intake or stored-object candidate ref
- `owner_ref`
- `trace_ref`
- duplicate handling policy input when required by policy
- stale-check result proving the candidate remains valid at runtime entry

Rules:

- input artifacts must be portable refs and sanitized metadata only
- input artifacts must not contain raw payloads, secrets, or private absolute host paths
- runtime must reject any request chain that bypasses request, candidate, or accepted confirmation

### Future output contract

Future runtime output may produce only:

- one governed `ManifestExposureEntry` written to `project_manifest.json`
- one structured runtime result describing success or typed safe failure

Rules:

- output must preserve portable refs only
- output must preserve `owner_ref` and `trace_ref`
- output must preserve duplicate-safe logical identity
- output must not imply registry generation, graph writes, derivatives, or tree mutation by convenience

### Future manifest write policy

When `F13.5B` opens later:

- `project_runtime` is the only approved manifest writer boundary
- the manifest write must append or record exactly one governed `ManifestExposureEntry`
- the manifest write must preserve prior manifest authority and must not rewrite exposure history ad hoc
- the manifest write must persist sanitized portable refs only
- the manifest write must not persist raw payloads, secrets, or private absolute host paths

### Duplicate policy enforcement

Future runtime must enforce:

- physical blob reuse is allowed
- logical exposure identity must remain distinct unless explicit policy says the same logical entry is being updated
- duplicate content must never silently collapse `intake_item_id`, request identity, candidate identity, confirmation identity, `owner_ref`, `trace_ref`, comments, or usage rationale
- duplicate blocking or warning behavior must be structured and visible

### Stale candidate enforcement

Future runtime must require a non-stale candidate at execution entry.

Rules:

- rejected confirmation blocks
- stale candidate blocks
- unknown stale-check result blocks
- candidate drift between confirmation and manifest write must block and return a typed safe failure

### Required typed failures

Future `F13.5B` must return structured safe failures including at minimum:

- `source_not_imported`
- `blocked_intake_item`
- `unsupported_kind`
- `missing_owner_ref`
- `missing_trace_ref`
- `rejected_confirmation`
- `stale_candidate`
- `duplicate_policy_block`
- `unsafe_metadata`
- `missing_stored_object`
- `manifest_conflict`
- `unauthorized_transition`

### Required future tests

Future `F13.5B` must test:

- valid exposure from an `imported_not_exposed` item
- blocked item cannot expose
- unsupported item cannot expose
- missing `owner_ref` blocks
- missing `trace_ref` blocks
- rejected confirmation blocks
- stale candidate blocks
- duplicate hash does not collapse logical exposure identity
- manifest entry uses portable refs only
- no raw host paths are persisted
- no registry writes occur
- no graph writes occur
- no derivatives occur
- no `document_text_runtime` call occurs
- no `tool_runtime` orchestration occurs
- no UI or `app_services` authority is introduced

### Mandatory future audit

Future `F13.5B` closure requires `dev/scripts/audits/audit_f13_manifest_exposure_runtime_boundary.bat`.

The audit must check:

- manifest writer exists only in approved `project_runtime` boundary when runtime opens
- no `project_manifest.json` writes from `ui_slint`, `app_services`, `io_runtime`, `tool_runtime`, or `document_text_runtime`
- no `list_project_documents` or filesystem scanning authority
- no registry, graph, or derivative writes from exposure
- no `document_text_runtime` trigger from exposure
- no `tool_runtime` orchestration
- no raw absolute host path patterns in manifest exposure outputs
- no bypass of `ExposureRequest -> ExposureCandidate -> accepted HumanConfirmation`

### Closure criteria before F13.5B runtime

Before `F13.5B` may be declared ready to implement:

- `F13.4` contract remains canonical and unambiguous
- `audit_f13_exposure_boundary.bat` continues to pass with no FAIL findings
- future `audit_f13_manifest_exposure_runtime_boundary.bat` is registered as required
- future runtime ownership remains limited to `project_runtime`
- no new bypass path is opened in UI, `app_services`, `io_runtime`, `tool_runtime`, or `document_text_runtime`

## Invariants

- `INV-FILE-INTAKE-001`: file intake is governed, not a raw copy operation.
- `INV-FILE-INTAKE-002`: UI captures selection intent only.
- `INV-FILE-INTAKE-003`: source paths are not portable identity.
- `INV-FILE-INTAKE-004`: original host files remain readonly.
- `INV-FILE-INTAKE-005`: filesystem presence does not imply project exposure.
- `INV-FILE-INTAKE-006`: `project_manifest.json` remains exposure authority.
- `INV-FILE-INTAKE-007`: every intake item requires `owner_ref` and `trace_ref` before durable import.
- `INV-FILE-INTAKE-008`: metadata must be sanitized and must not contain secrets or raw payloads.
- `INV-FILE-INTAKE-009`: derivatives are regenerable and not source truth.
- `INV-FILE-INTAKE-010`: intake may use tools later, but `tool_runtime` must not become a general intake orchestrator.
- `INV-FILE-INTAKE-011`: `app_services` and UI must remain thin.
- `INV-FILE-INTAKE-012`: F12.4 creates no runtime, files, manifests, registry entries, graph entries, or derivatives.
- `INV-FILE-INTAKE-COMMENT-001`: comments are metadata, not authority.
- `INV-FILE-INTAKE-COMMENT-002`: UI captures comments but does not persist.
- `INV-FILE-INTAKE-COMMENT-003`: chat may propose but not persist comments.
- `INV-FILE-INTAKE-COMMENT-004`: comments must be sanitized.
- `INV-FILE-INTAKE-COMMENT-005`: comments must not contain secrets or private paths.
- `INV-FILE-INTAKE-COMMENT-006`: comments do not affect classification, exposure, or derivation.
- `INV-FILE-INTAKE-COMMENT-007`: comments are project-scoped metadata.
- `INV-FILE-INTAKE-PLAN-001`: F12.5 is plan-only.
- `INV-FILE-INTAKE-PLAN-002`: F12.5 introduces no runtime.
- `INV-FILE-INTAKE-PLAN-003`: F12.6 must not implement project exposure unless separately opened.
- `INV-FILE-INTAKE-PLAN-004`: F12.6 must not generate derivatives.
- `INV-FILE-INTAKE-PLAN-005`: F12.6 must not use `tool_runtime` as intake orchestrator.
- `INV-FILE-INTAKE-PLAN-006`: F12.6 must preserve `owner_ref` and `trace_ref`.
- `INV-FILE-INTAKE-PLAN-007`: source paths are transient and not portable truth.
- `INV-FILE-INTAKE-PLAN-008`: `user_comment` remains sanitized metadata and non-authoritative.
- `INV-FILE-INTAKE-PLAN-009`: future `audit_f12_file_intake_boundary` is mandatory before closure.
- `INV-FILE-INTAKE-PLAN-010`: `project_runtime`, `app_services`, and UI authority remain unchanged.
- `INV-FILE-INTAKE-BASELINE-001`: `F12.5`, `F12.6`, and `F12.7` together define the minimal governed file intake baseline.
- `INV-FILE-INTAKE-BASELINE-002`: the minimal intake baseline remains `imported_not_exposed` only and does not open project exposure.
- `INV-FILE-INTAKE-BASELINE-003`: the minimal intake baseline does not generate registries, graph writes, or derivatives.
- `INV-FILE-INTAKE-BASELINE-004`: the minimal intake baseline does not invoke `document_text_runtime` or use `tool_runtime` as an intake orchestrator.
- `INV-FILE-INTAKE-BASELINE-005`: `System View` visibility remains readonly and consumes prepared intake state only.
- `INV-FILE-INTAKE-BASELINE-006`: UI, providers, network access, LLM execution, and external binaries remain outside the minimal intake baseline.
- `INV-FILE-INTAKE-DEDUP-001`: physical artifact sameness is determined by governed content hash, not filename or source path.
- `INV-FILE-INTAKE-DEDUP-002`: duplicate user selections remain distinct `IntakeItem` records.
- `INV-FILE-INTAKE-DEDUP-003`: duplicate content may reuse one physical blob without merging logical stored object candidates.
- `INV-FILE-INTAKE-DEDUP-004`: `file_ref` identifies content and must not imply project exposure.
- `INV-FILE-INTAKE-DEDUP-005`: `stored_object_candidate_ref` identifies logical intake context and must not imply project exposure.
- `INV-FILE-INTAKE-DEDUP-006`: dedup hardening must not mutate `project_manifest.json`, generate registries, write graph entries, or generate derivatives.
- `INV-FILE-INTAKE-DEDUP-007`: dedup hardening must not invoke `document_text_runtime`, `tool_runtime`, providers, network access, LLM execution, or external binaries.
- `INV-FILE-INTAKE-HISTORY-001`: `F12.9` intake history/index is SPEC-only until separately implemented.
- `INV-FILE-INTAKE-HISTORY-002`: intake history/index is readonly, derivable, rebuildable, and non-authoritative.
- `INV-FILE-INTAKE-HISTORY-003`: intake history/index must not imply project exposure or mutate `project_manifest.json`.
- `INV-FILE-INTAKE-HISTORY-004`: intake history/index must preserve one visible row per governed `IntakeItem`.
- `INV-FILE-INTAKE-HISTORY-005`: blocked items must remain visible with sanitized blocking reasons.
- `INV-FILE-INTAKE-HISTORY-006`: duplicate/reuse visibility must not collapse item identity or create dedup mappings.
- `INV-FILE-INTAKE-HISTORY-007`: history/index comment previews are sanitized metadata only and non-authoritative.
- `INV-FILE-INTAKE-HISTORY-008`: history/index must not generate `registry.json`, graph writes, derivatives, or call `document_text_runtime` or `tool_runtime`.
- `INV-PROJECT-EXPOSURE-001`: `F13.0` Project Exposure Gate is SPEC-only until separately implemented.
- `INV-PROJECT-EXPOSURE-002`: project exposure requires explicit governed request, candidate, and human confirmation before any future manifest mutation.
- `INV-PROJECT-EXPOSURE-003`: `project_manifest.json` is the only project exposure authority.
- `INV-PROJECT-EXPOSURE-004`: filesystem presence, `file_store` presence, registry presence, graph presence, history/index presence, UI selection, and chat references must not imply exposure.
- `INV-PROJECT-EXPOSURE-005`: only supported `imported_not_exposed` items may be candidates for future exposure.
- `INV-PROJECT-EXPOSURE-006`: blocked and unsupported items must not be exposed.
- `INV-PROJECT-EXPOSURE-007`: `owner_ref`, `trace_ref`, `file_ref`, and logical object refs must be preserved through the exposure chain.
- `INV-PROJECT-EXPOSURE-008`: duplicate exposure must not collapse distinct intake items or logical object identity.
- `INV-PROJECT-EXPOSURE-009`: document tree visibility after exposure is manifest-driven and presentation-only.
- `INV-PROJECT-EXPOSURE-010`: registry remains derivable and non-authoritative; graph remains optional and future-only.
- `INV-PROJECT-EXPOSURE-011`: rollback and revocation are future-only and not opened by `F13.0`.
- `INV-PROJECT-EXPOSURE-012`: `F13.0` must not create runtime, writers, registries, graph writes, derivatives, `document_text_runtime` calls, `tool_runtime` calls, providers, network, LLM execution, external binaries, or UI authority.
- `INV-PROJECT-EXPOSURE-013`: `exposed_to_project` is the canonical project exposure state and legacy `exposed` wording is shorthand only.
- `INV-PROJECT-EXPOSURE-014`: `imported_not_exposed` is not project membership and must not appear as project exposure without manifest authority.
- `INV-PROJECT-EXPOSURE-015`: manifest-governed exposure requires `ExposureRequest`, `ExposureCandidate`, and accepted `HumanConfirmation` before any future runtime manifest write.
- `INV-PROJECT-EXPOSURE-016`: accepted confirmation is required but is not itself execution or manifest mutation.
- `INV-PROJECT-EXPOSURE-017`: `ManifestExposureEntry` is a future runtime output only and is not created by `F13.4`.
- `INV-PROJECT-EXPOSURE-018`: duplicate physical blob reuse must not collapse logical exposure identity, request identity, candidate identity, confirmation identity, comments, `owner_ref`, or `trace_ref`.
- `INV-PROJECT-EXPOSURE-019`: `registry.json`, graph state, derivatives, document tree presence, and `System View` presence remain non-authoritative for project exposure.
- `INV-PROJECT-EXPOSURE-020`: UI, `System View`, and `Document Tree` must not authorize exposure.
- `INV-PROJECT-EXPOSURE-021`: manifest exposure refs must be portable and sanitized and must not contain raw payloads, secrets, or private absolute host paths.
- `INV-PROJECT-EXPOSURE-022`: `owner_ref` and `trace_ref` are mandatory across request, candidate, confirmation, and future manifest exposure entry.
- `INV-PROJECT-EXPOSURE-PLAN-001`: `F13.5A` is SPEC-only and introduces no runtime.
- `INV-PROJECT-EXPOSURE-PLAN-002`: future `F13.5B` manifest exposure runtime is owned by `project_runtime` only.
- `INV-PROJECT-EXPOSURE-PLAN-003`: future `F13.5B` must require `ExposureRequest`, `ExposureCandidate`, accepted `HumanConfirmation`, non-stale candidate state, `owner_ref`, and `trace_ref`.
- `INV-PROJECT-EXPOSURE-PLAN-004`: future `F13.5B` must not generate `registry.json`, graph writes, derivatives, `document_text_runtime` calls, or `tool_runtime` orchestration.
- `INV-PROJECT-EXPOSURE-PLAN-005`: future manifest exposure runtime must preserve duplicate-safe logical identity and must not collapse identity by blob reuse.
- `INV-PROJECT-EXPOSURE-PLAN-006`: future `audit_f13_manifest_exposure_runtime_boundary.bat` is mandatory before `F13.5B` closure.
- `INV-LEGACY-BYPASS-001`: the only canonical project exposure path is `F12` intake followed by `F13` exposure gate and future manifest-governed visibility.
- `INV-LEGACY-BYPASS-002`: legacy import, tree scanning, chat-resource registration, and direct derivation must not become implicit project exposure pipelines.
- `INV-LEGACY-BYPASS-003`: filesystem presence, copied files, chat references, registry rows, graph rows, and derivatives must not create project exposure.
- `INV-LEGACY-BYPASS-004`: UI and `app_services` must not import, expose, register, derive, confirm, or shadow manifest authority directly.
- `INV-LEGACY-BYPASS-005`: `import_project_document` and `list_project_documents` are legacy-only and must be blocked from production exposure authority before `F13` runtime.
- `INV-LEGACY-BYPASS-006`: `register_chat_resource` may remain chat-local only and must not become intake or exposure authority.
- `INV-LEGACY-BYPASS-007`: `derive_document_text` remains a separate derivative concern and must not be triggered by intake or exposure.
- `INV-LEGACY-BYPASS-008`: `F13.1` is SPEC-only and creates no runtime, manifest writer, registry generator, graph writer, derivative call, `tool_runtime` call, `document_text_runtime` call, or UI authority.

## Related specs

- `governance/GOVERNANCE.md`
- `governance/FUNCTIONAL_SCOPE.md`
- `architecture/ARCHITECTURE.md`
- `governance/WORKSPACE_RULES.md`
- `docs/specs/storage_policy.md`
- `docs/specs/project_folder_layout.md`
- `docs/specs/project_runtime.md`
- `docs/specs/document_tree.md`
- `docs/specs/document_references.md`
- `docs/specs/document_text_runtime.md`
- `docs/specs/security_sanitization_policy.md`
- `docs/specs/system_observability.md`
- `docs/specs/system_view.md`
- `docs/specs/tool_implementation_governance.md`
- `docs/specs/tools_catalogs.md`
