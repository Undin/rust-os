#![feature(abi_x86_interrupt)]

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rust_os::{serial_print, serial_println, exit_qemu, QemuExitCode};
use rust_os::gdt;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");
    rust_os::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Unexpected execution after stack overflow")
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault.set_handler_fn(test_double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn test_double_fault_handler(_frame: InterruptStackFrame, _exit_code: u64) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn init_test_idt() {
    TEST_IDT.load();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read(); // prevents tail recursion optimization
}
