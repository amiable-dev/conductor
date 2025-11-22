// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

/**
 * API wrapper for Tauri commands
 * Provides centralized error handling and type-safe access to backend
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * Custom error class for API errors
 */
export class ApiError extends Error {
  constructor(message, code = null) {
    super(message);
    this.name = 'ApiError';
    this.code = code;
    this.timestamp = new Date();
  }
}

/**
 * Daemon API - Commands for interacting with the daemon
 */
export const daemon = {
  /**
   * Get current daemon status
   * @returns {Promise<DaemonStatus>}
   * @throws {ApiError}
   */
  async getStatus() {
    try {
      return await invoke('get_daemon_status');
    } catch (error) {
      throw new ApiError(`Failed to get daemon status: ${error}`, 'DAEMON_STATUS_ERROR');
    }
  },

  /**
   * Reload daemon configuration
   * @returns {Promise<void>}
   * @throws {ApiError}
   */
  async reload() {
    try {
      return await invoke('reload_config');
    } catch (error) {
      throw new ApiError(`Failed to reload config: ${error}`, 'RELOAD_ERROR');
    }
  },

  /**
   * Stop the daemon
   * @returns {Promise<void>}
   * @throws {ApiError}
   */
  async stop() {
    try {
      return await invoke('stop_daemon');
    } catch (error) {
      throw new ApiError(`Failed to stop daemon: ${error}`, 'STOP_ERROR');
    }
  },

  /**
   * Ping the daemon to measure latency
   * @returns {Promise<number>} Latency in milliseconds
   * @throws {ApiError}
   */
  async ping() {
    try {
      return await invoke('ping_daemon');
    } catch (error) {
      throw new ApiError(`Failed to ping daemon: ${error}`, 'PING_ERROR');
    }
  },
};

/**
 * Config API - Commands for managing configuration
 */
export const config = {
  /**
   * Validate the current configuration
   * @returns {Promise<ConfigValidation>}
   * @throws {ApiError}
   */
  async validate() {
    try {
      return await invoke('validate_config');
    } catch (error) {
      throw new ApiError(`Failed to validate config: ${error}`, 'VALIDATE_ERROR');
    }
  },

  /**
   * Get the configuration file path
   * @returns {Promise<string>}
   * @throws {ApiError}
   */
  async getPath() {
    try {
      return await invoke('get_config_path');
    } catch (error) {
      throw new ApiError(`Failed to get config path: ${error}`, 'CONFIG_PATH_ERROR');
    }
  },

  /**
   * Get current configuration as JSON
   * @returns {Promise<object>}
   * @throws {ApiError}
   */
  async get() {
    try {
      return await invoke('get_config');
    } catch (error) {
      throw new ApiError(`Failed to get config: ${error}`, 'GET_CONFIG_ERROR');
    }
  },

  /**
   * Save configuration
   * @param {object} configData - Configuration object to save
   * @returns {Promise<void>}
   * @throws {ApiError}
   */
  async save(configData) {
    try {
      return await invoke('save_config', { config: configData });
    } catch (error) {
      throw new ApiError(`Failed to save config: ${error}`, 'SAVE_CONFIG_ERROR');
    }
  },
};

/**
 * Devices API - Commands for managing MIDI devices
 */
export const devices = {
  /**
   * List available MIDI devices
   * @returns {Promise<MidiDevice[]>}
   * @throws {ApiError}
   */
  async list() {
    try {
      return await invoke('list_midi_devices');
    } catch (error) {
      throw new ApiError(`Failed to list MIDI devices: ${error}`, 'LIST_DEVICES_ERROR');
    }
  },
};

/**
 * MIDI Learn API - Commands for MIDI Learn mode
 */
export const midiLearn = {
  /**
   * Start a MIDI Learn session
   * @param {number} timeoutSecs - Timeout duration in seconds
   * @returns {Promise<string>} Session ID
   * @throws {ApiError}
   */
  async start(timeoutSecs = 10) {
    try {
      return await invoke('start_midi_learn', { timeoutSecs });
    } catch (error) {
      throw new ApiError(`Failed to start MIDI Learn: ${error}`, 'MIDI_LEARN_START_ERROR');
    }
  },

  /**
   * Get the status of the current MIDI Learn session
   * @returns {Promise<string>} Session state (Idle, Waiting, Captured, TimedOut, Cancelled)
   * @throws {ApiError}
   */
  async getStatus() {
    try {
      return await invoke('get_midi_learn_status');
    } catch (error) {
      throw new ApiError(`Failed to get MIDI Learn status: ${error}`, 'MIDI_LEARN_STATUS_ERROR');
    }
  },

  /**
   * Get remaining time for MIDI Learn session
   * @returns {Promise<number>} Remaining seconds
   * @throws {ApiError}
   */
  async getRemaining() {
    try {
      return await invoke('get_midi_learn_remaining');
    } catch (error) {
      throw new ApiError(`Failed to get remaining time: ${error}`, 'MIDI_LEARN_REMAINING_ERROR');
    }
  },

  /**
   * Cancel the current MIDI Learn session
   * @returns {Promise<void>}
   * @throws {ApiError}
   */
  async cancel() {
    try {
      return await invoke('cancel_midi_learn');
    } catch (error) {
      throw new ApiError(`Failed to cancel MIDI Learn: ${error}`, 'MIDI_LEARN_CANCEL_ERROR');
    }
  },

  /**
   * Get the result of the MIDI Learn session
   * @returns {Promise<MidiLearnResult|null>}
   * @throws {ApiError}
   */
  async getResult() {
    try {
      return await invoke('get_midi_learn_result');
    } catch (error) {
      throw new ApiError(`Failed to get MIDI Learn result: ${error}`, 'MIDI_LEARN_RESULT_ERROR');
    }
  },
};

/**
 * Templates API - Commands for managing device templates
 */
export const templates = {
  /**
   * List all available device templates
   * @returns {Promise<DeviceTemplate[]>}
   * @throws {ApiError}
   */
  async list() {
    try {
      return await invoke('list_device_templates');
    } catch (error) {
      throw new ApiError(`Failed to list device templates: ${error}`, 'LIST_TEMPLATES_ERROR');
    }
  },

  /**
   * Get a specific device template by ID
   * @param {string} templateId - Template identifier
   * @returns {Promise<DeviceTemplate>}
   * @throws {ApiError}
   */
  async get(templateId) {
    try {
      return await invoke('get_device_template', { templateId });
    } catch (error) {
      throw new ApiError(`Failed to get device template: ${error}`, 'GET_TEMPLATE_ERROR');
    }
  },

  /**
   * Create config from a device template
   * @param {string} templateId - Template identifier
   * @returns {Promise<void>}
   * @throws {ApiError}
   */
  async createConfig(templateId) {
    try {
      return await invoke('create_config_from_template', { templateId });
    } catch (error) {
      throw new ApiError(`Failed to create config from template: ${error}`, 'CREATE_CONFIG_ERROR');
    }
  },

  /**
   * Find templates matching MIDI device info
   * @param {object} midiInfo - MIDI device information
   * @returns {Promise<DeviceTemplate[]>}
   * @throws {ApiError}
   */
  async findByMidi(midiInfo) {
    try {
      return await invoke('find_templates_by_midi', { midiInfo });
    } catch (error) {
      throw new ApiError(`Failed to find templates by MIDI: ${error}`, 'FIND_TEMPLATES_ERROR');
    }
  },

  /**
   * Get template categories
   * @returns {Promise<string[]>}
   * @throws {ApiError}
   */
  async getCategories() {
    try {
      return await invoke('get_template_categories');
    } catch (error) {
      throw new ApiError(`Failed to get template categories: ${error}`, 'GET_CATEGORIES_ERROR');
    }
  },

  /**
   * List templates by category
   * @param {string} category - Category name
   * @returns {Promise<DeviceTemplate[]>}
   * @throws {ApiError}
   */
  async listByCategory(category) {
    try {
      return await invoke('list_templates_by_category', { category });
    } catch (error) {
      throw new ApiError(`Failed to list templates by category: ${error}`, 'LIST_BY_CATEGORY_ERROR');
    }
  },
};

/**
 * MIDI Output API - Commands for MIDI output functionality (v2.1)
 */
export const midiOutput = {
  /**
   * List all available MIDI output ports
   * @returns {Promise<MidiOutputPort[]>}
   * @throws {ApiError}
   */
  async listPorts() {
    try {
      return await invoke('list_midi_output_ports');
    } catch (error) {
      throw new ApiError(`Failed to list MIDI output ports: ${error}`, 'LIST_MIDI_PORTS_ERROR');
    }
  },

  /**
   * Test MIDI output by sending a test message
   * @param {string} portName - Name of the MIDI port
   * @param {string} messageType - Type of message ('note_on', 'note_off', 'cc')
   * @param {number} channel - MIDI channel (0-15)
   * @param {Object} options - Optional parameters
   * @param {number} options.note - MIDI note number (0-127)
   * @param {number} options.velocity - Note velocity (0-127)
   * @param {number} options.ccNumber - CC number (0-127)
   * @param {number} options.ccValue - CC value (0-127)
   * @returns {Promise<string>} Success message
   * @throws {ApiError}
   */
  async testOutput(portName, messageType, channel, options = {}) {
    try {
      return await invoke('test_midi_output', {
        portName,
        messageType,
        channel,
        note: options.note,
        velocity: options.velocity,
        ccNumber: options.ccNumber,
        ccValue: options.ccValue,
      });
    } catch (error) {
      throw new ApiError(`Failed to test MIDI output: ${error}`, 'TEST_MIDI_OUTPUT_ERROR');
    }
  },

  /**
   * Validate a SendMIDI action configuration
   * @param {Object} config - SendMIDI action configuration
   * @returns {Promise<ValidationResult>}
   * @throws {ApiError}
   */
  async validateAction(config) {
    try {
      return await invoke('validate_send_midi_action', { config });
    } catch (error) {
      throw new ApiError(`Failed to validate SendMIDI action: ${error}`, 'VALIDATE_MIDI_ACTION_ERROR');
    }
  },
};

/**
 * Export all APIs as a single default object
 */
export default {
  daemon,
  config,
  devices,
  midiLearn,
  templates,
  midiOutput,
  ApiError,
};
