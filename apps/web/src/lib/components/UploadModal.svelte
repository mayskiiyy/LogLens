<script lang="ts">
  import { client, currentWorkspace, events, isUploadModalOpen, sources } from '../stores/appState';
  import { X, Upload, FileText, Loader2 } from 'lucide-svelte';

  let selectedFile: File | null = null;
  let isUploading = false;
  let errorMsg = '';

  function handleFileSelect(e: Event) {
    const files = (e.target as HTMLInputElement).files;
    if (files && files.length > 0) {
      selectedFile = files[0];
    }
  }

  async function submitUpload() {
    if (!selectedFile || !$currentWorkspace) return;
    isUploading = true;
    errorMsg = '';

    try {
      const src = await client.uploadSource($currentWorkspace.id, selectedFile);
      sources.update((list) => [src, ...list]);
      const evs = await client.queryEvents($currentWorkspace.id, {});
      events.set(evs);
      isUploadModalOpen.set(false);
    } catch (err: unknown) {
      errorMsg = err instanceof Error ? err.message : 'Upload failed';
    } finally {
      isUploading = false;
    }
  }

  function closeModal() {
    isUploadModalOpen.set(false);
  }
</script>

{#if $isUploadModalOpen}
  <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50 p-4 select-none backdrop-blur-sm">
    <div class="bg-surface border border-border w-full max-w-md rounded-lg shadow-2xl p-5 space-y-4">
      <div class="flex items-center justify-between border-b border-border pb-3">
        <h3 class="text-sm font-bold text-gray-100 flex items-center gap-2">
          <Upload class="w-4 h-4 text-blue-400" />
          <span>Import Log File</span>
        </h3>
        <button on:click={closeModal} class="text-gray-400 hover:text-gray-200">
          <X class="w-4 h-4" />
        </button>
      </div>

      {#if errorMsg}
        <div class="p-2.5 bg-red-950/60 border border-red-800 text-red-300 text-xs rounded">
          {errorMsg}
        </div>
      {/if}

      <div class="border-2 border-dashed border-border hover:border-blue-500/50 rounded-lg p-6 flex flex-col items-center justify-center gap-2 bg-background/50 transition-colors cursor-pointer relative">
        <input
          type="file"
          accept=".log,.txt,.json,.jsonl,.ndjson"
          on:change={handleFileSelect}
          class="absolute inset-0 opacity-0 cursor-pointer"
        />
        <FileText class="w-8 h-8 text-blue-400 opacity-60" />
        <div class="text-xs text-gray-300 font-medium">
          {selectedFile ? selectedFile.name : 'Click or drop log file here'}
        </div>
        <div class="text-[10px] text-gray-500">Supports .log, .txt, .jsonl, .ndjson up to 1GB</div>
      </div>

      <div class="flex justify-end gap-2 pt-2 border-t border-border">
        <button
          on:click={closeModal}
          disabled={isUploading}
          class="px-3 py-1.5 text-xs text-gray-300 hover:bg-background rounded border border-border"
        >
          Cancel
        </button>
        <button
          on:click={submitUpload}
          disabled={!selectedFile || isUploading}
          class="px-4 py-1.5 text-xs bg-blue-600 hover:bg-blue-500 text-white rounded font-medium disabled:opacity-50 flex items-center gap-1.5"
        >
          {#if isUploading}
            <Loader2 class="w-3.5 h-3.5 animate-spin" />
            <span>Parsing...</span>
          {:else}
            <span>Start Import</span>
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
