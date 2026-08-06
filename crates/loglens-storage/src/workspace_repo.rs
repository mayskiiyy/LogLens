use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use crate::db::StorageError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRecord {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}

pub struct WorkspaceRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> WorkspaceRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_workspace(&self, ws: &WorkspaceRecord) -> Result<(), StorageError> {
        sqlx::query!(
            r#"INSERT INTO workspaces (id, name, owner_id, created_at)
               VALUES (?, ?, ?, ?)"#,
            ws.id.to_string(),
            ws.name,
            ws.owner_id.to_string(),
            ws.created_at.to_rfc3339()
        )
        .execute(self.pool)
        .await?;

        // Add owner as workspace member with 'owner' role
        sqlx::query!(
            r#"INSERT INTO workspace_members (workspace_id, user_id, role)
               VALUES (?, ?, 'owner')"#,
            ws.id.to_string(),
            ws.owner_id.to_string()
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkspaceRecord>, StorageError> {
        let row = sqlx::query("SELECT id, name, owner_id, created_at FROM workspaces WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(self.pool)
            .await?;

        if let Some(r) = row {
            let id_str: String = r.get("id");
            let owner_str: String = r.get("owner_id");
            let created_str: String = r.get("created_at");

            Ok(Some(WorkspaceRecord {
                id: Uuid::parse_str(&id_str).unwrap(),
                name: r.get("name"),
                owner_id: Uuid::parse_str(&owner_str).unwrap(),
                created_at: DateTime::parse_from_rfc3339(&created_str).unwrap().with_timezone(&Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_user_workspaces(&self, user_id: Uuid) -> Result<Vec<WorkspaceRecord>, StorageError> {
        let rows = sqlx::query(
            r#"SELECT w.id, w.name, w.owner_id, w.created_at
               FROM workspaces w
               JOIN workspace_members wm ON w.id = wm.workspace_id
               WHERE wm.user_id = ?"#
        )
        .bind(user_id.to_string())
        .fetch_all(self.pool)
        .await?;

        let mut list = Vec::new();
        for r in rows {
            let id_str: String = r.get("id");
            let owner_str: String = r.get("owner_id");
            let created_str: String = r.get("created_at");

            list.push(WorkspaceRecord {
                id: Uuid::parse_str(&id_str).unwrap(),
                name: r.get("name"),
                owner_id: Uuid::parse_str(&owner_str).unwrap(),
                created_at: DateTime::parse_from_rfc3339(&created_str).unwrap().with_timezone(&Utc),
            });
        }
        Ok(list)
    }

    pub async fn verify_user_access(&self, workspace_id: Uuid, user_id: Uuid) -> Result<bool, StorageError> {
        let row = sqlx::query("SELECT 1 FROM workspace_members WHERE workspace_id = ? AND user_id = ?")
            .bind(workspace_id.to_string())
            .bind(user_id.to_string())
            .fetch_optional(self.pool)
            .await?;
        Ok(row.is_some())
    }
}
