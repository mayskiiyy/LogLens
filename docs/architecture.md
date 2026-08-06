# LogLens Architecture Overview

LogLens is built as a modular monolith in Rust with a SvelteKit single-page application frontend.

```
+-------------------------------------------------------------+
|                      SvelteKit 5 SPA                        |
|   Timeline | Groups | Sources | Event Details | Settings   |
+------------------------------+------------------------------+
                               | (LogLensClient TS Interface)
             +-----------------+-----------------+
             |                                   |
             v                                   v
+------------------------+           +------------------------+
|  Axum REST & SSE API   |           |  Tauri v2 Native IPC   |
|   (loglens-server)     |           |   (loglens-desktop)    |
+------------+-----------+           +-----------+------------+
             |                                   |
             +-----------------+-----------------+
                               |
                               v
             +-----------------------------------+
             |    loglens-storage DB Abstraction |
             |   (SQLite WAL + FTS5 / PostgreSQL)|
             +-----------------+-----------------+
                               |
                               v
             +-----------------------------------+
             |    loglens-core Domain Engine     |
             | Parsers | Normalizer | BLAKE3     |
             +-----------------------------------+
```

## Shared Core (`loglens-core`)

- Fully decoupled from HTTP, database, or UI logic.
- Implements bounded streaming chunk processing to prevent loading whole files into memory.
- Performs regex/field normalization to strip dynamic variables (IPs, UUIDs, hex values) before computing BLAKE3 hashes for fingerprinting.

## Persistence (`loglens-storage`)

- Uses SQLx for async transactions.
- Provides repository traits allowing seamless execution over SQLite (default WAL mode) or PostgreSQL.
- Implements full-text search indexing via SQLite FTS5.

## Transport Layers (`loglens-api` & `loglens-desktop`)

- **Self-Hosted API**: Axum handlers, Argon2id authentication, HttpOnly cookies, SSE streaming for live tail.
- **Desktop IPC**: Tauri v2 command wrappers binding directly to core and storage primitives without opening network sockets.
