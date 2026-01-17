use crate::errors::domain_error::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("unexpected error")]
    Unexpected(#[from] anyhow::Error),
}
