use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::HeaderMap;
use loglens_storage::{Database, UserRecord, UserRepository};
use uuid::Uuid;

use crate::error::ApiError;

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| ApiError::Internal(format!("Password hashing error: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed = match PasswordHash::new(hash) {
        Ok(p) => p,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

pub fn hash_token(token: &str) -> String {
    blake3::hash(token.as_bytes()).to_hex().to_string()
}

#[derive(Clone)]
pub struct AuthUser {
    pub user: UserRecord,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    Database: axum::extract::FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let db = Database::from_ref(state);
        let headers: &HeaderMap = &parts.headers;

        let cookie_header = headers
            .get(axum::http::header::COOKIE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or_default();

        let mut token = None;
        for cookie in cookie_header.split(';') {
            let parts: Vec<&str> = cookie.trim().split('=').collect();
            if parts.len() == 2 && parts[0] == "loglens_session" {
                token = Some(parts[1]);
                break;
            }
        }

        let token_str = token.ok_or_else(|| ApiError::Unauthorized("Missing session cookie".to_string()))?;
        let token_hash = hash_token(token_str);

        let user_repo = UserRepository::new(db.pool());
        let session = user_repo
            .find_session_by_hash(&token_hash)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .ok_or_else(|| ApiError::Unauthorized("Invalid or expired session".to_string()))?;

        let user = user_repo
            .find_by_id(session.user_id)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .ok_or_else(|| ApiError::Unauthorized("User not found".to_string()))?;

        Ok(AuthUser { user })
    }
}
