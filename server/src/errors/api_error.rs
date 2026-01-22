use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::errors::{AppError, DomainError};

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}

impl From<AppError> for ApiError {
    fn from(value: AppError) -> Self {
        match value {
            // Domain errors -> 400 Bad Request
            AppError::Domain(DomainError::ProductNameEmpty) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "PRODUCT_NAME_EMPTY",
                message: "Product name cannot be empty".into(),
            },
            AppError::Domain(DomainError::ProductPriceInvalid) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "PRODUCT_PRICE_INVALID",
                message: "Product price must be positive".into(),
            },
            AppError::Domain(DomainError::InvalidEmail(email)) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "INVALID_EMAIL",
                message: format!("Invalid email format: {}", email),
            },
            AppError::Domain(DomainError::InvalidUsername) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "INVALID_USERNAME",
                message: "Username must be between 3 and 50 characters".into(),
            },
            AppError::Domain(DomainError::InvalidFlashSaleStartTime) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "INVALID_FLASH_SALE_START_TIME",
                message: "Flash sale start time must be in the future".into(),
            },
            AppError::Domain(DomainError::InvalidFlashSaleEndTime) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "INVALID_FLASH_SALE_END_TIME",
                message: "Flash sale end time must be after start time".into(),
            },
            AppError::Domain(DomainError::InvalidFlashSaleQuantity) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "INVALID_FLASH_SALE_QUANTITY",
                message: "Flash sale quantity must be positive".into(),
            },
            AppError::Domain(DomainError::InvalidOrderQuantity) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "INVALID_ORDER_QUANTITY",
                message: "Order quantity must be positive".into(),
            },
            AppError::Domain(DomainError::OrderAlreadyCompleted) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "ORDER_ALREADY_COMPLETED",
                message: "Cannot modify completed order".into(),
            },

            // Repository errors
            AppError::Repo(crate::errors::RepoError::NotFound { entity_type }) => Self {
                status: StatusCode::NOT_FOUND,
                code: "NOT_FOUND",
                message: format!("{} not found", entity_type),
            },
            AppError::Repo(crate::errors::RepoError::Conflict { constraint }) => Self {
                status: StatusCode::CONFLICT,
                code: "CONFLICT",
                message: format!("Resource already exists: {}", constraint),
            },
            AppError::Repo(crate::errors::RepoError::ForeignKeyViolation { constraint }) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "FOREIGN_KEY_VIOLATION",
                message: format!("Invalid reference: {}", constraint),
            },
            AppError::Repo(crate::errors::RepoError::CheckViolation { constraint }) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "CHECK_VIOLATION",
                message: format!("Constraint violation: {}", constraint),
            },
            AppError::Repo(crate::errors::RepoError::SerializationFailure) => {
                tracing::warn!("Database serialization failure (concurrent modification detected)");
                Self {
                    status: StatusCode::CONFLICT,
                    code: "CONCURRENT_MODIFICATION",
                    message: "Resource was modified by another request".into(),
                }
            }
            AppError::Repo(crate::errors::RepoError::Transaction(ref err)) => {
                tracing::error!(error = ?err, "Database transaction failed");
                Self {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    code: "TRANSACTION_ERROR",
                    message: "Database transaction failed".into(),
                }
            }
            AppError::Repo(crate::errors::RepoError::ConnectionPool(ref err)) => {
                tracing::error!(error = ?err, "Database connection pool exhausted or unavailable");
                Self {
                    status: StatusCode::SERVICE_UNAVAILABLE,
                    code: "DATABASE_UNAVAILABLE",
                    message: "Database connection failed".into(),
                }
            }
            AppError::Repo(crate::errors::RepoError::Database {
                ref source,
                operation,
            }) => {
                tracing::error!(
                    error = ?source,
                    operation = operation,
                    "Database operation failed"
                );
                Self {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    code: "DATABASE_ERROR",
                    message: "Database operation failed".into(),
                }
            }

            // Service errors
            AppError::Service(crate::errors::ServiceError::Unauthenticated) => Self {
                status: StatusCode::UNAUTHORIZED,
                code: "UNAUTHENTICATED",
                message: "Authentication required".into(),
            },
            AppError::Service(crate::errors::ServiceError::Forbidden(msg)) => Self {
                status: StatusCode::FORBIDDEN,
                code: "FORBIDDEN",
                message: msg,
            },
            AppError::Service(crate::errors::ServiceError::BusinessRule(msg)) => Self {
                status: StatusCode::UNPROCESSABLE_ENTITY,
                code: "BUSINESS_RULE_VIOLATION",
                message: msg,
            },
            AppError::Service(crate::errors::ServiceError::Conflict(msg)) => Self {
                status: StatusCode::CONFLICT,
                code: "CONFLICT",
                message: msg,
            },
            AppError::Service(crate::errors::ServiceError::InvalidStateTransition(msg)) => Self {
                status: StatusCode::CONFLICT,
                code: "INVALID_STATE_TRANSITION",
                message: msg,
            },
            AppError::Service(crate::errors::ServiceError::ExternalService { service, source }) => {
                tracing::error!(
                    service = service,
                    error = ?source,
                    "External service call failed"
                );
                Self {
                    status: StatusCode::BAD_GATEWAY,
                    code: "EXTERNAL_SERVICE_ERROR",
                    message: format!("External service error: {}", service),
                }
            }
            AppError::Service(crate::errors::ServiceError::RateLimitExceeded) => Self {
                status: StatusCode::TOO_MANY_REQUESTS,
                code: "RATE_LIMIT_EXCEEDED",
                message: "Too many requests".into(),
            },

            // Catch-all for unexpected errors
            AppError::Unexpected(ref err) => {
                tracing::error!(error = ?err, "Unexpected error occurred");
                Self {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    code: "INTERNAL_ERROR",
                    message: "An unexpected error occurred".into(),
                }
            }
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        let fallback_message = value.to_string();

        if let Ok(app) = value.downcast::<AppError>() {
            return ApiError::from(app);
        }

        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "INTERNAL_ERROR",
            message: fallback_message,
        }
    }
}

#[derive(serde::Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(ErrorBody {
            code: self.code,
            message: self.message,
        });

        (self.status, body).into_response()
    }
}

impl ApiError {
    pub fn transaction_error(source: sqlx::Error) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "TRANSACTION_ERROR",
            message: format!("Database transaction failed: {}", source),
        }
    }

    pub fn connection_error(_source: sqlx::Error) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "DATABASE_UNAVAILABLE",
            message: "Database connection failed".into(),
        }
    }

    pub fn service_unavailable(message: String) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "SERVICE_UNAVAILABLE",
            message,
        }
    }

    pub fn internal(message: String) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "INTERNAL_ERROR",
            message,
        }
    }
}
