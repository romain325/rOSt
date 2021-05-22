#![no_std]  // disable standard lib
#![no_main] // disable default entry point

use core::panic::PanicInfo;
// define panic handler
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    loop{}
}

#[no_mangle] // keep the _start name as it is
// Override start function as we don't have anything before to init it
pub extern "C" fn _start() -> ! {
    // loop in the void
    loop{}
}