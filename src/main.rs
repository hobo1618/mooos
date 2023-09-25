#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mooos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use mooos::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);



// #[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // use mooos::memory::active_level_4_table;
    use mooos::memory::translate_addr;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    mooos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        // identity mapped vga_buffer
        0x8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }


    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
    //
    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);
    //     }
    // }

    // uncomment lines below to trigger a stack overflow
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }
    // stack_overflow();
    
    // doesn't trigger a page fault because this
    // address has read permissions
    let ptr = 0x2031b2 as *mut u8;
    unsafe { let x = *ptr; }
    println!("read worked");

    // triggers page fault for because no write permissions
    // unsafe { *ptr = 42; }
    // println!("write worked");
    
    // triggers a page fault because this
    // address isn't read or writable
    // let ptr = 0xdeadbeef as *mut u8;
    

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
