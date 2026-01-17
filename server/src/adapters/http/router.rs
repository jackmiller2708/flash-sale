use axum::Router;

use crate::adapters::http::{middleware::logging, routes};
use crate::app::state::AppState;

pub fn http_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::routes())
        .layer(logging())
        .with_state(state)
}
