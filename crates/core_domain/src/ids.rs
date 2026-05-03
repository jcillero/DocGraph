use std::fmt;

use crate::{ErrorDomain, PortableAppError, PortableAppResult};

/// Stable project identifier used by project-facing Rust contracts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectId(String);

impl ProjectId {
    /// Build a validated project identifier.
    pub fn new(value: impl AsRef<str>) -> PortableAppResult<Self> {
        validate_identifier("project_id", value.as_ref()).map(Self)
    }

    /// Return the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Stable manifest identifier used by manifest-facing Rust contracts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ManifestId(String);

impl ManifestId {
    /// Build a validated manifest identifier.
    pub fn new(value: impl AsRef<str>) -> PortableAppResult<Self> {
        validate_identifier("manifest_id", value.as_ref()).map(Self)
    }

    /// Return the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ManifestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Stable manifest reference identifier for ref-driven project visibility.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ManifestRefId(String);

impl ManifestRefId {
    /// Build a validated manifest reference identifier.
    pub fn new(value: impl AsRef<str>) -> PortableAppResult<Self> {
        validate_identifier("target_ref", value.as_ref()).map(Self)
    }

    /// Return the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ManifestRefId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn validate_identifier(field_name: &'static str, raw: &str) -> PortableAppResult<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(PortableAppError::new(
            ErrorDomain::CoreDomain,
            "empty_identifier",
            format!("{field_name} must not be empty"),
        ));
    }

    if trimmed.contains('/') || trimmed.contains('\\') {
        return Err(PortableAppError::new(
            ErrorDomain::CoreDomain,
            "invalid_identifier",
            format!("{field_name} must not contain path separators"),
        ));
    }

    Ok(trimmed.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{ManifestId, ManifestRefId, ProjectId};

    #[test]
    fn rejects_empty_identifiers() {
        assert!(ProjectId::new("   ").is_err());
        assert!(ManifestId::new("").is_err());
        assert!(ManifestRefId::new(" ").is_err());
    }

    #[test]
    fn rejects_path_like_identifiers() {
        assert!(ProjectId::new("project/one").is_err());
        assert!(ManifestRefId::new("tree\\node").is_err());
    }
}
