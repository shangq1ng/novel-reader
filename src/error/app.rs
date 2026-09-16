use crate::error::auth::AuthError;
use crate::error::config::ConfigError;
use crate::error::db::DbError;
use crate::error::s3::S3Error;
use crate::error::session::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // Sub Errors
    #[error(transparent)]
    DbError(#[from] DbError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    SessionError(#[from] SessionError),
    #[error(transparent)]
    S3Error(#[from] S3Error),
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
}
