<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  interface MidiEventInfo {
    timestamp: number;
    event_type: string;
    channel?: number;
    note?: number;
    velocity?: number;
    cc_number?: number;
    cc_value?: number;
    pitch_bend?: number;
    aftertouch?: number;
    raw_bytes: number[];
    description: string;
  }

  export let maxEvents: number = 100;
  export let autoScroll: boolean = true;

  let events: MidiEventInfo[] = [];
  let isMonitoring: boolean = false;
  let eventListener: UnlistenFn | null = null;
  let consoleElement: HTMLDivElement;
  let filterType: string = 'all';
  let filterChannel: string = 'all';
  let paused: boolean = false;

  onMount(async () => {
    // Listen for MIDI events from backend
    eventListener = await listen<MidiEventInfo>('midi-event', (event) => {
      if (!paused) {
        addEvent(event.payload);
      }
    });
  });

  onDestroy(async () => {
    if (isMonitoring) {
      await stopMonitoring();
    }
    if (eventListener) {
      eventListener();
    }
  });

  async function startMonitoring() {
    try {
      await invoke('start_event_monitoring');
      isMonitoring = true;
    } catch (e) {
      console.error('Failed to start event monitoring:', e);
    }
  }

  async function stopMonitoring() {
    try {
      await invoke('stop_event_monitoring');
      isMonitoring = false;
    } catch (e) {
      console.error('Failed to stop event monitoring:', e);
    }
  }

  function addEvent(event: MidiEventInfo) {
    events = [event, ...events].slice(0, maxEvents);

    if (autoScroll && consoleElement) {
      // Scroll to top since events are prepended
      requestAnimationFrame(() => {
        consoleElement.scrollTop = 0;
      });
    }
  }

  function clearEvents() {
    events = [];
  }

  function getEventColor(eventType: string): string {
    switch (eventType) {
      case 'NoteOn': return '#10b981';
      case 'NoteOff': return '#6b7280';
      case 'ControlChange': return '#3b82f6';
      case 'PitchBend': return '#8b5cf6';
      case 'PolyAftertouch':
      case 'ChannelAftertouch': return '#f59e0b';
      case 'ProgramChange': return '#ec4899';
      default: return '#6b7280';
    }
  }

  function formatTimestamp(timestamp: number): string {
    const date = new Date(timestamp);
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    const seconds = date.getSeconds().toString().padStart(2, '0');
    const ms = date.getMilliseconds().toString().padStart(3, '0');
    return `${hours}:${minutes}:${seconds}.${ms}`;
  }

  function formatBytes(bytes: number[]): string {
    return bytes.map(b => b.toString(16).toUpperCase().padStart(2, '0')).join(' ');
  }

  function formatNoteName(note: number): string {
    const noteNames = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    const octave = Math.floor(note / 12) - 1;
    const noteIndex = note % 12;
    return `${noteNames[noteIndex]}${octave}`;
  }

  $: filteredEvents = events.filter(event => {
    if (filterType !== 'all' && event.event_type !== filterType) {
      return false;
    }
    if (filterChannel !== 'all' && event.channel !== parseInt(filterChannel)) {
      return false;
    }
    return true;
  });
</script>

<div class="live-console">
  <div class="controls">
    <div class="control-group">
      {#if isMonitoring}
        <button class="btn btn-danger" on:click={stopMonitoring}>
          ⏹ Stop
        </button>
      {:else}
        <button class="btn btn-primary" on:click={startMonitoring}>
          ▶ Start Monitoring
        </button>
      {/if}

      <button
        class="btn btn-secondary"
        on:click={() => paused = !paused}
        disabled={!isMonitoring}
      >
        {paused ? '▶' : '⏸'} {paused ? 'Resume' : 'Pause'}
      </button>

      <button class="btn btn-secondary" on:click={clearEvents}>
        🗑️ Clear
      </button>
    </div>

    <div class="control-group">
      <label class="filter-label">
        Type:
        <select bind:value={filterType}>
          <option value="all">All</option>
          <option value="NoteOn">Note On</option>
          <option value="NoteOff">Note Off</option>
          <option value="ControlChange">CC</option>
          <option value="PitchBend">Pitch Bend</option>
          <option value="PolyAftertouch">Poly Aftertouch</option>
          <option value="ChannelAftertouch">Channel Aftertouch</option>
          <option value="ProgramChange">Program Change</option>
        </select>
      </label>

      <label class="filter-label">
        Channel:
        <select bind:value={filterChannel}>
          <option value="all">All</option>
          {#each Array(16).fill(0).map((_, i) => i) as ch}
            <option value={ch}>{ch + 1}</option>
          {/each}
        </select>
      </label>

      <label class="checkbox-label">
        <input type="checkbox" bind:checked={autoScroll} />
        Auto-scroll
      </label>
    </div>
  </div>

  <div class="event-list" bind:this={consoleElement}>
    {#if filteredEvents.length === 0}
      <div class="empty-state">
        {#if isMonitoring}
          <p>Waiting for MIDI events...</p>
          <p class="hint">Play your MIDI controller to see events here</p>
        {:else}
          <p>Click "Start Monitoring" to begin capturing MIDI events</p>
        {/if}
      </div>
    {:else}
      {#each filteredEvents as event (event.timestamp)}
        <div class="event-row">
          <div class="event-time">{formatTimestamp(event.timestamp)}</div>
          <div
            class="event-type"
            style="color: {getEventColor(event.event_type)}"
          >
            {event.event_type}
          </div>
          <div class="event-details">
            {#if event.channel !== undefined}
              <span class="detail-item">Ch {event.channel + 1}</span>
            {/if}
            {#if event.note !== undefined}
              <span class="detail-item">Note {event.note} ({formatNoteName(event.note)})</span>
            {/if}
            {#if event.velocity !== undefined}
              <span class="detail-item">Vel {event.velocity}</span>
            {/if}
            {#if event.cc_number !== undefined}
              <span class="detail-item">CC#{event.cc_number}</span>
            {/if}
            {#if event.cc_value !== undefined && event.cc_number !== undefined}
              <span class="detail-item">Val {event.cc_value}</span>
            {/if}
            {#if event.pitch_bend !== undefined}
              <span class="detail-item">Bend {event.pitch_bend}</span>
            {/if}
            {#if event.aftertouch !== undefined}
              <span class="detail-item">Pressure {event.aftertouch}</span>
            {/if}
          </div>
          <div class="event-raw">
            {formatBytes(event.raw_bytes)}
          </div>
        </div>
      {/each}
    {/if}
  </div>

  <div class="footer">
    <span>{filteredEvents.length} / {events.length} events</span>
    {#if paused}
      <span class="status-paused">PAUSED</span>
    {:else if isMonitoring}
      <span class="status-active">MONITORING</span>
    {:else}
      <span class="status-stopped">STOPPED</span>
    {/if}
  </div>
</div>

<style>
  .live-console {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-primary, #fff);
    border-radius: 0.5rem;
    overflow: hidden;
  }

  .controls {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    padding: 1rem;
    border-bottom: 1px solid var(--border, #d1d5db);
    background: var(--bg-secondary, #f9fafb);
  }

  .control-group {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary {
    background: var(--primary, #3b82f6);
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--primary-dark, #2563eb);
  }

  .btn-danger {
    background: var(--error, #dc2626);
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background: #b91c1c;
  }

  .btn-secondary {
    background: var(--bg-secondary, #e5e7eb);
    color: var(--text-primary, #333);
  }

  .btn-secondary:hover:not(:disabled) {
    background: var(--border, #d1d5db);
  }

  .filter-label,
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-secondary, #666);
  }

  select {
    padding: 0.375rem 0.5rem;
    border: 1px solid var(--border, #d1d5db);
    border-radius: 0.25rem;
    font-size: 0.875rem;
  }

  .event-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
    font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
    font-size: 0.8125rem;
  }

  .event-row {
    display: grid;
    grid-template-columns: 120px 140px 1fr 120px;
    gap: 1rem;
    padding: 0.5rem;
    border-bottom: 1px solid var(--border, #e5e7eb);
    transition: background 0.1s;
  }

  .event-row:hover {
    background: var(--bg-hover, #f3f4f6);
  }

  .event-time {
    color: var(--text-secondary, #666);
    font-weight: 500;
  }

  .event-type {
    font-weight: 600;
  }

  .event-details {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .detail-item {
    padding: 0.125rem 0.375rem;
    background: var(--bg-secondary, #f3f4f6);
    border-radius: 0.25rem;
    font-size: 0.75rem;
  }

  .event-raw {
    color: var(--text-secondary, #666);
    font-size: 0.75rem;
    text-align: right;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-secondary, #666);
    text-align: center;
  }

  .empty-state p {
    margin: 0.25rem 0;
  }

  .hint {
    font-size: 0.875rem;
    color: var(--text-tertiary, #9ca3af);
  }

  .footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    border-top: 1px solid var(--border, #d1d5db);
    background: var(--bg-secondary, #f9fafb);
    font-size: 0.875rem;
  }

  .status-active {
    color: var(--success, #10b981);
    font-weight: 600;
  }

  .status-paused {
    color: var(--warning, #f59e0b);
    font-weight: 600;
  }

  .status-stopped {
    color: var(--text-secondary, #666);
    font-weight: 600;
  }
</style>
