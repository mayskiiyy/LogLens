use regex::Regex;
use std::sync::OnceLock;

static RE_BEARER: OnceLock<Regex> = OnceLock::new();
static RE_JWT: OnceLock<Regex> = OnceLock::new();
static RE_API_KEY: OnceLock<Regex> = OnceLock::new();
static RE_CREDIT_CARD: OnceLock<Regex> = OnceLock::new();

fn get_bearer_regex() -> &'static Regex {
    RE_BEARER.get_or_init(|| {
        Regex::new(r"(?i)bearer\s+[a-zA-Z0-9\-\._~\+\/]+=*").unwrap()
    })
}

fn get_jwt_regex() -> &'static Regex {
    RE_JWT.get_or_init(|| {
        Regex::new(r"eyJ[a-zA-Z0-9_-]{10,}\.eyJ[a-zA-Z0-9_-]{10,}\.[a-zA-Z0-9_-]{10,}").unwrap()
    })
}

fn get_api_key_regex() -> &'static Regex {
    RE_API_KEY.get_or_init(|| {
        Regex::new(r"(?i)(api[_-]?key|secret|password|passwd|auth[_-]?token)\s*[:=]\s*['""]?[a-zA-Z0-9_\-]{8,}['""]?").unwrap()
    })
}

fn get_cc_regex() -> &'static Regex {
    RE_CREDIT_CARD.get_or_init(|| {
        Regex::new(r"\b(?:\d[ -]*?){13,16}\b").unwrap()
    })
}

pub fn redact_text(input: &str) -> String {
    let s = get_bearer_regex().replace_all(input, "Bearer [REDACTED]");
    let s = get_jwt_regex().replace_all(&s, "[REDACTED_JWT]");
    let s = get_api_key_regex().replace_all(&s, "$1: [REDACTED]");
    let s = get_cc_regex().replace_all(&s, "[REDACTED_CARD]");
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redaction() {
        let text = "Authorization: Bearer secrettoken123 and api_key=sk-live-1234567890";
        let redacted = redact_text(text);
        assert!(redacted.contains("Bearer [REDACTED]"));
        assert!(redacted.contains("api_key: [REDACTED]"));
    }
}
