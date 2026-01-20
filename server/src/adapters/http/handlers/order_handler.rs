use axum::{Json, extract::State};

use crate::{
    adapters::http::dtos::order_dto::{CreateOrderRequest, OrderResponse},
    app::state::AppState,
    errors::ApiError,
    logic::order_logic,
};

pub async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<OrderResponse>, ApiError> {
    let mut tx = state
        .db_pool()
        .begin()
        .await
        .map_err(ApiError::transaction_error)?;

    let command = order_logic::CreateOrderCommand::from(payload);

    let order = order_logic::create_order(
        &mut *tx,
        &*state.flash_sale_repo,
        &*state.order_repo,
        command,
    )
    .await
    .map_err(ApiError::from)?;

    tx.commit().await.map_err(ApiError::transaction_error)?;

    Ok(Json(order.into()))
}
