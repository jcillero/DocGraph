//! Shared IO and persisted resource access boundaries for the sandbox runtime.
//!
//! This crate owns controlled filesystem mutations that do not belong inside a
//! governed domain vertical.

mod storage_contracts;
mod file_intake;

pub use file_intake::{
    run_file_intake_batch, CommentSanitizationState, FileIntakeBatch,
    FileIntakeBatchRequest, FileIntakeError, FileIntakeItem, FileIntakeSelectedFile,
    FileIntakeStatus, IntakeClassification, IntakeCommentMetadata, IntakeDetectedKind,
    IntakeExposureState, IntakeMetadata, IntakeSecuritySanitizationState,
};
pub use storage_contracts::{BlobIntegrity, BlobIntegrityStatus, BlobRecord, BlobRecordHashAlgorithm};

use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use workspace_core::{ProjectRoot, WorkspacePath};

const CHAT_RESOURCES_DIRNAME: &str = "resources";
const CHAT_EVENT_LOG_FILENAME: &str = "chat.jsonl";
const KNOWLEDGE_DIRNAME: &str = "knowledge";
const PROJECT_RESOURCES_DIRNAME: &str = "resources";
const OUTPUTS_DIRNAME: &str = "outputs";
const TEXT_DIRNAME: &str = "text";
const SEMANTICS_DIRNAME: &str = "semantics";
const PAGES_DIRNAME: &str = "pages";
const CHUNKS_DIRNAME: &str = "chunks";
const RESOURCE_EVENT_TYPE: &str = "resource_linked";

/// Canonical persisted chat resource registered under a chat root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatResource {
    resource_id: String,
    original_filename: String,
    relative_path: String,
    created_at: String,
}

impl ChatResource {
    /// Build a minimal persisted chat resource contract.
    pub fn new(
        resource_id: impl Into<String>,
        original_filename: impl Into<String>,
        relative_path: impl Into<String>,
        created_at: impl Into<String>,
    ) -> Self {
        Self {
            resource_id: resource_id.into(),
            original_filename: original_filename.into(),
            relative_path: relative_path.into(),
            created_at: created_at.into(),
        }
    }

    /// Return the stable resource identifier.
    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }

    /// Return the original filename preserved on disk.
    pub fn original_filename(&self) -> &str {
        &self.original_filename
    }

    /// Return the portable path relative to the chat root.
    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }

    /// Return the creation timestamp captured during registration.
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}

/// Canonical readonly knowledge document exposed from the project `knowledge/` tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeDocumentRef {
    document_id: String,
    display_name: String,
    logical_path: WorkspacePath,
}

/// Canonical readonly project document exposed from known project roots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDocumentRef {
    document_id: String,
    display_name: String,
    logical_path: WorkspacePath,
    area: ProjectDocumentArea,
}

impl ProjectDocumentRef {
    /// Build a minimal project document descriptor.
    pub fn new(
        document_id: impl Into<String>,
        display_name: impl Into<String>,
        logical_path: WorkspacePath,
        area: ProjectDocumentArea,
    ) -> Self {
        Self {
            document_id: document_id.into(),
            display_name: display_name.into(),
            logical_path,
            area,
        }
    }

    /// Return the stable project document identifier.
    pub fn document_id(&self) -> &str {
        &self.document_id
    }

    /// Return the display name shown by thin presentation layers.
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Return the project-relative logical path.
    pub fn logical_path(&self) -> &WorkspacePath {
        &self.logical_path
    }

    /// Return the governed area that owns the primary source.
    pub fn area(&self) -> ProjectDocumentArea {
        self.area
    }
}

/// Small governed area discriminator for project-owned primary documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectDocumentArea {
    Knowledge,
    Resource,
    Output,
}

impl ProjectDocumentArea {
    /// Return the stable root directory name for this area.
    pub fn root_dirname(self) -> &'static str {
        match self {
            Self::Knowledge => KNOWLEDGE_DIRNAME,
            Self::Resource => PROJECT_RESOURCES_DIRNAME,
            Self::Output => OUTPUTS_DIRNAME,
        }
    }

    /// Return the stable portable area label used by light consumers.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Knowledge => "knowledge",
            Self::Resource => "resource",
            Self::Output => "output",
        }
    }
}

impl KnowledgeDocumentRef {
    /// Build a minimal readonly knowledge document descriptor.
    pub fn new(
        document_id: impl Into<String>,
        display_name: impl Into<String>,
        logical_path: WorkspacePath,
    ) -> Self {
        Self {
            document_id: document_id.into(),
            display_name: display_name.into(),
            logical_path,
        }
    }

    /// Return the stable knowledge document identifier.
    pub fn document_id(&self) -> &str {
        &self.document_id
    }

    /// Return the display name shown by thin presentation layers.
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Return the project-relative logical path under `knowledge/`.
    pub fn logical_path(&self) -> &WorkspacePath {
        &self.logical_path
    }
}

/// Small IO error for chat resource registration and JSONL event persistence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IoRuntimeError {
    ChatRootMustBeAbsolute {
        path: PathBuf,
    },
    ChatRootDoesNotExist {
        path: PathBuf,
    },
    ChatRootIsNotDirectory {
        path: PathBuf,
    },
    SourceFileMustBeAbsolute {
        path: PathBuf,
    },
    SourceFileDoesNotExist {
        path: PathBuf,
    },
    SourceFileIsNotFile {
        path: PathBuf,
    },
    FileNameMissing {
        path: PathBuf,
    },
    TimestampUnavailable,
    PathEscapeDetected {
        root: PathBuf,
        path: PathBuf,
    },
    ResourceDirectoryCreateFailed {
        path: PathBuf,
        message: String,
    },
    ResourceAlreadyExists {
        path: PathBuf,
    },
    ResourceCopyFailed {
        source: PathBuf,
        destination: PathBuf,
        message: String,
    },
    JsonlSerializeFailed {
        message: String,
    },
    JsonlAppendFailed {
        path: PathBuf,
        message: String,
    },
    KnowledgeRootIsNotDirectory {
        path: PathBuf,
    },
    KnowledgeDirectoryReadFailed {
        path: PathBuf,
        message: String,
    },
    KnowledgeDocumentNotFound {
        document_id: String,
    },
    ProjectDocumentDirectoryReadFailed {
        path: PathBuf,
        message: String,
    },
    ProjectDocumentNotFound {
        document_id: String,
    },
    ProjectDocumentRootCreateFailed {
        path: PathBuf,
        message: String,
    },
    ProjectDocumentDirectoryCreateFailed {
        path: PathBuf,
        message: String,
    },
    ProjectDocumentCopyFailed {
        source: PathBuf,
        destination: PathBuf,
        message: String,
    },
}

impl std::fmt::Display for IoRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChatRootMustBeAbsolute { path } => {
                write!(f, "chat_root must be absolute: {}", path.display())
            }
            Self::ChatRootDoesNotExist { path } => {
                write!(f, "chat_root does not exist: {}", path.display())
            }
            Self::ChatRootIsNotDirectory { path } => {
                write!(f, "chat_root must be a directory: {}", path.display())
            }
            Self::SourceFileMustBeAbsolute { path } => {
                write!(f, "source_file_path must be absolute: {}", path.display())
            }
            Self::SourceFileDoesNotExist { path } => {
                write!(f, "source_file_path does not exist: {}", path.display())
            }
            Self::SourceFileIsNotFile { path } => {
                write!(f, "source_file_path must be a file: {}", path.display())
            }
            Self::FileNameMissing { path } => {
                write!(f, "source file has no filename: {}", path.display())
            }
            Self::TimestampUnavailable => write!(f, "failed to obtain current timestamp"),
            Self::PathEscapeDetected { root, path } => write!(
                f,
                "path '{}' escapes chat_root '{}'",
                path.display(),
                root.display()
            ),
            Self::ResourceDirectoryCreateFailed { path, message } => write!(
                f,
                "failed to create resource directory '{}': {}",
                path.display(),
                message
            ),
            Self::ResourceAlreadyExists { path } => {
                write!(f, "resource directory already exists: {}", path.display())
            }
            Self::ResourceCopyFailed {
                source,
                destination,
                message,
            } => write!(
                f,
                "failed to copy '{}' to '{}': {}",
                source.display(),
                destination.display(),
                message
            ),
            Self::JsonlSerializeFailed { message } => {
                write!(f, "failed to serialize JSONL record: {message}")
            }
            Self::JsonlAppendFailed { path, message } => {
                write!(
                    f,
                    "failed to append JSONL record to '{}': {}",
                    path.display(),
                    message
                )
            }
            Self::KnowledgeRootIsNotDirectory { path } => {
                write!(f, "knowledge root must be a directory: {}", path.display())
            }
            Self::KnowledgeDirectoryReadFailed { path, message } => write!(
                f,
                "failed to read knowledge directory '{}': {}",
                path.display(),
                message
            ),
            Self::KnowledgeDocumentNotFound { document_id } => {
                write!(f, "knowledge document was not found: {document_id}")
            }
            Self::ProjectDocumentDirectoryReadFailed { path, message } => write!(
                f,
                "failed to read project document directory '{}': {}",
                path.display(),
                message
            ),
            Self::ProjectDocumentNotFound { document_id } => {
                write!(f, "project document was not found: {document_id}")
            }
            Self::ProjectDocumentRootCreateFailed { path, message } => write!(
                f,
                "failed to create project document root '{}': {}",
                path.display(),
                message
            ),
            Self::ProjectDocumentDirectoryCreateFailed { path, message } => write!(
                f,
                "failed to create project document directory '{}': {}",
                path.display(),
                message
            ),
            Self::ProjectDocumentCopyFailed {
                source,
                destination,
                message,
            } => write!(
                f,
                "failed to copy '{}' to '{}': {}",
                source.display(),
                destination.display(),
                message
            ),
        }
    }
}

impl std::error::Error for IoRuntimeError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ChatResourceLinkedRecord {
    record_id: String,
    #[serde(rename = "type")]
    type_name: &'static str,
    timestamp: String,
    event_id: String,
    resource_id: String,
    relative_path: String,
    original_filename: String,
}

/// Register a user-provided file as a persisted chat resource under the chat root.
pub fn register_chat_resource(
    chat_root: impl AsRef<Path>,
    source_file_path: impl AsRef<Path>,
) -> Result<ChatResource, IoRuntimeError> {
    let chat_root = canonicalize_chat_root(chat_root.as_ref())?;
    let source_file_path = canonicalize_source_file(source_file_path.as_ref())?;
    let original_filename = source_file_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| IoRuntimeError::FileNameMissing {
            path: source_file_path.clone(),
        })?
        .to_owned();
    let slug = slug_from_filename(&original_filename);
    let short_id = generate_short_id()?;
    let resource_id = format!("doc_{slug}__{short_id}");
    let created_at = current_timestamp_text()?;
    let resources_root = chat_root.join(CHAT_RESOURCES_DIRNAME);
    fs::create_dir_all(&resources_root).map_err(|error| {
        IoRuntimeError::ResourceDirectoryCreateFailed {
            path: resources_root.clone(),
            message: error.to_string(),
        }
    })?;

    let resource_dir = resources_root.join(&resource_id);
    if resource_dir.exists() {
        return Err(IoRuntimeError::ResourceAlreadyExists { path: resource_dir });
    }

    fs::create_dir(&resource_dir).map_err(|error| {
        IoRuntimeError::ResourceDirectoryCreateFailed {
            path: resource_dir.clone(),
            message: error.to_string(),
        }
    })?;
    let canonical_resource_dir =
        fs::canonicalize(&resource_dir).map_err(|_| IoRuntimeError::PathEscapeDetected {
            root: chat_root.clone(),
            path: resource_dir.clone(),
        })?;
    ensure_path_within_root(&chat_root, &canonical_resource_dir)?;

    let destination_file = resource_dir.join(&original_filename);
    fs::copy(&source_file_path, &destination_file).map_err(|error| {
        IoRuntimeError::ResourceCopyFailed {
            source: source_file_path.clone(),
            destination: destination_file.clone(),
            message: error.to_string(),
        }
    })?;
    let canonical_destination_file =
        fs::canonicalize(&destination_file).map_err(|_| IoRuntimeError::PathEscapeDetected {
            root: chat_root.clone(),
            path: destination_file.clone(),
        })?;
    let relative_path =
        build_chat_relative_path(&chat_root, &canonical_destination_file)?.portable_text();

    let resource = ChatResource::new(
        resource_id.clone(),
        original_filename.clone(),
        relative_path.clone(),
        created_at.clone(),
    );
    let record = ChatResourceLinkedRecord {
        record_id: generate_prefixed_id("record")?,
        type_name: RESOURCE_EVENT_TYPE,
        timestamp: created_at,
        event_id: generate_prefixed_id("event")?,
        resource_id,
        relative_path,
        original_filename,
    };
    append_jsonl_record(chat_root.join(CHAT_EVENT_LOG_FILENAME), &record)?;

    Ok(resource)
}

/// List readonly project knowledge documents under the canonical `knowledge/` root.
pub fn list_project_knowledge_documents(
    project_root: &ProjectRoot,
) -> Result<Vec<KnowledgeDocumentRef>, IoRuntimeError> {
    let knowledge_root = project_root.join(&WorkspacePath::new(KNOWLEDGE_DIRNAME).expect("valid"));

    if !knowledge_root.exists() {
        return Ok(Vec::new());
    }
    if !knowledge_root.is_dir() {
        return Err(IoRuntimeError::KnowledgeRootIsNotDirectory {
            path: knowledge_root,
        });
    }

    let mut documents = Vec::new();
    collect_knowledge_documents(project_root.as_path(), &knowledge_root, &mut documents)?;
    documents.sort_by_key(|document| document.logical_path().portable_text());
    Ok(documents)
}

/// Resolve a readonly knowledge document by its stable document id.
pub fn resolve_project_knowledge_document(
    project_root: &ProjectRoot,
    document_id: &str,
) -> Result<KnowledgeDocumentRef, IoRuntimeError> {
    let documents = list_project_knowledge_documents(project_root)?;
    documents
        .into_iter()
        .find(|document| document.document_id() == document_id)
        .ok_or_else(|| IoRuntimeError::KnowledgeDocumentNotFound {
            document_id: document_id.to_owned(),
        })
}

/// List primary project documents from governed project roots without surfacing derived text.
pub fn list_project_documents(
    project_root: &ProjectRoot,
) -> Result<Vec<ProjectDocumentRef>, IoRuntimeError> {
    let mut documents = Vec::new();

    for area in [
        ProjectDocumentArea::Knowledge,
        ProjectDocumentArea::Resource,
        ProjectDocumentArea::Output,
    ] {
        let root = project_root.join(&WorkspacePath::new(area.root_dirname()).expect("valid"));
        if !root.exists() {
            continue;
        }
        if !root.is_dir() {
            return Err(IoRuntimeError::ProjectDocumentDirectoryReadFailed {
                path: root,
                message: "area root must be a directory".to_owned(),
            });
        }

        collect_project_documents(project_root.as_path(), &root, area, &mut documents)?;
    }

    documents.sort_by_key(|document| document.logical_path().portable_text());
    Ok(documents)
}

/// Resolve a primary project document by its stable document id.
pub fn resolve_project_document(
    project_root: &ProjectRoot,
    document_id: &str,
) -> Result<ProjectDocumentRef, IoRuntimeError> {
    let documents = list_project_documents(project_root)?;
    documents
        .into_iter()
        .find(|document| document.document_id() == document_id)
        .ok_or_else(|| IoRuntimeError::ProjectDocumentNotFound {
            document_id: document_id.to_owned(),
        })
}

/// Import an external file into a governed project document unit under the requested area.
pub fn import_project_document(
    project_root: &ProjectRoot,
    source_file_path: impl AsRef<Path>,
    area: ProjectDocumentArea,
) -> Result<ProjectDocumentRef, IoRuntimeError> {
    let source_file_path = canonicalize_source_file(source_file_path.as_ref())?;
    let original_filename = source_file_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| IoRuntimeError::FileNameMissing {
            path: source_file_path.clone(),
        })?
        .to_owned();
    let slug = slug_from_filename(&original_filename);
    let short_id = generate_short_id()?;
    let unit_id = format!("doc_{slug}__{short_id}");
    let area_root = project_root.join(&WorkspacePath::new(area.root_dirname()).expect("valid"));

    fs::create_dir_all(&area_root).map_err(|error| {
        IoRuntimeError::ProjectDocumentRootCreateFailed {
            path: area_root.clone(),
            message: error.to_string(),
        }
    })?;

    let document_root = area_root.join(&unit_id);
    if document_root.exists() {
        return Err(IoRuntimeError::ResourceAlreadyExists {
            path: document_root,
        });
    }

    fs::create_dir(&document_root).map_err(|error| {
        IoRuntimeError::ProjectDocumentDirectoryCreateFailed {
            path: document_root.clone(),
            message: error.to_string(),
        }
    })?;
    let canonical_document_root =
        fs::canonicalize(&document_root).map_err(|_| IoRuntimeError::PathEscapeDetected {
            root: project_root.as_path().to_path_buf(),
            path: document_root.clone(),
        })?;
    ensure_path_within_root(project_root.as_path(), &canonical_document_root)?;

    let destination_file = document_root.join(&original_filename);
    fs::copy(&source_file_path, &destination_file).map_err(|error| {
        IoRuntimeError::ProjectDocumentCopyFailed {
            source: source_file_path.clone(),
            destination: destination_file.clone(),
            message: error.to_string(),
        }
    })?;
    let canonical_destination_file =
        fs::canonicalize(&destination_file).map_err(|_| IoRuntimeError::PathEscapeDetected {
            root: project_root.as_path().to_path_buf(),
            path: destination_file.clone(),
        })?;
    let logical_path =
        build_root_relative_path(project_root.as_path(), &canonical_destination_file)?;
    let document_id = logical_path.portable_text();

    Ok(ProjectDocumentRef::new(
        document_id,
        original_filename,
        logical_path,
        area,
    ))
}

fn canonicalize_chat_root(path: &Path) -> Result<PathBuf, IoRuntimeError> {
    if !path.is_absolute() {
        return Err(IoRuntimeError::ChatRootMustBeAbsolute {
            path: path.to_path_buf(),
        });
    }
    if !path.exists() {
        return Err(IoRuntimeError::ChatRootDoesNotExist {
            path: path.to_path_buf(),
        });
    }
    if !path.is_dir() {
        return Err(IoRuntimeError::ChatRootIsNotDirectory {
            path: path.to_path_buf(),
        });
    }
    fs::canonicalize(path).map_err(|_| IoRuntimeError::PathEscapeDetected {
        root: path.to_path_buf(),
        path: path.to_path_buf(),
    })
}

fn canonicalize_source_file(path: &Path) -> Result<PathBuf, IoRuntimeError> {
    if !path.is_absolute() {
        return Err(IoRuntimeError::SourceFileMustBeAbsolute {
            path: path.to_path_buf(),
        });
    }
    if !path.exists() {
        return Err(IoRuntimeError::SourceFileDoesNotExist {
            path: path.to_path_buf(),
        });
    }
    if !path.is_file() {
        return Err(IoRuntimeError::SourceFileIsNotFile {
            path: path.to_path_buf(),
        });
    }
    fs::canonicalize(path).map_err(|_| IoRuntimeError::SourceFileDoesNotExist {
        path: path.to_path_buf(),
    })
}

fn build_root_relative_path(
    root: &Path,
    candidate: &Path,
) -> Result<WorkspacePath, IoRuntimeError> {
    ensure_path_within_root(root, candidate)?;
    let normalized_root = normalize_absolute_comparison_path(root);
    let normalized_candidate = normalize_absolute_comparison_path(candidate);
    let relative = normalized_candidate
        .strip_prefix(&normalized_root)
        .map_err(|_| IoRuntimeError::PathEscapeDetected {
            root: root.to_path_buf(),
            path: candidate.to_path_buf(),
        })?;
    WorkspacePath::from_path(relative).map_err(|_| IoRuntimeError::PathEscapeDetected {
        root: root.to_path_buf(),
        path: candidate.to_path_buf(),
    })
}

fn build_chat_relative_path(
    chat_root: &Path,
    candidate: &Path,
) -> Result<WorkspacePath, IoRuntimeError> {
    build_root_relative_path(chat_root, candidate)
}

fn ensure_path_within_root(root: &Path, path: &Path) -> Result<(), IoRuntimeError> {
    let normalized_root = normalize_absolute_comparison_path(root);
    let normalized_path = normalize_absolute_comparison_path(path);
    if normalized_path.starts_with(&normalized_root) {
        Ok(())
    } else {
        Err(IoRuntimeError::PathEscapeDetected {
            root: root.to_path_buf(),
            path: path.to_path_buf(),
        })
    }
}

fn normalize_absolute_comparison_path(path: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        let rendered = path.display().to_string();
        if let Some(trimmed) = rendered.strip_prefix(r"\\?\") {
            PathBuf::from(trimmed)
        } else {
            path.to_path_buf()
        }
    }

    #[cfg(not(windows))]
    {
        path.to_path_buf()
    }
}

fn collect_knowledge_documents(
    project_root: &Path,
    current_dir: &Path,
    documents: &mut Vec<KnowledgeDocumentRef>,
) -> Result<(), IoRuntimeError> {
    let mut entries = fs::read_dir(current_dir)
        .map_err(|error| IoRuntimeError::KnowledgeDirectoryReadFailed {
            path: current_dir.to_path_buf(),
            message: error.to_string(),
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| IoRuntimeError::KnowledgeDirectoryReadFailed {
            path: current_dir.to_path_buf(),
            message: error.to_string(),
        })?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();

        if path.is_dir() {
            if is_derived_document_dir(&path) {
                continue;
            }
            collect_knowledge_documents(project_root, &path, documents)?;
            continue;
        }
        if !path.is_file() {
            continue;
        }

        let canonical_path =
            fs::canonicalize(&path).map_err(|_| IoRuntimeError::PathEscapeDetected {
                root: project_root.to_path_buf(),
                path: path.clone(),
            })?;
        let logical_path = build_root_relative_path(project_root, &canonical_path)?;
        let document_id = logical_path.portable_text();
        let display_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| IoRuntimeError::FileNameMissing { path: path.clone() })?
            .to_owned();

        documents.push(KnowledgeDocumentRef::new(
            document_id,
            display_name,
            logical_path,
        ));
    }

    Ok(())
}

fn collect_project_documents(
    project_root: &Path,
    current_dir: &Path,
    area: ProjectDocumentArea,
    documents: &mut Vec<ProjectDocumentRef>,
) -> Result<(), IoRuntimeError> {
    let mut entries = fs::read_dir(current_dir)
        .map_err(|error| IoRuntimeError::ProjectDocumentDirectoryReadFailed {
            path: current_dir.to_path_buf(),
            message: error.to_string(),
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| IoRuntimeError::ProjectDocumentDirectoryReadFailed {
            path: current_dir.to_path_buf(),
            message: error.to_string(),
        })?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();

        if path.is_dir() {
            if is_derived_document_dir(&path) {
                continue;
            }
            collect_project_documents(project_root, &path, area, documents)?;
            continue;
        }
        if !path.is_file() {
            continue;
        }

        let canonical_path =
            fs::canonicalize(&path).map_err(|_| IoRuntimeError::PathEscapeDetected {
                root: project_root.to_path_buf(),
                path: path.clone(),
            })?;
        let logical_path = build_root_relative_path(project_root, &canonical_path)?;
        let document_id = logical_path.portable_text();
        let display_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| IoRuntimeError::FileNameMissing { path: path.clone() })?
            .to_owned();

        documents.push(ProjectDocumentRef::new(
            document_id,
            display_name,
            logical_path,
            area,
        ));
    }

    Ok(())
}

fn is_derived_document_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| {
            matches!(
                name,
                TEXT_DIRNAME | SEMANTICS_DIRNAME | PAGES_DIRNAME | CHUNKS_DIRNAME
            )
        })
        .unwrap_or(false)
}

fn slug_from_filename(filename: &str) -> String {
    let base = Path::new(filename)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(filename);
    let mut slug = String::new();
    let mut last_was_separator = false;

    for ch in base.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator {
            slug.push('_');
            last_was_separator = true;
        }
    }

    let trimmed = slug.trim_matches('_').to_owned();
    if trimmed.is_empty() {
        "resource".to_owned()
    } else {
        trimmed
    }
}

fn current_timestamp_text() -> Result<String, IoRuntimeError> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| IoRuntimeError::TimestampUnavailable)?
        .as_millis();
    Ok(format!("unix_ms:{millis}"))
}

fn generate_short_id() -> Result<String, IoRuntimeError> {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| IoRuntimeError::TimestampUnavailable)?
        .as_nanos();
    Ok(format!("{:010x}", (nanos as u64) & 0xffffffffff))
}

fn generate_prefixed_id(prefix: &str) -> Result<String, IoRuntimeError> {
    Ok(format!("{prefix}_{}", generate_short_id()?))
}

fn append_jsonl_record<T: Serialize>(path: PathBuf, record: &T) -> Result<(), IoRuntimeError> {
    let line =
        serde_json::to_string(record).map_err(|error| IoRuntimeError::JsonlSerializeFailed {
            message: error.to_string(),
        })?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| IoRuntimeError::JsonlAppendFailed {
            path: path.clone(),
            message: error.to_string(),
        })?;
    file.write_all(line.as_bytes())
        .and_then(|_| file.write_all(b"\n"))
        .map_err(|error| IoRuntimeError::JsonlAppendFailed {
            path,
            message: error.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::{
        build_chat_relative_path, import_project_document, list_project_documents,
        list_project_knowledge_documents, register_chat_resource, resolve_project_document,
        resolve_project_knowledge_document, IoRuntimeError, ProjectDocumentArea,
        RESOURCE_EVENT_TYPE,
    };
    use serde_json::Value;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use workspace_core::{ProjectRoot, WorkspaceRoot};

    #[test]
    fn registers_chat_resource_and_returns_portable_contract() {
        let (chat_root, source_file, upload_root) =
            create_chat_root_with_source("io_runtime_register_ok");

        let resource = register_chat_resource(&chat_root, &source_file).expect("chat resource");

        assert!(resource.resource_id().starts_with("doc_readme__"));
        assert_eq!(resource.original_filename(), "README.md");
        assert!(resource
            .relative_path()
            .starts_with("resources/doc_readme__"));
        assert!(resource.relative_path().ends_with("/README.md"));
        assert!(resource.created_at().starts_with("unix_ms:"));

        fs::remove_dir_all(chat_root).expect("cleanup chat root");
        fs::remove_dir_all(upload_root).expect("cleanup upload root");
    }

    #[test]
    fn creates_one_resource_folder_per_registered_file() {
        let (chat_root, source_file, upload_root) =
            create_chat_root_with_source("io_runtime_register_structure");

        let resource = register_chat_resource(&chat_root, &source_file).expect("chat resource");
        let resource_path = chat_root.join(resource.relative_path());

        assert!(resource_path.exists());
        assert_eq!(
            resource_path.file_name().and_then(|name| name.to_str()),
            Some("README.md")
        );
        assert!(resource_path.parent().expect("resource dir").is_dir());
        assert_eq!(
            resource_path
                .parent()
                .and_then(|dir| dir.file_name())
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("doc_readme__")),
            Some(true)
        );

        fs::remove_dir_all(chat_root).expect("cleanup chat root");
        fs::remove_dir_all(upload_root).expect("cleanup upload root");
    }

    #[test]
    fn appends_valid_resource_linked_event_to_chat_jsonl() {
        let (chat_root, source_file, upload_root) =
            create_chat_root_with_source("io_runtime_register_jsonl");

        let resource = register_chat_resource(&chat_root, &source_file).expect("chat resource");
        let jsonl = fs::read_to_string(chat_root.join("chat.jsonl")).expect("read chat jsonl");
        let line = jsonl.lines().next().expect("first jsonl line");
        let record = serde_json::from_str::<Value>(line).expect("parse json line");

        assert_eq!(record["type"], RESOURCE_EVENT_TYPE);
        assert_eq!(record["resource_id"], resource.resource_id());
        assert_eq!(record["relative_path"], resource.relative_path());
        assert_eq!(record["original_filename"], resource.original_filename());
        assert!(record["record_id"].as_str().is_some());
        assert!(record["event_id"].as_str().is_some());
        assert!(record["timestamp"].as_str().is_some());

        fs::remove_dir_all(chat_root).expect("cleanup chat root");
        fs::remove_dir_all(upload_root).expect("cleanup upload root");
    }

    #[test]
    fn rejects_relative_path_escape_outside_chat_root() {
        let chat_root = unique_temp_dir("io_runtime_escape_root");
        let outside_root = unique_temp_dir("io_runtime_escape_outside");
        let outside_file = outside_root.join("outside.txt");

        fs::create_dir_all(&chat_root).expect("create chat root");
        fs::create_dir_all(outside_file.parent().expect("outside parent"))
            .expect("create outside parent");
        fs::write(&outside_file, "outside").expect("write outside file");

        let chat_root = fs::canonicalize(chat_root).expect("canonical chat root");
        let outside_file = fs::canonicalize(outside_file).expect("canonical outside file");
        let error = build_chat_relative_path(&chat_root, &outside_file).expect_err("escape error");

        assert!(matches!(error, IoRuntimeError::PathEscapeDetected { .. }));

        fs::remove_dir_all(chat_root).expect("cleanup chat root");
        fs::remove_dir_all(outside_root).expect("cleanup outside root");
    }

    #[test]
    fn lists_project_knowledge_documents_from_project_knowledge_root() {
        let (_, project_root, workspace_dir) =
            create_project_root_with_knowledge("io_runtime_knowledge_list");

        let documents = list_project_knowledge_documents(&project_root).expect("knowledge docs");

        assert_eq!(documents.len(), 2);
        assert_eq!(documents[0].document_id(), "knowledge/guides/setup.md");
        assert_eq!(documents[0].display_name(), "setup.md");
        assert_eq!(
            documents[0].logical_path().portable_text(),
            "knowledge/guides/setup.md"
        );
        assert_eq!(documents[1].document_id(), "knowledge/overview.md");

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn returns_empty_when_project_knowledge_root_is_missing() {
        let workspace_dir = unique_temp_dir("io_runtime_knowledge_empty");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        fs::create_dir_all(&project_dir).expect("create project dir");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let documents = list_project_knowledge_documents(&project_root).expect("knowledge docs");

        assert!(documents.is_empty());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn resolves_project_knowledge_document_by_stable_document_id() {
        let (_, project_root, workspace_dir) =
            create_project_root_with_knowledge("io_runtime_knowledge_resolve");

        let document = resolve_project_knowledge_document(&project_root, "knowledge/overview.md")
            .expect("knowledge document");

        assert_eq!(document.document_id(), "knowledge/overview.md");
        assert_eq!(document.display_name(), "overview.md");

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn fails_when_project_knowledge_document_id_is_unknown() {
        let (_, project_root, workspace_dir) =
            create_project_root_with_knowledge("io_runtime_knowledge_missing");

        let error = resolve_project_knowledge_document(&project_root, "knowledge/missing.md")
            .expect_err("missing knowledge document");

        assert!(matches!(
            error,
            IoRuntimeError::KnowledgeDocumentNotFound { .. }
        ));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn imports_project_document_into_resources_as_governed_unit() {
        let workspace_dir = unique_temp_dir("io_runtime_project_import");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let uploads_dir = workspace_dir.join("uploads");
        let source_file = uploads_dir.join("brief.md");

        fs::create_dir_all(&project_dir).expect("create project dir");
        fs::create_dir_all(&uploads_dir).expect("create uploads dir");
        fs::write(&source_file, "# Brief").expect("write source");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let document = import_project_document(
            &project_root,
            fs::canonicalize(&source_file).expect("canonical source"),
            ProjectDocumentArea::Resource,
        )
        .expect("project document");

        assert_eq!(document.area(), ProjectDocumentArea::Resource);
        assert!(document.document_id().starts_with("resources/doc_brief__"));
        assert!(document.document_id().ends_with("/brief.md"));
        assert!(project_root.join(document.logical_path()).exists());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn lists_project_documents_without_exposing_derived_text_or_semantics() {
        let workspace_dir = unique_temp_dir("io_runtime_project_docs");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let knowledge_unit = project_dir.join("knowledge").join("doc_alpha__1234");
        let text_dir = knowledge_unit.join("text");
        let semantics_dir = knowledge_unit.join("semantics");
        let resources_dir = project_dir.join("resources").join("doc_beta__5678");

        fs::create_dir_all(&text_dir).expect("create text dir");
        fs::create_dir_all(&semantics_dir).expect("create semantics dir");
        fs::create_dir_all(&resources_dir).expect("create resources dir");
        fs::write(knowledge_unit.join("alpha.md"), "# Alpha").expect("write knowledge source");
        fs::write(text_dir.join("extracted.txt"), "derived").expect("write derived text");
        fs::write(semantics_dir.join("graph.json"), "{}").expect("write derived semantics");
        fs::write(resources_dir.join("beta.txt"), "beta").expect("write resource source");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let documents = list_project_documents(&project_root).expect("project documents");

        assert_eq!(documents.len(), 2);
        assert!(documents
            .iter()
            .any(|document| document.document_id() == "knowledge/doc_alpha__1234/alpha.md"));
        assert!(documents
            .iter()
            .any(|document| document.document_id() == "resources/doc_beta__5678/beta.txt"));
        assert!(!documents
            .iter()
            .any(|document| document.document_id().contains("/text/")));
        assert!(!documents
            .iter()
            .any(|document| document.document_id().contains("/semantics/")));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn resolves_project_document_by_stable_id() {
        let workspace_dir = unique_temp_dir("io_runtime_project_resolve");
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let resources_dir = project_dir.join("resources").join("doc_beta__5678");

        fs::create_dir_all(&resources_dir).expect("create resources dir");
        fs::write(resources_dir.join("beta.txt"), "beta").expect("write resource source");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");
        let document = resolve_project_document(&project_root, "resources/doc_beta__5678/beta.txt")
            .expect("resolved project document");

        assert_eq!(document.display_name(), "beta.txt");
        assert_eq!(document.area(), ProjectDocumentArea::Resource);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    fn create_chat_root_with_source(prefix: &str) -> (PathBuf, PathBuf, PathBuf) {
        let chat_root = unique_temp_dir(prefix);
        let upload_root = unique_temp_dir(&format!("{prefix}_upload"));
        let source_file = upload_root.join("README.md");

        fs::create_dir_all(&chat_root).expect("create chat root");
        fs::create_dir_all(&upload_root).expect("create upload root");
        fs::write(&source_file, "# uploaded").expect("write source file");

        (
            fs::canonicalize(chat_root).expect("canonical chat root"),
            fs::canonicalize(source_file).expect("canonical source file"),
            fs::canonicalize(upload_root).expect("canonical upload root"),
        )
    }

    fn create_project_root_with_knowledge(prefix: &str) -> (WorkspaceRoot, ProjectRoot, PathBuf) {
        let workspace_dir = unique_temp_dir(prefix);
        let project_dir = workspace_dir.join("user").join("projects").join("alpha");
        let knowledge_dir = project_dir.join("knowledge");
        let guides_dir = knowledge_dir.join("guides");

        fs::create_dir_all(&guides_dir).expect("create guides dir");
        fs::write(knowledge_dir.join("overview.md"), "# Overview").expect("write overview");
        fs::write(guides_dir.join("setup.md"), "# Setup").expect("write setup");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root = ProjectRoot::new(&workspace_root, project_dir).expect("project root");

        (workspace_root, project_root, workspace_dir)
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after unix epoch")
            .as_nanos();
        path.push(format!("rust_portable_app_{prefix}_{nanos}"));
        path
    }
}
