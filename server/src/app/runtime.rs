use std::sync::Arc;
use tracing_subscriber::EnvFilter;

use crate::{
    adapters::{
        db::{
            pool::create_pool, product::repository::PostgresProductRepo,
            user::repository::PostgresUserRepo,
        },
        http::router::http_router,
    },
    app::{config::Config, state::AppState},
};

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug").add_directive("sqlx::query=info".parse().unwrap()))
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .compact()
        .init();

    let config = Config::from_env()?;
    tracing::debug!("Configuration loaded: {:?}", config);

    let pool = create_pool(&config).await?;

    let user_repo = Arc::new(PostgresUserRepo::new(pool.clone()));
    tracing::debug!("initialized repository: User");

    let product_repo = Arc::new(PostgresProductRepo::new(pool.clone()));
    tracing::debug!("initialized repository: Product");

    let state = AppState {
        user_repo,
        product_repo,
    };
    let app = http_router(state);
    tracing::debug!("HTTP router configured");

    let listener = tokio::net::TcpListener::bind(&config.http_addr).await?;

    tracing::info!("Server listening on {}", config.http_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
