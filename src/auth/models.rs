use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

// Provider Response (eg; Google, Discord, etc)
#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderResponse {
    pub state: String,
    pub code: String,
}

// Provider's response when we make a request for user info
#[derive(Debug, Deserialize)]
pub struct DiscordUserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserResponse {
    pub sub: String,
    pub name: String,
    pub picture: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
}
