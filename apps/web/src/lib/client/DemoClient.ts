import type { LogLensClient } from "./LogLensClient";
import type {
  EventGroup,
  LogEvent,
  LogSource,
  QueryFilter,
  Severity,
  User,
  Workspace,
} from "../types";

export class DemoLogLensClient implements LogLensClient {
  private user: User = {
    id: "demo-user",
    email: "demo@loglens.dev",
    role: "admin",
  };
  private workspace: Workspace = {
    id: "demo-workspace",
    name: "LogLens Live Demo Workspace",
    owner_id: "demo-user",
    created_at: new Date().toISOString(),
  };

  private sources: LogSource[] = [
    {
      id: "src-1",
      workspace_id: "demo-workspace",
      display_name: "application.log",
      original_path: "/var/log/application.log",
      source_type: "file",
      parser_name: "generic",
      parser_confidence: 0.95,
      detected_encoding: "UTF-8",
      size_bytes: 48912,
      current_offset: 48912,
      line_count: 350,
      event_count: 6,
      imported_at: new Date().toISOString(),
      status: "ready",
    },
    {
      id: "src-2",
      workspace_id: "demo-workspace",
      display_name: "structured.jsonl",
      original_path: "/var/log/structured.jsonl",
      source_type: "file",
      parser_name: "jsonl",
      parser_confidence: 1.0,
      detected_encoding: "UTF-8",
      size_bytes: 124050,
      current_offset: 124050,
      line_count: 820,
      event_count: 4,
      imported_at: new Date().toISOString(),
      status: "ready",
    },
  ];

  private events: LogEvent[] = [
    {
      id: "ev-1",
      workspace_id: "demo-workspace",
      source_id: "src-1",
      sequence_number: 1,
      line_start: 1,
      line_end: 1,
      byte_start: 0,
      byte_end: 80,
      parsed_timestamp: new Date(Date.now() - 3600000).toISOString(),
      ingested_at: new Date(Date.now() - 3600000).toISOString(),
      severity: "info",
      target: "main",
      message: "Application initialized successfully in production environment",
      raw: "2026-08-07T08:00:00Z [INFO] main Application initialized successfully in production environment",
      normalized_message:
        "Application initialized successfully in production environment",
      fingerprint: "fp-info-init-001",
      parser_name: "generic",
      structured_fields: {},
      warnings: [],
    },
    {
      id: "ev-2",
      workspace_id: "demo-workspace",
      source_id: "src-1",
      sequence_number: 2,
      line_start: 2,
      line_end: 2,
      byte_start: 81,
      byte_end: 170,
      parsed_timestamp: new Date(Date.now() - 3300000).toISOString(),
      ingested_at: new Date(Date.now() - 3300000).toISOString(),
      severity: "warning",
      target: "auth",
      message: "Failed login attempt for user admin from IP <IP>",
      raw: "2026-08-07T08:00:10Z [WARN] auth Failed login attempt for user admin from IP 192.168.1.100",
      normalized_message: "Failed login attempt for user admin from IP <IP>",
      fingerprint: "fp-warn-auth-002",
      parser_name: "generic",
      structured_fields: { ip: "192.168.1.100", user: "admin" },
      warnings: [],
    },
    {
      id: "ev-3",
      workspace_id: "demo-workspace",
      source_id: "src-1",
      sequence_number: 3,
      line_start: 3,
      line_end: 7,
      byte_start: 171,
      byte_end: 450,
      parsed_timestamp: new Date(Date.now() - 3000000).toISOString(),
      ingested_at: new Date(Date.now() - 3000000).toISOString(),
      severity: "error",
      target: "com.example.service.OrderService",
      message: "Failed to process order ORD-9901 due to connection loss",
      stack_trace:
        "java.lang.IllegalStateException: Database connection lost\n\tat com.example.db.DatabasePool.getConnection(DatabasePool.java:88)\n\tat com.example.service.OrderService.processOrder(OrderService.java:45)\nCaused by: java.net.ConnectException: Connection refused",
      raw: "2026-08-07 08:00:00,123 [main] ERROR com.example.service.OrderService - Failed to process order ORD-9901\njava.lang.IllegalStateException: Database connection lost\n\tat com.example.db.DatabasePool.getConnection(DatabasePool.java:88)\n\tat com.example.service.OrderService.processOrder(OrderService.java:45)\n\tCaused by: java.net.ConnectException: Connection refused",
      normalized_message:
        "Failed to process order <NUM> due to connection loss",
      fingerprint: "fp-err-db-conn-003",
      parser_name: "java",
      structured_fields: { order_id: "ORD-9901" },
      warnings: [],
    },
    {
      id: "ev-4",
      workspace_id: "demo-workspace",
      source_id: "src-2",
      sequence_number: 4,
      line_start: 1,
      line_end: 1,
      byte_start: 0,
      byte_end: 150,
      parsed_timestamp: new Date(Date.now() - 2500000).toISOString(),
      ingested_at: new Date(Date.now() - 2500000).toISOString(),
      severity: "error",
      target: "db_pool",
      message: "Database connection refused at 10.0.0.5:5432",
      raw: '{"timestamp":"2026-08-07T08:00:05Z","level":"error","logger":"db_pool","message":"Database connection refused at 10.0.0.5:5432","error_code":"ECONNREFUSED"}',
      normalized_message: "Database connection refused at <IP>:<NUM>",
      fingerprint: "fp-err-db-conn-003",
      parser_name: "jsonl",
      structured_fields: { logger: "db_pool", error_code: "ECONNREFUSED" },
      warnings: [],
    },
    {
      id: "ev-5",
      workspace_id: "demo-workspace",
      source_id: "src-1",
      sequence_number: 5,
      line_start: 8,
      line_end: 8,
      byte_start: 451,
      byte_end: 520,
      parsed_timestamp: new Date(Date.now() - 1200000).toISOString(),
      ingested_at: new Date(Date.now() - 1200000).toISOString(),
      severity: "fatal",
      target: "storage",
      message: "Disk space critically low: 98% full on /data",
      raw: "2026-08-07T08:00:20Z [FATAL] storage Disk space critically low: 98% full on /data",
      normalized_message: "Disk space critically low: <NUM>% full on /data",
      fingerprint: "fp-fatal-storage-005",
      parser_name: "generic",
      structured_fields: { usage_pct: 98 },
      warnings: [],
    },
  ];

  async bootstrap(_email: string, _pass: string): Promise<User> {
    return this.user;
  }

  async login(_email: string, _pass: string): Promise<User> {
    return this.user;
  }

  async getCurrentUser(): Promise<User | null> {
    return this.user;
  }

  async listWorkspaces(): Promise<Workspace[]> {
    return [this.workspace];
  }

  async createWorkspace(name: string): Promise<Workspace> {
    return {
      id: `ws-${Date.now()}`,
      name,
      owner_id: "demo-user",
      created_at: new Date().toISOString(),
    };
  }

  async listSources(_workspaceId: string): Promise<LogSource[]> {
    return this.sources;
  }

  async uploadSource(_workspaceId: string, file: File): Promise<LogSource> {
    const text = await file.text();
    const lines = text.split("\n").filter((l) => l.trim().length > 0);
    const newSrc: LogSource = {
      id: `src-${Date.now()}`,
      workspace_id: _workspaceId,
      display_name: file.name,
      original_path: file.name,
      source_type: "uploaded_file",
      parser_name: "generic",
      parser_confidence: 0.9,
      detected_encoding: "UTF-8",
      size_bytes: file.size,
      current_offset: file.size,
      line_count: lines.length,
      event_count: lines.length,
      imported_at: new Date().toISOString(),
      status: "ready",
    };

    lines.forEach((line, idx) => {
      this.events.unshift({
        id: `ev-up-${Date.now()}-${idx}`,
        workspace_id: _workspaceId,
        source_id: newSrc.id,
        sequence_number: this.events.length + 1,
        line_start: idx + 1,
        line_end: idx + 1,
        byte_start: 0,
        byte_end: line.length,
        parsed_timestamp: new Date().toISOString(),
        ingested_at: new Date().toISOString(),
        severity: line.toLowerCase().includes("error")
          ? "error"
          : line.toLowerCase().includes("warn")
            ? "warning"
            : "info",
        message: line,
        raw: line,
        normalized_message: line,
        fingerprint: `fp-up-${idx}`,
        parser_name: "generic",
        structured_fields: {},
        warnings: [],
      });
    });

    this.sources.unshift(newSrc);
    return newSrc;
  }

  async deleteSource(_workspaceId: string, sourceId: string): Promise<void> {
    this.sources = this.sources.filter((s) => s.id !== sourceId);
    this.events = this.events.filter((e) => e.source_id !== sourceId);
  }

  async queryEvents(
    _workspaceId: string,
    filter: QueryFilter,
  ): Promise<LogEvent[]> {
    let result = [...this.events];

    if (filter.search_query) {
      const q = filter.search_query.toLowerCase();
      if (q.startsWith("fingerprint:")) {
        const fp = q.replace("fingerprint:", "").trim();
        result = result.filter((e) => e.fingerprint === fp);
      } else {
        result = result.filter(
          (e) =>
            e.message.toLowerCase().includes(q) ||
            e.raw.toLowerCase().includes(q),
        );
      }
    }

    if (filter.severities && filter.severities.length > 0) {
      result = result.filter((e) => filter.severities?.includes(e.severity));
    }

    return result;
  }

  async listGroups(_workspaceId: string): Promise<EventGroup[]> {
    const map = new Map<string, LogEvent[]>();
    for (const ev of this.events) {
      const existing = map.get(ev.fingerprint) || [];
      existing.push(ev);
      map.set(ev.fingerprint, existing);
    }

    const groups: EventGroup[] = [];
    map.forEach((evList, fp) => {
      const first = evList[evList.length - 1];
      const last = evList[0];
      groups.push({
        fingerprint: fp,
        representative_event_id: last.id,
        occurrence_count: evList.length,
        first_seen: first.ingested_at,
        last_seen: last.ingested_at,
        severity: last.severity,
        sample_message: last.message,
        affected_sources_count: new Set(evList.map((e) => e.source_id)).size,
        trend_buckets: [1, 2, 4, 3, evList.length],
      });
    });

    return groups;
  }

  async exportEvents(
    _workspaceId: string,
    format: string,
    query?: string,
  ): Promise<Blob> {
    const events = await this.queryEvents(_workspaceId, {
      search_query: query,
    });
    if (format === "csv") {
      let csv = "id,timestamp,severity,target,message\n";
      events.forEach((ev) => {
        csv += `"${ev.id}","${ev.ingested_at}","${ev.severity}","${ev.target || ""}","${ev.message.replace(/"/g, '""')}"\n`;
      });
      return new Blob([csv], { type: "text/csv" });
    }
    return new Blob([JSON.stringify(events, null, 2)], {
      type: "application/json",
    });
  }
}
