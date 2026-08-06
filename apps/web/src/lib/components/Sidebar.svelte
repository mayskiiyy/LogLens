<script lang="ts">
  import { activeTab, errorCounts, selectedSeverities, sources } from '../stores/appState';
  import type { Severity } from '../types';
  import { Layers, ListFilter, FileText, LayoutDashboard, HardDrive } from 'lucide-svelte';

  const severities: Severity[] = ['error', 'warning', 'info', 'debug', 'trace'];

  function toggleSeverity(sev: Severity) {
    selectedSeverities.update((list) => {
      if (list.includes(sev)) {
        return list.filter((s) => s !== sev);
      } else {
        return [...list, sev];
      }
    });
  }
</script>

<aside class="w-56 border-r border-border bg-surface flex flex-col text-xs select-none">
  <!-- Navigation Tabs -->
  <div class="p-2 border-b border-border space-y-0.5">
    <button
      on:click={() => activeTab.set('timeline')}
      class={`w-full flex items-center gap-2 px-3 py-2 rounded font-medium transition-colors ${
        $activeTab === 'timeline' ? 'bg-blue-600/20 text-blue-400 border border-blue-500/30' : 'text-gray-400 hover:bg-background hover:text-gray-200'
      }`}
    >
      <ListFilter class="w-4 h-4" />
      <span>Timeline</span>
    </button>

    <button
      on:click={() => activeTab.set('groups')}
      class={`w-full flex items-center gap-2 px-3 py-2 rounded font-medium transition-colors ${
        $activeTab === 'groups' ? 'bg-blue-600/20 text-blue-400 border border-blue-500/30' : 'text-gray-400 hover:bg-background hover:text-gray-200'
      }`}
    >
      <Layers class="w-4 h-4" />
      <span>Error Groups</span>
    </button>

    <button
      on:click={() => activeTab.set('sources')}
      class={`w-full flex items-center gap-2 px-3 py-2 rounded font-medium transition-colors ${
        $activeTab === 'sources' ? 'bg-blue-600/20 text-blue-400 border border-blue-500/30' : 'text-gray-400 hover:bg-background hover:text-gray-200'
      }`}
    >
      <FileText class="w-4 h-4" />
      <span>Log Sources</span>
    </button>

    <button
      on:click={() => activeTab.set('dashboard')}
      class={`w-full flex items-center gap-2 px-3 py-2 rounded font-medium transition-colors ${
        $activeTab === 'dashboard' ? 'bg-blue-600/20 text-blue-400 border border-blue-500/30' : 'text-gray-400 hover:bg-background hover:text-gray-200'
      }`}
    >
      <LayoutDashboard class="w-4 h-4" />
      <span>Dashboard</span>
    </button>
  </div>

  <!-- Severities Filter -->
  <div class="p-3 border-b border-border">
    <div class="text-[10px] font-semibold tracking-wider text-gray-500 uppercase mb-2">Severity Levels</div>
    <div class="space-y-1">
      {#each severities as sev}
        <button
          on:click={() => toggleSeverity(sev)}
          class={`w-full flex items-center justify-between px-2.5 py-1.5 rounded transition-colors ${
            $selectedSeverities.includes(sev) ? 'bg-background text-gray-100 font-medium' : 'text-gray-400 hover:text-gray-200'
          }`}
        >
          <div class="flex items-center gap-2">
            <span class={`w-2 h-2 rounded-full bg-severity-${sev}`}></span>
            <span class="capitalize">{sev}</span>
          </div>
          <span class="text-[10px] px-1.5 py-0.5 rounded bg-background border border-border text-gray-400">
            {$errorCounts[sev] || 0}
          </span>
        </button>
      {/each}
    </div>
  </div>

  <!-- Active Log Sources -->
  <div class="flex-1 p-3 overflow-y-auto">
    <div class="text-[10px] font-semibold tracking-wider text-gray-500 uppercase mb-2 flex items-center justify-between">
      <span>Sources ({$sources.length})</span>
      <HardDrive class="w-3 h-3" />
    </div>

    {#if $sources.length === 0}
      <div class="text-gray-500 italic text-[11px] py-2">No log sources imported yet.</div>
    {:else}
      <div class="space-y-1">
        {#each $sources as src}
          <div class="px-2 py-1.5 rounded bg-background/50 border border-border/50 text-gray-300 flex items-center justify-between">
            <span class="truncate font-mono text-[11px]" title={src.original_path}>{src.display_name}</span>
            <span class="text-[9px] text-gray-500">{src.event_count} ev</span>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</aside>
