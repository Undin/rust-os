#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_entry_point"]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
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
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_test_ok() {
    print!("trivial assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[test_case]
fn trivial_test_fail() {
    print!("trivial assertion...");
    assert_eq!(1, 2);
    println!("[ok]");
}
