#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("entity not found")]
    NotFound,

    #[error("duplicate key")]
    Conflict,

    #[error(transparent)]
    Unexpected(#[from] sqlx::Error),
}
