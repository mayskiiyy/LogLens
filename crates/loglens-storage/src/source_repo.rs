use crate::db::StorageError;
use chrono::{DateTime, Utc};
use loglens_core::models::LogSource;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub struct SourceRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SourceRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_source(&self, src: &LogSource) -> Result<(), StorageError> {
        sqlx::query!(
            r#"INSERT INTO sources (id, workspace_id, name, source_type, file_path, file_size, event_count, status, error_message, ingested_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            src.id.to_string(),
            src.workspace_id.to_string(),
            src.name,
            src.source_type,
            src.file_path,
            src.file_size as i64,
            src.event_count as i64,
            src.status,
            src.error_message,
            src.ingested_at.to_rfc3339()
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<LogSource>, StorageError> {
        let row = sqlx::query(
            "SELECT id, workspace_id, name, source_type, file_path, file_size, event_count, status, error_message, ingested_at FROM sources WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(self.pool)
        .await?;

        if let Some(r) = row {
            let id_str: String = r.get("id");
            let ws_str: String = r.get("workspace_id");
            let ingested_str: String = r.get("ingested_at");

            Ok(Some(LogSource {
                id: Uuid::parse_str(&id_str).unwrap(),
                workspace_id: Uuid::parse_str(&ws_str).unwrap(),
                name: r.get("name"),
                source_type: r.get("source_type"),
                file_path: r.get("file_path"),
                file_size: r.get::<i64, _>("file_size") as u64,
                event_count: r.get::<i64, _>("event_count") as u64,
                status: r.get("status"),
                error_message: r.get("error_message"),
                ingested_at: DateTime::parse_from_rfc3339(&ingested_str)
                    .unwrap()
                    .with_timezone(&Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_workspace_sources(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<LogSource>, StorageError> {
        let rows = sqlx::query(
            "SELECT id, workspace_id, name, source_type, file_path, file_size, event_count, status, error_message, ingested_at FROM sources WHERE workspace_id = ? ORDER BY ingested_at DESC",
        )
        .bind(workspace_id.to_string())
        .fetch_all(self.pool)
        .await?;

        let mut list = Vec::new();
        for r in rows {
            let id_str: String = r.get("id");
            let ws_str: String = r.get("workspace_id");
            let ingested_str: String = r.get("ingested_at");

            list.push(LogSource {
                id: Uuid::parse_str(&id_str).unwrap(),
                workspace_id: Uuid::parse_str(&ws_str).unwrap(),
                name: r.get("name"),
                source_type: r.get("source_type"),
                file_path: r.get("file_path"),
                file_size: r.get::<i64, _>("file_size") as u64,
                event_count: r.get::<i64, _>("event_count") as u64,
                status: r.get("status"),
                error_message: r.get("error_message"),
                ingested_at: DateTime::parse_from_rfc3339(&ingested_str)
                    .unwrap()
                    .with_timezone(&Utc),
            });
        }
        Ok(list)
    }

    pub async fn delete_source(&self, id: Uuid) -> Result<(), StorageError> {
        sqlx::query("DELETE FROM sources WHERE id = ?")
            .bind(id.to_string())
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
