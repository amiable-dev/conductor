<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let gamepads = [];
  let loading = true;
  let error = null;

  /**
   * Load gamepad devices
   */
  async function loadGamepads() {
    try {
      loading = true;
      error = null;
      gamepads = await invoke('list_gamepads');
    } catch (err) {
      console.error('Failed to load gamepads:', err);
      error = err.message || String(err);
      gamepads = [];
    } finally {
      loading = false;
    }
  }

  /**
   * Refresh device list
   */
  function handleRefresh() {
    loadGamepads();
  }

  /**
   * Load on mount and set up auto-refresh
   */
  onMount(() => {
    loadGamepads();

    // Auto-refresh every 5 seconds to detect newly connected gamepads
    const interval = setInterval(loadGamepads, 5000);

    return () => {
      clearInterval(interval);
    };
  });

  /**
   * Get controller icon based on name/uuid
   */
  function getControllerIcon(gamepad) {
    const name = gamepad.name.toLowerCase();
    if (name.includes('xbox')) return '🎮';
    if (name.includes('playstation') || name.includes('dualshock') || name.includes('dualsense')) return '🕹️';
    if (name.includes('nintendo') || name.includes('switch')) return '🎮';
    return '🎮';
  }
</script>

<div class="hid-device-list">
  {#if loading && gamepads.length === 0}
    <div class="loading-state">
      <span class="spinner"></span>
      <p>Detecting gamepads...</p>
    </div>
  {:else if error}
    <div class="error-state">
      <p class="error-msg">⚠️ {error}</p>
      <button class="btn-secondary" on:click={handleRefresh}>Retry</button>
    </div>
  {:else if gamepads.length === 0}
    <div class="empty-state">
      <p class="hint">No gamepads detected</p>
      <p class="sub-hint">Connect a gamepad (Xbox, PlayStation, Switch) to get started</p>
      <button class="btn-secondary" on:click={handleRefresh}>🔄 Refresh</button>
    </div>
  {:else}
    <div class="device-grid">
      {#each gamepads as gamepad (gamepad.id)}
        <div class="device-card" class:connected={gamepad.connected}>
          <div class="device-header">
            <span class="device-icon">{getControllerIcon(gamepad)}</span>
            <span class="device-status {gamepad.connected ? 'connected' : 'disconnected'}">
              {gamepad.connected ? '●' : '○'}
            </span>
          </div>
          <div class="device-info">
            <h4 class="device-name">{gamepad.name}</h4>
            <p class="device-id">ID: {gamepad.id}</p>
            {#if gamepad.uuid}
              <p class="device-uuid">UUID: {gamepad.uuid.substring(0, 8)}...</p>
            {/if}
          </div>
        </div>
      {/each}
    </div>
    <div class="actions">
      <button class="btn-secondary" on:click={handleRefresh}>🔄 Refresh</button>
    </div>
  {/if}
</div>

<style>
  .hid-device-list {
    min-height: 200px;
  }

  .loading-state,
  .error-state,
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem 1rem;
    text-align: center;
  }

  .spinner {
    display: inline-block;
    width: 24px;
    height: 24px;
    border: 3px solid #4a9eff;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-bottom: 1rem;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .hint {
    color: #999;
    margin: 0 0 0.5rem;
    font-size: 1.1rem;
  }

  .sub-hint {
    color: #666;
    margin: 0 0 1.5rem;
    font-size: 0.9rem;
  }

  .error-msg {
    color: #ef4444;
    margin: 0 0 1rem;
  }

  .device-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .device-card {
    background: #2a2a2a;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1.25rem;
    transition: all 0.2s;
  }

  .device-card:hover {
    border-color: #4a9eff;
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(74, 158, 255, 0.15);
  }

  .device-card.connected {
    border-color: #4ade80;
  }

  .device-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .device-icon {
    font-size: 2rem;
  }

  .device-status {
    font-size: 1.25rem;
  }

  .device-status.connected {
    color: #4ade80;
  }

  .device-status.disconnected {
    color: #666;
  }

  .device-info {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .device-name {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: #e0e0e0;
  }

  .device-id,
  .device-uuid {
    margin: 0;
    font-size: 0.85rem;
    color: #999;
    font-family: 'Monaco', 'Courier New', monospace;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 1rem;
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
