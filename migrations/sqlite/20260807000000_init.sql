-- Enable WAL mode and foreign keys for SQLite
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'member',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT UNIQUE NOT NULL,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Workspaces table
CREATE TABLE IF NOT EXISTS workspaces (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    owner_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL
);

-- Workspace Members
CREATE TABLE IF NOT EXISTS workspace_members (
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL DEFAULT 'viewer',
    PRIMARY KEY (workspace_id, user_id)
);

-- Sources table
CREATE TABLE IF NOT EXISTS sources (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    owner_id TEXT REFERENCES users(id) ON DELETE SET NULL,
    display_name TEXT NOT NULL,
    original_path TEXT NOT NULL,
    source_type TEXT NOT NULL,
    parser_name TEXT NOT NULL,
    parser_confidence REAL NOT NULL,
    detected_encoding TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    current_offset INTEGER NOT NULL,
    line_count INTEGER NOT NULL,
    event_count INTEGER NOT NULL,
    imported_at TEXT NOT NULL,
    last_scanned_at TEXT,
    last_modified_at TEXT,
    checksum TEXT,
    live_watch_enabled INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    error_details TEXT
);

-- Events table
CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    source_id TEXT NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    line_start INTEGER NOT NULL,
    line_end INTEGER NOT NULL,
    byte_start INTEGER NOT NULL,
    byte_end INTEGER NOT NULL,
    parsed_timestamp TEXT,
    ingested_at TEXT NOT NULL,
    severity TEXT NOT NULL,
    target TEXT,
    message TEXT NOT NULL,
    stack_trace TEXT,
    structured_fields TEXT NOT NULL DEFAULT '{}',
    raw TEXT NOT NULL,
    normalized_message TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    parser_name TEXT NOT NULL,
    warnings TEXT NOT NULL DEFAULT '[]',
    correlation_id TEXT,
    request_id TEXT,
    trace_id TEXT
);

-- FTS5 Full Text Search Table for SQLite
CREATE VIRTUAL TABLE IF NOT EXISTS events_fts USING fts5(
    event_id UNINDEXED,
    message,
    target,
    raw,
    normalized_message,
    tokenize = 'porter unicode61'
);

-- Triggers for SQLite FTS5 synchronization
CREATE TRIGGER IF NOT EXISTS events_ai AFTER INSERT ON events BEGIN
    INSERT INTO events_fts(event_id, message, target, raw, normalized_message)
    VALUES (new.id, new.message, new.target, new.raw, new.normalized_message);
END;

CREATE TRIGGER IF NOT EXISTS events_ad AFTER DELETE ON events BEGIN
    INSERT INTO events_fts(events_fts, event_id, message, target, raw, normalized_message)
    VALUES('delete', old.id, old.message, old.target, old.raw, old.normalized_message);
END;

-- Saved Searches
CREATE TABLE IF NOT EXISTS saved_searches (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    query_string TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Audit Events
CREATE TABLE IF NOT EXISTS audit_events (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT REFERENCES users(id) ON DELETE SET NULL,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    details TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_events_workspace_ts ON events(workspace_id, parsed_timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_events_workspace_severity ON events(workspace_id, severity);
CREATE INDEX IF NOT EXISTS idx_events_fingerprint ON events(workspace_id, fingerprint);
CREATE INDEX IF NOT EXISTS idx_events_source_seq ON events(source_id, sequence_number);
