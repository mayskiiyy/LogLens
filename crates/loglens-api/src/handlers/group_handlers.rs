use axum::extract::{Query, State};
use axum::Json;
use loglens_core::models::EventGroup;
use loglens_storage::{Database, GroupRepository, WorkspaceRepository};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct GroupQuery {
    pub workspace_id: Uuid,
}

pub async fn list_groups_handler(
    State(db): State<Database>,
    AuthUser { user }: AuthUser,
    Query(q): Query<GroupQuery>,
) -> Result<Json<Vec<EventGroup>>, ApiError> {
    let ws_repo = WorkspaceRepository::new(db.pool());
    if !ws_repo.verify_user_access(q.workspace_id, user.id).await.map_err(|e| ApiError::Internal(e.to_string()))? {
        return Err(ApiError::Forbidden("Access to workspace denied".to_string()));
    }

    let group_repo = GroupRepository::new(db.pool());
    let groups = group_repo.list_groups(q.workspace_id).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(groups))
}
