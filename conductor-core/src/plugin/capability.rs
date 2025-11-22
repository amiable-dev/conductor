// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Plugin capability flags for permission system

use serde::{Deserialize, Serialize};

/// Plugin capability flags for permission system
///
/// Capabilities represent permissions that plugins require to function.
/// The user is prompted to approve dangerous capabilities before the
/// plugin can execute.
///
/// # Example
///
/// ```rust
/// use conductor_core::plugin::Capability;
///
/// let caps = vec![Capability::Network, Capability::Filesystem];
/// assert!(caps.contains(&Capability::Network));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Capability {
    /// Can make HTTP/HTTPS requests
    ///
    /// Required for plugins that communicate with web services,
    /// APIs, or external servers.
    ///
    /// **Risk**: Network access (data exfiltration)
    Network,

    /// Can read/write files
    ///
    /// Required for plugins that save state, cache data, or
    /// interact with the filesystem.
    ///
    /// **Risk**: File system access (data theft, file corruption)
    Filesystem,

    /// Can access audio devices
    ///
    /// Required for plugins that play sounds, record audio, or
    /// control audio routing.
    ///
    /// **Risk**: Audio device control
    Audio,

    /// Can send/receive MIDI
    ///
    /// Required for plugins that send MIDI output or receive
    /// MIDI input from additional devices.
    ///
    /// **Risk**: MIDI device control
    Midi,

    /// Can spawn processes
    ///
    /// Required for plugins that execute shell commands or
    /// launch external applications.
    ///
    /// **Risk**: Arbitrary code execution
    Subprocess,

    /// Can control system (volume, display, etc.)
    ///
    /// Required for plugins that adjust system settings like
    /// volume, brightness, or power management.
    ///
    /// **Risk**: System control
    SystemControl,
}

impl Capability {
    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            Capability::Network => "Network Access",
            Capability::Filesystem => "Filesystem Access",
            Capability::Audio => "Audio Device Access",
            Capability::Midi => "MIDI Device Access",
            Capability::Subprocess => "Process Execution",
            Capability::SystemControl => "System Control",
        }
    }

    /// Get description of what this capability allows
    pub fn description(&self) -> &'static str {
        match self {
            Capability::Network => "Make HTTP/HTTPS requests to external servers",
            Capability::Filesystem => "Read and write files on your computer",
            Capability::Audio => "Access and control audio devices",
            Capability::Midi => "Send and receive MIDI messages",
            Capability::Subprocess => "Execute shell commands and launch applications",
            Capability::SystemControl => "Control system settings (volume, display, etc.)",
        }
    }

    /// Get risk level (Low, Medium, High)
    pub fn risk_level(&self) -> RiskLevel {
        match self {
            Capability::Network => RiskLevel::Medium,
            Capability::Filesystem => RiskLevel::High,
            Capability::Audio => RiskLevel::Low,
            Capability::Midi => RiskLevel::Low,
            Capability::Subprocess => RiskLevel::High,
            Capability::SystemControl => RiskLevel::Medium,
        }
    }

    /// List all capabilities
    pub fn all() -> Vec<Capability> {
        vec![
            Capability::Network,
            Capability::Filesystem,
            Capability::Audio,
            Capability::Midi,
            Capability::Subprocess,
            Capability::SystemControl,
        ]
    }
}

/// Risk level for capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

impl RiskLevel {
    /// Check if this risk level is safe for auto-granting
    ///
    /// Safe capabilities (Low risk) can be auto-granted without user confirmation.
    /// Medium and High risk capabilities require explicit user approval.
    pub fn is_safe(&self) -> bool {
        matches!(self, RiskLevel::Low)
    }

    /// Get color for UI display (CSS class or color name)
    pub fn color(&self) -> &'static str {
        match self {
            RiskLevel::Low => "green",
            RiskLevel::Medium => "yellow",
            RiskLevel::High => "red",
        }
    }

    /// Get emoji indicator
    pub fn emoji(&self) -> &'static str {
        match self {
            RiskLevel::Low => "✅",
            RiskLevel::Medium => "⚠️",
            RiskLevel::High => "🚨",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_metadata() {
        let cap = Capability::Network;
        assert_eq!(cap.name(), "Network Access");
        assert!(!cap.description().is_empty());
        assert_eq!(cap.risk_level(), RiskLevel::Medium);
    }

    #[test]
    fn test_all_capabilities() {
        let caps = Capability::all();
        assert_eq!(caps.len(), 6);
        assert!(caps.contains(&Capability::Network));
        assert!(caps.contains(&Capability::Filesystem));
    }

    #[test]
    fn test_risk_levels() {
        assert_eq!(Capability::Audio.risk_level(), RiskLevel::Low);
        assert_eq!(Capability::Network.risk_level(), RiskLevel::Medium);
        assert_eq!(Capability::Filesystem.risk_level(), RiskLevel::High);
        assert_eq!(Capability::Subprocess.risk_level(), RiskLevel::High);
    }

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Low < RiskLevel::Medium);
        assert!(RiskLevel::Medium < RiskLevel::High);
    }

    #[test]
    fn test_risk_level_display() {
        assert_eq!(RiskLevel::Low.color(), "green");
        assert_eq!(RiskLevel::Medium.color(), "yellow");
        assert_eq!(RiskLevel::High.color(), "red");
    }

    #[test]
    fn test_capability_equality() {
        let cap1 = Capability::Network;
        let cap2 = Capability::Network;
        let cap3 = Capability::Filesystem;

        assert_eq!(cap1, cap2);
        assert_ne!(cap1, cap3);
    }

    #[test]
    fn test_capability_serialization() {
        let cap = Capability::Network;
        let json = serde_json::to_string(&cap).unwrap();
        assert_eq!(json, "\"network\"");

        let deserialized: Capability = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, cap);
    }
}
