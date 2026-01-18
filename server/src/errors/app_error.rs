use crate::errors::{RepoError, domain_error::DomainError};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Repo(#[from] RepoError),

    #[error("unexpected error")]
    Unexpected(#[from] anyhow::Error),
}
