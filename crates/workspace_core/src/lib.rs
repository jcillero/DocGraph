//! Portable workspace boundaries and conservative path validation helpers.

use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

use core_domain::{ErrorDomain, PortableAppError};

/// Workspace-level result alias for conservative boundary validation.
pub type WorkspaceResult<T> = Result<T, WorkspaceError>;

/// Error type for workspace root and relative path validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceError {
    EmptyPath {
        field: &'static str,
    },
    PathMustBeAbsolute {
        field: &'static str,
        path: PathBuf,
    },
    PathDoesNotExist {
        field: &'static str,
        path: PathBuf,
    },
    PathIsNotDirectory {
        field: &'static str,
        path: PathBuf,
    },
    CanonicalizationFailed {
        field: &'static str,
        path: PathBuf,
    },
    PathOutsideWorkspace {
        workspace_root: PathBuf,
        path: PathBuf,
    },
    InvalidRelativePath {
        input: String,
        reason: &'static str,
    },
    ProjectOutsideWorkspace {
        workspace_root: PathBuf,
        project_root: PathBuf,
    },
}

impl WorkspaceError {
    fn code(&self) -> &'static str {
        match self {
            Self::EmptyPath { .. } => "empty_path",
            Self::PathMustBeAbsolute { .. } => "path_must_be_absolute",
            Self::PathDoesNotExist { .. } => "path_does_not_exist",
            Self::PathIsNotDirectory { .. } => "path_is_not_directory",
            Self::CanonicalizationFailed { .. } => "canonicalization_failed",
            Self::PathOutsideWorkspace { .. } => "path_outside_workspace",
            Self::InvalidRelativePath { .. } => "invalid_relative_path",
            Self::ProjectOutsideWorkspace { .. } => "project_outside_workspace",
        }
    }
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPath { field } => write!(f, "{field} must not be empty"),
            Self::PathMustBeAbsolute { field, path } => {
                write!(f, "{field} must be absolute: {}", path.display())
            }
            Self::PathDoesNotExist { field, path } => {
                write!(f, "{field} does not exist: {}", path.display())
            }
            Self::PathIsNotDirectory { field, path } => {
                write!(f, "{field} must point to a directory: {}", path.display())
            }
            Self::CanonicalizationFailed { field, path } => {
                write!(f, "failed to canonicalize {field}: {}", path.display())
            }
            Self::PathOutsideWorkspace {
                workspace_root,
                path,
            } => write!(
                f,
                "path '{}' is outside workspace root '{}'",
                path.display(),
                workspace_root.display()
            ),
            Self::InvalidRelativePath { input, reason } => {
                write!(f, "invalid relative path '{input}': {reason}")
            }
            Self::ProjectOutsideWorkspace {
                workspace_root,
                project_root,
            } => write!(
                f,
                "project root '{}' is outside workspace root '{}'",
                project_root.display(),
                workspace_root.display()
            ),
        }
    }
}

impl Error for WorkspaceError {}

impl From<WorkspaceError> for PortableAppError {
    fn from(value: WorkspaceError) -> Self {
        PortableAppError::new(ErrorDomain::Workspace, value.code(), value.to_string())
    }
}

/// Canonical absolute workspace root under the inherited portable workspace model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceRoot {
    path: PathBuf,
}

impl WorkspaceRoot {
    /// Build a validated canonical workspace root.
    pub fn new(path: impl Into<PathBuf>) -> WorkspaceResult<Self> {
        let canonical = canonicalize_directory("workspace_root", path.into())?;
        Ok(Self { path: canonical })
    }

    /// Borrow the canonical workspace root path.
    pub fn as_path(&self) -> &Path {
        &self.path
    }

    /// Join a validated workspace-relative path onto the workspace root.
    pub fn join(&self, relative: &WorkspacePath) -> PathBuf {
        self.path.join(relative.as_path())
    }

    /// Convert an absolute path under this workspace into a validated workspace-relative path.
    pub fn relative_path_for(&self, path: impl AsRef<Path>) -> WorkspaceResult<WorkspacePath> {
        let candidate = path.as_ref();

        if !candidate.is_absolute() {
            return Err(WorkspaceError::PathMustBeAbsolute {
                field: "workspace_relative_path",
                path: candidate.to_path_buf(),
            });
        }

        let normalized_workspace_root = normalize_absolute_comparison_path(self.as_path());
        let normalized_candidate = normalize_absolute_comparison_path(candidate);

        let relative = normalized_candidate
            .strip_prefix(&normalized_workspace_root)
            .map_err(|_| WorkspaceError::PathOutsideWorkspace {
                workspace_root: self.as_path().to_path_buf(),
                path: candidate.to_path_buf(),
            })?;

        WorkspacePath::from_path(relative)
    }
}

/// Canonical project root that must remain inside a workspace root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectRoot {
    path: PathBuf,
}

impl ProjectRoot {
    /// Build a validated project root under the given workspace root.
    pub fn new(workspace_root: &WorkspaceRoot, path: impl Into<PathBuf>) -> WorkspaceResult<Self> {
        let canonical = canonicalize_directory("project_root", path.into())?;
        validate_project_under_workspace(workspace_root, &canonical)?;
        Ok(Self { path: canonical })
    }

    /// Borrow the canonical project root path.
    pub fn as_path(&self) -> &Path {
        &self.path
    }

    /// Join a validated workspace-relative path onto the project root.
    pub fn join(&self, relative: &WorkspacePath) -> PathBuf {
        self.path.join(relative.as_path())
    }
}

/// Conservative normalized relative path accepted by the workspace layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkspacePath {
    path: PathBuf,
}

impl WorkspacePath {
    /// Parse and validate a conservative relative path.
    pub fn new(input: impl AsRef<str>) -> WorkspaceResult<Self> {
        Self::from_path(Path::new(input.as_ref().trim()))
    }

    /// Parse and validate a conservative relative path from a path value.
    pub fn from_path(path: impl AsRef<Path>) -> WorkspaceResult<Self> {
        let candidate = path.as_ref();
        let raw = candidate.to_string_lossy();
        let trimmed = raw.trim();

        if trimmed.is_empty() {
            return Err(WorkspaceError::InvalidRelativePath {
                input: raw.into_owned(),
                reason: "path must not be empty",
            });
        }

        let mut normalized = PathBuf::new();

        for component in candidate.components() {
            match component {
                Component::Normal(part) => normalized.push(part),
                Component::CurDir => {
                    return Err(WorkspaceError::InvalidRelativePath {
                        input: raw.into_owned(),
                        reason: "current-directory segments are not allowed",
                    });
                }
                Component::ParentDir => {
                    return Err(WorkspaceError::InvalidRelativePath {
                        input: raw.into_owned(),
                        reason: "parent-directory segments are not allowed",
                    });
                }
                Component::RootDir | Component::Prefix(_) => {
                    return Err(WorkspaceError::InvalidRelativePath {
                        input: raw.into_owned(),
                        reason: "absolute paths are not allowed",
                    });
                }
            }
        }

        if normalized.as_os_str().is_empty() {
            return Err(WorkspaceError::InvalidRelativePath {
                input: raw.into_owned(),
                reason: "path must contain at least one normal segment",
            });
        }

        Ok(Self { path: normalized })
    }

    /// Borrow the normalized relative path.
    pub fn as_path(&self) -> &Path {
        &self.path
    }

    /// Return the logical relative path in a stable portable form.
    pub fn portable_text(&self) -> String {
        stable_relative_path(&self.path)
    }
}

impl fmt::Display for WorkspacePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.portable_text())
    }
}

/// Validate that a candidate project root remains under the workspace root.
pub fn validate_project_under_workspace(
    workspace_root: &WorkspaceRoot,
    project_root: &Path,
) -> WorkspaceResult<()> {
    if project_root.starts_with(workspace_root.as_path()) {
        return Ok(());
    }

    Err(WorkspaceError::ProjectOutsideWorkspace {
        workspace_root: workspace_root.as_path().to_path_buf(),
        project_root: project_root.to_path_buf(),
    })
}

fn canonicalize_directory(field: &'static str, path: PathBuf) -> WorkspaceResult<PathBuf> {
    if path.as_os_str().is_empty() {
        return Err(WorkspaceError::EmptyPath { field });
    }

    if !path.is_absolute() {
        return Err(WorkspaceError::PathMustBeAbsolute { field, path });
    }

    if !path.exists() {
        return Err(WorkspaceError::PathDoesNotExist { field, path });
    }

    if !path.is_dir() {
        return Err(WorkspaceError::PathIsNotDirectory { field, path });
    }

    fs::canonicalize(&path).map_err(|_| WorkspaceError::CanonicalizationFailed { field, path })
}

fn stable_relative_path(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
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

#[cfg(test)]
mod tests {
    use super::{ProjectRoot, WorkspaceError, WorkspacePath, WorkspaceRoot};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

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
    fn accepts_project_root_inside_workspace() {
        let workspace_dir = unique_temp_dir("workspace_core_inside");
        let project_dir = workspace_dir.join("user").join("projects").join("demo");

        fs::create_dir_all(&project_dir).expect("create project dir");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let project_root =
            ProjectRoot::new(&workspace_root, project_dir.clone()).expect("project root");

        assert!(project_root.as_path().starts_with(workspace_root.as_path()));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn rejects_project_root_outside_workspace() {
        let workspace_dir = unique_temp_dir("workspace_core_root");
        let project_dir = unique_temp_dir("workspace_core_outside");

        fs::create_dir_all(&workspace_dir).expect("create workspace dir");
        fs::create_dir_all(&project_dir).expect("create outside project dir");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let error = ProjectRoot::new(&workspace_root, project_dir.clone()).expect_err("outside");

        assert!(matches!(
            error,
            WorkspaceError::ProjectOutsideWorkspace { .. }
        ));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
        fs::remove_dir_all(project_dir).expect("cleanup outside dir");
    }

    #[test]
    fn rejects_parent_segment_in_relative_path() {
        let error = WorkspacePath::new("../escape").expect_err("invalid path");
        assert!(matches!(error, WorkspaceError::InvalidRelativePath { .. }));
    }

    #[test]
    fn normalizes_conservative_relative_path() {
        let relative = WorkspacePath::new("projects/demo/config").expect("relative path");
        assert_eq!(relative.to_string(), "projects/demo/config");
    }

    #[test]
    fn builds_workspace_path_from_relative_path_value() {
        let relative =
            WorkspacePath::from_path(PathBuf::from("system").join("spec").join("example.json"))
                .expect("relative path");

        assert_eq!(relative.to_string(), "system/spec/example.json");
    }

    #[test]
    fn derives_workspace_relative_path_for_absolute_path_inside_workspace() {
        let workspace_dir = unique_temp_dir("workspace_core_relative");
        let file_path = workspace_dir
            .join("system")
            .join("spec")
            .join("example.json");

        fs::create_dir_all(file_path.parent().expect("parent dir")).expect("create spec dir");
        fs::write(&file_path, "{}").expect("write file");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let relative = workspace_root
            .relative_path_for(&file_path)
            .expect("relative path");

        assert_eq!(relative.to_string(), "system/spec/example.json");

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
    }

    #[test]
    fn rejects_workspace_relative_path_outside_workspace() {
        let workspace_dir = unique_temp_dir("workspace_core_relative_root");
        let outside_dir = unique_temp_dir("workspace_core_relative_outside");
        let outside_file = outside_dir.join("system").join("spec").join("example.json");

        fs::create_dir_all(&workspace_dir).expect("create workspace dir");
        fs::create_dir_all(outside_file.parent().expect("parent dir")).expect("create spec dir");
        fs::write(&outside_file, "{}").expect("write file");

        let workspace_root = WorkspaceRoot::new(workspace_dir.clone()).expect("workspace root");
        let error = workspace_root
            .relative_path_for(&outside_file)
            .expect_err("outside workspace error");

        assert!(matches!(error, WorkspaceError::PathOutsideWorkspace { .. }));

        fs::remove_dir_all(workspace_dir).expect("cleanup workspace dir");
        fs::remove_dir_all(outside_dir).expect("cleanup outside dir");
    }
}
