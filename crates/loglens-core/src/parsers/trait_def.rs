use std::collections::HashMap;
use chrono::{DateTime, Utc};
use thiserror::Error;
use crate::models::Severity;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Failed to parse line {line}: {reason}")]
    InvalidFormat { line: u64, reason: String },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct ParsedLogEvent {
    pub timestamp: Option<DateTime<Utc>>,
    pub severity: Severity,
    pub target: Option<String>,
    pub message: String,
    pub structured_fields: HashMap<String, serde_json::Value>,
    pub correlation_id: Option<String>,
    pub request_id: Option<String>,
    pub trace_id: Option<String>,
}

pub trait Parser: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, sample: &str) -> f32;
    fn parse(&self, input: &str, line_number: u64) -> Result<ParsedLogEvent, ParseError>;
    fn supports_multiline(&self) -> bool {
        false
    }
    fn reset(&mut self) {}
}
