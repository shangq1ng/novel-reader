use std::env::VarError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConfigError {
    #[error("EnvError: {0}")]
    EnvError(#[from] VarError),
    #[error("Migration Error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}

impl IntoResponse for ConfigError {
    fn into_response(self) -> Response {
        let config_error = match self {
            Self::MigrationError(err) => {
                tracing::error!("Migration error: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            },
            Self::EnvError(e) => {
                tracing::error!("EnvError: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            },
        };
        config_error.into_response()
    }
}

impl From<anyhow::Error> for ConfigError {
    fn from(err: anyhow::Error) -> Self {
        err.into()
    }
}