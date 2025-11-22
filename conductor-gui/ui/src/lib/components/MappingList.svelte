<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { createEventDispatcher } from 'svelte';

  /**
   * Props
   */
  export let mappings = []; // Array of mapping objects: { trigger, action, description }
  export let readonly = false; // Disable editing

  const dispatch = createEventDispatcher();

  /**
   * Local state
   */
  let selectedMappingIndex = null;
  let showDeleteConfirm = false;
  let mappingToDelete = null;

  /**
   * Format trigger for display
   */
  function formatTrigger(trigger) {
    if (!trigger || !trigger.type) return 'Unknown';

    switch (trigger.type) {
      case 'Note':
        return `Note ${trigger.note}${trigger.velocity_min ? ` (vel ≥ ${trigger.velocity_min})` : ''}`;

      case 'VelocityRange':
        const softMax = trigger.soft_max || 40;
        const mediumMax = trigger.medium_max || 80;
        return `Note ${trigger.note} (Soft: 0-${softMax}, Med: ${softMax + 1}-${mediumMax}, Hard: ${mediumMax + 1}-127)`;

      case 'LongPress':
        return `Long Press Note ${trigger.note} (${trigger.duration_ms || 2000}ms)`;

      case 'DoubleTap':
        return `Double Tap Note ${trigger.note} (${trigger.timeout_ms || 300}ms)`;

      case 'NoteChord':
        return `Chord: Notes ${trigger.notes?.join(', ') || '?'}`;

      case 'EncoderTurn':
        return `Encoder CC ${trigger.cc}${trigger.direction ? ` (${trigger.direction})` : ''}`;

      case 'CC':
        return `CC ${trigger.cc}${trigger.value_min ? ` (≥ ${trigger.value_min})` : ''}`;

      case 'Aftertouch':
        return `Aftertouch${trigger.pressure_min ? ` (≥ ${trigger.pressure_min})` : ''}`;

      case 'PitchBend':
        return `Pitch Bend${trigger.value_min != null ? ` (${trigger.value_min}-${trigger.value_max || 16383})` : ''}`;

      default:
        return trigger.type;
    }
  }

  /**
   * Format action for display
   */
  function formatAction(action) {
    if (!action || !action.type) return 'Unknown';

    switch (action.type) {
      case 'Keystroke':
        const mods = action.modifiers?.length > 0 ? action.modifiers.join('+') + '+' : '';
        return `${mods}${action.keys}`;

      case 'Text':
        return `Type: "${action.text?.substring(0, 20)}${action.text?.length > 20 ? '...' : ''}"`;

      case 'Launch':
        return `Launch: ${action.app}`;

      case 'Shell':
        return `Shell: ${action.command?.substring(0, 30)}${action.command?.length > 30 ? '...' : ''}`;

      case 'VolumeControl':
        return `Volume: ${action.action || 'control'}`;

      case 'ModeChange':
        return `Switch to mode: ${action.mode}`;

      case 'Sequence':
        return `Sequence (${action.actions?.length || 0} actions)`;

      case 'Delay':
        return `Delay ${action.duration_ms}ms`;

      case 'MouseClick':
        return `Mouse: ${action.button || 'left'} ${action.clicks || 1}x`;

      case 'Repeat':
        return `Repeat ${action.count}x`;

      case 'Conditional':
        return `Conditional (${action.condition})`;

      default:
        return action.type;
    }
  }

  /**
   * Get trigger type badge color
   */
  function getTriggerBadgeColor(type) {
    const colors = {
      Note: '#3b82f6',
      VelocityRange: '#8b5cf6',
      LongPress: '#f59e0b',
      DoubleTap: '#ec4899',
      NoteChord: '#10b981',
      EncoderTurn: '#06b6d4',
      CC: '#6366f1',
      Aftertouch: '#f97316',
      PitchBend: '#14b8a6',
    };
    return colors[type] || '#6b7280';
  }

  /**
   * Select a mapping
   */
  function selectMapping(index) {
    selectedMappingIndex = index;
    dispatch('mappingSelected', { index, mapping: mappings[index] });
  }

  /**
   * Add new mapping
   */
  function addMapping() {
    dispatch('addMapping');
  }

  /**
   * Edit mapping
   */
  function editMapping(mapping, index) {
    dispatch('editMapping', { index, mapping });
  }

  /**
   * Delete mapping with confirmation
   */
  function confirmDelete(mapping, index) {
    mappingToDelete = { mapping, index };
    showDeleteConfirm = true;
  }

  function executeDelete() {
    if (mappingToDelete) {
      dispatch('deleteMapping', { index: mappingToDelete.index });
    }
    showDeleteConfirm = false;
    mappingToDelete = null;
  }

  function cancelDelete() {
    showDeleteConfirm = false;
    mappingToDelete = null;
  }

  /**
   * Duplicate mapping
   */
  function duplicateMapping(mapping, index) {
    dispatch('duplicateMapping', { index, mapping });
  }
</script>

<div class="mapping-list">
  <div class="header">
    <h2>Mappings</h2>
    {#if !readonly}
      <button class="btn btn-primary" on:click={addMapping}>
        + Add Mapping
      </button>
    {/if}
  </div>

  <div class="list-container">
    {#each mappings as mapping, index}
      <div
        class="mapping-item"
        class:selected={index === selectedMappingIndex}
        on:click={() => selectMapping(index)}
      >
        <div class="mapping-content">
          <div class="mapping-header">
            <span
              class="trigger-badge"
              style="background-color: {getTriggerBadgeColor(mapping.trigger?.type)}"
            >
              {mapping.trigger?.type || 'Unknown'}
            </span>
            {#if mapping.description}
              <span class="description">{mapping.description}</span>
            {/if}
          </div>

          <div class="mapping-details">
            <div class="detail-row">
              <span class="label">Trigger:</span>
              <span class="value">{formatTrigger(mapping.trigger)}</span>
            </div>
            <div class="detail-row">
              <span class="label">Action:</span>
              <span class="value action-value">{formatAction(mapping.action)}</span>
            </div>
          </div>
        </div>

        {#if !readonly}
          <div class="mapping-actions">
            <button
              class="btn-icon"
              on:click|stopPropagation={() => editMapping(mapping, index)}
              title="Edit mapping"
            >
              ✏️
            </button>
            <button
              class="btn-icon"
              on:click|stopPropagation={() => duplicateMapping(mapping, index)}
              title="Duplicate mapping"
            >
              📋
            </button>
            <button
              class="btn-icon"
              on:click|stopPropagation={() => confirmDelete(mapping, index)}
              title="Delete mapping"
            >
              🗑️
            </button>
          </div>
        {/if}
      </div>
    {/each}

    {#if mappings.length === 0}
      <div class="empty-state">
        <div class="empty-icon">🎹</div>
        <p>No mappings in this mode</p>
        {#if !readonly}
          <button class="btn btn-secondary" on:click={addMapping}>
            Create First Mapping
          </button>
        {/if}
      </div>
    {/if}
  </div>
</div>

<!-- Delete Confirmation Modal -->
{#if showDeleteConfirm && mappingToDelete}
  <div class="modal-overlay" on:click={cancelDelete}>
    <div class="modal modal-small" on:click|stopPropagation>
      <div class="modal-header">
        <h3>Delete Mapping?</h3>
        <button class="close-btn" on:click={cancelDelete}>×</button>
      </div>
      <div class="modal-body">
        <p>Are you sure you want to delete this mapping?</p>
        <div class="mapping-preview">
          <div class="preview-row">
            <strong>Trigger:</strong> {formatTrigger(mappingToDelete.mapping.trigger)}
          </div>
          <div class="preview-row">
            <strong>Action:</strong> {formatAction(mappingToDelete.mapping.action)}
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" on:click={cancelDelete}>Cancel</button>
        <button class="btn btn-danger" on:click={executeDelete}>Delete Mapping</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .mapping-list {
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

  .list-container {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .mapping-item {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 1rem;
    margin-bottom: 0.5rem;
    background: var(--bg-tertiary, #1a1a1a);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
    border: 2px solid transparent;
  }

  .mapping-item:hover {
    background: var(--bg-hover, #333);
  }

  .mapping-item.selected {
    border-color: var(--accent-color, #3b82f6);
    background: var(--bg-hover, #333);
  }

  .mapping-content {
    flex: 1;
    min-width: 0;
  }

  .mapping-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .trigger-badge {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    color: white;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .description {
    font-size: 0.875rem;
    color: var(--text-secondary, #999);
    font-style: italic;
  }

  .mapping-details {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .detail-row {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    font-size: 0.875rem;
  }

  .detail-row .label {
    color: var(--text-secondary, #999);
    min-width: 60px;
    flex-shrink: 0;
  }

  .detail-row .value {
    color: var(--text-primary, #fff);
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    word-break: break-word;
  }

  .action-value {
    color: var(--accent-color, #3b82f6);
  }

  .mapping-actions {
    display: flex;
    flex-direction: column;
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
    padding: 4rem 1rem;
  }

  .empty-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
    opacity: 0.5;
  }

  .empty-state p {
    color: var(--text-secondary, #999);
    margin-bottom: 1.5rem;
    font-size: 1.125rem;
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

  .mapping-preview {
    background: var(--bg-tertiary, #1a1a1a);
    padding: 1rem;
    border-radius: 6px;
    margin-top: 1rem;
  }

  .preview-row {
    font-size: 0.875rem;
    margin-bottom: 0.5rem;
    color: var(--text-primary, #fff);
  }

  .preview-row:last-child {
    margin-bottom: 0;
  }

  .preview-row strong {
    color: var(--text-secondary, #999);
    display: inline-block;
    min-width: 60px;
  }
</style>
