<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { createEventDispatcher } from 'svelte';
  import ActionSelector from './ActionSelector.svelte';

  /**
   * Props
   */
  export let action = null; // Current Conditional action config
  export let readonly = false;
  export let availableModes = []; // For ModeIs condition

  const dispatch = createEventDispatcher();

  /**
   * Condition type definitions
   */
  const conditionTypes = [
    { value: 'Always', label: 'Always', description: 'Always execute' },
    { value: 'Never', label: 'Never', description: 'Never execute (disabled)' },
    { value: 'TimeRange', label: 'Time Range', description: 'Within time window' },
    { value: 'DayOfWeek', label: 'Day of Week', description: 'Specific days only' },
    { value: 'AppRunning', label: 'App Running', description: 'Application is running' },
    { value: 'AppFrontmost', label: 'App Frontmost', description: 'Application has focus' },
    { value: 'ModeIs', label: 'Mode Is', description: 'Current mode matches' },
    { value: 'And', label: 'AND (All)', description: 'All sub-conditions true' },
    { value: 'Or', label: 'OR (Any)', description: 'Any sub-condition true' },
    { value: 'Not', label: 'NOT (Inverse)', description: 'Invert condition result' },
  ];

  /**
   * Days of week (Monday=1, Sunday=7)
   */
  const daysOfWeek = [
    { value: 1, label: 'Monday' },
    { value: 2, label: 'Tuesday' },
    { value: 3, label: 'Wednesday' },
    { value: 4, label: 'Thursday' },
    { value: 5, label: 'Friday' },
    { value: 6, label: 'Saturday' },
    { value: 7, label: 'Sunday' },
  ];

  /**
   * Local state
   */
  let config = action || createDefaultConfig();
  let conditionType = getConditionType(config.condition);
  let selectedDays = getDaySelection(config.condition);
  let hasElseAction = config.else_action !== null && config.else_action !== undefined;

  // Logical operator state
  let subConditionTypes = []; // Track types of sub-conditions for And/Or
  let notConditionType = 'Always'; // Track type of Not condition

  // Initialize logical operator state
  $: {
    if (conditionType === 'And' && config.condition?.And?.conditions) {
      subConditionTypes = config.condition.And.conditions.map(getConditionType);
    } else if (conditionType === 'Or' && config.condition?.Or?.conditions) {
      subConditionTypes = config.condition.Or.conditions.map(getConditionType);
    } else if (conditionType === 'Not' && config.condition?.Not?.condition) {
      notConditionType = getConditionType(config.condition.Not.condition);
    }
  }

  /**
   * Get condition type from condition object
   */
  function getConditionType(condition) {
    if (typeof condition === 'string') {
      return condition;
    }
    if (typeof condition === 'object' && condition !== null) {
      return Object.keys(condition)[0];
    }
    return 'Always';
  }

  /**
   * Get selected days from DayOfWeek condition
   */
  function getDaySelection(condition) {
    if (typeof condition === 'object' && condition?.DayOfWeek) {
      return condition.DayOfWeek.days || [];
    }
    return [];
  }

  /**
   * Create default config
   */
  function createDefaultConfig() {
    return {
      type: 'Conditional',
      condition: 'Always',
      then_action: { type: 'Keystroke', keys: '', modifiers: [] },
      else_action: null,
    };
  }

  /**
   * Create condition object from type and parameters
   */
  function createConditionObject(type) {
    switch (type) {
      case 'Always':
      case 'Never':
        return type;

      case 'TimeRange':
        return {
          TimeRange: {
            start: '09:00',
            end: '17:00',
          },
        };

      case 'DayOfWeek':
        return {
          DayOfWeek: {
            days: selectedDays.length > 0 ? selectedDays : [1, 2, 3, 4, 5], // Default to weekdays
          },
        };

      case 'AppRunning':
        return {
          AppRunning: {
            app_name: '',
          },
        };

      case 'AppFrontmost':
        return {
          AppFrontmost: {
            app_name: '',
          },
        };

      case 'ModeIs':
        return {
          ModeIs: {
            mode: availableModes[0] || 'Default',
          },
        };

      case 'And':
        return {
          And: {
            conditions: ['Always'],
          },
        };

      case 'Or':
        return {
          Or: {
            conditions: ['Always'],
          },
        };

      case 'Not':
        return {
          Not: {
            condition: 'Always',
          },
        };

      default:
        return 'Always';
    }
  }

  /**
   * Handle condition type change
   */
  function handleConditionTypeChange() {
    config.condition = createConditionObject(conditionType);
    selectedDays = getDaySelection(config.condition);
    emitChange();
  }

  /**
   * Toggle day selection
   */
  function toggleDay(day) {
    if (selectedDays.includes(day)) {
      selectedDays = selectedDays.filter(d => d !== day);
    } else {
      selectedDays = [...selectedDays, day].sort();
    }

    if (config.condition?.DayOfWeek) {
      config.condition.DayOfWeek.days = selectedDays;
      emitChange();
    }
  }

  /**
   * Handle TimeRange field change
   */
  function handleTimeRangeChange() {
    emitChange();
  }

  /**
   * Handle AppRunning/AppFrontmost field change
   */
  function handleAppNameChange() {
    emitChange();
  }

  /**
   * Handle ModeIs field change
   */
  function handleModeChange() {
    emitChange();
  }

  /**
   * Toggle else action
   */
  function toggleElseAction() {
    if (hasElseAction) {
      config.else_action = null;
    } else {
      config.else_action = { type: 'Keystroke', keys: '', modifiers: [] };
    }
    hasElseAction = !hasElseAction;
    emitChange();
  }

  /**
   * Handle then_action change
   */
  function handleThenActionChange(event) {
    config.then_action = event.detail;
    emitChange();
  }

  /**
   * Handle else_action change
   */
  function handleElseActionChange(event) {
    config.else_action = event.detail;
    emitChange();
  }

  /**
   * Emit change event
   */
  function emitChange() {
    dispatch('change', { config });
  }

  /**
   * Add a sub-condition to And/Or operator
   */
  function addSubCondition(operatorType) {
    const newCondition = 'Always';

    if (operatorType === 'And') {
      config.condition.And.conditions = [...config.condition.And.conditions, newCondition];
      subConditionTypes = [...subConditionTypes, 'Always'];
    } else if (operatorType === 'Or') {
      config.condition.Or.conditions = [...config.condition.Or.conditions, newCondition];
      subConditionTypes = [...subConditionTypes, 'Always'];
    }

    emitChange();
  }

  /**
   * Remove a sub-condition from And/Or operator
   */
  function removeSubCondition(operatorType, index) {
    if (operatorType === 'And') {
      config.condition.And.conditions = config.condition.And.conditions.filter((_, i) => i !== index);
      subConditionTypes = subConditionTypes.filter((_, i) => i !== index);
    } else if (operatorType === 'Or') {
      config.condition.Or.conditions = config.condition.Or.conditions.filter((_, i) => i !== index);
      subConditionTypes = subConditionTypes.filter((_, i) => i !== index);
    }

    emitChange();
  }

  /**
   * Update a sub-condition when its type changes
   */
  function updateSubCondition(operatorType, index) {
    const newType = subConditionTypes[index];
    const newCondition = createConditionObject(newType);

    if (operatorType === 'And') {
      config.condition.And.conditions[index] = newCondition;
    } else if (operatorType === 'Or') {
      config.condition.Or.conditions[index] = newCondition;
    }

    emitChange();
  }

  /**
   * Update the Not condition when its type changes
   */
  function updateNotCondition() {
    const newCondition = createConditionObject(notConditionType);
    config.condition.Not.condition = newCondition;
    emitChange();
  }
</script>

<div class="conditional-editor">
  <div class="section-header">
    <h4>Condition</h4>
  </div>

  <div class="form-group">
    <label for="condition-type">Condition Type</label>
    <select
      id="condition-type"
      bind:value={conditionType}
      on:change={handleConditionTypeChange}
      disabled={readonly}
    >
      {#each conditionTypes as type}
        <option value={type.value}>
          {type.label} - {type.description}
        </option>
      {/each}
    </select>
  </div>

  <!-- TimeRange Condition -->
  {#if conditionType === 'TimeRange'}
    <div class="condition-config">
      <div class="form-row">
        <div class="form-group">
          <label for="start-time">Start Time (24h)</label>
          <input
            id="start-time"
            type="time"
            bind:value={config.condition.TimeRange.start}
            on:input={handleTimeRangeChange}
            disabled={readonly}
          />
        </div>
        <div class="form-group">
          <label for="end-time">End Time (24h)</label>
          <input
            id="end-time"
            type="time"
            bind:value={config.condition.TimeRange.end}
            on:input={handleTimeRangeChange}
            disabled={readonly}
          />
        </div>
      </div>
      <small>Automatically handles time ranges that cross midnight</small>
    </div>
  {/if}

  <!-- DayOfWeek Condition -->
  {#if conditionType === 'DayOfWeek'}
    <div class="condition-config">
      <label>Active Days</label>
      <div class="day-grid">
        {#each daysOfWeek as day}
          <button
            class="day-btn"
            class:active={selectedDays.includes(day.value)}
            on:click={() => toggleDay(day.value)}
            disabled={readonly}
          >
            {day.label}
          </button>
        {/each}
      </div>
      <small>{selectedDays.length === 0 ? 'Select at least one day' : `Active on ${selectedDays.length} day(s)`}</small>
    </div>
  {/if}

  <!-- AppRunning Condition -->
  {#if conditionType === 'AppRunning'}
    <div class="condition-config">
      <div class="form-group">
        <label for="app-name">Application Name</label>
        <input
          id="app-name"
          type="text"
          bind:value={config.condition.AppRunning.app_name}
          on:input={handleAppNameChange}
          placeholder="e.g., Xcode, Chrome, Spotify"
          disabled={readonly}
        />
        <small>Uses pgrep (case-insensitive) - macOS and Linux only</small>
      </div>
    </div>
  {/if}

  <!-- AppFrontmost Condition -->
  {#if conditionType === 'AppFrontmost'}
    <div class="condition-config">
      <div class="form-group">
        <label for="app-frontmost">Application Name</label>
        <input
          id="app-frontmost"
          type="text"
          bind:value={config.condition.AppFrontmost.app_name}
          on:input={handleAppNameChange}
          placeholder="e.g., Xcode, Chrome, Spotify"
          disabled={readonly}
        />
        <small>Application must have focus - macOS only</small>
      </div>
    </div>
  {/if}

  <!-- ModeIs Condition -->
  {#if conditionType === 'ModeIs'}
    <div class="condition-config">
      <div class="form-group">
        <label for="mode-name">Mode Name</label>
        {#if availableModes.length > 0}
          <select
            id="mode-name"
            bind:value={config.condition.ModeIs.mode}
            on:change={handleModeChange}
            disabled={readonly}
          >
            {#each availableModes as mode}
              <option value={mode}>{mode}</option>
            {/each}
          </select>
        {:else}
          <input
            id="mode-name"
            type="text"
            bind:value={config.condition.ModeIs.mode}
            on:input={handleModeChange}
            placeholder="Mode name"
            disabled={readonly}
          />
          <small class="warning">No modes available. Enter mode name manually.</small>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Logical AND operator -->
  {#if conditionType === 'And'}
    <div class="condition-config">
      <div class="logical-operator-container">
        <div class="operator-header">
          <label>Sub-Conditions (All must be true)</label>
          {#if !readonly}
            <button class="btn btn-small btn-primary" on:click={() => addSubCondition('And')}>
              + Add Condition
            </button>
          {/if}
        </div>
        <div class="sub-conditions-list">
          {#each config.condition.And.conditions as subCond, index}
            <div class="sub-condition-item">
              <div class="sub-condition-header">
                <span class="sub-condition-label">Condition {index + 1}</span>
                {#if !readonly && config.condition.And.conditions.length > 1}
                  <button class="btn btn-small btn-danger" on:click={() => removeSubCondition('And', index)}>
                    Remove
                  </button>
                {/if}
              </div>
              <div class="sub-condition-content">
                <select
                  bind:value={subConditionTypes[index]}
                  on:change={() => updateSubCondition('And', index)}
                  disabled={readonly}
                  class="sub-condition-select"
                >
                  {#each conditionTypes.filter(t => t.value !== 'And' && t.value !== 'Or' && t.value !== 'Not') as type}
                    <option value={type.value}>
                      {type.label}
                    </option>
                  {/each}
                </select>
                <!-- Note: Full nested condition editing would require recursive component -->
                <small>For complex nested logic, use TOML editor</small>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {/if}

  <!-- Logical OR operator -->
  {#if conditionType === 'Or'}
    <div class="condition-config">
      <div class="logical-operator-container">
        <div class="operator-header">
          <label>Sub-Conditions (At least one must be true)</label>
          {#if !readonly}
            <button class="btn btn-small btn-primary" on:click={() => addSubCondition('Or')}>
              + Add Condition
            </button>
          {/if}
        </div>
        <div class="sub-conditions-list">
          {#each config.condition.Or.conditions as subCond, index}
            <div class="sub-condition-item">
              <div class="sub-condition-header">
                <span class="sub-condition-label">Condition {index + 1}</span>
                {#if !readonly && config.condition.Or.conditions.length > 1}
                  <button class="btn btn-small btn-danger" on:click={() => removeSubCondition('Or', index)}>
                    Remove
                  </button>
                {/if}
              </div>
              <div class="sub-condition-content">
                <select
                  bind:value={subConditionTypes[index]}
                  on:change={() => updateSubCondition('Or', index)}
                  disabled={readonly}
                  class="sub-condition-select"
                >
                  {#each conditionTypes.filter(t => t.value !== 'And' && t.value !== 'Or' && t.value !== 'Not') as type}
                    <option value={type.value}>
                      {type.label}
                    </option>
                  {/each}
                </select>
                <small>For complex nested logic, use TOML editor</small>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {/if}

  <!-- Logical NOT operator -->
  {#if conditionType === 'Not'}
    <div class="condition-config">
      <div class="logical-operator-container">
        <div class="operator-header">
          <label>Negate Condition (Make it opposite)</label>
        </div>
        <div class="sub-condition-item">
          <div class="sub-condition-content">
            <select
              bind:value={notConditionType}
              on:change={updateNotCondition}
              disabled={readonly}
              class="sub-condition-select"
            >
              {#each conditionTypes.filter(t => t.value !== 'And' && t.value !== 'Or' && t.value !== 'Not') as type}
                <option value={type.value}>
                  {type.label}
                </option>
              {/each}
            </select>
            <small>Inverts the result of the selected condition. For complex nested logic, use TOML editor</small>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <div class="divider"></div>

  <div class="section-header">
    <h4>Then Action (when condition is true)</h4>
  </div>

  <ActionSelector
    action={config.then_action}
    {readonly}
    {availableModes}
    on:change={handleThenActionChange}
  />

  <div class="divider"></div>

  <div class="else-toggle">
    <label>
      <input
        type="checkbox"
        bind:checked={hasElseAction}
        on:change={toggleElseAction}
        disabled={readonly}
      />
      Add else action (when condition is false)
    </label>
  </div>

  {#if hasElseAction}
    <div class="section-header">
      <h4>Else Action (when condition is false)</h4>
    </div>

    <ActionSelector
      action={config.else_action}
      {readonly}
      {availableModes}
      on:change={handleElseActionChange}
    />
  {/if}
</div>

<style>
  .conditional-editor {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    padding: 1rem;
    background: var(--bg-tertiary, #1a1a1a);
    border-radius: 8px;
    border: 1px solid var(--border-color, #444);
  }

  .section-header {
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border-color, #444);
  }

  .section-header h4 {
    margin: 0;
    font-size: 0.875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-secondary, #999);
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
    background: var(--bg-primary, #0a0a0a);
    border: 1px solid var(--border-color, #444);
    border-radius: 6px;
    color: var(--text-primary, #fff);
    font-size: 0.875rem;
    font-family: inherit;
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

  .form-group small.warning {
    color: var(--error-color, #ef4444);
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .condition-config {
    padding: 1rem;
    background: var(--bg-primary, #0a0a0a);
    border-radius: 6px;
    border: 1px solid var(--border-color, #444);
  }

  .day-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .day-btn {
    padding: 0.75rem;
    background: var(--bg-tertiary, #1a1a1a);
    border: 2px solid var(--border-color, #444);
    border-radius: 6px;
    color: var(--text-secondary, #999);
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .day-btn:hover:not(:disabled) {
    border-color: var(--accent-color, #3b82f6);
  }

  .day-btn.active {
    background: var(--accent-color, #3b82f6);
    border-color: var(--accent-color, #3b82f6);
    color: white;
  }

  .day-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .info-box {
    padding: 1rem;
    background: var(--bg-secondary, #141414);
    border-radius: 6px;
    border-left: 3px solid var(--accent-color, #3b82f6);
  }

  .info-box p {
    margin: 0.5rem 0;
    font-size: 0.875rem;
    color: var(--text-secondary, #999);
  }

  .info-box strong {
    color: var(--text-primary, #fff);
  }

  .divider {
    height: 1px;
    background: var(--border-color, #444);
  }

  .else-toggle {
    padding: 0.75rem 1rem;
    background: var(--bg-primary, #0a0a0a);
    border-radius: 6px;
    border: 1px solid var(--border-color, #444);
  }

  .else-toggle label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
    cursor: pointer;
  }

  .else-toggle input[type="checkbox"] {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .else-toggle input[type="checkbox"]:disabled {
    cursor: not-allowed;
  }

  /* Logical Operator Styles */
  .logical-operator-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .operator-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--border-color, #444);
  }

  .operator-header label {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
  }

  .btn {
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.75rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all 0.2s;
  }

  .btn-small {
    padding: 0.375rem 0.75rem;
    font-size: 0.75rem;
  }

  .btn-primary {
    background: var(--accent-color, #3b82f6);
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--accent-hover, #2563eb);
  }

  .btn-danger {
    background: var(--error-color, #ef4444);
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background: #dc2626;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .sub-conditions-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .sub-condition-item {
    padding: 1rem;
    background: var(--bg-secondary, #141414);
    border-radius: 6px;
    border: 1px solid var(--border-color, #444);
  }

  .sub-condition-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  .sub-condition-label {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--accent-color, #3b82f6);
  }

  .sub-condition-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .sub-condition-select {
    padding: 0.75rem;
    background: var(--bg-primary, #0a0a0a);
    border: 1px solid var(--border-color, #444);
    border-radius: 6px;
    color: var(--text-primary, #fff);
    font-size: 0.875rem;
    font-family: inherit;
  }

  .sub-condition-select:focus {
    outline: none;
    border-color: var(--accent-color, #3b82f6);
  }

  .sub-condition-select:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .sub-condition-content small {
    font-size: 0.75rem;
    color: var(--text-secondary, #999);
    font-style: italic;
  }
</style>
