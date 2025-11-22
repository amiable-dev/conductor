<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  interface Settings {
    autoStart: boolean;
    minimizeToTray: boolean;
    theme: 'light' | 'dark' | 'system';
    eventBufferSize: number;
    autoSaveConfigs: boolean;
    checkForUpdates: boolean;
    logLevel: 'error' | 'warn' | 'info' | 'debug';
    midiLearnTimeout: number;
  }

  let settings: Settings = {
    autoStart: false,
    minimizeToTray: true,
    theme: 'system',
    eventBufferSize: 1000,
    autoSaveConfigs: true,
    checkForUpdates: true,
    logLevel: 'info',
    midiLearnTimeout: 10,
  };

  let originalSettings: Settings = { ...settings };
  let hasChanges = false;
  let saving = false;
  let error: string | null = null;
  let successMessage: string | null = null;

  onMount(async () => {
    await loadSettings();
  });

  async function loadSettings() {
    try {
      // Load settings from backend
      // For now, using defaults
      // TODO: Add get_settings command
      settings = { ...settings };
      originalSettings = { ...settings };
    } catch (e) {
      error = `Failed to load settings: ${e}`;
      console.error(error);
    }
  }

  async function saveSettings() {
    saving = true;
    error = null;
    successMessage = null;

    try {
      // Save settings to backend
      // TODO: Add save_settings command
      await new Promise(resolve => setTimeout(resolve, 500)); // Simulate save

      originalSettings = { ...settings };
      hasChanges = false;
      successMessage = 'Settings saved successfully';

      setTimeout(() => {
        successMessage = null;
      }, 3000);
    } catch (e) {
      error = `Failed to save settings: ${e}`;
      console.error(error);
    } finally {
      saving = false;
    }
  }

  function resetSettings() {
    settings = { ...originalSettings };
    hasChanges = false;
  }

  function restoreDefaults() {
    settings = {
      autoStart: false,
      minimizeToTray: true,
      theme: 'system',
      eventBufferSize: 1000,
      autoSaveConfigs: true,
      checkForUpdates: true,
      logLevel: 'info',
      midiLearnTimeout: 10,
    };
    hasChanges = true;
  }

  $: hasChanges = JSON.stringify(settings) !== JSON.stringify(originalSettings);
</script>

<div class="settings-panel">
  <div class="header">
    <h2>Settings</h2>
    {#if successMessage}
      <div class="success-message">{successMessage}</div>
    {/if}
    {#if error}
      <div class="error-message">{error}</div>
    {/if}
  </div>

  <div class="settings-content">
    <!-- General Settings -->
    <section class="settings-section">
      <h3>General</h3>

      <div class="setting-item">
        <label class="setting-label">
          <input type="checkbox" bind:checked={settings.autoStart} />
          <div class="setting-text">
            <div class="setting-title">Start on login</div>
            <div class="setting-description">
              Automatically launch Conductor when you log in to your computer
            </div>
          </div>
        </label>
      </div>

      <div class="setting-item">
        <label class="setting-label">
          <input type="checkbox" bind:checked={settings.minimizeToTray} />
          <div class="setting-text">
            <div class="setting-title">Minimize to system tray</div>
            <div class="setting-description">
              Keep Conductor running in the background when window is closed
            </div>
          </div>
        </label>
      </div>

      <div class="setting-item">
        <label class="setting-label">
          <input type="checkbox" bind:checked={settings.autoSaveConfigs} />
          <div class="setting-text">
            <div class="setting-title">Auto-save configurations</div>
            <div class="setting-description">
              Automatically save configuration changes without prompting
            </div>
          </div>
        </label>
      </div>

      <div class="setting-item">
        <label class="setting-label">
          <input type="checkbox" bind:checked={settings.checkForUpdates} />
          <div class="setting-text">
            <div class="setting-title">Check for updates</div>
            <div class="setting-description">
              Automatically check for new versions on startup
            </div>
          </div>
        </label>
      </div>
    </section>

    <!-- Appearance Settings -->
    <section class="settings-section">
      <h3>Appearance</h3>

      <div class="setting-item">
        <div class="setting-label-row">
          <div class="setting-text">
            <div class="setting-title">Theme</div>
            <div class="setting-description">
              Choose your preferred color theme
            </div>
          </div>
          <select bind:value={settings.theme} class="setting-select">
            <option value="light">Light</option>
            <option value="dark">Dark</option>
            <option value="system">System</option>
          </select>
        </div>
      </div>
    </section>

    <!-- MIDI Settings -->
    <section class="settings-section">
      <h3>MIDI</h3>

      <div class="setting-item">
        <div class="setting-label-row">
          <div class="setting-text">
            <div class="setting-title">Event buffer size</div>
            <div class="setting-description">
              Maximum number of events to keep in live console
            </div>
          </div>
          <input
            type="number"
            bind:value={settings.eventBufferSize}
            min="100"
            max="10000"
            step="100"
            class="setting-input"
          />
        </div>
      </div>

      <div class="setting-item">
        <div class="setting-label-row">
          <div class="setting-text">
            <div class="setting-title">MIDI Learn timeout</div>
            <div class="setting-description">
              Seconds to wait for MIDI input during MIDI Learn mode
            </div>
          </div>
          <input
            type="number"
            bind:value={settings.midiLearnTimeout}
            min="5"
            max="60"
            step="5"
            class="setting-input"
          />
        </div>
      </div>
    </section>

    <!-- Advanced Settings -->
    <section class="settings-section">
      <h3>Advanced</h3>

      <div class="setting-item">
        <div class="setting-label-row">
          <div class="setting-text">
            <div class="setting-title">Log level</div>
            <div class="setting-description">
              Verbosity of logging for troubleshooting
            </div>
          </div>
          <select bind:value={settings.logLevel} class="setting-select">
            <option value="error">Error</option>
            <option value="warn">Warning</option>
            <option value="info">Info</option>
            <option value="debug">Debug</option>
          </select>
        </div>
      </div>
    </section>

    <!-- About Section -->
    <section class="settings-section">
      <h3>About</h3>

      <div class="about-content">
        <div class="app-info">
          <div class="app-name">Conductor GUI</div>
          <div class="app-version">Version 2.0.0</div>
          <div class="app-description">
            Visual configuration interface for Conductor daemon
          </div>
        </div>

        <div class="links">
          <a href="https://github.com/amiable/conductor" target="_blank" rel="noopener noreferrer">
            GitHub Repository
          </a>
          <a href="https://github.com/amiable/conductor/issues" target="_blank" rel="noopener noreferrer">
            Report an Issue
          </a>
          <a href="https://github.com/amiable/conductor/wiki" target="_blank" rel="noopener noreferrer">
            Documentation
          </a>
        </div>

        <div class="copyright">
          Copyright © 2025 Amiable<br />
          Licensed under MIT License
        </div>
      </div>
    </section>
  </div>

  <div class="settings-footer">
    <button
      class="btn btn-secondary"
      on:click={restoreDefaults}
      disabled={saving}
    >
      Restore Defaults
    </button>

    <div class="footer-actions">
      <button
        class="btn btn-secondary"
        on:click={resetSettings}
        disabled={!hasChanges || saving}
      >
        Cancel
      </button>

      <button
        class="btn btn-primary"
        on:click={saveSettings}
        disabled={!hasChanges || saving}
      >
        {saving ? 'Saving...' : 'Save Changes'}
      </button>
    </div>
  </div>
</div>

<style>
  .settings-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-primary, #fff);
  }

  .header {
    padding: 1.5rem;
    border-bottom: 1px solid var(--border, #d1d5db);
  }

  .header h2 {
    margin: 0 0 0.5rem 0;
    font-size: 1.5rem;
    color: var(--text-primary, #333);
  }

  .success-message {
    margin-top: 0.75rem;
    padding: 0.75rem 1rem;
    background: rgba(16, 185, 129, 0.1);
    border-left: 3px solid var(--success, #10b981);
    border-radius: 0.25rem;
    color: var(--success, #10b981);
    font-size: 0.875rem;
  }

  .error-message {
    margin-top: 0.75rem;
    padding: 0.75rem 1rem;
    background: rgba(220, 38, 38, 0.1);
    border-left: 3px solid var(--error, #dc2626);
    border-radius: 0.25rem;
    color: var(--error, #dc2626);
    font-size: 0.875rem;
  }

  .settings-content {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
  }

  .settings-section {
    margin-bottom: 2rem;
  }

  .settings-section:last-child {
    margin-bottom: 0;
  }

  .settings-section h3 {
    margin: 0 0 1rem 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary, #333);
  }

  .setting-item {
    padding: 1rem 0;
    border-bottom: 1px solid var(--border, #e5e7eb);
  }

  .setting-item:last-child {
    border-bottom: none;
  }

  .setting-label {
    display: flex;
    gap: 0.75rem;
    cursor: pointer;
  }

  .setting-label input[type="checkbox"] {
    margin-top: 0.125rem;
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .setting-label-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  .setting-text {
    flex: 1;
  }

  .setting-title {
    font-weight: 500;
    color: var(--text-primary, #333);
    margin-bottom: 0.25rem;
  }

  .setting-description {
    font-size: 0.875rem;
    color: var(--text-secondary, #666);
    line-height: 1.4;
  }

  .setting-select,
  .setting-input {
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--border, #d1d5db);
    border-radius: 0.375rem;
    font-size: 0.875rem;
    background: var(--bg-secondary, #fff);
    min-width: 150px;
  }

  .setting-input {
    width: 100px;
    text-align: right;
  }

  .about-content {
    padding: 1rem 0;
  }

  .app-info {
    margin-bottom: 1.5rem;
  }

  .app-name {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary, #333);
    margin-bottom: 0.25rem;
  }

  .app-version {
    font-size: 0.875rem;
    color: var(--text-secondary, #666);
    margin-bottom: 0.5rem;
  }

  .app-description {
    font-size: 0.875rem;
    color: var(--text-secondary, #666);
    line-height: 1.5;
  }

  .links {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
  }

  .links a {
    color: var(--primary, #3b82f6);
    text-decoration: none;
    font-size: 0.875rem;
  }

  .links a:hover {
    text-decoration: underline;
  }

  .copyright {
    font-size: 0.8125rem;
    color: var(--text-tertiary, #9ca3af);
    line-height: 1.5;
  }

  .settings-footer {
    display: flex;
    justify-content: space-between;
    padding: 1rem 1.5rem;
    border-top: 1px solid var(--border, #d1d5db);
    background: var(--bg-secondary, #f9fafb);
  }

  .footer-actions {
    display: flex;
    gap: 0.75rem;
  }

  .btn {
    padding: 0.625rem 1.25rem;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary {
    background: var(--primary, #3b82f6);
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--primary-dark, #2563eb);
  }

  .btn-secondary {
    background: var(--bg-secondary, #e5e7eb);
    color: var(--text-primary, #333);
  }

  .btn-secondary:hover:not(:disabled) {
    background: var(--border, #d1d5db);
  }
</style>
