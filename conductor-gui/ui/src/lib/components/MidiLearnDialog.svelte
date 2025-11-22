<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { midiLearnStore } from '$lib/stores.js';
  import { invoke } from '@tauri-apps/api/core';
  import { onDestroy } from 'svelte';

  /**
   * Props
   */
  export let isOpen = false;
  export let onCapture = null; // Callback when input is captured: (result) => void
  export let onClose = null;   // Callback when dialog closes: () => void
  export let timeoutSecs = 10; // Default timeout
  export let modeName = "Default"; // Current mode name for config generation

  /**
   * Local state
   */
  let startError = null;
  let generatedToml = '';
  let copySuccess = false;

  /**
   * Reactive statements
   */
  $: state = $midiLearnStore.state;
  $: remaining = $midiLearnStore.remaining;
  $: result = $midiLearnStore.result;
  $: error = $midiLearnStore.error;

  /**
   * Start session when dialog opens
   */
  $: if (isOpen && state === 'Idle') {
    startSession();
  }

  /**
   * Handle successful capture - generate TOML
   */
  $: if (state === 'Captured' && result?.success) {
    generateTomlConfig();
    if (onCapture) {
      onCapture(result);
    }
  }

  /**
   * Generate TOML config from captured trigger
   */
  async function generateTomlConfig() {
    if (!result?.trigger) return;

    try {
      generatedToml = await invoke('generate_trigger_config_toml', {
        suggestion: result.trigger,
        modeName,
      });
    } catch (err) {
      console.error('Failed to generate TOML:', err);
      generatedToml = '# Error generating config';
    }
  }

  /**
   * Copy TOML to clipboard
   */
  async function copyToml() {
    try {
      await navigator.clipboard.writeText(generatedToml);
      copySuccess = true;
      setTimeout(() => {
        copySuccess = false;
      }, 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  }

  /**
   * Start MIDI Learn session
   */
  async function startSession() {
    startError = null;
    try {
      await midiLearnStore.start(timeoutSecs);
    } catch (err) {
      console.error('Failed to start MIDI Learn:', err);
      startError = err.message || String(err);
    }
  }

  /**
   * Cancel session
   */
  async function handleCancel() {
    try {
      await midiLearnStore.cancel();
      close();
    } catch (err) {
      console.error('Failed to cancel MIDI Learn:', err);
    }
  }

  /**
   * Close dialog
   */
  function close() {
    midiLearnStore.reset();
    if (onClose) {
      onClose();
    }
    isOpen = false;
  }

  /**
   * Handle "Use This" button
   */
  function handleUse() {
    if (result?.success && onCapture) {
      onCapture(result);
    }
    close();
  }

  /**
   * Cleanup on destroy
   */
  onDestroy(() => {
    midiLearnStore.reset();
  });

  /**
   * Format trigger suggestion for display
   */
  function formatTrigger(trigger) {
    if (!trigger) return '';

    switch (trigger.type) {
      case 'Note':
        return `Note ${trigger.note}${trigger.velocity_range ? ` (velocity ${trigger.velocity_range[0]}-${trigger.velocity_range[1]})` : ''}`;

      case 'VelocityRange':
        return `Note ${trigger.note} - ${trigger.level} (${trigger.velocity_min}-${trigger.velocity_max})`;

      case 'LongPress':
        return `Long Press Note ${trigger.note} (${trigger.duration_ms}ms)`;

      case 'DoubleTap':
        return `Double Tap Note ${trigger.note} (${trigger.timeout_ms}ms window)`;

      case 'Chord':
        return `Chord: Notes ${trigger.notes.join(', ')} (${trigger.window_ms}ms window)`;

      case 'Encoder':
        return `Encoder CC ${trigger.cc}${trigger.direction ? ` (${trigger.direction})` : ''}`;

      case 'CC':
        return `CC ${trigger.cc}${trigger.value_range ? ` (${trigger.value_range[0]}-${trigger.value_range[1]})` : ''}`;

      case 'Aftertouch':
        return `Aftertouch${trigger.note != null ? ` Note ${trigger.note}` : ''} (pressure ${trigger.pressure_range[0]}-${trigger.pressure_range[1]})`;

      case 'PitchBend':
        return `Pitch Bend (${trigger.bend_range[0]} to ${trigger.bend_range[1]})`;

      default:
        return JSON.stringify(trigger);
    }
  }
</script>

{#if isOpen}
  <div class="overlay" on:click={handleCancel}>
    <div class="dialog" on:click|stopPropagation>
      <div class="dialog-header">
        <h2>MIDI Learn</h2>
        <button class="close-btn" on:click={close} aria-label="Close">×</button>
      </div>

      <div class="dialog-body">
        {#if startError}
          <!-- Start error -->
          <div class="error-state">
            <div class="icon">⚠️</div>
            <p class="error-message">{startError}</p>
            <button class="btn btn-primary" on:click={close}>Close</button>
          </div>

        {:else if state === 'Waiting'}
          <!-- Waiting for input -->
          <div class="waiting-state">
            <div class="pulse-indicator"></div>
            <p class="instruction">Press any key, pad, or move a controller on your MIDI device</p>
            <div class="countdown">
              <div class="countdown-circle">
                <svg viewBox="0 0 100 100">
                  <circle
                    class="countdown-bg"
                    cx="50"
                    cy="50"
                    r="45"
                  />
                  <circle
                    class="countdown-progress"
                    cx="50"
                    cy="50"
                    r="45"
                    style="--progress: {(remaining / timeoutSecs) * 100}%"
                  />
                </svg>
                <span class="countdown-text">{remaining}s</span>
              </div>
            </div>
            <button class="btn btn-secondary" on:click={handleCancel}>Cancel</button>
          </div>

        {:else if state === 'Captured' && result?.success}
          <!-- Successfully captured -->
          <div class="success-state">
            <div class="icon success">✓</div>
            <p class="success-message">MIDI input captured!</p>
            <div class="trigger-preview">
              <h3>Detected Trigger:</h3>
              <div class="trigger-details">
                {formatTrigger(result.trigger)}
              </div>
            </div>

            {#if generatedToml}
              <div class="config-preview">
                <div class="config-header">
                  <h3>Generated Config:</h3>
                  <button
                    class="btn btn-icon"
                    on:click={copyToml}
                    title="Copy to clipboard"
                  >
                    {#if copySuccess}
                      <span class="copy-icon">✓</span>
                    {:else}
                      <span class="copy-icon">📋</span>
                    {/if}
                  </button>
                </div>
                <pre class="config-code">{generatedToml}</pre>
              </div>
            {/if}

            <div class="actions">
              <button class="btn btn-secondary" on:click={close}>Cancel</button>
              <button class="btn btn-primary" on:click={handleUse}>Use This</button>
            </div>
          </div>

        {:else if state === 'TimedOut'}
          <!-- Timeout -->
          <div class="error-state">
            <div class="icon">⏱️</div>
            <p class="error-message">
              {result?.error || 'No MIDI input received within the timeout period'}
            </p>
            <button class="btn btn-primary" on:click={close}>Close</button>
          </div>

        {:else if state === 'Cancelled'}
          <!-- Cancelled -->
          <div class="info-state">
            <div class="icon">🚫</div>
            <p>MIDI Learn cancelled</p>
            <button class="btn btn-primary" on:click={close}>Close</button>
          </div>

        {:else}
          <!-- Unknown state -->
          <div class="info-state">
            <p>State: {state}</p>
            <button class="btn btn-primary" on:click={close}>Close</button>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(4px);
  }

  .dialog {
    background: var(--bg-secondary, #2a2a2a);
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    width: 90%;
    max-width: 500px;
    max-height: 80vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .dialog-header {
    padding: 1.5rem;
    border-bottom: 1px solid var(--border-color, #444);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .dialog-header h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 2rem;
    line-height: 1;
    cursor: pointer;
    color: var(--text-secondary, #999);
    padding: 0;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    transition: all 0.2s;
  }

  .close-btn:hover {
    background: var(--bg-hover, #333);
    color: var(--text-primary, #fff);
  }

  .dialog-body {
    padding: 2rem;
    overflow-y: auto;
  }

  /* State containers */
  .waiting-state,
  .success-state,
  .error-state,
  .info-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.5rem;
    text-align: center;
  }

  .icon {
    font-size: 3rem;
    line-height: 1;
  }

  .icon.success {
    color: var(--success-color, #4ade80);
  }

  /* Waiting state */
  .pulse-indicator {
    width: 12px;
    height: 12px;
    background: var(--accent-color, #3b82f6);
    border-radius: 50%;
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
      transform: scale(1);
    }
    50% {
      opacity: 0.5;
      transform: scale(1.3);
    }
  }

  .instruction {
    font-size: 1.125rem;
    color: var(--text-primary, #fff);
    margin: 0;
  }

  /* Countdown */
  .countdown {
    margin: 1rem 0;
  }

  .countdown-circle {
    position: relative;
    width: 120px;
    height: 120px;
  }

  .countdown-circle svg {
    width: 100%;
    height: 100%;
    transform: rotate(-90deg);
  }

  .countdown-bg {
    fill: none;
    stroke: var(--border-color, #444);
    stroke-width: 8;
  }

  .countdown-progress {
    fill: none;
    stroke: var(--accent-color, #3b82f6);
    stroke-width: 8;
    stroke-linecap: round;
    stroke-dasharray: 283;
    stroke-dashoffset: calc(283 - (283 * var(--progress) / 100));
    transition: stroke-dashoffset 0.1s linear;
  }

  .countdown-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 2rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
  }

  /* Success state */
  .success-message {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--success-color, #4ade80);
    margin: 0;
  }

  .trigger-preview {
    width: 100%;
    background: var(--bg-tertiary, #1a1a1a);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .trigger-preview h3 {
    margin: 0 0 0.75rem 0;
    font-size: 0.875rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--text-secondary, #999);
  }

  .trigger-details {
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 1rem;
    color: var(--text-primary, #fff);
    padding: 0.75rem;
    background: var(--bg-primary, #0a0a0a);
    border-radius: 4px;
    border: 1px solid var(--border-color, #444);
  }

  /* Config preview */
  .config-preview {
    width: 100%;
    margin-top: 1rem;
  }

  .config-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.5rem;
  }

  .config-header h3 {
    margin: 0;
    font-size: 0.875rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--text-secondary, #999);
  }

  .btn-icon {
    background: none;
    border: none;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
    border-radius: 4px;
    transition: background 0.2s;
  }

  .btn-icon:hover {
    background: var(--bg-hover, #333);
  }

  .copy-icon {
    font-size: 1.25rem;
  }

  .config-code {
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 0.875rem;
    color: var(--text-primary, #fff);
    padding: 1rem;
    background: var(--bg-primary, #0a0a0a);
    border-radius: 4px;
    border: 1px solid var(--border-color, #444);
    overflow-x: auto;
    max-height: 300px;
    overflow-y: auto;
    margin: 0;
    white-space: pre;
  }

  /* Error state */
  .error-message {
    color: var(--error-color, #ef4444);
    font-size: 1rem;
    margin: 0;
  }

  /* Buttons */
  .btn {
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all 0.2s;
    outline: none;
  }

  .btn-primary {
    background: var(--accent-color, #3b82f6);
    color: white;
  }

  .btn-primary:hover {
    background: var(--accent-hover, #2563eb);
  }

  .btn-secondary {
    background: var(--bg-tertiary, #1a1a1a);
    color: var(--text-primary, #fff);
    border: 1px solid var(--border-color, #444);
  }

  .btn-secondary:hover {
    background: var(--bg-hover, #333);
  }

  .actions {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
  }
</style>
