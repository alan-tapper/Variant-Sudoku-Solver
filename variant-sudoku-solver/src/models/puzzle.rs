use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::Json};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub type Board = Vec<Vec<u8>>; // 9x9 grid, 0 = empty

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Puzzle {
    pub id: Uuid,
    pub name: String,
    pub author: Option<String>,
    pub variant: String,
    pub difficulty: Option<i16>,
    pub board: Json<Board>,
    pub variant_data: Option<Json<serde_json::Value>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Puzzle {
    pub async fn list(pool: &PgPool, limit: i64) -> sqlx::Result<Vec<Puzzle>> {
        sqlx::query_as::<_, Puzzle>("SELECT * FROM puzzles ORDER BY created_at DESC LIMIT $1")
            .bind(limit)
            .fetch_all(pool)
            .await
    }

    pub async fn fetch_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Puzzle> {
        sqlx::query_as::<_, Puzzle>("SELECT * FROM puzzles WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }
}
