# Third-Party Licenses

MIDIMon depends on the following third-party Rust crates. All dependencies use OSI-approved licenses compatible with the MIT License.

## Direct Dependencies

### midir (MIT)
Cross-platform MIDI I/O library
- License: MIT
- Repository: https://github.com/Boddlnagg/midir

### enigo (Apache-2.0)
Cross-platform keyboard and mouse input simulation
- License: Apache-2.0
- Repository: https://github.com/enigo-rs/enigo

### hidapi (BSD-3-Clause)
HID device access with macOS shared device support
- License: BSD-3-Clause
- Repository: https://github.com/ruabmbua/hidapi-rs

### serde (MIT/Apache-2.0)
Serialization framework
- License: MIT OR Apache-2.0
- Repository: https://github.com/serde-rs/serde

### toml (MIT/Apache-2.0)
TOML parser
- License: MIT OR Apache-2.0
- Repository: https://github.com/toml-rs/toml

### quick-xml (MIT)
XML parser for device profiles
- License: MIT
- Repository: https://github.com/tafia/quick-xml

### crossbeam-channel (MIT/Apache-2.0)
Multi-producer multi-consumer channels
- License: MIT OR Apache-2.0
- Repository: https://github.com/crossbeam-rs/crossbeam

### colored (MPL-2.0)
Terminal color output
- License: MPL-2.0
- Repository: https://github.com/colored-rs/colored

### ctrlc (MIT/Apache-2.0)
Signal handling for graceful shutdown
- License: MIT OR Apache-2.0
- Repository: https://github.com/Detegr/rust-ctrlc

## License Compatibility

All dependencies use licenses compatible with MIDIMon's MIT License:

- **MIT License**: Fully compatible (most permissive)
- **Apache-2.0**: Compatible with MIT (adds patent grant)
- **BSD-3-Clause**: Compatible with MIT (similar permissive terms)
- **MPL-2.0**: Compatible with MIT (file-level copyleft)

No GPL, LGPL, or AGPL dependencies (which would require copyleft terms).

## Full License Texts

For complete license texts of all dependencies, run:
```bash
cargo install cargo-license
cargo license --all-features
```

Or visit each repository's LICENSE file linked above.

## Audit

License audit performed: 2025-11-11
MIDIMon version: v0.1.0-monolithic

To verify dependency licenses:
```bash
cargo install cargo-license
cargo license --all-features | grep -v "MIT\|Apache-2.0\|BSD\|MPL-2.0"
```

Any non-standard licenses should be manually reviewed before inclusion.
