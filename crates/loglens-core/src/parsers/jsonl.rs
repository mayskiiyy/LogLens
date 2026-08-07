use crate::models::Severity;
use crate::parsers::trait_def::{ParseError, ParsedLogEvent, Parser};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

pub struct JsonLinesParser;

impl JsonLinesParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_timestamp(v: &Value) -> Option<DateTime<Utc>> {
        match v {
            Value::String(s) => DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&Utc))
                .ok(),
            Value::Number(n) => {
                if let Some(secs) = n.as_i64() {
                    DateTime::from_timestamp(secs, 0)
                } else if let Some(f) = n.as_f64() {
                    let secs = f.floor() as i64;
                    let nsecs = ((f - secs as f64) * 1_000_000_000.0) as u32;
                    DateTime::from_timestamp(secs, nsecs)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Parser for JsonLinesParser {
    fn name(&self) -> &'static str {
        "jsonl"
    }

    fn detect(&self, sample: &str) -> f32 {
        let mut json_count = 0;
        let mut total = 0;
        for line in sample.lines().take(20) {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            total += 1;
            if (trimmed.starts_with('{') && trimmed.ends_with('}'))
                && serde_json::from_str::<Value>(trimmed).is_ok()
            {
                json_count += 1;
            }
        }
        if total == 0 {
            0.0
        } else {
            json_count as f32 / total as f32
        }
    }

    fn parse(&self, input: &str, line_number: u64) -> Result<ParsedLogEvent, ParseError> {
        let obj: HashMap<String, Value> =
            serde_json::from_str(input.trim()).map_err(|e| ParseError::InvalidFormat {
                line: line_number,
                reason: format!("JSON parse error: {}", e),
            })?;

        let mut timestamp = None;
        let ts_keys = ["timestamp", "time", "ts", "@timestamp", "datetime", "date"];
        for k in &ts_keys {
            if let Some(v) = obj.get(*k) {
                if let Some(parsed) = Self::parse_timestamp(v) {
                    timestamp = Some(parsed);
                    break;
                }
            }
        }

        let mut severity = Severity::Unknown;
        let lvl_keys = ["level", "severity", "log_level", "lvl", "status"];
        for k in &lvl_keys {
            if let Some(v) = obj.get(*k) {
                if let Some(s) = v.as_str() {
                    severity = Severity::from_str_loose(s);
                    break;
                }
            }
        }

        let mut message = String::new();
        let msg_keys = ["message", "msg", "text", "log"];
        for k in &msg_keys {
            if let Some(v) = obj.get(*k) {
                if let Some(s) = v.as_str() {
                    message = s.to_string();
                    break;
                }
            }
        }
        if message.is_empty() {
            message = input.trim().to_string();
        }

        let mut target = None;
        let target_keys = ["logger", "target", "module", "name", "component"];
        for k in &target_keys {
            if let Some(v) = obj.get(*k) {
                if let Some(s) = v.as_str() {
                    target = Some(s.to_string());
                    break;
                }
            }
        }

        let req_id = obj
            .get("req_id")
            .or_else(|| obj.get("request_id"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let corr_id = obj
            .get("correlation_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let trace_id = obj
            .get("trace_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(ParsedLogEvent {
            timestamp,
            severity,
            target,
            message,
            structured_fields: obj,
            correlation_id: corr_id,
            request_id: req_id,
            trace_id,
        })
    }
}
