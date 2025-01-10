

// No standard library to compile on baremetal
#![no_std]
// We dont want to use the normal entry point chain.
#![no_main]

use core::panic::PanicInfo;
// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
// Disable mangling of function name so the compiler knows the actual function we intend to start with.
// _start is the default entry point name for most systems.
// Needs to never return because the function is called by the bootloader to start the OS.
static HELLO: &[u8] = b"Hello World!";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

