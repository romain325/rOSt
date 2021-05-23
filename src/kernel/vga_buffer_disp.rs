const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
pub const VGA_BUFFER_ADDRESS: *mut VGABuffer = 0xb8000 as *mut VGABuffer;

use core::fmt;
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // contains u8
pub enum VGAColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // use the exact same datalayout as VGAColor 
pub struct VGAColorCode(u8);
impl VGAColorCode {
    pub fn new(fg: VGAColor, bg: VGAColor) -> VGAColorCode {
        VGAColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // ensure the struct form is C acceptable
struct VGABufferChar {
    ascii_char: u8,
    color: VGAColorCode,
}

#[repr(transparent)]
pub struct VGABuffer {
    chars : [[Volatile<VGABufferChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VGAWriter {
    pub col_pos: usize,
    pub color_code: VGAColorCode,
    pub buff: &'static mut VGABuffer,
}
impl VGAWriter {
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let chr = self.buff.chars[row][col].read();
                self.buff.chars[row -1][col].write(chr);
            }
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.col_pos = 0;
    }

    fn clear_row(&mut self, row : usize) {
        let blank = VGABufferChar {
            ascii_char: b' ',
            color: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buff.chars[row][col].write(blank);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                self.buff.chars[BUFFER_HEIGHT-1][self.col_pos].write(VGABufferChar {
                    ascii_char: byte,
                    color: self.color_code,
                });
                self.col_pos += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str){
        for b in s.bytes() {
            match b {
                // check if is known char or new line
                0x20..=0x7e | b'\n' => self.write_byte(b),
                // if unknown write special char
                _ => self.write_byte(0xfe) 
            }
        }
    }
    
    pub fn change_color(&mut self, color: VGAColorCode) {
        self.color_code = color;
    }
}
impl fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}
use spin::Mutex;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref VGA_WRITER : Mutex<VGAWriter> = Mutex::new(VGAWriter {
        col_pos: 0,
        color_code: VGAColorCode::new(VGAColor::Magenta, VGAColor::Black),
        buff: unsafe { &mut *(VGA_BUFFER_ADDRESS) },
    });
}

/******************MACROS******************/

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA_WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::kernel::vga_buffer_disp::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! set_print_color {
    ($($arg:tt)*) => ($crate::kernel::vga_buffer_disp::VGA_WRITER.lock().change_color($($arg)*));
}
