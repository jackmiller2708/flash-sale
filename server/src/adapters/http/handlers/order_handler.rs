use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    adapters::http::dtos::order_dto::{
        CreateOrderRequest, OrderAcceptedResponse, OrderResult, OrderStatusResponse,
    },
    app::{order_queue::OrderQueueMessage, state::AppState},
    domain::order::OrderProcessingStatus,
    errors::ApiError,
    logic::order_logic,
};

pub async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderAcceptedResponse>), ApiError> {
    let command = order_logic::CreateOrderCommand::from(payload);

    // Check rate limit for this user
    if !state.rate_limiter.check(command.user_id) {
        metrics::counter!("rate_limit_rejections_total").increment(1);
        return Err(ApiError::from(crate::errors::AppError::Service(
            crate::errors::ServiceError::RateLimitExceeded,
        )));
    }

    // 1. Try to reserve a slot in the queue FIRST
    // This avoids unnecessary DashMap churn for 503s
    let permit = match state.order_queue_tx.try_reserve() {
        Ok(permit) => permit,
        Err(_) => {
            metrics::counter!("order_queue_overflow_total").increment(1);
            return Err(ApiError::service_unavailable(
                "Order queue is full. Please try again later.".to_string(),
            ));
        }
    };

    // 2. Only now generate ID and insert into status store
    let order_id = Uuid::new_v4();
    state
        .order_status_store
        .insert(order_id, OrderProcessingStatus::Pending);

    // 3. Send the message using the permit (guaranteed slot)
    permit.send(OrderQueueMessage { order_id, command });

    // Return 202 Accepted immediately (don't wait for worker!)
    Ok((
        StatusCode::ACCEPTED,
        Json(OrderAcceptedResponse {
            order_id,
            status: "pending".to_string(),
            status_url: format!("/orders/{}/status", order_id),
        }),
    ))
}

pub async fn get_order_status(
    State(state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderStatusResponse>, ApiError> {
    // DashMap::get returns a read-only view, no .read().await needed
    match state.order_status_store.get(&order_id) {
        Some(entry) => {
            let status = entry.value();
            match status {
                OrderProcessingStatus::Pending => Ok(Json(OrderStatusResponse {
                    order_id,
                    status: "pending".to_string(),
                    result: None,
                })),
                OrderProcessingStatus::Completed(order) => Ok(Json(OrderStatusResponse {
                    order_id,
                    status: "completed".to_string(),
                    result: Some(OrderResult::Success(order.clone().into())),
                })),
                OrderProcessingStatus::Failed(error) => Ok(Json(OrderStatusResponse {
                    order_id,
                    status: "failed".to_string(),
                    result: Some(OrderResult::Error {
                        message: error.clone(),
                    }),
                })),
            }
        }
        None => Err(ApiError {
            status: StatusCode::NOT_FOUND,
            code: "ORDER_NOT_FOUND",
            message: "Order not found".to_string(),
        }),
    }
}
