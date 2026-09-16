use crate::database::models::UserResponseDTO;
use crate::error::db::DbError;
use sqlx::FromRow;
use sqlx::types::Uuid;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(FromRow, Debug)]
pub struct UserEntity {
    pub id: Uuid,
    pub sub: String,
    pub provider: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Debug)]
pub struct NovelEntity {
    pub id: Uuid,
    pub title: String,
    pub cover_url: Option<String>,
    pub synopsis: Option<String>,
    pub author: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(FromRow, Debug)]
pub struct ChaptersEntity {
    pub id: Uuid,
    pub novel_id: Uuid,
    pub chapter_numbers: String,
    pub title: String,
    pub content_key: Option<String>,
    pub word_count: i64,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

impl TryFrom<UserEntity> for UserResponseDTO {
    type Error = DbError;
    fn try_from(entity: UserEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            id: entity.id,
            display_name: entity.display_name,
            username: entity.username,
            avatar_url: entity.avatar_url,
            email: entity.email,
            created_at: entity.created_at.to_utc(),
        })
    }
}
