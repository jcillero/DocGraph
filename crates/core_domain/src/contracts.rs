use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::FileRef;

/// Logical storage contract only. Stored objects do not execute and do not
/// imply project exposure, filesystem mutation, or runtime authority.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoredObject {
    pub object_ref: String,
    pub object_kind: StoredObjectKind,
    pub content_ref: Option<FileRef>,
    pub metadata: StoredObjectMetadata,
    pub lifecycle_state: StoredObjectLifecycleState,
    pub derivation_capabilities: BTreeMap<String, Value>,
    pub quad_flags: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StoredObjectKind {
    SourceFile,
    Document,
    ChatSession,
    ChatAttachment,
    Artifact,
    ToolOutput,
    Dataset,
    Code,
    Media,
    SemanticQuadset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StoredObjectLifecycleState {
    Created,
    Ingested,
    Active,
    Archived,
    Deprecated,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoredObjectMetadata {
    pub semantic: SemanticMetadata,
    pub technical: TechnicalMetadata,
    pub operational: OperationalMetadata,
}

/// Proposal/review metadata only. It does not approve knowledge or generate quads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SemanticMetadata(pub BTreeMap<String, Value>);

/// Technical metadata only. It does not imply project exposure or authority.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TechnicalMetadata(pub BTreeMap<String, Value>);

/// Operational metadata only. It must stay free of execution authority.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperationalMetadata(pub BTreeMap<String, Value>);

/// Usage-reference contract only. It declares logical usage and does not execute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageRef {
    pub usage_ref: String,
    pub object_ref: String,
    pub usage_kind: UsageKind,
    pub owner_ref: String,
    pub declared_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageKind {
    Project,
    Document,
    Chat,
    Knowledge,
    ToolOutput,
    Artifact,
    Semantic,
}

/// Semantic identity only. It does not imply approval, RDF projection, or execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticQuad {
    pub quad_id: String,
    pub subject: String,
    pub predicate: String,
    pub object: SemanticValue,
    pub graph: SemanticGraph,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticGraph {
    DomainKnowledge,
    DocumentGovernance,
    AiGovernance,
    SystemGovernance,
}

/// Quad-instance contract only. Generation metadata is trace, not authority.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuadInstance {
    pub quad_instance_id: String,
    pub quad_id: String,
    pub source_kind: SourceKind,
    pub source_ref: String,
    pub evidence: Vec<QuadEvidence>,
    pub generation: QuadGeneration,
    pub lifecycle_state: QuadLifecycleState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    File,
    Chat,
    Artifact,
    ToolOutput,
    Metadata,
}

/// Evidence anchors reference governed sources only. They do not duplicate raw text authority.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuadEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ref: Option<FileRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_range: Option<QuadTextRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_snapshot_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_run_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuadTextRange {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuadGeneration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuadLifecycleState {
    Proposed,
    UnderReview,
    Approved,
    Rejected,
    Stale,
    Superseded,
    Orphaned,
}

/// Contextual quad grouping only. It is descriptive and not authoritative knowledge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuadSet {
    pub quadset_id: String,
    pub source_kind: SourceKind,
    pub source_ref: String,
    pub generation_context: BTreeMap<String, Value>,
    pub quad_instances: Vec<String>,
}

/// Explicit graph-edge contract only. It does not open graph runtime or execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuadRelation {
    pub relation_id: String,
    pub source_quad_instance_id: String,
    pub target_quad_instance_id: String,
    pub relation_type: RelationType,
    pub lifecycle_state: QuadLifecycleState,
    pub evidence: Vec<QuadEvidence>,
    pub metadata: RelationMetadata,
    pub trace_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    Supports,
    Contradicts,
    Refines,
    DependsOn,
    DerivedFrom,
    EquivalentTo,
    BroaderThan,
    NarrowerThan,
    UsesAsEvidence,
    Supersedes,
}

/// Relation metadata is descriptive only and must stay secret-free.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelationMetadata(pub BTreeMap<String, Value>);

/// Descriptive graph-analysis profile only. It does not implement formulas or algorithms.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphAnalysis {
    pub profile_id: AnalysisProfile,
    pub objective: String,
    pub node_metrics: Vec<NodeMetric>,
    pub edge_metrics: Vec<EdgeMetric>,
    pub filters: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnalysisProfile {
    RiskPath,
    DependencyPath,
    LowConfidencePath,
    ImpactPath,
    StalePath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeMetric {
    Confidence,
    Criticality,
    Risk,
    Freshness,
    ReviewLevel,
    SourceQuality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeMetric {
    DependencyStrength,
    RiskPropagation,
    SemanticWeight,
    EvidenceStrength,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::{json, Value};

    use super::{
        AnalysisProfile, EdgeMetric, GraphAnalysis, NodeMetric, OperationalMetadata, QuadEvidence,
        QuadGeneration, QuadInstance, QuadLifecycleState, QuadRelation, QuadSet, QuadTextRange,
        RelationMetadata, RelationType, SemanticGraph, SemanticMetadata, SemanticQuad,
        SemanticValue, SourceKind, StoredObject, StoredObjectKind, StoredObjectLifecycleState,
        StoredObjectMetadata, TechnicalMetadata, UsageKind, UsageRef,
    };
    use crate::FileRef;

    #[test]
    fn stored_object_json_roundtrip() {
        let mut semantic = BTreeMap::new();
        semantic.insert("topic".to_owned(), json!("alpha"));
        let mut technical = BTreeMap::new();
        technical.insert("media_type".to_owned(), json!("text/markdown"));
        let mut operational = BTreeMap::new();
        operational.insert("owner".to_owned(), json!("chat/demo"));
        let mut derivation_capabilities = BTreeMap::new();
        derivation_capabilities.insert("text".to_owned(), json!(true));
        let mut quad_flags = BTreeMap::new();
        quad_flags.insert("eligible".to_owned(), json!(false));

        let stored_object = StoredObject {
            object_ref: "obj_demo".to_owned(),
            object_kind: StoredObjectKind::Document,
            content_ref: Some(FileRef::from_bytes(b"doc")),
            metadata: StoredObjectMetadata {
                semantic: SemanticMetadata(semantic),
                technical: TechnicalMetadata(technical),
                operational: OperationalMetadata(operational),
            },
            lifecycle_state: StoredObjectLifecycleState::Active,
            derivation_capabilities,
            quad_flags,
        };

        let roundtrip: StoredObject = serde_json::from_str(
            &serde_json::to_string(&stored_object).expect("serialize stored object"),
        )
        .expect("deserialize stored object");

        assert_eq!(roundtrip, stored_object);
    }

    #[test]
    fn usage_ref_json_roundtrip() {
        let usage_ref = UsageRef {
            usage_ref: "usage_demo".to_owned(),
            object_ref: "obj_demo".to_owned(),
            usage_kind: UsageKind::Knowledge,
            owner_ref: "owner/demo".to_owned(),
            declared_at: "2026-05-01T00:00:00Z".to_owned(),
        };

        let roundtrip: UsageRef =
            serde_json::from_str(&serde_json::to_string(&usage_ref).expect("serialize usage ref"))
                .expect("deserialize usage ref");

        assert_eq!(roundtrip, usage_ref);
    }

    #[test]
    fn semantic_quad_json_roundtrip() {
        let semantic_quad = SemanticQuad {
            quad_id: "quad_demo".to_owned(),
            subject: "doc:alpha".to_owned(),
            predicate: "mentions".to_owned(),
            object: SemanticValue::String("topic:beta".to_owned()),
            graph: SemanticGraph::DomainKnowledge,
        };

        let serialized = serde_json::to_value(&semantic_quad).expect("serialize semantic quad");
        assert_eq!(serialized["graph"], Value::String("domain_knowledge".to_owned()));

        let roundtrip: SemanticQuad =
            serde_json::from_value(serialized).expect("deserialize semantic quad");

        assert_eq!(roundtrip, semantic_quad);
    }

    #[test]
    fn quad_instance_json_roundtrip() {
        let quad_instance = QuadInstance {
            quad_instance_id: "inst_demo".to_owned(),
            quad_id: "quad_demo".to_owned(),
            source_kind: SourceKind::File,
            source_ref: "obj_demo".to_owned(),
            evidence: vec![QuadEvidence {
                file_ref: Some(FileRef::from_bytes(b"doc")),
                object_ref: Some("obj_demo".to_owned()),
                chunk_id: Some("chunk_0001".to_owned()),
                chunk_hash: Some("chunk_hash".to_owned()),
                text_range: Some(QuadTextRange { start: 0, end: 12 }),
                metadata_snapshot_hash: Some("meta_hash".to_owned()),
                message_ref: None,
                artifact_ref: None,
                tool_run_ref: None,
                trace_ref: Some("trace_demo".to_owned()),
            }],
            generation: QuadGeneration {
                prompt_ref: Some("prompt_demo".to_owned()),
                model_ref: Some("model_demo".to_owned()),
                trace_ref: Some("trace_demo".to_owned()),
                timestamp: Some("2026-05-01T00:00:00Z".to_owned()),
            },
            lifecycle_state: QuadLifecycleState::Proposed,
        };

        let roundtrip: QuadInstance = serde_json::from_str(
            &serde_json::to_string(&quad_instance).expect("serialize quad instance"),
        )
        .expect("deserialize quad instance");

        assert_eq!(roundtrip, quad_instance);
    }

    #[test]
    fn quadset_json_roundtrip() {
        let mut generation_context = BTreeMap::new();
        generation_context.insert("prompt_ref".to_owned(), json!("prompt_demo"));
        let quad_set = QuadSet {
            quadset_id: "quadset_demo".to_owned(),
            source_kind: SourceKind::Chat,
            source_ref: "chat/demo".to_owned(),
            generation_context,
            quad_instances: vec!["inst_demo".to_owned()],
        };

        let roundtrip: QuadSet =
            serde_json::from_str(&serde_json::to_string(&quad_set).expect("serialize quadset"))
                .expect("deserialize quadset");

        assert_eq!(roundtrip, quad_set);
    }

    #[test]
    fn quad_relation_json_roundtrip() {
        let mut metadata = BTreeMap::new();
        metadata.insert("reason".to_owned(), json!("shared evidence"));
        let quad_relation = QuadRelation {
            relation_id: "rel_demo".to_owned(),
            source_quad_instance_id: "inst_a".to_owned(),
            target_quad_instance_id: "inst_b".to_owned(),
            relation_type: RelationType::Supports,
            lifecycle_state: QuadLifecycleState::Approved,
            evidence: vec![QuadEvidence {
                file_ref: None,
                object_ref: Some("obj_demo".to_owned()),
                chunk_id: None,
                chunk_hash: None,
                text_range: None,
                metadata_snapshot_hash: None,
                message_ref: None,
                artifact_ref: Some("artifact/demo".to_owned()),
                tool_run_ref: None,
                trace_ref: Some("trace_demo".to_owned()),
            }],
            metadata: RelationMetadata(metadata),
            trace_ref: "trace_demo".to_owned(),
        };

        let roundtrip: QuadRelation = serde_json::from_str(
            &serde_json::to_string(&quad_relation).expect("serialize quad relation"),
        )
        .expect("deserialize quad relation");

        assert_eq!(roundtrip, quad_relation);
    }

    #[test]
    fn graph_analysis_json_roundtrip() {
        let mut filters = BTreeMap::new();
        filters.insert("owner_ref".to_owned(), json!("owner/demo"));
        let graph_analysis = GraphAnalysis {
            profile_id: AnalysisProfile::RiskPath,
            objective: "surface risky links".to_owned(),
            node_metrics: vec![NodeMetric::Confidence, NodeMetric::Risk],
            edge_metrics: vec![EdgeMetric::DependencyStrength, EdgeMetric::EvidenceStrength],
            filters,
        };

        let serialized = serde_json::to_value(&graph_analysis).expect("serialize graph analysis");
        assert_eq!(serialized["profile_id"], Value::String("RISK_PATH".to_owned()));

        let roundtrip: GraphAnalysis =
            serde_json::from_value(serialized).expect("deserialize graph analysis");

        assert_eq!(roundtrip, graph_analysis);
    }
}
