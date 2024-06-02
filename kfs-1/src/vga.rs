
// #[allow(dead_code)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    White = 15
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground : Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenCharacter {
    ascii_value : u8,
    color : ColorCode
}

const BUFFER_HEIGHT : usize = 25;
const BUFFER_WIDTH : usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenCharacter; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
    column_position : usize,
    _row_position : usize,
    color_code : ColorCode,
    buffer : &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line()
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenCharacter {
                    ascii_value: byte,
                    color : color_code
                };
                self.column_position +=1;
            }
        }
    }
    pub fn write_string(&mut self, str : &str) {
        for char in str.bytes() {
            match char {
                0x20..=0x7e | 0x0A => self.write_byte(char),
                _ => self.write_byte(0xfe)
            }
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    fn clear_row(&mut self, index : usize){
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[index][col] = ScreenCharacter {
                ascii_value : 0x20,
                color : ColorCode::new(Color::Black, Color::Black)
            };
        }
    }
}

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        _row_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Red),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
    };

    writer.write_string("HELLO WORLD !");
    writer.new_line();
    writer.write_byte(b'4');
    writer.write_byte(b'2');

}