<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { createEventDispatcher } from 'svelte';
  import SendMidiActionEditor from './SendMidiActionEditor.svelte';
  import ConditionalActionEditor from './ConditionalActionEditor.svelte';

  /**
   * Props
   */
  export let action = null; // Current action object
  export let readonly = false;
  export let availableModes = []; // For ModeChange action

  const dispatch = createEventDispatcher();

  /**
   * Action type definitions
   */
  const actionTypes = [
    { value: 'Keystroke', label: 'Keystroke', description: 'Press keyboard keys' },
    { value: 'Text', label: 'Type Text', description: 'Type a text string' },
    { value: 'Launch', label: 'Launch App', description: 'Open an application' },
    { value: 'Shell', label: 'Shell Command', description: 'Run a shell command' },
    { value: 'MouseClick', label: 'Mouse Click', description: 'Simulate mouse click' },
    { value: 'VolumeControl', label: 'Volume Control', description: 'Control system volume' },
    { value: 'ModeChange', label: 'Change Mode', description: 'Switch to another mode' },
    { value: 'Delay', label: 'Delay', description: 'Wait for duration' },
    { value: 'SendMidi', label: 'Send MIDI', description: 'Send MIDI output message' },
    { value: 'Conditional', label: 'Conditional', description: 'Execute action based on condition' },
  ];

  /**
   * Available modifiers for Keystroke
   */
  const modifierKeys = ['cmd', 'ctrl', 'shift', 'alt', 'option'];

  /**
   * Volume operations
   */
  const volumeOperations = ['Up', 'Down', 'Mute', 'Unmute', 'Set'];

  /**
   * Mouse buttons
   */
  const mouseButtons = ['left', 'right', 'middle'];

  /**
   * Local state
   */
  let selectedType = action?.type || 'Keystroke';
  let config = action || createDefaultConfig('Keystroke');
  let selectedModifiers = action?.modifiers || [];

  /**
   * Create default config for an action type
   */
  function createDefaultConfig(type) {
    const defaults = {
      Keystroke: { type: 'Keystroke', keys: '', modifiers: [] },
      Text: { type: 'Text', text: '' },
      Launch: { type: 'Launch', app: '' },
      Shell: { type: 'Shell', command: '' },
      MouseClick: { type: 'MouseClick', button: 'left', x: null, y: null },
      VolumeControl: { type: 'VolumeControl', operation: 'Up', value: null },
      ModeChange: { type: 'ModeChange', mode: availableModes[0] || 'Default' },
      Delay: { type: 'Delay', ms: 100 },
      SendMidi: {
        type: 'SendMidi',
        port: '',
        message_type: 'note_on',
        channel: 0,
        note: 60,
        velocity: 100,
      },
      Conditional: {
        type: 'Conditional',
        condition: 'Always',
        then_action: { type: 'Keystroke', keys: '', modifiers: [] },
        else_action: null,
      },
    };
    return defaults[type] || defaults.Keystroke;
  }

  /**
   * Handle type change
   */
  function handleTypeChange() {
    config = createDefaultConfig(selectedType);
    selectedModifiers = [];
    emitChange();
  }

  /**
   * Toggle modifier key
   */
  function toggleModifier(mod) {
    if (selectedModifiers.includes(mod)) {
      selectedModifiers = selectedModifiers.filter(m => m !== mod);
    } else {
      selectedModifiers = [...selectedModifiers, mod];
    }
    config.modifiers = selectedModifiers;
    emitChange();
  }

  /**
   * Emit change event
   */
  function emitChange() {
    dispatch('change', config);
  }

  /**
   * Open keystroke picker
   */
  function openKeystrokePicker() {
    dispatch('keystrokePicker');
  }
</script>

<div class="action-selector">
  <div class="section-header">
    <h3>Action Configuration</h3>
  </div>

  <div class="form-group">
    <label for="action-type">Action Type</label>
    <select
      id="action-type"
      bind:value={selectedType}
      on:change={handleTypeChange}
      disabled={readonly}
    >
      {#each actionTypes as type}
        <option value={type.value}>
          {type.label} - {type.description}
        </option>
      {/each}
    </select>
  </div>

  <!-- Keystroke Action -->
  {#if selectedType === 'Keystroke'}
    <div class="config-fields">
      <div class="form-group">
        <label for="keys">Key(s)</label>
        <div class="input-with-button">
          <input
            id="keys"
            type="text"
            bind:value={config.keys}
            on:input={emitChange}
            placeholder="e.g., space, Return, a"
            disabled={readonly}
          />
          {#if !readonly}
            <button class="btn btn-secondary" on:click={openKeystrokePicker}>
              🎹 Pick
            </button>
          {/if}
        </div>
        <small>Enter key name (case-sensitive)</small>
      </div>
      <div class="form-group">
        <label>Modifiers</label>
        <div class="modifier-grid">
          {#each modifierKeys as mod}
            <button
              class="modifier-btn"
              class:active={selectedModifiers.includes(mod)}
              on:click={() => toggleModifier(mod)}
              disabled={readonly}
            >
              {mod}
            </button>
          {/each}
        </div>
      </div>
      {#if config.keys || selectedModifiers.length > 0}
        <div class="preview">
          <strong>Preview:</strong>
          <code>{selectedModifiers.join(' + ')}{selectedModifiers.length > 0 ? ' + ' : ''}{config.keys || '(key)'}</code>
        </div>
      {/if}
    </div>
  {/if}

  <!-- Text Action -->
  {#if selectedType === 'Text'}
    <div class="config-fields">
      <div class="form-group">
        <label for="text">Text to Type</label>
        <textarea
          id="text"
          bind:value={config.text}
          on:input={emitChange}
          placeholder="Enter text to type..."
          rows="4"
          disabled={readonly}
        />
        <small>{config.text?.length || 0} characters</small>
      </div>
    </div>
  {/if}

  <!-- Launch Action -->
  {#if selectedType === 'Launch'}
    <div class="config-fields">
      <div class="form-group">
        <label for="app">Application Name or Path</label>
        <input
          id="app"
          type="text"
          bind:value={config.app}
          on:input={emitChange}
          placeholder="e.g., Safari, /Applications/Xcode.app"
          disabled={readonly}
        />
        <small>Application name or full path to executable</small>
      </div>
    </div>
  {/if}

  <!-- Shell Action -->
  {#if selectedType === 'Shell'}
    <div class="config-fields">
      <div class="form-group">
        <label for="command">Shell Command</label>
        <textarea
          id="command"
          bind:value={config.command}
          on:input={emitChange}
          placeholder="e.g., open ~/Desktop"
          rows="3"
          disabled={readonly}
        />
        <small class="warning">⚠️ Be cautious with shell commands</small>
      </div>
    </div>
  {/if}

  <!-- Mouse Click Action -->
  {#if selectedType === 'MouseClick'}
    <div class="config-fields">
      <div class="form-group">
        <label for="button">Mouse Button</label>
        <select
          id="button"
          bind:value={config.button}
          on:change={emitChange}
          disabled={readonly}
        >
          {#each mouseButtons as btn}
            <option value={btn}>{btn}</option>
          {/each}
        </select>
      </div>
      <div class="form-group">
        <label>Position (optional)</label>
        <div class="coord-inputs">
          <input
            type="number"
            bind:value={config.x}
            on:input={emitChange}
            placeholder="X"
            disabled={readonly}
          />
          <input
            type="number"
            bind:value={config.y}
            on:input={emitChange}
            placeholder="Y"
            disabled={readonly}
          />
        </div>
        <small>Leave empty to click at current mouse position</small>
      </div>
    </div>
  {/if}

  <!-- Volume Control Action -->
  {#if selectedType === 'VolumeControl'}
    <div class="config-fields">
      <div class="form-group">
        <label for="operation">Operation</label>
        <select
          id="operation"
          bind:value={config.operation}
          on:change={emitChange}
          disabled={readonly}
        >
          {#each volumeOperations as op}
            <option value={op}>{op}</option>
          {/each}
        </select>
      </div>
      {#if config.operation === 'Set'}
        <div class="form-group">
          <label for="volume-value">Volume Level (0-100)</label>
          <input
            id="volume-value"
            type="number"
            min="0"
            max="100"
            bind:value={config.value}
            on:input={emitChange}
            disabled={readonly}
          />
        </div>
      {/if}
    </div>
  {/if}

  <!-- Mode Change Action -->
  {#if selectedType === 'ModeChange'}
    <div class="config-fields">
      <div class="form-group">
        <label for="mode">Target Mode</label>
        {#if availableModes.length > 0}
          <select
            id="mode"
            bind:value={config.mode}
            on:change={emitChange}
            disabled={readonly}
          >
            {#each availableModes as mode}
              <option value={mode}>{mode}</option>
            {/each}
          </select>
        {:else}
          <input
            id="mode"
            type="text"
            bind:value={config.mode}
            on:input={emitChange}
            placeholder="Mode name"
            disabled={readonly}
          />
          <small class="warning">No modes available. Enter mode name manually.</small>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Delay Action -->
  {#if selectedType === 'Delay'}
    <div class="config-fields">
      <div class="form-group">
        <label for="delay-ms">Duration (milliseconds)</label>
        <input
          id="delay-ms"
          type="number"
          min="1"
          max="10000"
          step="10"
          bind:value={config.ms}
          on:input={emitChange}
          disabled={readonly}
        />
        <small>{(config.ms / 1000).toFixed(2)}s delay</small>
      </div>
    </div>
  {/if}

  <!-- SendMidi Action -->
  {#if selectedType === 'SendMidi'}
    <SendMidiActionEditor
      bind:action={config}
      {readonly}
      on:change={(e) => {
        config = e.detail.config;
        emitChange();
      }}
    />
  {/if}

  <!-- Conditional Action -->
  {#if selectedType === 'Conditional'}
    <ConditionalActionEditor
      bind:action={config}
      {readonly}
      {availableModes}
      on:change={(e) => {
        config = e.detail.config;
        emitChange();
      }}
    />
  {/if}
</div>

<style>
  .action-selector {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .section-header {
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
  .form-group select,
  .form-group textarea {
    padding: 0.75rem;
    background: var(--bg-tertiary, #1a1a1a);
    border: 1px solid var(--border-color, #444);
    border-radius: 6px;
    color: var(--text-primary, #fff);
    font-size: 0.875rem;
    font-family: inherit;
  }

  .form-group textarea {
    resize: vertical;
    min-height: 80px;
  }

  .form-group input:focus,
  .form-group select:focus,
  .form-group textarea:focus {
    outline: none;
    border-color: var(--accent-color, #3b82f6);
  }

  .form-group input:disabled,
  .form-group select:disabled,
  .form-group textarea:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .form-group small {
    font-size: 0.75rem;
    color: var(--text-secondary, #999);
  }

  .form-group small.warning {
    color: var(--error-color, #ef4444);
  }

  .input-with-button {
    display: flex;
    gap: 0.5rem;
  }

  .input-with-button input {
    flex: 1;
  }

  .btn {
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .btn-secondary {
    background: var(--bg-tertiary, #1a1a1a);
    color: var(--text-primary, #fff);
    border: 1px solid var(--border-color, #444);
  }

  .btn-secondary:hover {
    background: var(--bg-hover, #333);
  }

  .btn-secondary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .modifier-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
    gap: 0.5rem;
  }

  .modifier-btn {
    padding: 0.75rem;
    background: var(--bg-primary, #0a0a0a);
    border: 2px solid var(--border-color, #444);
    border-radius: 6px;
    color: var(--text-secondary, #999);
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    text-transform: uppercase;
  }

  .modifier-btn:hover:not(:disabled) {
    border-color: var(--accent-color, #3b82f6);
  }

  .modifier-btn.active {
    background: var(--accent-color, #3b82f6);
    border-color: var(--accent-color, #3b82f6);
    color: white;
  }

  .modifier-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .coord-inputs {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
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

  .preview {
    padding: 0.75rem;
    background: var(--bg-primary, #0a0a0a);
    border-radius: 6px;
    border: 1px solid var(--border-color, #444);
    font-size: 0.875rem;
  }

  .preview strong {
    color: var(--text-secondary, #999);
    margin-right: 0.5rem;
  }

  .preview code {
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    color: var(--accent-color, #3b82f6);
    font-weight: 600;
  }
</style>
