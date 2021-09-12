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
    loop {}
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

    // check breakpoint exception
    x86_64::instructions::interrupts::int3();
    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    #[cfg(test)]
    test_entry_point();

    println!("I'm still alive!");

    loop {}
}
