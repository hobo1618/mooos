#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mooos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use mooos::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    mooos::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // uncomment line below to trigger a stack overflow
    // stack_overflow();
    
    // trigger page fault
    let ptr = 0x2031b2 as *mut u8;
    unsafe { let x = *ptr; }
    println!("read worked");
    // let ptr = 0xdeadbeef as *mut u8;
    // unsafe { *ptr = 42; }
    // println!("write worked");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    mooos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    mooos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mooos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
