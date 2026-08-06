use std::collections::HashMap;
use chrono::{DateTime, Utc};
use regex::Regex;
use std::sync::OnceLock;
use crate::models::Severity;
use crate::parsers::trait_def::{ParseError, ParsedLogEvent, Parser};

static RE_BRACKETED: OnceLock<Regex> = OnceLock::new();

fn get_bracketed_regex() -> &'static Regex {
    RE_BRACKETED.get_or_init(|| {
        Regex::new(r"^\[(?P<ts>[^\]]+)\]\s+\[(?P<level>[^\]]+)\]\s+(?:\[(?P<target>[^\]]+)\]\s+)?(?P<msg>.*)$").unwrap()
    })
}

pub struct BracketedTimestampParser;

impl BracketedTimestampParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for BracketedTimestampParser {
    fn name(&self) -> &'static str {
        "bracketed"
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
            if get_bracketed_regex().is_match(trimmed) {
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
        if let Some(caps) = get_bracketed_regex().captures(trimmed) {
            let ts_str = caps.name("ts").unwrap().as_str();
            let timestamp = DateTime::parse_from_rfc3339(ts_str)
                .map(|dt| dt.with_timezone(&Utc))
                .ok();

            let lvl_str = caps.name("level").map(|m| m.as_str()).unwrap_or("INFO");
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
            Err(ParseError::InvalidFormat {
                line: line_number,
                reason: "Does not match bracketed log format".to_string(),
            })
        }
    }
}
