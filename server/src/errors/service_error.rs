#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("authentication required")]
    Unauthenticated,

    #[error("insufficient permissions: {0}")]
    Forbidden(String),

    #[error("business rule violation: {0}")]
    BusinessRule(String),

    #[error("resource conflict: {0}")]
    Conflict(String),

    #[error("invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("external service error: {service}")]
    ExternalService {
        service: &'static str,
        #[source]
        source: anyhow::Error,
    },

    #[error("rate limit exceeded")]
    RateLimitExceeded,
}
