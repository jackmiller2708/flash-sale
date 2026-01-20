use metrics_exporter_prometheus::PrometheusBuilder;
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
        .json() // Enable JSON formatting
        .init();

    // Initialize Prometheus metrics with Histogram buckets
    let prometheus_handle = PrometheusBuilder::new()
        .set_buckets(&[
            0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ])
        .expect("Failed to set buckets")
        .install_recorder()
        .expect("failed to install Prometheus recorder");
    tracing::info!("Prometheus metrics initialized");

    let config = Config::from_env()?;
    tracing::debug!("Configuration loaded: {:?}", config);

    let pool = create_pool(&config).await?;

    // Periodically record SQLx pool metrics
    let pool_clone = pool.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;

            let stats = pool_clone.size();
            let idle = pool_clone.num_idle();

            metrics::gauge!("sqlx_pool_active_connections").set(stats as f64 - idle as f64);
            metrics::gauge!("sqlx_pool_idle_connections").set(idle as f64);
            // sqlx 0.8 stats don't easily expose 'waiters' without more complex access,
            // but we can at least monitor the pool size and idle connections.
        }
    });

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
        prometheus_handle,
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
