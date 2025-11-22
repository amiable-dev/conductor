<script>
  import { onMount } from 'svelte';
  import Sidebar from './lib/components/Sidebar.svelte';
  import StatusBar from './lib/components/StatusBar.svelte';
  import DevicesView from './lib/views/DevicesView.svelte';
  import ModesView from './lib/views/ModesView.svelte';
  import MappingsView from './lib/views/MappingsView.svelte';
  import PluginsView from './lib/views/PluginsView.svelte';
  import SettingsView from './lib/views/SettingsView.svelte';
  import { currentSection, restoreNavigationState, SECTIONS } from './lib/stores/navigation.js';

  // Restore navigation state on mount
  onMount(() => {
    restoreNavigationState();
  });
</script>

<div class="app">
  <Sidebar />

  <div class="content-wrapper">
    <StatusBar />

    <main class="main-content">
      {#if $currentSection === SECTIONS.DEVICES}
        <DevicesView />
      {:else if $currentSection === SECTIONS.MODES}
        <ModesView />
      {:else if $currentSection === SECTIONS.MAPPINGS}
        <MappingsView />
      {:else if $currentSection === SECTIONS.PLUGINS}
        <PluginsView />
      {:else if $currentSection === SECTIONS.SETTINGS}
        <SettingsView />
      {/if}
    </main>
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
                 'Helvetica Neue', Arial, sans-serif;
    background: #1e1e1e;
    color: #e0e0e0;
    overflow: hidden;
  }

  :global(*) {
    box-sizing: border-box;
  }

  :global(button) {
    font-family: inherit;
  }

  .app {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  .content-wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: #1e1e1e;
    overflow: hidden;
  }

  /* Scrollbar styling */
  :global(::-webkit-scrollbar) {
    width: 8px;
  }

  :global(::-webkit-scrollbar-track) {
    background: #1a1a1a;
  }

  :global(::-webkit-scrollbar-thumb) {
    background: #444;
    border-radius: 4px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: #555;
  }
</style>
