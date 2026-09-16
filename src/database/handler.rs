// TODO: Implement uh, database operations here
// Probably, fetching user profiles, storing user profiles
// Fetching novel metadata, and so on.

use crate::config::configs::Config;
use crate::database::entity::UserEntity;
use crate::database::models::UserResponseDTO;
use crate::error::auth::AuthError;
use crate::error::db::DbError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use tower_sessions::Session;

pub async fn get_user(State(state): State<Config>) -> Result<Json<UserResponseDTO>, DbError> {
    let user = sqlx::query_as!(
        UserEntity,
        "SELECT sub, id, provider, username, display_name, email, created_at, avatar_url  FROM users",
    ).fetch_optional(&state.db).await?;

    match user {
        Some(e) => {
            let response = UserResponseDTO::try_from(e)?;
            Ok(Json(response))
        }
        None => Err(DbError::NotFound),
    }
}

pub async fn delete_user(
    State(_state): State<Config>,
    session: Session,
) -> Result<StatusCode, AuthError> {
    let _user: String = session
        .get("authenticated_user")
        .await?
        .ok_or(AuthError::NotFound)?;

    Ok(StatusCode::NO_CONTENT)
}
