use crate::db::StorageError;
use chrono::{DateTime, Utc};
use loglens_core::models::{EventGroup, Severity};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub struct GroupRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> GroupRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn upsert_group(&self, group: &EventGroup) -> Result<(), StorageError> {
        sqlx::query!(
            r#"INSERT INTO event_groups (
                id, workspace_id, fingerprint, title, severity, count,
                first_seen, last_seen, sample_stack_trace
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(workspace_id, fingerprint) DO UPDATE SET
                count = count + excluded.count,
                last_seen = excluded.last_seen,
                sample_stack_trace = COALESCE(event_groups.sample_stack_trace, excluded.sample_stack_trace)"#,
            group.id.to_string(),
            group.workspace_id.to_string(),
            group.fingerprint,
            group.title,
            group.severity.as_str(),
            group.count as i64,
            group.first_seen.to_rfc3339(),
            group.last_seen.to_rfc3339(),
            group.sample_stack_trace
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_workspace_groups(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<EventGroup>, StorageError> {
        let rows = sqlx::query(
            r#"SELECT id, workspace_id, fingerprint, title, severity, count, first_seen, last_seen, sample_stack_trace
               FROM event_groups
               WHERE workspace_id = ?
               ORDER BY count DESC"#,
        )
        .bind(workspace_id.to_string())
        .fetch_all(self.pool)
        .await?;

        let mut groups = Vec::new();
        for r in rows {
            let id_str: String = r.get("id");
            let ws_str: String = r.get("workspace_id");
            let sev_str: String = r.get("severity");
            let first_str: String = r.get("first_seen");
            let last_str: String = r.get("last_seen");

            groups.push(EventGroup {
                id: Uuid::parse_str(&id_str).unwrap(),
                workspace_id: Uuid::parse_str(&ws_str).unwrap(),
                fingerprint: r.get("fingerprint"),
                title: r.get("title"),
                severity: Severity::from_str_loose(&sev_str),
                count: r.get::<i64, _>("count") as u64,
                first_seen: DateTime::parse_from_rfc3339(&first_str)
                    .unwrap()
                    .with_timezone(&Utc),
                last_seen: DateTime::parse_from_rfc3339(&last_str)
                    .unwrap()
                    .with_timezone(&Utc),
                sample_stack_trace: r.get("sample_stack_trace"),
            });
        }

        Ok(groups)
    }
}
