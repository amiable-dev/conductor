<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { createEventDispatcher } from 'svelte';

  /**
   * Props
   */
  export let modes = []; // Array of mode objects: { name, color, mappings }
  export let selectedModeIndex = 0; // Currently selected mode
  export let readonly = false; // Disable editing

  const dispatch = createEventDispatcher();

  /**
   * Color presets for quick selection
   */
  const colorPresets = [
    { name: 'Blue', value: 'blue' },
    { name: 'Green', value: 'green' },
    { name: 'Purple', value: 'purple' },
    { name: 'Red', value: 'red' },
    { name: 'Yellow', value: 'yellow' },
    { name: 'Orange', value: 'orange' },
    { name: 'Pink', value: 'pink' },
    { name: 'Cyan', value: 'cyan' },
  ];

  /**
   * Local state
   */
  let editingMode = null; // Mode being edited in modal
  let showAddModal = false;
  let showEditModal = false;
  let showDeleteConfirm = false;
  let modeToDelete = null;

  /**
   * Reactive: Current mode
   */
  $: currentMode = modes[selectedModeIndex] || null;

  /**
   * Select a mode
   */
  function selectMode(index) {
    selectedModeIndex = index;
    dispatch('modeSelected', { index, mode: modes[index] });
  }

  /**
   * Open add mode modal
   */
  function openAddModal() {
    editingMode = {
      name: '',
      color: 'blue',
      mappings: [],
    };
    showAddModal = true;
  }

  /**
   * Open edit mode modal
   */
  function openEditModal(mode) {
    editingMode = { ...mode };
    showEditModal = true;
  }

  /**
   * Save new mode
   */
  function saveNewMode() {
    if (!editingMode.name.trim()) {
      alert('Mode name is required');
      return;
    }

    dispatch('modeAdded', editingMode);
    showAddModal = false;
    editingMode = null;
  }

  /**
   * Save edited mode
   */
  function saveEditedMode() {
    if (!editingMode.name.trim()) {
      alert('Mode name is required');
      return;
    }

    dispatch('modeUpdated', { index: selectedModeIndex, mode: editingMode });
    showEditModal = false;
    editingMode = null;
  }

  /**
   * Delete mode with confirmation
   */
  function confirmDelete(mode, index) {
    modeToDelete = { mode, index };
    showDeleteConfirm = true;
  }

  function executeDelete() {
    if (modeToDelete) {
      dispatch('modeDeleted', { index: modeToDelete.index });

      // Adjust selection if needed
      if (selectedModeIndex >= modes.length - 1) {
        selectedModeIndex = Math.max(0, modes.length - 2);
      }
    }
    showDeleteConfirm = false;
    modeToDelete = null;
  }

  /**
   * Cancel modals
   */
  function cancelModal() {
    showAddModal = false;
    showEditModal = false;
    showDeleteConfirm = false;
    editingMode = null;
    modeToDelete = null;
  }
</script>

<div class="mode-editor">
  <div class="header">
    <h2>Modes</h2>
    {#if !readonly}
      <button class="btn btn-primary" on:click={openAddModal}>
        + Add Mode
      </button>
    {/if}
  </div>

  <div class="mode-list">
    {#each modes as mode, index}
      <div
        class="mode-item"
        class:active={index === selectedModeIndex}
        on:click={() => selectMode(index)}
      >
        <div class="mode-indicator" style="background-color: {mode.color || '#666'}"></div>
        <div class="mode-info">
          <div class="mode-name">{mode.name}</div>
          <div class="mode-stats">{mode.mappings?.length || 0} mappings</div>
        </div>
        {#if !readonly}
          <div class="mode-actions">
            <button
              class="btn-icon"
              on:click|stopPropagation={() => openEditModal(mode)}
              title="Edit mode"
            >
              ✏️
            </button>
            <button
              class="btn-icon"
              on:click|stopPropagation={() => confirmDelete(mode, index)}
              title="Delete mode"
            >
              🗑️
            </button>
          </div>
        {/if}
      </div>
    {/each}

    {#if modes.length === 0}
      <div class="empty-state">
        <p>No modes configured</p>
        {#if !readonly}
          <button class="btn btn-secondary" on:click={openAddModal}>
            Create First Mode
          </button>
        {/if}
      </div>
    {/if}
  </div>
</div>

<!-- Add Mode Modal -->
{#if showAddModal && editingMode}
  <div class="modal-overlay" on:click={cancelModal}>
    <div class="modal" on:click|stopPropagation>
      <div class="modal-header">
        <h3>Add New Mode</h3>
        <button class="close-btn" on:click={cancelModal}>×</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label for="mode-name">Mode Name</label>
          <input
            id="mode-name"
            type="text"
            bind:value={editingMode.name}
            placeholder="e.g., Default, Development, Media"
          />
        </div>
        <div class="form-group">
          <label>Color</label>
          <div class="color-picker">
            {#each colorPresets as preset}
              <button
                class="color-btn"
                class:active={editingMode.color === preset.value}
                style="background-color: {preset.value}"
                on:click={() => editingMode.color = preset.value}
                title={preset.name}
              >
                {#if editingMode.color === preset.value}
                  <span class="check">✓</span>
                {/if}
              </button>
            {/each}
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" on:click={cancelModal}>Cancel</button>
        <button class="btn btn-primary" on:click={saveNewMode}>Add Mode</button>
      </div>
    </div>
  </div>
{/if}

<!-- Edit Mode Modal -->
{#if showEditModal && editingMode}
  <div class="modal-overlay" on:click={cancelModal}>
    <div class="modal" on:click|stopPropagation>
      <div class="modal-header">
        <h3>Edit Mode</h3>
        <button class="close-btn" on:click={cancelModal}>×</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label for="edit-mode-name">Mode Name</label>
          <input
            id="edit-mode-name"
            type="text"
            bind:value={editingMode.name}
            placeholder="e.g., Default, Development, Media"
          />
        </div>
        <div class="form-group">
          <label>Color</label>
          <div class="color-picker">
            {#each colorPresets as preset}
              <button
                class="color-btn"
                class:active={editingMode.color === preset.value}
                style="background-color: {preset.value}"
                on:click={() => editingMode.color = preset.value}
                title={preset.name}
              >
                {#if editingMode.color === preset.value}
                  <span class="check">✓</span>
                {/if}
              </button>
            {/each}
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" on:click={cancelModal}>Cancel</button>
        <button class="btn btn-primary" on:click={saveEditedMode}>Save Changes</button>
      </div>
    </div>
  </div>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteConfirm && modeToDelete}
  <div class="modal-overlay" on:click={cancelModal}>
    <div class="modal modal-small" on:click|stopPropagation>
      <div class="modal-header">
        <h3>Delete Mode?</h3>
        <button class="close-btn" on:click={cancelModal}>×</button>
      </div>
      <div class="modal-body">
        <p>Are you sure you want to delete mode <strong>{modeToDelete.mode.name}</strong>?</p>
        <p class="warning-text">This will delete all {modeToDelete.mode.mappings?.length || 0} mappings in this mode.</p>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" on:click={cancelModal}>Cancel</button>
        <button class="btn btn-danger" on:click={executeDelete}>Delete Mode</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .mode-editor {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-secondary, #2a2a2a);
    border-radius: 8px;
    overflow: hidden;
  }

  .header {
    padding: 1rem 1.5rem;
    border-bottom: 1px solid var(--border-color, #444);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .header h2 {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
  }

  .mode-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .mode-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    margin-bottom: 0.5rem;
    background: var(--bg-tertiary, #1a1a1a);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
    border: 2px solid transparent;
  }

  .mode-item:hover {
    background: var(--bg-hover, #333);
  }

  .mode-item.active {
    border-color: var(--accent-color, #3b82f6);
    background: var(--bg-hover, #333);
  }

  .mode-indicator {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .mode-info {
    flex: 1;
    min-width: 0;
  }

  .mode-name {
    font-weight: 600;
    color: var(--text-primary, #fff);
    margin-bottom: 0.25rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .mode-stats {
    font-size: 0.875rem;
    color: var(--text-secondary, #999);
  }

  .mode-actions {
    display: flex;
    gap: 0.25rem;
  }

  .btn-icon {
    background: none;
    border: none;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
    font-size: 1rem;
    border-radius: 4px;
    transition: background 0.2s;
  }

  .btn-icon:hover {
    background: var(--bg-primary, #0a0a0a);
  }

  .empty-state {
    text-align: center;
    padding: 3rem 1rem;
  }

  .empty-state p {
    color: var(--text-secondary, #999);
    margin-bottom: 1rem;
  }

  /* Buttons */
  .btn {
    padding: 0.5rem 1rem;
    border-radius: 6px;
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

  .btn-danger {
    background: var(--error-color, #ef4444);
    color: white;
  }

  .btn-danger:hover {
    background: #dc2626;
  }

  /* Modal */
  .modal-overlay {
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

  .modal {
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

  .modal-small {
    max-width: 400px;
  }

  .modal-header {
    padding: 1.5rem;
    border-bottom: 1px solid var(--border-color, #444);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .modal-header h3 {
    margin: 0;
    font-size: 1.125rem;
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

  .modal-body {
    padding: 1.5rem;
    overflow-y: auto;
  }

  .modal-footer {
    padding: 1rem 1.5rem;
    border-top: 1px solid var(--border-color, #444);
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
  }

  /* Form */
  .form-group {
    margin-bottom: 1.5rem;
  }

  .form-group label {
    display: block;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
    margin-bottom: 0.5rem;
  }

  .form-group input {
    width: 100%;
    padding: 0.75rem;
    background: var(--bg-tertiary, #1a1a1a);
    border: 1px solid var(--border-color, #444);
    border-radius: 6px;
    color: var(--text-primary, #fff);
    font-size: 1rem;
  }

  .form-group input:focus {
    outline: none;
    border-color: var(--accent-color, #3b82f6);
  }

  .color-picker {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(48px, 1fr));
    gap: 0.5rem;
  }

  .color-btn {
    width: 48px;
    height: 48px;
    border-radius: 8px;
    border: 2px solid transparent;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
  }

  .color-btn:hover {
    transform: scale(1.1);
  }

  .color-btn.active {
    border-color: var(--text-primary, #fff);
    box-shadow: 0 0 0 2px var(--bg-secondary, #2a2a2a);
  }

  .color-btn .check {
    color: white;
    font-weight: bold;
    font-size: 1.25rem;
    text-shadow: 0 1px 3px rgba(0, 0, 0, 0.5);
  }

  .warning-text {
    color: var(--error-color, #ef4444);
    font-size: 0.875rem;
  }
</style>
