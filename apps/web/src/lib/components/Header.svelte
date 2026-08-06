<script lang="ts">
  import { activeTab, currentWorkspace, isLiveTail, isUploadModalOpen, searchQuery } from '../stores/appState';
  import { Search, Plus, Radio, Sun, Moon, Database } from 'lucide-svelte';

  let isLight = false;

  function toggleTheme() {
    isLight = !isLight;
    if (isLight) {
      document.documentElement.classList.remove('dark');
      document.documentElement.classList.add('light');
    } else {
      document.documentElement.classList.remove('light');
      document.documentElement.classList.add('dark');
    }
  }

  function handleSearchInput(e: Event) {
    const val = (e.target as HTMLInputElement).value;
    searchQuery.set(val);
  }
</script>

<header class="h-12 border-b border-border bg-surface flex items-center justify-between px-4 text-xs select-none">
  <!-- Brand and Workspace -->
  <div class="flex items-center gap-3">
    <div class="flex items-center gap-2 font-bold text-sm tracking-wide text-blue-400">
      <Database class="w-4 h-4 text-blue-500" />
      <span>LogLens</span>
    </div>

    {#if $currentWorkspace}
      <div class="flex items-center gap-1.5 px-2 py-1 rounded bg-background border border-border text-gray-300">
        <span class="w-2 h-2 rounded-full bg-emerald-500"></span>
        <span class="font-medium">{$currentWorkspace.name}</span>
      </div>
    {/if}
  </div>

  <!-- Search Input Bar -->
  <div class="flex-1 max-w-xl mx-4 relative">
    <Search class="w-3.5 h-3.5 absolute left-3 top-2.5 text-gray-400" />
    <input
      type="text"
      placeholder='Filter logs... (e.g. level:error "connection refused" source:app.log)'
      value={$searchQuery}
      on:input={handleSearchInput}
      class="w-full bg-background border border-border rounded pl-9 pr-3 py-1.5 text-xs text-gray-200 focus:outline-none focus:border-blue-500 transition-colors"
    />
  </div>

  <!-- Actions -->
  <div class="flex items-center gap-2">
    <button
      on:click={() => isUploadModalOpen.set(true)}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-blue-600 hover:bg-blue-500 text-white rounded font-medium transition-colors"
    >
      <Plus class="w-3.5 h-3.5" />
      <span>Import Log</span>
    </button>

    <button
      on:click={() => isLiveTail.update((v) => !v)}
      class={`flex items-center gap-1.5 px-2.5 py-1.5 rounded border border-border font-medium transition-colors ${
        $isLiveTail ? 'bg-emerald-950/60 border-emerald-600 text-emerald-400' : 'bg-background text-gray-400'
      }`}
    >
      <Radio class={`w-3.5 h-3.5 ${$isLiveTail ? 'animate-pulse text-emerald-400' : ''}`} />
      <span>{$isLiveTail ? 'LIVE' : 'PAUSED'}</span>
    </button>

    <button
      on:click={toggleTheme}
      class="p-1.5 border border-border rounded text-gray-400 hover:text-gray-200 bg-background transition-colors"
      title="Toggle Theme"
    >
      {#if isLight}
        <Moon class="w-4 h-4" />
      {:else}
        <Sun class="w-4 h-4" />
      {/if}
    </button>
  </div>
</header>
