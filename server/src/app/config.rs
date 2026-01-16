use anyhow::Context;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub http_addr: SocketAddr,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            http_addr: "127.0.0.1:3000".parse()?,
            database_url: std::env::var("DATABASE_URL")
                .context("DATABASE_URL must be set (e.g. in .env)")?,
        })
    }
}
