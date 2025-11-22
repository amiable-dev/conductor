# MIDI Macro Pad & Monitor

This project is a Rust-based MIDI monitoring and macro pad tool. It allows you to map MIDI events to custom actions, such as running shell commands or triggering macros, using a simple configuration file.

## Project Structure

```text
midi-macro-pad/
├── Cargo.toml
├── src/
│   ├── main.rs         # Entry point, config loading, MIDI event handling
│   ├── config.rs       # Loads/parses config.toml
│   ├── actions.rs      # Defines actions triggered by MIDI events
│   └── mappings.rs     # Maps MIDI events to actions
└── config.toml         # User-editable config for device, mappings, macros
```

## How to Use

1. **Configure:** Edit `config.toml` to set up your MIDI device, event mappings, and macros.
2. **Build:** Run `cargo build` to compile the project. Binaries are in `target/debug/`.
3. **Run:** Start the app with `cargo run` or execute the binary directly.
4. **Test:** (If tests exist) Run `cargo test`. Experimental binaries can be placed in `src/bin/`.
5. **Debug:** For verbose logging, use `RUST_LOG=debug cargo run` (if logging is implemented).
6. **Update Config:** After changing `config.toml`, restart the app to apply changes.

## Key Features
- All MIDI-to-action logic is driven by config; avoid hardcoding mappings in Rust files.
- Modular action definitions in `actions.rs` and mapping logic in `mappings.rs`.
- Uses `midir` and `coremidi` crates for MIDI I/O, and `serde` for config parsing.
- External commands/macros are triggered via Rust's process APIs.

## Example Workflow
- To add a new macro: define it in `config.toml`, implement its logic in `actions.rs`, and map it in `mappings.rs`.
- To support a new MIDI device: update config and ensure device handling in `main.rs` and `config.rs`.

---

For more details, see `.github/copilot-instructions.md`.