use axum::{Json, extract::State, http::StatusCode};
use tokio::sync::oneshot;

use crate::{
    adapters::http::dtos::order_dto::{CreateOrderRequest, OrderResponse},
    app::{order_queue::OrderQueueMessage, state::AppState},
    errors::ApiError,
    logic::order_logic,
};

pub async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), ApiError> {
    let command = order_logic::CreateOrderCommand::from(payload);

    // Check rate limit for this user
    if !state.rate_limiter.check(command.user_id) {
        metrics::counter!("rate_limit_rejections_total").increment(1);
        return Err(ApiError::from(crate::errors::AppError::Service(
            crate::errors::ServiceError::RateLimitExceeded,
        )));
    }

    // Create a oneshot channel to receive the response
    let (response_tx, response_rx) = oneshot::channel();

    // Try to enqueue the order request
    let queue_msg = OrderQueueMessage {
        command,
        response_tx,
    };

    if state.order_queue_tx.send(queue_msg).await.is_err() {
        // Queue is full - fail fast with 503
        metrics::counter!("order_queue_overflow_total").increment(1);
        return Err(ApiError::service_unavailable(
            "Order queue is full. Please try again later.".to_string(),
        ));
    }

    // Wait for the worker to process the order
    let order = response_rx
        .await
        .map_err(|_| ApiError::internal("Failed to receive order processing result".to_string()))?
        .map_err(ApiError::from)?;

    Ok((StatusCode::CREATED, Json(order.into())))
}
