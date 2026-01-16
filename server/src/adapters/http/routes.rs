use axum::{
    Router,
    routing::{get, post},
};

use crate::adapters::http::handlers::{product_handler, user_handler};
use crate::app::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(user_handler::create_user))
        .route("/users", get(user_handler::get_users))
        .route("/users/{id}", get(user_handler::get_user_by_id))
        .route("/products", post(product_handler::create_product))
        .route("/products", get(product_handler::get_products))
}
