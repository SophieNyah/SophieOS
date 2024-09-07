#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(sophie_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sophie_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sophie_os::test_panic_handler(info);
}

#[test_case]
fn teste_println() {
    println!("Saída do teste_println")
}
