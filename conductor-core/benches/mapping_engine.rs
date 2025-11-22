// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Mapping engine benchmarks
//!
//! This module benchmarks the MappingEngine::get_action() method which matches
//! MIDI events against configured mappings to return corresponding actions.
//!
//! Performance targets:
//! - Event → Action match: <300μs
//! - Global + mode-specific search: <400μs
//! - Large mapping set (50+ mappings): <500μs

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use conductor_core::{
    ActionConfig, Config, DeviceConfig, Mapping, MappingEngine, MidiEvent, Mode, Trigger,
};
use std::time::Instant;

/// Helper to create a minimal config with N mappings
fn create_config_with_mappings(num_mappings: usize) -> Config {
    let mut mappings = Vec::new();
    for i in 0..num_mappings {
        mappings.push(Mapping {
            trigger: Trigger::Note {
                note: (36 + (i % 50)) as u8,
                velocity_min: None,
            },
            action: ActionConfig::Keystroke {
                keys: "a".to_string(),
                modifiers: vec![],
            },
            description: None,
        });
    }

    let mode = Mode {
        name: "Default".to_string(),
        color: Some("blue".to_string()),
        mappings,
    };

    Config {
        device: DeviceConfig {
            name: "Test Device".to_string(),
            auto_connect: false,
            auto_reconnect: true,
            port: None,
        },
        modes: vec![mode],
        global_mappings: vec![],
        logging: None,
        advanced_settings: Default::default(),
    }
}

/// Benchmark basic note mapping lookup
/// Tests the common case of a single note mapped to an action.
fn bench_simple_note_mapping(c: &mut Criterion) {
    let config = create_config_with_mappings(1);
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    c.bench_function("mapping_engine::simple_note", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::NoteOn {
                note: 36,
                velocity: 64,
                time: black_box(now),
            });
            engine.get_action(&event, 0)
        })
    });
}

/// Benchmark mapping lookup with no match
/// Tests the failure case where event doesn't match any mapping.
fn bench_no_match_lookup(c: &mut Criterion) {
    let config = create_config_with_mappings(10);
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    c.bench_function("mapping_engine::no_match", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::NoteOn {
                note: 127, // Note not in mappings
                velocity: 64,
                time: black_box(now),
            });
            engine.get_action(&event, 0)
        })
    });
}

/// Benchmark mapping search with increasing mapping set sizes
/// Tests performance scaling as configuration grows.
fn bench_mapping_set_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("mapping_engine::scaling");

    for size in [1, 5, 10, 20, 50].iter() {
        let config = create_config_with_mappings(*size);
        let mut engine = MappingEngine::new();
        engine.load_from_config(&config);
        let now = Instant::now();

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_mappings", size)),
            size,
            |b, _| {
                b.iter(|| {
                    let event = black_box(MidiEvent::NoteOn {
                        note: 36 + (*size as u8 / 2),
                        velocity: 64,
                        time: black_box(now),
                    });
                    engine.get_action(&event, 0)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark mode-specific mapping lookup
/// Tests retrieval with explicit mode selection.
fn bench_mode_specific_lookup(c: &mut Criterion) {
    let config = create_config_with_mappings(10);
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    c.bench_function("mapping_engine::mode_lookup", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::NoteOn {
                note: 40,
                velocity: 64,
                time: black_box(now),
            });
            engine.get_action(&event, 0) // Mode 0
        })
    });
}

/// Benchmark control change mapping lookup
/// Tests CC event matching (used for encoders, knobs).
fn bench_cc_mapping_lookup(c: &mut Criterion) {
    let config = Config {
        device: DeviceConfig {
            name: "Test Device".to_string(),
            auto_connect: false,
            auto_reconnect: true,
            port: None,
        },
        modes: vec![Mode {
            name: "Default".to_string(),
            color: Some("blue".to_string()),
            mappings: vec![Mapping {
                trigger: Trigger::CC {
                    cc: 14,
                    value_min: None,
                },
                action: ActionConfig::Keystroke {
                    keys: "a".to_string(),
                    modifiers: vec![],
                },
                description: None,
            }],
        }],
        global_mappings: vec![],
        logging: None,
        advanced_settings: Default::default(),
    };

    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    c.bench_function("mapping_engine::cc_lookup", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::ControlChange {
                cc: 14,
                value: 64,
                time: black_box(now),
            });
            engine.get_action(&event, 0)
        })
    });
}

/// Benchmark rapid mode switches with mapping lookup
/// Tests realistic scenario of switching modes and then querying mappings.
fn bench_mode_switching_with_lookup(c: &mut Criterion) {
    let config = create_config_with_mappings(15);
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    c.bench_function("mapping_engine::mode_switch_lookup", |b| {
        b.iter(|| {
            let modes = black_box([0u8, 1, 0, 1, 2, 0]);

            for &mode in &modes {
                let event = black_box(MidiEvent::NoteOn {
                    note: 36,
                    velocity: 64,
                    time: black_box(now),
                });
                let _ = engine.get_action(&event, black_box(mode));
            }
        })
    });
}

/// Benchmark consecutive note lookups (simulating rapid pad input)
/// Tests cache-friendly and non-cached access patterns.
fn bench_rapid_note_lookups(c: &mut Criterion) {
    let mut group = c.benchmark_group("mapping_engine::rapid_lookup");
    let config = create_config_with_mappings(20);
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    // Sequential notes (cache-friendly)
    group.bench_function("sequential_notes", |b| {
        b.iter(|| {
            for note in 36..44 {
                let event = black_box(MidiEvent::NoteOn {
                    note,
                    velocity: 64,
                    time: black_box(now),
                });
                engine.get_action(&event, 0);
            }
        })
    });

    // Random notes (cache-unfriendly)
    let random_notes = black_box(vec![36, 42, 38, 44, 40, 37, 43, 39]);
    group.bench_function("random_notes", |b| {
        b.iter(|| {
            for &note in &random_notes {
                let event = black_box(MidiEvent::NoteOn {
                    note,
                    velocity: 64,
                    time: black_box(now),
                });
                engine.get_action(&event, 0);
            }
        })
    });

    group.finish();
}

/// Benchmark note off event matching
/// Tests release event lookups which should be similarly fast.
fn bench_note_off_lookup(c: &mut Criterion) {
    let config = create_config_with_mappings(10);
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    c.bench_function("mapping_engine::note_off_lookup", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::NoteOff {
                note: 40,
                time: black_box(now),
            });
            engine.get_action(&event, 0)
        })
    });
}

criterion_group!(
    benches,
    bench_simple_note_mapping,
    bench_no_match_lookup,
    bench_mapping_set_scaling,
    bench_mode_specific_lookup,
    bench_cc_mapping_lookup,
    bench_mode_switching_with_lookup,
    bench_rapid_note_lookups,
    bench_note_off_lookup,
);
criterion_main!(benches);
