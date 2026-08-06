export type Severity =
  | 'trace'
  | 'debug'
  | 'info'
  | 'notice'
  | 'warning'
  | 'error'
  | 'critical'
  | 'fatal'
  | 'unknown';

export interface User {
  id: string;
  email: string;
  role: 'admin' | 'member' | 'viewer';
}

export interface Workspace {
  id: string;
  name: string;
  owner_id: string;
  created_at: string;
}

export interface LogSource {
  id: string;
  workspace_id: string;
  owner_id?: string;
  display_name: string;
  original_path: string;
  source_type: string;
  parser_name: string;
  parser_confidence: number;
  detected_encoding: string;
  size_bytes: number;
  current_offset: number;
  line_count: number;
  event_count: number;
  imported_at: string;
  status: string;
  error_details?: string;
}

export interface LogEvent {
  id: string;
  workspace_id: string;
  source_id: string;
  sequence_number: number;
  line_start: number;
  line_end: number;
  byte_start: number;
  byte_end: number;
  parsed_timestamp?: string;
  ingested_at: string;
  severity: Severity;
  target?: string;
  message: string;
  stack_trace?: string;
  structured_fields: Record<string, unknown>;
  raw: string;
  normalized_message: string;
  fingerprint: string;
  parser_name: string;
  warnings: string[];
  correlation_id?: string;
  request_id?: string;
  trace_id?: string;
}

export interface EventGroup {
  fingerprint: string;
  representative_event_id: string;
  occurrence_count: number;
  first_seen: string;
  last_seen: string;
  severity: Severity;
  sample_message: string;
  affected_sources_count: number;
  trend_buckets: number[];
}

export interface QueryFilter {
  search_query?: string;
  severities?: Severity[];
  source_ids?: string[];
  fingerprint?: string;
  limit?: number;
  offset?: number;
}
