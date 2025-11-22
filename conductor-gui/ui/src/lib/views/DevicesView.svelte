<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { onMount } from 'svelte';
  import DeviceList from '$lib/components/DeviceList.svelte';
  import HidDeviceList from '$lib/components/HidDeviceList.svelte';
  import TemplateSelector from '$lib/components/TemplateSelector.svelte';
  import ProfileManager from '$lib/components/ProfileManager.svelte';
  import { statusStore, devicesStore, appStore } from '$lib/stores.js';
  import api from '$lib/api.js';
  import { confirm } from '@tauri-apps/plugin-dialog';
  import { message } from '@tauri-apps/plugin-dialog';

  /**
   * Local state
   */
  let templates = [];
  let selectedTemplate = null;
  let showTemplateSelector = false;
  let showProfileManager = false;
  let loading = false;
  let error = null;

  /**
   * Load templates on mount
   */
  onMount(async () => {
    await loadTemplates();
    statusStore.startAutoRefresh();
    devicesStore.startAutoRefresh();

    return () => {
      statusStore.stopAutoRefresh();
      devicesStore.stopAutoRefresh();
    };
  });

  /**
   * Load device templates
   */
  async function loadTemplates() {
    try {
      templates = await api.templates.list();
    } catch (err) {
      console.warn('Failed to load templates:', err);
    }
  }

  /**
   * Open template selector
   */
  function openTemplateSelector() {
    showTemplateSelector = true;
  }

  /**
   * Open profile manager
   */
  function openProfileManager() {
    showProfileManager = true;
  }

  /**
   * Handle template selection
   */
  async function handleTemplateSelected(event) {
    console.log('Template selected event:', event.detail);
    selectedTemplate = event.detail.template;
    showTemplateSelector = false;

    // Create config from template
    try {
      loading = true;
      error = null;

      console.log('Creating config from template:', selectedTemplate.id);
      const resultMessage = await api.templates.createConfig(selectedTemplate.id);
      console.log('Config created, message:', resultMessage);

      appStore.setError(null);

      // Show success message and prompt for reload
      const confirmMessage = `${resultMessage}\n\nWould you like to reload the daemon to apply the new configuration?`;
      console.log('Showing confirm dialog:', confirmMessage);

      const doReload = await confirm(confirmMessage, {
        title: 'Configuration Created',
        kind: 'info',
        okLabel: 'Reload Daemon',
        cancelLabel: 'Not Now'
      });
      console.log('User response:', doReload);

      if (doReload) {
        console.log('Reloading daemon...');
        await api.daemon.reload();
        console.log('Daemon reloaded successfully');
        await message('Configuration reloaded successfully!', {
          title: 'Success',
          kind: 'info'
        });
      } else {
        await message('Configuration saved. Reload the daemon manually to apply changes.', {
          title: 'Configuration Saved',
          kind: 'info'
        });
      }
    } catch (err) {
      console.error('Error creating config from template:', err);
      error = err.message || String(err);
      appStore.setError(error);
      await message(`Error: ${error}`, {
        title: 'Error',
        kind: 'error'
      });
    } finally {
      loading = false;
    }
  }

  /**
   * Handle profile switch
   */
  function handleProfileSwitch(event) {
    const profileId = event.detail.profileId;
    // Profile switching is automatic via app detection
    // This is just for manual override if needed
  }
</script>

<div class="view">
  <header class="view-header">
    <div class="header-content">
      <div>
        <h2>Devices & Profiles</h2>
        <p class="subtitle">Manage input device connections, templates, and per-app profiles</p>
      </div>
      <div class="header-actions">
        {#if loading}
          <div class="loading-indicator">
            <span class="spinner"></span>
            Loading...
          </div>
        {/if}
        <button class="btn-secondary" on:click={openTemplateSelector}>
          📋 Device Templates
        </button>
        <button class="btn-secondary" on:click={openProfileManager}>
          🔄 Profiles
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

    <section class="status-section">
      <h3>Daemon Status</h3>
      {#if $statusStore.status}
        <div class="status-card">
          <div class="status-row">
            <span class="label">Running:</span>
            <span class="value {$statusStore.status.running ? 'success' : 'error'}">
              {$statusStore.status.running ? '✅ Yes' : '❌ No'}
            </span>
          </div>
          <div class="status-row">
            <span class="label">Connected:</span>
            <span class="value {$statusStore.status.connected ? 'success' : 'error'}">
              {$statusStore.status.connected ? '✅ Yes' : '❌ No'}
            </span>
          </div>
          {#if $statusStore.status.lifecycle_state}
            <div class="status-row">
              <span class="label">State:</span>
              <span class="value">{$statusStore.status.lifecycle_state}</span>
            </div>
          {/if}
          {#if $statusStore.status.input_mode}
            <div class="status-row">
              <span class="label">Input Mode:</span>
              <span class="value">{$statusStore.status.input_mode}</span>
            </div>
          {/if}
          {#if $statusStore.status.hid_devices && $statusStore.status.hid_devices.length > 0}
            <div class="status-row">
              <span class="label">Gamepads:</span>
              <span class="value">
                {$statusStore.status.hid_devices.map(d => d.name).join(', ')}
              </span>
            </div>
          {/if}
          {#if $statusStore.status.uptime_secs !== null && $statusStore.status.uptime_secs !== undefined}
            <div class="status-row">
              <span class="label">Uptime:</span>
              <span class="value">
                {Math.floor($statusStore.status.uptime_secs / 60)}m {$statusStore.status.uptime_secs % 60}s
              </span>
            </div>
          {/if}
          {#if $statusStore.status.events_processed !== null && $statusStore.status.events_processed !== undefined}
            <div class="status-row">
              <span class="label">Events Processed:</span>
              <span class="value">{$statusStore.status.events_processed.toLocaleString()}</span>
            </div>
          {/if}
          {#if $statusStore.status.error}
            <div class="status-row">
              <span class="label">Error:</span>
              <span class="value error">{$statusStore.status.error}</span>
            </div>
          {/if}
        </div>
      {:else if $statusStore.error}
        <p class="error-msg">{$statusStore.error}</p>
      {:else}
        <p class="loading-msg">Loading status...</p>
      {/if}
    </section>

    <section class="devices-section">
      <h3>MIDI Controllers</h3>
      <DeviceList />
    </section>

    <section class="devices-section">
      <h3>HID Gamepads</h3>
      <HidDeviceList />
    </section>
  </div>
</div>

{#if showTemplateSelector}
  <TemplateSelector
    on:selected={handleTemplateSelected}
    on:close={() => showTemplateSelector = false}
  />
{/if}

{#if showProfileManager}
  <ProfileManager
    on:profileSwitch={handleProfileSwitch}
    on:close={() => showProfileManager = false}
  />
{/if}

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

  .status-section,
  .devices-section {
    margin-bottom: 3rem;
  }

  h3 {
    margin: 0 0 1.5rem;
    font-size: 1.25rem;
    color: #e0e0e0;
    font-weight: 600;
  }

  .status-card {
    background: #2a2a2a;
    border-radius: 8px;
    padding: 1.5rem;
    border: 1px solid #333;
  }

  .status-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 0;
    border-bottom: 1px solid #333;
  }

  .status-row:last-child {
    border-bottom: none;
  }

  .label {
    font-weight: 500;
    color: #999;
    font-size: 0.95rem;
  }

  .value {
    color: #e0e0e0;
    font-weight: 500;
    font-size: 0.95rem;
  }

  .value.success {
    color: #4ade80;
  }

  .value.error {
    color: #ef4444;
  }

  .error-msg {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    padding: 1rem;
    border-radius: 6px;
  }

  .loading-msg {
    color: #999;
    font-style: italic;
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

  .btn-primary,
  .btn-secondary {
    padding: 0.6rem 1.25rem;
    border-radius: 6px;
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-primary {
    background: #4a9eff;
    border: none;
    color: white;
  }

  .btn-primary:hover {
    background: #3a8eef;
  }

  .btn-secondary {
    background: #333;
    border: 1px solid #444;
    color: #e0e0e0;
  }

  .btn-secondary:hover {
    background: #3a3a3a;
  }
</style>
