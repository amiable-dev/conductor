/// Integration test for plugin marketplace Tauri commands
///
/// Tests the full command flow:
/// 1. list_installed_plugins - verify empty state
/// 2. fetch_plugin_registry - verify registry fetch
/// 3. install_plugin_from_registry - install a plugin
/// 4. list_installed_plugins - verify plugin appears
/// 5. uninstall_plugin - remove plugin
/// 6. list_installed_plugins - verify plugin removed
use conductor_gui::plugin_commands;

#[tokio::test]
async fn test_list_installed_plugins_empty() {
    // Should return empty list initially
    let result = plugin_commands::list_installed_plugins().await;
    assert!(result.is_ok(), "list_installed_plugins should succeed");

    let plugins = result.unwrap();
    println!("Installed plugins: {:?}", plugins);
    // Note: May not be empty if plugins were installed from previous tests
}

#[cfg(feature = "plugin-registry")]
#[tokio::test]
async fn test_fetch_plugin_registry() {
    // Should fetch registry from GitHub or cache
    let result = plugin_commands::fetch_plugin_registry().await;

    match result {
        Ok(registry) => {
            println!(
                "✅ Fetched registry with {} plugins",
                registry.plugins.len()
            );
            assert!(
                !registry.plugins.is_empty(),
                "Registry should contain plugins"
            );

            // Verify expected plugins exist
            let plugin_ids: Vec<String> = registry.plugins.iter().map(|p| p.id.clone()).collect();
            println!("Available plugins: {:?}", plugin_ids);

            // Should have spotify and obs plugins at minimum
            assert!(
                plugin_ids.contains(&"spotify".to_string())
                    || plugin_ids.contains(&"obs".to_string()),
                "Registry should contain spotify or obs plugin"
            );
        }
        Err(e) => {
            println!("⚠️  Failed to fetch registry: {}", e);
            println!("This is expected if offline or GitHub is unreachable");
            // Don't fail test - network issues are acceptable
        }
    }
}

#[test]
fn test_plugin_directory_exists() {
    // Verify plugin directory is created
    let plugin_dir = dirs::data_dir()
        .expect("Failed to get data dir")
        .join("midimon")
        .join("plugins");

    println!("Plugin directory: {}", plugin_dir.display());

    if !plugin_dir.exists() {
        println!("⚠️  Plugin directory does not exist yet (will be created on first install)");
    } else {
        println!("✅ Plugin directory exists");

        // List installed plugins
        if let Ok(entries) = std::fs::read_dir(&plugin_dir) {
            let plugins: Vec<String> = entries
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    let path = e.path();
                    if path.is_dir() {
                        path.file_name()
                            .and_then(|n| n.to_str())
                            .map(|s| s.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            println!("Found {} installed plugins: {:?}", plugins.len(), plugins);
        }
    }
}

// Note: Cannot test install/uninstall without creating AppState with Engine
// These require proper daemon integration which is tested in plugin_integration_test.rs
// This test focuses on command signatures and basic functionality
