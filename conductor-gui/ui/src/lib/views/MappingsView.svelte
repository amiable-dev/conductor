<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { onMount } from 'svelte';
  import MappingList from '$lib/components/MappingList.svelte';
  import TriggerSelector from '$lib/components/TriggerSelector.svelte';
  import ActionSelector from '$lib/components/ActionSelector.svelte';
  import MidiLearnDialog from '$lib/components/MidiLearnDialog.svelte';
  import { configStore, appStore, midiLearnStore } from '$lib/stores.js';

  /**
   * Local state
   */
  let config = null;
  let selectedMode = 0;
  let mappings = [];
  let globalMappings = [];
  let showMidiLearn = false;
  let showMappingEditor = false;
  let editingMapping = null;
  let editingIndex = null;
  let isGlobal = false;
  let loading = false;
  let error = null;

  /**
   * Load configuration on mount
   */
  onMount(async () => {
    await loadConfig();
  });

  /**
   * Load configuration from daemon
   */
  async function loadConfig() {
    loading = true;
    error = null;
    try {
      const cfg = await configStore.fetch();
      config = cfg;

      // Extract mappings for selected mode
      if (cfg && cfg.modes && cfg.modes[selectedMode]) {
        mappings = cfg.modes[selectedMode].mappings || [];
      }

      // Extract global mappings
      globalMappings = cfg.global_mappings || [];

    } catch (err) {
      error = err.message || String(err);
      appStore.setError(error);
    } finally {
      loading = false;
    }
  }

  /**
   * Save configuration to daemon
   */
  async function saveConfig() {
    loading = true;
    error = null;
    try {
      // Update config object
      if (isGlobal) {
        config.global_mappings = globalMappings;
      } else {
        if (config.modes && config.modes[selectedMode]) {
          config.modes[selectedMode].mappings = mappings;
        }
      }

      // Save via store
      await configStore.save(config);

    } catch (err) {
      error = err.message || String(err);
      appStore.setError(error);
    } finally {
      loading = false;
    }
  }

  /**
   * Open MIDI Learn dialog
   */
  function openMidiLearn() {
    showMidiLearn = true;
  }

  /**
   * Handle MIDI Learn complete
   */
  async function handleMidiLearnComplete(event) {
    const suggestion = event.detail;

    // Create new mapping from suggestion
    editingMapping = {
      trigger: suggestion.trigger,
      action: { type: 'Keystroke', keys: '', modifiers: [] },
      description: suggestion.description || ''
    };
    editingIndex = null; // New mapping

    showMidiLearn = false;
    showMappingEditor = true;
  }

  /**
   * Open mapping editor for new mapping
   */
  function addNewMapping() {
    editingMapping = {
      trigger: { type: 'Note', note: 60 },
      action: { type: 'Keystroke', keys: '', modifiers: [] },
      description: ''
    };
    editingIndex = null;
    showMappingEditor = true;
  }

  /**
   * Edit existing mapping
   */
  function editMapping(event) {
    const index = event.detail.index;
    const mappingArray = isGlobal ? globalMappings : mappings;
    editingMapping = { ...mappingArray[index] };
    editingIndex = index;
    showMappingEditor = true;
  }

  /**
   * Save edited/new mapping
   */
  async function saveMappingEdit() {
    if (!editingMapping) return;

    const mappingArray = isGlobal ? globalMappings : mappings;

    if (editingIndex !== null) {
      // Update existing
      mappingArray[editingIndex] = editingMapping;
    } else {
      // Add new
      mappingArray.push(editingMapping);
    }

    if (isGlobal) {
      globalMappings = [...mappingArray];
    } else {
      mappings = [...mappingArray];
    }

    await saveConfig();
    closeMappingEditor();
  }

  /**
   * Delete mapping
   */
  async function deleteMapping(event) {
    const index = event.detail.index;
    const mappingArray = isGlobal ? globalMappings : mappings;
    mappingArray.splice(index, 1);

    if (isGlobal) {
      globalMappings = [...mappingArray];
    } else {
      mappings = [...mappingArray];
    }

    await saveConfig();
  }

  /**
   * Close mapping editor
   */
  function closeMappingEditor() {
    showMappingEditor = false;
    editingMapping = null;
    editingIndex = null;
  }

  /**
   * Change selected mode
   */
  function changeMode(event) {
    selectedMode = parseInt(event.target.value);
    if (config && config.modes && config.modes[selectedMode]) {
      mappings = config.modes[selectedMode].mappings || [];
    }
  }

  /**
   * Toggle global/mode-specific mappings
   */
  function toggleGlobal() {
    isGlobal = !isGlobal;
  }

  /**
   * Reactive: Update mappings when mode or config changes
   */
  $: if ($configStore.config && !isGlobal) {
    const cfg = $configStore.config;
    if (cfg.modes && cfg.modes[selectedMode]) {
      mappings = cfg.modes[selectedMode].mappings || [];
    }
  }

  $: if ($configStore.config && isGlobal) {
    globalMappings = $configStore.config.global_mappings || [];
  }

  $: currentMappings = isGlobal ? globalMappings : mappings;
</script>

<div class="view">
  <header class="view-header">
    <div class="header-content">
      <div>
        <h2>Mapping Editor</h2>
        <p class="subtitle">Configure MIDI event to action mappings</p>
      </div>
      <div class="header-actions">
        {#if loading}
          <div class="loading-indicator">
            <span class="spinner"></span>
            Saving...
          </div>
        {/if}
        <button class="btn-primary" on:click={openMidiLearn}>
          🎹 MIDI Learn
        </button>
        <button class="btn-secondary" on:click={addNewMapping}>
          + Add Mapping
        </button>
      </div>
    </div>
  </header>

  <div class="content">
    {#if error}
      <div class="error-banner">
        <span class="error-icon">⚠️</span>
        <span>{error}</span>
        <button class="dismiss-btn" on:click={() => { error = null; appStore.clearError(); }}>
          ✕
        </button>
      </div>
    {/if}

    <div class="controls">
      <div class="mode-selector">
        <label>
          <input type="checkbox" checked={isGlobal} on:change={toggleGlobal} />
          Global Mappings (all modes)
        </label>
        {#if !isGlobal}
          <select value={selectedMode} on:change={changeMode}>
            {#if config && config.modes}
              {#each config.modes as mode, i}
                <option value={i}>{mode.name || `Mode ${i}`}</option>
              {/each}
            {/if}
          </select>
        {/if}
      </div>
    </div>

    <MappingList
      mappings={currentMappings}
      on:editMapping={editMapping}
      on:deleteMapping={deleteMapping}
    />
  </div>
</div>

{#if showMidiLearn}
  <MidiLearnDialog
    on:complete={handleMidiLearnComplete}
    on:cancel={() => showMidiLearn = false}
  />
{/if}

{#if showMappingEditor}
  <div class="modal-overlay" on:click={closeMappingEditor}>
    <div class="modal-dialog" on:click|stopPropagation>
      <div class="modal-header">
        <h3>{editingIndex !== null ? 'Edit' : 'New'} Mapping</h3>
        <button class="close-btn" on:click={closeMappingEditor}>✕</button>
      </div>
      <div class="modal-body">
        <div class="section">
          <h4>Trigger</h4>
          <TriggerSelector bind:trigger={editingMapping.trigger} />
        </div>

        <div class="section">
          <h4>Action</h4>
          <ActionSelector bind:action={editingMapping.action} />
        </div>

        <div class="section">
          <label>
            Description (optional)
            <input
              type="text"
              bind:value={editingMapping.description}
              placeholder="Brief description of this mapping"
            />
          </label>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn-secondary" on:click={closeMappingEditor}>Cancel</button>
        <button class="btn-primary" on:click={saveMappingEdit}>Save Mapping</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .view {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .view-header {
    padding: 2rem 2.5rem 1.5rem;
    border-bottom: 1px solid #333;
    background: #1e1e1e;
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .header-actions {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .view-header h2 {
    margin: 0 0 0.5rem;
    font-size: 1.75rem;
    color: #e0e0e0;
  }

  .subtitle {
    margin: 0;
    color: #999;
    font-size: 0.95rem;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .controls {
    padding: 1.5rem 2.5rem;
    border-bottom: 1px solid #333;
    background: #252525;
  }

  .mode-selector {
    display: flex;
    gap: 1.5rem;
    align-items: center;
  }

  .mode-selector label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #e0e0e0;
  }

  .mode-selector select {
    padding: 0.5rem 1rem;
    background: #333;
    border: 1px solid #444;
    border-radius: 4px;
    color: #e0e0e0;
    font-size: 0.95rem;
  }

  .loading-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #4a9eff;
    font-size: 0.9rem;
  }

  .spinner {
    display: inline-block;
    width: 16px;
    height: 16px;
    border: 2px solid #4a9eff;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .error-banner {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem 1.5rem;
    margin: 1.5rem 2.5rem;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 8px;
    color: #ef4444;
  }

  .error-icon {
    font-size: 1.25rem;
  }

  .dismiss-btn {
    margin-left: auto;
    background: none;
    border: none;
    color: #ef4444;
    cursor: pointer;
    font-size: 1.25rem;
    padding: 0.25rem 0.5rem;
    opacity: 0.6;
    transition: opacity 0.2s;
  }

  .dismiss-btn:hover {
    opacity: 1;
  }

  .btn-primary,
  .btn-secondary {
    padding: 0.6rem 1.25rem;
    border-radius: 6px;
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-primary {
    background: #4a9eff;
    border: none;
    color: white;
  }

  .btn-primary:hover {
    background: #3a8eef;
  }

  .btn-secondary {
    background: #333;
    border: 1px solid #444;
    color: #e0e0e0;
  }

  .btn-secondary:hover {
    background: #3a3a3a;
  }

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.75);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-dialog {
    background: #2a2a2a;
    border-radius: 12px;
    width: 90%;
    max-width: 700px;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 2rem;
    border-bottom: 1px solid #333;
  }

  .modal-header h3 {
    margin: 0;
    font-size: 1.5rem;
    color: #e0e0e0;
  }

  .close-btn {
    background: none;
    border: none;
    color: #999;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-btn:hover {
    color: #e0e0e0;
  }

  .modal-body {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
  }

  .section {
    margin-bottom: 2rem;
  }

  .section h4 {
    margin: 0 0 1rem;
    color: #4a9eff;
    font-size: 1.1rem;
  }

  .section label {
    display: block;
    margin-bottom: 0.5rem;
    color: #e0e0e0;
    font-size: 0.95rem;
  }

  .section input[type="text"] {
    width: 100%;
    padding: 0.75rem;
    background: #333;
    border: 1px solid #444;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 0.95rem;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    padding: 1.5rem 2rem;
    border-top: 1px solid #333;
  }
</style>
