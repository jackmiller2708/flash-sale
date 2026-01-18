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

#[derive(serde::Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
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

impl From<AppError> for ApiError {
    fn from(value: AppError) -> Self {
        match value {
            AppError::Domain(DomainError::ProductNameEmpty) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "PRODUCT_NAME_IS_EMPTY",
                message: "Product name cannot be empty or contain only whitespaces".into(),
            },
            _ => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                code: "INTERNAL_ERROR",
                message: "Something went wrong".into(),
            },
        }
    }
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
