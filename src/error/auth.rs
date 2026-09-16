use axum::http::StatusCode;
use axum::response::IntoResponse;
use oauth2::basic::BasicErrorResponseType;
use oauth2::{HttpClientError, RequestTokenError, StandardErrorResponse};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum AuthError {
    #[error(transparent)]
    FailedToParseUrl(#[from] oauth2::url::ParseError),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("Invalid request")]
    InvalidRequest,
    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let error = match self {
            Self::FailedToParseUrl(e) => {
                tracing::error!("Failed to parse URL: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::InvalidCredentials => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::InvalidRequest => StatusCode::BAD_REQUEST,
            Self::InternalServerError(e) => {
                tracing::error!("Internal Server Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        error.into_response()
    }
}

impl From<tower_sessions::session::Error> for AuthError {
    fn from(err: tower_sessions::session::Error) -> Self {
        err.into()
    }
}

impl
    From<
        RequestTokenError<
            HttpClientError<oauth2::reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    > for AuthError
{
    fn from(
        err: RequestTokenError<
            HttpClientError<oauth2::reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ) -> Self {
        err.into()
    }
}

impl From<oauth2::reqwest::Error> for AuthError {
    fn from(err: oauth2::reqwest::Error) -> Self {
        err.into()
    }
}

impl From<sqlx::Error> for AuthError {
    fn from(err: sqlx::Error) -> Self {
        err.into()
    }
}
