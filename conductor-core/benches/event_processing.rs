// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Event processing benchmarks
//!
//! This module benchmarks the EventProcessor::process() method which converts
//! raw MIDI events into ProcessedEvents with timing, velocity detection, and
//! special event detection (long press, double-tap, chords, etc.).
//!
//! Performance targets:
//! - Single MIDI event → ProcessedEvent: <200μs
//! - Chord detection: <300μs
//! - Double-tap detection: <150μs

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use conductor_core::{EventProcessor, MidiEvent};
use std::time::Instant;

/// Benchmark basic note on event processing
/// Tests the common case of pressing a pad with velocity.
fn bench_note_on_event(c: &mut Criterion) {
    let mut processor = EventProcessor::new();
    let now = Instant::now();

    c.bench_function("event_processing::note_on", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::NoteOn {
                note: 36,
                velocity: 64,
                time: black_box(now),
            });
            processor.process(event)
        })
    });
}

/// Benchmark note off event processing
/// Tests the release detection and hold duration calculation.
fn bench_note_off_event(c: &mut Criterion) {
    let mut processor = EventProcessor::new();
    let now = Instant::now();

    // First send a note on
    processor.process(MidiEvent::NoteOn {
        note: 36,
        velocity: 64,
        time: now,
    });

    c.bench_function("event_processing::note_off", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::NoteOff {
                note: 36,
                time: black_box(now),
            });
            processor.process(event)
        })
    });
}

/// Benchmark velocity level detection
/// Tests categorization of velocity into soft/medium/hard ranges.
fn bench_velocity_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_processing::velocity");
    let now = Instant::now();

    for (velocity, label) in &[(20u8, "soft"), (60u8, "medium"), (100u8, "hard")] {
        let mut processor = EventProcessor::new();
        group.bench_with_input(BenchmarkId::from_parameter(label), velocity, |b, &vel| {
            b.iter(|| {
                let event = black_box(MidiEvent::NoteOn {
                    note: 36,
                    velocity: vel,
                    time: black_box(now),
                });
                processor.process(event)
            })
        });
    }
    group.finish();
}

/// Benchmark control change (CC) event processing
/// Tests encoder/control detection and value tracking.
fn bench_control_change_event(c: &mut Criterion) {
    let mut processor = EventProcessor::new();
    let now = Instant::now();

    c.bench_function("event_processing::control_change", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::ControlChange {
                cc: 14,
                value: 64,
                time: black_box(now),
            });
            processor.process(event)
        })
    });
}

/// Benchmark encoder direction detection
/// Tests the logic that determines clockwise vs counter-clockwise rotation.
fn bench_encoder_direction_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_processing::encoder_direction");
    let now = Instant::now();

    // Clockwise (64 -> 65)
    let mut processor_cw = EventProcessor::new();
    processor_cw.process(MidiEvent::ControlChange {
        cc: 14,
        value: 64,
        time: now,
    });

    group.bench_function("clockwise", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::ControlChange {
                cc: 14,
                value: 65,
                time: black_box(now),
            });
            processor_cw.process(event)
        })
    });

    // Counter-clockwise (64 -> 63)
    let mut processor_ccw = EventProcessor::new();
    processor_ccw.process(MidiEvent::ControlChange {
        cc: 14,
        value: 64,
        time: now,
    });

    group.bench_function("counter_clockwise", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::ControlChange {
                cc: 14,
                value: 63,
                time: black_box(now),
            });
            processor_ccw.process(event)
        })
    });

    group.finish();
}

/// Benchmark aftertouch (pressure) event processing
/// Tests pressure sensitivity event handling.
fn bench_aftertouch_event(c: &mut Criterion) {
    let mut processor = EventProcessor::new();
    let now = Instant::now();

    c.bench_function("event_processing::aftertouch", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::Aftertouch {
                pressure: 80,
                time: black_box(now),
            });
            processor.process(event)
        })
    });
}

/// Benchmark pitch bend event processing
/// Tests touch strip/pitch bend detection.
fn bench_pitch_bend_event(c: &mut Criterion) {
    let mut processor = EventProcessor::new();
    let now = Instant::now();

    c.bench_function("event_processing::pitch_bend", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::PitchBend {
                value: 8192,
                time: black_box(now),
            });
            processor.process(event)
        })
    });
}

/// Benchmark program change event processing
/// Tests program/bank change handling.
fn bench_program_change_event(c: &mut Criterion) {
    let mut processor = EventProcessor::new();
    let now = Instant::now();

    c.bench_function("event_processing::program_change", |b| {
        b.iter(|| {
            let event = black_box(MidiEvent::ProgramChange {
                program: 5,
                time: black_box(now),
            });
            processor.process(event)
        })
    });
}

/// Benchmark consecutive note events (simulating real performance)
/// Tests the performance of processing a stream of notes rapidly.
fn bench_consecutive_notes(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_processing::stream");
    group.measurement_time(std::time::Duration::from_secs(10));

    let _processor = EventProcessor::new();
    let now = Instant::now();
    let notes = black_box(vec![36, 37, 38, 39, 40, 41, 42, 43]);

    group.bench_function("10_consecutive_notes", |b| {
        b.iter(|| {
            let mut local_processor = EventProcessor::new();
            for &note in &notes {
                local_processor.process(MidiEvent::NoteOn {
                    note,
                    velocity: 64,
                    time: black_box(now),
                });
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_note_on_event,
    bench_note_off_event,
    bench_velocity_detection,
    bench_control_change_event,
    bench_encoder_direction_detection,
    bench_aftertouch_event,
    bench_pitch_bend_event,
    bench_program_change_event,
    bench_consecutive_notes,
);
criterion_main!(benches);
