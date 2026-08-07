use crate::db::StorageError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearchRecord {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub query: String,
    pub created_at: DateTime<Utc>,
}

pub struct SavedSearchRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SavedSearchRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_search(&self, rec: &SavedSearchRecord) -> Result<(), StorageError> {
        sqlx::query!(
            r#"INSERT INTO saved_searches (id, workspace_id, user_id, name, query, created_at)
               VALUES (?, ?, ?, ?, ?, ?)"#,
            rec.id.to_string(),
            rec.workspace_id.to_string(),
            rec.user_id.to_string(),
            rec.name,
            rec.query,
            rec.created_at.to_rfc3339()
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_workspace_searches(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<SavedSearchRecord>, StorageError> {
        let rows = sqlx::query(
            "SELECT id, workspace_id, user_id, name, query, created_at FROM saved_searches WHERE workspace_id = ? ORDER BY created_at DESC",
        )
        .bind(workspace_id.to_string())
        .fetch_all(self.pool)
        .await?;

        let mut list = Vec::new();
        for r in rows {
            let id_str: String = r.get("id");
            let ws_str: String = r.get("workspace_id");
            let user_str: String = r.get("user_id");
            let created_str: String = r.get("created_at");

            list.push(SavedSearchRecord {
                id: Uuid::parse_str(&id_str).unwrap(),
                workspace_id: Uuid::parse_str(&ws_str).unwrap(),
                user_id: Uuid::parse_str(&user_str).unwrap(),
                name: r.get("name"),
                query: r.get("query"),
                created_at: DateTime::parse_from_rfc3339(&created_str)
                    .unwrap()
                    .with_timezone(&Utc),
            });
        }
        Ok(list)
    }

    pub async fn delete_search(&self, id: Uuid) -> Result<(), StorageError> {
        sqlx::query("DELETE FROM saved_searches WHERE id = ?")
            .bind(id.to_string())
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
