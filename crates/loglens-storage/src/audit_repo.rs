use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use crate::db::StorageError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventRecord {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

pub struct AuditRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> AuditRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn record_event(&self, event: &AuditEventRecord) -> Result<(), StorageError> {
        let details_json = serde_json::to_string(&event.details).unwrap_or_default();
        sqlx::query!(
            r#"INSERT INTO audit_events (id, user_id, action, resource_type, resource_id, details, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            event.id.to_string(),
            event.user_id.map(|u| u.to_string()),
            event.action,
            event.resource_type,
            event.resource_id,
            details_json,
            event.created_at.to_rfc3339()
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
