# Copilot Instructions for AI Agents

## Project Overview
This is a Rust-based MIDI monitoring and macro pad tool. The main components are:
- `src/main.rs`: Entry point, orchestrates config loading, MIDI event handling, and action dispatch.
- `src/config.rs`: Loads and parses configuration from `config.toml`.
- `src/actions.rs`: Defines actions triggered by MIDI events (e.g., macros, system commands).
- `src/mappings.rs`: Maps MIDI events to actions based on config.
- `config.toml`: User-editable config for MIDI device, mappings, and macros.

## Architecture & Data Flow
- On startup, `main.rs` loads configuration via `config.rs`.
- MIDI events are received and mapped to actions using logic in `mappings.rs`.
- Actions are executed as defined in `actions.rs` (e.g., running shell commands, triggering macros).
- All configuration is centralized in `config.toml`.

## Developer Workflows
- **Build:** Use `cargo build` to compile. Binaries are in `target/debug/`.
- **Run:** Use `cargo run` or execute the binary directly.
- **Test:** If tests exist, run with `cargo test`. (No test files detected; add tests in `src/bin/` or as unit tests in modules.)
- **Debug:** Use `RUST_LOG=debug cargo run` for verbose logging if implemented.
- **Config changes:** Edit `config.toml` and restart the app.

## Project-Specific Patterns
- All MIDI-to-action logic is driven by config; avoid hardcoding mappings in Rust files.
- Use Rust enums and pattern matching for MIDI event handling and action dispatch.
- Keep new actions modular in `actions.rs` and update mapping logic in `mappings.rs`.
- Place experimental binaries in `src/bin/` (e.g., `test_midi.rs`).

## Integration Points & Dependencies
- Uses `midir` and `coremidi` crates for MIDI I/O.
- May use `serde` for config parsing.
- External commands/macros are triggered via Rust's process APIs.

## Examples
- To add a new macro: define it in `config.toml`, implement its logic in `actions.rs`, and map it in `mappings.rs`.
- To support a new MIDI device: update config and ensure device handling in `main.rs` and `config.rs`.

## Key Files
- `src/main.rs`, `src/config.rs`, `src/actions.rs`, `src/mappings.rs`, `config.toml`

---

If any section is unclear or missing details, please provide feedback or specify which workflows, patterns, or integrations need further documentation.
