use axum::{Json, extract::State};

use crate::{
    adapters::http::dtos::{CreateProductRequest, ProductResponse},
    app::state::AppState,
    errors::ApiError,
    logic::product_logic,
};

pub async fn create_product(
    State(state): State<AppState>,
    Json(req): Json<CreateProductRequest>,
) -> Result<Json<ProductResponse>, ApiError> {
    let mut tx = state.db_pool.begin().await.map_err(|e| ApiError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        code: "INTERNAL_ERROR",
        message: e.to_string(),
    })?;

    let create_product_command =
        product_logic::CreateProductCommand::try_from(req).map_err(ApiError::from)?;

    let product =
        product_logic::save_product(&mut tx, &*state.product_repo, create_product_command)
            .await
            .map_err(ApiError::from)?;

    tx.commit().await.map_err(|e| ApiError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        code: "INTERNAL_ERROR",
        message: e.to_string(),
    })?;

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
