#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_entry_point"]

use core::any;
use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}", info);
    #[cfg(test)]
    exit_qemu(QemuExitCode::Fail);
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World!");

    #[cfg(test)]
    test_entry_point();

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
enum QemuExitCode {
    Success = 0x10,
    Fail = 0x11
}

fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0xf4);
    unsafe {
        port.write(exit_code as u32);
    }
}

#[test_case]
fn trivial_test_ok() {
    assert_eq!(1, 1);
}

#[test_case]
fn trivial_test_fail() {
    assert_eq!(1, 2);
}

trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) -> () {
        serial_print!("{}...\t", any::type_name::<T>());
        self();
        serial_println!("[ok]")
    }
}
