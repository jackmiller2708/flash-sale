use axum::{Router, extract::State, routing::get};

use crate::adapters::http::{middleware::logging, routes};
use crate::app::state::AppState;

pub fn http_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/metrics",
            get(|State(state): State<AppState>| async move { state.prometheus_handle.render() }),
        )
        .merge(routes::routes())
        .layer(axum::middleware::from_fn(
            crate::adapters::http::middleware::track_metrics,
        ))
        .layer(logging())
        .with_state(state)
}
