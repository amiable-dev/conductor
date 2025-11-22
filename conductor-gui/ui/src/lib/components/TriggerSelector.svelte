<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { createEventDispatcher } from 'svelte';

  /**
   * Props
   */
  export let trigger = null; // Current trigger object
  export let readonly = false;

  const dispatch = createEventDispatcher();

  /**
   * Trigger type definitions
   */
  const triggerTypes = [
    { value: 'Note', label: 'Note', description: 'Basic note on/off trigger' },
    { value: 'VelocityRange', label: 'Velocity Range', description: 'Velocity-sensitive trigger' },
    { value: 'LongPress', label: 'Long Press', description: 'Hold note for duration' },
    { value: 'DoubleTap', label: 'Double Tap', description: 'Quick double press' },
    { value: 'NoteChord', label: 'Chord', description: 'Multiple notes simultaneously' },
    { value: 'EncoderTurn', label: 'Encoder', description: 'Rotary encoder/knob' },
    { value: 'CC', label: 'Control Change', description: 'MIDI CC message' },
    { value: 'Aftertouch', label: 'Aftertouch', description: 'Pressure sensitivity' },
    { value: 'PitchBend', label: 'Pitch Bend', description: 'Pitch bend/touch strip' },
  ];

  /**
   * Local state
   */
  let selectedType = trigger?.type || 'Note';
  let config = trigger || createDefaultConfig('Note');
  let chordNotes = trigger?.notes?.join(', ') || '';

  /**
   * Create default config for a trigger type
   */
  function createDefaultConfig(type) {
    const defaults = {
      Note: { type: 'Note', note: 60, velocity_min: null },
      VelocityRange: { type: 'VelocityRange', note: 60, soft_max: 40, medium_max: 80 },
      LongPress: { type: 'LongPress', note: 60, duration_ms: 2000 },
      DoubleTap: { type: 'DoubleTap', note: 60, timeout_ms: 300 },
      NoteChord: { type: 'NoteChord', notes: [60, 64, 67], timeout_ms: 100 },
      EncoderTurn: { type: 'EncoderTurn', cc: 1, direction: null },
      CC: { type: 'CC', cc: 1, value_min: null },
      Aftertouch: { type: 'Aftertouch', pressure_min: null },
      PitchBend: { type: 'PitchBend', value_min: null, value_max: null },
    };
    return defaults[type] || defaults.Note;
  }

  /**
   * Handle type change
   */
  function handleTypeChange() {
    config = createDefaultConfig(selectedType);
    if (selectedType === 'NoteChord') {
      chordNotes = config.notes.join(', ');
    }
    emitChange();
  }

  /**
   * Handle chord notes input
   */
  function handleChordNotesChange() {
    const notes = chordNotes
      .split(',')
      .map(s => parseInt(s.trim()))
      .filter(n => !isNaN(n) && n >= 0 && n <= 127);
    config.notes = notes.length > 0 ? notes : [60];
    emitChange();
  }

  /**
   * Emit change event
   */
  function emitChange() {
    dispatch('change', config);
  }

  /**
   * Open MIDI Learn
   */
  function openMidiLearn() {
    dispatch('midiLearn');
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
</script>

<div class="trigger-selector">
  <div class="section-header">
    <h3>Trigger Configuration</h3>
    {#if !readonly}
      <button class="btn btn-learn" on:click={openMidiLearn}>
        🎹 MIDI Learn
      </button>
    {/if}
  </div>

  <div class="form-group">
    <label for="trigger-type">Trigger Type</label>
    <select
      id="trigger-type"
      bind:value={selectedType}
      on:change={handleTypeChange}
      disabled={readonly}
    >
      {#each triggerTypes as type}
        <option value={type.value}>
          {type.label} - {type.description}
        </option>
      {/each}
    </select>
  </div>

  <!-- Note Trigger -->
  {#if selectedType === 'Note'}
    <div class="config-fields">
      <div class="form-group">
        <label for="note">MIDI Note (0-127)</label>
        <div class="input-with-preview">
          <input
            id="note"
            type="number"
            min="0"
            max="127"
            bind:value={config.note}
            on:input={emitChange}
            disabled={readonly}
          />
          <span class="note-preview">{getNoteName(config.note)}</span>
        </div>
      </div>
      <div class="form-group">
        <label for="velocity-min">Minimum Velocity (optional)</label>
        <input
          id="velocity-min"
          type="number"
          min="1"
          max="127"
          bind:value={config.velocity_min}
          on:input={emitChange}
          placeholder="Any velocity"
          disabled={readonly}
        />
      </div>
    </div>
  {/if}

  <!-- Velocity Range Trigger -->
  {#if selectedType === 'VelocityRange'}
    <div class="config-fields">
      <div class="form-group">
        <label for="vr-note">MIDI Note (0-127)</label>
        <div class="input-with-preview">
          <input
            id="vr-note"
            type="number"
            min="0"
            max="127"
            bind:value={config.note}
            on:input={emitChange}
            disabled={readonly}
          />
          <span class="note-preview">{getNoteName(config.note)}</span>
        </div>
      </div>
      <div class="form-group">
        <label for="soft-max">Soft Max Velocity (0-127)</label>
        <input
          id="soft-max"
          type="number"
          min="0"
          max="127"
          bind:value={config.soft_max}
          on:input={emitChange}
          disabled={readonly}
        />
        <small>Soft: 0-{config.soft_max}</small>
      </div>
      <div class="form-group">
        <label for="medium-max">Medium Max Velocity (0-127)</label>
        <input
          id="medium-max"
          type="number"
          min="0"
          max="127"
          bind:value={config.medium_max}
          on:input={emitChange}
          disabled={readonly}
        />
        <small>Medium: {config.soft_max + 1}-{config.medium_max}, Hard: {config.medium_max + 1}-127</small>
      </div>
    </div>
  {/if}

  <!-- Long Press Trigger -->
  {#if selectedType === 'LongPress'}
    <div class="config-fields">
      <div class="form-group">
        <label for="lp-note">MIDI Note (0-127)</label>
        <div class="input-with-preview">
          <input
            id="lp-note"
            type="number"
            min="0"
            max="127"
            bind:value={config.note}
            on:input={emitChange}
            disabled={readonly}
          />
          <span class="note-preview">{getNoteName(config.note)}</span>
        </div>
      </div>
      <div class="form-group">
        <label for="duration-ms">Hold Duration (ms)</label>
        <input
          id="duration-ms"
          type="number"
          min="100"
          max="10000"
          step="100"
          bind:value={config.duration_ms}
          on:input={emitChange}
          disabled={readonly}
        />
        <small>{(config.duration_ms / 1000).toFixed(1)}s</small>
      </div>
    </div>
  {/if}

  <!-- Double Tap Trigger -->
  {#if selectedType === 'DoubleTap'}
    <div class="config-fields">
      <div class="form-group">
        <label for="dt-note">MIDI Note (0-127)</label>
        <div class="input-with-preview">
          <input
            id="dt-note"
            type="number"
            min="0"
            max="127"
            bind:value={config.note}
            on:input={emitChange}
            disabled={readonly}
          />
          <span class="note-preview">{getNoteName(config.note)}</span>
        </div>
      </div>
      <div class="form-group">
        <label for="timeout-ms">Time Window (ms)</label>
        <input
          id="timeout-ms"
          type="number"
          min="50"
          max="1000"
          step="50"
          bind:value={config.timeout_ms}
          on:input={emitChange}
          disabled={readonly}
        />
        <small>Double tap within {config.timeout_ms}ms</small>
      </div>
    </div>
  {/if}

  <!-- Chord Trigger -->
  {#if selectedType === 'NoteChord'}
    <div class="config-fields">
      <div class="form-group">
        <label for="chord-notes">MIDI Notes (comma-separated)</label>
        <input
          id="chord-notes"
          type="text"
          bind:value={chordNotes}
          on:input={handleChordNotesChange}
          placeholder="60, 64, 67"
          disabled={readonly}
        />
        <small>
          {#if config.notes?.length > 0}
            Notes: {config.notes.map(n => getNoteName(n)).join(', ')}
          {/if}
        </small>
      </div>
      <div class="form-group">
        <label for="chord-timeout">Time Window (ms)</label>
        <input
          id="chord-timeout"
          type="number"
          min="10"
          max="500"
          step="10"
          bind:value={config.timeout_ms}
          on:input={emitChange}
          disabled={readonly}
        />
        <small>All notes pressed within {config.timeout_ms}ms</small>
      </div>
    </div>
  {/if}

  <!-- Encoder Turn Trigger -->
  {#if selectedType === 'EncoderTurn'}
    <div class="config-fields">
      <div class="form-group">
        <label for="encoder-cc">CC Number (0-127)</label>
        <input
          id="encoder-cc"
          type="number"
          min="0"
          max="127"
          bind:value={config.cc}
          on:input={emitChange}
          disabled={readonly}
        />
      </div>
      <div class="form-group">
        <label for="direction">Direction (optional)</label>
        <select
          id="direction"
          bind:value={config.direction}
          on:change={emitChange}
          disabled={readonly}
        >
          <option value={null}>Any direction</option>
          <option value="Clockwise">Clockwise</option>
          <option value="CounterClockwise">Counter-clockwise</option>
        </select>
      </div>
    </div>
  {/if}

  <!-- CC Trigger -->
  {#if selectedType === 'CC'}
    <div class="config-fields">
      <div class="form-group">
        <label for="cc-number">CC Number (0-127)</label>
        <input
          id="cc-number"
          type="number"
          min="0"
          max="127"
          bind:value={config.cc}
          on:input={emitChange}
          disabled={readonly}
        />
      </div>
      <div class="form-group">
        <label for="cc-value-min">Minimum Value (optional)</label>
        <input
          id="cc-value-min"
          type="number"
          min="0"
          max="127"
          bind:value={config.value_min}
          on:input={emitChange}
          placeholder="Any value"
          disabled={readonly}
        />
      </div>
    </div>
  {/if}

  <!-- Aftertouch Trigger -->
  {#if selectedType === 'Aftertouch'}
    <div class="config-fields">
      <div class="form-group">
        <label for="pressure-min">Minimum Pressure (optional)</label>
        <input
          id="pressure-min"
          type="number"
          min="0"
          max="127"
          bind:value={config.pressure_min}
          on:input={emitChange}
          placeholder="Any pressure"
          disabled={readonly}
        />
      </div>
    </div>
  {/if}

  <!-- Pitch Bend Trigger -->
  {#if selectedType === 'PitchBend'}
    <div class="config-fields">
      <div class="form-group">
        <label for="pb-min">Minimum Value (0-16383, optional)</label>
        <input
          id="pb-min"
          type="number"
          min="0"
          max="16383"
          bind:value={config.value_min}
          on:input={emitChange}
          placeholder="0"
          disabled={readonly}
        />
      </div>
      <div class="form-group">
        <label for="pb-max">Maximum Value (0-16383, optional)</label>
        <input
          id="pb-max"
          type="number"
          min="0"
          max="16383"
          bind:value={config.value_max}
          on:input={emitChange}
          placeholder="16383"
          disabled={readonly}
        />
      </div>
    </div>
  {/if}
</div>

<style>
  .trigger-selector {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 0.75rem;
    border-bottom: 2px solid var(--border-color, #444);
  }

  .section-header h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-primary, #fff);
  }

  .btn-learn {
    padding: 0.5rem 1rem;
    background: var(--accent-color, #3b82f6);
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-learn:hover {
    background: var(--accent-hover, #2563eb);
  }

  .btn-learn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
  }

  .form-group input,
  .form-group select {
    padding: 0.75rem;
    background: var(--bg-tertiary, #1a1a1a);
    border: 1px solid var(--border-color, #444);
    border-radius: 6px;
    color: var(--text-primary, #fff);
    font-size: 0.875rem;
  }

  .form-group input:focus,
  .form-group select:focus {
    outline: none;
    border-color: var(--accent-color, #3b82f6);
  }

  .form-group input:disabled,
  .form-group select:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .form-group small {
    font-size: 0.75rem;
    color: var(--text-secondary, #999);
  }

  .input-with-preview {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .input-with-preview input {
    flex: 1;
  }

  .note-preview {
    padding: 0.5rem 0.75rem;
    background: var(--bg-primary, #0a0a0a);
    border: 1px solid var(--border-color, #444);
    border-radius: 6px;
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--accent-color, #3b82f6);
    min-width: 60px;
    text-align: center;
  }

  .config-fields {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
    background: var(--bg-tertiary, #1a1a1a);
    border-radius: 8px;
    border: 1px solid var(--border-color, #444);
  }
</style>
