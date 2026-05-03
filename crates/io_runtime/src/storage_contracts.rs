use core_domain::FileRef;
use serde::{Deserialize, Serialize};

/// Physical blob metadata contract only. It does not imply ownership, project
/// exposure, execution authority, or semantic approval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlobRecord {
    pub file_ref: FileRef,
    pub hash_algorithm: BlobRecordHashAlgorithm,
    pub byte_size: u64,
    pub media_type: String,
    pub original_name_optional: Option<String>,
    pub created_at: String,
    pub integrity: BlobIntegrity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlobRecordHashAlgorithm {
    Sha256,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlobIntegrity {
    pub status: BlobIntegrityStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_at: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlobIntegrityStatus {
    Verified,
    Unverified,
    Unknown,
}

#[cfg(test)]
mod tests {
    use core_domain::FileRef;

    use super::{BlobIntegrity, BlobIntegrityStatus, BlobRecord, BlobRecordHashAlgorithm};

    #[test]
    fn blob_record_json_roundtrip() {
        let blob_record = BlobRecord {
            file_ref: FileRef::from_bytes(b"blob"),
            hash_algorithm: BlobRecordHashAlgorithm::Sha256,
            byte_size: 4,
            media_type: "text/plain".to_owned(),
            original_name_optional: Some("blob.txt".to_owned()),
            created_at: "2026-05-01T00:00:00Z".to_owned(),
            integrity: BlobIntegrity {
                status: BlobIntegrityStatus::Verified,
                checked_at: Some("2026-05-01T00:00:30Z".to_owned()),
            },
        };

        let roundtrip: BlobRecord = serde_json::from_str(
            &serde_json::to_string(&blob_record).expect("serialize blob record"),
        )
        .expect("deserialize blob record");

        assert_eq!(roundtrip, blob_record);
    }
}
