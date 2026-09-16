use sqlx::FromRow;
use sqlx::types::Uuid;
use sqlx::types::chrono::NaiveDateTime;

#[derive(FromRow, Debug)]
pub struct UserEntity {
    pub id: Uuid,
    pub provider: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(FromRow, Debug)]
pub struct NovelEntity {
    pub id: Uuid,
    pub title: String,
    pub cover_url: Option<String>,
    pub synopsis: Option<String>,
    pub author: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(FromRow, Debug)]
pub struct ChaptersEntity {
    pub id: Uuid,
    pub novel_id: Uuid,
    pub chapter_numbers: String,
    pub title: String,
    pub content_key: Option<String>,
    pub word_count: i64,
    pub published_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}
