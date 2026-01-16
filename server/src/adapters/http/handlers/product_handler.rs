use axum::{Json, extract::State};

use crate::{
    adapters::http::dtos::product_dto::{CreateProductRequest, ProductResponse},
    app::state::AppState,
    logic::product_logic,
};

pub async fn create_product(
    State(state): State<AppState>,
    Json(req): Json<CreateProductRequest>,
) -> Result<Json<ProductResponse>, (axum::http::StatusCode, String)> {
    let create_product_command = product_logic::CreateProductCommand::try_from(req)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    let product = product_logic::save_product(&*state.product_repo, create_product_command)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(product.into()))
}

pub async fn get_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductResponse>>, (axum::http::StatusCode, String)> {
    let products = product_logic::get_products(&*state.product_repo)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(products.into_iter().map(|p| p.into()).collect()))
}
