use chrono::{DateTime, Utc};
use loglens_core::models::{EventGroup, Severity};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use crate::db::StorageError;

pub struct GroupRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> GroupRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list_groups(&self, workspace_id: Uuid) -> Result<Vec<EventGroup>, StorageError> {
        let rows = sqlx::query(
            r#"SELECT
                fingerprint,
                id as representative_event_id,
                message as sample_message,
                severity,
                COUNT(*) as occurrence_count,
                COUNT(DISTINCT source_id) as affected_sources_count,
                MIN(ingested_at) as first_seen,
                MAX(ingested_at) as last_seen
               FROM events
               WHERE workspace_id = ?
               GROUP BY fingerprint
               ORDER BY occurrence_count DESC"#
        )
        .bind(workspace_id.to_string())
        .fetch_all(self.pool)
        .await?;

        let mut groups = Vec::new();
        for r in rows {
            let fp: String = r.get("fingerprint");
            let ev_id_str: String = r.get("representative_event_id");
            let first_str: String = r.get("first_seen");
            let last_str: String = r.get("last_seen");
            let sev_str: String = r.get("severity");

            groups.push(EventGroup {
                fingerprint: fp,
                representative_event_id: Uuid::parse_str(&ev_id_str).unwrap(),
                occurrence_count: r.get::<i64, _>("occurrence_count") as u64,
                first_seen: DateTime::parse_from_rfc3339(&first_str).unwrap().with_timezone(&Utc),
                last_seen: DateTime::parse_from_rfc3339(&last_str).unwrap().with_timezone(&Utc),
                severity: Severity::from_str_loose(&sev_str),
                sample_message: r.get("sample_message"),
                affected_sources_count: r.get::<i64, _>("affected_sources_count") as u64,
                trend_buckets: vec![1, 2, 5, 3, 4],
            });
        }

        Ok(groups)
    }
}
