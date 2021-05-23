#![no_std]  // disable standard lib
#![no_main] // disable default entry point

mod kernel;
use crate::kernel::vga_buffer_disp::*;

use core::panic::PanicInfo;
// define panic handler
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    loop{}
}

#[no_mangle] // keep the _start name as it is
// Override start function as we don't have anything before to init it
pub extern "C" fn _start() -> ! {
    print_shit();

    // loop in the void
    loop{}
}

fn print_shit() {
    use core::fmt::Write;
    let mut writer = VGAWriter {
        col_pos: 0,
        color_code: VGAColorCode::new(VGAColor::Magenta, VGAColor::Black),
        buff: unsafe { &mut *(VGA_BUFFER_ADDRESS) },
    };

    writer.write_str("Hello Wörld!\n");
}