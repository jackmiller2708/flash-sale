use anyhow::Context;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub http_addr: SocketAddr,
    pub database_url: String,
    pub log_dir: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            http_addr: std::env::var("HTTP_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
                .parse()?,
            database_url: std::env::var("DATABASE_URL")
                .context("DATABASE_URL must be set (e.g. in .env)")?,
            log_dir: std::env::var("LOG_DIR").unwrap_or_else(|_| "./logs".to_string()),
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        })
    }
}
