#![no_std]  // disable standard lib
#![no_main] // disable default entry point

mod memory;

use core::panic::PanicInfo;
// define panic handler
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    loop{}
}

#[no_mangle] // keep the _start name as it is
// Override start function as we don't have anything before to init it
pub extern "C" fn _start() -> ! {
    // vga buffer address
    let vga_buff = 0xb8000 as *mut u8;
    // The message we want to display as byte
    let hello = b"Hello World!";
    // iterate through all the bytes
    for (i, &byte) in hello.iter().enumerate() {
        unsafe{
            // display the wanted char
            *vga_buff.offset(i as isize *2) = byte;
            // as sepecified by vga text mode, we set the color to magenta
            *vga_buff.offset(i as isize *2+1) = 0x5;
        }
    }


    // loop in the void
    loop{}
}