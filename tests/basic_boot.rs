#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mooos::test_runner)]

/// This renames the `main` function to "test_main"
/// because the `custom_test_frameworks` feature
/// generates a function called `main` by default.
/// Since we are in no_std mode, we can't have 
/// a function called `main` because we 
/// use the `#![no_main]` attribute.
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mooos::test_panic_handler(info)
}
