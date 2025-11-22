// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Action executor benchmarks
//!
//! This module benchmarks the ActionExecutor::execute() method which takes an
//! Action enum and performs the requested operation (keystroke, text, etc.).
//!
//! NOTE: Some actions (Keystroke, Text, Launch, etc.) interact with the OS and
//! can have high variance. These are benchmarked but may require OS interaction.
//! Mocked action execution would be more consistent but less realistic.
//!
//! Performance targets:
//! - Action instantiation/routing: <50μs
//! - Keystroke execution (with modifiers): <500μs (OS dependent)
//! - Delay action: <100μs (excluding the actual delay)
//! - Shell command launch: <1ms (OS dependent)

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use enigo::Key;
use conductor_core::actions::{Action, ActionExecutor};

/// Benchmark simple keystroke action execution
/// Tests pressing a single key without modifiers.
fn bench_simple_keystroke(c: &mut Criterion) {
    c.bench_function("action_executor::simple_keystroke", |b| {
        let mut executor = ActionExecutor::new();
        b.iter(|| {
            let action = black_box(Action::Keystroke {
                keys: vec![Key::Space],
                modifiers: vec![],
            });
            executor.execute(action)
        })
    });
}

/// Benchmark keystroke with modifiers
/// Tests pressing a key with shift/ctrl/alt modifiers.
fn bench_modified_keystroke(c: &mut Criterion) {
    let mut group = c.benchmark_group("action_executor::modified_keystroke");

    // Single modifier (e.g., Shift+Space)
    group.bench_function("single_modifier", |b| {
        let mut executor = ActionExecutor::new();
        b.iter(|| {
            let action = black_box(Action::Keystroke {
                keys: vec![Key::Space],
                modifiers: vec![Key::Shift],
            });
            executor.execute(action)
        })
    });

    // Multiple modifiers (e.g., Ctrl+Shift+Space)
    group.bench_function("multiple_modifiers", |b| {
        let mut executor = ActionExecutor::new();
        b.iter(|| {
            let action = black_box(Action::Keystroke {
                keys: vec![Key::Space],
                modifiers: vec![Key::Control, Key::Shift],
            });
            executor.execute(action)
        })
    });

    group.finish();
}

/// Benchmark text typing action
/// Tests the text input simulation.
fn bench_text_action(c: &mut Criterion) {
    let mut group = c.benchmark_group("action_executor::text");

    let test_cases = vec![
        ("single_char", "a"),
        ("short_word", "hello"),
        (
            "long_sentence",
            "The quick brown fox jumps over the lazy dog",
        ),
    ];

    for (label, text) in test_cases {
        group.bench_function(label, |b| {
            let mut executor = ActionExecutor::new();
            b.iter(|| {
                let action = black_box(Action::Text(black_box(text.to_string())));
                executor.execute(action)
            })
        });
    }

    group.finish();
}

/// Benchmark delay action execution
/// Tests the overhead of the delay action itself (not the actual delay).
fn bench_delay_action(c: &mut Criterion) {
    let mut group = c.benchmark_group("action_executor::delay");

    for duration_ms in &[10u64, 50, 100, 500] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}ms", duration_ms)),
            duration_ms,
            |b, &ms| {
                let mut executor = ActionExecutor::new();
                b.iter(|| {
                    let action = black_box(Action::Delay(ms));
                    executor.execute(action)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark launch application action
/// NOTE: This test launches a real application, so timing may be variable.
/// macOS-specific implementation shown.
#[cfg(target_os = "macos")]
fn bench_launch_action(c: &mut Criterion) {
    let mut group = c.benchmark_group("action_executor::launch");
    group.sample_size(10); // Reduce samples due to OS variability

    group.bench_function("launch_app", |b| {
        let mut executor = ActionExecutor::new();
        b.iter(|| {
            // Note: Launching a lightweight application for consistent timing
            let action = black_box(Action::Launch("Finder".to_string()));
            executor.execute(action)
        })
    });

    group.finish();
}

/// Benchmark shell command execution
/// NOTE: This test executes real shell commands, so timing may be variable.
fn bench_shell_action(c: &mut Criterion) {
    let mut group = c.benchmark_group("action_executor::shell");
    group.sample_size(10); // Reduce samples due to OS variability

    group.bench_function("shell_echo", |b| {
        let mut executor = ActionExecutor::new();
        b.iter(|| {
            let action = black_box(Action::Shell("echo 'test'".to_string()));
            executor.execute(action)
        })
    });

    group.finish();
}

/// Benchmark sequence action execution
/// Tests the overhead of executing multiple actions in sequence.
fn bench_sequence_action(c: &mut Criterion) {
    let mut group = c.benchmark_group("action_executor::sequence");

    // Sequence of keystroke actions
    group.bench_function("keystroke_sequence", |b| {
        let mut executor = ActionExecutor::new();
        b.iter(|| {
            let action = black_box(Action::Sequence(vec![
                Action::Keystroke {
                    keys: vec![Key::Space],
                    modifiers: vec![],
                },
                Action::Keystroke {
                    keys: vec![Key::Return],
                    modifiers: vec![],
                },
                Action::Keystroke {
                    keys: vec![Key::Escape],
                    modifiers: vec![],
                },
            ]));
            executor.execute(action)
        })
    });

    // Sequence with delays for realistic spacing
    group.bench_function("with_delays", |b| {
        let mut executor = ActionExecutor::new();
        b.iter(|| {
            let action = black_box(Action::Sequence(vec![
                Action::Keystroke {
                    keys: vec![Key::Space],
                    modifiers: vec![],
                },
                Action::Delay(50), // 50ms delay between actions
                Action::Keystroke {
                    keys: vec![Key::Return],
                    modifiers: vec![],
                },
            ]));
            executor.execute(action)
        })
    });

    group.finish();
}

/// Benchmark action executor instantiation
/// Tests the cost of creating a new executor (Enigo initialization).
fn bench_executor_creation(c: &mut Criterion) {
    c.bench_function("action_executor::creation", |b| {
        b.iter(|| {
            let _executor = ActionExecutor::new();
        })
    });
}

/// Benchmark the routing/dispatch overhead of execute()
/// This tests how much time is spent in pattern matching and delegation
/// vs. the actual action execution. Uses lightweight Delay action to minimize
/// OS interaction effects.
fn bench_dispatch_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("action_executor::dispatch_overhead");

    group.bench_function("delay_action_routing", |b| {
        let mut executor = ActionExecutor::new();
        b.iter(|| {
            let action = black_box(Action::Delay(0)); // 0ms delay to minimize overhead
            executor.execute(action)
        })
    });

    group.finish();
}

/// Benchmark mouse click action
/// Tests mouse movement and click simulation.
fn bench_mouse_click_action(c: &mut Criterion) {
    let mut group = c.benchmark_group("action_executor::mouse");

    // Click without movement
    group.bench_function("click", |b| {
        let mut executor = ActionExecutor::new();
        b.iter(|| {
            let action = black_box(Action::MouseClick {
                button: enigo::Button::Left,
                x: None,
                y: None,
            });
            executor.execute(action)
        })
    });

    // Click with movement
    group.bench_function("move_and_click", |b| {
        let mut executor = ActionExecutor::new();
        b.iter(|| {
            let action = black_box(Action::MouseClick {
                button: enigo::Button::Left,
                x: Some(500),
                y: Some(500),
            });
            executor.execute(action)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_keystroke,
    bench_modified_keystroke,
    bench_text_action,
    bench_delay_action,
    bench_shell_action,
    bench_sequence_action,
    bench_executor_creation,
    bench_dispatch_overhead,
    bench_mouse_click_action,
);

#[cfg(target_os = "macos")]
criterion_group!(macos_benches, bench_launch_action);

#[cfg(target_os = "macos")]
criterion_main!(benches, macos_benches);

#[cfg(not(target_os = "macos"))]
criterion_main!(benches);
