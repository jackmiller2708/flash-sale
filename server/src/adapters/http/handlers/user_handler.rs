use axum::{Json, extract::State, http::StatusCode};
use chrono::DateTime;
use uuid::Uuid;

use crate::{
    adapters::http::dtos::user_dto::UserResponse, app::state::AppState, domain::user::User,
    errors::ApiError, logic::user_logic,
};

pub async fn create_user(State(state): State<AppState>) -> Result<Json<UserResponse>, ApiError> {
    let mut tx = state
        .db_pool
        .begin()
        .await
        .map_err(ApiError::transaction_error)?;

    let user = User {
        id: Uuid::new_v4(),
        created_at: DateTime::default(),
    };

    let saved_user = user_logic::save_user(&mut *tx, &*state.user_repo, user)
        .await
        .map_err(ApiError::from)?;

    tx.commit().await.map_err(ApiError::transaction_error)?;

    Ok(Json(saved_user.into()))
}

pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<UserResponse>>, ApiError> {
    let mut conn = state
        .db_pool
        .acquire()
        .await
        .map_err(ApiError::connection_error)?;

    let users = user_logic::get_users(&mut *conn, &*state.user_repo)
        .await
        .map_err(ApiError::from)?;

    Ok(Json(users.into_iter().map(|u| u.into()).collect()))
}

pub async fn get_user_by_id(
    State(state): State<AppState>,
    params: axum::extract::Path<String>,
) -> Result<Json<UserResponse>, ApiError> {
    let uuid = Uuid::parse_str(&params.0).map_err(|_| ApiError {
        status: StatusCode::BAD_REQUEST,
        code: "INVALID_UUID",
        message: "Invalid UUID format".into(),
    })?;

    let mut conn = state
        .db_pool
        .acquire()
        .await
        .map_err(ApiError::connection_error)?;

    let user = user_logic::get_user_by_id(&mut *conn, &*state.user_repo, uuid)
        .await
        .map_err(ApiError::from)?;

    Ok(Json(user.into()))
}
