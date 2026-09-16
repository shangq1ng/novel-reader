use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::env::VarError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum S3Error {
    #[error("EnvError: {0}")]
    EnvError(#[from] VarError),

    #[error("S3 Error: {0}")]
    S3(#[from] s3::error::S3Error),

    #[error("Object not found: {0}")]
    ObjectNotFound(String),

    #[error("Unexpected status {status} for {key}")]
    Status { status: u16, key: String },

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for S3Error {
    fn into_response(self) -> Response {
        let error = match self {
            Self::UnexpectedError(e) => {
                tracing::error!("Unexpected error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            },
            Self::ObjectNotFound(e) => {
                tracing::error!("Object not found: {}", e);
                StatusCode::NOT_FOUND
            }
            Self::S3(e) => {
                tracing::error!("S3 Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Status { status, key } => {
                tracing::error!("Unexpected status {status} for {key}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => StatusCode::BAD_REQUEST,
        };
        error.into_response()
    }
}
