#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(sophie_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::arch::asm;
use core::panic::PanicInfo;
use sophie_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sophie_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    sophie_os::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();
    
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
