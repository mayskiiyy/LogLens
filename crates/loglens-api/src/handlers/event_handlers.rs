use axum::extract::{Query, State};
use axum::response::sse::{Event, Sse};
use axum::Json;
use futures_util::stream::Stream;
use loglens_core::models::{LogEvent, QueryFilter, Severity};
use loglens_storage::{Database, EventRepository, WorkspaceRepository};
use serde::Deserialize;
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct EventQueryParams {
    pub workspace_id: Uuid,
    pub search: Option<String>,
    pub level: Option<String>,
    pub source_id: Option<Uuid>,
    pub fingerprint: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub async fn query_events_handler(
    State(db): State<Database>,
    AuthUser { user }: AuthUser,
    Query(q): Query<EventQueryParams>,
) -> Result<Json<Vec<LogEvent>>, ApiError> {
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

    let mut filter = QueryFilter {
        search_query: q.search,
        fingerprint: q.fingerprint,
        limit: q.limit.unwrap_or(100),
        offset: q.offset.unwrap_or(0),
        ..Default::default()
    };

    if let Some(lvl) = q.level {
        filter.severities.push(Severity::from_str_loose(&lvl));
    }
    if let Some(src_id) = q.source_id {
        filter.source_ids.push(src_id);
    }

    let event_repo = EventRepository::new(db.pool());
    let events = event_repo
        .query_events(q.workspace_id, &filter)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(events))
}

pub async fn event_sse_stream_handler(
    State(_db): State<Database>,
    AuthUser { user: _ }: AuthUser,
    Query(q): Query<EventQueryParams>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(
        Duration::from_secs(2),
    ))
    .map(move |_| {
        let msg = format!("{{\"type\":\"ping\",\"workspace_id\":\"{}\"}}", q.workspace_id);
        Ok(Event::default().data(msg))
    });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}
