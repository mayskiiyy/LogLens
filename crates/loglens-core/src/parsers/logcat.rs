use std::collections::HashMap;
use chrono::{Datelike, DateTime, TimeZone, Utc};
use regex::Regex;
use std::sync::OnceLock;
use crate::models::Severity;
use crate::parsers::trait_def::{ParseError, ParsedLogEvent, Parser};

static RE_LOGCAT: OnceLock<Regex> = OnceLock::new();

fn get_logcat_regex() -> &'static Regex {
    RE_LOGCAT.get_or_init(|| {
        Regex::new(r"^(?P<month>\d{2})-(?P<day>\d{2})\s+(?P<time>\d{2}:\d{2}:\d{2}\.\d{3})\s+(?P<pid>\d+)\s+(?P<tid>\d+)\s+(?P<level>[VDIWEF])\s+(?P<tag>[^:]+):\s+(?P<msg>.*)$").unwrap()
    })
}

pub struct LogcatParser;

impl LogcatParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for LogcatParser {
    fn name(&self) -> &'static str {
        "logcat"
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
            if get_logcat_regex().is_match(trimmed) {
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
        if let Some(caps) = get_logcat_regex().captures(trimmed) {
            let month: u32 = caps.name("month").unwrap().as_str().parse().unwrap_or(1);
            let day: u32 = caps.name("day").unwrap().as_str().parse().unwrap_or(1);
            let time_str = caps.name("time").unwrap().as_str();

            let year = Utc::now().year();
            let ts_full = format!("{:04}-{:02}-{:02}T{}Z", year, month, day, time_str);
            let timestamp = DateTime::parse_from_rfc3339(&ts_full)
                .map(|dt| dt.with_timezone(&Utc))
                .ok();

            let lvl_char = caps.name("level").unwrap().as_str();
            let severity = match lvl_char {
                "V" => Severity::Trace,
                "D" => Severity::Debug,
                "I" => Severity::Info,
                "W" => Severity::Warning,
                "E" => Severity::Error,
                "F" => Severity::Fatal,
                _ => Severity::Unknown,
            };

            let pid = caps.name("pid").unwrap().as_str();
            let tid = caps.name("tid").unwrap().as_str();
            let tag = caps.name("tag").unwrap().as_str();
            let message = caps.name("msg").unwrap().as_str().to_string();

            let mut fields = HashMap::new();
            fields.insert("pid".to_string(), serde_json::json!(pid));
            fields.insert("tid".to_string(), serde_json::json!(tid));
            fields.insert("tag".to_string(), serde_json::json!(tag));

            Ok(ParsedLogEvent {
                timestamp,
                severity,
                target: Some(tag.to_string()),
                message,
                structured_fields: fields,
                correlation_id: None,
                request_id: None,
                trace_id: None,
            })
        } else {
            Err(ParseError::InvalidFormat {
                line: line_number,
                reason: "Does not match Logcat format".to_string(),
            })
        }
    }
}
