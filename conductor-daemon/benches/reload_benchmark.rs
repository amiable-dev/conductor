// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Benchmark for config reload latency
//!
//! Run with: cargo bench --package midimon-daemon --bench reload_benchmark
//!
//! This benchmark measures the performance of the core reload operations:
//! - Config file parsing (TOML deserialization)
//! - Mapping engine compilation
//! - Data structure swap time
//!
//! Target: <50ms total reload time for production configs

use conductor_core::{Config, MappingEngine};
use std::path::PathBuf;
use std::time::Instant;
use tempfile::tempdir;

/// Create a test config file with various complexity levels
fn create_test_config(path: &PathBuf, num_modes: usize, mappings_per_mode: usize) {
    let mut config_str = String::from(
        r#"
[device]
name = "Benchmark Device"
auto_connect = false

"#,
    );

    // Add modes
    for mode_idx in 0..num_modes {
        config_str.push_str(&format!(
            r#"
[[modes]]
name = "Mode {}"
color = "blue"

"#,
            mode_idx
        ));

        // Add mappings (variety of trigger types)
        for mapping_idx in 0..mappings_per_mode {
            let note = 36 + (mapping_idx % 16) as u8;

            // Vary the trigger types for realistic scenarios
            let trigger_type = match mapping_idx % 4 {
                0 => format!(
                    r#"{{ type = "Note", note = {}, velocity_min = 0, velocity_max = 127 }}"#,
                    note
                ),
                1 => format!(
                    r#"{{ type = "VelocityRange", note = {}, velocity_ranges = [{{level = "Soft", action_index = 0}}, {{level = "Hard", action_index = 1}}] }}"#,
                    note
                ),
                2 => format!(
                    r#"{{ type = "LongPress", note = {}, duration_ms = 2000 }}"#,
                    note
                ),
                _ => format!(r#"{{ type = "EncoderTurn", cc = 1, direction = "Clockwise" }}"#),
            };

            config_str.push_str(&format!(
                r#"
[[modes.mappings]]
trigger = {}
action = {{ type = "Keystroke", keys = "f{}" }}

"#,
                trigger_type,
                (mapping_idx % 12) + 1
            ));
        }
    }

    std::fs::write(path, config_str).unwrap();
}

/// Benchmark config reload latency
fn benchmark_reload(num_modes: usize, mappings_per_mode: usize) -> (u64, u64, u64, u64) {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    // Create test config
    create_test_config(&config_path, num_modes, mappings_per_mode);

    let mut load_times = Vec::new();
    let mut compile_times = Vec::new();
    let mut total_times = Vec::new();

    // Perform 10 iterations
    for _i in 0..10 {
        let total_start = Instant::now();

        // Phase 1: Load config from file
        let load_start = Instant::now();
        let config = Config::load(config_path.to_str().unwrap()).unwrap();
        let load_ms = load_start.elapsed().as_micros() as u64 / 1000;
        load_times.push(load_ms);

        // Phase 2: Compile mapping engine
        let compile_start = Instant::now();
        let mut mapping_engine = MappingEngine::new();
        mapping_engine.load_from_config(&config);
        let compile_ms = compile_start.elapsed().as_micros() as u64 / 1000;
        compile_times.push(compile_ms);

        let total_ms = total_start.elapsed().as_micros() as u64 / 1000;
        total_times.push(total_ms);
    }

    // Calculate averages
    let avg_load = load_times.iter().sum::<u64>() / load_times.len() as u64;
    let avg_compile = compile_times.iter().sum::<u64>() / compile_times.len() as u64;
    let avg_total = total_times.iter().sum::<u64>() / total_times.len() as u64;
    let min_total = *total_times.iter().min().unwrap_or(&0);

    (avg_load, avg_compile, avg_total, min_total)
}

fn main() {
    println!("\n═══════════════════════════════════════════════");
    println!("   Config Reload Latency Benchmark");
    println!("═══════════════════════════════════════════════\n");
    println!("Target: <50ms total reload time\n");
    println!("Breakdown phases:");
    println!("  1. Config Load   - Parse TOML file");
    println!("  2. Engine Compile - Build mapping engine");
    println!("  3. Swap          - Atomic data swap (measured in production)\n");
    println!(
        "\n{:<40} {:>8} {:>10} {:>10} {:>6} {:>6}",
        "Test Case", "Total", "Load", "Compile", "Grade", "Status"
    );
    println!("{}", "─".repeat(86));

    let test_cases = vec![
        (1, 10, "Small (1 mode, 10 mappings)"),
        (3, 30, "Medium (3 modes, 30 mappings each)"),
        (5, 50, "Production (5 modes, 50 mappings)"),
        (10, 100, "Large (10 modes, 100 mappings)"),
        (15, 150, "Extra Large (15 modes, 150 mappings)"),
    ];

    for (num_modes, mappings_per_mode, description) in test_cases {
        let (avg_load, avg_compile, avg_total, _min_total) =
            benchmark_reload(num_modes, mappings_per_mode);

        let grade = if avg_total < 20 {
            'A'
        } else if avg_total < 50 {
            'B'
        } else if avg_total < 100 {
            'C'
        } else if avg_total < 200 {
            'D'
        } else {
            'F'
        };

        let status = if avg_total < 50 {
            "✓ PASS"
        } else {
            "✗ FAIL"
        };

        println!(
            "{:<40} {:>6} ms {:>8} ms {:>8} ms {:>6} {:>6}",
            description, avg_total, avg_load, avg_compile, grade, status
        );
    }

    println!("\n{}", "─".repeat(86));
    println!("\nPerformance Grades:");
    println!("  A (0-20ms)    - Excellent");
    println!("  B (21-50ms)   - Good (Target)");
    println!("  C (51-100ms)  - Acceptable");
    println!("  D (101-200ms) - Poor");
    println!("  F (>200ms)    - Unacceptable");
    println!("\n═══════════════════════════════════════════════\n");
}
