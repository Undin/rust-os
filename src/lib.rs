#![no_std]
#![cfg_attr(test, no_main)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![feature(abi_x86_interrupt)]

#![reexport_test_harness_main = "test_entry_point"]

use core::panic::PanicInfo;

pub mod vga_buffer;
pub mod serial;
pub mod interrupts;
pub mod gdt;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Fail);
    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_entry_point();
    loop {}
}

#[doc(hidden)]
pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]")
    }
}

#[doc(hidden)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Fail = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0xf4);
    unsafe {
        port.write(exit_code as u32);
    }
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}