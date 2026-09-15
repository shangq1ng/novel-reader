use crate::error::config::ConfigError;
use anyhow::Context;
use s3::Bucket;
use s3::creds::Credentials;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Config {
    pub db: PgPool,
    pub port: u16,
    pub host: Arc<String>,
}

impl Config {
    pub async fn new() -> Result<Self, ConfigError> {
        let db_url = env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
        let port = env::var("PORT")?
            .parse::<u16>()
            .context("PORT is not a number")?;
        let host = Arc::new(env::var("HOST").context("HOST is not set")?);
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .context("Failed to connect to database")?;

        Ok(Self {
            db: pool,
            port,
            host,
        })
    }

    pub async fn migrate(&self) -> Result<(), ConfigError> {
        sqlx::migrate!()
            .run(&self.db)
            .await
            .context("Migration failed")?;
        Ok(())
    }
}
