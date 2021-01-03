#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

#![reexport_test_harness_main = "test_entry_point"]

use core::panic::PanicInfo;

use rust_os::{exit_qemu, QemuExitCode, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_entry_point();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[test_case]
fn should_panic() {
    serial_print!("should_panic::should_panic...\t");
    assert_eq!(1, 2);
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Fail);
    }
    exit_qemu(QemuExitCode::Success);
}
