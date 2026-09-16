use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("SQLX Error: {0}")]
    DatabaseError(String),
    #[error("Conflict Error")]
    Conflict,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),
}

impl IntoResponse for DbError {
    fn into_response(self) -> axum::response::Response {
        let error = match self {
            Self::Conflict => StatusCode::CONFLICT,
            Self::DatabaseError(e) => {
                tracing::error!("Database error: {}", e);
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

impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::NotFound,
            sqlx::Error::Database(db_err)
                if db_err.is_foreign_key_violation()
                    || db_err.is_unique_violation()
                    || db_err.is_unique_violation() =>
            {
                tracing::error!("Internal Server Error: {db_err}");
                Self::Conflict
            }
            _ => Self::InternalServerError(e.into()),
        }
    }
}
