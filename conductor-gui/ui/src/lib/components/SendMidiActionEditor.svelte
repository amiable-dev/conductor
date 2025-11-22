<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { createEventDispatcher, onMount } from 'svelte';
  import MidiOutputSelector from './MidiOutputSelector.svelte';
  import VelocityMappingSelector from './VelocityMappingSelector.svelte';
  import api from '../api.js';

  /**
   * Props
   */
  export let action = null; // Current SendMIDI action config
  export let readonly = false;

  const dispatch = createEventDispatcher();

  /**
   * Message type definitions
   */
  const messageTypes = [
    { value: 'note_on', label: 'Note On', description: 'Trigger a MIDI note' },
    { value: 'note_off', label: 'Note Off', description: 'Release a MIDI note' },
    { value: 'cc', label: 'Control Change', description: 'Send CC message' },
    { value: 'program_change', label: 'Program Change', description: 'Change instrument/preset' },
    { value: 'pitch_bend', label: 'Pitch Bend', description: 'Pitch wheel control' },
    { value: 'aftertouch', label: 'Aftertouch', description: 'Channel pressure' },
  ];

  /**
   * Note names for piano keyboard
   */
  const noteNames = [
    'C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'
  ];

  /**
   * Common CC numbers
   */
  const commonCCs = [
    { value: 1, label: 'Modulation Wheel' },
    { value: 7, label: 'Volume' },
    { value: 10, label: 'Pan' },
    { value: 11, label: 'Expression' },
    { value: 64, label: 'Sustain Pedal' },
    { value: 71, label: 'Filter Resonance' },
    { value: 74, label: 'Filter Cutoff' },
  ];

  /**
   * Local state
   */
  let config = action || createDefaultConfig('note_on');
  let validationResult = null;
  let validating = false;

  /**
   * Create default config for a message type
   */
  function createDefaultConfig(messageType) {
    return {
      type: 'SendMidi',
      port: '',
      message_type: messageType,
      channel: 0,
      note: 60,          // Middle C
      velocity_mapping: { Fixed: { velocity: 100 } },  // v2.2: Variable velocity
      cc_number: 1,      // Modulation wheel
      cc_value: 64,      // Center
      program: 0,
      pitch_bend: 0,     // Center
      aftertouch: 64,    // Center
    };
  }

  /**
   * Handle message type change
   */
  function handleMessageTypeChange() {
    // Preserve common fields, reset type-specific fields
    const newConfig = {
      ...config,
      message_type: config.message_type,
    };
    config = newConfig;
    validateAndEmit();
  }

  /**
   * Handle port change from MidiOutputSelector
   */
  function handlePortChange(event) {
    config.port = event.detail.portName;
    validateAndEmit();
  }

  /**
   * Handle field change
   */
  function handleFieldChange() {
    validateAndEmit();
  }

  /**
   * Handle velocity mapping change from VelocityMappingSelector
   */
  function handleVelocityMappingChange(event) {
    config.velocity_mapping = event.detail.velocityMapping;
    validateAndEmit();
  }

  /**
   * Validate and emit changes
   */
  async function validateAndEmit() {
    validating = true;
    try {
      validationResult = await api.midiOutput.validateAction(config);
      dispatch('change', { config, validation: validationResult });
    } catch (error) {
      console.error('Validation failed:', error);
      validationResult = {
        valid: false,
        errors: [error.message || String(error)],
        warnings: [],
      };
    } finally {
      validating = false;
    }
  }

  /**
   * Get note name from MIDI note number
   */
  function getNoteName(noteNumber) {
    const octave = Math.floor(noteNumber / 12) - 1;
    const noteName = noteNames[noteNumber % 12];
    return `${noteName}${octave}`;
  }

  /**
   * Get display value for pitch bend
   */
  function getPitchBendDisplay(value) {
    if (value === 0) return '0 (Center)';
    return value > 0 ? `+${value}` : `${value}`;
  }

  /**
   * Initialize validation on mount
   */
  onMount(() => {
    if (config.port) {
      validateAndEmit();
    }
  });

  /**
   * Reactive statement to re-validate when config changes
   */
  $: if (config && config.port) {
    // Re-validate when config changes
    validateAndEmit();
  }
</script>

<div class="send-midi-editor">
  <div class="editor-header">
    <h3 class="editor-title">SendMIDI Action Configuration</h3>
    {#if validationResult}
      <div class="validation-status" class:valid={validationResult.valid} class:invalid={!validationResult.valid}>
        {validationResult.valid ? '✅ Valid' : '❌ Invalid'}
      </div>
    {/if}
  </div>

  <!-- MIDI Output Port Selector -->
  <div class="editor-section">
    <MidiOutputSelector
      bind:selectedPort={config.port}
      {readonly}
      on:change={handlePortChange}
    />
  </div>

  <!-- Message Type Selector -->
  <div class="editor-section">
    <label for="message-type" class="field-label">Message Type</label>
    <select
      id="message-type"
      class="field-select"
      bind:value={config.message_type}
      on:change={handleMessageTypeChange}
      disabled={readonly}
    >
      {#each messageTypes as msgType}
        <option value={msgType.value}>
          {msgType.label} - {msgType.description}
        </option>
      {/each}
    </select>
  </div>

  <!-- MIDI Channel -->
  <div class="editor-section">
    <label for="channel" class="field-label">
      MIDI Channel
      <span class="field-hint">(1-16)</span>
    </label>
    <div class="channel-selector">
      <input
        id="channel"
        type="range"
        min="0"
        max="15"
        bind:value={config.channel}
        on:input={handleFieldChange}
        disabled={readonly}
        class="channel-slider"
      />
      <span class="channel-display">{config.channel + 1}</span>
    </div>
  </div>

  <!-- Note On / Note Off Parameters -->
  {#if config.message_type === 'note_on' || config.message_type === 'note_off'}
    <div class="editor-section">
      <label for="note" class="field-label">
        Note Number
        <span class="field-hint">(0-127)</span>
      </label>
      <div class="note-selector">
        <input
          id="note"
          type="range"
          min="0"
          max="127"
          bind:value={config.note}
          on:input={handleFieldChange}
          disabled={readonly}
          class="note-slider"
        />
        <div class="note-display">
          <span class="note-number">{config.note}</span>
          <span class="note-name">{getNoteName(config.note)}</span>
        </div>
      </div>
      <div class="piano-hint">
        <span>🎹 C4 (Middle C) = 60</span>
      </div>
    </div>

    {#if config.message_type === 'note_on'}
      <div class="editor-section">
        <VelocityMappingSelector
          bind:velocityMapping={config.velocity_mapping}
          {readonly}
          on:change={handleVelocityMappingChange}
        />
      </div>
    {/if}
  {/if}

  <!-- Control Change Parameters -->
  {#if config.message_type === 'cc'}
    <div class="editor-section">
      <label for="cc-number" class="field-label">
        CC Number
        <span class="field-hint">(0-127)</span>
      </label>
      <div class="cc-selector">
        <select
          id="cc-number"
          class="field-select"
          bind:value={config.cc_number}
          on:change={handleFieldChange}
          disabled={readonly}
        >
          <optgroup label="Common CCs">
            {#each commonCCs as cc}
              <option value={cc.value}>
                CC {cc.value} - {cc.label}
              </option>
            {/each}
          </optgroup>
          <optgroup label="Other">
            <option value="custom">Custom CC Number...</option>
          </optgroup>
        </select>
        {#if config.cc_number === 'custom'}
          <input
            type="number"
            min="0"
            max="127"
            bind:value={config.cc_number}
            on:input={handleFieldChange}
            disabled={readonly}
            class="value-input"
            placeholder="Enter CC number"
          />
        {/if}
      </div>
    </div>

    <div class="editor-section">
      <label for="cc-value" class="field-label">
        CC Value
        <span class="field-hint">(0-127)</span>
      </label>
      <div class="value-selector">
        <input
          id="cc-value"
          type="range"
          min="0"
          max="127"
          bind:value={config.cc_value}
          on:input={handleFieldChange}
          disabled={readonly}
          class="value-slider"
        />
        <input
          type="number"
          min="0"
          max="127"
          bind:value={config.cc_value}
          on:input={handleFieldChange}
          disabled={readonly}
          class="value-input"
        />
      </div>
    </div>
  {/if}

  <!-- Program Change Parameters -->
  {#if config.message_type === 'program_change'}
    <div class="editor-section">
      <label for="program" class="field-label">
        Program Number
        <span class="field-hint">(0-127)</span>
      </label>
      <div class="value-selector">
        <input
          id="program"
          type="range"
          min="0"
          max="127"
          bind:value={config.program}
          on:input={handleFieldChange}
          disabled={readonly}
          class="value-slider"
        />
        <input
          type="number"
          min="0"
          max="127"
          bind:value={config.program}
          on:input={handleFieldChange}
          disabled={readonly}
          class="value-input"
        />
      </div>
      <div class="program-hint">
        <span>💡 Program 0 = Acoustic Grand Piano (GM)</span>
      </div>
    </div>
  {/if}

  <!-- Pitch Bend Parameters -->
  {#if config.message_type === 'pitch_bend'}
    <div class="editor-section">
      <label for="pitch-bend" class="field-label">
        Pitch Bend
        <span class="field-hint">(-8192 to +8191)</span>
      </label>
      <div class="value-selector">
        <input
          id="pitch-bend"
          type="range"
          min="-8192"
          max="8191"
          bind:value={config.pitch_bend}
          on:input={handleFieldChange}
          disabled={readonly}
          class="value-slider"
        />
        <span class="pitch-bend-display">{getPitchBendDisplay(config.pitch_bend)}</span>
      </div>
      <div class="pitch-bend-indicator">
        <div class="pitch-bend-center"></div>
        <div
          class="pitch-bend-bar"
          class:positive={config.pitch_bend > 0}
          class:negative={config.pitch_bend < 0}
          style="width: {Math.abs(config.pitch_bend / 8192) * 50}%; {config.pitch_bend >= 0 ? 'left: 50%' : 'right: 50%'}"
        ></div>
      </div>
    </div>
  {/if}

  <!-- Aftertouch Parameters -->
  {#if config.message_type === 'aftertouch'}
    <div class="editor-section">
      <label for="aftertouch" class="field-label">
        Aftertouch Pressure
        <span class="field-hint">(0-127)</span>
      </label>
      <div class="value-selector">
        <input
          id="aftertouch"
          type="range"
          min="0"
          max="127"
          bind:value={config.aftertouch}
          on:input={handleFieldChange}
          disabled={readonly}
          class="value-slider"
        />
        <input
          type="number"
          min="0"
          max="127"
          bind:value={config.aftertouch}
          on:input={handleFieldChange}
          disabled={readonly}
          class="value-input"
        />
      </div>
    </div>
  {/if}

  <!-- Validation Messages -->
  {#if validationResult}
    <div class="validation-section">
      {#if validationResult.errors.length > 0}
        <div class="validation-errors">
          <h4 class="validation-heading">❌ Errors</h4>
          <ul class="validation-list">
            {#each validationResult.errors as error}
              <li class="validation-item error">{error}</li>
            {/each}
          </ul>
        </div>
      {/if}

      {#if validationResult.warnings.length > 0}
        <div class="validation-warnings">
          <h4 class="validation-heading">⚠️ Warnings</h4>
          <ul class="validation-list">
            {#each validationResult.warnings as warning}
              <li class="validation-item warning">{warning}</li>
            {/each}
          </ul>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .send-midi-editor {
    background: #1e1e1e;
    border-radius: 8px;
    border: 1px solid #333;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .editor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid #333;
  }

  .editor-title {
    font-size: 1.1rem;
    font-weight: 600;
    color: #e0e0e0;
    margin: 0;
  }

  .validation-status {
    padding: 0.35rem 0.75rem;
    border-radius: 4px;
    font-size: 0.85rem;
    font-weight: 600;
  }

  .validation-status.valid {
    background: rgba(34, 197, 94, 0.2);
    color: #4ade80;
    border: 1px solid rgba(34, 197, 94, 0.3);
  }

  .validation-status.invalid {
    background: rgba(220, 38, 38, 0.2);
    color: #f87171;
    border: 1px solid rgba(220, 38, 38, 0.3);
  }

  .editor-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .field-label {
    font-size: 0.95rem;
    font-weight: 600;
    color: #e0e0e0;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .field-hint {
    font-size: 0.8rem;
    font-weight: 400;
    color: #909090;
  }

  .field-select {
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 6px;
    padding: 0.65rem 0.85rem;
    color: #e0e0e0;
    font-size: 0.95rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .field-select:hover:not(:disabled) {
    border-color: #666;
  }

  .field-select:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
  }

  .field-select:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .channel-selector,
  .note-selector,
  .value-selector {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .channel-slider,
  .note-slider,
  .value-slider {
    flex: 1;
    height: 6px;
    border-radius: 3px;
    background: #333;
    outline: none;
    -webkit-appearance: none;
  }

  .channel-slider::-webkit-slider-thumb,
  .note-slider::-webkit-slider-thumb,
  .value-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: #3b82f6;
    cursor: pointer;
    transition: all 0.2s;
  }

  .channel-slider::-webkit-slider-thumb:hover,
  .note-slider::-webkit-slider-thumb:hover,
  .value-slider::-webkit-slider-thumb:hover {
    background: #2563eb;
    transform: scale(1.1);
  }

  .channel-slider::-moz-range-thumb,
  .note-slider::-moz-range-thumb,
  .value-slider::-moz-range-thumb {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: #3b82f6;
    cursor: pointer;
    border: none;
    transition: all 0.2s;
  }

  .channel-slider::-moz-range-thumb:hover,
  .note-slider::-moz-range-thumb:hover,
  .value-slider::-moz-range-thumb:hover {
    background: #2563eb;
    transform: scale(1.1);
  }

  .channel-display {
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 4px;
    padding: 0.4rem 0.75rem;
    font-size: 0.95rem;
    font-weight: 600;
    color: #3b82f6;
    min-width: 3rem;
    text-align: center;
  }

  .note-display {
    display: flex;
    flex-direction: column;
    align-items: center;
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 4px;
    padding: 0.4rem 0.75rem;
    min-width: 4rem;
  }

  .note-number {
    font-size: 0.95rem;
    font-weight: 600;
    color: #3b82f6;
  }

  .note-name {
    font-size: 0.75rem;
    color: #909090;
  }

  .value-input {
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 4px;
    padding: 0.4rem 0.6rem;
    color: #e0e0e0;
    font-size: 0.95rem;
    width: 5rem;
    text-align: center;
  }

  .value-input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
  }

  .value-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .piano-hint,
  .program-hint {
    font-size: 0.8rem;
    color: #909090;
    padding: 0.35rem 0.5rem;
    background: rgba(59, 130, 246, 0.1);
    border-radius: 4px;
    border: 1px solid rgba(59, 130, 246, 0.2);
  }

  .velocity-indicator {
    height: 4px;
    background: #333;
    border-radius: 2px;
    overflow: hidden;
  }

  .velocity-bar {
    height: 100%;
    background: linear-gradient(to right, #4ade80, #facc15, #f87171);
    transition: width 0.2s;
  }

  .pitch-bend-display {
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 4px;
    padding: 0.4rem 0.75rem;
    font-size: 0.95rem;
    font-weight: 600;
    color: #3b82f6;
    min-width: 6rem;
    text-align: center;
  }

  .pitch-bend-indicator {
    position: relative;
    height: 8px;
    background: #333;
    border-radius: 4px;
    overflow: visible;
  }

  .pitch-bend-center {
    position: absolute;
    left: 50%;
    top: 0;
    bottom: 0;
    width: 2px;
    background: #666;
    transform: translateX(-50%);
  }

  .pitch-bend-bar {
    position: absolute;
    top: 0;
    bottom: 0;
    transition: all 0.2s;
  }

  .pitch-bend-bar.positive {
    background: #3b82f6;
  }

  .pitch-bend-bar.negative {
    background: #f59e0b;
  }

  .validation-section {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px solid #333;
  }

  .validation-errors,
  .validation-warnings {
    padding: 0.75rem 1rem;
    border-radius: 6px;
  }

  .validation-errors {
    background: rgba(220, 38, 38, 0.1);
    border: 1px solid rgba(220, 38, 38, 0.3);
  }

  .validation-warnings {
    background: rgba(245, 158, 11, 0.1);
    border: 1px solid rgba(245, 158, 11, 0.3);
  }

  .validation-heading {
    font-size: 0.9rem;
    font-weight: 600;
    margin: 0 0 0.5rem 0;
  }

  .validation-list {
    margin: 0;
    padding-left: 1.25rem;
    list-style: none;
  }

  .validation-item {
    font-size: 0.85rem;
    margin-bottom: 0.25rem;
    position: relative;
  }

  .validation-item::before {
    content: '•';
    position: absolute;
    left: -1rem;
  }

  .validation-item.error {
    color: #f87171;
  }

  .validation-item.warning {
    color: #fbbf24;
  }

  .cc-selector {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
</style>
