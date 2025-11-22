// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! End-to-end pipeline benchmarks
//!
//! This module benchmarks the complete MIDI processing pipeline:
//! 1. Raw MIDI event input
//! 2. EventProcessor: MIDI → ProcessedEvent
//! 3. MappingEngine: ProcessedEvent → Action
//! 4. Action execution (moved to midimon-daemon in Phase 2)
//!
//! These benchmarks measure the latency from MIDI input to action dispatch.
//! Action execution benchmarks are now in midimon-daemon.
//!
//! Performance targets:
//! - Complete pipeline (input to action dispatch): <1ms
//! - Median latency: <300μs
//! - Worst case (with 50 mappings): <500μs

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use conductor_core::{
    ActionConfig, Config, DeviceConfig, EventProcessor, Mapping, MappingEngine, MidiEvent, Mode,
    Trigger,
};
use std::time::Instant;

/// Helper to create a test configuration
fn create_test_config(num_mappings: usize) -> Config {
    let mappings = (0..num_mappings)
        .map(|i| Mapping {
            trigger: Trigger::Note {
                note: (36 + (i % 50)) as u8,
                velocity_min: None,
            },
            action: ActionConfig::Keystroke {
                keys: "a".to_string(),
                modifiers: vec![],
            },
            description: None,
        })
        .collect();

    Config {
        device: DeviceConfig {
            name: "Test Device".to_string(),
            auto_connect: false,
            auto_reconnect: true,
            port: None,
        },
        modes: vec![Mode {
            name: "Default".to_string(),
            color: Some("blue".to_string()),
            mappings,
        }],
        global_mappings: vec![],
        logging: None,
        advanced_settings: Default::default(),
    }
}

/// Benchmark simple note press → action dispatch
/// This is the most common user interaction: press a pad, get an action.
/// Action execution is now handled by midimon-daemon's ActionExecutor.
fn bench_simple_note_to_action(c: &mut Criterion) {
    let config = create_test_config(10);
    let mut processor = EventProcessor::new();
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);

    let now = Instant::now();

    c.bench_function("e2e::simple_note_press", |b| {
        b.iter(|| {
            let midi_event = black_box(MidiEvent::NoteOn {
                note: 36,
                velocity: 64,
                time: black_box(now),
            });

            // Process MIDI event
            let _processed = processor.process(midi_event.clone());

            // Get action from mapping (execution moved to daemon)
            let _action = engine.get_action(&midi_event, 0);
        })
    });
}

/// Benchmark complete pipeline without action execution
/// This isolates the event processing and mapping logic from OS interaction.
fn bench_pipeline_without_execution(c: &mut Criterion) {
    let config = create_test_config(10);
    let mut processor = EventProcessor::new();
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);

    let now = Instant::now();

    c.bench_function("e2e::pipeline_no_execution", |b| {
        b.iter(|| {
            let midi_event = black_box(MidiEvent::NoteOn {
                note: 36,
                velocity: 64,
                time: black_box(now),
            });

            // Process MIDI event
            let _processed = processor.process(midi_event.clone());

            // Get action from mapping (don't execute to avoid OS overhead)
            let _action = engine.get_action(&midi_event, 0);
        })
    });
}

/// Benchmark full pipeline with varying configuration sizes
/// Tests how performance scales as configuration grows.
fn bench_pipeline_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e::scaling");

    for size in [1, 5, 10, 20, 50].iter() {
        let config = create_test_config(*size);
        let mut processor = EventProcessor::new();
        let mut engine = MappingEngine::new();
        engine.load_from_config(&config);
        let now = Instant::now();

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_mappings", size)),
            size,
            |b, _| {
                b.iter(|| {
                    let midi_event = black_box(MidiEvent::NoteOn {
                        note: 36 + (*size as u8 / 2),
                        velocity: 64,
                        time: black_box(now),
                    });

                    let _processed = processor.process(midi_event.clone());
                    let _action = engine.get_action(&midi_event, 0);
                })
            },
        );
    }

    group.finish();
}

/// Benchmark rapid note input processing
/// Simulates user playing notes rapidly on device.
fn bench_rapid_note_input(c: &mut Criterion) {
    let config = create_test_config(15);
    let mut processor = EventProcessor::new();
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    let notes = black_box(vec![36, 37, 38, 39, 40, 41, 42, 43]);

    c.bench_function("e2e::rapid_note_input", |b| {
        b.iter(|| {
            for &note in &notes {
                let midi_event = MidiEvent::NoteOn {
                    note,
                    velocity: 64,
                    time: black_box(now),
                };

                let _processed = processor.process(midi_event.clone());
                let _action = engine.get_action(&midi_event, 0);
            }
        })
    });
}

/// Benchmark encoder input processing
/// Tests CC events (used for encoders, knobs).
fn bench_encoder_input(c: &mut Criterion) {
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

    let mut processor = EventProcessor::new();
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    c.bench_function("e2e::encoder_input", |b| {
        b.iter(|| {
            let midi_event = black_box(MidiEvent::ControlChange {
                cc: 14,
                value: 64,
                time: black_box(now),
            });

            let _processed = processor.process(midi_event.clone());
            let _action = engine.get_action(&midi_event, 0);
        })
    });
}

/// Benchmark note off (release) processing
/// Tests the release detection path.
fn bench_note_release_processing(c: &mut Criterion) {
    let config = create_test_config(10);
    let mut processor = EventProcessor::new();
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    c.bench_function("e2e::note_release", |b| {
        b.iter(|| {
            let midi_event = black_box(MidiEvent::NoteOff {
                note: 36,
                time: black_box(now),
            });

            let _processed = processor.process(midi_event.clone());
            let _action = engine.get_action(&midi_event, 0);
        })
    });
}

/// Benchmark velocity-sensitive note processing
/// Tests different velocity ranges (soft/medium/hard).
fn bench_velocity_sensitive_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e::velocity_sensitive");
    let config = create_test_config(10);
    let now = Instant::now();

    for (velocity, label) in &[(20u8, "soft"), (60u8, "medium"), (100u8, "hard")] {
        let mut processor = EventProcessor::new();
        let mut engine = MappingEngine::new();
        engine.load_from_config(&config);

        group.bench_with_input(BenchmarkId::from_parameter(label), velocity, |b, &vel| {
            b.iter(|| {
                let midi_event = black_box(MidiEvent::NoteOn {
                    note: 36,
                    velocity: vel,
                    time: black_box(now),
                });

                let _processed = processor.process(midi_event.clone());
                let _action = engine.get_action(&midi_event, 0);
            })
        });
    }

    group.finish();
}

/// Benchmark mode switching with input processing
/// Tests realistic scenario of switching modes and processing notes.
fn bench_mode_switching(c: &mut Criterion) {
    let config = create_test_config(15);
    let mut processor = EventProcessor::new();
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    c.bench_function("e2e::mode_switching", |b| {
        b.iter(|| {
            let modes = black_box([0u8, 1, 0, 1, 2, 0]);

            for &mode in &modes {
                let midi_event = MidiEvent::NoteOn {
                    note: 36,
                    velocity: 64,
                    time: black_box(now),
                };

                let _processed = processor.process(midi_event.clone());
                let _action = engine.get_action(&midi_event, black_box(mode));
            }
        })
    });
}

/// Benchmark realistic mixed input pattern
/// Simulates typical user interaction: notes, releases, encoders, velocity variations.
fn bench_mixed_input_pattern(c: &mut Criterion) {
    let config = create_test_config(20);
    let mut processor = EventProcessor::new();
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);
    let now = Instant::now();

    c.bench_function("e2e::mixed_pattern", |b| {
        b.iter(|| {
            // Simulate: note on, encoder turn, note release, another note with different velocity
            let events = black_box(vec![
                MidiEvent::NoteOn {
                    note: 36,
                    velocity: 50,
                    time: now,
                },
                MidiEvent::ControlChange {
                    cc: 14,
                    value: 65,
                    time: now,
                },
                MidiEvent::NoteOff {
                    note: 36,
                    time: now,
                },
                MidiEvent::NoteOn {
                    note: 37,
                    velocity: 100,
                    time: now,
                },
            ]);

            for event in events {
                let _processed = processor.process(event.clone());
                let _action = engine.get_action(&event, 0);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_simple_note_to_action,
    bench_pipeline_without_execution,
    bench_pipeline_scaling,
    bench_rapid_note_input,
    bench_encoder_input,
    bench_note_release_processing,
    bench_velocity_sensitive_processing,
    bench_mode_switching,
    bench_mixed_input_pattern,
);
criterion_main!(benches);
