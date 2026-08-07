use crate::db::StorageError;
use chrono::{DateTime, Utc};
use loglens_core::models::{LogEvent, QueryFilter, Severity};
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct EventRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> EventRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_events_batch(&self, events: &[LogEvent]) -> Result<(), StorageError> {
        if events.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;

        for event in events {
            let structured_json =
                serde_json::to_string(&event.structured_fields).unwrap_or_default();
            let warnings_json = serde_json::to_string(&event.warnings).unwrap_or_default();
            let parsed_ts_str = event.parsed_timestamp.map(|t| t.to_rfc3339());

            sqlx::query(
                r#"INSERT INTO events (
                    id, workspace_id, source_id, sequence_number, line_start, line_end,
                    byte_start, byte_end, parsed_timestamp, ingested_at, severity, target,
                    message, stack_trace, structured_fields, raw, normalized_message,
                    fingerprint, parser_name, warnings, correlation_id, request_id, trace_id
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            )
            .bind(event.id.to_string())
            .bind(event.workspace_id.to_string())
            .bind(event.source_id.to_string())
            .bind(event.sequence_number as i64)
            .bind(event.line_start as i64)
            .bind(event.line_end as i64)
            .bind(event.byte_start as i64)
            .bind(event.byte_end as i64)
            .bind(parsed_ts_str)
            .bind(event.ingested_at.to_rfc3339())
            .bind(event.severity.as_str())
            .bind(&event.target)
            .bind(&event.message)
            .bind(&event.stack_trace)
            .bind(structured_json)
            .bind(&event.raw)
            .bind(&event.normalized_message)
            .bind(&event.fingerprint)
            .bind(&event.parser_name)
            .bind(warnings_json)
            .bind(&event.correlation_id)
            .bind(&event.request_id)
            .bind(&event.trace_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn query_events(
        &self,
        workspace_id: Uuid,
        filter: &QueryFilter,
    ) -> Result<Vec<LogEvent>, StorageError> {
        let mut query_sql = String::from("SELECT e.* FROM events e ");

        if let Some(ref q) = filter.search_query {
            if !q.trim().is_empty() {
                query_sql.push_str("JOIN events_fts fts ON e.id = fts.event_id ");
            }
        }

        query_sql.push_str("WHERE e.workspace_id = ? ");

        if let Some(ref q) = filter.search_query {
            if !q.trim().is_empty() {
                query_sql.push_str("AND events_fts MATCH ? ");
            }
        }

        if !filter.severities.is_empty() {
            let sevs: Vec<String> = filter
                .severities
                .iter()
                .map(|s| format!("'{}'", s.as_str()))
                .collect();
            query_sql.push_str(&format!("AND e.severity IN ({}) ", sevs.join(",")));
        }

        if !filter.source_ids.is_empty() {
            let ids: Vec<String> = filter.source_ids.iter().map(|id| format!("'{}'", id)).collect();
            query_sql.push_str(&format!("AND e.source_id IN ({}) ", ids.join(",")));
        }

        if let Some(ref fp) = filter.fingerprint {
            query_sql.push_str(&format!("AND e.fingerprint = '{}' ", fp));
        }

        if let Some(ref after) = filter.after {
            query_sql.push_str(&format!("AND e.ingested_at >= '{}' ", after.to_rfc3339()));
        }

        if let Some(ref before) = filter.before {
            query_sql.push_str(&format!("AND e.ingested_at <= '{}' ", before.to_rfc3339()));
        }

        query_sql.push_str("ORDER BY e.sequence_number DESC LIMIT ? OFFSET ?");

        let limit = if filter.limit == 0 { 100 } else { filter.limit };
        let mut query = sqlx::query(&query_sql).bind(workspace_id.to_string());

        if let Some(ref q) = filter.search_query {
            if !q.trim().is_empty() {
                query = query.bind(q);
            }
        }

        query = query.bind(limit as i64).bind(filter.offset as i64);

        let rows = query.fetch_all(self.pool).await?;
        let mut events = Vec::new();

        for r in rows {
            let id_str: String = r.get("id");
            let ws_str: String = r.get("workspace_id");
            let src_str: String = r.get("source_id");
            let parsed_ts_str: Option<String> = r.get("parsed_timestamp");
            let ingested_str: String = r.get("ingested_at");
            let sev_str: String = r.get("severity");
            let struct_str: String = r.get("structured_fields");
            let warn_str: String = r.get("warnings");

            let structured: HashMap<String, serde_json::Value> =
                serde_json::from_str(&struct_str).unwrap_or_default();
            let warnings: Vec<String> = serde_json::from_str(&warn_str).unwrap_or_default();

            events.push(LogEvent {
                id: Uuid::parse_str(&id_str).unwrap(),
                workspace_id: Uuid::parse_str(&ws_str).unwrap(),
                source_id: Uuid::parse_str(&src_str).unwrap(),
                sequence_number: r.get::<i64, _>("sequence_number") as u64,
                line_start: r.get::<i64, _>("line_start") as u64,
                line_end: r.get::<i64, _>("line_end") as u64,
                byte_start: r.get::<i64, _>("byte_start") as u64,
                byte_end: r.get::<i64, _>("byte_end") as u64,
                parsed_timestamp: parsed_ts_str.map(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .unwrap()
                        .with_timezone(&Utc)
                }),
                ingested_at: DateTime::parse_from_rfc3339(&ingested_str)
                    .unwrap()
                    .with_timezone(&Utc),
                severity: Severity::from_str_loose(&sev_str),
                target: r.get("target"),
                message: r.get("message"),
                stack_trace: r.get("stack_trace"),
                structured_fields: structured,
                raw: r.get("raw"),
                normalized_message: r.get("normalized_message"),
                fingerprint: r.get("fingerprint"),
                parser_name: r.get("parser_name"),
                warnings,
                correlation_id: r.get("correlation_id"),
                request_id: r.get("request_id"),
                trace_id: r.get("trace_id"),
            });
        }

        Ok(events)
    }

    pub async fn count_events(&self, workspace_id: Uuid) -> Result<i64, StorageError> {
        let res: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events WHERE workspace_id = ?")
            .bind(workspace_id.to_string())
            .fetch_one(self.pool)
            .await?;
        Ok(res.0)
    }
}
