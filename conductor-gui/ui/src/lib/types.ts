// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

/**
 * Type definitions for MIDIMon GUI
 */

// Daemon status information
export interface DaemonStatus {
  running: boolean;
  connected: boolean;
  lifecycle_state: string | null;
  uptime_secs: number | null;
  events_processed: number | null;
  device: DeviceInfo | null;
  error: string | null;
}

// Device information
export interface DeviceInfo {
  connected: boolean;
  name: string | null;
  port: number | null;
}

// MIDI device for device list
export interface MidiDevice {
  index: number;
  name: string;
  connected: boolean;
}

// Config validation result
export interface ConfigValidation {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

// API error type
export interface ApiError {
  message: string;
  code?: string;
  timestamp: Date;
}

// Store state types
export interface AppState {
  daemonConnected: boolean;
  lastError: ApiError | null;
  loading: boolean;
}

export interface DevicesState {
  devices: MidiDevice[];
  selectedDevice: MidiDevice | null;
  loading: boolean;
  error: string | null;
}

export interface StatusState {
  status: DaemonStatus | null;
  loading: boolean;
  error: string | null;
}
