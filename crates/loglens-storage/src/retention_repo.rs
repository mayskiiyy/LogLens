use crate::db::StorageError;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct RetentionRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> RetentionRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn purge_events_older_than(
        &self,
        workspace_id: Uuid,
        cutoff: DateTime<Utc>,
    ) -> Result<u64, StorageError> {
        let res =
            sqlx::query("DELETE FROM events WHERE workspace_id = ? AND ingested_at < ?")
                .bind(workspace_id.to_string())
                .bind(cutoff.to_rfc3339())
                .execute(self.pool)
                .await?;
        Ok(res.rows_affected())
    }

    pub async fn purge_sources_older_than(
        &self,
        workspace_id: Uuid,
        cutoff: DateTime<Utc>,
    ) -> Result<u64, StorageError> {
        let res =
            sqlx::query("DELETE FROM sources WHERE workspace_id = ? AND ingested_at < ?")
                .bind(workspace_id.to_string())
                .bind(cutoff.to_rfc3339())
                .execute(self.pool)
                .await?;
        Ok(res.rows_affected())
    }
}
