//! Minimal cloud LLM invocation contract for the Rust sandbox.
//!
//! In F9 and default builds, this crate preserves request/result shapes only.
//! Real provider execution remains disabled.

use llm_core::CloudProviderConfig;
use std::fmt;

const OPENAI_PROVIDER: &str = "openai";

/// Minimal cloud invocation request.
#[derive(Clone, PartialEq, Eq)]
pub struct CloudInvokeRequest {
    model: String,
    prompt_text: String,
}

impl CloudInvokeRequest {
    /// Build a minimal cloud invocation request.
    pub fn new(
        model: impl Into<String>,
        prompt_text: impl Into<String>,
    ) -> Result<Self, LlmCloudInvokeError> {
        let model = model.into();
        if model.trim().is_empty() {
            return Err(LlmCloudInvokeError::InvalidRequest {
                reason: "model must not be empty",
            });
        }

        let prompt_text = prompt_text.into();
        if prompt_text.trim().is_empty() {
            return Err(LlmCloudInvokeError::InvalidRequest {
                reason: "prompt_text must not be empty",
            });
        }

        Ok(Self { model, prompt_text })
    }

    /// Return the target model identifier.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Return the user prompt text.
    pub fn prompt_text(&self) -> &str {
        &self.prompt_text
    }
}

impl fmt::Debug for CloudInvokeRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CloudInvokeRequest")
            .field("model", &self.model)
            .field("prompt_text", &"<redacted>")
            .finish()
    }
}

/// Minimal usage payload when the provider returns token counts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudInvokeUsage {
    input_tokens: u64,
    output_tokens: u64,
    total_tokens: u64,
}

impl CloudInvokeUsage {
    /// Build a normalized usage object.
    pub fn new(input_tokens: u64, output_tokens: u64, total_tokens: u64) -> Self {
        Self {
            input_tokens,
            output_tokens,
            total_tokens,
        }
    }

    /// Return input token count.
    pub fn input_tokens(&self) -> u64 {
        self.input_tokens
    }

    /// Return output token count.
    pub fn output_tokens(&self) -> u64 {
        self.output_tokens
    }

    /// Return total token count.
    pub fn total_tokens(&self) -> u64 {
        self.total_tokens
    }
}

/// Normalized cloud invocation result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudInvokeResult {
    provider: &'static str,
    model: String,
    text: String,
    usage: Option<CloudInvokeUsage>,
    readiness_checked: bool,
    latency_ms: u128,
}

impl CloudInvokeResult {
    /// Return provider id.
    pub fn provider(&self) -> &'static str {
        self.provider
    }

    /// Return provider model id.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Return normalized text output.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Return normalized usage if present.
    pub fn usage(&self) -> Option<&CloudInvokeUsage> {
        self.usage.as_ref()
    }

    /// Return whether readiness was checked before invoke.
    pub fn readiness_checked(&self) -> bool {
        self.readiness_checked
    }

    /// Return invoke latency in milliseconds.
    pub fn latency_ms(&self) -> u128 {
        self.latency_ms
    }
}

/// Small invoke error taxonomy for cloud adapter usage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmCloudInvokeError {
    InvalidRequest {
        reason: &'static str,
    },
    ExecutionDisabled {
        provider: &'static str,
        model: String,
        reason: &'static str,
    },
}

impl fmt::Display for LlmCloudInvokeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest { reason } => {
                write!(f, "[llm_cloud:invalid_request] {reason}")
            }
            Self::ExecutionDisabled {
                provider,
                model,
                reason,
            } => write!(
                f,
                "[llm_cloud:execution_disabled] provider={provider} model={model} {reason}"
            ),
        }
    }
}

impl std::error::Error for LlmCloudInvokeError {}

/// In F9 and default builds, real cloud invocation remains disabled.
pub fn invoke_openai_cloud_minimal(
    _provider_config: &CloudProviderConfig,
    request: &CloudInvokeRequest,
) -> Result<CloudInvokeResult, LlmCloudInvokeError> {
    Err(LlmCloudInvokeError::ExecutionDisabled {
        provider: OPENAI_PROVIDER,
        model: request.model().to_owned(),
        reason: "F9 default build keeps cloud provider execution disabled",
    })
}

#[cfg(test)]
mod tests {
    use super::{invoke_openai_cloud_minimal, CloudInvokeRequest, LlmCloudInvokeError, OPENAI_PROVIDER};
    use llm_core::{CloudCredentialRef, CloudProviderConfig};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn rejects_empty_model() {
        let error = CloudInvokeRequest::new("", "hello").expect_err("empty model must fail");

        assert_eq!(
            error,
            LlmCloudInvokeError::InvalidRequest {
                reason: "model must not be empty",
            }
        );
    }

    #[test]
    fn rejects_empty_prompt_text() {
        let error = CloudInvokeRequest::new("gpt-5", "   ").expect_err("empty prompt must fail");

        assert_eq!(
            error,
            LlmCloudInvokeError::InvalidRequest {
                reason: "prompt_text must not be empty",
            }
        );
    }

    #[test]
    fn default_build_blocks_cloud_execution_without_resolving_credentials() {
        let credential_ref = unique_credential_ref("llm_cloud_disabled");
        let provider_config = CloudProviderConfig::new(credential_ref);
        let request = CloudInvokeRequest::new("gpt-5", "Say hello").expect("request");

        let error =
            invoke_openai_cloud_minimal(&provider_config, &request).expect_err("must be disabled");

        assert_eq!(
            error,
            LlmCloudInvokeError::ExecutionDisabled {
                provider: OPENAI_PROVIDER,
                model: "gpt-5".to_owned(),
                reason: "F9 default build keeps cloud provider execution disabled",
            }
        );
    }

    #[test]
    fn request_debug_does_not_expose_prompt_text() {
        let request =
            CloudInvokeRequest::new("gpt-5", "super private prompt text").expect("request");

        let debug_output = format!("{request:?}");

        assert!(debug_output.contains("<redacted>"));
        assert!(!debug_output.contains("super private prompt text"));
    }

    fn unique_credential_ref(prefix: &str) -> CloudCredentialRef {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after unix epoch")
            .as_nanos();
        CloudCredentialRef::new(format!("{prefix}_{nanos}")).expect("credential ref")
    }
}
