use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tower_sessions_redis_store::RedisStoreError;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Redis error: `{0}`")]
    RedisError(#[from] RedisStoreError),
    #[error(transparent)]
    TowerSessionRedisStoreError(#[from] tower_sessions_redis_store::fred::error::Error),
    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error)
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        let session_error = match self {
            SessionError::InternalServerError(e) => {
                tracing::error!("Unexpected Error: `{0}`", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            SessionError::TowerSessionRedisStoreError(e) => {
                tracing::error!("TowerSessionStoreError: `{0}`", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::RedisError(e) => {
                tracing::error!(" Redis Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        session_error.into_response()
    }
}
