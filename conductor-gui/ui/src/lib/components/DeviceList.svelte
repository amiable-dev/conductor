<script>
  import { onMount, onDestroy } from 'svelte';
  import { devicesStore } from '../stores.js';

  async function handleRefresh() {
    await devicesStore.fetch();
  }

  onMount(() => {
    devicesStore.startAutoRefresh();
  });

  onDestroy(() => {
    devicesStore.stopAutoRefresh();
  });
</script>

<div class="device-list">
  <div class="device-list-header">
    <h3>Available MIDI Devices</h3>
    <button class="refresh-button" on:click={handleRefresh} disabled={$devicesStore.loading}>
      <span class="icon">{$devicesStore.loading ? '⏳' : '🔄'}</span>
      <span class="label">Refresh</span>
    </button>
  </div>

  {#if $devicesStore.loading && $devicesStore.devices.length === 0}
    <div class="loading-state">
      <p>Scanning for MIDI devices...</p>
    </div>
  {:else if $devicesStore.error}
    <div class="error-state">
      <p class="error-message">{$devicesStore.error}</p>
      <button class="retry-button" on:click={handleRefresh}>
        Try Again
      </button>
    </div>
  {:else if $devicesStore.devices.length === 0}
    <div class="empty-state">
      <p class="empty-icon">🎹</p>
      <p class="empty-title">No MIDI Devices Found</p>
      <p class="empty-description">
        Please connect a MIDI device and click refresh.
      </p>
    </div>
  {:else}
    <div class="devices">
      {#each $devicesStore.devices as device, index (device.index)}
        <div class="device-card" class:connected={device.connected}>
          <div class="device-info">
            <div class="device-icon">
              {device.connected ? '✅' : '🎹'}
            </div>
            <div class="device-details">
              <div class="device-name">{device.name}</div>
              <div class="device-meta">
                <span class="device-index">Port {device.index}</span>
                <span class="device-status">
                  {device.connected ? 'Connected' : 'Available'}
                </span>
              </div>
            </div>
          </div>
          <div class="device-actions">
            {#if device.connected}
              <button class="action-button disconnect" disabled>
                Disconnect
              </button>
            {:else}
              <button class="action-button connect" disabled>
                Connect
              </button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="device-list-footer">
    <p class="device-count">
      {$devicesStore.devices.length} {$devicesStore.devices.length === 1 ? 'device' : 'devices'} found
    </p>
    <p class="auto-refresh">Auto-refresh every 5 seconds</p>
  </div>
</div>

<style>
  .device-list {
    background: #2a2a2a;
    border-radius: 8px;
    border: 1px solid #333;
    overflow: hidden;
  }

  .device-list-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.25rem 1.5rem;
    border-bottom: 1px solid #333;
  }

  .device-list-header h3 {
    margin: 0;
    font-size: 1.1rem;
    color: #e0e0e0;
  }

  .refresh-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: #3a3a3a;
    border: 1px solid #444;
    border-radius: 4px;
    color: #e0e0e0;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .refresh-button:hover:not(:disabled) {
    background: #444;
    border-color: #555;
  }

  .refresh-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .refresh-button .icon {
    font-size: 1rem;
  }

  .loading-state,
  .error-state,
  .empty-state {
    padding: 3rem 2rem;
    text-align: center;
  }

  .loading-state p {
    color: #999;
    margin: 0;
  }

  .error-state {
    color: #ff6b6b;
  }

  .error-message {
    margin: 0 0 1rem;
  }

  .retry-button {
    padding: 0.5rem 1.5rem;
    background: #ff6b6b;
    border: none;
    border-radius: 4px;
    color: #fff;
    font-size: 0.9rem;
    cursor: pointer;
    transition: background 0.2s ease;
  }

  .retry-button:hover {
    background: #ff5252;
  }

  .empty-icon {
    font-size: 3rem;
    margin: 0 0 1rem;
  }

  .empty-title {
    font-size: 1.1rem;
    color: #e0e0e0;
    margin: 0 0 0.5rem;
    font-weight: 500;
  }

  .empty-description {
    color: #999;
    margin: 0;
    font-size: 0.9rem;
  }

  .devices {
    padding: 1rem;
  }

  .device-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.25rem;
    background: #333;
    border: 1px solid #444;
    border-radius: 6px;
    margin-bottom: 0.75rem;
    transition: all 0.2s ease;
  }

  .device-card:last-child {
    margin-bottom: 0;
  }

  .device-card:hover {
    background: #3a3a3a;
    border-color: #555;
  }

  .device-card.connected {
    background: #1a3a2a;
    border-color: #2a5a3a;
  }

  .device-card.connected:hover {
    background: #1e4230;
  }

  .device-info {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex: 1;
  }

  .device-icon {
    font-size: 1.5rem;
  }

  .device-details {
    flex: 1;
  }

  .device-name {
    font-size: 1rem;
    color: #e0e0e0;
    font-weight: 500;
    margin-bottom: 0.25rem;
  }

  .device-meta {
    display: flex;
    gap: 1rem;
    font-size: 0.85rem;
  }

  .device-index {
    color: #888;
  }

  .device-status {
    color: #999;
  }

  .device-card.connected .device-status {
    color: #6ad98b;
  }

  .device-actions {
    margin-left: 1rem;
  }

  .action-button {
    padding: 0.5rem 1.25rem;
    border: none;
    border-radius: 4px;
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .action-button.connect {
    background: #4a9eff;
    color: #fff;
  }

  .action-button.connect:hover:not(:disabled) {
    background: #3a8eef;
  }

  .action-button.disconnect {
    background: #ff6b6b;
    color: #fff;
  }

  .action-button.disconnect:hover:not(:disabled) {
    background: #ff5252;
  }

  .action-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .device-list-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    border-top: 1px solid #333;
    background: #252525;
  }

  .device-count {
    margin: 0;
    font-size: 0.85rem;
    color: #999;
  }

  .auto-refresh {
    margin: 0;
    font-size: 0.85rem;
    color: #666;
  }

  @media (max-width: 768px) {
    .device-card {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }

    .device-actions {
      width: 100%;
      margin-left: 0;
    }

    .action-button {
      width: 100%;
    }

    .device-list-footer {
      flex-direction: column;
      gap: 0.5rem;
      align-items: flex-start;
    }
  }
</style>
