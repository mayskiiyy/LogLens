-- PostgreSQL schema migration for LogLens

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'member',
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT UNIQUE NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS workspaces (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS workspace_members (
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL DEFAULT 'viewer',
    PRIMARY KEY (workspace_id, user_id)
);

CREATE TABLE IF NOT EXISTS sources (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    owner_id UUID REFERENCES users(id) ON DELETE SET NULL,
    display_name TEXT NOT NULL,
    original_path TEXT NOT NULL,
    source_type TEXT NOT NULL,
    parser_name TEXT NOT NULL,
    parser_confidence REAL NOT NULL,
    detected_encoding TEXT NOT NULL,
    size_bytes BIGINT NOT NULL,
    current_offset BIGINT NOT NULL,
    line_count BIGINT NOT NULL,
    event_count BIGINT NOT NULL,
    imported_at TIMESTAMPTZ NOT NULL,
    last_scanned_at TIMESTAMPTZ,
    last_modified_at TIMESTAMPTZ,
    checksum TEXT,
    live_watch_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    status TEXT NOT NULL,
    error_details TEXT
);

CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    source_id UUID NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
    sequence_number BIGINT NOT NULL,
    line_start BIGINT NOT NULL,
    line_end BIGINT NOT NULL,
    byte_start BIGINT NOT NULL,
    byte_end BIGINT NOT NULL,
    parsed_timestamp TIMESTAMPTZ,
    ingested_at TIMESTAMPTZ NOT NULL,
    severity TEXT NOT NULL,
    target TEXT,
    message TEXT NOT NULL,
    stack_trace TEXT,
    structured_fields JSONB NOT NULL DEFAULT '{}'::jsonb,
    raw TEXT NOT NULL,
    normalized_message TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    parser_name TEXT NOT NULL,
    warnings JSONB NOT NULL DEFAULT '[]'::jsonb,
    correlation_id TEXT,
    request_id TEXT,
    trace_id TEXT
);

CREATE TABLE IF NOT EXISTS saved_searches (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    query_string TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS audit_events (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    details JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_events_workspace_ts ON events(workspace_id, parsed_timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_events_workspace_severity ON events(workspace_id, severity);
CREATE INDEX IF NOT EXISTS idx_events_fingerprint ON events(workspace_id, fingerprint);
CREATE INDEX IF NOT EXISTS idx_events_source_seq ON events(source_id, sequence_number);
