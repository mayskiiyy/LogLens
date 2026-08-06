<script lang="ts">
  import { events, selectedEvent } from '../stores/appState';
  import type { LogEvent } from '../types';
  import { AlertCircle, AlertTriangle, Info, Bug, FileCode } from 'lucide-svelte';

  function getSeverityColor(sev: string) {
    switch (sev) {
      case 'error':
      case 'critical':
      case 'fatal':
        return 'text-red-400 bg-red-950/40 border-red-800/50';
      case 'warning':
        return 'text-amber-400 bg-amber-950/40 border-amber-800/50';
      case 'info':
      case 'notice':
        return 'text-emerald-400 bg-emerald-950/40 border-emerald-800/50';
      case 'debug':
      case 'trace':
        return 'text-blue-400 bg-blue-950/40 border-blue-800/50';
      default:
        return 'text-gray-400 bg-gray-900 border-gray-800';
    }
  }

  function selectRow(ev: LogEvent) {
    selectedEvent.set(ev);
  }
</script>

<div class="flex-1 flex flex-col bg-background overflow-hidden">
  <!-- Table Header -->
  <div class="h-8 border-b border-border bg-surface/50 flex items-center px-3 text-[11px] font-semibold text-gray-400 select-none">
    <div class="w-12 text-center">#</div>
    <div class="w-24">Severity</div>
    <div class="w-40">Timestamp</div>
    <div class="w-32 truncate">Target</div>
    <div class="flex-1 px-2">Message</div>
  </div>

  <!-- Event List Container -->
  <div class="flex-1 overflow-y-auto font-mono text-xs divide-y divide-border/40">
    {#if $events.length === 0}
      <div class="h-full flex flex-col items-center justify-center text-gray-500 gap-2 select-none">
        <FileCode class="w-8 h-8 opacity-40" />
        <div>No log events match current query or workspace is empty.</div>
      </div>
    {:else}
      {#each $events as ev (ev.id)}
        <button
          type="button"
          on:click={() => selectRow(ev)}
          class={`w-full text-left flex items-start px-3 py-1.5 cursor-pointer hover:bg-surface/80 transition-colors ${
            $selectedEvent?.id === ev.id ? 'bg-blue-950/40 border-l-2 border-blue-500' : ''
          }`}
        >
          <!-- Sequence Number -->
          <div class="w-12 text-[10px] text-gray-500 text-center py-0.5 select-none">{ev.sequence_number}</div>

          <!-- Severity Badge -->
          <div class="w-24 pr-2">
            <span class={`inline-flex items-center gap-1 px-2 py-0.5 rounded border text-[10px] uppercase font-bold tracking-wider ${getSeverityColor(ev.severity)}`}>
              {#if ev.severity === 'error' || ev.severity === 'fatal'}
                <AlertCircle class="w-3 h-3" />
              {:else if ev.severity === 'warning'}
                <AlertTriangle class="w-3 h-3" />
              {:else if ev.severity === 'info'}
                <Info class="w-3 h-3" />
              {:else}
                <Bug class="w-3 h-3" />
              {/if}
              <span>{ev.severity}</span>
            </span>
          </div>

          <!-- Timestamp -->
          <div class="w-40 text-gray-400 py-0.5 text-[11px]">
            {ev.parsed_timestamp ? new Date(ev.parsed_timestamp).toISOString() : new Date(ev.ingested_at).toISOString()}
          </div>

          <!-- Target / Module -->
          <div class="w-32 text-gray-400 py-0.5 truncate text-[11px]" title={ev.target}>
            {ev.target || '-'}
          </div>

          <!-- Log Message -->
          <div class="flex-1 px-2 text-gray-200 py-0.5 break-all">
            <span>{ev.message}</span>
            {#if ev.stack_trace}
              <span class="ml-2 px-1.5 py-0.2 bg-red-950/60 text-red-300 border border-red-800/40 rounded text-[9px]">Stack Trace</span>
            {/if}
          </div>
        </button>
      {/each}
    {/if}
  </div>
</div>
