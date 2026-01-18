use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::app::config::Config;

pub async fn create_pool(cfg: &Config) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.database_url)
        .await?;

    tracing::info!("Connection pool created with max_connections: 10");

    Ok(pool)
}
