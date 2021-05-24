use core::panic::PanicInfo;
use crate::*;

pub trait Testable {
    fn run(&self) -> ();
}
impl<T> Testable for T where T: Fn() {
    fn run(&self) {
        println!("{}...\t", core::any::type_name::<T>());
        self();
        set_print_color!(VGAColorCode::new(VGAColor::Green, VGAColor::Black));
        println!("Done !");
        set_print_color!(VGAColorCode::new(VGAColor::White, VGAColor::Black));
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    set_print_color!(VGAColorCode::new(VGAColor::White, VGAColor::Black));
    for t in tests.iter() {
        t.run();
    }
    exit_qemu(0x10);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    set_print_color!(VGAColorCode::new(VGAColor::Red, VGAColor::Black));   
    println!("[failed]\n");
    println!("Error: {}\n", info);
    set_print_color!(VGAColorCode::new(VGAColor::White, VGAColor::Black));   
    exit_qemu(0x11);
    loop {}
}

pub fn exit_qemu(exit_code: u32) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
