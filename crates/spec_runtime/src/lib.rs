//! Minimal declarative loading contracts for the Rust sandbox.

use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Component, PathBuf};

use core_domain::{ErrorDomain, PortableAppError};
use workspace_core::{WorkspacePath, WorkspaceRoot};

/// Result alias for spec and config contract construction.
pub type SpecRuntimeResult<T> = Result<T, SpecRuntimeError>;

/// Error type for declarative source and provenance construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecRuntimeError {
    EmptyDocumentName,
    EmptyInheritedSource,
    ProvenancePathMismatch,
    DocumentReadFailed { path: PathBuf },
}

impl SpecRuntimeError {
    fn code(&self) -> &'static str {
        match self {
            Self::EmptyDocumentName => "empty_document_name",
            Self::EmptyInheritedSource => "empty_inherited_source",
            Self::ProvenancePathMismatch => "provenance_path_mismatch",
            Self::DocumentReadFailed { .. } => "document_read_failed",
        }
    }
}

impl fmt::Display for SpecRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDocumentName => f.write_str("document name must not be empty"),
            Self::EmptyInheritedSource => {
                f.write_str("inherited source must not be empty when provided")
            }
            Self::ProvenancePathMismatch => {
                f.write_str("provenance declared path must match the document source path")
            }
            Self::DocumentReadFailed { path } => {
                write!(f, "failed to read document from disk: {}", path.display())
            }
        }
    }
}

impl Error for SpecRuntimeError {}

impl From<SpecRuntimeError> for PortableAppError {
    fn from(value: SpecRuntimeError) -> Self {
        PortableAppError::new(ErrorDomain::SpecRuntime, value.code(), value.to_string())
    }
}

/// Declarative category for a loaded document source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentCategory {
    Governance,
    Spec,
    Contract,
    Config,
    Registry,
    Manifest,
    Resource,
}

impl DocumentCategory {
    /// Return the stable category name used by lightweight consumers.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Governance => "governance",
            Self::Spec => "spec",
            Self::Contract => "contract",
            Self::Config => "config",
            Self::Registry => "registry",
            Self::Manifest => "manifest",
            Self::Resource => "resource",
        }
    }
}

/// Authority that declares the source of a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceAuthority {
    Governance,
    Spec,
    Config,
    Project,
    Sandbox,
}

impl SourceAuthority {
    /// Return the stable authority name used by lightweight consumers.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Governance => "governance",
            Self::Spec => "spec",
            Self::Config => "config",
            Self::Project => "project",
            Self::Sandbox => "sandbox",
        }
    }
}

/// Typed source descriptor for a loaded declarative document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSource {
    category: DocumentCategory,
    name: String,
    path: WorkspacePath,
}

impl DocumentSource {
    /// Build a typed document source descriptor.
    pub fn new(
        category: DocumentCategory,
        name: impl AsRef<str>,
        path: WorkspacePath,
    ) -> SpecRuntimeResult<Self> {
        let trimmed = name.as_ref().trim();
        if trimmed.is_empty() {
            return Err(SpecRuntimeError::EmptyDocumentName);
        }

        Ok(Self {
            category,
            name: trimmed.to_owned(),
            path,
        })
    }

    /// Return the source category.
    pub fn category(&self) -> DocumentCategory {
        self.category
    }

    /// Return the logical document name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the relative path declared for the source.
    pub fn path(&self) -> &WorkspacePath {
        &self.path
    }

    /// Return the declared relative path in a stable portable form.
    pub fn path_text(&self) -> String {
        stable_relative_path(&self.path)
    }
}

/// Provenance metadata kept alongside declarative payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceProvenance {
    authority: SourceAuthority,
    declared_path: WorkspacePath,
    inherited_from: Option<String>,
}

impl SourceProvenance {
    /// Build provenance metadata for a declarative source.
    pub fn new(
        authority: SourceAuthority,
        declared_path: WorkspacePath,
        inherited_from: Option<impl AsRef<str>>,
    ) -> SpecRuntimeResult<Self> {
        let inherited_from = match inherited_from {
            Some(value) => {
                let trimmed = value.as_ref().trim();
                if trimmed.is_empty() {
                    return Err(SpecRuntimeError::EmptyInheritedSource);
                }
                Some(trimmed.to_owned())
            }
            None => None,
        };

        Ok(Self {
            authority,
            declared_path,
            inherited_from,
        })
    }

    /// Return the declaring authority.
    pub fn authority(&self) -> SourceAuthority {
        self.authority
    }

    /// Return the declared relative path.
    pub fn declared_path(&self) -> &WorkspacePath {
        &self.declared_path
    }

    /// Return the declared relative path in a stable portable form.
    pub fn declared_path_text(&self) -> String {
        stable_relative_path(&self.declared_path)
    }

    /// Return the optional inherited source label.
    pub fn inherited_from(&self) -> Option<&str> {
        self.inherited_from.as_deref()
    }
}

/// Thin wrapper for a loaded declarative payload plus source metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedDocument {
    source: DocumentSource,
    provenance: SourceProvenance,
    contents: String,
}

impl LoadedDocument {
    /// Construct a loaded document wrapper while preserving provenance alignment.
    pub fn new(
        source: DocumentSource,
        provenance: SourceProvenance,
        contents: impl Into<String>,
    ) -> SpecRuntimeResult<Self> {
        if source.path() != provenance.declared_path() {
            return Err(SpecRuntimeError::ProvenancePathMismatch);
        }

        Ok(Self {
            source,
            provenance,
            contents: contents.into(),
        })
    }

    /// Return the source descriptor.
    pub fn source(&self) -> &DocumentSource {
        &self.source
    }

    /// Return the loaded document category.
    pub fn category(&self) -> DocumentCategory {
        self.source.category()
    }

    /// Return the logical document name.
    pub fn name(&self) -> &str {
        self.source.name()
    }

    /// Return the declared relative source path.
    pub fn declared_path(&self) -> &WorkspacePath {
        self.source.path()
    }

    /// Return the declared relative path in a stable portable form.
    pub fn declared_path_text(&self) -> String {
        self.source.path_text()
    }

    /// Return the provenance metadata.
    pub fn provenance(&self) -> &SourceProvenance {
        &self.provenance
    }

    /// Return the source authority carried by provenance.
    pub fn authority(&self) -> SourceAuthority {
        self.provenance.authority()
    }

    /// Return the optional inherited source label carried by provenance.
    pub fn inherited_from(&self) -> Option<&str> {
        self.provenance.inherited_from()
    }

    /// Return the raw loaded contents.
    pub fn contents(&self) -> &str {
        &self.contents
    }
}

/// Minimal loader contract for later workspace-backed loading.
pub trait DocumentLoader {
    /// Load a document using a typed declarative source descriptor.
    fn load(&self, source: &DocumentSource) -> SpecRuntimeResult<LoadedDocument>;
}

/// Load a document from disk using the workspace root plus the declarative source path.
pub fn load_document_from_disk(
    workspace_root: &WorkspaceRoot,
    source: DocumentSource,
    authority: SourceAuthority,
) -> SpecRuntimeResult<LoadedDocument> {
    let full_path = workspace_root.join(source.path());
    let contents = fs::read_to_string(&full_path)
        .map_err(|_| SpecRuntimeError::DocumentReadFailed { path: full_path })?;
    let provenance = SourceProvenance::new(authority, source.path().clone(), None::<&str>)?;

    LoadedDocument::new(source, provenance, contents)
}

/// Safe placeholder validation state for the first contract layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationState {
    Accepted,
    Deferred,
    Rejected,
}

/// Small machine-facing validation note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationNote {
    pub code: &'static str,
    pub message: String,
}

/// Minimal validation wrapper for declarative payload handling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub state: ValidationState,
    pub notes: Vec<ValidationNote>,
}

impl ValidationReport {
    /// Produce a deferred validation result with a single explanatory note.
    pub fn deferred(reason: impl Into<String>) -> Self {
        Self {
            state: ValidationState::Deferred,
            notes: vec![ValidationNote {
                code: "validation_deferred",
                message: reason.into(),
            }],
        }
    }
}

/// Conservative placeholder validation surface.
pub fn validate_loaded_document_placeholder(document: &LoadedDocument) -> ValidationReport {
    if document.contents().trim().is_empty() {
        ValidationReport::deferred("loaded document is empty; full schema validation is deferred")
    } else {
        ValidationReport {
            state: ValidationState::Accepted,
            notes: Vec::new(),
        }
    }
}

fn stable_relative_path(path: &WorkspacePath) -> String {
    path.as_path()
        .components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::{
        load_document_from_disk, validate_loaded_document_placeholder, DocumentCategory,
        DocumentSource, LoadedDocument, SourceAuthority, SourceProvenance, ValidationState,
    };
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use workspace_core::{WorkspacePath, WorkspaceRoot};

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before epoch")
            .as_nanos();
        path.push(format!("{prefix}_{}_{}", std::process::id(), stamp));
        path
    }

    #[test]
    fn builds_source_and_provenance() {
        let path = WorkspacePath::new("system/spec/example.json").expect("workspace path");
        let source = DocumentSource::new(DocumentCategory::Spec, "example_spec", path.clone())
            .expect("source");
        let provenance =
            SourceProvenance::new(SourceAuthority::Spec, path, Some("OPERATIONAL_DEFINITION"))
                .expect("provenance");

        let loaded = LoadedDocument::new(source, provenance, "{\"ok\":true}").expect("loaded");

        assert_eq!(loaded.source().name(), "example_spec");
        assert_eq!(loaded.category().as_str(), "spec");
        assert_eq!(loaded.name(), "example_spec");
        assert_eq!(loaded.declared_path_text(), "system/spec/example.json");
        assert_eq!(loaded.authority().as_str(), "spec");
        assert_eq!(loaded.inherited_from(), Some("OPERATIONAL_DEFINITION"));
    }

    #[test]
    fn rejects_mismatched_provenance_path() {
        let source_path = WorkspacePath::new("system/spec/one.json").expect("source path");
        let provenance_path = WorkspacePath::new("system/spec/two.json").expect("prov path");
        let source =
            DocumentSource::new(DocumentCategory::Spec, "example", source_path).expect("source");
        let provenance =
            SourceProvenance::new(SourceAuthority::Spec, provenance_path, None::<&str>)
                .expect("provenance");

        assert!(LoadedDocument::new(source, provenance, "{}").is_err());
    }

    #[test]
    fn marks_empty_document_validation_as_deferred() {
        let path = WorkspacePath::new("system/spec/empty.json").expect("workspace path");
        let source =
            DocumentSource::new(DocumentCategory::Spec, "empty", path.clone()).expect("source");
        let provenance =
            SourceProvenance::new(SourceAuthority::Spec, path, None::<&str>).expect("provenance");
        let loaded = LoadedDocument::new(source, provenance, "").expect("loaded");

        let report = validate_loaded_document_placeholder(&loaded);

        assert_eq!(report.state, ValidationState::Deferred);
    }

    #[test]
    fn loads_document_from_disk() {
        let workspace_dir = unique_temp_dir("spec_runtime_load_ok");
        let spec_dir = workspace_dir.join("system").join("spec");
        let spec_path = spec_dir.join("example.json");

        fs::create_dir_all(&spec_dir).expect("create spec dir");
        fs::write(&spec_path, "{\"status\":\"ok\"}").expect("write spec file");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let source = DocumentSource::new(
            DocumentCategory::Spec,
            "example",
            WorkspacePath::new("system/spec/example.json").expect("workspace path"),
        )
        .expect("source");

        let loaded = load_document_from_disk(&workspace_root, source, SourceAuthority::Spec)
            .expect("loaded");

        assert_eq!(loaded.contents(), "{\"status\":\"ok\"}");
        assert_eq!(loaded.category().as_str(), "spec");
        assert_eq!(loaded.name(), "example");
        assert_eq!(loaded.authority().as_str(), "spec");
        assert_eq!(loaded.inherited_from(), None);

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn missing_document_fails_cleanly() {
        let workspace_dir = unique_temp_dir("spec_runtime_load_missing");
        fs::create_dir_all(&workspace_dir).expect("create workspace dir");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let source = DocumentSource::new(
            DocumentCategory::Spec,
            "missing",
            WorkspacePath::new("system/spec/missing.json").expect("workspace path"),
        )
        .expect("source");

        assert!(load_document_from_disk(&workspace_root, source, SourceAuthority::Spec).is_err());

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }
}
