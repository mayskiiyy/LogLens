use chrono::{DateTime, Utc};
use loglens_core::models::LogSource;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use crate::db::StorageError;

pub struct SourceRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SourceRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_source(&self, source: &LogSource) -> Result<(), StorageError> {
        sqlx::query(
            r#"INSERT INTO sources (
                id, workspace_id, owner_id, display_name, original_path, source_type,
                parser_name, parser_confidence, detected_encoding, size_bytes, current_offset,
                line_count, event_count, imported_at, last_scanned_at, last_modified_at,
                checksum, live_watch_enabled, status, error_details
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(source.id.to_string())
        .bind(source.workspace_id.to_string())
        .bind(source.owner_id.map(|u| u.to_string()))
        .bind(&source.display_name)
        .bind(&source.original_path)
        .bind(&source.source_type)
        .bind(&source.parser_name)
        .bind(source.parser_confidence)
        .bind(&source.detected_encoding)
        .bind(source.size_bytes as i64)
        .bind(source.current_offset as i64)
        .bind(source.line_count as i64)
        .bind(source.event_count as i64)
        .bind(source.imported_at.to_rfc3339())
        .bind(source.last_scanned_at.map(|t| t.to_rfc3339()))
        .bind(source.last_modified_at.map(|t| t.to_rfc3339()))
        .bind(&source.checksum)
        .bind(if source.live_watch_enabled { 1 } else { 0 })
        .bind(&source.status)
        .bind(&source.error_details)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_sources(&self, workspace_id: Uuid) -> Result<Vec<LogSource>, StorageError> {
        let rows = sqlx::query("SELECT * FROM sources WHERE workspace_id = ? ORDER BY imported_at DESC")
            .bind(workspace_id.to_string())
            .fetch_all(self.pool)
            .await?;

        let mut list = Vec::new();
        for r in rows {
            let id_str: String = r.get("id");
            let ws_str: String = r.get("workspace_id");
            let owner_str: Option<String> = r.get("owner_id");
            let imp_str: String = r.get("imported_at");
            let scan_str: Option<String> = r.get("last_scanned_at");
            let mod_str: Option<String> = r.get("last_modified_at");

            list.push(LogSource {
                id: Uuid::parse_str(&id_str).unwrap(),
                owner_id: owner_str.map(|s| Uuid::parse_str(&s).unwrap()),
                workspace_id: Uuid::parse_str(&ws_str).unwrap(),
                display_name: r.get("display_name"),
                original_path: r.get("original_path"),
                source_type: r.get("source_type"),
                parser_name: r.get("parser_name"),
                parser_confidence: r.get("parser_confidence"),
                detected_encoding: r.get("detected_encoding"),
                size_bytes: r.get::<i64, _>("size_bytes") as u64,
                current_offset: r.get::<i64, _>("current_offset") as u64,
                line_count: r.get::<i64, _>("line_count") as u64,
                event_count: r.get::<i64, _>("event_count") as u64,
                imported_at: DateTime::parse_from_rfc3339(&imp_str).unwrap().with_timezone(&Utc),
                last_scanned_at: scan_str.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                last_modified_at: mod_str.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                checksum: r.get("checksum"),
                live_watch_enabled: r.get::<i32, _>("live_watch_enabled") == 1,
                status: r.get("status"),
                error_details: r.get("error_details"),
            });
        }
        Ok(list)
    }

    pub async fn delete_source(&self, source_id: Uuid, workspace_id: Uuid) -> Result<(), StorageError> {
        sqlx::query("DELETE FROM sources WHERE id = ? AND workspace_id = ?")
            .bind(source_id.to_string())
            .bind(workspace_id.to_string())
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
