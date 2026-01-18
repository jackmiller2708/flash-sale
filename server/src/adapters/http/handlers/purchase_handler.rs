use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use uuid::Uuid;

use crate::{
    adapters::http::dtos::purchase_dto::{PurchaseRequest, PurchaseResponse},
    app::state::AppState,
    domain::order::Order,
};

pub async fn create_purchase(
    State(state): State<AppState>,
    Json(payload): Json<PurchaseRequest>,
) -> impl IntoResponse {
    // START TRANSACTION
    let mut tx = match state.db_pool().begin().await {
        Ok(tx) => tx,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "failed to begin transaction"})),
            )
                .into_response();
        }
    };

    // 1. Fetch Flash Sale with Lock
    let flash_sale = match state
        .flash_sale_repo
        .find_by_id_with_lock(&mut tx, payload.flash_sale_id)
        .await
    {
        Ok(Some(fs)) => fs,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "flash sale not found"})),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response();
        }
    };

    // 2. Check Inventory
    if flash_sale.remaining_inventory < payload.quantity {
        return (StatusCode::CONFLICT, Json(json!({"error": "sold out"}))).into_response();
    }

    // 3. Decrement Inventory
    let mut updated_flash_sale = flash_sale.clone();
    updated_flash_sale.remaining_inventory -= payload.quantity;

    if let Err(e) = state
        .flash_sale_repo
        .update(&mut tx, &updated_flash_sale)
        .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response();
    }

    // 4. Create Order
    let user_id = Uuid::new_v4();
    let order = Order::new(user_id, payload.flash_sale_id, payload.quantity);

    let saved_order = match state.order_repo.save(&mut tx, &order).await {
        Ok(o) => o,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response();
        }
    };

    // 5. Commit
    if let Err(_) = tx.commit().await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "failed to commit transaction"})),
        )
            .into_response();
    }

    (
        StatusCode::CREATED,
        Json(PurchaseResponse {
            order_id: saved_order.id,
            status: "CONFIRMED".to_string(),
        }),
    )
        .into_response()
}
