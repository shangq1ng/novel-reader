use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

// What users can make request w
#[derive(Debug, Deserialize)]
pub struct UserRequestDTO {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
}
// What server responds with
#[derive(Debug, Serialize)]
pub struct UserResponseDTO {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
}
