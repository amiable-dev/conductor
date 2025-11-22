<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, createEventDispatcher } from 'svelte';

  interface DeviceTemplate {
    id: string;
    name: string;
    manufacturer: string;
    model: string;
    description: string;
    midi_patterns: string[];
    category: string;
    is_official: boolean;
    version: string;
  }

  export let currentMidiDevice: string | null = null;

  const dispatch = createEventDispatcher();

  let templates: DeviceTemplate[] = [];
  let filteredTemplates: DeviceTemplate[] = [];
  let categories: string[] = [];
  let selectedCategory: string = 'all';
  let searchQuery: string = '';
  let loading: boolean = false;
  let error: string | null = null;
  let matchingTemplates: DeviceTemplate[] = [];
  let selectedTemplate: DeviceTemplate | null = null;
  let showConfirmDialog: boolean = false;

  onMount(async () => {
    await loadTemplates();
    if (currentMidiDevice) {
      await findMatchingTemplates();
    }
  });

  async function loadTemplates() {
    loading = true;
    error = null;
    try {
      templates = await invoke<DeviceTemplate[]>('list_device_templates');
      categories = await invoke<string[]>('get_template_categories');
      filterTemplates();
    } catch (e) {
      error = `Failed to load templates: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  async function findMatchingTemplates() {
    if (!currentMidiDevice) return;

    try {
      matchingTemplates = await invoke<DeviceTemplate[]>('find_templates_by_midi', {
        midiName: currentMidiDevice
      });
    } catch (e) {
      console.error('Failed to find matching templates:', e);
    }
  }

  function filterTemplates() {
    let result = templates;

    // Filter by category
    if (selectedCategory !== 'all') {
      result = result.filter(t => t.category === selectedCategory);
    }

    // Filter by search query
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      result = result.filter(t =>
        t.name.toLowerCase().includes(query) ||
        t.manufacturer.toLowerCase().includes(query) ||
        t.model.toLowerCase().includes(query) ||
        t.description.toLowerCase().includes(query)
      );
    }

    filteredTemplates = result;
  }

  function handleCategoryChange(category: string) {
    selectedCategory = category;
    filterTemplates();
  }

  function handleSearch(event: Event) {
    searchQuery = (event.target as HTMLInputElement).value;
    filterTemplates();
  }

  async function handleTemplateSelect(template: DeviceTemplate) {
    selectedTemplate = template;
    showConfirmDialog = true;
  }

  function confirmSelection() {
    if (selectedTemplate) {
      dispatch('selected', { template: selectedTemplate });
      showConfirmDialog = false;
      selectedTemplate = null;
    }
  }

  function cancelSelection() {
    showConfirmDialog = false;
    selectedTemplate = null;
  }

  function closeModal() {
    dispatch('close');
  }

  function getCategoryIcon(category: string): string {
    switch (category) {
      case 'pad-controller': return '🎹';
      case 'keyboard': return '🎼';
      case 'mixer-controller': return '🎚️';
      case 'gamepad-controller': return '🎮';
      default: return '🎛️';
    }
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="modal-overlay" on:click|self={closeModal}>
  <div class="modal-content" on:click|stopPropagation>
    <div class="modal-header">
      <h2>Device Templates</h2>
      <button class="close-btn" on:click={closeModal}>×</button>
    </div>

    <div class="template-selector">
      <div class="header">
        {#if currentMidiDevice}
          <p class="device-info">Connected: {currentMidiDevice}</p>
        {/if}
      </div>

  {#if loading}
    <div class="loading">Loading templates...</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else}
    <!-- Matching templates section -->
    {#if matchingTemplates.length > 0}
      <div class="matching-section">
        <h4>✨ Recommended for Your Device</h4>
        <div class="template-grid">
          {#each matchingTemplates as template}
            <button
              class="template-card recommended"
              on:click={() => handleTemplateSelect(template)}
            >
              <div class="template-header">
                <span class="category-icon">{getCategoryIcon(template.category)}</span>
                <span class="template-name">{template.name}</span>
                {#if template.is_official}
                  <span class="badge official">Official</span>
                {/if}
              </div>
              <div class="template-body">
                <p class="manufacturer">{template.manufacturer}</p>
                <p class="description">{template.description}</p>
              </div>
              <div class="template-footer">
                <span class="version">v{template.version}</span>
                <span class="category">{template.category}</span>
              </div>
            </button>
          {/each}
        </div>
      </div>
    {/if}

    <!-- Search and filter controls -->
    <div class="controls">
      <input
        type="text"
        placeholder="Search templates..."
        value={searchQuery}
        on:input={handleSearch}
        class="search-input"
      />

      <div class="category-filter">
        <button
          class:active={selectedCategory === 'all'}
          on:click={() => handleCategoryChange('all')}
        >
          All
        </button>
        {#each categories as category}
          <button
            class:active={selectedCategory === category}
            on:click={() => handleCategoryChange(category)}
          >
            {getCategoryIcon(category)} {category}
          </button>
        {/each}
      </div>
    </div>

    <!-- All templates grid -->
    <div class="template-grid">
      {#if filteredTemplates.length === 0}
        <p class="no-results">No templates found matching your criteria.</p>
      {:else}
        {#each filteredTemplates as template}
          <button
            class="template-card"
            on:click={() => handleTemplateSelect(template)}
          >
            <div class="template-header">
              <span class="category-icon">{getCategoryIcon(template.category)}</span>
              <span class="template-name">{template.name}</span>
              {#if template.is_official}
                <span class="badge official">Official</span>
              {/if}
            </div>
            <div class="template-body">
              <p class="manufacturer">{template.manufacturer}</p>
              <p class="description">{template.description}</p>
            </div>
            <div class="template-footer">
              <span class="version">v{template.version}</span>
              <span class="category">{template.category}</span>
            </div>
          </button>
        {/each}
      {/if}
    </div>
  {/if}
    </div>

    <!-- Confirmation Dialog -->
    {#if showConfirmDialog && selectedTemplate}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div class="confirm-overlay" on:click|self={cancelSelection}>
        <div class="confirm-dialog">
          <h3>Create Configuration</h3>
          <p>Create configuration from template:</p>
          <div class="template-preview">
            <div class="preview-icon">{getCategoryIcon(selectedTemplate.category)}</div>
            <div class="preview-details">
              <h4>{selectedTemplate.name}</h4>
              <p class="preview-manufacturer">{selectedTemplate.manufacturer} - {selectedTemplate.model}</p>
              <p class="preview-description">{selectedTemplate.description}</p>
            </div>
          </div>
          <p class="warning">⚠️ This will create a new configuration file. Make sure to back up your current config if needed.</p>
          <div class="confirm-actions">
            <button class="btn-cancel" on:click={cancelSelection}>Cancel</button>
            <button class="btn-confirm" on:click={confirmSelection}>Create Config</button>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .template-selector {
    padding: 1rem;
    height: 100%;
    overflow-y: auto;
  }

  .header h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.5rem;
    color: var(--text-primary, #333);
  }

  .device-info {
    margin: 0;
    font-size: 0.875rem;
    color: var(--text-secondary, #666);
  }

  .loading, .error {
    padding: 2rem;
    text-align: center;
  }

  .error {
    color: var(--error, #dc2626);
  }

  .matching-section {
    margin-bottom: 2rem;
    padding: 1rem;
    background: linear-gradient(135deg, rgba(59, 130, 246, 0.1), rgba(139, 92, 246, 0.1));
    border-radius: 0.5rem;
    border: 1px solid rgba(59, 130, 246, 0.3);
  }

  .matching-section h4 {
    margin: 0 0 1rem 0;
    font-size: 1.125rem;
    color: var(--text-primary, #333);
  }

  .controls {
    margin-bottom: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .search-input {
    width: 100%;
    padding: 0.625rem 1rem;
    border: 1px solid var(--border, #d1d5db);
    border-radius: 0.375rem;
    font-size: 0.875rem;
    background: var(--bg-secondary, #fff);
    color: var(--text-primary, #333);
  }

  .search-input:focus {
    outline: none;
    border-color: var(--primary, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .category-filter {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .category-filter button {
    padding: 0.5rem 1rem;
    border: 1px solid var(--border, #d1d5db);
    border-radius: 0.375rem;
    background: var(--bg-secondary, #fff);
    color: var(--text-primary, #333);
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .category-filter button:hover {
    background: var(--bg-hover, #f3f4f6);
  }

  .category-filter button.active {
    background: var(--primary, #3b82f6);
    color: white;
    border-color: var(--primary, #3b82f6);
  }

  .template-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 1rem;
  }

  .template-card {
    padding: 1rem;
    border: 1px solid var(--border, #d1d5db);
    border-radius: 0.5rem;
    background: var(--bg-secondary, #fff);
    cursor: pointer;
    transition: all 0.2s;
    text-align: left;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .template-card:hover {
    border-color: var(--primary, #3b82f6);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    transform: translateY(-2px);
  }

  .template-card.recommended {
    border-color: rgba(59, 130, 246, 0.5);
    background: rgba(59, 130, 246, 0.05);
  }

  .template-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .category-icon {
    font-size: 1.5rem;
  }

  .template-name {
    font-weight: 600;
    color: var(--text-primary, #333);
    flex: 1;
  }

  .badge {
    padding: 0.125rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .badge.official {
    background: var(--success, #10b981);
    color: white;
  }

  .template-body {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .manufacturer {
    margin: 0;
    font-size: 0.875rem;
    color: var(--text-secondary, #666);
    font-weight: 500;
  }

  .description {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--text-secondary, #666);
    line-height: 1.4;
  }

  .template-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 0.5rem;
    border-top: 1px solid var(--border, #e5e7eb);
    font-size: 0.75rem;
    color: var(--text-secondary, #666);
  }

  .version {
    font-weight: 500;
  }

  .category {
    font-style: italic;
  }

  .no-results {
    grid-column: 1 / -1;
    text-align: center;
    padding: 2rem;
    color: var(--text-secondary, #666);
  }

  /* Modal styles */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: var(--bg-primary, #fff);
    border-radius: 0.75rem;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
    max-width: 90vw;
    max-height: 90vh;
    width: 1200px;
    display: flex;
    flex-direction: column;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 2rem;
    border-bottom: 1px solid var(--border, #d1d5db);
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.5rem;
    color: var(--text-primary, #333);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 2rem;
    color: var(--text-secondary, #666);
    cursor: pointer;
    padding: 0;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.25rem;
    transition: all 0.2s;
  }

  .close-btn:hover {
    background: var(--bg-hover, #f3f4f6);
    color: var(--text-primary, #333);
  }

  /* Confirmation dialog */
  .confirm-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10;
  }

  .confirm-dialog {
    background: var(--bg-primary, #fff);
    border-radius: 0.75rem;
    padding: 2rem;
    max-width: 500px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  }

  .confirm-dialog h3 {
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
    color: var(--text-primary, #333);
  }

  .confirm-dialog > p {
    margin: 0 0 1rem 0;
    color: var(--text-secondary, #666);
  }

  .template-preview {
    display: flex;
    gap: 1rem;
    padding: 1rem;
    background: var(--bg-secondary, #f9fafb);
    border-radius: 0.5rem;
    margin-bottom: 1rem;
  }

  .preview-icon {
    font-size: 3rem;
    flex-shrink: 0;
  }

  .preview-details h4 {
    margin: 0 0 0.25rem 0;
    font-size: 1.125rem;
    color: var(--text-primary, #333);
  }

  .preview-manufacturer {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: var(--text-secondary, #666);
    font-weight: 500;
  }

  .preview-description {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--text-secondary, #666);
    line-height: 1.4;
  }

  .warning {
    padding: 0.75rem 1rem;
    background: rgba(251, 191, 36, 0.1);
    border: 1px solid rgba(251, 191, 36, 0.3);
    border-radius: 0.375rem;
    color: #92400e;
    font-size: 0.875rem;
    margin-bottom: 1.5rem;
  }

  .confirm-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
  }

  .btn-cancel,
  .btn-confirm {
    padding: 0.625rem 1.25rem;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-cancel {
    background: var(--bg-secondary, #f3f4f6);
    border: 1px solid var(--border, #d1d5db);
    color: var(--text-primary, #333);
  }

  .btn-cancel:hover {
    background: var(--bg-hover, #e5e7eb);
  }

  .btn-confirm {
    background: var(--primary, #3b82f6);
    border: none;
    color: white;
  }

  .btn-confirm:hover {
    background: #2563eb;
  }
</style>
