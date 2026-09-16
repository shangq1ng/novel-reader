use axum::http::StatusCode;
use axum::response::IntoResponse;
use oauth2::basic::BasicErrorResponseType;
use oauth2::{HttpClientError, RequestTokenError, StandardErrorResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    TowerSessionError(#[from] tower_sessions::session::Error),
    #[error(transparent)]
    FailedToParseUrl(#[from] oauth2::url::ParseError),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let error = match self {
            Self::SqlxError(e) => {
                tracing::error!("Sqlx Error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::ReqwestError(e) => {
                tracing::error!("oauth2::reqwest error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::TowerSessionError(e) => {
                tracing::error!("Tower session error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::FailedToParseUrl(e) => {
                tracing::error!("Failed to parse URL: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::InternalServerError(e) => {
                tracing::error!("Internal Server Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        error.into_response()
    }
}


impl From<RequestTokenError<HttpClientError<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>> for AuthError {
    fn from(err: RequestTokenError<HttpClientError<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>) -> Self {
        AuthError::InternalServerError(err.into())
    }
}

