<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { onMount } from 'svelte';
  import ModeEditor from '$lib/components/ModeEditor.svelte';
  import { configStore, appStore } from '$lib/stores.js';

  /**
   * Local state
   */
  let modes = [];
  let selectedModeIndex = 0;
  let loading = false;
  let error = null;

  /**
   * Load configuration on mount
   */
  onMount(async () => {
    await loadConfig();
  });

  /**
   * Load configuration from daemon
   */
  async function loadConfig() {
    loading = true;
    error = null;
    try {
      const config = await configStore.fetch();
      if (config && config.modes) {
        modes = config.modes;
      }
    } catch (err) {
      error = err.message || String(err);
      appStore.setError(error);
    } finally {
      loading = false;
    }
  }

  /**
   * Save configuration to daemon
   */
  async function saveConfig() {
    loading = true;
    error = null;
    try {
      // Get current config
      const currentConfig = $configStore.config || {};

      // Update modes
      const updatedConfig = {
        ...currentConfig,
        modes
      };

      // Save via store
      await configStore.save(updatedConfig);

      // Reload daemon config
      // This will be handled by the daemon reload command

    } catch (err) {
      error = err.message || String(err);
      appStore.setError(error);
    } finally {
      loading = false;
    }
  }

  /**
   * Event handler: Mode selected
   */
  function handleModeSelected(event) {
    selectedModeIndex = event.detail.index;
  }

  /**
   * Event handler: Mode added
   */
  async function handleModeAdded(event) {
    const newMode = event.detail;
    modes = [...modes, newMode];
    selectedModeIndex = modes.length - 1;
    await saveConfig();
  }

  /**
   * Event handler: Mode updated
   */
  async function handleModeUpdated(event) {
    const { index, mode } = event.detail;
    modes = modes.map((m, i) => i === index ? mode : m);
    await saveConfig();
  }

  /**
   * Event handler: Mode deleted
   */
  async function handleModeDeleted(event) {
    const index = event.detail.index;
    modes = modes.filter((_, i) => i !== index);

    // Adjust selected index if needed
    if (selectedModeIndex >= modes.length) {
      selectedModeIndex = Math.max(0, modes.length - 1);
    }

    await saveConfig();
  }

  /**
   * Event handler: Mode reordered
   */
  async function handleModeReordered(event) {
    const { fromIndex, toIndex } = event.detail;
    const newModes = [...modes];
    const [removed] = newModes.splice(fromIndex, 1);
    newModes.splice(toIndex, 0, removed);
    modes = newModes;

    // Update selected index
    if (selectedModeIndex === fromIndex) {
      selectedModeIndex = toIndex;
    } else if (selectedModeIndex > fromIndex && selectedModeIndex <= toIndex) {
      selectedModeIndex--;
    } else if (selectedModeIndex < fromIndex && selectedModeIndex >= toIndex) {
      selectedModeIndex++;
    }

    await saveConfig();
  }

  /**
   * Reactive: Subscribe to config store
   */
  $: if ($configStore.config && $configStore.config.modes) {
    modes = $configStore.config.modes;
  }
</script>

<div class="view">
  <header class="view-header">
    <div class="header-content">
      <div>
        <h2>Mode Configuration</h2>
        <p class="subtitle">Create and manage mapping modes with different button layouts</p>
      </div>
      {#if loading}
        <div class="loading-indicator">
          <span class="spinner"></span>
          Saving...
        </div>
      {/if}
    </div>
  </header>

  <div class="content">
    {#if error}
      <div class="error-banner">
        <span class="error-icon">⚠️</span>
        <span>{error}</span>
        <button class="dismiss-btn" on:click={() => { error = null; appStore.clearError(); }}>
          ✕
        </button>
      </div>
    {/if}

    <ModeEditor
      bind:modes
      bind:selectedModeIndex
      on:modeSelected={handleModeSelected}
      on:modeAdded={handleModeAdded}
      on:modeUpdated={handleModeUpdated}
      on:modeDeleted={handleModeDeleted}
      on:modeReordered={handleModeReordered}
    />
  </div>
</div>

<style>
  .view {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .view-header {
    padding: 2rem 2.5rem 1.5rem;
    border-bottom: 1px solid #333;
    background: #1e1e1e;
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .view-header h2 {
    margin: 0 0 0.5rem;
    font-size: 1.75rem;
    color: #e0e0e0;
  }

  .subtitle {
    margin: 0;
    color: #999;
    font-size: 0.95rem;
  }

  .loading-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #4a9eff;
    font-size: 0.9rem;
  }

  .spinner {
    display: inline-block;
    width: 16px;
    height: 16px;
    border: 2px solid #4a9eff;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .error-banner {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem 1.5rem;
    margin: 1.5rem 2.5rem;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 8px;
    color: #ef4444;
  }

  .error-icon {
    font-size: 1.25rem;
  }

  .dismiss-btn {
    margin-left: auto;
    background: none;
    border: none;
    color: #ef4444;
    cursor: pointer;
    font-size: 1.25rem;
    padding: 0.25rem 0.5rem;
    opacity: 0.6;
    transition: opacity 0.2s;
  }

  .dismiss-btn:hover {
    opacity: 1;
  }
</style>
