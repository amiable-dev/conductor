# MIDIMon Benchmark Suite

Comprehensive performance benchmarking for the MIDIMon MIDI mapping engine using [Criterion.rs](https://bheisler.github.io/criterion.rs/book/criterion_rs.html).

## Overview

The benchmark suite validates performance targets and provides regression detection across the core MIDI processing pipeline:

1. **Event Processing** (EventProcessor)
2. **Mapping Engine** (MappingEngine)
3. **Action Execution** (ActionExecutor)
4. **End-to-End Pipeline** (complete flow)

## Performance Targets

| Component | Target | Typical |
|-----------|--------|---------|
| MIDI Event Processing | <200μs | ~50-100μs |
| Mapping Lookup (10 mappings) | <300μs | ~100-150μs |
| Event → Action Pipeline | <500μs | ~150-250μs |
| Complete E2E (no action execution) | <1ms | ~300-500μs |
| Action Execution (OS-dependent) | <500μs | Varies by action |

## Quick Start

### Run All Benchmarks

```bash
# Run all benchmarks with default settings
cargo bench --package midimon-core

# Run specific benchmark suite
cargo bench --package midimon-core event_processing
cargo bench --package midimon-core mapping_engine
cargo bench --package midimon-core action_executor
cargo bench --package midimon-core end_to_end
```

### Run with Custom Configuration

```bash
# Run with shorter measurement time (faster iteration)
cargo bench --package midimon-core -- --measurement-time 5

# Run specific benchmark group
cargo bench --package midimon-core -- event_processing::note_on

# Run benchmarks and save results for comparison
cargo bench --package midimon-core -- --save-baseline my_baseline

# Compare against a saved baseline
cargo bench --package midimon-core -- --baseline my_baseline
```

## Benchmark Files

### 1. event_processing.rs

Benchmarks for `EventProcessor::process()` - the MIDI event → ProcessedEvent transformation.

**Key Benchmarks:**

- `note_on` - Basic note-on event processing
- `note_off` - Note release detection
- `velocity_detection` - Velocity level categorization (soft/medium/hard)
- `control_change` - CC/encoder event processing
- `encoder_direction_detection` - Clockwise vs counter-clockwise detection
- `aftertouch` - Pressure sensitivity events
- `pitch_bend` - Touch strip events
- `program_change` - Bank/program change events
- `consecutive_notes` - Stream of rapid note inputs

**Performance Characteristics:**

- Single event: ~50-100μs
- Chord detection: ~200-300μs
- Double-tap detection: ~100-150μs
- Stream processing: ~50-100μs per event

### 2. mapping_engine.rs

Benchmarks for `MappingEngine::get_action()` - matching MIDI events to configured actions.

**Key Benchmarks:**

- `simple_note_mapping` - Single mapping lookup
- `no_match_lookup` - Failure case (event not mapped)
- `mapping_set_scaling` - Performance with 1, 5, 10, 20, 50 mappings
- `mode_specific_lookup` - Mode-based mapping retrieval
- `cc_mapping_lookup` - Control change mapping
- `mode_switching_with_lookup` - Rapid mode changes
- `rapid_note_lookups` - Sequential vs random note access
- `note_off_lookup` - Release event matching

**Performance Characteristics:**

- Simple lookup: ~100-150μs
- 10 mappings: ~150-200μs
- 50 mappings: ~200-300μs
- Scaling is sub-linear (HashMap with good cache locality)

### 3. action_executor.rs

Benchmarks for `ActionExecutor::execute()` - action execution dispatch.

**Key Benchmarks:**

- `simple_keystroke` - Single key press
- `modified_keystroke` - Key with modifiers
- `text_action` - Text typing simulation
- `delay_action` - Delay action execution
- `sequence_action` - Multiple actions in sequence
- `executor_creation` - Enigo initialization
- `dispatch_overhead` - Routing overhead
- `mouse_click_action` - Mouse movement and clicks
- `launch_action` (macOS) - Application launching
- `shell_action` - Shell command execution

**Performance Characteristics:**

⚠️ Note: Action execution times vary significantly based on OS state and focus. These benchmarks are provided for tracking, but expect high variance:

- Keystroke dispatch: ~50-100μs
- Keystroke with modifiers: ~100-200μs
- Text typing: ~100-300μs per character
- Delay (0ms): ~5-10μs overhead
- Mouse operations: ~200-400μs
- Application launch: ~500-2000μs (highly variable)
- Shell commands: ~1-5ms (highly variable)

### 4. end_to_end.rs

Benchmarks for the complete MIDI processing pipeline without executing actions.

**Key Benchmarks:**

- `simple_note_press` - Note on → action lookup (no execution)
- `pipeline_without_execution` - E2E without OS action execution
- `pipeline_scaling` - Performance with 1, 5, 10, 20, 50 mappings
- `rapid_note_input` - 8 consecutive notes
- `encoder_input` - Control change processing
- `note_release_processing` - Note off path
- `velocity_sensitive_processing` - Different velocities
- `mode_switching` - Mode changes during input
- `mixed_input_pattern` - Realistic interaction pattern

**Performance Characteristics:**

- Simple note: ~100-150μs
- Full pipeline (10 mappings): ~150-250μs
- 50 mappings: ~250-350μs
- Rapid input (8 notes): ~800-1200μs total (~100-150μs per note)

## Interpreting Results

### Criterion Output

Criterion provides detailed statistical analysis:

```
simple_note_press                 time:   [156.25 ms 158.42 ms 160.78 ms]
```

- First value: Lower confidence bound
- Second value: Point estimate (median)
- Third value: Upper confidence bound

### Statistical Significance

Criterion detects performance regressions with 99% confidence. Look for:

- `!` symbol: Significant change detected
- `?` symbol: Inconclusive (high variance)

### Variance Factors

High variance can be caused by:

- System load and CPU frequency scaling
- OS scheduler interaction
- Cache effects
- Background processes

## Regression Detection

### Saving Baselines

Establish a baseline for comparison:

```bash
cargo bench --package midimon-core -- --save-baseline v0.2.0
```

This creates: `target/criterion/baseline/v0.2.0/`

### Comparing Baselines

After making changes, compare against the baseline:

```bash
cargo bench --package midimon-core -- --baseline v0.2.0
```

Criterion will report any significant regressions.

### CI Integration

For CI/CD pipelines, run with baseline comparison:

```bash
# In your CI pipeline
cargo bench --package midimon-core -- --baseline main --output-format bencher
```

## HTML Reports

Criterion generates detailed HTML reports:

```bash
# After running benchmarks
open target/criterion/report/index.html
```

Reports include:
- Execution time distributions
- Regression/improvement detection
- Throughput analysis
- Historical comparisons

## Performance Analysis Workflow

1. **Establish baseline** before making changes
   ```bash
   cargo bench --package midimon-core -- --save-baseline before_changes
   ```

2. **Make code changes**

3. **Run benchmarks** and compare
   ```bash
   cargo bench --package midimon-core -- --baseline before_changes
   ```

4. **Review HTML report**
   ```bash
   open target/criterion/report/index.html
   ```

5. **Accept or revert** based on results

## Optimization Guidelines

### Event Processing

If `event_processing` benchmarks show regression:

1. Check for new allocations in hot path
2. Verify HashMap/Vec operations are efficient
3. Review timing calculations
4. Consider inlining for small functions

### Mapping Engine

If `mapping_engine` benchmarks show regression:

1. Review mapping lookup logic
2. Check HashMap key distribution
3. Verify cache efficiency (sequential vs random access)
4. Consider pre-computed indices for common patterns

### End-to-End Pipeline

If `e2e` benchmarks show regression:

1. Profile with `perf` or Instruments
2. Identify which stage is slow (processor vs engine)
3. Check for unexpected allocations
4. Review data structure choices

## Platform-Specific Notes

### macOS

- `launch_action` benchmark includes `open -a` overhead
- Mouse/keyboard simulation may have system effects
- Run benchmarks on consistent power settings (plugged in, max performance)

### Linux

- May have different scheduler behavior affecting variance
- Check CPU governor settings (`cpupower` or `/sys/devices/system/cpu/cpu*/cpufreq/`)

### Windows

- Timing may be less precise on some systems
- Consider running multiple times to get stable results

## Dependencies

Benchmarks use:

- **criterion** - Statistical benchmarking framework
- **midimon-core** - The library being benchmarked
- **enigo** - For action execution benchmarks

## Benchmark Maintenance

### When to Update Benchmarks

- Add benchmarks when:
  - New features are added (especially hot path)
  - Performance-critical code is refactored
  - New action or trigger types are implemented

- Update baselines when:
  - Intentional performance improvements are made
  - Regressions are fixed
  - Target performance is changed

### Removing Benchmarks

Only remove benchmarks if:

- The feature is deprecated/removed
- The benchmark is no longer relevant
- Better benchmarks replace it

## Tips for Consistent Results

1. **Close unnecessary applications** to reduce system variance
2. **Disable CPU frequency scaling** if possible
3. **Run multiple times** - use `--sample-size 100` for more stable results
4. **Warm up** - run benchmarks twice (first run warms up)
5. **Compare on same hardware** - use same machine when comparing baselines
6. **Use save/load baselines** for precise regression detection

## Troubleshooting

### High Variance in Results

```bash
# Increase sample size for more stable measurements
cargo bench --package midimon-core -- --sample-size 100
```

### Benchmark Won't Compile

Ensure you're using Rust 1.70+:

```bash
rustc --version
cargo update
```

### Performance Seems Wrong

Check if you're using release mode:

```bash
# Benchmarks automatically use release mode, but verify
cargo bench --package midimon-core --release
```

## References

- [Criterion.rs Book](https://bheisler.github.io/criterion.rs/book/)
- [Criterion Best Practices](https://bheisler.github.io/criterion.rs/book/user_guide/best_practices.html)
- [Statistical Benchmarking](https://bheisler.github.io/criterion.rs/book/user_guide/stats.html)

## Future Enhancements

Proposed benchmark improvements:

- [ ] GPU-assisted benchmarking (for future Vulkan/Metal LED feedback)
- [ ] Multi-threaded benchmarks (parallel event processing)
- [ ] Memory allocation profiling
- [ ] Cache miss analysis
- [ ] Flame graph integration
- [ ] Continuous benchmarking CI/CD integration
- [ ] Benchmark visualization dashboard
- [ ] Compiler version impact analysis

---

**Last Updated:** November 2025
**MIDIMon Version:** 0.2.0
**Benchmark Suite:** v1.0
