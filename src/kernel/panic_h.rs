use crate::*;
use core::panic::PanicInfo;

// define panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    set_print_color!(VGAColorCode::new(VGAColor::Red, VGAColor::Black));
    println!("{}", _info);
    set_print_color!(VGAColorCode::new(VGAColor::White, VGAColor::Black));

    loop{}
}