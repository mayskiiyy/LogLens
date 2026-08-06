use std::collections::HashMap;
use chrono::{DateTime, Utc};
use regex::Regex;
use std::sync::OnceLock;
use crate::models::Severity;
use crate::parsers::trait_def::{ParseError, ParsedLogEvent, Parser};

static RE_NGINX_ACCESS: OnceLock<Regex> = OnceLock::new();
static RE_NGINX_ERROR: OnceLock<Regex> = OnceLock::new();

fn get_nginx_access_regex() -> &'static Regex {
    RE_NGINX_ACCESS.get_or_init(|| {
        Regex::new(r#"^(?P<ip>\S+)\s+\S+\s+(?P<user>\S+)\s+\[(?P<ts>[^\]]+)\]\s+"(?P<req>[^"]*)"\s+(?P<status>\d{3})\s+(?P<bytes>\d+)\s+"(?P<ref>[^"]*)"\s+"(?P<ua>[^"]*)"$"#).unwrap()
    })
}

fn get_nginx_error_regex() -> &'static Regex {
    RE_NGINX_ERROR.get_or_init(|| {
        Regex::new(r"^(?P<ts>\d{4}/\d{2}/\d{2}\s+\d{2}:\d{2}:\d{2})\s+\[(?P<level>\w+)\]\s+(?P<pid>\d+)#(?P<tid>\d+):\s+(?P<msg>.*)$").unwrap()
    })
}

pub struct NginxAccessParser;
pub struct NginxErrorParser;

impl NginxAccessParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for NginxAccessParser {
    fn name(&self) -> &'static str {
        "nginx_access"
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
            if get_nginx_access_regex().is_match(trimmed) {
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
        if let Some(caps) = get_nginx_access_regex().captures(trimmed) {
            let ip = caps.name("ip").unwrap().as_str();
            let user = caps.name("user").unwrap().as_str();
            let ts_str = caps.name("ts").unwrap().as_str();
            let req = caps.name("req").unwrap().as_str();
            let status: u16 = caps.name("status").unwrap().as_str().parse().unwrap_or(200);
            let bytes: u64 = caps.name("bytes").unwrap().as_str().parse().unwrap_or(0);
            let referer = caps.name("ref").unwrap().as_str();
            let ua = caps.name("ua").unwrap().as_str();

            let timestamp = DateTime::parse_from_str(ts_str, "%d/%b/%Y:%H:%M:%S %z")
                .map(|dt| dt.with_timezone(&Utc))
                .ok();

            let severity = if status >= 500 {
                Severity::Error
            } else if status >= 400 {
                Severity::Warning
            } else {
                Severity::Info
            };

            let mut fields = HashMap::new();
            fields.insert("client_ip".to_string(), serde_json::json!(ip));
            fields.insert("auth_user".to_string(), serde_json::json!(user));
            fields.insert("request".to_string(), serde_json::json!(req));
            fields.insert("status_code".to_string(), serde_json::json!(status));
            fields.insert("bytes_sent".to_string(), serde_json::json!(bytes));
            fields.insert("referer".to_string(), serde_json::json!(referer));
            fields.insert("user_agent".to_string(), serde_json::json!(ua));

            Ok(ParsedLogEvent {
                timestamp,
                severity,
                target: Some("nginx_access".to_string()),
                message: format!("{} {} {}", ip, req, status),
                structured_fields: fields,
                correlation_id: None,
                request_id: None,
                trace_id: None,
            })
        } else {
            Err(ParseError::InvalidFormat {
                line: line_number,
                reason: "Does not match Nginx access format".to_string(),
            })
        }
    }
}

impl NginxErrorParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for NginxErrorParser {
    fn name(&self) -> &'static str {
        "nginx_error"
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
            if get_nginx_error_regex().is_match(trimmed) {
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
        if let Some(caps) = get_nginx_error_regex().captures(trimmed) {
            let ts_str = caps.name("ts").unwrap().as_str();
            let timestamp = NaiveDateTime::parse_from_str(ts_str, "%Y/%m/%d %H:%M:%S")
                .map(|ndt| DateTime::from_naive_utc_and_offset(ndt, Utc))
                .ok();

            let lvl_str = caps.name("level").unwrap().as_str();
            let severity = Severity::from_str_loose(lvl_str);
            let message = caps.name("msg").unwrap().as_str().to_string();

            Ok(ParsedLogEvent {
                timestamp,
                severity,
                target: Some("nginx_error".to_string()),
                message,
                structured_fields: HashMap::new(),
                correlation_id: None,
                request_id: None,
                trace_id: None,
            })
        } else {
            Err(ParseError::InvalidFormat {
                line: line_number,
                reason: "Does not match Nginx error format".to_string(),
            })
        }
    }
}

use chrono::NaiveDateTime;
