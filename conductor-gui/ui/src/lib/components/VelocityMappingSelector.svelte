<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { createEventDispatcher } from 'svelte';

  /**
   * Props
   */
  export let velocityMapping = { Fixed: { velocity: 100 } }; // Current velocity mapping config
  export let readonly = false; // Readonly mode
  export let label = "Velocity Mapping"; // Label text

  const dispatch = createEventDispatcher();

  // Reactive mode tracking
  $: mode = getMode(velocityMapping);
  $: curveType = velocityMapping?.Curve?.curve_type || 'Exponential';
  $: intensity = velocityMapping?.Curve?.intensity ?? 0.5;
  $: minVelocity = velocityMapping?.Linear?.min ?? 0;
  $: maxVelocity = velocityMapping?.Linear?.max ?? 127;
  $: fixedVelocity = velocityMapping?.Fixed?.velocity ?? 100;

  /**
   * Get current mode from velocity mapping object
   */
  function getMode(mapping) {
    if (!mapping) return 'Fixed';
    if ('Fixed' in mapping) return 'Fixed';
    if ('PassThrough' in mapping) return 'PassThrough';
    if ('Linear' in mapping) return 'Linear';
    if ('Curve' in mapping) return 'Curve';
    return 'Fixed';
  }

  /**
   * Handle mode change
   */
  function handleModeChange(event) {
    const newMode = event.target.value;

    switch (newMode) {
      case 'Fixed':
        velocityMapping = { Fixed: { velocity: fixedVelocity } };
        break;
      case 'PassThrough':
        velocityMapping = 'PassThrough';
        break;
      case 'Linear':
        velocityMapping = { Linear: { min: minVelocity, max: maxVelocity } };
        break;
      case 'Curve':
        velocityMapping = {
          Curve: {
            curve_type: curveType,
            intensity: intensity
          }
        };
        break;
    }

    emitChange();
  }

  /**
   * Handle fixed velocity change
   */
  function handleFixedVelocityChange(event) {
    const value = parseInt(event.target.value, 10);
    fixedVelocity = Math.max(0, Math.min(127, value));
    velocityMapping = { Fixed: { velocity: fixedVelocity } };
    emitChange();
  }

  /**
   * Handle linear min change
   */
  function handleMinChange(event) {
    const value = parseInt(event.target.value, 10);
    minVelocity = Math.max(0, Math.min(127, value));
    velocityMapping = { Linear: { min: minVelocity, max: maxVelocity } };
    emitChange();
  }

  /**
   * Handle linear max change
   */
  function handleMaxChange(event) {
    const value = parseInt(event.target.value, 10);
    maxVelocity = Math.max(0, Math.min(127, value));
    velocityMapping = { Linear: { min: minVelocity, max: maxVelocity } };
    emitChange();
  }

  /**
   * Handle curve type change
   */
  function handleCurveTypeChange(event) {
    curveType = event.target.value;
    velocityMapping = {
      Curve: {
        curve_type: curveType,
        intensity: intensity
      }
    };
    emitChange();
  }

  /**
   * Handle intensity change
   */
  function handleIntensityChange(event) {
    const value = parseFloat(event.target.value);
    intensity = Math.max(0.0, Math.min(1.0, value));
    velocityMapping = {
      Curve: {
        curve_type: curveType,
        intensity: intensity
      }
    };
    emitChange();
  }

  /**
   * Emit change event
   */
  function emitChange() {
    dispatch('change', { velocityMapping });
  }

  /**
   * Get description for current mode
   */
  function getModeDescription(mode) {
    const descriptions = {
      'Fixed': 'Always outputs the same velocity value',
      'PassThrough': 'Output velocity = trigger velocity (1:1 mapping)',
      'Linear': 'Maps full input range (0-127) to custom output range',
      'Curve': 'Applies non-linear transformation to velocity'
    };
    return descriptions[mode] || '';
  }

  /**
   * Get description for curve type
   */
  function getCurveDescription(type) {
    const descriptions = {
      'Exponential': 'Makes soft hits louder while preserving hard hits',
      'Logarithmic': 'Compresses dynamic range (soft hits quieter)',
      'SCurve': 'Smooth transitions with acceleration in middle range'
    };
    return descriptions[type] || '';
  }

  /**
   * Generate velocity curve path for visual preview
   */
  function generateCurvePath(mode, type, intensity, min, max, fixedVal) {
    const points = [];
    const steps = 64; // Number of points to sample

    for (let i = 0; i <= steps; i++) {
      const input = (i / steps) * 127;
      let output = 0;

      switch (mode) {
        case 'Fixed':
          output = fixedVal;
          break;

        case 'PassThrough':
          output = input;
          break;

        case 'Linear':
          output = min + (input / 127) * (max - min);
          break;

        case 'Curve': {
          const normalized = input / 127;
          let transformed = 0;

          switch (type) {
            case 'Exponential':
              // Exponential curve: output = input^(1-intensity)
              // intensity 0 = linear, intensity 1 = max compression
              transformed = Math.pow(normalized, 1 - intensity);
              break;

            case 'Logarithmic':
              // Logarithmic curve: output = log(1 + intensity * input)
              transformed = Math.log(1 + intensity * normalized) / Math.log(1 + intensity);
              break;

            case 'SCurve':
              // S-curve: smooth sigmoid with intensity controlling steepness
              const k = intensity * 10 + 0.5; // Scale intensity to reasonable range
              transformed = 1 / (1 + Math.exp(-k * (normalized - 0.5)));
              // Normalize to 0-1 range
              const min_val = 1 / (1 + Math.exp(k * 0.5));
              const max_val = 1 / (1 + Math.exp(-k * 0.5));
              transformed = (transformed - min_val) / (max_val - min_val);
              break;
          }

          output = transformed * 127;
          break;
        }
      }

      // Convert to SVG coordinates (flip Y axis)
      const x = (input / 127) * 200; // Scale to viewBox width
      const y = 100 - (output / 127) * 100; // Scale to viewBox height and flip

      points.push(`${x},${y}`);
    }

    return `M ${points.join(' L ')}`;
  }

  // Reactive curve path
  $: curvePath = generateCurvePath(mode, curveType, intensity, minVelocity, maxVelocity, fixedVelocity);
</script>

<div class="velocity-mapping-selector">
  <div class="selector-header">
    <label for="velocity-mode-select" class="selector-label">
      {label}
    </label>
    <span class="mode-badge">{mode}</span>
  </div>

  <!-- Mode Selection -->
  <div class="form-group">
    <label for="velocity-mode-select">Mode</label>
    <select
      id="velocity-mode-select"
      bind:value={mode}
      on:change={handleModeChange}
      disabled={readonly}
      class="mode-select"
    >
      <option value="Fixed">Fixed</option>
      <option value="PassThrough">Pass-Through</option>
      <option value="Linear">Linear</option>
      <option value="Curve">Curve</option>
    </select>
    <p class="help-text">{getModeDescription(mode)}</p>
  </div>

  <!-- Velocity Curve Preview -->
  <div class="curve-preview">
    <div class="preview-header">
      <label>Velocity Response Curve</label>
      <span class="preview-label">Input → Output</span>
    </div>
    <svg viewBox="0 0 220 120" class="curve-graph">
      <!-- Background grid -->
      <g class="grid">
        {#each [0, 25, 50, 75, 100] as tick}
          <line
            x1={tick * 2}
            y1="0"
            x2={tick * 2}
            y2="100"
            class="grid-line"
          />
          <line
            x1="0"
            y1={100 - tick}
            x2="200"
            y2={100 - tick}
            class="grid-line"
          />
        {/each}
      </g>

      <!-- Axes -->
      <line x1="0" y1="100" x2="200" y2="100" class="axis" />
      <line x1="0" y1="0" x2="0" y2="100" class="axis" />

      <!-- Reference diagonal (1:1 PassThrough) -->
      <line x1="0" y1="100" x2="200" y2="0" class="reference-line" />

      <!-- Actual velocity curve -->
      <path d={curvePath} class="velocity-curve" />

      <!-- Axis labels -->
      <text x="100" y="115" class="axis-label">Input (0-127)</text>
      <text x="-50" y="5" class="axis-label axis-label-vertical" transform="rotate(-90 -50 5)">
        Output (0-127)
      </text>

      <!-- Grid value labels -->
      {#each [0, 64, 127] as value}
        <text x={(value / 127) * 200} y="112" class="tick-label">
          {value}
        </text>
        <text x="-5" y={100 - (value / 127) * 100 + 3} class="tick-label tick-label-y">
          {value}
        </text>
      {/each}
    </svg>
    <p class="help-text">
      <span class="legend-item">
        <span class="legend-line reference"></span>
        1:1 Reference
      </span>
      <span class="legend-item">
        <span class="legend-line curve"></span>
        Current Mapping
      </span>
    </p>
  </div>

  <!-- Fixed Mode Fields -->
  {#if mode === 'Fixed'}
    <div class="form-group">
      <label for="fixed-velocity">Velocity</label>
      <div class="slider-group">
        <input
          type="range"
          id="fixed-velocity"
          min="0"
          max="127"
          value={fixedVelocity}
          on:input={handleFixedVelocityChange}
          disabled={readonly}
          class="velocity-slider"
        />
        <input
          type="number"
          min="0"
          max="127"
          value={fixedVelocity}
          on:input={handleFixedVelocityChange}
          disabled={readonly}
          class="velocity-number"
        />
      </div>
    </div>
  {/if}

  <!-- Linear Mode Fields -->
  {#if mode === 'Linear'}
    <div class="form-group">
      <label for="linear-min">Minimum Velocity</label>
      <div class="slider-group">
        <input
          type="range"
          id="linear-min"
          min="0"
          max="127"
          value={minVelocity}
          on:input={handleMinChange}
          disabled={readonly}
          class="velocity-slider"
        />
        <input
          type="number"
          min="0"
          max="127"
          value={minVelocity}
          on:input={handleMinChange}
          disabled={readonly}
          class="velocity-number"
        />
      </div>
    </div>

    <div class="form-group">
      <label for="linear-max">Maximum Velocity</label>
      <div class="slider-group">
        <input
          type="range"
          id="linear-max"
          min="0"
          max="127"
          value={maxVelocity}
          on:input={handleMaxChange}
          disabled={readonly}
          class="velocity-slider"
        />
        <input
          type="number"
          min="0"
          max="127"
          value={maxVelocity}
          on:input={handleMaxChange}
          disabled={readonly}
          class="velocity-number"
        />
      </div>
    </div>

    <p class="help-text range-info">
      Maps 0-127 → {minVelocity}-{maxVelocity}
    </p>
  {/if}

  <!-- Curve Mode Fields -->
  {#if mode === 'Curve'}
    <div class="form-group">
      <label for="curve-type-select">Curve Type</label>
      <select
        id="curve-type-select"
        value={curveType}
        on:change={handleCurveTypeChange}
        disabled={readonly}
        class="curve-type-select"
      >
        <option value="Exponential">Exponential</option>
        <option value="Logarithmic">Logarithmic</option>
        <option value="SCurve">S-Curve</option>
      </select>
      <p class="help-text">{getCurveDescription(curveType)}</p>
    </div>

    <div class="form-group">
      <label for="curve-intensity">Intensity</label>
      <div class="slider-group">
        <input
          type="range"
          id="curve-intensity"
          min="0"
          max="1"
          step="0.01"
          value={intensity}
          on:input={handleIntensityChange}
          disabled={readonly}
          class="intensity-slider"
        />
        <input
          type="number"
          min="0"
          max="1"
          step="0.01"
          value={intensity}
          on:input={handleIntensityChange}
          disabled={readonly}
          class="intensity-number"
        />
      </div>
      <p class="help-text">0.0 = linear, 1.0 = maximum effect</p>
    </div>
  {/if}
</div>

<style>
  .velocity-mapping-selector {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
    background: var(--surface-1);
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
  }

  .selector-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .selector-label {
    font-weight: 600;
    font-size: 0.875rem;
    color: var(--text-primary);
  }

  .mode-badge {
    padding: 0.25rem 0.75rem;
    background: var(--accent-color);
    color: white;
    border-radius: 1rem;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .mode-select,
  .curve-type-select {
    padding: 0.5rem;
    border: 1px solid var(--border-color);
    border-radius: 0.25rem;
    background: var(--surface-0);
    color: var(--text-primary);
    font-size: 0.875rem;
  }

  .mode-select:disabled,
  .curve-type-select:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .slider-group {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 0.75rem;
    align-items: center;
  }

  .velocity-slider,
  .intensity-slider {
    width: 100%;
    height: 0.5rem;
    border-radius: 0.25rem;
    background: var(--surface-2);
    outline: none;
    -webkit-appearance: none;
  }

  .velocity-slider::-webkit-slider-thumb,
  .intensity-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 1rem;
    height: 1rem;
    border-radius: 50%;
    background: var(--accent-color);
    cursor: pointer;
  }

  .velocity-slider::-moz-range-thumb,
  .intensity-slider::-moz-range-thumb {
    width: 1rem;
    height: 1rem;
    border-radius: 50%;
    background: var(--accent-color);
    cursor: pointer;
    border: none;
  }

  .velocity-slider:disabled,
  .intensity-slider:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .velocity-number,
  .intensity-number {
    width: 5rem;
    padding: 0.5rem;
    border: 1px solid var(--border-color);
    border-radius: 0.25rem;
    background: var(--surface-0);
    color: var(--text-primary);
    font-size: 0.875rem;
    text-align: center;
  }

  .velocity-number:disabled,
  .intensity-number:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .help-text {
    font-size: 0.75rem;
    color: var(--text-tertiary);
    margin: 0;
    font-style: italic;
  }

  .range-info {
    font-weight: 500;
    color: var(--text-secondary);
  }

  /* Curve Preview Styles */
  .curve-preview {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 1rem;
    background: var(--surface-0);
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .preview-header label {
    font-weight: 600;
    font-size: 0.875rem;
    color: var(--text-primary);
  }

  .preview-label {
    font-size: 0.75rem;
    color: var(--text-tertiary);
    font-style: italic;
  }

  .curve-graph {
    width: 100%;
    height: auto;
    max-height: 150px;
    background: var(--surface-1);
    border-radius: 0.25rem;
    padding: 0.5rem;
  }

  .grid-line {
    stroke: var(--border-color);
    stroke-width: 0.5;
    opacity: 0.3;
  }

  .axis {
    stroke: var(--text-secondary);
    stroke-width: 1.5;
  }

  .reference-line {
    stroke: var(--text-tertiary);
    stroke-width: 1;
    stroke-dasharray: 4 2;
    opacity: 0.5;
  }

  .velocity-curve {
    stroke: var(--accent-color);
    stroke-width: 2.5;
    fill: none;
    stroke-linecap: round;
    stroke-linejoin: round;
  }

  .axis-label {
    font-size: 0.625rem;
    fill: var(--text-secondary);
    text-anchor: middle;
    font-weight: 500;
  }

  .axis-label-vertical {
    text-anchor: middle;
  }

  .tick-label {
    font-size: 0.625rem;
    fill: var(--text-tertiary);
    text-anchor: middle;
  }

  .tick-label-y {
    text-anchor: end;
  }

  .legend-item {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    margin-right: 1rem;
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .legend-line {
    display: inline-block;
    width: 1.5rem;
    height: 2px;
    border-radius: 1px;
  }

  .legend-line.reference {
    background: var(--text-tertiary);
    opacity: 0.5;
  }

  .legend-line.curve {
    background: var(--accent-color);
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .velocity-mapping-selector {
      --surface-0: #1a1a1a;
      --surface-1: #2a2a2a;
      --surface-2: #3a3a3a;
      --text-primary: #ffffff;
      --text-secondary: #b0b0b0;
      --text-tertiary: #808080;
      --border-color: #404040;
      --accent-color: #3b82f6;
    }
  }

  /* Light mode */
  @media (prefers-color-scheme: light) {
    .velocity-mapping-selector {
      --surface-0: #ffffff;
      --surface-1: #f5f5f5;
      --surface-2: #e0e0e0;
      --text-primary: #000000;
      --text-secondary: #606060;
      --text-tertiary: #909090;
      --border-color: #d0d0d0;
      --accent-color: #2563eb;
    }
  }
</style>
