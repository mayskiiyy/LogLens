<script lang="ts">
  import { groups, activeTab, searchQuery } from '../stores/appState';
  import { Layers, ArrowRight } from 'lucide-svelte';

  function filterTimelineByFingerprint(fp: string) {
    searchQuery.set(`fingerprint:${fp}`);
    activeTab.set('timeline');
  }
</script>

<div class="flex-1 bg-background p-4 overflow-y-auto">
  <div class="flex items-center justify-between mb-4">
    <div>
      <h2 class="text-sm font-semibold text-gray-200 flex items-center gap-2">
        <Layers class="w-4 h-4 text-blue-400" />
        <span>Normalized Error Groups ({$groups.length})</span>
      </h2>
      <p class="text-xs text-gray-400 mt-0.5">Clustered by BLAKE3 fingerprint over normalized messages and stack traces.</p>
    </div>
  </div>

  {#if $groups.length === 0}
    <div class="p-8 text-center text-gray-500 border border-dashed border-border rounded">
      No error groups found in the selected workspace.
    </div>
  {:else}
    <div class="space-y-3">
      {#each $groups as group}
        <div class="p-3 bg-surface border border-border rounded flex flex-col gap-2 hover:border-gray-600 transition-colors">
          <div class="flex items-center justify-between text-xs">
            <div class="flex items-center gap-2">
              <span class="px-2 py-0.5 rounded font-bold uppercase text-[10px] bg-red-950/60 border border-red-800 text-red-400">
                {group.severity}
              </span>
              <span class="font-mono text-gray-400 text-[11px]">{group.fingerprint.substring(0, 12)}...</span>
            </div>

            <div class="flex items-center gap-4 text-gray-400 text-xs">
              <div>Occurrences: <strong class="text-gray-100">{group.occurrence_count}</strong></div>
              <div>Sources: <strong class="text-gray-100">{group.affected_sources_count}</strong></div>
              <button
                on:click={() => filterTimelineByFingerprint(group.fingerprint)}
                class="flex items-center gap-1 text-blue-400 hover:text-blue-300 font-medium"
              >
                <span>View Events</span>
                <ArrowRight class="w-3.5 h-3.5" />
              </button>
            </div>
          </div>

          <div class="font-mono text-xs text-gray-200 bg-background p-2 rounded border border-border/60 break-all">
            {group.sample_message}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
