# Developing Custom Parsers for LogLens

All LogLens parsers implement the `Parser` trait defined in `loglens-core`:

```rust
pub trait Parser: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, sample: &str) -> f32; // Returns confidence score between 0.0 and 1.0
    fn parse(&self, input: &str, line_no: u64) -> Result<ParsedLogEvent, ParseError>;
    fn supports_multiline(&self) -> bool { false }
    fn reset(&mut self) {}
}
```

## Tutorial: Creating a Custom Parser

1. Create your parser struct in `crates/loglens-core/src/parsers/my_parser.rs`.
2. Implement `detect()` to score log lines based on sample patterns.
3. Implement `parse()` to return structured `ParsedLogEvent`.
4. Register your parser in `ParserRegistry::default()`.
