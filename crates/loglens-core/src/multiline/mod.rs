#[derive(Debug, Clone)]
pub struct MultilineConfig {
    pub max_lines: usize,
    pub max_bytes: usize,
}

impl Default for MultilineConfig {
    fn default() -> Self {
        Self {
            max_lines: 500,
            max_bytes: 512 * 1024, // 512 KB
        }
    }
}

pub struct RawLine {
    pub line_number: u64,
    pub byte_offset: u64,
    pub content: String,
}

pub struct AssembledEvent {
    pub line_start: u64,
    pub line_end: u64,
    pub byte_start: u64,
    pub byte_end: u64,
    pub raw_lines: Vec<String>,
    pub main_line: String,
    pub stack_trace: Option<String>,
    pub warnings: Vec<String>,
}

pub struct MultilineAssembler {
    config: MultilineConfig,
    current_lines: Vec<RawLine>,
}

impl MultilineAssembler {
    pub fn new(config: MultilineConfig) -> Self {
        Self {
            config,
            current_lines: Vec::new(),
        }
    }

    pub fn is_continuation(line: &str) -> bool {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            return false;
        }

        line.starts_with(' ')
            || line.starts_with('\t')
            || trimmed.starts_with("at ")
            || trimmed.starts_with("Caused by:")
            || trimmed.starts_with("Suppressed:")
            || trimmed.starts_with("Traceback (most recent call last):")
            || trimmed.starts_with("File \"")
            || trimmed.starts_with("thread '")
            || trimmed.starts_with("stack backtrace:")
            || (trimmed.starts_with('|') && line.contains("-->"))
    }

    pub fn push_line(
        &mut self,
        line_number: u64,
        byte_offset: u64,
        content: String,
    ) -> Option<AssembledEvent> {
        let is_cont = Self::is_continuation(&content);

        if is_cont && !self.current_lines.is_empty() {
            self.current_lines.push(RawLine {
                line_number,
                byte_offset,
                content,
            });

            let current_bytes: usize = self.current_lines.iter().map(|l| l.content.len()).sum();
            if self.current_lines.len() >= self.config.max_lines
                || current_bytes >= self.config.max_bytes
            {
                return self.flush(Some("Reached maximum multiline event limits".to_string()));
            }

            None
        } else {
            let flushed = self.flush(None);
            self.current_lines.push(RawLine {
                line_number,
                byte_offset,
                content,
            });
            flushed
        }
    }

    pub fn finish(&mut self) -> Option<AssembledEvent> {
        self.flush(None)
    }

    fn flush(&mut self, warning: Option<String>) -> Option<AssembledEvent> {
        if self.current_lines.is_empty() {
            return None;
        }

        let lines = std::mem::take(&mut self.current_lines);
        let line_start = lines.first().unwrap().line_number;
        let line_end = lines.last().unwrap().line_number;
        let byte_start = lines.first().unwrap().byte_offset;
        let last_line = lines.last().unwrap();
        let byte_end = last_line.byte_offset + last_line.content.len() as u64;

        let main_line = lines.first().unwrap().content.clone();
        let mut warnings = Vec::new();
        if let Some(w) = warning {
            warnings.push(w);
        }

        let (raw_strings, stack_trace) = if lines.len() > 1 {
            let st_lines: Vec<String> = lines[1..].iter().map(|l| l.content.clone()).collect();
            let st = st_lines.join("\n");
            let raw_all: Vec<String> = lines.into_iter().map(|l| l.content).collect();
            (raw_all, Some(st))
        } else {
            (vec![main_line.clone()], None)
        };

        Some(AssembledEvent {
            line_start,
            line_end,
            byte_start,
            byte_end,
            raw_lines: raw_strings,
            main_line,
            stack_trace,
            warnings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiline_java_stacktrace() {
        let mut assembler = MultilineAssembler::new(MultilineConfig::default());
        assert!(assembler.push_line(1, 0, "2026-08-07 ERROR Main - Failed".to_string()).is_none());
        assert!(assembler.push_line(2, 30, "java.lang.NullPointerException".to_string()).is_none());
        assert!(assembler.push_line(3, 60, "\tat com.example.App.main(App.java:10)".to_string()).is_none());

        let event = assembler.push_line(4, 100, "2026-08-07 INFO Main - Next event".to_string()).unwrap();
        assert_eq!(event.line_start, 1);
        assert_eq!(event.line_end, 3);
        assert!(event.stack_trace.is_some());
    }
}
