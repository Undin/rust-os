#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    writeln!(vga_buffer::VGA_WRITER.lock(), "Hello, World!").unwrap();
    write!(vga_buffer::VGA_WRITER.lock(), "Some numbers: {} {}", 1, 2.345).unwrap();
    loop {}
}
