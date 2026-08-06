<script lang="ts">
  import { selectedEvent } from '../stores/appState';
  import { X, Copy, Check, Hash, FileText, Code2 } from 'lucide-svelte';

  let copied = false;

  function copyRaw() {
    if ($selectedEvent) {
      navigator.clipboard.writeText($selectedEvent.raw);
      copied = true;
      setTimeout(() => (copied = false), 2000);
    }
  }

  function closeDrawer() {
    selectedEvent.set(null);
  }
</script>

{#if $selectedEvent}
  <aside class="w-96 border-l border-border bg-surface flex flex-col text-xs select-none shadow-xl">
    <!-- Drawer Header -->
    <div class="h-10 border-b border-border flex items-center justify-between px-3">
      <div class="font-semibold text-gray-200 flex items-center gap-1.5">
        <FileText class="w-4 h-4 text-blue-400" />
        <span>Event Inspector</span>
      </div>
      <button on:click={closeDrawer} class="p-1 rounded text-gray-400 hover:text-gray-200 hover:bg-background">
        <X class="w-4 h-4" />
      </button>
    </div>

    <div class="flex-1 overflow-y-auto p-3 space-y-4 font-mono">
      <!-- General Metadata -->
      <div class="space-y-1.5 bg-background p-2.5 rounded border border-border">
        <div class="flex justify-between">
          <span class="text-gray-400">Sequence:</span>
          <span class="text-gray-200">#{$selectedEvent.sequence_number}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-400">Severity:</span>
          <span class="uppercase font-bold text-gray-200">{$selectedEvent.severity}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-400">Parser:</span>
          <span class="text-gray-200">{$selectedEvent.parser_name}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-400">Fingerprint:</span>
          <span class="text-blue-400 truncate max-w-[160px]" title={$selectedEvent.fingerprint}>{$selectedEvent.fingerprint.substring(0, 12)}...</span>
        </div>
      </div>

      <!-- Raw Log Line -->
      <div>
        <div class="flex items-center justify-between mb-1">
          <span class="text-[10px] uppercase font-semibold text-gray-400 flex items-center gap-1">
            <Code2 class="w-3 h-3" />
            <span>Raw Log Line</span>
          </span>
          <button on:click={copyRaw} class="text-[10px] text-blue-400 flex items-center gap-1 hover:underline">
            {#if copied}
              <Check class="w-3 h-3 text-emerald-400" />
              <span class="text-emerald-400">Copied</span>
            {:else}
              <Copy class="w-3 h-3" />
              <span>Copy Raw</span>
            {/if}
          </button>
        </div>
        <pre class="p-2.5 bg-background border border-border rounded text-[11px] text-gray-200 whitespace-pre-wrap break-all">{$selectedEvent.raw}</pre>
      </div>

      <!-- Stack Trace (If Present) -->
      {#if $selectedEvent.stack_trace}
        <div>
          <div class="text-[10px] uppercase font-semibold text-red-400 mb-1">Stack Trace</div>
          <pre class="p-2.5 bg-red-950/40 border border-red-900/60 rounded text-[11px] text-red-200 whitespace-pre-wrap break-all overflow-x-auto font-mono">{$selectedEvent.stack_trace}</pre>
        </div>
      {/if}

      <!-- Structured Fields -->
      {#if Object.keys($selectedEvent.structured_fields).length > 0}
        <div>
          <div class="text-[10px] uppercase font-semibold text-gray-400 mb-1 flex items-center gap-1">
            <Hash class="w-3 h-3" />
            <span>Parsed Fields</span>
          </div>
          <pre class="p-2.5 bg-background border border-border rounded text-[11px] text-emerald-300 whitespace-pre-wrap break-all">{JSON.stringify($selectedEvent.structured_fields, null, 2)}</pre>
        </div>
      {/if}
    </div>
  </aside>
{/if}
