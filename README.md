# LogLens

> Local-first log exploration, search, grouping, and live monitoring.

[![License](https-img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![GitHub Pages Demo](https://img.shields.io/badge/Live_Demo-GitHub_Pages-brightgreen)](https://mayskiiyy.github.io/LogLens/)

LogLens is an open-source application for importing, parsing, searching, filtering, grouping, and monitoring log files. It offers both a standalone native desktop application (via Tauri 2) and a self-hosted web service (via Docker Compose and Axum).

Both deployment modes share the same Rust parsing engine and database abstraction layer, ensuring zero third-party cloud dependencies or external proprietary APIs.

License: [Apache-2.0](LICENSE)

---

## Live Interactive Demo

Try LogLens directly in your browser without installing anything:
👉 **[LogLens Live GitHub Pages Demo](https://mayskiiyy.github.io/LogLens/)**

The live demo runs entirely client-side, preloaded with sample logs (plain text, JSONL, Nginx, and Java stack traces). You can test log filtering, error grouping, stack trace inspection, and upload local files directly in browser memory.

---

## Features

- **Dual Mode Deployment**:
  - **Desktop Application**: Local SQLite database, offline execution, native file selection.
  - **Self-Hosted Mode**: Docker Compose, role-based access control (Admin, Member, Viewer), multi-user isolation, SSE live tailing.
- **High-Performance Streaming Pipeline**: Handles large log files using streaming readers and bounded memory buffers.
- **Intelligent Error Grouping**: Normalizes dynamic data (UUIDs, IP addresses, memory addresses, timestamps, tokens) and clusters similar error events via BLAKE3 fingerprints.
- **Multiline & Stack Trace Handling**: Automatically groups multiline Java, Python, and Rust stack traces into single semantic log events.
- **Built-in Parsers**: JSON Lines (JSONL), Android Logcat, Nginx Access/Error logs, generic timestamp/severity logs, and unstructured fallback.
- **Full-Text & Filter Queries**: Supports structured query syntax (e.g. `level:error "connection refused" source:app.log after:2026-01-01`).
- **Data Privacy & Redaction**: Built-in secret redaction patterns for API keys, passwords, and bearer tokens.

---

## Quick Start (Docker Compose)

```bash
# 1. Clone the repository
git clone https://github.com/mayskiiyy/LogLens.git
cd LogLens

# 2. Copy environment settings
cp .env.example .env

# 3. Start the application with Docker Compose
docker compose up -d

# 4. Access the web interface
# Open http://localhost:8080 in your browser.
```

Initial setup will prompt you to create the administrator user account on first launch.

---

## Desktop Mode Development

```bash
# Install frontend dependencies
npm install

# Run Desktop mode (Tauri 2)
npm run tauri dev
```

---

## Architecture Overview

LogLens is structured as a Rust Cargo workspace and SvelteKit frontend:

- `crates/loglens-core`: Shared domain models, streaming log reader, multiline assembler, normalizer, parsers, and BLAKE3 fingerprinter.
- `crates/loglens-storage`: SQLx database abstraction (SQLite WAL / PostgreSQL) with FTS5 search.
- `crates/loglens-api`: Axum HTTP routers, authentication middleware, DTOs, and Server-Sent Events.
- `crates/loglens-server`: Executable for self-hosted deployment.
- `crates/loglens-cli`: CLI utility (`import`, `search`, `groups`, `inspect`, `doctor`).
- `crates/loglens-desktop`: Tauri v2 desktop application.
- `apps/web`: SvelteKit (Svelte 5) virtualized log viewer SPA.

Detailed technical design documents can be found in [`docs/architecture.md`](docs/architecture.md).

---

## Documentation

- [Architecture Guide](docs/architecture.md)
- [Configuration Reference](docs/configuration.md)
- [Deployment Guide](docs/deployment.md)
- [Writing Custom Parsers](docs/parser-development.md)
- [Security Model & Redaction](docs/security.md)

---

## Contributing

Contributions are welcome! Please review [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions, coding conventions, and PR requirements.

---

## Security

Please report security vulnerabilities privately according to our [SECURITY.md](SECURITY.md) guidelines.
