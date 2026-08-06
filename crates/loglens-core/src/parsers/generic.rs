use std::collections::HashMap;
use chrono::{DateTime, Utc};
use regex::Regex;
use std::sync::OnceLock;
use crate::models::Severity;
use crate::parsers::trait_def::{ParseError, ParsedLogEvent, Parser};

static RE_GENERIC: OnceLock<Regex> = OnceLock::new();

fn get_generic_regex() -> &'static Regex {
    RE_GENERIC.get_or_init(|| {
        Regex::new(r"^(?P<ts>\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})?)\s+(?:\[(?P<level1>[A-Z]+)\]|(?P<level2>[A-Z]+))\s+(?:(?P<target>[a-zA-Z0-9_\-\.]+)\s+)?(?P<msg>.*)$").unwrap()
    })
}

pub struct GenericLogParser;

impl GenericLogParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for GenericLogParser {
    fn name(&self) -> &'static str {
        "generic"
    }

    fn detect(&self, sample: &str) -> f32 {
        let mut matches = 0;
        let mut total = 0;
        for line in sample.lines().take(20) {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            total += 1;
            if get_generic_regex().is_match(trimmed) {
                matches += 1;
            }
        }
        if total == 0 {
            0.0
        } else {
            matches as f32 / total as f32
        }
    }

    fn parse(&self, input: &str, line_number: u64) -> Result<ParsedLogEvent, ParseError> {
        let trimmed = input.trim();
        if let Some(caps) = get_generic_regex().captures(trimmed) {
            let ts_str = caps.name("ts").unwrap().as_str();
            let timestamp = DateTime::parse_from_rfc3339(ts_str)
                .map(|dt| dt.with_timezone(&Utc))
                .ok();

            let lvl_str = caps.name("level1").or_else(|| caps.name("level2")).map(|m| m.as_str()).unwrap_or("INFO");
            let severity = Severity::from_str_loose(lvl_str);

            let target = caps.name("target").map(|m| m.as_str().to_string());
            let message = caps.name("msg").map(|m| m.as_str().to_string()).unwrap_or_default();

            Ok(ParsedLogEvent {
                timestamp,
                severity,
                target,
                message,
                structured_fields: HashMap::new(),
                correlation_id: None,
                request_id: None,
                trace_id: None,
            })
        } else {
            // Fallback for unstructured line
            let severity = if trimmed.to_lowercase().contains("error") {
                Severity::Error
            } else if trimmed.to_lowercase().contains("warn") {
                Severity::Warning
            } else {
                Severity::Unknown
            };

            Ok(ParsedLogEvent {
                timestamp: None,
                severity,
                target: None,
                message: trimmed.to_string(),
                structured_fields: HashMap::new(),
                correlation_id: None,
                request_id: None,
                trace_id: None,
            })
        }
    }
}
