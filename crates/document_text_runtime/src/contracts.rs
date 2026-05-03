use serde::{Deserialize, Serialize};

/// Deterministic derivative-state contract only. It does not execute, does
/// not imply project exposure, and does not open semantic or RDF runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivationManifest {
    pub object_ref: String,
    pub derivation_state: DerivationState,
    pub expected_derivatives: Vec<DerivativeKind>,
    pub available_derivatives: Vec<DerivativeKind>,
    pub failed_derivatives: Vec<DerivativeKind>,
    pub not_applicable_derivatives: Vec<DerivativeKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DerivationState {
    DerivationPending,
    DerivedPartial,
    DerivedComplete,
    DerivationFailed,
    DerivationNotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivativeKind {
    Text,
    Pages,
    Chunks,
    Metadata,
    Previews,
}

#[cfg(test)]
mod tests {
    use super::{DerivationManifest, DerivationState, DerivativeKind};

    #[test]
    fn derivation_manifest_json_roundtrip() {
        let manifest = DerivationManifest {
            object_ref: "obj_demo".to_owned(),
            derivation_state: DerivationState::DerivedPartial,
            expected_derivatives: vec![
                DerivativeKind::Text,
                DerivativeKind::Pages,
                DerivativeKind::Chunks,
            ],
            available_derivatives: vec![DerivativeKind::Text],
            failed_derivatives: vec![DerivativeKind::Pages],
            not_applicable_derivatives: vec![DerivativeKind::Previews],
        };

        let roundtrip: DerivationManifest = serde_json::from_str(
            &serde_json::to_string(&manifest).expect("serialize derivation manifest"),
        )
        .expect("deserialize derivation manifest");

        assert_eq!(roundtrip, manifest);
    }
}
