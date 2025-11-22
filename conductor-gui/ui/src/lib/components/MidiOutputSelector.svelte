<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { midiOutputPortsStore } from '../stores.js';

  /**
   * Props
   */
  export let selectedPort = null; // Current selected port name
  export let readonly = false; // Readonly mode
  export let showTestButton = true; // Show test output button
  export let showPlatformBadge = true; // Show platform badge

  const dispatch = createEventDispatcher();

  /**
   * Load ports on mount
   */
  onMount(async () => {
    await handleRefresh();
  });

  /**
   * Handle port selection change
   */
  function handlePortChange(event) {
    const portName = event.target.value;
    midiOutputPortsStore.selectPort(portName);
    selectedPort = portName;
    dispatch('change', { portName });
  }

  /**
   * Handle refresh button click
   */
  async function handleRefresh() {
    try {
      await midiOutputPortsStore.fetch();
      // Update parent component with first port if none selected
      if (!selectedPort && $midiOutputPortsStore.ports.length > 0) {
        selectedPort = $midiOutputPortsStore.ports[0].name;
        midiOutputPortsStore.selectPort(selectedPort);
        dispatch('change', { portName: selectedPort });
      }
    } catch (error) {
      console.error('Failed to fetch MIDI output ports:', error);
    }
  }

  /**
   * Handle test output button click
   */
  async function handleTest() {
    try {
      midiOutputPortsStore.clearMessages();
      await midiOutputPortsStore.testPort('note_on', 0, {
        note: 60,  // Middle C
        velocity: 100,
      });
    } catch (error) {
      console.error('Failed to test MIDI output:', error);
    }
  }

  /**
   * Get badge class for port type
   */
  function getPortBadgeClass(port) {
    return port.is_virtual ? 'port-badge virtual' : 'port-badge physical';
  }

  /**
   * Get platform emoji
   */
  function getPlatformEmoji(platform) {
    const emojis = {
      'macOS': '🍎',
      'Linux': '🐧',
      'Windows': '🪟',
    };
    return emojis[platform] || '💻';
  }
</script>

<div class="midi-output-selector">
  <div class="selector-header">
    <label for="midi-port-select" class="selector-label">
      MIDI Output Port
    </label>
    <button
      class="refresh-button"
      on:click={handleRefresh}
      disabled={$midiOutputPortsStore.loading || readonly}
      title="Refresh port list"
    >
      <span class="icon">{$midiOutputPortsStore.loading ? '⏳' : '🔄'}</span>
    </button>
  </div>

  {#if $midiOutputPortsStore.loading && $midiOutputPortsStore.ports.length === 0}
    <div class="loading-state">
      <p>Scanning for MIDI output ports...</p>
    </div>
  {:else if $midiOutputPortsStore.error}
    <div class="error-state">
      <p class="error-message">{$midiOutputPortsStore.error}</p>
      <button class="retry-button" on:click={handleRefresh}>
        Try Again
      </button>
    </div>
  {:else if $midiOutputPortsStore.ports.length === 0}
    <div class="empty-state">
      <p class="empty-icon">🎹</p>
      <p class="empty-title">No MIDI Output Ports Found</p>
      <p class="empty-description">
        Please ensure MIDI output devices are connected or create a virtual MIDI port.
      </p>
    </div>
  {:else}
    <div class="port-selector-group">
      <select
        id="midi-port-select"
        class="port-select"
        value={selectedPort}
        on:change={handlePortChange}
        disabled={readonly}
      >
        {#each $midiOutputPortsStore.ports as port (port.index)}
          <option value={port.name}>
            {port.name}
            {#if port.is_virtual}(Virtual){/if}
          </option>
        {/each}
      </select>

      {#if $midiOutputPortsStore.selectedPort && $midiOutputPortsStore.ports.length > 0}
        {@const selectedPortData = $midiOutputPortsStore.ports.find(
          p => p.name === $midiOutputPortsStore.selectedPort
        )}
        {#if selectedPortData}
          <div class="port-metadata">
            <span class={getPortBadgeClass(selectedPortData)}>
              {selectedPortData.is_virtual ? '🔷 Virtual' : '🔌 Physical'}
            </span>
            {#if showPlatformBadge}
              <span class="platform-badge">
                {getPlatformEmoji(selectedPortData.platform)} {selectedPortData.platform}
              </span>
            {/if}
          </div>
        {/if}
      {/if}
    </div>

    {#if showTestButton && !readonly}
      <div class="test-section">
        <button
          class="test-button"
          on:click={handleTest}
          disabled={!$midiOutputPortsStore.selectedPort || $midiOutputPortsStore.loading}
        >
          <span class="icon">🎵</span>
          <span class="label">Test Output (Middle C)</span>
        </button>

        {#if $midiOutputPortsStore.testResult}
          <div class="test-success">
            ✅ {$midiOutputPortsStore.testResult}
          </div>
        {/if}

        {#if $midiOutputPortsStore.error}
          <div class="test-error">
            ❌ {$midiOutputPortsStore.error}
          </div>
        {/if}
      </div>
    {/if}
  {/if}
</div>

<style>
  .midi-output-selector {
    background: #2a2a2a;
    border-radius: 8px;
    border: 1px solid #333;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .selector-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .selector-label {
    font-size: 0.95rem;
    font-weight: 600;
    color: #e0e0e0;
  }

  .refresh-button {
    background: #333;
    border: 1px solid #444;
    border-radius: 4px;
    padding: 0.35rem 0.6rem;
    color: #e0e0e0;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.85rem;
    transition: all 0.2s;
  }

  .refresh-button:hover:not(:disabled) {
    background: #404040;
    border-color: #555;
  }

  .refresh-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .icon {
    font-size: 1rem;
    line-height: 1;
  }

  .loading-state,
  .error-state,
  .empty-state {
    padding: 1.5rem;
    text-align: center;
    color: #b0b0b0;
  }

  .error-state {
    background: rgba(220, 38, 38, 0.1);
    border-radius: 6px;
    border: 1px solid rgba(220, 38, 38, 0.3);
  }

  .error-message {
    color: #f87171;
    margin-bottom: 0.75rem;
    font-size: 0.9rem;
  }

  .retry-button {
    background: #dc2626;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    transition: background 0.2s;
  }

  .retry-button:hover {
    background: #b91c1c;
  }

  .empty-icon {
    font-size: 3rem;
    margin-bottom: 0.5rem;
  }

  .empty-title {
    font-size: 1rem;
    font-weight: 600;
    color: #d0d0d0;
    margin-bottom: 0.25rem;
  }

  .empty-description {
    font-size: 0.85rem;
    color: #909090;
  }

  .port-selector-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .port-select {
    background: #1e1e1e;
    border: 1px solid #444;
    border-radius: 6px;
    padding: 0.65rem 0.85rem;
    color: #e0e0e0;
    font-size: 0.95rem;
    cursor: pointer;
    transition: all 0.2s;
    width: 100%;
  }

  .port-select:hover:not(:disabled) {
    border-color: #666;
  }

  .port-select:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
  }

  .port-select:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .port-metadata {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .port-badge,
  .platform-badge {
    padding: 0.25rem 0.6rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 600;
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
  }

  .port-badge.virtual {
    background: rgba(59, 130, 246, 0.2);
    color: #60a5fa;
    border: 1px solid rgba(59, 130, 246, 0.3);
  }

  .port-badge.physical {
    background: rgba(34, 197, 94, 0.2);
    color: #4ade80;
    border: 1px solid rgba(34, 197, 94, 0.3);
  }

  .platform-badge {
    background: rgba(168, 85, 247, 0.2);
    color: #c084fc;
    border: 1px solid rgba(168, 85, 247, 0.3);
  }

  .test-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding-top: 0.5rem;
    border-top: 1px solid #333;
  }

  .test-button {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 0.65rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    font-size: 0.9rem;
    font-weight: 600;
    transition: all 0.2s;
  }

  .test-button:hover:not(:disabled) {
    background: #2563eb;
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(59, 130, 246, 0.3);
  }

  .test-button:active:not(:disabled) {
    transform: translateY(0);
  }

  .test-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .test-success,
  .test-error {
    padding: 0.65rem 0.85rem;
    border-radius: 6px;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .test-success {
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
    border: 1px solid rgba(34, 197, 94, 0.3);
  }

  .test-error {
    background: rgba(220, 38, 38, 0.15);
    color: #f87171;
    border: 1px solid rgba(220, 38, 38, 0.3);
  }

  .label {
    line-height: 1;
  }
</style>
