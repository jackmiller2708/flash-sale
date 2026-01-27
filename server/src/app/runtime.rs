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
    // Load configuration first to get log settings
    let config = Config::from_env()?;

    // Create logs directory if it doesn't exist
    std::fs::create_dir_all(&config.log_dir)?;

    // Set up file appender with daily rotation
    let file_appender = tracing_appender::rolling::daily(&config.log_dir, "flash-sale.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Initialize tracing subscriber with file output
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::new(&config.log_level).add_directive("sqlx::query=info".parse().unwrap()),
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .json() // Maintain JSON formatting for structured logs
        .with_writer(non_blocking)
        .init();

    tracing::info!(
        "Logging initialized: dir={}, level={}",
        config.log_dir,
        config.log_level
    );
    tracing::debug!("Configuration loaded: {:?}", config);

    // Initialize Prometheus metrics with Histogram buckets
    let prometheus_handle = metrics_exporter_prometheus::PrometheusBuilder::new()
        .set_buckets(&[
            0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ])
        .expect("Failed to set buckets")
        .install_recorder()
        .expect("failed to install Prometheus recorder");
    tracing::info!("Prometheus metrics initialized");

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

    // Initialize order queue worker
    const ORDER_QUEUE_CAPACITY: usize = 100;

    // Create the order status store
    let order_status_store = std::sync::Arc::new(dashmap::DashMap::new());

    let order_queue_tx = crate::app::order_queue::spawn_order_queue_worker(
        pool.clone(),
        flash_sale_repo.clone(),
        order_repo.clone(),
        order_status_store.clone(),
        ORDER_QUEUE_CAPACITY,
    );
    tracing::info!(
        "Order queue worker spawned with capacity {}",
        ORDER_QUEUE_CAPACITY
    );

    // Initialize rate limiter (10 requests per second per user)
    const RATE_LIMIT_PER_USER: u32 = 10;
    let rate_limiter = crate::adapters::http::middleware::UserRateLimiter::new(RATE_LIMIT_PER_USER);
    tracing::info!(
        "Rate limiter initialized: {} req/s per user",
        RATE_LIMIT_PER_USER
    );

    let app = http_router(AppState {
        user_repo,
        product_repo,
        flash_sale_repo,
        order_repo,
        db_pool: pool,
        prometheus_handle,
        order_queue_tx,
        rate_limiter,
        order_status_store,
    });
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
