#![no_std]  // disable standard lib
#![no_main] // disable default entry point
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod kernel;
mod test;
use crate::kernel::vga_buffer_disp::VGA_WRITER;
use crate::kernel::vga_buffer_disp::*;

#[cfg(not(test))]
#[no_mangle] // keep the _start name as it is
// Override start function as we don't have anything before to init it
pub extern "C" fn _start() -> ! {
    print_shit();

    #[cfg(test)]
    test_main();

    panic!("omfg, she's fucking dead");
    
    // loop in the void
    // loop{}
}

fn print_shit() {
    use core::fmt::Write;
    VGA_WRITER.lock().write_str("Hello Wörld!\n");
    write!(VGA_WRITER.lock(), "FavNb: {}\n", 4).unwrap();
    VGA_WRITER.lock().change_color(VGAColorCode::new(VGAColor::White, VGAColor::Black));
    print!("Helloooo World, ");
    set_print_color!(VGAColorCode::new(VGAColor::Red, VGAColor::Black));
    println!("S{}t{}n",8,4);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}