use crate::errors::RepoError;

/// Maps SQLx errors to RepoError variants with proper context
pub fn map_sqlx_error(
    err: sqlx::Error,
    operation: &'static str,
    entity_type: &'static str,
) -> RepoError {
    match &err {
        sqlx::Error::RowNotFound => RepoError::NotFound { entity_type },
        sqlx::Error::Database(db_err) => {
            if let Some(constraint) = db_err.constraint() {
                if db_err.is_unique_violation() {
                    return RepoError::Conflict {
                        constraint: constraint.to_string(),
                    };
                }
                if db_err.is_foreign_key_violation() {
                    return RepoError::ForeignKeyViolation {
                        constraint: constraint.to_string(),
                    };
                }
                if db_err.is_check_violation() {
                    return RepoError::CheckViolation {
                        constraint: constraint.to_string(),
                    };
                }
            }

            // Check for serialization failure (PostgreSQL error code 40001)
            if db_err.code() == Some(std::borrow::Cow::Borrowed("40001")) {
                return RepoError::SerializationFailure;
            }

            RepoError::Database {
                operation,
                source: err,
            }
        }
        _ => RepoError::Database {
            operation,
            source: err,
        },
    }
}
