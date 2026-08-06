import { writable, derived } from 'svelte/store';
import type { EventGroup, LogEvent, LogSource, Severity, User, Workspace } from '../types';
import type { LogLensClient } from '../client/LogLensClient';
import { HttpLogLensClient } from '../client/HttpClient';
import { TauriLogLensClient } from '../client/TauriClient';
import { DemoLogLensClient } from '../client/DemoClient';

function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

function isGithubPagesOrDemo(): boolean {
  if (typeof window === 'undefined') return false;
  const host = window.location.hostname;
  return host.includes('github.io') || window.location.search.includes('demo=true');
}

function createClient(): LogLensClient {
  if (isTauri()) {
    return new TauriLogLensClient();
  }
  if (isGithubPagesOrDemo()) {
    return new DemoLogLensClient();
  }
  return new HttpLogLensClient();
}

export const client: LogLensClient = createClient();

export const currentUser = writable<User | null>(null);
export const workspaces = writable<Workspace[]>([]);
export const currentWorkspace = writable<Workspace | null>(null);

export const activeTab = writable<'timeline' | 'groups' | 'sources' | 'dashboard'>('timeline');

export const searchQuery = writable<string>('');
export const selectedSeverities = writable<Severity[]>([]);
export const selectedSourceId = writable<string | null>(null);

export const events = writable<LogEvent[]>([]);
export const selectedEvent = writable<LogEvent | null>(null);

export const groups = writable<EventGroup[]>([]);
export const sources = writable<LogSource[]>([]);

export const isLiveTail = writable<boolean>(true);
export const isUploadModalOpen = writable<boolean>(false);

export const errorCounts = derived(events, ($events) => {
  const counts: Record<Severity, number> = {
    trace: 0,
    debug: 0,
    info: 0,
    notice: 0,
    warning: 0,
    error: 0,
    critical: 0,
    fatal: 0,
    unknown: 0
  };

  for (const ev of $events) {
    if (counts[ev.severity] !== undefined) {
      counts[ev.severity]++;
    }
  }

  return counts;
});
