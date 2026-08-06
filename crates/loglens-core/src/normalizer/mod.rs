use regex::Regex;
use std::sync::OnceLock;
use crate::models::Severity;

static RE_UUID: OnceLock<Regex> = OnceLock::new();
static RE_IPV4: OnceLock<Regex> = OnceLock::new();
static RE_IPV6: OnceLock<Regex> = OnceLock::new();
static RE_HEX: OnceLock<Regex> = OnceLock::new();
static RE_EMAIL: OnceLock<Regex> = OnceLock::new();
static RE_ISO_TIMESTAMP: OnceLock<Regex> = OnceLock::new();
static RE_URL: OnceLock<Regex> = OnceLock::new();
static RE_NUMERIC_ID: OnceLock<Regex> = OnceLock::new();

fn get_uuid_regex() -> &'static Regex {
    RE_UUID.get_or_init(|| {
        Regex::new(r"(?i)[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}").unwrap()
    })
}

fn get_ipv4_regex() -> &'static Regex {
    RE_IPV4.get_or_init(|| {
        Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap()
    })
}

fn get_ipv6_regex() -> &'static Regex {
    RE_IPV6.get_or_init(|| {
        Regex::new(r"(?i)\b(?:[0-9a-f]{1,4}:){7}[0-9a-f]{1,4}\b").unwrap()
    })
}

fn get_hex_regex() -> &'static Regex {
    RE_HEX.get_or_init(|| {
        Regex::new(r"0x[0-9a-fA-F]+").unwrap()
    })
}

fn get_email_regex() -> &'static Regex {
    RE_EMAIL.get_or_init(|| {
        Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap()
    })
}

fn get_iso_ts_regex() -> &'static Regex {
    RE_ISO_TIMESTAMP.get_or_init(|| {
        Regex::new(r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})?").unwrap()
    })
}

fn get_url_regex() -> &'static Regex {
    RE_URL.get_or_init(|| {
        Regex::new(r"https?://[^\s]+").unwrap()
    })
}

fn get_num_id_regex() -> &'static Regex {
    RE_NUMERIC_ID.get_or_init(|| {
        Regex::new(r"\b\d{5,}\b").unwrap()
    })
}

pub fn normalize_message(msg: &str) -> String {
    let s = get_uuid_regex().replace_all(msg, "<UUID>");
    let s = get_iso_ts_regex().replace_all(&s, "<TIMESTAMP>");
    let s = get_ipv4_regex().replace_all(&s, "<IP>");
    let s = get_ipv6_regex().replace_all(&s, "<IP>");
    let s = get_hex_regex().replace_all(&s, "<HEX>");
    let s = get_email_regex().replace_all(&s, "<EMAIL>");
    let s = get_url_regex().replace_all(&s, "<URL>");
    let s = get_num_id_regex().replace_all(&s, "<NUM>");
    s.trim().to_string()
}

pub fn compute_fingerprint(
    severity: Severity,
    target: Option<&str>,
    normalized_msg: &str,
    stack_trace: Option<&str>,
) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(severity.as_str().as_bytes());
    hasher.update(b"|");
    hasher.update(target.unwrap_or("").as_bytes());
    hasher.update(b"|");
    hasher.update(normalized_msg.as_bytes());

    if let Some(st) = stack_trace {
        let first_lines: Vec<&str> = st.lines().take(3).collect();
        hasher.update(b"|");
        hasher.update(first_lines.join("\n").as_bytes());
    }

    hasher.finalize().to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let raw = "Connection failed for user 18422 at 192.168.1.100 with UUID 550e8400-e29b-41d4-a716-446655440000";
        let norm = normalize_message(raw);
        assert!(norm.contains("<NUM>"));
        assert!(norm.contains("<IP>"));
        assert!(norm.contains("<UUID>"));
    }

    #[test]
    fn test_fingerprint_stability() {
        let fp1 = compute_fingerprint(Severity::Error, Some("auth"), "User <NUM> failed", None);
        let fp2 = compute_fingerprint(Severity::Error, Some("auth"), "User <NUM> failed", None);
        assert_eq!(fp1, fp2);
    }
}
