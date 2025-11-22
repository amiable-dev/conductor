<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  // State
  let plugins = [];
  let categories = [];
  let selectedCategory = 'all';
  let searchQuery = '';
  let selectedPlugin = null;
  let isLoading = true;
  let error = null;
  let installedPlugins = new Set();

  // Fetch plugin registry on mount
  onMount(async () => {
    await loadRegistry();
    await loadInstalledPlugins();
  });

  async function loadRegistry() {
    try {
      isLoading = true;
      error = null;
      const registry = await invoke('fetch_plugin_registry');
      plugins = registry.plugins;
      categories = [
        { id: 'all', name: 'All Plugins', description: 'Browse all available plugins' },
        ...registry.categories
      ];
    } catch (e) {
      error = `Failed to load plugin registry: ${e}`;
      console.error(error);
    } finally {
      isLoading = false;
    }
  }

  async function loadInstalledPlugins() {
    try {
      const installed = await invoke('list_installed_plugins');
      // list_installed_plugins returns array of strings (plugin names)
      installedPlugins = new Set(installed);
    } catch (e) {
      console.error('Failed to load installed plugins:', e);
    }
  }

  async function installPlugin(pluginId) {
    try {
      await invoke('install_plugin_from_registry', { pluginId });
      installedPlugins.add(pluginId);
      installedPlugins = installedPlugins; // Trigger reactivity
      alert(`Plugin "${pluginId}" installed successfully!`);
    } catch (e) {
      alert(`Failed to install plugin: ${e}`);
    }
  }

  async function uninstallPlugin(pluginId) {
    if (!confirm(`Are you sure you want to uninstall "${pluginId}"?`)) {
      return;
    }

    try {
      await invoke('uninstall_plugin', { pluginName: pluginId });
      installedPlugins.delete(pluginId);
      installedPlugins = installedPlugins; // Trigger reactivity
      alert(`Plugin "${pluginId}" uninstalled successfully!`);
    } catch (e) {
      alert(`Failed to uninstall plugin: ${e}`);
    }
  }

  // Filter plugins based on category and search
  $: filteredPlugins = plugins.filter(plugin => {
    const matchesCategory = selectedCategory === 'all' || plugin.category === selectedCategory;
    const matchesSearch = !searchQuery ||
      plugin.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      plugin.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
      (plugin.tags && plugin.tags.some(tag => tag.toLowerCase().includes(searchQuery.toLowerCase())));
    return matchesCategory && matchesSearch;
  });

  function selectPlugin(plugin) {
    selectedPlugin = plugin;
  }

  function closeDetails() {
    selectedPlugin = null;
  }
</script>

<div class="plugin-marketplace">
  <header class="marketplace-header">
    <h1>Plugin Marketplace</h1>
    <p class="subtitle">Extend Conductor with community plugins</p>
  </header>

  {#if error}
    <div class="error-banner">
      <span class="error-icon">⚠️</span>
      <span>{error}</span>
      <button class="retry-button" on:click={loadRegistry}>Retry</button>
    </div>
  {/if}

  <div class="marketplace-content">
    <!-- Sidebar: Categories and Search -->
    <aside class="sidebar">
      <div class="search-box">
        <input
          type="text"
          placeholder="Search plugins..."
          bind:value={searchQuery}
          class="search-input"
        />
      </div>

      <div class="categories">
        <h3>Categories</h3>
        <ul class="category-list">
          {#each categories as category}
            <li>
              <button
                class="category-button"
                class:active={selectedCategory === category.id}
                on:click={() => selectedCategory = category.id}
              >
                {category.name}
              </button>
            </li>
          {/each}
        </ul>
      </div>

      <div class="stats">
        <p class="stat-item">
          <strong>{plugins.length}</strong> available
        </p>
        <p class="stat-item">
          <strong>{installedPlugins.size}</strong> installed
        </p>
      </div>
    </aside>

    <!-- Main: Plugin Grid -->
    <main class="plugin-grid-container">
      {#if isLoading}
        <div class="loading">
          <div class="spinner"></div>
          <p>Loading plugins...</p>
        </div>
      {:else if filteredPlugins.length === 0}
        <div class="empty-state">
          <p>No plugins found</p>
          {#if searchQuery}
            <button class="clear-search" on:click={() => searchQuery = ''}>Clear search</button>
          {/if}
        </div>
      {:else}
        <div class="plugin-grid">
          {#each filteredPlugins as plugin}
            <article class="plugin-card" on:click={() => selectPlugin(plugin)}>
              <div class="plugin-header">
                <h3 class="plugin-name">{plugin.name}</h3>
                {#if installedPlugins.has(plugin.id)}
                  <span class="badge installed">Installed</span>
                {/if}
              </div>

              <p class="plugin-description">{plugin.description}</p>

              <div class="plugin-meta">
                <span class="meta-item">
                  <span class="meta-icon">👤</span>
                  {plugin.author}
                </span>
                <span class="meta-item">
                  <span class="meta-icon">📦</span>
                  v{plugin.version}
                </span>
              </div>

              <div class="plugin-tags">
                {#each plugin.tags.slice(0, 3) as tag}
                  <span class="tag">{tag}</span>
                {/each}
              </div>

              <div class="plugin-footer">
                {#if installedPlugins.has(plugin.id)}
                  <button
                    class="btn btn-secondary"
                    on:click|stopPropagation={() => uninstallPlugin(plugin.id)}
                  >
                    Uninstall
                  </button>
                {:else}
                  <button
                    class="btn btn-primary"
                    on:click|stopPropagation={() => installPlugin(plugin.id)}
                  >
                    Install
                  </button>
                {/if}
                {#if plugin.install_count !== undefined}
                  <span class="install-count">{plugin.install_count} installs</span>
                {/if}
              </div>
            </article>
          {/each}
        </div>
      {/if}
    </main>
  </div>

  <!-- Plugin Details Modal -->
  {#if selectedPlugin}
    <div class="modal-overlay" on:click={closeDetails}>
      <div class="modal-content" on:click|stopPropagation>
        <button class="close-button" on:click={closeDetails}>×</button>

        <header class="modal-header">
          <h2>{selectedPlugin.name}</h2>
          <div class="modal-meta">
            <span>{selectedPlugin.author}</span>
            <span>•</span>
            <span>v{selectedPlugin.version}</span>
            <span>•</span>
            <span>{selectedPlugin.license}</span>
          </div>
        </header>

        <div class="modal-body">
          <p class="plugin-description-full">{selectedPlugin.description}</p>

          <section class="details-section">
            <h3>Category</h3>
            <div class="category-badges">
              <span class="badge category">{selectedPlugin.category}</span>
            </div>
          </section>

          <section class="details-section">
            <h3>Tags</h3>
            <div class="tag-list">
              {#each selectedPlugin.tags as tag}
                <span class="tag">{tag}</span>
              {/each}
            </div>
          </section>

          <section class="details-section">
            <h3>Capabilities Required</h3>
            <ul class="capability-list">
              {#each selectedPlugin.capabilities as capability}
                <li>{capability}</li>
              {/each}
            </ul>
          </section>

          {#if selectedPlugin.platforms && selectedPlugin.platforms.length > 0}
            <section class="details-section">
              <h3>Supported Platforms</h3>
              <div class="platform-badges">
                {#each selectedPlugin.platforms as platform}
                  <span class="badge platform">{platform}</span>
                {/each}
              </div>
            </section>
          {/if}

          {#if selectedPlugin.repository || selectedPlugin.documentation}
            <section class="details-section">
              <h3>Links</h3>
              <div class="links">
                {#if selectedPlugin.repository}
                  <a href={selectedPlugin.repository} target="_blank" class="link">Repository</a>
                {/if}
                {#if selectedPlugin.documentation}
                  <a href={selectedPlugin.documentation} target="_blank" class="link">Documentation</a>
                {/if}
              </div>
            </section>
          {/if}
        </div>

        <footer class="modal-footer">
          {#if installedPlugins.has(selectedPlugin.id)}
            <button class="btn btn-secondary btn-lg" on:click={() => {
              uninstallPlugin(selectedPlugin.id);
              closeDetails();
            }}>
              Uninstall Plugin
            </button>
          {:else}
            <button class="btn btn-primary btn-lg" on:click={() => {
              installPlugin(selectedPlugin.id);
              closeDetails();
            }}>
              Install Plugin
            </button>
          {/if}
        </footer>
      </div>
    </div>
  {/if}
</div>

<style>
  .plugin-marketplace {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--background, #1a1a1a);
    color: var(--text, #e0e0e0);
  }

  .marketplace-header {
    padding: 2rem;
    border-bottom: 1px solid var(--border, #333);
  }

  .marketplace-header h1 {
    margin: 0 0 0.5rem 0;
    font-size: 2rem;
  }

  .subtitle {
    margin: 0;
    color: var(--text-secondary, #999);
  }

  .error-banner {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem 2rem;
    background: var(--error-bg, #ff000020);
    color: var(--error, #ff4444);
    border-bottom: 1px solid var(--error, #ff4444);
  }

  .retry-button {
    margin-left: auto;
    padding: 0.5rem 1rem;
    background: var(--error, #ff4444);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .marketplace-content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .sidebar {
    width: 250px;
    padding: 1.5rem;
    border-right: 1px solid var(--border, #333);
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    overflow-y: auto;
  }

  .search-box {
    position: relative;
  }

  .search-input {
    width: 100%;
    padding: 0.75rem;
    background: var(--input-bg, #2a2a2a);
    border: 1px solid var(--border, #333);
    border-radius: 8px;
    color: var(--text, #e0e0e0);
    font-size: 0.9rem;
  }

  .search-input:focus {
    outline: none;
    border-color: var(--primary, #4a9eff);
  }

  .categories h3 {
    margin: 0 0 0.75rem 0;
    font-size: 0.9rem;
    text-transform: uppercase;
    color: var(--text-secondary, #999);
  }

  .category-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .category-button {
    width: 100%;
    padding: 0.75rem 1rem;
    text-align: left;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: var(--text, #e0e0e0);
    cursor: pointer;
    transition: all 0.2s;
  }

  .category-button:hover {
    background: var(--hover-bg, #2a2a2a);
  }

  .category-button.active {
    background: var(--primary, #4a9eff);
    color: white;
  }

  .stats {
    padding-top: 1rem;
    border-top: 1px solid var(--border, #333);
  }

  .stat-item {
    margin: 0.5rem 0;
    font-size: 0.9rem;
    color: var(--text-secondary, #999);
  }

  .plugin-grid-container {
    flex: 1;
    padding: 2rem;
    overflow-y: auto;
  }

  .loading, .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-secondary, #999);
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--border, #333);
    border-top-color: var(--primary, #4a9eff);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .plugin-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
  }

  .plugin-card {
    background: var(--card-bg, #2a2a2a);
    border: 1px solid var(--border, #333);
    border-radius: 12px;
    padding: 1.5rem;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .plugin-card:hover {
    border-color: var(--primary, #4a9eff);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .plugin-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .plugin-name {
    margin: 0;
    font-size: 1.25rem;
  }

  .badge {
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .badge.installed {
    background: var(--success, #22c55e);
    color: white;
  }

  .plugin-description {
    margin: 0;
    color: var(--text-secondary, #999);
    font-size: 0.9rem;
    line-height: 1.5;
    flex: 1;
  }

  .plugin-meta {
    display: flex;
    gap: 1rem;
    font-size: 0.85rem;
    color: var(--text-secondary, #999);
  }

  .meta-item {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .plugin-tags {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .tag {
    padding: 0.25rem 0.5rem;
    background: var(--tag-bg, #3a3a3a);
    border-radius: 4px;
    font-size: 0.75rem;
    color: var(--text-secondary, #999);
  }

  .plugin-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-top: 1rem;
    border-top: 1px solid var(--border, #333);
  }

  .btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 6px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-primary {
    background: var(--primary, #4a9eff);
    color: white;
  }

  .btn-primary:hover {
    background: var(--primary-hover, #3a8eef);
  }

  .btn-secondary {
    background: var(--secondary-bg, #3a3a3a);
    color: var(--text, #e0e0e0);
  }

  .btn-secondary:hover {
    background: var(--secondary-hover, #4a4a4a);
  }

  .btn-lg {
    padding: 0.75rem 2rem;
    font-size: 1rem;
  }

  .install-count {
    font-size: 0.85rem;
    color: var(--text-secondary, #999);
  }

  /* Modal Styles */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: var(--card-bg, #2a2a2a);
    border-radius: 16px;
    width: 90%;
    max-width: 600px;
    max-height: 90vh;
    overflow-y: auto;
    position: relative;
  }

  .close-button {
    position: absolute;
    top: 1rem;
    right: 1rem;
    width: 32px;
    height: 32px;
    border: none;
    background: var(--hover-bg, #3a3a3a);
    color: var(--text, #e0e0e0);
    border-radius: 50%;
    font-size: 1.5rem;
    line-height: 1;
    cursor: pointer;
    transition: all 0.2s;
  }

  .close-button:hover {
    background: var(--error, #ff4444);
    color: white;
  }

  .modal-header {
    padding: 2rem;
    border-bottom: 1px solid var(--border, #333);
  }

  .modal-header h2 {
    margin: 0 0 0.5rem 0;
  }

  .modal-meta {
    display: flex;
    gap: 0.5rem;
    font-size: 0.9rem;
    color: var(--text-secondary, #999);
  }

  .modal-body {
    padding: 2rem;
  }

  .plugin-description-full {
    margin: 0 0 2rem 0;
    line-height: 1.6;
  }

  .details-section {
    margin-bottom: 2rem;
  }

  .details-section h3 {
    margin: 0 0 1rem 0;
    font-size: 1.1rem;
  }

  .category-badges, .tag-list, .platform-badges {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .badge.category, .badge.platform {
    background: var(--primary, #4a9eff);
    color: white;
  }

  .capability-list {
    margin: 0;
    padding-left: 1.5rem;
    color: var(--text-secondary, #999);
  }

  .links {
    display: flex;
    gap: 1rem;
  }

  .link {
    color: var(--primary, #4a9eff);
    text-decoration: none;
  }

  .link:hover {
    text-decoration: underline;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 1rem;
    background: var(--hover-bg, #3a3a3a);
    border-radius: 8px;
  }

  .stat strong {
    font-size: 1.5rem;
    margin-bottom: 0.25rem;
  }

  .stat span {
    font-size: 0.85rem;
    color: var(--text-secondary, #999);
  }

  .modal-footer {
    padding: 1.5rem 2rem;
    border-top: 1px solid var(--border, #333);
    display: flex;
    justify-content: center;
  }
</style>
