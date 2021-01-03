#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rust_os::{exit_qemu, QemuExitCode, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_panic();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Fail);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_panic() {
    serial_print!("should_panic::should_panic...\t");
    assert_eq!(1, 2);
}
