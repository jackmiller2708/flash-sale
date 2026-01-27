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
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderAcceptedResponse>), ApiError> {
    // Extract and validate Idempotency-Key header
    let idempotency_key = match headers.get("idempotency-key") {
        Some(value) => {
            let s = value.to_str().map_err(|_| {
                ApiError::bad_request("Idempotency-Key must be valid UTF-8".to_string())
            })?;

            uuid::Uuid::parse_str(s.trim())
                .map(|u| u.to_string())
                .map_err(|_| {
                    ApiError::bad_request(
                        "Invalid Idempotency-Key format. Expected UUID.".to_string(),
                    )
                })?
        }
        None => {
            return Err(ApiError::bad_request(
                "Idempotency-Key header is required for all order requests".to_string(),
            ));
        }
    };

    // 1. Generate deterministic ID from idempotency key
    // This allows us to return the same order_id even before it hits the DB
    let namespace = Uuid::from_u128(0x6ba7b810_9dad_11d1_80b4_00c04fd430c8);
    let order_id = Uuid::new_v5(&namespace, idempotency_key.as_bytes());

    // 2. Construct command
    let command = order_logic::CreateOrderCommand {
        order_id,
        user_id: payload.user_id,
        flash_sale_id: payload.flash_sale_id,
        quantity: payload.quantity,
        idempotency_key, // Key is moved here
    };

    // 3. Check rate limit
    if !state.rate_limiter.check(command.user_id) {
        metrics::counter!("rate_limit_rejections_total").increment(1);
        return Err(ApiError::from(crate::errors::AppError::Service(
            crate::errors::ServiceError::RateLimitExceeded,
        )));
    }

    // 4. Try to reserve a slot in the queue
    let permit = match state.order_queue_tx.try_reserve() {
        Ok(permit) => permit,
        Err(_) => {
            metrics::counter!("order_queue_overflow_total").increment(1);
            return Err(ApiError::service_unavailable(
                "Order queue is full. Please try again later.".to_string(),
            ));
        }
    };

    // 5. Update status store
    state
        .order_status_store
        .insert(order_id, OrderProcessingStatus::Pending);

    // 6. Send to worker
    permit.send(OrderQueueMessage { order_id, command });

    // 7. Return 202 Accepted
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
