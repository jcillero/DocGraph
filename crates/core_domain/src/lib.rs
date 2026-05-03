//! Core domain contracts shared by the first Rust sandbox runtime crates.
//!
//! The types in this crate stay intentionally small and are limited to
//! cross-crate primitives that already have a clear inherited justification.

mod contracts;
mod errors;
mod file_ref;
mod ids;

pub use contracts::{
    AnalysisProfile, EdgeMetric, GraphAnalysis, NodeMetric, OperationalMetadata, QuadEvidence,
    QuadGeneration, QuadInstance, QuadLifecycleState, QuadRelation, QuadSet, QuadTextRange,
    RelationMetadata, RelationType, SemanticGraph, SemanticMetadata, SemanticQuad, SemanticValue,
    SourceKind, StoredObject, StoredObjectKind, StoredObjectLifecycleState, StoredObjectMetadata,
    TechnicalMetadata, UsageKind, UsageRef,
};
pub use errors::{ErrorDomain, PortableAppError, PortableAppResult};
pub use file_ref::{sha256_hex, FileRef};
pub use ids::{ManifestId, ManifestRefId, ProjectId};
