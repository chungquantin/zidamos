// OS can't use any pre written standard libraries
#![no_std]
// Tell Rust that we don't use the normal entry points (lang="start") which uses C runtime zero
// Remove main() as we no longer use the normal runtime system that calls main() function
#![no_main]
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Disable name mangling. Make sure Rust compiler truly return _start as entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
