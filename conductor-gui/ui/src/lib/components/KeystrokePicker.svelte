<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';

  /**
   * Props
   */
  export let isOpen = false;
  export let onSelect = null; // Callback: ({ keys, modifiers }) => void
  export let onClose = null;

  const dispatch = createEventDispatcher();

  /**
   * Local state
   */
  let capturedKey = '';
  let capturedModifiers = [];
  let keyPressed = false;
  let listening = false;

  /**
   * Key name mapping for special keys
   */
  const keyNameMap = {
    ' ': 'space',
    'Enter': 'Return',
    'Escape': 'Escape',
    'Backspace': 'Backspace',
    'Delete': 'Delete',
    'Tab': 'Tab',
    'ArrowUp': 'Up',
    'ArrowDown': 'Down',
    'ArrowLeft': 'Left',
    'ArrowRight': 'Right',
    'Home': 'Home',
    'End': 'End',
    'PageUp': 'PageUp',
    'PageDown': 'PageDown',
  };

  /**
   * Function keys F1-F12
   */
  for (let i = 1; i <= 12; i++) {
    keyNameMap[`F${i}`] = `F${i}`;
  }

  /**
   * Handle keydown event
   */
  function handleKeyDown(event) {
    if (!listening) return;

    // Prevent default browser behavior
    event.preventDefault();
    event.stopPropagation();

    // Ignore modifier-only presses
    if (['Control', 'Shift', 'Alt', 'Meta', 'Command'].includes(event.key)) {
      return;
    }

    // Get key name
    let keyName = keyNameMap[event.key] || event.key;

    // For single character keys, use the key as-is
    if (keyName.length === 1) {
      keyName = event.key;
    }

    capturedKey = keyName;
    capturedModifiers = [];

    // Capture modifiers
    if (event.metaKey || event.key === 'Meta') capturedModifiers.push('cmd');
    if (event.ctrlKey || event.key === 'Control') capturedModifiers.push('ctrl');
    if (event.shiftKey || event.key === 'Shift') capturedModifiers.push('shift');
    if (event.altKey || event.key === 'Alt') capturedModifiers.push('alt');

    keyPressed = true;
  }

  /**
   * Start listening for key presses
   */
  function startListening() {
    listening = true;
    keyPressed = false;
    capturedKey = '';
    capturedModifiers = [];
  }

  /**
   * Stop listening
   */
  function stopListening() {
    listening = false;
  }

  /**
   * Use the captured keystroke
   */
  function useKeystroke() {
    if (!capturedKey) return;

    const result = {
      keys: capturedKey,
      modifiers: capturedModifiers,
    };

    if (onSelect) {
      onSelect(result);
    }

    dispatch('select', result);
    close();
  }

  /**
   * Clear captured keystroke
   */
  function clearCapture() {
    capturedKey = '';
    capturedModifiers = [];
    keyPressed = false;
  }

  /**
   * Close dialog
   */
  function close() {
    stopListening();
    if (onClose) {
      onClose();
    }
    isOpen = false;
  }

  /**
   * Lifecycle: Mount event listeners
   */
  onMount(() => {
    if (isOpen) {
      window.addEventListener('keydown', handleKeyDown);
      startListening();
    }
  });

  /**
   * Lifecycle: Cleanup
   */
  onDestroy(() => {
    window.removeEventListener('keydown', handleKeyDown);
    stopListening();
  });

  /**
   * Watch isOpen changes
   */
  $: if (isOpen) {
    window.addEventListener('keydown', handleKeyDown);
    startListening();
  } else {
    window.removeEventListener('keydown', handleKeyDown);
    stopListening();
  }
</script>

{#if isOpen}
  <div class="overlay" on:click={close}>
    <div class="dialog" on:click|stopPropagation>
      <div class="dialog-header">
        <h2>Press Any Key</h2>
        <button class="close-btn" on:click={close} aria-label="Close">×</button>
      </div>

      <div class="dialog-body">
        {#if !keyPressed}
          <!-- Waiting for input -->
          <div class="listening-state">
            <div class="keyboard-icon">⌨️</div>
            <p class="instruction">Press any key combination...</p>
            <div class="pulse-indicator"></div>
          </div>
        {:else}
          <!-- Key captured -->
          <div class="captured-state">
            <div class="success-icon">✓</div>
            <p class="success-message">Key captured!</p>

            <div class="keystroke-preview">
              <h3>Keystroke:</h3>
              <div class="keystroke-display">
                {#if capturedModifiers.length > 0}
                  {#each capturedModifiers as mod}
                    <span class="key-badge modifier">{mod}</span>
                    <span class="plus">+</span>
                  {/each}
                {/if}
                <span class="key-badge main">{capturedKey}</span>
              </div>
            </div>

            <div class="actions">
              <button class="btn btn-secondary" on:click={clearCapture}>
                Try Again
              </button>
              <button class="btn btn-primary" on:click={useKeystroke}>
                Use This
              </button>
            </div>
          </div>
        {/if}
      </div>

      <div class="dialog-footer">
        <button class="btn btn-ghost" on:click={close}>Cancel</button>
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
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
    backdrop-filter: blur(6px);
  }

  .dialog {
    background: var(--bg-secondary, #2a2a2a);
    border-radius: 16px;
    box-shadow: 0 12px 48px rgba(0, 0, 0, 0.5);
    width: 90%;
    max-width: 500px;
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
    padding: 3rem 2rem;
    min-height: 300px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .dialog-footer {
    padding: 1rem 1.5rem;
    border-top: 1px solid var(--border-color, #444);
    display: flex;
    justify-content: center;
  }

  /* Listening state */
  .listening-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.5rem;
    text-align: center;
  }

  .keyboard-icon {
    font-size: 4rem;
    line-height: 1;
    animation: float 3s ease-in-out infinite;
  }

  @keyframes float {
    0%, 100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-10px);
    }
  }

  .instruction {
    font-size: 1.25rem;
    color: var(--text-primary, #fff);
    margin: 0;
  }

  .pulse-indicator {
    width: 16px;
    height: 16px;
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
      opacity: 0.4;
      transform: scale(1.4);
    }
  }

  /* Captured state */
  .captured-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.5rem;
    width: 100%;
  }

  .success-icon {
    width: 64px;
    height: 64px;
    background: var(--success-color, #4ade80);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    color: white;
    animation: scaleIn 0.3s ease-out;
  }

  @keyframes scaleIn {
    0% {
      transform: scale(0);
    }
    50% {
      transform: scale(1.1);
    }
    100% {
      transform: scale(1);
    }
  }

  .success-message {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--success-color, #4ade80);
    margin: 0;
  }

  .keystroke-preview {
    width: 100%;
    background: var(--bg-tertiary, #1a1a1a);
    border-radius: 12px;
    padding: 1.5rem;
    border: 2px solid var(--border-color, #444);
  }

  .keystroke-preview h3 {
    margin: 0 0 1rem 0;
    font-size: 0.875rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--text-secondary, #999);
  }

  .keystroke-display {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .key-badge {
    padding: 0.75rem 1.25rem;
    border-radius: 8px;
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 1.125rem;
    font-weight: 600;
    text-transform: uppercase;
    border: 2px solid var(--border-color, #444);
  }

  .key-badge.modifier {
    background: var(--bg-primary, #0a0a0a);
    color: var(--accent-color, #3b82f6);
  }

  .key-badge.main {
    background: var(--accent-color, #3b82f6);
    color: white;
    border-color: var(--accent-color, #3b82f6);
  }

  .plus {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-secondary, #999);
  }

  .actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 1rem;
  }

  /* Buttons */
  .btn {
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all 0.2s;
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

  .btn-ghost {
    background: transparent;
    color: var(--text-secondary, #999);
  }

  .btn-ghost:hover {
    color: var(--text-primary, #fff);
    background: var(--bg-hover, #333);
  }
</style>
