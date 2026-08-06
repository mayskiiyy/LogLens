use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use crate::db::StorageError;

pub struct RetentionRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> RetentionRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn purge_events_older_than(&self, cutoff: DateTime<Utc>) -> Result<u64, StorageError> {
        let res = sqlx::query("DELETE FROM events WHERE ingested_at < ?")
            .bind(cutoff.to_rfc3339())
            .execute(self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    pub async fn purge_expired_sessions(&self) -> Result<u64, StorageError> {
        let res = sqlx::query("DELETE FROM sessions WHERE expires_at < ?")
            .bind(Utc::now().to_rfc3339())
            .execute(self.pool)
            .await?;
        Ok(res.rows_affected())
    }
}
