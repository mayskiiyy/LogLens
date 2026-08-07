use axum::extract::{Multipart, Path, Query, State};
use axum::Json;
use loglens_core::models::LogSource;
use loglens_core::StreamingLogReader;
use loglens_storage::{Database, EventRepository, SourceRepository, WorkspaceRepository};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct SourceQuery {
    pub workspace_id: Uuid,
}

pub async fn list_sources_handler(
    State(db): State<Database>,
    AuthUser { user }: AuthUser,
    Query(q): Query<SourceQuery>,
) -> Result<Json<Vec<LogSource>>, ApiError> {
    let ws_repo = WorkspaceRepository::new(db.pool());
    if !ws_repo
        .verify_user_access(q.workspace_id, user.id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
    {
        return Err(ApiError::Forbidden(
            "Access to workspace denied".to_string(),
        ));
    }

    let src_repo = SourceRepository::new(db.pool());
    let sources = src_repo
        .list_workspace_sources(q.workspace_id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(sources))
}

pub async fn upload_source_handler(
    State(db): State<Database>,
    AuthUser { user }: AuthUser,
    Query(q): Query<SourceQuery>,
    mut multipart: Multipart,
) -> Result<Json<LogSource>, ApiError> {
    let ws_repo = WorkspaceRepository::new(db.pool());
    if !ws_repo
        .verify_user_access(q.workspace_id, user.id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
    {
        return Err(ApiError::Forbidden(
            "Access to workspace denied".to_string(),
        ));
    }

    let temp_dir = std::env::var("LOGLENS_TEMP_DIR").unwrap_or_else(|_| "./data/tmp".to_string());
    tokio::fs::create_dir_all(&temp_dir).await.ok();

    let mut temp_path = None;
    let mut original_name = String::from("uploaded.log");

    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            if let Some(filename) = field.file_name() {
                original_name = std::path::Path::new(filename)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
            }

            let file_id = Uuid::new_v4();
            let dest_path = std::path::Path::new(&temp_dir).join(format!("{}.tmp", file_id));
            let data = field
                .bytes()
                .await
                .map_err(|e| ApiError::BadRequest(format!("File upload read error: {}", e)))?;

            tokio::fs::write(&dest_path, &data)
                .await
                .map_err(|e| ApiError::Internal(format!("Failed to write temp file: {}", e)))?;
            temp_path = Some(dest_path);
            break;
        }
    }

    let path = temp_path.ok_or_else(|| {
        ApiError::BadRequest("No file provided in multipart body".to_string())
    })?;

    let src_id = Uuid::new_v4();
    let reader = StreamingLogReader::new(Default::default());
    let (mut source, events) = reader
        .process_file(&path, q.workspace_id, src_id, None)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to process log file: {}", e)))?;

    source.name = original_name;

    let src_repo = SourceRepository::new(db.pool());
    src_repo
        .create_source(&source)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let event_repo = EventRepository::new(db.pool());
    event_repo
        .insert_events_batch(&events)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    tokio::fs::remove_file(path).await.ok();

    Ok(Json(source))
}

pub async fn delete_source_handler(
    State(db): State<Database>,
    AuthUser { user }: AuthUser,
    Query(q): Query<SourceQuery>,
    Path(source_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let ws_repo = WorkspaceRepository::new(db.pool());
    if !ws_repo
        .verify_user_access(q.workspace_id, user.id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
    {
        return Err(ApiError::Forbidden(
            "Access to workspace denied".to_string(),
        ));
    }

    let src_repo = SourceRepository::new(db.pool());
    src_repo
        .delete_source(source_id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({ "status": "deleted" })))
}
