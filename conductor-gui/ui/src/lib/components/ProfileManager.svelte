<!-- Copyright 2025 Amiable -->
<!-- SPDX-License-Identifier: MIT -->

<script>
  import { createEventDispatcher, onMount } from 'svelte';

  /**
   * Props
   */
  export let profiles = []; // Array of AppProfile objects
  export let activeProfileId = null; // Currently active profile ID
  export let currentApp = null; // Current frontmost app info
  export let readonly = false;

  const dispatch = createEventDispatcher();

  /**
   * Local state
   */
  let selectedProfileIndex = null;
  let showAddModal = false;
  let showEditModal = false;
  let showDeleteConfirm = false;
  let profileToDelete = null;
  let editingProfile = null;

  /**
   * Form state for profile editing
   */
  let profileForm = {
    name: '',
    bundle_ids: [],
    is_default: false,
  };

  let bundleIdInput = '';

  /**
   * Select a profile
   */
  function selectProfile(index) {
    selectedProfileIndex = index;
    dispatch('profileSelected', { index, profile: profiles[index] });
  }

  /**
   * Add bundle ID to list
   */
  function addBundleId() {
    if (bundleIdInput.trim() && !profileForm.bundle_ids.includes(bundleIdInput.trim())) {
      profileForm.bundle_ids = [...profileForm.bundle_ids, bundleIdInput.trim()];
      bundleIdInput = '';
    }
  }

  /**
   * Remove bundle ID from list
   */
  function removeBundleId(bundleId) {
    profileForm.bundle_ids = profileForm.bundle_ids.filter(id => id !== bundleId);
  }

  /**
   * Use current app's bundle ID
   */
  function useCurrentApp() {
    if (currentApp && currentApp.bundle_id) {
      bundleIdInput = currentApp.bundle_id;
      addBundleId();
    }
  }

  /**
   * Open add profile modal
   */
  function openAddModal() {
    profileForm = {
      name: '',
      bundle_ids: [],
      is_default: false,
    };
    bundleIdInput = '';
    showAddModal = true;
  }

  /**
   * Open edit profile modal
   */
  function openEditModal(profile) {
    editingProfile = profile;
    profileForm = {
      name: profile.name,
      bundle_ids: [...profile.bundle_ids],
      is_default: profile.is_default || false,
    };
    bundleIdInput = '';
    showEditModal = true;
  }

  /**
   * Save new profile
   */
  function saveNewProfile() {
    if (!profileForm.name.trim()) {
      alert('Profile name is required');
      return;
    }

    dispatch('profileAdded', profileForm);
    showAddModal = false;
  }

  /**
   * Save edited profile
   */
  function saveEditedProfile() {
    if (!profileForm.name.trim()) {
      alert('Profile name is required');
      return;
    }

    dispatch('profileUpdated', {
      profile: editingProfile,
      updates: profileForm,
    });
    showEditModal = false;
    editingProfile = null;
  }

  /**
   * Delete profile with confirmation
   */
  function confirmDelete(profile, index) {
    profileToDelete = { profile, index };
    showDeleteConfirm = true;
  }

  function executeDelete() {
    if (profileToDelete) {
      dispatch('profileDeleted', { index: profileToDelete.index, profile: profileToDelete.profile });
    }
    showDeleteConfirm = false;
    profileToDelete = null;
  }

  function cancelModal() {
    showAddModal = false;
    showEditModal = false;
    showDeleteConfirm = false;
    editingProfile = null;
    profileToDelete = null;
  }

  /**
   * Activate profile
   */
  function activateProfile(profile) {
    dispatch('profileActivated', profile);
  }

  /**
   * Export profile
   */
  function exportProfile(profile) {
    dispatch('profileExported', profile);
  }

  /**
   * Get app names for bundle IDs
   */
  function formatBundleIds(bundleIds) {
    if (!bundleIds || bundleIds.length === 0) return 'No apps assigned';
    return bundleIds.map(id => {
      // Extract app name from bundle ID (e.g., com.apple.Safari → Safari)
      const parts = id.split('.');
      return parts[parts.length - 1];
    }).join(', ');
  }
</script>

<div class="profile-manager">
  <div class="header">
    <div class="title-section">
      <h2>Per-App Profiles</h2>
      {#if currentApp}
        <div class="current-app">
          <span class="app-icon">📱</span>
          <span class="app-name">{currentApp.name}</span>
          <span class="bundle-id">{currentApp.bundle_id}</span>
        </div>
      {/if}
    </div>
    {#if !readonly}
      <button class="btn btn-primary" on:click={openAddModal}>
        + New Profile
      </button>
    {/if}
  </div>

  <div class="profiles-list">
    {#each profiles as profile, index}
      <div
        class="profile-item"
        class:active={profile.id === activeProfileId}
        class:selected={index === selectedProfileIndex}
        on:click={() => selectProfile(index)}
      >
        <div class="profile-header">
          <div class="profile-name-section">
            <span class="profile-name">{profile.name}</span>
            {#if profile.is_default}
              <span class="badge badge-default">Default</span>
            {/if}
            {#if profile.id === activeProfileId}
              <span class="badge badge-active">Active</span>
            {/if}
          </div>
        </div>

        <div class="profile-details">
          <div class="detail-row">
            <span class="label">Apps:</span>
            <span class="value">{formatBundleIds(profile.bundle_ids)}</span>
          </div>
          <div class="detail-row">
            <span class="label">Config:</span>
            <span class="value config-path">{profile.config_path}</span>
          </div>
        </div>

        {#if !readonly}
          <div class="profile-actions">
            <button
              class="btn-icon"
              on:click|stopPropagation={() => activateProfile(profile)}
              title="Activate profile"
              disabled={profile.id === activeProfileId}
            >
              ▶️
            </button>
            <button
              class="btn-icon"
              on:click|stopPropagation={() => openEditModal(profile)}
              title="Edit profile"
            >
              ✏️
            </button>
            <button
              class="btn-icon"
              on:click|stopPropagation={() => exportProfile(profile)}
              title="Export profile"
            >
              💾
            </button>
            <button
              class="btn-icon"
              on:click|stopPropagation={() => confirmDelete(profile, index)}
              title="Delete profile"
              disabled={profile.is_default}
            >
              🗑️
            </button>
          </div>
        {/if}
      </div>
    {/each}

    {#if profiles.length === 0}
      <div class="empty-state">
        <div class="empty-icon">📁</div>
        <p>No profiles configured</p>
        {#if !readonly}
          <button class="btn btn-secondary" on:click={openAddModal}>
            Create First Profile
          </button>
        {/if}
      </div>
    {/if}
  </div>
</div>

<!-- Add Profile Modal -->
{#if showAddModal}
  <div class="modal-overlay" on:click={cancelModal}>
    <div class="modal" on:click|stopPropagation>
      <div class="modal-header">
        <h3>Create New Profile</h3>
        <button class="close-btn" on:click={cancelModal}>×</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label for="profile-name">Profile Name</label>
          <input
            id="profile-name"
            type="text"
            bind:value={profileForm.name}
            placeholder="e.g., Development, Media Player"
          />
        </div>

        <div class="form-group">
          <label>Application Bundle IDs</label>
          <div class="bundle-id-input">
            <input
              type="text"
              bind:value={bundleIdInput}
              placeholder="e.g., com.apple.Safari"
              on:keypress={(e) => e.key === 'Enter' && addBundleId()}
            />
            {#if currentApp}
              <button class="btn btn-secondary" on:click={useCurrentApp}>
                Use Current App
              </button>
            {/if}
            <button class="btn btn-primary" on:click={addBundleId}>
              Add
            </button>
          </div>
          <div class="bundle-ids-list">
            {#each profileForm.bundle_ids as bundleId}
              <div class="bundle-id-tag">
                <span>{bundleId}</span>
                <button class="remove-btn" on:click={() => removeBundleId(bundleId)}>×</button>
              </div>
            {/each}
          </div>
          <small>Add bundle IDs of apps that should use this profile</small>
        </div>

        <div class="form-group">
          <label class="checkbox-label">
            <input type="checkbox" bind:checked={profileForm.is_default} />
            <span>Set as default profile</span>
          </label>
          <small>Default profile is used when no app-specific profile matches</small>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" on:click={cancelModal}>Cancel</button>
        <button class="btn btn-primary" on:click={saveNewProfile}>Create Profile</button>
      </div>
    </div>
  </div>
{/if}

<!-- Edit Profile Modal -->
{#if showEditModal && editingProfile}
  <div class="modal-overlay" on:click={cancelModal}>
    <div class="modal" on:click|stopPropagation>
      <div class="modal-header">
        <h3>Edit Profile</h3>
        <button class="close-btn" on:click={cancelModal}>×</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label for="edit-profile-name">Profile Name</label>
          <input
            id="edit-profile-name"
            type="text"
            bind:value={profileForm.name}
          />
        </div>

        <div class="form-group">
          <label>Application Bundle IDs</label>
          <div class="bundle-id-input">
            <input
              type="text"
              bind:value={bundleIdInput}
              placeholder="e.g., com.apple.Safari"
              on:keypress={(e) => e.key === 'Enter' && addBundleId()}
            />
            {#if currentApp}
              <button class="btn btn-secondary" on:click={useCurrentApp}>
                Use Current App
              </button>
            {/if}
            <button class="btn btn-primary" on:click={addBundleId}>
              Add
            </button>
          </div>
          <div class="bundle-ids-list">
            {#each profileForm.bundle_ids as bundleId}
              <div class="bundle-id-tag">
                <span>{bundleId}</span>
                <button class="remove-btn" on:click={() => removeBundleId(bundleId)}>×</button>
              </div>
            {/each}
          </div>
        </div>

        <div class="form-group">
          <label class="checkbox-label">
            <input type="checkbox" bind:checked={profileForm.is_default} />
            <span>Set as default profile</span>
          </label>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" on:click={cancelModal}>Cancel</button>
        <button class="btn btn-primary" on:click={saveEditedProfile}>Save Changes</button>
      </div>
    </div>
  </div>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteConfirm && profileToDelete}
  <div class="modal-overlay" on:click={cancelModal}>
    <div class="modal modal-small" on:click|stopPropagation>
      <div class="modal-header">
        <h3>Delete Profile?</h3>
        <button class="close-btn" on:click={cancelModal}>×</button>
      </div>
      <div class="modal-body">
        <p>Are you sure you want to delete profile <strong>{profileToDelete.profile.name}</strong>?</p>
        <p class="warning-text">This action cannot be undone.</p>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" on:click={cancelModal}>Cancel</button>
        <button class="btn btn-danger" on:click={executeDelete}>Delete Profile</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .profile-manager {
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
    gap: 1rem;
  }

  .title-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    flex: 1;
  }

  .title-section h2 {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
  }

  .current-app {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--bg-tertiary, #1a1a1a);
    border-radius: 6px;
    font-size: 0.875rem;
  }

  .app-icon {
    font-size: 1.25rem;
  }

  .app-name {
    font-weight: 600;
    color: var(--text-primary, #fff);
  }

  .bundle-id {
    color: var(--text-secondary, #999);
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 0.75rem;
  }

  .profiles-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .profile-item {
    padding: 1rem;
    margin-bottom: 0.5rem;
    background: var(--bg-tertiary, #1a1a1a);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
    border: 2px solid transparent;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .profile-item:hover {
    background: var(--bg-hover, #333);
  }

  .profile-item.selected {
    border-color: var(--accent-color, #3b82f6);
  }

  .profile-item.active {
    background: var(--bg-hover, #333);
    border-color: var(--success-color, #4ade80);
  }

  .profile-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .profile-name-section {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .profile-name {
    font-weight: 600;
    font-size: 1rem;
    color: var(--text-primary, #fff);
  }

  .badge {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .badge-default {
    background: var(--accent-color, #3b82f6);
    color: white;
  }

  .badge-active {
    background: var(--success-color, #4ade80);
    color: white;
  }

  .profile-details {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    font-size: 0.875rem;
  }

  .detail-row {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .detail-row .label {
    color: var(--text-secondary, #999);
    min-width: 60px;
    flex-shrink: 0;
    font-weight: 600;
  }

  .detail-row .value {
    color: var(--text-primary, #fff);
    flex: 1;
    word-break: break-word;
  }

  .config-path {
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 0.75rem;
    color: var(--text-secondary, #999);
  }

  .profile-actions {
    display: flex;
    gap: 0.5rem;
    padding-top: 0.5rem;
    border-top: 1px solid var(--border-color, #444);
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

  .btn-icon:hover:not(:disabled) {
    background: var(--bg-primary, #0a0a0a);
  }

  .btn-icon:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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
    max-width: 600px;
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

  .form-group input[type="text"],
  .form-group input[type="number"] {
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

  .form-group small {
    display: block;
    margin-top: 0.5rem;
    font-size: 0.75rem;
    color: var(--text-secondary, #999);
  }

  .checkbox-label {
    display: flex !important;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }

  .checkbox-label input[type="checkbox"] {
    width: auto;
  }

  .bundle-id-input {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
  }

  .bundle-id-input input {
    flex: 1;
  }

  .bundle-ids-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    min-height: 40px;
    padding: 0.5rem;
    background: var(--bg-primary, #0a0a0a);
    border-radius: 6px;
  }

  .bundle-id-tag {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    background: var(--bg-tertiary, #1a1a1a);
    border: 1px solid var(--border-color, #444);
    border-radius: 6px;
    font-size: 0.875rem;
    color: var(--text-primary, #fff);
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
  }

  .remove-btn {
    background: none;
    border: none;
    color: var(--error-color, #ef4444);
    cursor: pointer;
    font-size: 1.25rem;
    line-height: 1;
    padding: 0;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    transition: all 0.2s;
  }

  .remove-btn:hover {
    background: var(--error-color, #ef4444);
    color: white;
  }

  .warning-text {
    color: var(--error-color, #ef4444);
    font-size: 0.875rem;
  }
</style>
