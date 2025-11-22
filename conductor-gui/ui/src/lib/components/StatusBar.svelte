<script>
  import { onMount, onDestroy } from 'svelte';
  import { statusStore } from '../stores.js';

  function getStatusColor(status) {
    if (!status || !status.running) return '#ff6b6b'; // Red - Not running
    if (!status.connected) return '#ffa500'; // Orange - Not connected
    if (status.error) return '#ffa500'; // Orange - Error
    return '#6ad98b'; // Green - Running OK
  }

  function getStatusText(status) {
    if (!status || !status.running) return 'Daemon Stopped';
    if (!status.connected) return 'Disconnected';
    if (status.error) return 'Error';
    return 'Running';
  }

  function getCurrentMode(status) {
    if (!status || !status.lifecycle_state) return 'Unknown';
    return status.lifecycle_state;
  }

  function getConnectedDevice(status) {
    if (!status || !status.device) return 'No Device';
    if (!status.device.connected) return 'No Device';
    return status.device.name || `Port ${status.device.port}`;
  }

  function formatUptime(seconds) {
    if (seconds === null || seconds === undefined) return '--';
    const hours = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    if (hours > 0) return `${hours}h ${mins}m`;
    if (mins > 0) return `${mins}m ${secs}s`;
    return `${secs}s`;
  }

  onMount(() => {
    statusStore.startAutoRefresh();
  });

  onDestroy(() => {
    statusStore.stopAutoRefresh();
  });
</script>

<div class="status-bar">
  <div class="status-section">
    <div class="status-indicator" style="background-color: {getStatusColor($statusStore.status)}"></div>
    <span class="status-label">Status:</span>
    <span class="status-value">{getStatusText($statusStore.status)}</span>
  </div>

  <div class="status-section">
    <span class="status-label">Mode:</span>
    <span class="status-value">{getCurrentMode($statusStore.status)}</span>
  </div>

  <div class="status-section">
    <span class="status-label">Device:</span>
    <span class="status-value device-name">{getConnectedDevice($statusStore.status)}</span>
  </div>

  {#if $statusStore.status && $statusStore.status.uptime_secs !== null && $statusStore.status.uptime_secs !== undefined}
    <div class="status-section">
      <span class="status-label">Uptime:</span>
      <span class="status-value">{formatUptime($statusStore.status.uptime_secs)}</span>
    </div>
  {/if}

  {#if $statusStore.status && $statusStore.status.events_processed !== null && $statusStore.status.events_processed !== undefined}
    <div class="status-section">
      <span class="status-label">Events:</span>
      <span class="status-value">{$statusStore.status.events_processed.toLocaleString()}</span>
    </div>
  {/if}

  {#if $statusStore.error}
    <div class="status-section error-section">
      <span class="error-icon">⚠️</span>
      <span class="error-message">{$statusStore.error}</span>
    </div>
  {/if}
</div>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    gap: 2rem;
    padding: 0.75rem 1.5rem;
    background: #252525;
    border-bottom: 1px solid #333;
    font-size: 0.85rem;
  }

  .status-section {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    animation: pulse 2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.6;
    }
  }

  .status-label {
    color: #999;
    font-weight: 500;
  }

  .status-value {
    color: #e0e0e0;
    font-weight: 400;
  }

  .device-name {
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 0.8rem;
  }

  .error-section {
    margin-left: auto;
    color: #ff6b6b;
  }

  .error-icon {
    font-size: 1rem;
  }

  .error-message {
    color: #ff6b6b;
    font-size: 0.8rem;
    max-width: 300px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  @media (max-width: 1024px) {
    .status-bar {
      flex-wrap: wrap;
      gap: 1rem;
    }
  }

  @media (max-width: 768px) {
    .status-bar {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.75rem;
    }

    .status-section {
      width: 100%;
    }

    .error-section {
      margin-left: 0;
    }
  }
</style>
