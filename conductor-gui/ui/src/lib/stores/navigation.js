// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

/**
 * Navigation store for managing current view
 * Uses Svelte 5 runes for state management
 */

import { writable } from 'svelte/store';

/**
 * Available navigation sections
 */
export const SECTIONS = {
  DEVICES: 'devices',
  MODES: 'modes',
  MAPPINGS: 'mappings',
  PLUGINS: 'plugins',
  SETTINGS: 'settings',
};

/**
 * Current active section
 */
export const currentSection = writable(SECTIONS.DEVICES);

/**
 * Navigate to a specific section
 * @param {string} section - Section ID from SECTIONS
 */
export function navigateTo(section) {
  if (Object.values(SECTIONS).includes(section)) {
    currentSection.set(section);
    // Persist to sessionStorage
    try {
      sessionStorage.setItem('midimon-current-section', section);
    } catch (e) {
      console.warn('Failed to persist navigation state:', e);
    }
  }
}

/**
 * Restore navigation state from sessionStorage
 */
export function restoreNavigationState() {
  try {
    const saved = sessionStorage.getItem('midimon-current-section');
    if (saved && Object.values(SECTIONS).includes(saved)) {
      currentSection.set(saved);
    }
  } catch (e) {
    console.warn('Failed to restore navigation state:', e);
  }
}
