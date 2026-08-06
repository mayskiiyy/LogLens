use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use crate::db::StorageError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

pub struct UserRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: &UserRecord) -> Result<(), StorageError> {
        sqlx::query!(
            r#"INSERT INTO users (id, email, password_hash, role, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?)"#,
            user.id.to_string(),
            user.email,
            user.password_hash,
            user.role,
            user.created_at.to_rfc3339(),
            user.updated_at.to_rfc3339()
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserRecord>, StorageError> {
        let row = sqlx::query("SELECT id, email, password_hash, role, created_at, updated_at FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(self.pool)
            .await?;

        if let Some(r) = row {
            let id_str: String = r.get("id");
            let created_str: String = r.get("created_at");
            let updated_str: String = r.get("updated_at");

            Ok(Some(UserRecord {
                id: Uuid::parse_str(&id_str).unwrap(),
                email: r.get("email"),
                password_hash: r.get("password_hash"),
                role: r.get("role"),
                created_at: DateTime::parse_from_rfc3339(&created_str).unwrap().with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&updated_str).unwrap().with_timezone(&Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserRecord>, StorageError> {
        let row = sqlx::query("SELECT id, email, password_hash, role, created_at, updated_at FROM users WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(self.pool)
            .await?;

        if let Some(r) = row {
            let id_str: String = r.get("id");
            let created_str: String = r.get("created_at");
            let updated_str: String = r.get("updated_at");

            Ok(Some(UserRecord {
                id: Uuid::parse_str(&id_str).unwrap(),
                email: r.get("email"),
                password_hash: r.get("password_hash"),
                role: r.get("role"),
                created_at: DateTime::parse_from_rfc3339(&created_str).unwrap().with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&updated_str).unwrap().with_timezone(&Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn count_users(&self) -> Result<i64, StorageError> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(self.pool)
            .await?;
        Ok(count.0)
    }

    pub async fn create_session(&self, session: &SessionRecord) -> Result<(), StorageError> {
        sqlx::query!(
            r#"INSERT INTO sessions (id, user_id, token_hash, expires_at, created_at)
               VALUES (?, ?, ?, ?, ?)"#,
            session.id.to_string(),
            session.user_id.to_string(),
            session.token_hash,
            session.expires_at.to_rfc3339(),
            session.created_at.to_rfc3339()
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_session_by_hash(&self, token_hash: &str) -> Result<Option<SessionRecord>, StorageError> {
        let row = sqlx::query("SELECT id, user_id, token_hash, expires_at, created_at FROM sessions WHERE token_hash = ? AND expires_at > ?")
            .bind(token_hash)
            .bind(Utc::now().to_rfc3339())
            .fetch_optional(self.pool)
            .await?;

        if let Some(r) = row {
            let id_str: String = r.get("id");
            let user_id_str: String = r.get("user_id");
            let exp_str: String = r.get("expires_at");
            let created_str: String = r.get("created_at");

            Ok(Some(SessionRecord {
                id: Uuid::parse_str(&id_str).unwrap(),
                user_id: Uuid::parse_str(&user_id_str).unwrap(),
                token_hash: r.get("token_hash"),
                expires_at: DateTime::parse_from_rfc3339(&exp_str).unwrap().with_timezone(&Utc),
                created_at: DateTime::parse_from_rfc3339(&created_str).unwrap().with_timezone(&Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_session(&self, id: Uuid) -> Result<(), StorageError> {
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(id.to_string())
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
