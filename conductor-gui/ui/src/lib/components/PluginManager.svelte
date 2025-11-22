<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  // Plugin list state
  let plugins = [];
  let loadedPlugins = [];
  let selectedPlugin = null;
  let loading = true;
  let error = null;

  // Filter/search state
  let searchQuery = '';
  let filterType = 'all'; // all, loaded, available

  // Plugin action state
  let actionInProgress = false;
  let actionMessage = '';

  // Load plugins on mount
  onMount(async () => {
    await loadPlugins();
  });

  async function loadPlugins() {
    loading = true;
    error = null;

    try {
      // Discover all available plugins
      const availableCount = await invoke('plugin_discover');
      const availableList = await invoke('plugin_list_available');
      const loadedList = await invoke('plugin_list_loaded');

      // Get detailed info for each plugin
      plugins = await Promise.all(
        availableList.map(async (name) => {
          try {
            const metadata = await invoke('plugin_get_metadata', { pluginName: name });
            const isLoaded = loadedList.includes(name);
            const stats = isLoaded ? await invoke('plugin_get_stats', { pluginName: name }) : null;

            return {
              name,
              ...metadata,
              loaded: isLoaded,
              enabled: metadata.enabled ?? true,
              stats,
            };
          } catch (e) {
            console.error(`Failed to get metadata for ${name}:`, e);
            return null;
          }
        })
      );

      plugins = plugins.filter(p => p !== null);
      loadedPlugins = loadedList;

      console.log(`Discovered ${plugins.length} plugins, ${loadedPlugins.length} loaded`);
    } catch (e) {
      error = `Failed to load plugins: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  async function loadPlugin(pluginName) {
    actionInProgress = true;
    actionMessage = `Loading ${pluginName}...`;

    try {
      await invoke('plugin_load', { pluginName });
      actionMessage = `Successfully loaded ${pluginName}`;
      setTimeout(() => actionMessage = '', 3000);
      await loadPlugins(); // Refresh
    } catch (e) {
      error = `Failed to load ${pluginName}: ${e}`;
      actionMessage = '';
    } finally {
      actionInProgress = false;
    }
  }

  async function unloadPlugin(pluginName) {
    actionInProgress = true;
    actionMessage = `Unloading ${pluginName}...`;

    try {
      await invoke('plugin_unload', { pluginName });
      actionMessage = `Successfully unloaded ${pluginName}`;
      setTimeout(() => actionMessage = '', 3000);
      await loadPlugins(); // Refresh
    } catch (e) {
      error = `Failed to unload ${pluginName}: ${e}`;
      actionMessage = '';
    } finally {
      actionInProgress = false;
    }
  }

  async function enablePlugin(pluginName) {
    actionInProgress = true;

    try {
      await invoke('plugin_enable', { pluginName });
      await loadPlugins(); // Refresh
    } catch (e) {
      error = `Failed to enable ${pluginName}: ${e}`;
    } finally {
      actionInProgress = false;
    }
  }

  async function disablePlugin(pluginName) {
    actionInProgress = true;

    try {
      await invoke('plugin_disable', { pluginName });
      await loadPlugins(); // Refresh
    } catch (e) {
      error = `Failed to disable ${pluginName}: ${e}`;
    } finally {
      actionInProgress = false;
    }
  }

  async function grantCapability(pluginName, capability) {
    actionInProgress = true;

    try {
      await invoke('plugin_grant_capability', { pluginName, capability });
      actionMessage = `Granted ${capability} to ${pluginName}`;
      setTimeout(() => actionMessage = '', 3000);
      await loadPlugins(); // Refresh
    } catch (e) {
      error = `Failed to grant capability: ${e}`;
    } finally {
      actionInProgress = false;
    }
  }

  async function revokeCapability(pluginName, capability) {
    actionInProgress = true;

    try {
      await invoke('plugin_revoke_capability', { pluginName, capability });
      actionMessage = `Revoked ${capability} from ${pluginName}`;
      setTimeout(() => actionMessage = '', 3000);
      await loadPlugins(); // Refresh
    } catch (e) {
      error = `Failed to revoke capability: ${e}`;
    } finally {
      actionInProgress = false;
    }
  }

  // Filtered plugins based on search and filter type
  $: filteredPlugins = plugins.filter(p => {
    // Search filter
    if (searchQuery && !p.name.toLowerCase().includes(searchQuery.toLowerCase()) &&
        !p.description.toLowerCase().includes(searchQuery.toLowerCase())) {
      return false;
    }

    // Type filter
    if (filterType === 'loaded' && !p.loaded) return false;
    if (filterType === 'available' && p.loaded) return false;

    return true;
  });

  function selectPlugin(plugin) {
    selectedPlugin = plugin;
  }

  function getRiskLevelColor(riskLevel) {
    switch (riskLevel) {
      case 'Low': return 'green';
      case 'Medium': return 'yellow';
      case 'High': return 'red';
      default: return 'gray';
    }
  }

  function getRiskLevelEmoji(riskLevel) {
    switch (riskLevel) {
      case 'Low': return '✅';
      case 'Medium': return '⚠️';
      case 'High': return '🚨';
      default: return '❓';
    }
  }
</script>

<div class="plugin-manager">
  <div class="header">
    <h2>Plugin Manager</h2>
    <button on:click={loadPlugins} disabled={loading || actionInProgress}>
      {loading ? 'Loading...' : 'Refresh'}
    </button>
  </div>

  {#if error}
    <div class="error-banner">
      <span>⚠️ {error}</span>
      <button on:click={() => error = null}>✕</button>
    </div>
  {/if}

  {#if actionMessage}
    <div class="success-banner">
      <span>✓ {actionMessage}</span>
    </div>
  {/if}

  <div class="toolbar">
    <input
      type="text"
      placeholder="Search plugins..."
      bind:value={searchQuery}
      class="search-input"
    />

    <div class="filter-buttons">
      <button
        class:active={filterType === 'all'}
        on:click={() => filterType = 'all'}
      >
        All ({plugins.length})
      </button>
      <button
        class:active={filterType === 'loaded'}
        on:click={() => filterType = 'loaded'}
      >
        Loaded ({loadedPlugins.length})
      </button>
      <button
        class:active={filterType === 'available'}
        on:click={() => filterType = 'available'}
      >
        Available ({plugins.length - loadedPlugins.length})
      </button>
    </div>
  </div>

  <div class="content">
    <div class="plugin-list">
      {#if loading}
        <div class="loading-state">
          <div class="spinner"></div>
          <p>Loading plugins...</p>
        </div>
      {:else if filteredPlugins.length === 0}
        <div class="empty-state">
          <p>No plugins found</p>
          {#if searchQuery}
            <button on:click={() => searchQuery = ''}>Clear search</button>
          {/if}
        </div>
      {:else}
        {#each filteredPlugins as plugin}
          <div
            class="plugin-item"
            class:selected={selectedPlugin?.name === plugin.name}
            class:loaded={plugin.loaded}
            on:click={() => selectPlugin(plugin)}
          >
            <div class="plugin-header">
              <h3>{plugin.name}</h3>
              <span class="version">v{plugin.version}</span>
              {#if plugin.loaded}
                <span class="badge loaded-badge">Loaded</span>
              {/if}
              {#if plugin.loaded && !plugin.enabled}
                <span class="badge disabled-badge">Disabled</span>
              {/if}
            </div>
            <p class="description">{plugin.description}</p>
            <div class="plugin-meta">
              <span>📦 {plugin.author}</span>
              <span>📄 {plugin.license}</span>
              {#if plugin.stats}
                <span>🔢 {plugin.stats.executions} calls</span>
              {/if}
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <div class="plugin-details">
      {#if selectedPlugin}
        <div class="details-header">
          <h2>{selectedPlugin.name}</h2>
          <span class="version-large">v{selectedPlugin.version}</span>
        </div>

        <p class="description-full">{selectedPlugin.description}</p>

        <div class="details-section">
          <h3>Information</h3>
          <dl>
            <dt>Author:</dt>
            <dd>{selectedPlugin.author}</dd>
            <dt>License:</dt>
            <dd>{selectedPlugin.license}</dd>
            <dt>Type:</dt>
            <dd>{selectedPlugin.plugin_type}</dd>
            {#if selectedPlugin.homepage}
              <dt>Homepage:</dt>
              <dd><a href={selectedPlugin.homepage} target="_blank">{selectedPlugin.homepage}</a></dd>
            {/if}
          </dl>
        </div>

        <div class="details-section">
          <h3>Capabilities</h3>
          {#if selectedPlugin.capabilities && selectedPlugin.capabilities.length > 0}
            <div class="capabilities-list">
              {#each selectedPlugin.capabilities as capability}
                <div class="capability-item">
                  <div class="capability-info">
                    <span class="capability-emoji">{getRiskLevelEmoji(capability.risk_level)}</span>
                    <span class="capability-name">{capability.name}</span>
                    <span class="risk-badge" style="color: {getRiskLevelColor(capability.risk_level)}">
                      {capability.risk_level} Risk
                    </span>
                  </div>
                  <p class="capability-desc">{capability.description}</p>
                  {#if selectedPlugin.loaded}
                    {#if selectedPlugin.granted_capabilities?.includes(capability.name)}
                      <button
                        class="revoke-btn"
                        on:click={() => revokeCapability(selectedPlugin.name, capability.name)}
                        disabled={actionInProgress}
                      >
                        Revoke
                      </button>
                    {:else}
                      <button
                        class="grant-btn"
                        on:click={() => grantCapability(selectedPlugin.name, capability.name)}
                        disabled={actionInProgress}
                      >
                        Grant
                      </button>
                    {/if}
                  {/if}
                </div>
              {/each}
            </div>
          {:else}
            <p class="empty-message">No capabilities required</p>
          {/if}
        </div>

        {#if selectedPlugin.stats}
          <div class="details-section">
            <h3>Statistics</h3>
            <dl>
              <dt>Total Executions:</dt>
              <dd>{selectedPlugin.stats.executions}</dd>
              <dt>Failures:</dt>
              <dd>{selectedPlugin.stats.failures}</dd>
              <dt>Success Rate:</dt>
              <dd>
                {selectedPlugin.stats.executions > 0
                  ? ((1 - selectedPlugin.stats.failures / selectedPlugin.stats.executions) * 100).toFixed(1)
                  : 0}%
              </dd>
              <dt>Avg Execution Time:</dt>
              <dd>{(selectedPlugin.stats.avg_execution_time_us / 1000).toFixed(2)} ms</dd>
              {#if selectedPlugin.stats.last_execution_ms > 0}
                <dt>Last Executed:</dt>
                <dd>{new Date(selectedPlugin.stats.last_execution_ms).toLocaleString()}</dd>
              {/if}
            </dl>
          </div>
        {/if}

        <div class="actions">
          {#if selectedPlugin.loaded}
            <button
              class="btn btn-danger"
              on:click={() => unloadPlugin(selectedPlugin.name)}
              disabled={actionInProgress}
            >
              Unload Plugin
            </button>
            {#if selectedPlugin.enabled}
              <button
                class="btn btn-warning"
                on:click={() => disablePlugin(selectedPlugin.name)}
                disabled={actionInProgress}
              >
                Disable
              </button>
            {:else}
              <button
                class="btn btn-success"
                on:click={() => enablePlugin(selectedPlugin.name)}
                disabled={actionInProgress}
              >
                Enable
              </button>
            {/if}
          {:else}
            <button
              class="btn btn-primary"
              on:click={() => loadPlugin(selectedPlugin.name)}
              disabled={actionInProgress}
            >
              Load Plugin
            </button>
          {/if}
        </div>
      {:else}
        <div class="empty-details">
          <p>Select a plugin to view details</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .plugin-manager {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 1rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .error-banner {
    background-color: #fee;
    border: 1px solid #fcc;
    border-radius: 4px;
    padding: 0.75rem;
    margin-bottom: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: #c00;
  }

  .success-banner {
    background-color: #efe;
    border: 1px solid #cfc;
    border-radius: 4px;
    padding: 0.75rem;
    margin-bottom: 1rem;
    color: #060;
  }

  .toolbar {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .search-input {
    flex: 1;
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  .filter-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .filter-buttons button {
    padding: 0.5rem 1rem;
    border: 1px solid #ccc;
    border-radius: 4px;
    background: white;
    cursor: pointer;
  }

  .filter-buttons button.active {
    background: #007bff;
    color: white;
    border-color: #007bff;
  }

  .content {
    display: grid;
    grid-template-columns: 1fr 2fr;
    gap: 1rem;
    flex: 1;
    overflow: hidden;
  }

  .plugin-list {
    border: 1px solid #ddd;
    border-radius: 4px;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .plugin-item {
    padding: 1rem;
    border: 1px solid #eee;
    border-radius: 4px;
    margin-bottom: 0.5rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .plugin-item:hover {
    background: #f9f9f9;
  }

  .plugin-item.selected {
    border-color: #007bff;
    background: #f0f8ff;
  }

  .plugin-item.loaded {
    border-left: 4px solid #28a745;
  }

  .plugin-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .plugin-header h3 {
    margin: 0;
    font-size: 1rem;
  }

  .version {
    font-size: 0.85rem;
    color: #666;
  }

  .badge {
    padding: 0.25rem 0.5rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .loaded-badge {
    background: #28a745;
    color: white;
  }

  .disabled-badge {
    background: #ffc107;
    color: #000;
  }

  .description {
    margin: 0;
    font-size: 0.9rem;
    color: #666;
  }

  .plugin-meta {
    display: flex;
    gap: 1rem;
    margin-top: 0.5rem;
    font-size: 0.8rem;
    color: #888;
  }

  .plugin-details {
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 1.5rem;
    overflow-y: auto;
  }

  .details-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .version-large {
    font-size: 1.2rem;
    color: #666;
  }

  .description-full {
    color: #444;
    line-height: 1.5;
    margin-bottom: 1.5rem;
  }

  .details-section {
    margin-bottom: 1.5rem;
  }

  .details-section h3 {
    margin-top: 0;
    margin-bottom: 0.75rem;
    font-size: 1.1rem;
  }

  dl {
    margin: 0;
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 0.5rem 1rem;
  }

  dt {
    font-weight: 600;
    color: #666;
  }

  dd {
    margin: 0;
  }

  .capabilities-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .capability-item {
    padding: 1rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    background: #fafafa;
  }

  .capability-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .capability-emoji {
    font-size: 1.2rem;
  }

  .capability-name {
    font-weight: 600;
  }

  .risk-badge {
    padding: 0.25rem 0.5rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    background: rgba(0, 0, 0, 0.05);
  }

  .capability-desc {
    margin: 0.5rem 0;
    font-size: 0.9rem;
    color: #666;
  }

  .actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 1.5rem;
  }

  .btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #007bff;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #0056b3;
  }

  .btn-success {
    background: #28a745;
    color: white;
  }

  .btn-success:hover:not(:disabled) {
    background: #1e7e34;
  }

  .btn-warning {
    background: #ffc107;
    color: #000;
  }

  .btn-warning:hover:not(:disabled) {
    background: #e0a800;
  }

  .btn-danger {
    background: #dc3545;
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background: #c82333;
  }

  .grant-btn, .revoke-btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    font-weight: 600;
  }

  .grant-btn {
    background: #28a745;
    color: white;
  }

  .revoke-btn {
    background: #dc3545;
    color: white;
  }

  .empty-state, .empty-details {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    color: #999;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #f3f3f3;
    border-top: 4px solid #007bff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
</style>
