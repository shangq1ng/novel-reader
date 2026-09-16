use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::types::chrono::NaiveDateTime;

// Provider Response (eg; Google, Discord, etc)
#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderResponse {
    pub state: String,
    pub code: String,
}

// Provider's response when we make a request for user info
#[derive(Debug, Deserialize)]
pub struct ProviderUserResponse {
    pub sub: String,
    pub username: Option<String>,
    pub global_name: Option<String>,
    pub name: Option<String>,
    pub display_name: String,
    pub picture: Option<String>,
    pub email: String,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
}
// What users can make request w
#[derive(Debug, Serialize)]
pub struct UserRequestDTO {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub created_at: NaiveDateTime,
}
// What server responds with
#[derive(Debug, Serialize)]
pub struct UserResponseDTO {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub created_at: NaiveDateTime,
}
