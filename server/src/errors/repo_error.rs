#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("entity not found: {entity_type}")]
    NotFound { entity_type: &'static str },

    #[error("duplicate key violation on {constraint}")]
    Conflict { constraint: String },

    #[error("foreign key violation: {constraint}")]
    ForeignKeyViolation { constraint: String },

    #[error("check constraint violation: {constraint}")]
    CheckViolation { constraint: String },

    #[error("transaction error: {0}")]
    Transaction(String),

    #[error("connection pool error: {0}")]
    ConnectionPool(String),

    #[error("serialization failure - concurrent modification detected")]
    SerializationFailure,

    #[error("database error during {operation}")]
    Database {
        operation: &'static str,
        #[source]
        source: sqlx::Error,
    },
}
