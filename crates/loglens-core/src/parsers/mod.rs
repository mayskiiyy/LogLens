pub mod bracketed;
pub mod detector;
pub mod generic;
pub mod jsonl;
pub mod logcat;
pub mod nginx;
pub mod trait_def;

pub use detector::ParserRegistry;
pub use trait_def::{ParseError, ParsedLogEvent, Parser};
