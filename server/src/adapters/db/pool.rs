use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::app::config::Config;

pub async fn create_pool(cfg: &Config) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.database_url)
        .await
}
