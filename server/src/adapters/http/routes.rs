use axum::{
    Router,
    routing::{get, post},
};

use crate::adapters::http::handlers;
use crate::app::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::health_handler::hello_world))
        .route("/health", get(handlers::health_handler::health_check))
        .route("/users", post(handlers::user_handler::create_user))
        .route("/users", get(handlers::user_handler::get_users))
        .route("/users/{id}", get(handlers::user_handler::get_user_by_id))
        .route("/products", post(handlers::product_handler::create_product))
        .route("/products", get(handlers::product_handler::get_products))
        .route(
            "/purchase",
            post(handlers::purchase_handler::create_purchase),
        )
}
