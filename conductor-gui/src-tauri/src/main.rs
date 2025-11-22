// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon GUI - Visual Configuration Interface
//!
//! Tauri v2-based desktop application for configuring MIDIMon mappings,
//! MIDI Learn functionality, and visual feedback.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_detection;
mod commands;
mod config_helpers;
mod device_templates;
mod events;
mod menu_bar;
mod midi_learn;
mod plugin_commands;
mod profile_manager;
mod state;

use state::AppState;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("conductor_gui=debug".parse().unwrap()),
        )
        .init();

    let app = tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Setup tray after app is initialized
            menu_bar::setup_tray(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_daemon_status,
            commands::reload_config,
            commands::stop_daemon,
            commands::validate_config,
            commands::ping_daemon,
            commands::list_midi_devices,
            commands::list_gamepads, // v3.0: Gamepad discovery
            commands::get_config,
            commands::save_config,
            commands::get_config_path,
            commands::start_midi_learn,
            commands::get_midi_learn_status,
            commands::get_midi_learn_remaining,
            commands::cancel_midi_learn,
            commands::get_midi_learn_result,
            commands::generate_trigger_config_toml,
            commands::trigger_suggestion_to_json,
            commands::get_frontmost_app,
            commands::start_app_monitoring,
            commands::stop_app_monitoring,
            commands::list_profiles,
            commands::register_profile,
            commands::switch_profile,
            commands::switch_profile_for_app,
            commands::get_active_profile,
            commands::scan_profiles,
            commands::clear_profile_cache,
            commands::export_profile_json,
            commands::import_profile_json,
            commands::export_profile_toml,
            commands::import_profile_toml,
            commands::list_device_templates,
            commands::get_device_template,
            commands::find_templates_by_midi,
            commands::get_template_categories,
            commands::list_templates_by_category,
            commands::create_config_from_template,
            commands::start_event_monitoring,
            commands::stop_event_monitoring,
            commands::is_event_monitoring_active,
            commands::list_midi_output_ports,
            commands::test_midi_output,
            commands::validate_send_midi_action,
            plugin_commands::plugin_discover,
            plugin_commands::plugin_list_available,
            plugin_commands::plugin_list_loaded,
            plugin_commands::plugin_get_metadata,
            plugin_commands::plugin_load,
            plugin_commands::plugin_unload,
            plugin_commands::plugin_enable,
            plugin_commands::plugin_disable,
            plugin_commands::plugin_grant_capability,
            plugin_commands::plugin_revoke_capability,
            plugin_commands::plugin_get_stats,
            plugin_commands::list_installed_plugins,
            plugin_commands::uninstall_plugin,
            #[cfg(feature = "plugin-registry")]
            plugin_commands::fetch_plugin_registry,
            #[cfg(feature = "plugin-registry")]
            plugin_commands::install_plugin_from_registry,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    // Start background status polling
    menu_bar::start_status_polling(app.handle().clone());

    app.run(|_app_handle, event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            // Prevent default exit to allow graceful shutdown
            api.prevent_exit();
        }
    });
}
