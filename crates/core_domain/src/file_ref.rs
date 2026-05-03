use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{ErrorDomain, PortableAppError, PortableAppResult};

/// Stable content-addressed file reference for governed storage contracts.
///
/// This is a contract/preparation type only. It does not execute, does not
/// imply project exposure, and does not open semantic or RDF runtime behavior.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FileRef(String);

impl FileRef {
    /// Build a validated `sha256_<hex>` file reference.
    pub fn new(value: impl Into<String>) -> PortableAppResult<Self> {
        let value = value.into();
        if !is_valid_file_ref(&value) {
            return Err(PortableAppError::new(
                ErrorDomain::CoreDomain,
                "invalid_file_ref",
                "file_ref must match sha256_<64 lowercase hex characters>",
            ));
        }
        Ok(Self(value))
    }

    /// Derive a stable file reference from content bytes only.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(format!("sha256_{}", sha256_hex(bytes)))
    }

    /// Return the content-addressed value as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Return the lowercase hexadecimal SHA-256 digest for arbitrary bytes.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn is_valid_file_ref(value: &str) -> bool {
    const PREFIX: &str = "sha256_";
    if !value.starts_with(PREFIX) {
        return false;
    }

    let hash = &value[PREFIX.len()..];
    hash.len() == 64 && hash.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::FileRef;

    #[test]
    fn same_content_produces_same_file_ref() {
        let left = FileRef::from_bytes(b"alpha");
        let right = FileRef::from_bytes(b"alpha");

        assert_eq!(left, right);
    }

    #[test]
    fn different_content_produces_different_file_ref() {
        let left = FileRef::from_bytes(b"alpha");
        let right = FileRef::from_bytes(b"beta");

        assert_ne!(left, right);
    }

    #[test]
    fn file_ref_is_independent_from_filename_or_path() {
        let first_path_bytes = b"shared contents";
        let second_path_bytes = b"shared contents";

        assert_eq!(
            FileRef::from_bytes(first_path_bytes),
            FileRef::from_bytes(second_path_bytes)
        );
    }
}
