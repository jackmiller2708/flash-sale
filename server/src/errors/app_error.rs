use crate::errors::{DomainError, RepoError, ServiceError};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Repo(#[from] RepoError),

    #[error(transparent)]
    Service(#[from] ServiceError),

    #[error("unexpected error")]
    Unexpected(#[from] anyhow::Error),
}
