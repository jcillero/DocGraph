use std::error::Error;
use std::fmt;

/// Stable error domain tags used by the first Rust contract layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorDomain {
    CoreDomain,
    Workspace,
    SpecRuntime,
    ProjectRuntime,
}

impl fmt::Display for ErrorDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::CoreDomain => "core_domain",
            Self::Workspace => "workspace",
            Self::SpecRuntime => "spec_runtime",
            Self::ProjectRuntime => "project_runtime",
        };
        f.write_str(text)
    }
}

/// Small shared application error shape for cross-crate propagation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableAppError {
    domain: ErrorDomain,
    code: &'static str,
    message: String,
}

impl PortableAppError {
    /// Create a new typed sandbox error.
    pub fn new(domain: ErrorDomain, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            domain,
            code,
            message: message.into(),
        }
    }

    /// Return the stable domain tag for this error.
    pub fn domain(&self) -> ErrorDomain {
        self.domain
    }

    /// Return the stable machine-facing error code.
    pub fn code(&self) -> &'static str {
        self.code
    }

    /// Return the human-readable message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for PortableAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}] {}", self.domain, self.code, self.message)
    }
}

impl Error for PortableAppError {}

/// Shared result alias for small cross-crate constructor helpers.
pub type PortableAppResult<T> = Result<T, PortableAppError>;
