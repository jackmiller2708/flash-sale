use axum::Router;

use crate::adapters::http::{middleware::logging_middleware, routes};
use crate::app::state::AppState;

pub fn http_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::routes())
        .layer(logging_middleware::logging())
        .with_state(state)
}
