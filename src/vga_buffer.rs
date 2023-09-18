#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// u8 could technically be replaced by u4, but rust doesn't have a u4
#[repr(u8)]
pub enum Color {
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
#[repr(transparent)]
struct ColorCode(u8);

/// the `<<` operator "Left-Shifts" the u8 in binary by 4 spaces
/// Eg:
///     starting point
///     u8 -> 3
///     binary -> 11
///
///     left shift by 4:
///     binary -> 110000
///     u8 -> 48
///
/// the | operator performs a bitwise OR operation
/// Eg:
///     1100 | 0011 -> 1111
///     0010 | 1010 -> 1010
///
/// So, given a foreground color of u4
///     fg_decimal = 6
///     fg_binary = 0110
///
/// and a background color of u4
///     bg_decimal => 10
///     bg_binary => 1010
///     bg_bin_ls => 1010 >> 4 -> 10100000
///
/// we simply left-shift the bg color and combine the bg and fg with a bitwise operation:
///
///     bg_bin_ls | fg_binary => 10100000 | 00000110 -> 10100110
///
/// Which is a u8

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) { /* todo */
    }
}

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
