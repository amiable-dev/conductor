<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { onMount, onDestroy } from 'svelte';
  import { createEventDispatcher } from 'svelte';

  /**
   * Props
   */
  export let maxEvents = 50; // Maximum events to display
  export let showTimestamps = true; // Show event timestamps
  export let autoScroll = true; // Auto-scroll to latest events
  export let pauseOnHover = false; // Pause updates on hover

  const dispatch = createEventDispatcher();

  /**
   * Local state
   */
  let events = []; // Array of event objects
  let isConnected = false;
  let isPaused = false;
  let isHovered = false;
  let eventCount = 0;
  let pollInterval = null;
  let containerElement;

  /**
   * Event type color mapping
   */
  const eventTypeColors = {
    NoteOn: '#4ade80',      // Green
    NoteOff: '#f87171',     // Red
    CC: '#60a5fa',          // Blue
    PitchBend: '#a78bfa',   // Purple
    Aftertouch: '#fb923c',  // Orange
    ProgramChange: '#fbbf24', // Yellow
    Unknown: '#6b7280',     // Gray
  };

  /**
   * Format timestamp
   */
  function formatTimestamp(date) {
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    const seconds = date.getSeconds().toString().padStart(2, '0');
    const ms = date.getMilliseconds().toString().padStart(3, '0');
    return `${hours}:${minutes}:${seconds}.${ms}`;
  }

  /**
   * Format MIDI event for display
   */
  function formatEvent(event) {
    switch (event.type) {
      case 'NoteOn':
        return `Note On: ${event.note} (vel ${event.velocity})`;
      case 'NoteOff':
        return `Note Off: ${event.note}`;
      case 'CC':
        return `CC ${event.cc}: ${event.value}`;
      case 'PitchBend':
        return `Pitch Bend: ${event.value}`;
      case 'Aftertouch':
        return `Aftertouch: ${event.pressure}`;
      case 'ProgramChange':
        return `Program: ${event.program}`;
      default:
        return `Unknown: ${JSON.stringify(event)}`;
    }
  }

  /**
   * Get note name from MIDI number
   */
  function getNoteName(noteNumber) {
    const notes = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    const octave = Math.floor(noteNumber / 12) - 1;
    const noteName = notes[noteNumber % 12];
    return `${noteName}${octave}`;
  }

  /**
   * Add event to list
   */
  function addEvent(event) {
    if (isPaused || (pauseOnHover && isHovered)) {
      return;
    }

    const eventWithTimestamp = {
      ...event,
      timestamp: new Date(),
      id: eventCount++,
    };

    events = [eventWithTimestamp, ...events].slice(0, maxEvents);

    // Auto-scroll to top if enabled
    if (autoScroll && containerElement) {
      setTimeout(() => {
        containerElement.scrollTop = 0;
      }, 10);
    }

    dispatch('eventReceived', eventWithTimestamp);
  }

  /**
   * Poll for new events from daemon
   */
  async function pollEvents() {
    try {
      // TODO: Replace with actual API call to daemon
      // const response = await api.daemon.getRecentEvents();
      // response.events.forEach(addEvent);

      // For now, simulate with placeholder
      // This will be replaced when backend integration is complete
      isConnected = true;
    } catch (error) {
      console.error('Failed to poll events:', error);
      isConnected = false;
    }
  }

  /**
   * Start polling
   */
  function startPolling() {
    if (pollInterval) return;
    pollInterval = setInterval(pollEvents, 100); // Poll every 100ms
    isConnected = true;
  }

  /**
   * Stop polling
   */
  function stopPolling() {
    if (pollInterval) {
      clearInterval(pollInterval);
      pollInterval = null;
    }
    isConnected = false;
  }

  /**
   * Toggle pause
   */
  function togglePause() {
    isPaused = !isPaused;
  }

  /**
   * Clear all events
   */
  function clearEvents() {
    events = [];
    eventCount = 0;
  }

  /**
   * Export events to JSON
   */
  function exportEvents() {
    const data = JSON.stringify(events, null, 2);
    const blob = new Blob([data], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `midi-events-${new Date().toISOString()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }

  /**
   * Handle hover state
   */
  function handleMouseEnter() {
    isHovered = true;
  }

  function handleMouseLeave() {
    isHovered = false;
  }

  /**
   * Lifecycle
   */
  onMount(() => {
    startPolling();
  });

  onDestroy(() => {
    stopPolling();
  });
</script>

<div class="live-preview">
  <div class="header">
    <div class="title-section">
      <h3>Live MIDI Events</h3>
      <div class="connection-status">
        <span class="status-dot" class:connected={isConnected}></span>
        <span class="status-text">
          {isConnected ? 'Connected' : 'Disconnected'}
        </span>
      </div>
    </div>

    <div class="controls">
      <button
        class="btn btn-icon"
        on:click={togglePause}
        title={isPaused ? 'Resume' : 'Pause'}
      >
        {isPaused ? '▶️' : '⏸️'}
      </button>
      <button
        class="btn btn-icon"
        on:click={clearEvents}
        title="Clear events"
        disabled={events.length === 0}
      >
        🗑️
      </button>
      <button
        class="btn btn-icon"
        on:click={exportEvents}
        title="Export to JSON"
        disabled={events.length === 0}
      >
        💾
      </button>
    </div>
  </div>

  <div class="stats">
    <div class="stat">
      <span class="stat-label">Total:</span>
      <span class="stat-value">{eventCount}</span>
    </div>
    <div class="stat">
      <span class="stat-label">Visible:</span>
      <span class="stat-value">{events.length}</span>
    </div>
    {#if isPaused || (pauseOnHover && isHovered)}
      <div class="stat warning">
        <span class="stat-label">⏸ Paused</span>
      </div>
    {/if}
  </div>

  <div
    class="events-container"
    bind:this={containerElement}
    on:mouseenter={pauseOnHover ? handleMouseEnter : null}
    on:mouseleave={pauseOnHover ? handleMouseLeave : null}
  >
    {#if events.length === 0}
      <div class="empty-state">
        <div class="empty-icon">🎹</div>
        <p>No MIDI events received</p>
        <small>Play some notes or turn some knobs to see events here</small>
      </div>
    {:else}
      {#each events as event (event.id)}
        <div class="event-item" style="--event-color: {eventTypeColors[event.type] || eventTypeColors.Unknown}">
          <div class="event-indicator"></div>
          <div class="event-content">
            {#if showTimestamps}
              <span class="event-timestamp">{formatTimestamp(event.timestamp)}</span>
            {/if}
            <span class="event-type" style="color: {eventTypeColors[event.type] || eventTypeColors.Unknown}">
              {event.type}
            </span>
            <span class="event-details">{formatEvent(event)}</span>
            {#if event.type === 'NoteOn' || event.type === 'NoteOff'}
              <span class="event-note-name">{getNoteName(event.note)}</span>
            {/if}
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .live-preview {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-secondary, #2a2a2a);
    border-radius: 8px;
    overflow: hidden;
    border: 1px solid var(--border-color, #444);
  }

  .header {
    padding: 1rem 1.5rem;
    border-bottom: 1px solid var(--border-color, #444);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .title-section {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .title-section h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
  }

  .connection-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.25rem 0.75rem;
    background: var(--bg-tertiary, #1a1a1a);
    border-radius: 12px;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--error-color, #ef4444);
    transition: background 0.3s;
  }

  .status-dot.connected {
    background: var(--success-color, #4ade80);
    animation: pulse-dot 2s ease-in-out infinite;
  }

  @keyframes pulse-dot {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.6;
    }
  }

  .status-text {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-secondary, #999);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .controls {
    display: flex;
    gap: 0.5rem;
  }

  .btn-icon {
    background: var(--bg-tertiary, #1a1a1a);
    border: 1px solid var(--border-color, #444);
    padding: 0.5rem;
    font-size: 1rem;
    cursor: pointer;
    border-radius: 6px;
    transition: all 0.2s;
  }

  .btn-icon:hover:not(:disabled) {
    background: var(--bg-hover, #333);
    border-color: var(--accent-color, #3b82f6);
  }

  .btn-icon:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .stats {
    padding: 0.75rem 1.5rem;
    background: var(--bg-tertiary, #1a1a1a);
    border-bottom: 1px solid var(--border-color, #444);
    display: flex;
    gap: 1.5rem;
    font-size: 0.875rem;
  }

  .stat {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .stat.warning {
    color: var(--error-color, #ef4444);
    font-weight: 600;
  }

  .stat-label {
    color: var(--text-secondary, #999);
    font-weight: 600;
  }

  .stat-value {
    color: var(--accent-color, #3b82f6);
    font-weight: 600;
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
  }

  .events-container {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .empty-state {
    text-align: center;
    padding: 4rem 1rem;
  }

  .empty-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
    opacity: 0.5;
  }

  .empty-state p {
    color: var(--text-primary, #fff);
    margin-bottom: 0.5rem;
    font-size: 1rem;
  }

  .empty-state small {
    color: var(--text-secondary, #999);
    font-size: 0.875rem;
  }

  .event-item {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 0.75rem;
    margin-bottom: 0.25rem;
    background: var(--bg-tertiary, #1a1a1a);
    border-radius: 6px;
    border-left: 3px solid var(--event-color, #666);
    transition: all 0.2s;
    animation: slideIn 0.2s ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .event-item:hover {
    background: var(--bg-hover, #333);
  }

  .event-indicator {
    width: 6px;
    height: 6px;
    margin-top: 0.5rem;
    border-radius: 50%;
    background: var(--event-color, #666);
    flex-shrink: 0;
  }

  .event-content {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
    font-size: 0.875rem;
  }

  .event-timestamp {
    color: var(--text-secondary, #999);
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 0.75rem;
    min-width: 90px;
  }

  .event-type {
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    font-size: 0.75rem;
    min-width: 80px;
  }

  .event-details {
    color: var(--text-primary, #fff);
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    flex: 1;
  }

  .event-note-name {
    padding: 0.25rem 0.5rem;
    background: var(--bg-primary, #0a0a0a);
    border: 1px solid var(--event-color, #666);
    border-radius: 4px;
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--event-color, #666);
  }
</style>
