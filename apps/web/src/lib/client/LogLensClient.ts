import type { EventGroup, LogEvent, LogSource, QueryFilter, User, Workspace } from '../types';

export interface LogLensClient {
  bootstrap(email: string, pass: string): Promise<User>;
  login(email: string, pass: string): Promise<User>;
  getCurrentUser(): Promise<User | null>;
  listWorkspaces(): Promise<Workspace[]>;
  createWorkspace(name: string): Promise<Workspace>;
  listSources(workspaceId: string): Promise<LogSource[]>;
  uploadSource(workspaceId: string, file: File): Promise<LogSource>;
  deleteSource(workspaceId: string, sourceId: string): Promise<void>;
  queryEvents(workspaceId: string, filter: QueryFilter): Promise<LogEvent[]>;
  listGroups(workspaceId: string): Promise<EventGroup[]>;
  exportEvents(workspaceId: string, format: string, query?: string): Promise<Blob>;
}
