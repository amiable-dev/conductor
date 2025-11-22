// Minimal WASM plugin for testing - no std, no allocator, just raw exports

#![no_std]

// Pre-computed metadata JSON embedded in the binary
static METADATA: &[u8] = br#"{"name":"minimal_wasm_plugin","version":"0.1.0","description":"Minimal test plugin","author":"Amiable","license":"MIT","type":"action","capabilities":[]}"#;

/// Initialize the plugin - returns pointer and length packed in u64
#[no_mangle]
pub extern "C" fn init() -> u64 {
    // Return actual pointer to static metadata
    let ptr = METADATA.as_ptr() as u32;
    let len = METADATA.len() as u32;
    ((ptr as u64) << 32) | (len as u64)
}

/// Execute an action - stub implementation that always succeeds
#[no_mangle]
pub extern "C" fn execute(_ptr: u32, _len: u32) -> i32 {
    0 // Success
}

/// Allocate memory - required by host
#[no_mangle]
pub extern "C" fn alloc(_size: u32) -> *mut u8 {
    // For minimal plugin, we don't support dynamic allocation
    // Return null pointer to indicate failure
    core::ptr::null_mut()
}

/// Deallocate memory - stub implementation
#[no_mangle]
pub extern "C" fn dealloc(_ptr: *mut u8, _size: u32) {
    // No-op for minimal plugin
}

/// Panic handler - required for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
