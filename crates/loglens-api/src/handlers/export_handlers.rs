use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::Json;
use loglens_core::models::QueryFilter;
use loglens_storage::{Database, EventRepository, WorkspaceRepository};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::dto::ExportRequest;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct ExportQuery {
    pub workspace_id: Uuid,
}

fn sanitize_csv_field(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.starts_with('=')
        || trimmed.starts_with('+')
        || trimmed.starts_with('-')
        || trimmed.starts_with('@')
    {
        format!("'{}", trimmed)
    } else {
        trimmed.to_string()
    }
}

pub async fn export_events_handler(
    State(db): State<Database>,
    AuthUser { user }: AuthUser,
    Query(q): Query<ExportQuery>,
    Json(req): Json<ExportRequest>,
) -> Result<Response, ApiError> {
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

    let filter = QueryFilter {
        search_query: req.query,
        limit: 1000,
        ..Default::default()
    };

    let event_repo = EventRepository::new(db.pool());
    let events = event_repo
        .query_events(q.workspace_id, &filter)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let format = req.format.to_lowercase();
    let (content_type, body_str) = match format.as_str() {
        "csv" => {
            let mut out = String::from("id,timestamp,severity,target,message\n");
            for ev in events {
                out.push_str(&format!(
                    "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                    ev.id,
                    ev.ingested_at.to_rfc3339(),
                    ev.severity,
                    sanitize_csv_field(ev.target.as_deref().unwrap_or("")),
                    sanitize_csv_field(&ev.message).replace('"', "\"\"")
                ));
            }
            ("text/csv", out)
        }
        "jsonl" => {
            let mut out = String::new();
            for ev in events {
                if let Ok(line) = serde_json::to_string(&ev) {
                    out.push_str(&line);
                    out.push('\n');
                }
            }
            ("application/x-ndjson", out)
        }
        _ => {
            // default JSON
            let out = serde_json::to_string_pretty(&events).unwrap_or_default();
            ("application/json", out)
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        content_type.parse().unwrap(),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"export.{}\"", format)
            .parse()
            .unwrap(),
    );

    Ok((headers, body_str).into_response())
}
