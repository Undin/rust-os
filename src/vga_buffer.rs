use core::fmt;
use core::fmt::Write;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[allow(dead_code)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    Pink = 0xD,
    Yellow = 0xE,
    White = 0xF,
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaWriter {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl VgaWriter {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position == BUFFER_WIDTH - 1 {
                    self.new_line()
                }
                let row = BUFFER_HEIGHT - 1;
                let column = self.column_position;
                self.buffer.chars[row][column].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, str: &str) {
        for byte in str.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                let c = self.buffer.chars[row][column].read();
                self.buffer.chars[row - 1][column].write(c);
            }
        }
        for column in 0..BUFFER_WIDTH {
            let blank_char = ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code
            };
            self.buffer.chars[BUFFER_HEIGHT - 1][column].write(blank_char);
        }
        self.column_position = 0;
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref VGA_WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { $crate::vga_buffer::_print(format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print!("{}\n", format_args!($($arg)*));
    })
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        VGA_WRITER.lock().write_fmt(args).unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_simple_print() {
        println!("simple output");
    }

    #[test_case]
    fn test_formatting() {
        println!("{} {}", "simple", "output")
    }

    #[test_case]
    fn test_print_many_lines() {
        for i in 0..500 {
            println!("line {}", i);
        }
    }

    #[test_case]
    fn test_output() {
        let s = "single line text";
        println!("{}", s);
        for (i, expected_char) in s.chars().enumerate() {
            let screen_char: ScreenChar = VGA_WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), expected_char)
        }
    }
}