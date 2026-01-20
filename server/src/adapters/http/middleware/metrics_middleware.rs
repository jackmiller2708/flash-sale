use axum::{
    body::Body, extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse,
};
use metrics::histogram;
use std::time::Instant;

pub async fn track_metrics(req: Request<Body>, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let path = req
        .extensions()
        .get::<MatchedPath>()
        .map(|matched_path| matched_path.as_str().to_owned())
        .unwrap_or_else(|| req.uri().path().to_owned());
    let method = req.method().to_string();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    histogram!("http_requests_duration_seconds", "method" => method, "path" => path, "status" => status).record(latency);

    response
}
