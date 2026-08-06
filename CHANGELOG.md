# Changelog

All notable changes to LogLens will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-08-07

### Added
- Initial public release of LogLens.
- Core Rust parsing engine supporting JSONL, Logcat, Nginx, and multiline stack traces.
- SQLite WAL storage layer with FTS5 search.
- Axum self-hosted API server with SSE live tailing and Argon2id authentication.
- Tauri 2 Desktop app shell.
- SvelteKit + Svelte 5 virtualized log viewer frontend.
- Docker Compose deployment support.
