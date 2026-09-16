use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tower_sessions_redis_store::RedisStoreError;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Redis error: `{0}`")]
    RedisError(#[from] RedisStoreError),
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        let session_error = match self {
            Self::RedisError(e) => {
                tracing::error!(" Redis Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        session_error.into_response()
    }
}

impl From<anyhow::Error> for SessionError {
    fn from(error: anyhow::Error) -> Self {
        error.into()
    }
}

impl From<tower_sessions_redis_store::fred::error::Error> for SessionError {
    fn from(error: tower_sessions_redis_store::fred::error::Error) -> Self {
        error.into()
    }
}
