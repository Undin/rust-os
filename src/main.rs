#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]

#![reexport_test_harness_main = "test_entry_point"]

use core::panic::PanicInfo;

use rust_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World!");

    rust_os::init();

    #[cfg(test)]
    test_entry_point();

    println!("I'm still alive!");

    rust_os::hlt_loop();
}
