// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Frontmost Application Detection
//!
//! Provides cross-platform app detection with macOS-specific implementation.
//! Tracks the currently active application and notifies on changes.

// TODO Phase 5: Migrate to objc2-foundation crate to eliminate deprecation warnings
#![allow(deprecated)]
#![allow(unexpected_cfgs)]

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[cfg(target_os = "macos")]
use cocoa::base::{id, nil};
#[cfg(target_os = "macos")]
use cocoa::foundation::NSAutoreleasePool;
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};

/// Information about the frontmost application
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppInfo {
    /// Application bundle identifier (e.g., "com.apple.Safari")
    pub bundle_id: String,

    /// Application name (e.g., "Safari")
    pub name: String,

    /// Full path to application bundle
    pub path: String,

    /// Process ID
    pub pid: u32,
}

/// App detection state manager
pub struct AppDetector {
    /// Current frontmost app
    current_app: Arc<RwLock<Option<AppInfo>>>,

    /// Polling interval in milliseconds
    poll_interval_ms: u64,

    /// Whether detection is active
    is_active: Arc<RwLock<bool>>,
}

impl AppDetector {
    /// Create a new app detector with default 500ms polling
    pub fn new() -> Self {
        Self {
            current_app: Arc::new(RwLock::new(None)),
            poll_interval_ms: 500,
            is_active: Arc::new(RwLock::new(false)),
        }
    }

    /// Create with custom polling interval
    #[allow(dead_code)] // Part of public API, may be used by future features
    pub fn with_interval(poll_interval_ms: u64) -> Self {
        Self {
            current_app: Arc::new(RwLock::new(None)),
            poll_interval_ms,
            is_active: Arc::new(RwLock::new(false)),
        }
    }

    /// Get the current frontmost app
    pub async fn get_current_app(&self) -> Option<AppInfo> {
        self.current_app.read().await.clone()
    }

    /// Detect frontmost app once (synchronous platform-specific detection)
    #[cfg(target_os = "macos")]
    fn detect_frontmost_app_once() -> Option<AppInfo> {
        unsafe {
            let pool = NSAutoreleasePool::new(nil);

            // Get shared NSWorkspace
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];

            // Get frontmost application
            let frontmost_app: id = msg_send![workspace, frontmostApplication];

            if frontmost_app == nil {
                let _: () = msg_send![pool, drain];
                return None;
            }

            // Get bundle identifier
            let bundle_id_ns: id = msg_send![frontmost_app, bundleIdentifier];
            let bundle_id = if bundle_id_ns != nil {
                let c_str: *const i8 = msg_send![bundle_id_ns, UTF8String];
                if c_str.is_null() {
                    String::from("unknown")
                } else {
                    std::ffi::CStr::from_ptr(c_str)
                        .to_string_lossy()
                        .into_owned()
                }
            } else {
                String::from("unknown")
            };

            // Get localized name
            let name_ns: id = msg_send![frontmost_app, localizedName];
            let name = if name_ns != nil {
                let c_str: *const i8 = msg_send![name_ns, UTF8String];
                if c_str.is_null() {
                    String::from("Unknown App")
                } else {
                    std::ffi::CStr::from_ptr(c_str)
                        .to_string_lossy()
                        .into_owned()
                }
            } else {
                String::from("Unknown App")
            };

            // Get bundle URL path
            let bundle_url: id = msg_send![frontmost_app, bundleURL];
            let path = if bundle_url != nil {
                let path_ns: id = msg_send![bundle_url, path];
                if path_ns != nil {
                    let c_str: *const i8 = msg_send![path_ns, UTF8String];
                    if c_str.is_null() {
                        String::from("")
                    } else {
                        std::ffi::CStr::from_ptr(c_str)
                            .to_string_lossy()
                            .into_owned()
                    }
                } else {
                    String::from("")
                }
            } else {
                String::from("")
            };

            // Get process identifier
            let pid: i32 = msg_send![frontmost_app, processIdentifier];

            let _: () = msg_send![pool, drain];

            Some(AppInfo {
                bundle_id,
                name,
                path,
                pid: pid as u32,
            })
        }
    }

    /// Detect frontmost app once (non-macOS stub)
    #[cfg(not(target_os = "macos"))]
    fn detect_frontmost_app_once() -> Option<AppInfo> {
        // TODO: Implement for Windows/Linux
        None
    }

    /// Start continuous app detection with change notifications
    pub async fn start_detection<F>(&self, on_app_change: F)
    where
        F: Fn(Option<AppInfo>) + Send + Sync + 'static,
    {
        let mut is_active = self.is_active.write().await;
        if *is_active {
            return; // Already running
        }
        *is_active = true;
        drop(is_active);

        let current_app = Arc::clone(&self.current_app);
        let is_active = Arc::clone(&self.is_active);
        let poll_interval = self.poll_interval_ms;

        tokio::spawn(async move {
            loop {
                // Check if detection is still active
                let active = *is_active.read().await;
                if !active {
                    break;
                }

                // Detect current app
                let new_app = Self::detect_frontmost_app_once();

                // Check if app changed
                let mut current = current_app.write().await;
                let changed = match (&*current, &new_app) {
                    (None, Some(_)) => true,
                    (Some(_), None) => true,
                    (Some(old), Some(new)) => old.bundle_id != new.bundle_id,
                    (None, None) => false,
                };

                if changed {
                    *current = new_app.clone();
                    drop(current); // Release lock before callback
                    on_app_change(new_app);
                }

                // Wait before next poll
                tokio::time::sleep(Duration::from_millis(poll_interval)).await;
            }
        });
    }

    /// Stop continuous detection
    pub async fn stop_detection(&self) {
        let mut is_active = self.is_active.write().await;
        *is_active = false;
    }

    /// Check if detection is currently active
    #[allow(dead_code)] // Part of public API, used in tests
    pub async fn is_active(&self) -> bool {
        *self.is_active.read().await
    }
}

impl Default for AppDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detector_creation() {
        let detector = AppDetector::new();
        assert!(!detector.is_active().await);
        assert!(detector.get_current_app().await.is_none());
    }

    #[tokio::test]
    async fn test_custom_interval() {
        let detector = AppDetector::with_interval(1000);
        assert_eq!(detector.poll_interval_ms, 1000);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_frontmost_app_detection() {
        // This should detect the current test runner or IDE
        let app = AppDetector::detect_frontmost_app_once();
        assert!(app.is_some(), "Should detect at least one running app");

        if let Some(app_info) = app {
            assert!(!app_info.bundle_id.is_empty());
            assert!(!app_info.name.is_empty());
            println!("Detected app: {} ({})", app_info.name, app_info.bundle_id);
        }
    }

    #[tokio::test]
    async fn test_start_stop_detection() {
        let detector = AppDetector::new();
        let detector_ref = Arc::new(detector);

        let detector_clone = Arc::clone(&detector_ref);
        detector_clone
            .start_detection(|app| {
                if let Some(app_info) = app {
                    println!("App changed to: {}", app_info.name);
                }
            })
            .await;

        assert!(detector_ref.is_active().await);

        tokio::time::sleep(Duration::from_millis(100)).await;

        detector_ref.stop_detection().await;
        assert!(!detector_ref.is_active().await);
    }
}
