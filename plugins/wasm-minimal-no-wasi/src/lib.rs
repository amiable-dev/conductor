// Absolute minimal WASM plugin - no WASI, no std, just pure WASM

#![no_std]

/// Initialize the plugin - returns a hardcoded u64
#[no_mangle]
pub extern "C" fn init() -> u64 {
    0x0000000100000098  // Hardcoded: ptr=1, len=152
}

/// Execute an action - always succeeds
#[no_mangle]
pub extern "C" fn execute(_ptr: u32, _len: u32) -> i32 {
    0 // Success
}

/// Panic handler - required for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
