# Migration Guide: v0.1.0 → v0.2.0

**Status**: No action required for most users  
**Breaking Changes**: 0 (zero)  
**Compatibility**: 100% backward compatible

## Summary

v0.2.0 introduces a **workspace architecture** with no breaking changes. Your existing configs and workflows continue to work identically.

## What Changed?

### Architecture

**v0.1.0 (Monolithic)**:
- Single binary with all code in one crate
- src/ contains all modules

**v0.2.0 (Workspace)**:
- 3-package workspace structure
- `midimon-core`: Pure Rust engine library
- `midimon-daemon`: CLI daemon + tools
- `midimon`: Backward compatibility layer

### For End Users

**No changes required.** Your workflow is identical:

```bash
# v0.1.0 command
./midimon 2

# v0.2.0 command (same)
./midimon 2
```

All config files work without modification:
- Same TOML format
- Same trigger types
- Same action types
- Same LED schemes

### For Developers

#### Building

**v0.1.0**:
```bash
cargo build --release
```

**v0.2.0**:
```bash
cargo build --release --workspace
```

#### Testing

**v0.1.0**:
```bash
cargo test
```

**v0.2.0**:
```bash
cargo test --workspace
```

#### Imports (if writing code against MIDIMon)

**Old imports (still work via compatibility layer)**:
```rust
use midimon::config::Config;
use midimon::mappings::MappingEngine;
use midimon::actions::ActionExecutor;
```

**New imports (recommended for new code)**:
```rust
use midimon_core::Config;
use midimon_core::mapping::MappingEngine;  // Note: renamed from "mappings"
use midimon_core::actions::ActionExecutor;
```

**Note**: The compatibility layer ensures old imports continue working.

## Performance Improvements

| Metric | v0.1.0 | v0.2.0 | Change |
|--------|--------|--------|---------|
| Clean build time | 15-20s | 11.92s | ✅ 25-40% faster |
| Test execution | ~30s | 28.8s | ✅ 4% faster |
| Binary size | 869K | 869K | Same |

Faster builds due to workspace parallelization.

## New Capabilities (v0.2.0)

1. **Reusable Library**: `midimon-core` can be embedded in other applications
2. **Modular Architecture**: Easier to add new features (e.g., Tauri UI)
3. **Better Testing**: Workspace allows per-package testing
4. **Error Types**: Structured errors with `thiserror`

## Migration Checklist

### For Users

- [ ] Update binary to v0.2.0
- [ ] Verify your config still loads (`./midimon --help`)
- [ ] Test your mappings work as expected

**That's it!** No other changes needed.

### For Developers

- [ ] Update build commands to use `--workspace`
- [ ] Update CI scripts if building from source
- [ ] Consider using new `midimon_core` imports for new code
- [ ] Review `midimon-core/src/lib.rs` for public API

### For CI/CD

Update GitHub Actions or CI scripts:

```yaml
# Old
- run: cargo build --release
- run: cargo test

# New
- run: cargo build --release --workspace
- run: cargo test --workspace
```

## Rollback

If you encounter issues (unlikely), you can roll back:

```bash
git checkout v0.1.0-monolithic
cargo build --release
```

The v0.1.0 implementation is preserved at the `v0.1.0-monolithic` tag.

## FAQ

### Do I need to update my config.toml?
**No.** All v0.1.0 configs work identically in v0.2.0.

### Will my LED schemes still work?
**Yes.** All 10 LED schemes work identically.

### Do diagnostic tools still work?
**Yes.** All 6 diagnostic tools (`midi_diagnostic`, `led_tester`, etc.) work identically.

### Can I use midimon-core in my own project?
**Yes!** That's one of the benefits of v0.2.0. Add to your `Cargo.toml`:

```toml
[dependencies]
midimon-core = { path = "../midimon/midimon-core" }
# Or from crates.io when published:
# midimon-core = "0.2"
```

### What about Phase 3 (Tauri UI)?
Phase 3 is future work. v0.2.0 maintains the same CLI interface while preparing the architecture for a future GUI.

## Support

- **Issues**: https://github.com/amiable-dev/midimon/issues
- **Discussions**: https://github.com/amiable-dev/midimon/discussions
- **Documentation**: https://amiable-dev.github.io/midimon/

## Validation

v0.2.0 was thoroughly validated:
- ✅ 339 tests passing (100%)
- ✅ All 26 features tested
- ✅ Zero breaking changes
- ✅ Performance improved

See `docs/phase-2-step-8-validation.md` for complete validation results.
