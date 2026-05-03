//! LLM contracts, policy, and mode selection boundaries.
//!
//! Provider-specific behavior belongs in adapter crates, not here.

use std::env;
use std::fmt;

/// Governed desired and effective LLM modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmMode {
    Off,
    Local,
    Cloud,
}

/// Governed user interaction mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionMode {
    Guided,
    Assisted,
}

/// Deterministic capability input for LLM mode resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityInput {
    pub desired_llm_mode: LlmMode,
    pub provider_configured: bool,
    pub credential_ref_present: bool,
    pub local_engine_available: bool,
    pub local_model_available: bool,
    pub policy_allows_local: bool,
    pub policy_allows_cloud: bool,
}

/// Deterministic governed capability state derived from explicit inputs only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmCapabilityState {
    pub desired_llm_mode: LlmMode,
    pub effective_llm_mode: LlmMode,
    pub interaction_mode: InteractionMode,
    pub llm_available: bool,
    pub provider_configured: bool,
    pub credential_ref_present: bool,
    pub local_engine_available: bool,
    pub local_model_available: bool,
    pub policy_allows_local: bool,
    pub policy_allows_cloud: bool,
    pub reason_codes: Vec<String>,
}

/// Exact reason code used when assisted LLM processing lacks explicit consent.
pub const LLM_PROCESSING_CONSENT_MISSING: &str = "llm_processing_consent_missing";

/// Resolve governed capability state from explicit deterministic inputs only.
pub fn resolve_capability_state(input: CapabilityInput) -> LlmCapabilityState {
    let mut state = LlmCapabilityState {
        desired_llm_mode: input.desired_llm_mode,
        effective_llm_mode: LlmMode::Off,
        interaction_mode: InteractionMode::Guided,
        llm_available: false,
        provider_configured: input.provider_configured,
        credential_ref_present: input.credential_ref_present,
        local_engine_available: input.local_engine_available,
        local_model_available: input.local_model_available,
        policy_allows_local: input.policy_allows_local,
        policy_allows_cloud: input.policy_allows_cloud,
        reason_codes: Vec::new(),
    };

    match input.desired_llm_mode {
        LlmMode::Off => {
            state
                .reason_codes
                .push(String::from("llm_off_by_preference"));
        }
        LlmMode::Local => {
            if !input.local_engine_available {
                state
                    .reason_codes
                    .push(String::from("local_engine_missing"));
            }
            if !input.local_model_available {
                state
                    .reason_codes
                    .push(String::from("local_model_missing"));
            }
            if !input.policy_allows_local {
                state
                    .reason_codes
                    .push(String::from("local_policy_blocked"));
            }

            if state.reason_codes.is_empty() {
                state.effective_llm_mode = LlmMode::Local;
                state.interaction_mode = InteractionMode::Assisted;
                state.llm_available = true;
            } else {
                state
                    .reason_codes
                    .push(String::from("assisted_mode_unavailable"));
            }
        }
        LlmMode::Cloud => {
            if !input.provider_configured {
                state
                    .reason_codes
                    .push(String::from("cloud_provider_missing"));
            }
            if !input.credential_ref_present {
                state
                    .reason_codes
                    .push(String::from("cloud_credential_ref_missing"));
            }
            if !input.policy_allows_cloud {
                state
                    .reason_codes
                    .push(String::from("cloud_policy_blocked"));
            }

            if state.reason_codes.is_empty() {
                state.effective_llm_mode = LlmMode::Cloud;
                state.interaction_mode = InteractionMode::Assisted;
                state.llm_available = true;
            } else {
                state
                    .reason_codes
                    .push(String::from("assisted_mode_unavailable"));
            }
        }
    }

    state
}

/// Deterministic governed input for preparing an assisted-chat request envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistedChatInput {
    pub user_text: String,
    pub capability_state: LlmCapabilityState,
    pub consent_allows_llm_processing: bool,
    pub context_refs: Vec<String>,
    pub prompt_ref: Option<String>,
    pub tool_surface_summary: Option<EffectiveToolSurfaceSummary>,
}

/// Deterministic governed assisted-chat envelope without provider execution.
///
/// Invariants:
/// - injected summary is awareness only
/// - injected summary is not permission
/// - injected summary is not execution authority
/// - injected summary is precomputed by governed policy outside this function
/// - envelope does not inspect or modify tool awareness
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistedChatEnvelope {
    pub interaction_mode: InteractionMode,
    pub effective_llm_mode: LlmMode,
    pub can_attempt_assisted_response: bool,
    pub user_text: String,
    pub context_refs: Vec<String>,
    pub prompt_ref: Option<String>,
    pub tool_surface_summary: Option<EffectiveToolSurfaceSummary>,
    pub reason_codes: Vec<String>,
}

/// Inert LLM interaction trace for governed observability only.
///
/// Invariants:
/// - trace is observability metadata only
/// - trace is not execution
/// - trace is not permission
/// - trace is not persistence
/// - trace must not contain secrets
/// - trace does not expose raw catalogs
/// - trace does not call providers or tools
/// - trace does not bypass ActionResolution
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmInteractionTrace {
    pub effective_llm_mode: LlmMode,
    pub interaction_mode: InteractionMode,
    pub llm_available: bool,
    pub consent_allows_llm_processing: bool,
    pub context_refs: Vec<String>,
    pub prompt_ref: Option<String>,
    pub tool_surface_summary_present: bool,
    pub tool_use_proposal_present: bool,
    pub reason_codes: Vec<String>,
}

/// Prepare a deterministic assisted-chat envelope without invoking any provider.
pub fn prepare_assisted_chat_envelope(input: AssistedChatInput) -> AssistedChatEnvelope {
    let AssistedChatInput {
        user_text,
        capability_state,
        consent_allows_llm_processing,
        context_refs,
        prompt_ref,
        tool_surface_summary,
    } = input;

    let mut reason_codes = capability_state.reason_codes.clone();

    if !consent_allows_llm_processing
        && !reason_codes.iter().any(|code| code == LLM_PROCESSING_CONSENT_MISSING)
    {
        reason_codes.push(String::from(LLM_PROCESSING_CONSENT_MISSING));
    }

    let can_attempt_assisted_response = capability_state.interaction_mode == InteractionMode::Assisted
        && capability_state.llm_available
        && consent_allows_llm_processing;

    AssistedChatEnvelope {
        interaction_mode: capability_state.interaction_mode,
        effective_llm_mode: capability_state.effective_llm_mode,
        can_attempt_assisted_response,
        user_text,
        context_refs,
        prompt_ref,
        tool_surface_summary,
        reason_codes,
    }
}

/// Build inert LLM interaction observability metadata from a prepared envelope.
pub fn build_llm_interaction_trace(
    envelope: &AssistedChatEnvelope,
    consent_allows_llm_processing: bool,
    tool_use_proposal_present: bool,
) -> LlmInteractionTrace {
    LlmInteractionTrace {
        effective_llm_mode: envelope.effective_llm_mode,
        interaction_mode: envelope.interaction_mode,
        llm_available: envelope.can_attempt_assisted_response,
        consent_allows_llm_processing,
        context_refs: envelope.context_refs.clone(),
        prompt_ref: envelope.prompt_ref.clone(),
        tool_surface_summary_present: envelope.tool_surface_summary.is_some(),
        tool_use_proposal_present,
        reason_codes: envelope.reason_codes.clone(),
    }
}

/// Readonly availability marker for bounded LLM-facing tool summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolAvailability {
    Available,
    Disabled,
    Missing,
}

/// Readonly, non-authoritative item for LLM tool awareness.
///
/// Invariants:
/// - summary is not authoritative
/// - summary does not grant execution permission
/// - visibility != permission
/// - summary != raw catalog
/// - LLM consumes summary, not tool definitions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveToolSummaryItem {
    pub tool_id: String,
    pub tool_class: String,
    pub availability: ToolAvailability,
    pub reason_codes: Vec<String>,
}

/// Bounded readonly summary for LLM tool awareness only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveToolSurfaceSummary {
    pub items: Vec<EffectiveToolSummaryItem>,
}

/// Build a passthrough effective tool surface summary without inference or enrichment.
pub fn build_effective_tool_surface_summary(
    items: Vec<EffectiveToolSummaryItem>,
) -> EffectiveToolSurfaceSummary {
    EffectiveToolSurfaceSummary { items }
}

/// Readonly status for a proposal-only LLM tool-use intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolUseProposalStatus {
    Proposed,
    Blocked,
}

/// Non-executing LLM-facing tool-use proposal.
///
/// Invariants:
/// - proposal is not execution
/// - proposal is not permission
/// - proposal does not create ActionRequest
/// - proposal does not bypass ActionResolution
/// - proposal does not call tool_runtime
/// - proposal does not imply tool availability
/// - LLM may propose; system governs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolUseProposal {
    pub tool_id: String,
    pub tool_class: String,
    pub status: ToolUseProposalStatus,
    pub rationale: String,
    pub reason_codes: Vec<String>,
}

/// Build a proposed tool-use intent without execution, validation, or permission logic.
pub fn propose_tool_use(
    tool_id: String,
    tool_class: String,
    rationale: String,
    reason_codes: Vec<String>,
) -> ToolUseProposal {
    ToolUseProposal {
        tool_id,
        tool_class,
        status: ToolUseProposalStatus::Proposed,
        rationale,
        reason_codes,
    }
}

/// Build a blocked tool-use proposal without execution or action creation.
pub fn blocked_tool_use_proposal(
    tool_id: String,
    tool_class: String,
    rationale: String,
    reason_codes: Vec<String>,
) -> ToolUseProposal {
    ToolUseProposal {
        tool_id,
        tool_class,
        status: ToolUseProposalStatus::Blocked,
        rationale,
        reason_codes,
    }
}

/// Shared provider config that carries only the logical credential reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudProviderConfig {
    credential_ref: CloudCredentialRef,
}

impl CloudProviderConfig {
    /// Build a provider config from a logical credential reference.
    pub fn new(credential_ref: CloudCredentialRef) -> Self {
        Self { credential_ref }
    }

    /// Return the logical credential reference.
    pub fn credential_ref(&self) -> &CloudCredentialRef {
        &self.credential_ref
    }
}

/// Stable logical reference to a cloud credential.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudCredentialRef {
    name: String,
}

impl CloudCredentialRef {
    /// Build a logical credential reference.
    pub fn new(name: impl Into<String>) -> Result<Self, LlmCredentialError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(LlmCredentialError::InvalidCredentialRef {
                reason: "credential reference name must not be empty",
            });
        }

        Ok(Self { name })
    }

    /// Return the stable logical credential name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the deterministic environment variable name used for local lookup.
    pub fn environment_variable_name(&self) -> String {
        let canonical = self
            .name
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() {
                    ch.to_ascii_uppercase()
                } else {
                    '_'
                }
            })
            .collect::<String>();

        format!("PORTABLE_APP_LLM_CREDENTIAL_{canonical}")
    }
}

/// Small source-kind tag for cloud credential resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudCredentialSourceKind {
    Environment,
}

/// Minimal readiness state for a cloud provider configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudProviderReadiness {
    credential_ref: CloudCredentialRef,
    source_kind: CloudCredentialSourceKind,
    ready: bool,
}

impl CloudProviderReadiness {
    /// Build a minimal readiness state from a resolved local credential.
    pub fn new(
        credential_ref: CloudCredentialRef,
        source_kind: CloudCredentialSourceKind,
        ready: bool,
    ) -> Self {
        Self {
            credential_ref,
            source_kind,
            ready,
        }
    }

    /// Return the logical credential reference used to evaluate readiness.
    pub fn credential_ref(&self) -> &CloudCredentialRef {
        &self.credential_ref
    }

    /// Return the local source kind used during readiness evaluation.
    pub fn source_kind(&self) -> CloudCredentialSourceKind {
        self.source_kind
    }

    /// Return whether the provider is ready for later cloud usage.
    pub fn is_ready(&self) -> bool {
        self.ready
    }
}

/// Resolved cloud credential without exposing the secret through debug output.
#[derive(Clone, PartialEq, Eq)]
pub struct CloudCredentialResolution {
    credential_ref: CloudCredentialRef,
    source_kind: CloudCredentialSourceKind,
    secret_value: String,
}

impl CloudCredentialResolution {
    /// Build a resolved credential from its logical ref, source, and secret value.
    pub fn new(
        credential_ref: CloudCredentialRef,
        source_kind: CloudCredentialSourceKind,
        secret_value: impl Into<String>,
    ) -> Self {
        Self {
            credential_ref,
            source_kind,
            secret_value: secret_value.into(),
        }
    }

    /// Return the logical credential reference.
    pub fn credential_ref(&self) -> &CloudCredentialRef {
        &self.credential_ref
    }

    /// Return the source kind used for local credential resolution.
    pub fn source_kind(&self) -> CloudCredentialSourceKind {
        self.source_kind
    }

    /// Return the secret value for adapter-side consumption.
    pub fn secret_value(&self) -> &str {
        &self.secret_value
    }
}

impl fmt::Debug for CloudCredentialResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CloudCredentialResolution")
            .field("credential_ref", &self.credential_ref)
            .field("source_kind", &self.source_kind)
            .field("secret_value", &"<redacted>")
            .finish()
    }
}

/// Minimal credential resolution error for shared LLM contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmCredentialError {
    InvalidCredentialRef {
        reason: &'static str,
    },
    MissingCredential {
        credential_name: String,
        environment_variable: String,
    },
}

impl fmt::Display for LlmCredentialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCredentialRef { reason } => {
                write!(f, "[llm_core:invalid_credential_ref] {reason}")
            }
            Self::MissingCredential {
                credential_name,
                environment_variable,
            } => write!(
                f,
                "[llm_core:missing_credential] missing local credential '{}' in env '{}'",
                credential_name, environment_variable
            ),
        }
    }
}

impl std::error::Error for LlmCredentialError {}

/// Resolve a cloud credential from the local secure source layer.
///
/// The shared config carries only a logical credential reference. The secret is
/// resolved locally and is never exposed through display/debug output.
pub fn resolve_cloud_credential(
    provider_config: &CloudProviderConfig,
) -> Result<CloudCredentialResolution, LlmCredentialError> {
    let credential_ref = provider_config.credential_ref().clone();
    let environment_variable = credential_ref.environment_variable_name();
    let secret_value =
        env::var(&environment_variable).map_err(|_| LlmCredentialError::MissingCredential {
            credential_name: credential_ref.name().to_owned(),
            environment_variable,
        })?;

    Ok(CloudCredentialResolution::new(
        credential_ref,
        CloudCredentialSourceKind::Environment,
        secret_value,
    ))
}

/// Evaluate minimal cloud-provider readiness from config plus locally resolved credential.
///
/// This function performs only local deterministic checks. It does not call the
/// network or validate provider-side state.
pub fn evaluate_cloud_provider_readiness(
    provider_config: &CloudProviderConfig,
) -> Result<CloudProviderReadiness, LlmCredentialError> {
    let resolution = resolve_cloud_credential(provider_config)?;

    Ok(CloudProviderReadiness::new(
        resolution.credential_ref().clone(),
        resolution.source_kind(),
        true,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        blocked_tool_use_proposal, build_effective_tool_surface_summary,
        build_llm_interaction_trace, evaluate_cloud_provider_readiness,
        prepare_assisted_chat_envelope, propose_tool_use, resolve_capability_state,
        resolve_cloud_credential, AssistedChatEnvelope, AssistedChatInput, CapabilityInput,
        CloudCredentialRef, CloudCredentialSourceKind, CloudProviderConfig,
        CloudProviderReadiness, EffectiveToolSummaryItem, EffectiveToolSurfaceSummary,
        InteractionMode, LlmCapabilityState, LlmCredentialError, LlmInteractionTrace, LlmMode,
        ToolAvailability, ToolUseProposal, ToolUseProposalStatus,
        LLM_PROCESSING_CONSENT_MISSING,
    };
    use std::env;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn resolves_credential_present_from_environment() {
        let credential_ref = unique_credential_ref("cloud_present");
        let environment_variable = credential_ref.environment_variable_name();
        let secret = "super-secret-token";
        let provider_config = CloudProviderConfig::new(credential_ref.clone());

        env::set_var(&environment_variable, secret);
        let resolution = resolve_cloud_credential(&provider_config).expect("credential resolved");

        assert_eq!(resolution.credential_ref(), &credential_ref);
        assert_eq!(
            resolution.source_kind(),
            CloudCredentialSourceKind::Environment
        );
        assert_eq!(resolution.secret_value(), secret);

        env::remove_var(environment_variable);
    }

    #[test]
    fn fails_with_explicit_error_when_credential_is_missing() {
        let credential_ref = unique_credential_ref("cloud_missing");
        let environment_variable = credential_ref.environment_variable_name();
        let provider_config = CloudProviderConfig::new(credential_ref.clone());
        env::remove_var(&environment_variable);

        let error =
            resolve_cloud_credential(&provider_config).expect_err("credential must be missing");

        assert_eq!(
            error,
            LlmCredentialError::MissingCredential {
                credential_name: credential_ref.name().to_owned(),
                environment_variable,
            }
        );
    }

    #[test]
    fn does_not_expose_secret_value_in_debug_output() {
        let credential_ref = unique_credential_ref("cloud_redacted");
        let environment_variable = credential_ref.environment_variable_name();
        let secret = "top-secret-value";
        let provider_config = CloudProviderConfig::new(credential_ref.clone());

        env::set_var(&environment_variable, secret);
        let resolution = resolve_cloud_credential(&provider_config).expect("credential resolved");
        let debug_output = format!("{resolution:?}");

        assert!(debug_output.contains("<redacted>"));
        assert!(!debug_output.contains(secret));
        assert!(!format!("{resolution:?}").contains("top-secret-value"));

        env::remove_var(environment_variable);
    }

    #[test]
    fn provider_is_ready_when_credential_exists() {
        let credential_ref = unique_credential_ref("cloud_ready");
        let environment_variable = credential_ref.environment_variable_name();
        let provider_config = CloudProviderConfig::new(credential_ref.clone());

        env::set_var(&environment_variable, "ready-secret");
        let readiness =
            evaluate_cloud_provider_readiness(&provider_config).expect("provider readiness");

        assert_eq!(
            readiness,
            CloudProviderReadiness::new(
                credential_ref,
                CloudCredentialSourceKind::Environment,
                true,
            )
        );
        assert!(readiness.is_ready());

        env::remove_var(environment_variable);
    }

    #[test]
    fn provider_readiness_fails_with_explicit_error_when_credential_is_missing() {
        let credential_ref = unique_credential_ref("cloud_not_ready");
        let environment_variable = credential_ref.environment_variable_name();
        let provider_config = CloudProviderConfig::new(credential_ref.clone());
        env::remove_var(&environment_variable);

        let error = evaluate_cloud_provider_readiness(&provider_config)
            .expect_err("provider readiness must fail");

        assert_eq!(
            error,
            LlmCredentialError::MissingCredential {
                credential_name: credential_ref.name().to_owned(),
                environment_variable,
            }
        );
    }

    #[test]
    fn readiness_debug_output_does_not_expose_secret_value() {
        let credential_ref = unique_credential_ref("cloud_ready_redacted");
        let environment_variable = credential_ref.environment_variable_name();
        let provider_config = CloudProviderConfig::new(credential_ref);
        let secret = "cloud-readiness-secret";

        env::set_var(&environment_variable, secret);
        let readiness =
            evaluate_cloud_provider_readiness(&provider_config).expect("provider readiness");
        let debug_output = format!("{readiness:?}");

        assert!(!debug_output.contains(secret));
        assert!(debug_output.contains("ready: true"));

        env::remove_var(environment_variable);
    }

    #[test]
    fn capability_state_off_resolves_to_off_and_guided() {
        let state = resolve_capability_state(CapabilityInput {
            desired_llm_mode: LlmMode::Off,
            provider_configured: false,
            credential_ref_present: false,
            local_engine_available: false,
            local_model_available: false,
            policy_allows_local: false,
            policy_allows_cloud: false,
        });

        assert_eq!(
            state,
            LlmCapabilityState {
                desired_llm_mode: LlmMode::Off,
                effective_llm_mode: LlmMode::Off,
                interaction_mode: InteractionMode::Guided,
                llm_available: false,
                provider_configured: false,
                credential_ref_present: false,
                local_engine_available: false,
                local_model_available: false,
                policy_allows_local: false,
                policy_allows_cloud: false,
                reason_codes: vec![String::from("llm_off_by_preference")],
            }
        );
    }

    #[test]
    fn capability_state_local_missing_engine_falls_back_to_off_and_guided() {
        let state = resolve_capability_state(CapabilityInput {
            desired_llm_mode: LlmMode::Local,
            provider_configured: false,
            credential_ref_present: false,
            local_engine_available: false,
            local_model_available: true,
            policy_allows_local: true,
            policy_allows_cloud: false,
        });

        assert_eq!(state.effective_llm_mode, LlmMode::Off);
        assert_eq!(state.interaction_mode, InteractionMode::Guided);
        assert!(!state.llm_available);
        assert_eq!(
            state.reason_codes,
            vec![
                String::from("local_engine_missing"),
                String::from("assisted_mode_unavailable"),
            ]
        );
    }

    #[test]
    fn capability_state_local_valid_resolves_to_local_and_assisted() {
        let state = resolve_capability_state(CapabilityInput {
            desired_llm_mode: LlmMode::Local,
            provider_configured: false,
            credential_ref_present: false,
            local_engine_available: true,
            local_model_available: true,
            policy_allows_local: true,
            policy_allows_cloud: false,
        });

        assert_eq!(state.effective_llm_mode, LlmMode::Local);
        assert_eq!(state.interaction_mode, InteractionMode::Assisted);
        assert!(state.llm_available);
        assert!(state.reason_codes.is_empty());
    }

    #[test]
    fn capability_state_cloud_missing_credential_falls_back_to_off_and_guided() {
        let state = resolve_capability_state(CapabilityInput {
            desired_llm_mode: LlmMode::Cloud,
            provider_configured: true,
            credential_ref_present: false,
            local_engine_available: false,
            local_model_available: false,
            policy_allows_local: false,
            policy_allows_cloud: true,
        });

        assert_eq!(state.effective_llm_mode, LlmMode::Off);
        assert_eq!(state.interaction_mode, InteractionMode::Guided);
        assert!(!state.llm_available);
        assert_eq!(
            state.reason_codes,
            vec![
                String::from("cloud_credential_ref_missing"),
                String::from("assisted_mode_unavailable"),
            ]
        );
    }

    #[test]
    fn capability_state_cloud_valid_resolves_to_cloud_and_assisted() {
        let state = resolve_capability_state(CapabilityInput {
            desired_llm_mode: LlmMode::Cloud,
            provider_configured: true,
            credential_ref_present: true,
            local_engine_available: false,
            local_model_available: false,
            policy_allows_local: false,
            policy_allows_cloud: true,
        });

        assert_eq!(state.effective_llm_mode, LlmMode::Cloud);
        assert_eq!(state.interaction_mode, InteractionMode::Assisted);
        assert!(state.llm_available);
        assert!(state.reason_codes.is_empty());
    }

    #[test]
    fn guided_mode_cannot_attempt_assisted_response() {
        let envelope = prepare_assisted_chat_envelope(AssistedChatInput {
            user_text: String::from("hello"),
            capability_state: resolve_capability_state(CapabilityInput {
                desired_llm_mode: LlmMode::Off,
                provider_configured: false,
                credential_ref_present: false,
                local_engine_available: false,
                local_model_available: false,
                policy_allows_local: false,
                policy_allows_cloud: false,
            }),
            consent_allows_llm_processing: true,
            context_refs: vec![String::from("ctx:a")],
            prompt_ref: Some(String::from("prompt:lume")),
            tool_surface_summary: None,
        });

        assert_eq!(envelope.interaction_mode, InteractionMode::Guided);
        assert_eq!(envelope.effective_llm_mode, LlmMode::Off);
        assert!(!envelope.can_attempt_assisted_response);
        assert_eq!(
            envelope.reason_codes,
            vec![String::from("llm_off_by_preference")]
        );
    }

    #[test]
    fn assisted_mode_without_consent_cannot_attempt_assisted_response() {
        let envelope = prepare_assisted_chat_envelope(AssistedChatInput {
            user_text: String::from("help me"),
            capability_state: resolve_capability_state(CapabilityInput {
                desired_llm_mode: LlmMode::Local,
                provider_configured: false,
                credential_ref_present: false,
                local_engine_available: true,
                local_model_available: true,
                policy_allows_local: true,
                policy_allows_cloud: false,
            }),
            consent_allows_llm_processing: false,
            context_refs: vec![],
            prompt_ref: None,
            tool_surface_summary: None,
        });

        assert_eq!(envelope.interaction_mode, InteractionMode::Assisted);
        assert_eq!(envelope.effective_llm_mode, LlmMode::Local);
        assert!(!envelope.can_attempt_assisted_response);
        assert_eq!(
            envelope.reason_codes,
            vec![String::from(LLM_PROCESSING_CONSENT_MISSING)]
        );
    }

    #[test]
    fn assisted_mode_with_consent_can_attempt_assisted_response() {
        let envelope = prepare_assisted_chat_envelope(AssistedChatInput {
            user_text: String::from("summarize"),
            capability_state: resolve_capability_state(CapabilityInput {
                desired_llm_mode: LlmMode::Cloud,
                provider_configured: true,
                credential_ref_present: true,
                local_engine_available: false,
                local_model_available: false,
                policy_allows_local: false,
                policy_allows_cloud: true,
            }),
            consent_allows_llm_processing: true,
            context_refs: vec![String::from("doc:123")],
            prompt_ref: Some(String::from("prompt:assistant")),
            tool_surface_summary: None,
        });

        assert_eq!(envelope.interaction_mode, InteractionMode::Assisted);
        assert_eq!(envelope.effective_llm_mode, LlmMode::Cloud);
        assert!(envelope.can_attempt_assisted_response);
        assert!(envelope.reason_codes.is_empty());
    }

    #[test]
    fn existing_capability_reason_codes_are_preserved() {
        let capability_state = LlmCapabilityState {
            desired_llm_mode: LlmMode::Local,
            effective_llm_mode: LlmMode::Off,
            interaction_mode: InteractionMode::Guided,
            llm_available: false,
            provider_configured: false,
            credential_ref_present: false,
            local_engine_available: false,
            local_model_available: true,
            policy_allows_local: true,
            policy_allows_cloud: false,
            reason_codes: vec![
                String::from("local_engine_missing"),
                String::from("assisted_mode_unavailable"),
            ],
        };

        let envelope = prepare_assisted_chat_envelope(AssistedChatInput {
            user_text: String::from("status"),
            capability_state,
            consent_allows_llm_processing: true,
            context_refs: vec![],
            prompt_ref: None,
            tool_surface_summary: None,
        });

        assert_eq!(
            envelope.reason_codes,
            vec![
                String::from("local_engine_missing"),
                String::from("assisted_mode_unavailable"),
            ]
        );
    }

    #[test]
    fn prompt_ref_and_context_refs_are_carried_through_unchanged() {
        let context_refs = vec![String::from("doc:a"), String::from("doc:b")];
        let prompt_ref = Some(String::from("prompt:governed"));
        let tool_surface_summary = None;
        let expected_context_refs = context_refs.clone();
        let expected_prompt_ref = prompt_ref.clone();
        let expected_tool_surface_summary = tool_surface_summary.clone();

        let envelope = prepare_assisted_chat_envelope(AssistedChatInput {
            user_text: String::from("compare"),
            capability_state: resolve_capability_state(CapabilityInput {
                desired_llm_mode: LlmMode::Local,
                provider_configured: false,
                credential_ref_present: false,
                local_engine_available: true,
                local_model_available: true,
                policy_allows_local: true,
                policy_allows_cloud: false,
            }),
            consent_allows_llm_processing: true,
            context_refs,
            prompt_ref,
            tool_surface_summary,
        });

        assert_eq!(
            envelope,
            AssistedChatEnvelope {
                interaction_mode: InteractionMode::Assisted,
                effective_llm_mode: LlmMode::Local,
                can_attempt_assisted_response: true,
                user_text: String::from("compare"),
                context_refs: expected_context_refs,
                prompt_ref: expected_prompt_ref,
                tool_surface_summary: expected_tool_surface_summary,
                reason_codes: Vec::new(),
            }
        );
    }

    #[test]
    fn envelope_carries_tool_surface_summary_unchanged() {
        let summary = build_effective_tool_surface_summary(vec![EffectiveToolSummaryItem {
            tool_id: String::from("text.measure"),
            tool_class: String::from("operational"),
            availability: ToolAvailability::Available,
            reason_codes: vec![String::from("tool_visible")],
        }]);
        let expected_summary = summary.clone();

        let envelope = prepare_assisted_chat_envelope(AssistedChatInput {
            user_text: String::from("measure this"),
            capability_state: resolve_capability_state(CapabilityInput {
                desired_llm_mode: LlmMode::Local,
                provider_configured: false,
                credential_ref_present: false,
                local_engine_available: true,
                local_model_available: true,
                policy_allows_local: true,
                policy_allows_cloud: false,
            }),
            consent_allows_llm_processing: true,
            context_refs: vec![String::from("doc:1")],
            prompt_ref: Some(String::from("prompt:test")),
            tool_surface_summary: Some(summary),
        });

        assert_eq!(envelope.tool_surface_summary, Some(expected_summary));
    }

    #[test]
    fn envelope_allows_absent_tool_surface_summary() {
        let envelope = prepare_assisted_chat_envelope(AssistedChatInput {
            user_text: String::from("help"),
            capability_state: resolve_capability_state(CapabilityInput {
                desired_llm_mode: LlmMode::Off,
                provider_configured: false,
                credential_ref_present: false,
                local_engine_available: false,
                local_model_available: false,
                policy_allows_local: false,
                policy_allows_cloud: false,
            }),
            consent_allows_llm_processing: true,
            context_refs: Vec::new(),
            prompt_ref: None,
            tool_surface_summary: None,
        });

        assert!(envelope.tool_surface_summary.is_none());
    }

    #[test]
    fn envelope_does_not_change_tool_surface_reason_codes() {
        let expected_reason_codes = vec![
            String::from("tool_declared_only"),
            String::from("execution_not_open"),
        ];

        let envelope = prepare_assisted_chat_envelope(AssistedChatInput {
            user_text: String::from("can I use this tool?"),
            capability_state: resolve_capability_state(CapabilityInput {
                desired_llm_mode: LlmMode::Cloud,
                provider_configured: true,
                credential_ref_present: true,
                local_engine_available: false,
                local_model_available: false,
                policy_allows_local: false,
                policy_allows_cloud: true,
            }),
            consent_allows_llm_processing: true,
            context_refs: vec![],
            prompt_ref: None,
            tool_surface_summary: Some(build_effective_tool_surface_summary(vec![
                EffectiveToolSummaryItem {
                    tool_id: String::from("pdf.merge"),
                    tool_class: String::from("operational"),
                    availability: ToolAvailability::Disabled,
                    reason_codes: expected_reason_codes.clone(),
                },
            ])),
        });

        assert_eq!(
            envelope.tool_surface_summary,
            Some(EffectiveToolSurfaceSummary {
                items: vec![EffectiveToolSummaryItem {
                    tool_id: String::from("pdf.merge"),
                    tool_class: String::from("operational"),
                    availability: ToolAvailability::Disabled,
                    reason_codes: expected_reason_codes,
                }],
            })
        );
    }

    #[test]
    fn trace_copies_envelope_modes() {
        let envelope = prepare_assisted_chat_envelope(AssistedChatInput {
            user_text: String::from("summarize"),
            capability_state: resolve_capability_state(CapabilityInput {
                desired_llm_mode: LlmMode::Local,
                provider_configured: false,
                credential_ref_present: false,
                local_engine_available: true,
                local_model_available: true,
                policy_allows_local: true,
                policy_allows_cloud: false,
            }),
            consent_allows_llm_processing: true,
            context_refs: vec![String::from("doc:1")],
            prompt_ref: Some(String::from("prompt:a")),
            tool_surface_summary: None,
        });

        let trace = build_llm_interaction_trace(&envelope, true, false);

        assert_eq!(
            trace,
            LlmInteractionTrace {
                effective_llm_mode: LlmMode::Local,
                interaction_mode: InteractionMode::Assisted,
                llm_available: true,
                consent_allows_llm_processing: true,
                context_refs: vec![String::from("doc:1")],
                prompt_ref: Some(String::from("prompt:a")),
                tool_surface_summary_present: false,
                tool_use_proposal_present: false,
                reason_codes: Vec::new(),
            }
        );
    }

    #[test]
    fn trace_preserves_context_refs() {
        let envelope = AssistedChatEnvelope {
            interaction_mode: InteractionMode::Guided,
            effective_llm_mode: LlmMode::Off,
            can_attempt_assisted_response: false,
            user_text: String::from("hello"),
            context_refs: vec![String::from("ctx:a"), String::from("ctx:b")],
            prompt_ref: None,
            tool_surface_summary: None,
            reason_codes: vec![String::from("llm_off_by_preference")],
        };

        let trace = build_llm_interaction_trace(&envelope, false, false);

        assert_eq!(
            trace.context_refs,
            vec![String::from("ctx:a"), String::from("ctx:b")]
        );
    }

    #[test]
    fn trace_preserves_prompt_ref() {
        let envelope = AssistedChatEnvelope {
            interaction_mode: InteractionMode::Guided,
            effective_llm_mode: LlmMode::Off,
            can_attempt_assisted_response: false,
            user_text: String::from("hello"),
            context_refs: Vec::new(),
            prompt_ref: Some(String::from("prompt:trace")),
            tool_surface_summary: None,
            reason_codes: vec![String::from("llm_off_by_preference")],
        };

        let trace = build_llm_interaction_trace(&envelope, false, false);

        assert_eq!(trace.prompt_ref, Some(String::from("prompt:trace")));
    }

    #[test]
    fn trace_detects_tool_surface_summary_presence() {
        let envelope = AssistedChatEnvelope {
            interaction_mode: InteractionMode::Assisted,
            effective_llm_mode: LlmMode::Cloud,
            can_attempt_assisted_response: true,
            user_text: String::from("compare"),
            context_refs: Vec::new(),
            prompt_ref: None,
            tool_surface_summary: Some(build_effective_tool_surface_summary(vec![
                EffectiveToolSummaryItem {
                    tool_id: String::from("text.measure"),
                    tool_class: String::from("operational"),
                    availability: ToolAvailability::Available,
                    reason_codes: vec![String::from("tool_visible")],
                },
            ])),
            reason_codes: Vec::new(),
        };

        let trace = build_llm_interaction_trace(&envelope, true, false);

        assert!(trace.tool_surface_summary_present);
    }

    #[test]
    fn trace_preserves_reason_codes() {
        let expected_reason_codes = vec![
            String::from("local_engine_missing"),
            String::from("assisted_mode_unavailable"),
        ];
        let envelope = AssistedChatEnvelope {
            interaction_mode: InteractionMode::Guided,
            effective_llm_mode: LlmMode::Off,
            can_attempt_assisted_response: false,
            user_text: String::from("why unavailable"),
            context_refs: Vec::new(),
            prompt_ref: None,
            tool_surface_summary: None,
            reason_codes: expected_reason_codes.clone(),
        };

        let trace = build_llm_interaction_trace(&envelope, true, false);

        assert_eq!(trace.reason_codes, expected_reason_codes);
    }

    #[test]
    fn trace_records_tool_use_proposal_presence() {
        let envelope = AssistedChatEnvelope {
            interaction_mode: InteractionMode::Assisted,
            effective_llm_mode: LlmMode::Local,
            can_attempt_assisted_response: true,
            user_text: String::from("could a tool help"),
            context_refs: Vec::new(),
            prompt_ref: None,
            tool_surface_summary: None,
            reason_codes: Vec::new(),
        };

        let trace = build_llm_interaction_trace(&envelope, true, true);

        assert!(trace.tool_use_proposal_present);
    }

    #[test]
    fn trace_without_tool_surface_summary_is_valid() {
        let envelope = AssistedChatEnvelope {
            interaction_mode: InteractionMode::Guided,
            effective_llm_mode: LlmMode::Off,
            can_attempt_assisted_response: false,
            user_text: String::from("offline"),
            context_refs: Vec::new(),
            prompt_ref: None,
            tool_surface_summary: None,
            reason_codes: vec![String::from("llm_off_by_preference")],
        };

        let trace = build_llm_interaction_trace(&envelope, false, false);

        assert_eq!(
            trace,
            LlmInteractionTrace {
                effective_llm_mode: LlmMode::Off,
                interaction_mode: InteractionMode::Guided,
                llm_available: false,
                consent_allows_llm_processing: false,
                context_refs: Vec::new(),
                prompt_ref: None,
                tool_surface_summary_present: false,
                tool_use_proposal_present: false,
                reason_codes: vec![String::from("llm_off_by_preference")],
            }
        );
    }

    #[test]
    fn summary_preserves_items_unchanged() {
        let items = vec![
            EffectiveToolSummaryItem {
                tool_id: String::from("text.measure"),
                tool_class: String::from("operational"),
                availability: ToolAvailability::Available,
                reason_codes: vec![],
            },
            EffectiveToolSummaryItem {
                tool_id: String::from("pdf.merge"),
                tool_class: String::from("llm"),
                availability: ToolAvailability::Disabled,
                reason_codes: vec![String::from("execution_not_open")],
            },
        ];
        let expected = items.clone();

        let summary = build_effective_tool_surface_summary(items);

        assert_eq!(
            summary,
            EffectiveToolSurfaceSummary { items: expected }
        );
    }

    #[test]
    fn availability_values_are_passed_through() {
        let summary = build_effective_tool_surface_summary(vec![
            EffectiveToolSummaryItem {
                tool_id: String::from("a"),
                tool_class: String::from("operational"),
                availability: ToolAvailability::Available,
                reason_codes: vec![],
            },
            EffectiveToolSummaryItem {
                tool_id: String::from("b"),
                tool_class: String::from("llm"),
                availability: ToolAvailability::Disabled,
                reason_codes: vec![],
            },
            EffectiveToolSummaryItem {
                tool_id: String::from("c"),
                tool_class: String::from("external_dependency"),
                availability: ToolAvailability::Missing,
                reason_codes: vec![],
            },
        ]);

        assert_eq!(summary.items[0].availability, ToolAvailability::Available);
        assert_eq!(summary.items[1].availability, ToolAvailability::Disabled);
        assert_eq!(summary.items[2].availability, ToolAvailability::Missing);
    }

    #[test]
    fn reason_codes_are_preserved() {
        let expected_reason_codes = vec![
            String::from("tool_declared_only"),
            String::from("assisted_mode_unavailable"),
        ];

        let summary = build_effective_tool_surface_summary(vec![EffectiveToolSummaryItem {
            tool_id: String::from("x"),
            tool_class: String::from("llm"),
            availability: ToolAvailability::Disabled,
            reason_codes: expected_reason_codes.clone(),
        }]);

        assert_eq!(summary.items[0].reason_codes, expected_reason_codes);
    }

    #[test]
    fn empty_summary_is_valid() {
        let summary = build_effective_tool_surface_summary(Vec::new());

        assert!(summary.items.is_empty());
    }

    #[test]
    fn proposed_tool_use_preserves_fields() {
        let proposal = propose_tool_use(
            String::from("text.measure"),
            String::from("operational"),
            String::from("This tool may help measure the selected text."),
            vec![String::from("tool_relevant_to_request")],
        );

        assert_eq!(proposal.tool_id, "text.measure");
        assert_eq!(proposal.tool_class, "operational");
        assert_eq!(
            proposal.rationale,
            "This tool may help measure the selected text."
        );
        assert_eq!(
            proposal.reason_codes,
            vec![String::from("tool_relevant_to_request")]
        );
    }

    #[test]
    fn proposed_tool_use_has_proposed_status() {
        let proposal = propose_tool_use(
            String::from("a"),
            String::from("llm"),
            String::from("proposed rationale"),
            Vec::new(),
        );

        assert_eq!(proposal.status, ToolUseProposalStatus::Proposed);
    }

    #[test]
    fn blocked_tool_use_has_blocked_status() {
        let proposal = blocked_tool_use_proposal(
            String::from("b"),
            String::from("external_dependency"),
            String::from("blocked rationale"),
            Vec::new(),
        );

        assert_eq!(proposal.status, ToolUseProposalStatus::Blocked);
    }

    #[test]
    fn blocked_tool_use_preserves_reason_codes() {
        let expected_reason_codes = vec![
            String::from("tool_declared_only"),
            String::from("execution_not_open"),
        ];

        let proposal = blocked_tool_use_proposal(
            String::from("pdf.merge"),
            String::from("operational"),
            String::from("This proposal is blocked in the current phase."),
            expected_reason_codes.clone(),
        );

        assert_eq!(proposal.reason_codes, expected_reason_codes);
    }

    #[test]
    fn proposal_does_not_modify_rationale() {
        let rationale = String::from("Use this tool only as a governed proposal.");

        let proposed = propose_tool_use(
            String::from("c"),
            String::from("llm"),
            rationale.clone(),
            Vec::new(),
        );
        let blocked = blocked_tool_use_proposal(
            String::from("c"),
            String::from("llm"),
            rationale.clone(),
            Vec::new(),
        );

        assert_eq!(
            proposed,
            ToolUseProposal {
                tool_id: String::from("c"),
                tool_class: String::from("llm"),
                status: ToolUseProposalStatus::Proposed,
                rationale: rationale.clone(),
                reason_codes: Vec::new(),
            }
        );
        assert_eq!(blocked.rationale, rationale);
    }

    fn unique_credential_ref(prefix: &str) -> CloudCredentialRef {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after unix epoch")
            .as_nanos();
        CloudCredentialRef::new(format!("{prefix}_{nanos}")).expect("credential ref")
    }
}
