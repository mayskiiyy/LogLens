<script lang="ts">
  import { onMount } from 'svelte';
  import {
    activeTab,
    client,
    currentUser,
    currentWorkspace,
    events,
    groups,
    searchQuery,
    selectedSeverities,
    selectedSourceId,
    sources,
    workspaces
  } from '../lib/stores/appState';
  import Header from '../lib/components/Header.svelte';
  import Sidebar from '../lib/components/Sidebar.svelte';
  import Timeline from '../lib/components/Timeline.svelte';
  import Groups from '../lib/components/Groups.svelte';
  import EventDetails from '../lib/components/EventDetails.svelte';
  import Dashboard from '../lib/components/Dashboard.svelte';
  import UploadModal from '../lib/components/UploadModal.svelte';
  import { DemoLogLensClient } from '../lib/client/DemoClient';

  let isLoading = true;
  let showAuthModal = false;
  let authEmail = '';
  let authPass = '';
  let isBootstrap = false;
  let authError = '';
  let activeClient = client;

  onMount(async () => {
    try {
      let user = await activeClient.getCurrentUser();
      if (!user) {
        try {
          user = await activeClient.login('admin@loglens.local', 'admin');
        } catch {
          activeClient = new DemoLogLensClient();
          user = await activeClient.getCurrentUser();
        }
      }
      currentUser.set(user);
      await loadWorkspaces();
    } catch {
      activeClient = new DemoLogLensClient();
      const user = await activeClient.getCurrentUser();
      currentUser.set(user);
      await loadWorkspaces();
    } finally {
      isLoading = false;
    }
  });

  async function loadWorkspaces() {
    try {
      const list = await activeClient.listWorkspaces();
      workspaces.set(list);
      if (list.length > 0) {
        currentWorkspace.set(list[0]);
        await loadWorkspaceData(list[0].id);
      }
    } catch (e) {
      console.error('Failed to load workspaces', e);
    }
  }

  async function loadWorkspaceData(wsId: string) {
    try {
      const [srcs, evs, grps] = await Promise.all([
        activeClient.listSources(wsId),
        activeClient.queryEvents(wsId, { search_query: $searchQuery }),
        activeClient.listGroups(wsId)
      ]);
      sources.set(srcs);
      events.set(evs);
      groups.set(grps);
    } catch (e) {
      console.error('Failed to load workspace data', e);
    }
  }

  $: if ($currentWorkspace && ($searchQuery !== undefined || $selectedSeverities)) {
    if (typeof window !== 'undefined') {
      loadWorkspaceData($currentWorkspace.id);
    }
  }

  async function handleAuthSubmit() {
    authError = '';
    try {
      let user;
      if (isBootstrap) {
        user = await activeClient.bootstrap(authEmail, authPass);
      } else {
        user = await activeClient.login(authEmail, authPass);
      }
      currentUser.set(user);
      showAuthModal = false;
      await loadWorkspaces();
    } catch (err: unknown) {
      authError = err instanceof Error ? err.message : 'Authentication failed';
    }
  }
</script>

{#if isLoading}
  <div class="h-screen w-screen bg-background text-gray-400 flex items-center justify-center text-xs">
    Loading LogLens Engine...
  </div>
{:else if showAuthModal}
  <div class="h-screen w-screen bg-background flex items-center justify-center p-4">
    <div class="bg-surface border border-border w-full max-w-sm rounded-lg p-6 space-y-4 text-xs">
      <div class="text-center space-y-1">
        <h1 class="text-base font-bold text-gray-100">LogLens Log Explorer</h1>
        <p class="text-gray-400 text-[11px]">
          {isBootstrap ? 'Initial Administrator Setup' : 'Sign in to access your logs'}
        </p>
      </div>

      {#if authError}
        <div class="p-2 bg-red-950/60 border border-red-800 text-red-300 rounded text-[11px]">
          {authError}
        </div>
      {/if}

      <div class="space-y-3">
        <div>
          <label for="auth-email" class="block text-gray-400 mb-1 font-medium">Email Address</label>
          <input
            id="auth-email"
            type="email"
            bind:value={authEmail}
            placeholder="admin@loglens.local"
            class="w-full bg-background border border-border rounded px-3 py-1.5 text-gray-200 focus:outline-none focus:border-blue-500"
          />
        </div>

        <div>
          <label for="auth-password" class="block text-gray-400 mb-1 font-medium">Password</label>
          <input
            id="auth-password"
            type="password"
            bind:value={authPass}
            placeholder="••••••••"
            class="w-full bg-background border border-border rounded px-3 py-1.5 text-gray-200 focus:outline-none focus:border-blue-500"
          />
        </div>

        <button
          on:click={handleAuthSubmit}
          class="w-full py-2 bg-blue-600 hover:bg-blue-500 text-white rounded font-bold transition-colors"
        >
          {isBootstrap ? 'Complete Setup' : 'Sign In'}
        </button>
      </div>
    </div>
  </div>
{:else}
  <div class="h-screen w-screen flex flex-col overflow-hidden">
    <Header />
    <div class="flex-1 flex overflow-hidden">
      <Sidebar />
      <main class="flex-1 flex overflow-hidden">
        {#if $activeTab === 'timeline'}
          <Timeline />
        {:else if $activeTab === 'groups'}
          <Groups />
        {:else if $activeTab === 'dashboard'}
          <Dashboard />
        {:else}
          <div class="flex-1 p-6 bg-background text-gray-300 text-xs">
            Log Sources Management
          </div>
        {/if}
        <EventDetails />
      </main>
    </div>
    <UploadModal />
  </div>
{/if}
