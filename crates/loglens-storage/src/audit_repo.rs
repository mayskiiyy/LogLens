use crate::db::StorageError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogRecord {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub details: String,
    pub timestamp: DateTime<Utc>,
}

pub struct AuditRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> AuditRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn log_action(&self, rec: &AuditLogRecord) -> Result<(), StorageError> {
        let user_id_str = rec.user_id.map(|u| u.to_string());
        sqlx::query!(
            r#"INSERT INTO audit_logs (id, user_id, action, details, timestamp)
               VALUES (?, ?, ?, ?, ?)"#,
            rec.id.to_string(),
            user_id_str,
            rec.action,
            rec.details,
            rec.timestamp.to_rfc3339()
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_recent_logs(&self, limit: i64) -> Result<Vec<AuditLogRecord>, StorageError> {
        let rows = sqlx::query(
            "SELECT id, user_id, action, details, timestamp FROM audit_logs ORDER BY timestamp DESC LIMIT ?",
        )
        .bind(limit)
        .fetch_all(self.pool)
        .await?;

        let mut list = Vec::new();
        for r in rows {
            let id_str: String = r.get("id");
            let user_str: Option<String> = r.get("user_id");
            let ts_str: String = r.get("timestamp");

            list.push(AuditLogRecord {
                id: Uuid::parse_str(&id_str).unwrap(),
                user_id: user_str.map(|s| Uuid::parse_str(&s).unwrap()),
                action: r.get("action"),
                details: r.get("details"),
                timestamp: DateTime::parse_from_rfc3339(&ts_str)
                    .unwrap()
                    .with_timezone(&Utc),
            });
        }
        Ok(list)
    }
}
