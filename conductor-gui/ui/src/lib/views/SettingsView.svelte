<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { onMount } from 'svelte';
  import SettingsPanel from '$lib/components/SettingsPanel.svelte';
  import LiveEventConsole from '$lib/components/LiveEventConsole.svelte';
  import { configStore, appStore } from '$lib/stores.js';
  import { invoke } from '@tauri-apps/api/core';

  /**
   * Local state
   */
  let configPath = '';
  let settings = {
    autoStart: false,
    logLevel: 'info',
    theme: 'dark',
  };
  let showEventConsole = false;
  let loading = false;
  let error = null;

  /**
   * Load settings on mount
   */
  onMount(async () => {
    await loadConfigPath();
    await loadSettings();
  });

  /**
   * Load config file path
   */
  async function loadConfigPath() {
    try {
      configPath = await invoke('get_config_path');
    } catch (err) {
      error = err.toString();
    }
  }

  /**
   * Load settings from config
   */
  async function loadSettings() {
    loading = true;
    try {
      const config = await configStore.fetch();
      if (config && config.settings) {
        settings = { ...settings, ...config.settings };
      }
    } catch (err) {
      error = err.message || String(err);
    } finally {
      loading = false;
    }
  }

  /**
   * Save settings to config
   */
  async function saveSettings() {
    loading = true;
    error = null;
    try {
      const config = $configStore.config || {};
      const updatedConfig = {
        ...config,
        settings
      };

      await configStore.save(updatedConfig);
      appStore.setError(null);
    } catch (err) {
      error = err.message || String(err);
      appStore.setError(error);
    } finally {
      loading = false;
    }
  }

  /**
   * Handle settings change
   */
  async function handleSettingsChange(event) {
    settings = event.detail.settings;
    await saveSettings();
  }

  /**
   * Toggle event console
   */
  function toggleEventConsole() {
    showEventConsole = !showEventConsole;
  }

  /**
   * Open config file in editor
   */
  async function openConfigFile() {
    if (!configPath) return;

    try {
      // Platform-specific file opening
      if (navigator.platform.toLowerCase().includes('mac')) {
        await invoke('open_file', { path: configPath, editor: true });
      } else {
        // Generic fallback
        window.open(`file://${configPath}`);
      }
    } catch (err) {
      error = `Failed to open config file: ${err}`;
      appStore.setError(error);
    }
  }

  /**
   * Copy config path to clipboard
   */
  async function copyConfigPath() {
    if (!configPath) return;

    try {
      await navigator.clipboard.writeText(configPath);
    } catch (err) {
      console.warn('Failed to copy to clipboard:', err);
    }
  }
</script>

<div class="view">
  <header class="view-header">
    <div class="header-content">
      <div>
        <h2>Settings</h2>
        <p class="subtitle">Application and daemon configuration</p>
      </div>
      <div class="header-actions">
        {#if loading}
          <div class="loading-indicator">
            <span class="spinner"></span>
            Saving...
          </div>
        {/if}
        <button class="btn-secondary" on:click={toggleEventConsole}>
          {showEventConsole ? '📊 Hide' : '📊 Show'} Event Console
        </button>
      </div>
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

    <section class="settings-section">
      <h3>Configuration File</h3>
      <div class="setting-group">
        <label for="config-path">Config File Path:</label>
        {#if configPath}
          <div class="path-display">
            <input
              id="config-path"
              type="text"
              value={configPath}
              readonly
              class="config-path-input"
            />
            <button class="btn-icon" on:click={copyConfigPath} title="Copy to clipboard">
              📋
            </button>
            <button class="btn-icon" on:click={openConfigFile} title="Open in editor">
              📝
            </button>
          </div>
        {:else if error}
          <p class="error-msg">{error}</p>
        {:else}
          <p class="loading-msg">Loading...</p>
        {/if}
      </div>
    </section>

    <section class="settings-section">
      <h3>Application Settings</h3>
      <SettingsPanel
        bind:settings
        on:settingsChange={handleSettingsChange}
      />
    </section>

    {#if showEventConsole}
      <section class="settings-section">
        <h3>Live Event Console</h3>
        <div class="console-container">
          <LiveEventConsole />
        </div>
      </section>
    {/if}
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

  .header-actions {
    display: flex;
    gap: 1rem;
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

  .content {
    flex: 1;
    padding: 2rem 2.5rem;
    overflow-y: auto;
  }

  .settings-section {
    margin-bottom: 3rem;
  }

  h3 {
    margin: 0 0 1.5rem;
    font-size: 1.25rem;
    color: #e0e0e0;
    font-weight: 600;
  }

  .setting-group {
    margin-bottom: 1.5rem;
  }

  label {
    display: block;
    margin-bottom: 0.75rem;
    font-weight: 500;
    color: #999;
    font-size: 0.95rem;
  }

  .path-display {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .config-path-input {
    flex: 1;
    padding: 0.75rem;
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 6px;
    color: #e0e0e0;
    font-family: 'Monaco', 'Courier New', monospace;
    font-size: 0.85rem;
  }

  .config-path-input:focus {
    outline: none;
    border-color: #4a9eff;
  }

  .btn-icon {
    padding: 0.75rem;
    background: #333;
    border: 1px solid #444;
    border-radius: 6px;
    color: #e0e0e0;
    cursor: pointer;
    font-size: 1.1rem;
    transition: all 0.2s;
    min-width: 44px;
  }

  .btn-icon:hover {
    background: #3a3a3a;
    border-color: #555;
  }

  .console-container {
    background: #2a2a2a;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1rem;
    max-height: 400px;
    overflow: hidden;
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

  .error-banner {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem 1.5rem;
    margin-bottom: 2rem;
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

  .error-msg {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    padding: 1rem;
    border-radius: 6px;
    margin: 0;
  }

  .loading-msg {
    color: #999;
    font-style: italic;
    margin: 0;
  }

  .btn-secondary {
    padding: 0.6rem 1.25rem;
    border-radius: 6px;
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    background: #333;
    border: 1px solid #444;
    color: #e0e0e0;
  }

  .btn-secondary:hover {
    background: #3a3a3a;
  }
</style>
