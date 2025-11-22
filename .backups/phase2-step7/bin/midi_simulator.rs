// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Interactive MIDI Device Simulator CLI
//!
//! This tool provides an interactive command-line interface for simulating
//! MIDI device events without requiring physical hardware.

use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Import the simulator from tests module
#[path = "../../tests/midi_simulator.rs"]
mod midi_simulator;

use midi_simulator::{EncoderDirection, Gesture, MidiSimulator};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          MIDIMon - Interactive MIDI Simulator               ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let simulator = Arc::new(Mutex::new(MidiSimulator::new(0)));
    simulator.lock().unwrap().set_debug(true);

    print_help();

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "help" | "h" | "?" => print_help(),
            "quit" | "exit" | "q" => {
                println!("Goodbye!");
                break;
            }
            "clear" | "c" => {
                simulator.lock().unwrap().clear_events();
                println!("✓ Event queue cleared");
            }
            "events" | "e" => {
                let events = simulator.lock().unwrap().get_events();
                if events.is_empty() {
                    println!("No events in queue");
                } else {
                    println!("Captured events:");
                    for (i, event) in events.iter().enumerate() {
                        println!("  {}: {:02X?}", i + 1, event);
                    }
                }
            }

            // Note commands
            "note" | "n" => {
                if parts.len() < 3 {
                    println!("Usage: note <number> <velocity>");
                    continue;
                }
                if let (Ok(note), Ok(velocity)) = (parts[1].parse::<u8>(), parts[2].parse::<u8>()) {
                    let sim = simulator.lock().unwrap();
                    sim.note_on(note, velocity);
                    thread::sleep(Duration::from_millis(100));
                    sim.note_off(note);
                    println!("✓ Sent note {} with velocity {}", note, velocity);
                } else {
                    println!("✗ Invalid note or velocity value");
                }
            }

            // Velocity test
            "velocity" | "v" => {
                if parts.len() < 2 {
                    println!("Usage: velocity <note>");
                    continue;
                }
                if let Ok(note) = parts[1].parse::<u8>() {
                    println!("Simulating velocity levels (soft, medium, hard)...");
                    let sim = simulator.lock().unwrap();

                    // Soft
                    sim.note_on(note, 30);
                    thread::sleep(Duration::from_millis(100));
                    sim.note_off(note);
                    thread::sleep(Duration::from_millis(200));

                    // Medium
                    sim.note_on(note, 70);
                    thread::sleep(Duration::from_millis(100));
                    sim.note_off(note);
                    thread::sleep(Duration::from_millis(200));

                    // Hard
                    sim.note_on(note, 110);
                    thread::sleep(Duration::from_millis(100));
                    sim.note_off(note);

                    println!("✓ Velocity test complete");
                } else {
                    println!("✗ Invalid note number");
                }
            }

            // Long press
            "long" | "l" => {
                if parts.len() < 2 {
                    println!("Usage: long <note> [duration_ms]");
                    continue;
                }
                let note = parts[1].parse::<u8>().unwrap_or(60);
                let duration = parts
                    .get(2)
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(2500);

                println!("Simulating long press for {}ms...", duration);
                simulator
                    .lock()
                    .unwrap()
                    .perform_gesture(Gesture::LongPress {
                        note,
                        velocity: 80,
                        hold_ms: duration,
                    });
                println!("✓ Long press complete");
            }

            // Double tap
            "double" | "d" => {
                if parts.len() < 2 {
                    println!("Usage: double <note> [gap_ms]");
                    continue;
                }
                let note = parts[1].parse::<u8>().unwrap_or(60);
                let gap = parts
                    .get(2)
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(200);

                println!("Simulating double-tap with {}ms gap...", gap);
                simulator
                    .lock()
                    .unwrap()
                    .perform_gesture(Gesture::DoubleTap {
                        note,
                        velocity: 80,
                        tap_duration_ms: 50,
                        gap_ms: gap,
                    });
                println!("✓ Double-tap complete");
            }

            // Chord
            "chord" => {
                if parts.len() < 2 {
                    println!("Usage: chord <note1> <note2> [note3] [note4]...");
                    continue;
                }

                let notes: Vec<u8> = parts[1..]
                    .iter()
                    .filter_map(|s| s.parse::<u8>().ok())
                    .collect();

                if notes.len() < 2 {
                    println!("✗ Need at least 2 notes for a chord");
                    continue;
                }

                println!("Simulating chord: {:?}", notes);
                simulator.lock().unwrap().perform_gesture(Gesture::Chord {
                    notes,
                    velocity: 80,
                    stagger_ms: 10,
                    hold_ms: 500,
                });
                println!("✓ Chord complete");
            }

            // Encoder
            "encoder" | "enc" => {
                if parts.len() < 3 {
                    println!("Usage: encoder <cc> <cw|ccw> [steps]");
                    continue;
                }

                let cc = parts[1].parse::<u8>().unwrap_or(1);
                let direction = match parts[2].to_lowercase().as_str() {
                    "cw" | "clockwise" => EncoderDirection::Clockwise,
                    "ccw" | "counterclockwise" => EncoderDirection::CounterClockwise,
                    _ => {
                        println!("✗ Direction must be 'cw' or 'ccw'");
                        continue;
                    }
                };
                let steps = parts.get(3).and_then(|s| s.parse::<u8>().ok()).unwrap_or(5);

                println!(
                    "Simulating encoder CC{} {:?} {} steps...",
                    cc, direction, steps
                );
                simulator
                    .lock()
                    .unwrap()
                    .perform_gesture(Gesture::EncoderTurn {
                        cc,
                        direction,
                        steps,
                        step_delay_ms: 50,
                    });
                println!("✓ Encoder simulation complete");
            }

            // Aftertouch
            "aftertouch" | "at" => {
                if parts.len() < 2 {
                    println!("Usage: aftertouch <pressure>");
                    continue;
                }
                if let Ok(pressure) = parts[1].parse::<u8>() {
                    simulator.lock().unwrap().aftertouch(pressure);
                    println!("✓ Sent aftertouch pressure {}", pressure);
                } else {
                    println!("✗ Invalid pressure value");
                }
            }

            // Pitch bend
            "pitch" | "pb" => {
                if parts.len() < 2 {
                    println!("Usage: pitch <value>");
                    println!("  value: 0-16383 (center=8192)");
                    continue;
                }
                if let Ok(value) = parts[1].parse::<u16>() {
                    if value > 16383 {
                        println!("✗ Pitch bend value must be 0-16383");
                        continue;
                    }
                    simulator.lock().unwrap().pitch_bend(value);
                    println!("✓ Sent pitch bend {}", value);
                } else {
                    println!("✗ Invalid pitch bend value");
                }
            }

            // Control Change
            "cc" => {
                if parts.len() < 3 {
                    println!("Usage: cc <number> <value>");
                    continue;
                }
                if let (Ok(cc), Ok(value)) = (parts[1].parse::<u8>(), parts[2].parse::<u8>()) {
                    simulator.lock().unwrap().control_change(cc, value);
                    println!("✓ Sent CC{} = {}", cc, value);
                } else {
                    println!("✗ Invalid CC number or value");
                }
            }

            // Demo scenarios
            "demo" => {
                println!("\nRunning demonstration scenarios...\n");
                run_demo(&simulator);
                println!("\n✓ Demo complete");
            }

            "scenario" | "s" => {
                if parts.len() < 2 {
                    println!("Available scenarios:");
                    println!("  1. velocity    - Test all velocity levels");
                    println!("  2. timing      - Test short/medium/long press");
                    println!("  3. doubletap   - Test double-tap detection");
                    println!("  4. chord       - Test chord detection");
                    println!("  5. encoder     - Test encoder rotation");
                    println!("  6. complex     - Complex mixed scenario");
                    continue;
                }

                match parts[1] {
                    "velocity" | "1" => run_velocity_scenario(&simulator),
                    "timing" | "2" => run_timing_scenario(&simulator),
                    "doubletap" | "3" => run_doubletap_scenario(&simulator),
                    "chord" | "4" => run_chord_scenario(&simulator),
                    "encoder" | "5" => run_encoder_scenario(&simulator),
                    "complex" | "6" => run_complex_scenario(&simulator),
                    _ => println!("✗ Unknown scenario"),
                }
            }

            _ => {
                println!(
                    "✗ Unknown command '{}'. Type 'help' for available commands.",
                    command
                );
            }
        }
    }
}

fn print_help() {
    println!("╭─────────────────────────────────────────────────────────────╮");
    println!("│ COMMANDS                                                    │");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│ Basic:                                                      │");
    println!("│   help, h, ?              Show this help message            │");
    println!("│   quit, exit, q           Exit the simulator                │");
    println!("│   clear, c                Clear event queue                 │");
    println!("│   events, e               Show captured events              │");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│ MIDI Events:                                                │");
    println!("│   note <num> <vel>        Send note on/off                  │");
    println!("│   velocity <note>         Test velocity levels              │");
    println!("│   long <note> [ms]        Simulate long press               │");
    println!("│   double <note> [gap_ms]  Simulate double-tap               │");
    println!("│   chord <n1> <n2> ...     Simulate chord                    │");
    println!("│   encoder <cc> <cw|ccw>   Simulate encoder rotation         │");
    println!("│   aftertouch <pressure>   Send aftertouch                   │");
    println!("│   pitch <value>           Send pitch bend (0-16383)         │");
    println!("│   cc <num> <val>          Send control change               │");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│ Scenarios:                                                  │");
    println!("│   demo                    Run full demonstration            │");
    println!("│   scenario [name]         Run specific test scenario        │");
    println!("╰─────────────────────────────────────────────────────────────╯");
}

fn run_demo(simulator: &Arc<Mutex<MidiSimulator>>) {
    println!("1. Testing velocity levels...");
    let sim = simulator.lock().unwrap();
    sim.perform_gesture(Gesture::VelocityRamp {
        note: 60,
        min_velocity: 20,
        max_velocity: 120,
        steps: 3,
    });
    drop(sim);
    thread::sleep(Duration::from_millis(500));

    println!("2. Testing long press...");
    simulator
        .lock()
        .unwrap()
        .perform_gesture(Gesture::LongPress {
            note: 61,
            velocity: 80,
            hold_ms: 2500,
        });

    println!("3. Testing double-tap...");
    simulator
        .lock()
        .unwrap()
        .perform_gesture(Gesture::DoubleTap {
            note: 62,
            velocity: 80,
            tap_duration_ms: 50,
            gap_ms: 200,
        });

    println!("4. Testing chord...");
    simulator.lock().unwrap().perform_gesture(Gesture::Chord {
        notes: vec![60, 64, 67],
        velocity: 80,
        stagger_ms: 10,
        hold_ms: 500,
    });

    println!("5. Testing encoder...");
    simulator
        .lock()
        .unwrap()
        .perform_gesture(Gesture::EncoderTurn {
            cc: 1,
            direction: EncoderDirection::Clockwise,
            steps: 10,
            step_delay_ms: 30,
        });
}

fn run_velocity_scenario(simulator: &Arc<Mutex<MidiSimulator>>) {
    println!("Testing velocity levels: Soft (30), Medium (70), Hard (110)");
    let sim = simulator.lock().unwrap();

    sim.note_on(60, 30);
    thread::sleep(Duration::from_millis(100));
    sim.note_off(60);
    thread::sleep(Duration::from_millis(200));

    sim.note_on(60, 70);
    thread::sleep(Duration::from_millis(100));
    sim.note_off(60);
    thread::sleep(Duration::from_millis(200));

    sim.note_on(60, 110);
    thread::sleep(Duration::from_millis(100));
    sim.note_off(60);

    println!("✓ Velocity scenario complete");
}

fn run_timing_scenario(simulator: &Arc<Mutex<MidiSimulator>>) {
    println!("Testing press durations: Short (100ms), Medium (500ms), Long (2500ms)");

    // Short press
    simulator
        .lock()
        .unwrap()
        .perform_gesture(Gesture::SimpleTap {
            note: 60,
            velocity: 80,
            duration_ms: 100,
        });
    thread::sleep(Duration::from_millis(300));

    // Medium press
    simulator
        .lock()
        .unwrap()
        .perform_gesture(Gesture::SimpleTap {
            note: 60,
            velocity: 80,
            duration_ms: 500,
        });
    thread::sleep(Duration::from_millis(300));

    // Long press
    simulator
        .lock()
        .unwrap()
        .perform_gesture(Gesture::LongPress {
            note: 60,
            velocity: 80,
            hold_ms: 2500,
        });

    println!("✓ Timing scenario complete");
}

fn run_doubletap_scenario(simulator: &Arc<Mutex<MidiSimulator>>) {
    println!("Testing double-tap with 200ms gap");
    simulator
        .lock()
        .unwrap()
        .perform_gesture(Gesture::DoubleTap {
            note: 60,
            velocity: 80,
            tap_duration_ms: 50,
            gap_ms: 200,
        });
    println!("✓ Double-tap scenario complete");
}

fn run_chord_scenario(simulator: &Arc<Mutex<MidiSimulator>>) {
    println!("Testing chord detection: C major (60, 64, 67)");
    simulator.lock().unwrap().perform_gesture(Gesture::Chord {
        notes: vec![60, 64, 67],
        velocity: 80,
        stagger_ms: 10,
        hold_ms: 500,
    });
    println!("✓ Chord scenario complete");
}

fn run_encoder_scenario(simulator: &Arc<Mutex<MidiSimulator>>) {
    println!("Testing encoder: 5 steps CW, then 5 steps CCW");

    simulator
        .lock()
        .unwrap()
        .perform_gesture(Gesture::EncoderTurn {
            cc: 1,
            direction: EncoderDirection::Clockwise,
            steps: 5,
            step_delay_ms: 50,
        });

    thread::sleep(Duration::from_millis(500));

    simulator
        .lock()
        .unwrap()
        .perform_gesture(Gesture::EncoderTurn {
            cc: 1,
            direction: EncoderDirection::CounterClockwise,
            steps: 5,
            step_delay_ms: 50,
        });

    println!("✓ Encoder scenario complete");
}

fn run_complex_scenario(simulator: &Arc<Mutex<MidiSimulator>>) {
    println!("Running complex scenario: mixed events...");

    let sim = simulator.lock().unwrap();

    // Note + encoder
    sim.note_on(60, 80);
    thread::sleep(Duration::from_millis(100));
    sim.control_change(1, 70);
    thread::sleep(Duration::from_millis(100));
    sim.note_off(60);

    thread::sleep(Duration::from_millis(300));

    // Chord + aftertouch
    sim.note_on(60, 80);
    sim.note_on(64, 80);
    sim.note_on(67, 80);
    thread::sleep(Duration::from_millis(200));
    sim.aftertouch(100);
    thread::sleep(Duration::from_millis(300));
    sim.note_off(60);
    sim.note_off(64);
    sim.note_off(67);

    thread::sleep(Duration::from_millis(300));

    // Pitch bend + notes
    sim.pitch_bend(8192); // Center
    thread::sleep(Duration::from_millis(100));
    sim.note_on(72, 100);
    thread::sleep(Duration::from_millis(100));
    sim.pitch_bend(12000); // Bend up
    thread::sleep(Duration::from_millis(100));
    sim.pitch_bend(8192); // Back to center
    thread::sleep(Duration::from_millis(100));
    sim.note_off(72);

    println!("✓ Complex scenario complete");
}
