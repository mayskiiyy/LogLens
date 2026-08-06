import type { LogLensClient } from './LogLensClient';
import type { EventGroup, LogEvent, LogSource, QueryFilter, User, Workspace } from '../types';

export class TauriLogLensClient implements LogLensClient {
  private async invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
      const { invoke } = await import('@tauri-apps/api/core');
      return invoke<T>(cmd, args);
    }
    throw new Error('Tauri API not available in browser environment');
  }

  async bootstrap(_email: string, _pass: string): Promise<User> {
    return { id: 'local-user', email: 'local@desktop', role: 'admin' };
  }

  async login(_email: string, _pass: string): Promise<User> {
    return { id: 'local-user', email: 'local@desktop', role: 'admin' };
  }

  async getCurrentUser(): Promise<User | null> {
    return { id: 'local-user', email: 'local@desktop', role: 'admin' };
  }

  async listWorkspaces(): Promise<Workspace[]> {
    return [{ id: 'local-workspace', name: 'Local Desktop Workspace', owner_id: 'local-user', created_at: new Date().toISOString() }];
  }

  async createWorkspace(name: string): Promise<Workspace> {
    return { id: 'local-workspace', name, owner_id: 'local-user', created_at: new Date().toISOString() };
  }

  async listSources(_workspaceId: string): Promise<LogSource[]> {
    return this.invoke<LogSource[]>('list_sources');
  }

  async uploadSource(_workspaceId: string, file: File): Promise<LogSource> {
    return this.invoke<LogSource>('import_file', { path: file.name });
  }

  async deleteSource(_workspaceId: string, sourceId: string): Promise<void> {
    await this.invoke('delete_source', { sourceId });
  }

  async queryEvents(_workspaceId: string, filter: QueryFilter): Promise<LogEvent[]> {
    return this.invoke<LogEvent[]>('query_events', { filter });
  }

  async listGroups(_workspaceId: string): Promise<EventGroup[]> {
    return this.invoke<EventGroup[]>('list_groups');
  }

  async exportEvents(_workspaceId: string, format: string, query?: string): Promise<Blob> {
    const data = await this.invoke<string>('export_events', { format, query });
    return new Blob([data], { type: 'text/plain' });
  }
}
