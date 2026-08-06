use std::sync::Arc;
use crate::parsers::bracketed::BracketedTimestampParser;
use crate::parsers::generic::GenericLogParser;
use crate::parsers::jsonl::JsonLinesParser;
use crate::parsers::logcat::LogcatParser;
use crate::parsers::nginx::{NginxAccessParser, NginxErrorParser};
use crate::parsers::trait_def::Parser;

pub struct ParserRegistry {
    parsers: Vec<Arc<dyn Parser>>,
}

impl ParserRegistry {
    pub fn new() -> Self {
        Self {
            parsers: vec![
                Arc::new(JsonLinesParser::new()),
                Arc::new(LogcatParser::new()),
                Arc::new(NginxAccessParser::new()),
                Arc::new(NginxErrorParser::new()),
                Arc::new(BracketedTimestampParser::new()),
                Arc::new(GenericLogParser::new()),
            ],
        }
    }

    pub fn detect_best_parser(&self, sample: &str) -> (Arc<dyn Parser>, f32) {
        let mut best_score = 0.0f32;
        let mut best_parser: Option<Arc<dyn Parser>> = None;

        for p in &self.parsers {
            let score = p.detect(sample);
            if score > best_score {
                best_score = score;
                best_parser = Some(p.clone());
            }
        }

        if let Some(p) = best_parser {
            if best_score >= 0.3 {
                return (p, best_score);
            }
        }

        (Arc::new(GenericLogParser::new()), 0.1)
    }

    pub fn get_by_name(&self, name: &str) -> Option<Arc<dyn Parser>> {
        self.parsers.iter().find(|p| p.name() == name).cloned()
    }
}

impl Default for ParserRegistry {
    fn default() -> Self {
        Self::new()
    }
}
