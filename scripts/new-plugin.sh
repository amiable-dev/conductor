#!/bin/bash
# MIDIMon WASM Plugin Scaffolding Tool
# Creates a new WASM plugin from templates

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
PLUGINS_DIR="$PROJECT_ROOT/plugins"

# Print colored message
print_info() {
    echo -e "${BLUE}ℹ${NC}  $1"
}

print_success() {
    echo -e "${GREEN}✅${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC}  $1"
}

# Print usage
print_usage() {
    cat <<EOF
MIDIMon WASM Plugin Scaffolding Tool

Usage: ./scripts/new-plugin.sh [OPTIONS] <plugin-name>

Arguments:
  <plugin-name>    Name of the plugin (e.g., my-awesome-plugin)

Options:
  -t, --template <type>   Plugin template to use
                          Options: minimal, template, json
                          Default: template
  -a, --author <name>     Plugin author name
  -d, --description <desc> Plugin description
  -l, --license <license>  Plugin license (default: MIT)
  -h, --help              Show this help message

Templates:
  minimal    - Minimal plugin (~300 bytes, no dependencies)
  template   - Full-featured template (~108KB, best for most plugins)
  json       - JSON-enabled template (~54KB, for complex data handling)

Examples:
  # Create minimal plugin
  ./scripts/new-plugin.sh -t minimal my-plugin

  # Create template plugin with custom author
  ./scripts/new-plugin.sh -a "John Doe" my-awesome-plugin

  # Create JSON plugin with description
  ./scripts/new-plugin.sh -t json -d "Spotify control" spotify-plugin
EOF
}

# Default values
TEMPLATE="template"
AUTHOR="Your Name"
DESCRIPTION=""
LICENSE="MIT"
PLUGIN_NAME=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--template)
            TEMPLATE="$2"
            shift 2
            ;;
        -a|--author)
            AUTHOR="$2"
            shift 2
            ;;
        -d|--description)
            DESCRIPTION="$2"
            shift 2
            ;;
        -l|--license)
            LICENSE="$2"
            shift 2
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        -*)
            print_error "Unknown option: $1"
            print_usage
            exit 1
            ;;
        *)
            PLUGIN_NAME="$1"
            shift
            ;;
    esac
done

# Validate plugin name
if [ -z "$PLUGIN_NAME" ]; then
    print_error "Plugin name is required"
    print_usage
    exit 1
fi

# Validate plugin name format (lowercase, hyphens only)
if ! [[ "$PLUGIN_NAME" =~ ^[a-z][a-z0-9-]*$ ]]; then
    print_error "Invalid plugin name: must start with lowercase letter and contain only lowercase letters, numbers, and hyphens"
    exit 1
fi

# Set default description if not provided
if [ -z "$DESCRIPTION" ]; then
    DESCRIPTION="MIDIMon plugin: $PLUGIN_NAME"
fi

# Validate template
case $TEMPLATE in
    minimal|template|json)
        ;;
    *)
        print_error "Invalid template: $TEMPLATE"
        echo "Valid templates: minimal, template, json"
        exit 1
        ;;
esac

# Convert plugin name to various formats
PLUGIN_NAME_SNAKE=$(echo "$PLUGIN_NAME" | tr '-' '_')
PLUGIN_CRATE_NAME="midimon-wasm-$PLUGIN_NAME"
PLUGIN_DIR="$PLUGINS_DIR/wasm-$PLUGIN_NAME"

# Check if plugin already exists
if [ -d "$PLUGIN_DIR" ]; then
    print_error "Plugin already exists: $PLUGIN_DIR"
    exit 1
fi

print_info "Creating new WASM plugin: $PLUGIN_NAME"
print_info "  Template: $TEMPLATE"
print_info "  Author: $AUTHOR"
print_info "  License: $LICENSE"
print_info "  Description: $DESCRIPTION"
echo

# Create plugin directory
mkdir -p "$PLUGIN_DIR/src"

# Create Cargo.toml based on template
create_cargo_toml() {
    local has_json=$1

    cat > "$PLUGIN_DIR/Cargo.toml" <<EOF
[package]
name = "$PLUGIN_CRATE_NAME"
version = "0.1.0"
edition = "2021"
authors = ["$AUTHOR"]
description = "$DESCRIPTION"
license = "$LICENSE"

[workspace]
# Empty workspace to exclude from parent workspace

[lib]
crate-type = ["cdylib"]

EOF

    if [ "$has_json" = "true" ]; then
        cat >> "$PLUGIN_DIR/Cargo.toml" <<EOF
[dependencies]
serde = { version = "1.0", features = ["derive"], default-features = false }
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
wee_alloc = "0.4"

EOF
    fi

    cat >> "$PLUGIN_DIR/Cargo.toml" <<EOF
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Enable link-time optimization
codegen-units = 1   # Reduce number of codegen units
panic = "abort"     # Abort on panic
strip = true        # Strip symbols
EOF
}

# Create src/lib.rs based on template
create_lib_rs_minimal() {
    cat > "$PLUGIN_DIR/src/lib.rs" <<'EOF'
// Copyright 2025 {{ AUTHOR }}
// SPDX-License-Identifier: {{ LICENSE }}

//! {{ DESCRIPTION }}

#![no_std]

// Plugin metadata (embedded in binary)
static METADATA: &[u8] = br#"{
  "name": "{{ PLUGIN_NAME }}",
  "version": "0.1.0",
  "description": "{{ DESCRIPTION }}",
  "author": "{{ AUTHOR }}",
  "license": "{{ LICENSE }}",
  "type": "action",
  "capabilities": []
}"#;

// ============================================================================
// WASM Exports (Plugin Interface)
// ============================================================================

/// Initialize plugin and return metadata
///
/// Returns: (ptr, len) packed in u64 (ptr in high 32 bits, len in low 32 bits)
#[no_mangle]
pub extern "C" fn init() -> u64 {
    let ptr = METADATA.as_ptr() as u32;
    let len = METADATA.len() as u32;
    ((ptr as u64) << 32) | (len as u64)
}

/// Execute plugin action
///
/// Arguments:
/// - ptr: Pointer to JSON request string in WASM memory
/// - len: Length of JSON request string
///
/// Returns: 0 on success, non-zero error code on failure
#[no_mangle]
pub extern "C" fn execute(_ptr: u32, _len: u32) -> i32 {
    // TODO: Implement your plugin logic here
    0 // Success
}

/// Allocate memory in WASM linear memory (called by host)
#[no_mangle]
pub extern "C" fn alloc(_size: u32) -> *mut u8 {
    // For minimal plugin, we don't support dynamic allocation
    // Return null pointer to indicate failure
    core::ptr::null_mut()
}

/// Deallocate memory in WASM linear memory (called by host)
#[no_mangle]
pub extern "C" fn dealloc(_ptr: *mut u8, _size: u32) {
    // No-op for minimal plugin
}

/// Panic handler (required for no_std)
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
EOF

    # Replace placeholders
    sed -i '' "s/{{ AUTHOR }}/$AUTHOR/g" "$PLUGIN_DIR/src/lib.rs"
    sed -i '' "s/{{ LICENSE }}/$LICENSE/g" "$PLUGIN_DIR/src/lib.rs"
    sed -i '' "s/{{ DESCRIPTION }}/$DESCRIPTION/g" "$PLUGIN_DIR/src/lib.rs"
    sed -i '' "s/{{ PLUGIN_NAME }}/$PLUGIN_NAME_SNAKE/g" "$PLUGIN_DIR/src/lib.rs"
}

create_lib_rs_json() {
    cat > "$PLUGIN_DIR/src/lib.rs" <<'EOF'
// Copyright 2025 {{ AUTHOR }}
// SPDX-License-Identifier: {{ LICENSE }}

//! {{ DESCRIPTION }}

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// Plugin metadata
static METADATA: &[u8] = br#"{
  "name": "{{ PLUGIN_NAME }}",
  "version": "0.1.0",
  "description": "{{ DESCRIPTION }}",
  "author": "{{ AUTHOR }}",
  "license": "{{ LICENSE }}",
  "type": "action",
  "capabilities": []
}"#;

/// Action request from host
#[derive(Debug, Deserialize)]
pub struct ActionRequest {
    pub action: String,
    pub context: TriggerContext,
}

/// Trigger context from host
#[derive(Debug, Deserialize)]
pub struct TriggerContext {
    pub velocity: Option<u8>,
    pub current_mode: Option<usize>,
    pub timestamp: u64,
}

// ============================================================================
// WASM Exports (Plugin Interface)
// ============================================================================

#[no_mangle]
pub extern "C" fn init() -> u64 {
    let ptr = METADATA.as_ptr() as u32;
    let len = METADATA.len() as u32;
    ((ptr as u64) << 32) | (len as u64)
}

#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32 {
    // Read request JSON from memory
    let request_json = match read_string(ptr as usize, len as usize) {
        Ok(s) => s,
        Err(_) => return 1, // Error: invalid memory read
    };

    // Parse request
    let request: ActionRequest = match serde_json::from_str(&request_json) {
        Ok(r) => r,
        Err(_) => return 2, // Error: invalid JSON
    };

    // Execute the requested action
    match execute_action(&request.action) {
        Ok(_) => 0,  // Success
        Err(_) => 3, // Error: action execution failed
    }
}

#[no_mangle]
pub extern "C" fn alloc(size: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: u32) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size as usize);
    }
}

// ============================================================================
// Plugin Logic
// ============================================================================

/// Execute an action
fn execute_action(action: &str) -> Result<(), &'static str> {
    match action {
        "example_action" => {
            // TODO: Implement your action here
            Ok(())
        }
        _ => Err("Unknown action"),
    }
}

// ============================================================================
// Memory Helpers
// ============================================================================

/// Read a string from WASM linear memory
fn read_string(ptr: usize, len: usize) -> Result<String, alloc::string::FromUtf8Error> {
    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len) };
    String::from_utf8(slice.to_vec())
}

/// Panic handler (required for no_std)
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Global allocator (required for alloc)
#[global_allocator]
static ALLOCATOR: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ============================================================================
// Compiler Intrinsics (required by serde_json in no_std)
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.add(i) = c as u8;
        i += 1;
    }
    s
}
EOF

    # Replace placeholders
    sed -i '' "s/{{ AUTHOR }}/$AUTHOR/g" "$PLUGIN_DIR/src/lib.rs"
    sed -i '' "s/{{ LICENSE }}/$LICENSE/g" "$PLUGIN_DIR/src/lib.rs"
    sed -i '' "s/{{ DESCRIPTION }}/$DESCRIPTION/g" "$PLUGIN_DIR/src/lib.rs"
    sed -i '' "s/{{ PLUGIN_NAME }}/$PLUGIN_NAME_SNAKE/g" "$PLUGIN_DIR/src/lib.rs"
}

# Create README
create_readme() {
    cat > "$PLUGIN_DIR/README.md" <<EOF
# $PLUGIN_NAME

$DESCRIPTION

## Building

\`\`\`bash
cargo build --target wasm32-wasip1 --release
\`\`\`

The compiled plugin will be at:
\`target/wasm32-wasip1/release/$PLUGIN_CRATE_NAME.wasm\`

## Testing

\`\`\`bash
# Unit tests (native)
cargo test

# Integration tests (WASM)
cd ../..
cargo test --package midimon-core --test ${PLUGIN_NAME_SNAKE}_test --features plugin-wasm
\`\`\`

## Plugin Info

- **Name:** $PLUGIN_NAME_SNAKE
- **Version:** 0.1.0
- **Author:** $AUTHOR
- **License:** $LICENSE

## Actions

- \`example_action\` - TODO: Describe what this action does

## Development

See the [WASM Plugin Development Guide](../../docs/WASM_PLUGIN_DEVELOPMENT_GUIDE.md) for detailed information on developing plugins.
EOF
}

# Create build script
create_build_script() {
    cat > "$PLUGIN_DIR/build.sh" <<'EOF'
#!/bin/bash
set -e

echo "Building WASM plugin..."
cargo build --target wasm32-wasip1 --release

WASM_FILE="target/wasm32-wasip1/release/$(echo "{{ PLUGIN_CRATE_NAME }}" | tr '-' '_').wasm"
SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')

echo "✅ Plugin built successfully!"
echo "   Size: $SIZE"
echo "   Path: $WASM_FILE"
EOF

    sed -i '' "s/{{ PLUGIN_CRATE_NAME }}/$PLUGIN_CRATE_NAME/g" "$PLUGIN_DIR/build.sh"
    chmod +x "$PLUGIN_DIR/build.sh"
}

# Generate files based on template
case $TEMPLATE in
    minimal)
        create_cargo_toml false
        create_lib_rs_minimal
        ;;
    json)
        create_cargo_toml true
        create_lib_rs_json
        ;;
    template)
        # Copy from wasm-template if it exists
        if [ -d "$PLUGINS_DIR/wasm-template" ]; then
            cp -r "$PLUGINS_DIR/wasm-template"/* "$PLUGIN_DIR/"
            # Update Cargo.toml
            sed -i '' "s/name = \".*\"/name = \"$PLUGIN_CRATE_NAME\"/" "$PLUGIN_DIR/Cargo.toml"
            sed -i '' "s/authors = \\[.*\\]/authors = [\"$AUTHOR\"]/" "$PLUGIN_DIR/Cargo.toml"
            sed -i '' "s/description = \".*\"/description = \"$DESCRIPTION\"/" "$PLUGIN_DIR/Cargo.toml"
            sed -i '' "s/license = \".*\"/license = \"$LICENSE\"/" "$PLUGIN_DIR/Cargo.toml"
        else
            # Fallback to JSON template
            print_warning "Template plugin not found, using JSON template"
            create_cargo_toml true
            create_lib_rs_json
        fi
        ;;
esac

create_readme
create_build_script

print_success "Plugin created successfully!"
echo
print_info "Next steps:"
echo "  1. cd $PLUGIN_DIR"
echo "  2. Edit src/lib.rs to implement your plugin logic"
echo "  3. ./build.sh to compile the plugin"
echo
print_info "Plugin location: $PLUGIN_DIR"
print_info "Binary will be: $PLUGIN_DIR/target/wasm32-wasip1/release/$PLUGIN_CRATE_NAME.wasm"
echo
print_info "See docs/WASM_PLUGIN_DEVELOPMENT_GUIDE.md for development help"
