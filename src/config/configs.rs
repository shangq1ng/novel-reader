use crate::error::config::ConfigError;
use anyhow::Context;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Debug, Clone)]
pub struct Config {
    pub db: PgPool,
    pub port: u16,
}

impl Config {
    pub async fn new() -> Result<Self, ConfigError> {
        let db_url = env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
        let port = env::var("PORT")?
            .parse::<u16>()
            .context("PORT is not a number")?;
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .context("Failed to connect to database")?;

        Ok(Self { db: pool, port })
    }
}
