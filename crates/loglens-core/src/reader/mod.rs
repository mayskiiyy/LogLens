use std::io::BufRead;
use std::path::Path;
use chrono::Utc;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::models::{ImportProgress, LogEvent, LogSource};
use crate::multiline::{MultilineAssembler, MultilineConfig};
use crate::normalizer::{compute_fingerprint, normalize_message};
use crate::parsers::ParserRegistry;
use crate::redaction::redact_text;

#[derive(Debug, Clone)]
pub struct ReaderLimits {
    pub max_line_bytes: usize,
    pub max_event_bytes: usize,
}

impl Default for ReaderLimits {
    fn default() -> Self {
        Self {
            max_line_bytes: 1024 * 1024,      // 1 MB line max
            max_event_bytes: 8 * 1024 * 1024,  // 8 MB event max
        }
    }
}

pub struct StreamingLogReader {
    limits: ReaderLimits,
}

impl StreamingLogReader {
    pub fn new(limits: ReaderLimits) -> Self {
        Self { limits }
    }

    pub async fn process_file<P: AsRef<Path>>(
        &self,
        file_path: P,
        workspace_id: Uuid,
        source_id: Uuid,
        progress_tx: Option<mpsc::Sender<ImportProgress>>,
    ) -> Result<(LogSource, Vec<LogEvent>), Box<dyn std::error::Error + Send + Sync>> {
        let path_ref = file_path.as_ref();
        let metadata = tokio::fs::metadata(path_ref).await?;
        let file_size = metadata.len();

        let file = std::fs::File::open(path_ref)?;
        let mut reader = std::io::BufReader::new(file);

        // 1. Detect parser from sample
        let mut sample_buf = Vec::new();
        let sample_len = reader.by_ref().take(32768).read_to_end(&mut sample_buf)?;
        let sample_str = String::from_utf8_lossy(&sample_buf);

        let registry = ParserRegistry::new();
        let (parser, confidence) = registry.detect_best_parser(&sample_str);

        // Reset file reader to start
        use std::io::Seek;
        let mut file = std::fs::File::open(path_ref)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        let mut reader = std::io::BufReader::new(file);

        let mut assembler = MultilineAssembler::new(MultilineConfig::default());
        let mut events = Vec::new();
        let mut current_offset: u64 = 0;
        let mut line_number: u64 = 0;
        let mut seq_num: u64 = 0;

        let import_id = Uuid::new_v4();

        loop {
            let mut line_buf = String::new();
            let bytes_read = reader.read_line(&mut line_buf)?;
            if bytes_read == 0 {
                break;
            }

            line_number += 1;
            let start_offset = current_offset;
            current_offset += bytes_read as u64;

            let clean_line = if line_buf.len() > self.limits.max_line_bytes {
                let mut truncated = line_buf[..self.limits.max_line_bytes].to_string();
                truncated.push_str("...[TRUNCATED]");
                truncated
            } else {
                line_buf.trim_end_matches(['\r', '\n']).to_string()
            };

            if let Some(assembled) = assembler.push_line(line_number, start_offset, clean_line) {
                seq_num += 1;
                let event = self.build_event(
                    workspace_id,
                    source_id,
                    seq_num,
                    assembled,
                    parser.as_ref(),
                );
                events.push(event);
            }

            if line_number % 500 == 0 {
                if let Some(ref tx) = progress_tx {
                    let progress = ImportProgress {
                        import_id,
                        source_id,
                        bytes_processed: current_offset,
                        total_bytes: file_size,
                        events_parsed: events.len() as u64,
                        percentage: if file_size > 0 {
                            (current_offset as f32 / file_size as f32) * 100.0
                        } else {
                            100.0
                        },
                        is_completed: false,
                        error: None,
                    };
                    let _ = tx.send(progress).await;
                }
            }
        }

        if let Some(assembled) = assembler.finish() {
            seq_num += 1;
            let event = self.build_event(
                workspace_id,
                source_id,
                seq_num,
                assembled,
                parser.as_ref(),
            );
            events.push(event);
        }

        let source = LogSource {
            id: source_id,
            owner_id: None,
            workspace_id,
            display_name: path_ref
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            original_path: path_ref.to_string_lossy().to_string(),
            source_type: "file".to_string(),
            parser_name: parser.name().to_string(),
            parser_confidence: confidence,
            detected_encoding: "UTF-8".to_string(),
            size_bytes: file_size,
            current_offset,
            line_count: line_number,
            event_count: events.len() as u64,
            imported_at: Utc::now(),
            last_scanned_at: Some(Utc::now()),
            last_modified_at: Some(Utc::now()),
            checksum: None,
            live_watch_enabled: false,
            status: "ready".to_string(),
            error_details: None,
        };

        if let Some(ref tx) = progress_tx {
            let _ = tx
                .send(ImportProgress {
                    import_id,
                    source_id,
                    bytes_processed: current_offset,
                    total_bytes: file_size,
                    events_parsed: events.len() as u64,
                    percentage: 100.0,
                    is_completed: true,
                    error: None,
                })
                .await;
        }

        Ok((source, events))
    }

    fn build_event(
        &self,
        workspace_id: Uuid,
        source_id: Uuid,
        sequence_number: u64,
        assembled: crate::multiline::AssembledEvent,
        parser: &dyn crate::parsers::Parser,
    ) -> LogEvent {
        let parsed_res = parser.parse(&assembled.main_line, assembled.line_start);
        let (timestamp, severity, target, msg, structured, corr_id, req_id, trace_id) = match parsed_res {
            Ok(p) => (
                p.timestamp,
                p.severity,
                p.target,
                p.message,
                p.structured_fields,
                p.correlation_id,
                p.request_id,
                p.trace_id,
            ),
            Err(_) => (
                None,
                crate::models::Severity::Unknown,
                None,
                assembled.main_line.clone(),
                std::collections::HashMap::new(),
                None,
                None,
                None,
            ),
        };

        let raw_joined = assembled.raw_lines.join("\n");
        let raw_redacted = redact_text(&raw_joined);
        let normalized = normalize_message(&msg);
        let fingerprint = compute_fingerprint(
            severity,
            target.as_deref(),
            &normalized,
            assembled.stack_trace.as_deref(),
        );

        LogEvent {
            id: Uuid::new_v4(),
            workspace_id,
            source_id,
            sequence_number,
            line_start: assembled.line_start,
            line_end: assembled.line_end,
            byte_start: assembled.byte_start,
            byte_end: assembled.byte_end,
            parsed_timestamp: timestamp,
            ingested_at: Utc::now(),
            severity,
            target,
            message: msg,
            stack_trace: assembled.stack_trace,
            structured_fields: structured,
            raw: raw_redacted,
            normalized_message: normalized,
            fingerprint,
            parser_name: parser.name().to_string(),
            warnings: assembled.warnings,
            correlation_id: corr_id,
            request_id: req_id,
            trace_id,
        }
    }
}
