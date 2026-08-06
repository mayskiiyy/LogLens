use axum::extract::State;
use axum::Json;
use chrono::Utc;
use loglens_storage::{Database, WorkspaceRecord, WorkspaceRepository};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::dto::CreateWorkspaceRequest;
use crate::error::ApiError;

pub async fn list_workspaces_handler(
    State(db): State<Database>,
    AuthUser { user }: AuthUser,
) -> Result<Json<Vec<WorkspaceRecord>>, ApiError> {
    let ws_repo = WorkspaceRepository::new(db.pool());
    let list = ws_repo.list_user_workspaces(user.id).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(list))
}

pub async fn create_workspace_handler(
    State(db): State<Database>,
    AuthUser { user }: AuthUser,
    Json(req): Json<CreateWorkspaceRequest>,
) -> Result<Json<WorkspaceRecord>, ApiError> {
    let ws_repo = WorkspaceRepository::new(db.pool());
    let ws = WorkspaceRecord {
        id: Uuid::new_v4(),
        name: req.name,
        owner_id: user.id,
        created_at: Utc::now(),
    };
    ws_repo.create_workspace(&ws).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(ws))
}
