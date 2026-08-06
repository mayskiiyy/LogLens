use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Trace,
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Fatal,
    Unknown,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Trace => "trace",
            Severity::Debug => "debug",
            Severity::Info => "info",
            Severity::Notice => "notice",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Critical => "critical",
            Severity::Fatal => "fatal",
            Severity::Unknown => "unknown",
        }
    }

    pub fn from_str_loose(s: &str) -> Self {
        let clean = s.trim().to_lowercase();
        match clean.as_str() {
            "trace" | "trc" | "verbose" => Severity::Trace,
            "debug" | "dbg" => Severity::Debug,
            "info" | "inf" | "information" => Severity::Info,
            "notice" => Severity::Notice,
            "warn" | "warning" | "wrn" => Severity::Warning,
            "err" | "error" => Severity::Error,
            "crit" | "critical" => Severity::Critical,
            "fatal" | "emerg" | "emergency" | "panic" => Severity::Fatal,
            _ => Severity::Unknown,
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogSource {
    pub id: Uuid,
    pub owner_id: Option<Uuid>,
    pub workspace_id: Uuid,
    pub display_name: String,
    pub original_path: String,
    pub source_type: String,
    pub parser_name: String,
    pub parser_confidence: f32,
    pub detected_encoding: String,
    pub size_bytes: u64,
    pub current_offset: u64,
    pub line_count: u64,
    pub event_count: u64,
    pub imported_at: DateTime<Utc>,
    pub last_scanned_at: Option<DateTime<Utc>>,
    pub last_modified_at: Option<DateTime<Utc>>,
    pub checksum: Option<String>,
    pub live_watch_enabled: bool,
    pub status: String,
    pub error_details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub source_id: Uuid,
    pub sequence_number: u64,
    pub line_start: u64,
    pub line_end: u64,
    pub byte_start: u64,
    pub byte_end: u64,
    pub parsed_timestamp: Option<DateTime<Utc>>,
    pub ingested_at: DateTime<Utc>,
    pub severity: Severity,
    pub target: Option<String>,
    pub message: String,
    pub stack_trace: Option<String>,
    pub structured_fields: HashMap<String, serde_json::Value>,
    pub raw: String,
    pub normalized_message: String,
    pub fingerprint: String,
    pub parser_name: String,
    pub warnings: Vec<String>,
    pub correlation_id: Option<String>,
    pub request_id: Option<String>,
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGroup {
    pub fingerprint: String,
    pub representative_event_id: Uuid,
    pub occurrence_count: u64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub severity: Severity,
    pub sample_message: String,
    pub affected_sources_count: u64,
    pub trend_buckets: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportProgress {
    pub import_id: Uuid,
    pub source_id: Uuid,
    pub bytes_processed: u64,
    pub total_bytes: u64,
    pub events_parsed: u64,
    pub percentage: f32,
    pub is_completed: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryFilter {
    pub search_query: Option<String>,
    pub severities: Vec<Severity>,
    pub source_ids: Vec<Uuid>,
    pub parser_names: Vec<String>,
    pub targets: Vec<String>,
    pub after: Option<DateTime<Utc>>,
    pub before: Option<DateTime<Utc>>,
    pub fingerprint: Option<String>,
    pub correlation_id: Option<String>,
    pub has_stack: Option<bool>,
    pub has_timestamp: Option<bool>,
    pub malformed_only: Option<bool>,
    pub limit: usize,
    pub offset: usize,
}
