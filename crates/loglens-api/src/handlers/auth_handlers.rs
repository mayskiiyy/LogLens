use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{Duration, Utc};
use loglens_storage::{Database, SessionRecord, UserRecord, UserRepository, WorkspaceRecord, WorkspaceRepository};
use uuid::Uuid;

use crate::auth::{hash_password, hash_token, verify_password, AuthUser};
use crate::dto::{BootstrapAdminRequest, LoginRequest, UserResponse};
use crate::error::ApiError;

pub async fn bootstrap_handler(
    State(db): State<Database>,
    Json(req): Json<BootstrapAdminRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let user_repo = UserRepository::new(db.pool());
    let user_count = user_repo.count_users().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    if user_count > 0 {
        return Err(ApiError::BadRequest("System has already been bootstrapped".to_string()));
    }

    let password_hash = hash_password(&req.password)?;
    let now = Utc::now();
    let user = UserRecord {
        id: Uuid::new_v4(),
        email: req.email.clone(),
        password_hash,
        role: "admin".to_string(),
        created_at: now,
        updated_at: now,
    };

    user_repo.create_user(&user).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    // Create default workspace for admin
    let ws_repo = WorkspaceRepository::new(db.pool());
    let default_ws = WorkspaceRecord {
        id: Uuid::new_v4(),
        name: "Default Workspace".to_string(),
        owner_id: user.id,
        created_at: now,
    };
    ws_repo.create_workspace(&default_ws).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        role: user.role,
    }))
}

pub async fn login_handler(
    State(db): State<Database>,
    Json(req): Json<LoginRequest>,
) -> Result<Response, ApiError> {
    let user_repo = UserRepository::new(db.pool());
    let user = user_repo
        .find_by_email(&req.email)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::Unauthorized("Invalid email or password".to_string()))?;

    if !verify_password(&req.password, &user.password_hash) {
        return Err(ApiError::Unauthorized("Invalid email or password".to_string()));
    }

    let token_raw = Uuid::new_v4().to_string();
    let token_hash = hash_token(&token_raw);
    let now = Utc::now();
    let expires_at = now + Duration::days(7);

    let session = SessionRecord {
        id: Uuid::new_v4(),
        user_id: user.id,
        token_hash,
        expires_at,
        created_at: now,
    };

    user_repo.create_session(&session).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let cookie = format!(
        "loglens_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
        token_raw,
        7 * 24 * 60 * 60
    );

    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::SET_COOKIE, cookie.parse().unwrap());

    let body = Json(UserResponse {
        id: user.id,
        email: user.email,
        role: user.role,
    });

    Ok((headers, body).into_response())
}

pub async fn me_handler(AuthUser { user }: AuthUser) -> Json<UserResponse> {
    Json(UserResponse {
        id: user.id,
        email: user.email,
        role: user.role,
    })
}
