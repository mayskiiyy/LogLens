import type { LogLensClient } from './LogLensClient';
import type { EventGroup, LogEvent, LogSource, QueryFilter, User, Workspace } from '../types';

export class HttpLogLensClient implements LogLensClient {
  private baseUrl = '/api/v1';

  private async request<T>(path: string, options?: RequestInit): Promise<T> {
    const res = await fetch(`${this.baseUrl}${path}`, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...(options?.headers || {})
      }
    });

    if (!res.ok) {
      const err = await res.json().catch(() => ({ detail: res.statusText }));
      throw new Error(err.detail || 'HTTP Error');
    }

    return res.json();
  }

  async bootstrap(email: string, pass: string): Promise<User> {
    return this.request('/auth/bootstrap', {
      method: 'POST',
      body: JSON.stringify({ email, password: pass })
    });
  }

  async login(email: string, pass: string): Promise<User> {
    return this.request('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ email, password: pass })
    });
  }

  async getCurrentUser(): Promise<User | null> {
    return this.request<User>('/auth/me').catch(() => null);
  }

  async listWorkspaces(): Promise<Workspace[]> {
    return this.request('/workspaces');
  }

  async createWorkspace(name: string): Promise<Workspace> {
    return this.request('/workspaces', {
      method: 'POST',
      body: JSON.stringify({ name })
    });
  }

  async listSources(workspaceId: string): Promise<LogSource[]> {
    return this.request(`/sources?workspace_id=${workspaceId}`);
  }

  async uploadSource(workspaceId: string, file: File): Promise<LogSource> {
    const formData = new FormData();
    formData.append('file', file);

    const res = await fetch(`${this.baseUrl}/sources/upload?workspace_id=${workspaceId}`, {
      method: 'POST',
      body: formData
    });

    if (!res.ok) {
      throw new Error('Failed to upload file');
    }

    return res.json();
  }

  async deleteSource(workspaceId: string, sourceId: string): Promise<void> {
    await this.request(`/sources/${sourceId}?workspace_id=${workspaceId}`, {
      method: 'DELETE'
    });
  }

  async queryEvents(workspaceId: string, filter: QueryFilter): Promise<LogEvent[]> {
    const params = new URLSearchParams({ workspace_id: workspaceId });
    if (filter.search_query) params.append('search', filter.search_query);
    if (filter.fingerprint) params.append('fingerprint', filter.fingerprint);
    if (filter.limit) params.append('limit', filter.limit.toString());

    return this.request(`/events?${params.toString()}`);
  }

  async listGroups(workspaceId: string): Promise<EventGroup[]> {
    return this.request(`/groups?workspace_id=${workspaceId}`);
  }

  async exportEvents(workspaceId: string, format: string, query?: string): Promise<Blob> {
    const res = await fetch(`${this.baseUrl}/exports?workspace_id=${workspaceId}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ format, query })
    });
    return res.blob();
  }
}
