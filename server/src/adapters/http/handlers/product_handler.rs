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
    let mut tx = state
        .db_pool
        .begin()
        .await
        .map_err(ApiError::transaction_error)?;

    let command = product_logic::CreateProductCommand::try_from(req).map_err(ApiError::from)?;

    let product = product_logic::save_product(&mut *tx, &*state.product_repo, command)
        .await
        .map_err(ApiError::from)?;

    tx.commit().await.map_err(ApiError::transaction_error)?;

    Ok(Json(product.into()))
}

pub async fn get_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductResponse>>, ApiError> {
    let mut conn = state
        .db_pool
        .acquire()
        .await
        .map_err(ApiError::connection_error)?;

    let products = product_logic::get_products(&mut *conn, &*state.product_repo)
        .await
        .map_err(ApiError::from)?;

    Ok(Json(products.into_iter().map(Into::into).collect()))
}
