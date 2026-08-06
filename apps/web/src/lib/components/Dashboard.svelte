<script lang="ts">
  import { errorCounts, events, groups, sources } from '../stores/appState';
  import { Activity, AlertCircle, FileText, Layers, ShieldCheck, HardDrive } from 'lucide-svelte';
</script>

<div class="flex-1 bg-background p-6 overflow-y-auto space-y-6">
  <div>
    <h2 class="text-base font-bold text-gray-100 flex items-center gap-2">
      <Activity class="w-5 h-5 text-blue-400" />
      <span>LogLens System Dashboard</span>
    </h2>
    <p class="text-xs text-gray-400 mt-1">Real-time log ingestion metrics, severity breakdown, and storage status.</p>
  </div>

  <!-- Metric Cards -->
  <div class="grid grid-cols-4 gap-4">
    <div class="p-4 bg-surface border border-border rounded flex items-center justify-between">
      <div>
        <div class="text-xs text-gray-400 font-medium">Total Events</div>
        <div class="text-2xl font-bold text-gray-100 mt-1">{$events.length}</div>
      </div>
      <FileText class="w-8 h-8 text-blue-400 opacity-60" />
    </div>

    <div class="p-4 bg-surface border border-border rounded flex items-center justify-between">
      <div>
        <div class="text-xs text-gray-400 font-medium">Total Errors</div>
        <div class="text-2xl font-bold text-red-400 mt-1">{$errorCounts.error + $errorCounts.critical + $errorCounts.fatal}</div>
      </div>
      <AlertCircle class="w-8 h-8 text-red-400 opacity-60" />
    </div>

    <div class="p-4 bg-surface border border-border rounded flex items-center justify-between">
      <div>
        <div class="text-xs text-gray-400 font-medium">Error Clusters</div>
        <div class="text-2xl font-bold text-amber-400 mt-1">{$groups.length}</div>
      </div>
      <Layers class="w-8 h-8 text-amber-400 opacity-60" />
    </div>

    <div class="p-4 bg-surface border border-border rounded flex items-center justify-between">
      <div>
        <div class="text-xs text-gray-400 font-medium">Active Sources</div>
        <div class="text-2xl font-bold text-emerald-400 mt-1">{$sources.length}</div>
      </div>
      <HardDrive class="w-8 h-8 text-emerald-400 opacity-60" />
    </div>
  </div>

  <!-- Breakdown Section -->
  <div class="grid grid-cols-2 gap-6">
    <div class="p-4 bg-surface border border-border rounded">
      <h3 class="text-xs font-semibold text-gray-200 mb-3">Severity Breakdown</h3>
      <div class="space-y-2 text-xs">
        {#each Object.entries($errorCounts) as [sev, count]}
          <div class="flex items-center justify-between">
            <span class="capitalize text-gray-400">{sev}</span>
            <div class="flex items-center gap-2">
              <div class="w-32 h-2 bg-background rounded overflow-hidden">
                <div
                  class={`h-full bg-severity-${sev}`}
                  style={`width: ${$events.length > 0 ? (count / $events.length) * 100 : 0}%`}
                ></div>
              </div>
              <span class="w-10 text-right text-gray-200 font-mono">{count}</span>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <div class="p-4 bg-surface border border-border rounded">
      <h3 class="text-xs font-semibold text-gray-200 mb-3">System Health & Integrity</h3>
      <div class="space-y-3 text-xs">
        <div class="flex items-center justify-between border-b border-border/60 pb-2">
          <span class="text-gray-400">Database Engine:</span>
          <span class="font-mono text-emerald-400 flex items-center gap-1">
            <ShieldCheck class="w-3.5 h-3.5" />
            <span>SQLite WAL (FTS5 Enabled)</span>
          </span>
        </div>
        <div class="flex items-center justify-between border-b border-border/60 pb-2">
          <span class="text-gray-400">Redaction Pipeline:</span>
          <span class="font-mono text-emerald-400">Active (Auto-masking secrets)</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-gray-400">Offline Isolation:</span>
          <span class="font-mono text-emerald-400">Zero Cloud Dependencies</span>
        </div>
      </div>
    </div>
  </div>
</div>
