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

    let user_repo = Arc::new(PostgresUserRepo::new()) as Arc<dyn crate::ports::user_repo::UserRepo>;
    tracing::debug!("initialized repository: User");

    let product_repo =
        Arc::new(PostgresProductRepo::new()) as Arc<dyn crate::ports::product_repo::ProductRepo>;
    tracing::debug!("initialized repository: Product");

    let flash_sale_repo =
        Arc::new(crate::adapters::db::flash_sale::repository::PostgresFlashSaleRepo::new())
            as Arc<dyn crate::ports::flash_sale_repo::FlashSaleRepo>;
    tracing::debug!("initialized repository: FlashSale");

    let order_repo = Arc::new(crate::adapters::db::order::repository::PostgresOrderRepo::new())
        as Arc<dyn crate::ports::order_repo::OrderRepo>;
    tracing::debug!("initialized repository: Order");

    let state = AppState {
        user_repo,
        product_repo,
        flash_sale_repo,
        order_repo,
        db_pool: pool,
    };
    let app = http_router(state);
    tracing::debug!("HTTP router configured");

    let listener = tokio::net::TcpListener::bind(&config.http_addr).await?;

    tracing::info!("Server listening on {}", config.http_addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}
