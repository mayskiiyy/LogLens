use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use crate::db::StorageError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearchRecord {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub query_string: String,
    pub created_at: DateTime<Utc>,
}

pub struct SavedSearchRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SavedSearchRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, item: &SavedSearchRecord) -> Result<(), StorageError> {
        sqlx::query!(
            r#"INSERT INTO saved_searches (id, workspace_id, user_id, name, query_string, created_at)
               VALUES (?, ?, ?, ?, ?, ?)"#,
            item.id.to_string(),
            item.workspace_id.to_string(),
            item.user_id.to_string(),
            item.name,
            item.query_string,
            item.created_at.to_rfc3339()
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn list(&self, workspace_id: Uuid) -> Result<Vec<SavedSearchRecord>, StorageError> {
        let rows = sqlx::query("SELECT id, workspace_id, user_id, name, query_string, created_at FROM saved_searches WHERE workspace_id = ? ORDER BY created_at DESC")
            .bind(workspace_id.to_string())
            .fetch_all(self.pool)
            .await?;

        let mut list = Vec::new();
        for r in rows {
            let id_str: String = r.get("id");
            let ws_str: String = r.get("workspace_id");
            let u_str: String = r.get("user_id");
            let created_str: String = r.get("created_at");

            list.push(SavedSearchRecord {
                id: Uuid::parse_str(&id_str).unwrap(),
                workspace_id: Uuid::parse_str(&ws_str).unwrap(),
                user_id: Uuid::parse_str(&u_str).unwrap(),
                name: r.get("name"),
                query_string: r.get("query_string"),
                created_at: DateTime::parse_from_rfc3339(&created_str).unwrap().with_timezone(&Utc),
            });
        }
        Ok(list)
    }

    pub async fn delete(&self, id: Uuid, workspace_id: Uuid) -> Result<(), StorageError> {
        sqlx::query("DELETE FROM saved_searches WHERE id = ? AND workspace_id = ?")
            .bind(id.to_string())
            .bind(workspace_id.to_string())
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
