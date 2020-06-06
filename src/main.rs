#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

const INITIAL_TEXT: &[u8] = b"Hello, World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (index, value) in INITIAL_TEXT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(index as isize * 2) = *value;
            *vga_buffer.offset(index as isize * 2 + 1) = 0xA;
        }
    }

    loop {}
}
