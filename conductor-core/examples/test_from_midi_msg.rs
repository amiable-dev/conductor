use conductor_core::event_processor::MidiEvent;

fn main() {
    // Test Note On
    let note_on_bytes = [0x90, 60, 100]; // Note On, C4, velocity 100
    match MidiEvent::from_midi_msg(&note_on_bytes) {
        Ok(MidiEvent::NoteOn { note, velocity, .. }) => {
            println!("✓ Note On: note={}, velocity={}", note, velocity);
            assert_eq!(note, 60);
            assert_eq!(velocity, 100);
        }
        other => panic!("Expected NoteOn, got {:?}", other),
    }

    // Test Note On with velocity 0 (should become Note Off)
    let note_on_vel0_bytes = [0x90, 60, 0];
    match MidiEvent::from_midi_msg(&note_on_vel0_bytes) {
        Ok(MidiEvent::NoteOff { note, .. }) => {
            println!("✓ Note On velocity 0 → Note Off: note={}", note);
            assert_eq!(note, 60);
        }
        other => panic!("Expected NoteOff, got {:?}", other),
    }

    // Test Note Off
    let note_off_bytes = [0x80, 60, 64]; // Note Off, C4
    match MidiEvent::from_midi_msg(&note_off_bytes) {
        Ok(MidiEvent::NoteOff { note, .. }) => {
            println!("✓ Note Off: note={}", note);
            assert_eq!(note, 60);
        }
        other => panic!("Expected NoteOff, got {:?}", other),
    }

    // Test Control Change
    let cc_bytes = [0xB0, 7, 127]; // CC 7 (volume), value 127
    match MidiEvent::from_midi_msg(&cc_bytes) {
        Ok(MidiEvent::ControlChange { cc, value, .. }) => {
            println!("✓ Control Change: cc={}, value={}", cc, value);
            assert_eq!(cc, 7);
            assert_eq!(value, 127);
        }
        other => panic!("Expected ControlChange, got {:?}", other),
    }

    // Test Poly Pressure
    let poly_pressure_bytes = [0xA0, 60, 80]; // Poly Pressure, note 60, pressure 80
    match MidiEvent::from_midi_msg(&poly_pressure_bytes) {
        Ok(MidiEvent::PolyPressure { note, pressure, .. }) => {
            println!("✓ Poly Pressure: note={}, pressure={}", note, pressure);
            assert_eq!(note, 60);
            assert_eq!(pressure, 80);
        }
        other => panic!("Expected PolyPressure, got {:?}", other),
    }

    // Test Channel Pressure (Aftertouch)
    let aftertouch_bytes = [0xD0, 100]; // Channel Pressure, pressure 100
    match MidiEvent::from_midi_msg(&aftertouch_bytes) {
        Ok(MidiEvent::Aftertouch { pressure, .. }) => {
            println!("✓ Channel Pressure: pressure={}", pressure);
            assert_eq!(pressure, 100);
        }
        other => panic!("Expected Aftertouch, got {:?}", other),
    }

    // Test Pitch Bend
    let pitch_bend_bytes = [0xE0, 0, 64]; // Pitch Bend, center position (8192)
    match MidiEvent::from_midi_msg(&pitch_bend_bytes) {
        Ok(MidiEvent::PitchBend { value, .. }) => {
            println!("✓ Pitch Bend: value={}", value);
            assert_eq!(value, 8192);
        }
        other => panic!("Expected PitchBend, got {:?}", other),
    }

    // Test Program Change
    let program_change_bytes = [0xC0, 42]; // Program Change, program 42
    match MidiEvent::from_midi_msg(&program_change_bytes) {
        Ok(MidiEvent::ProgramChange { program, .. }) => {
            println!("✓ Program Change: program={}", program);
            assert_eq!(program, 42);
        }
        other => panic!("Expected ProgramChange, got {:?}", other),
    }

    // Test unsupported message (System Real-Time)
    let clock_bytes = [0xF8]; // MIDI Clock
    match MidiEvent::from_midi_msg(&clock_bytes) {
        Err(msg) => {
            println!("✓ Unsupported message correctly rejected: {}", msg);
            assert!(msg.contains("Real-Time"));
        }
        other => panic!("Expected error, got {:?}", other),
    }

    println!("\n✅ All tests passed!");
}
