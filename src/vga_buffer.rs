//representing different colors using an enum

#[allow(dead_code)] //disabling warning for unused variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // enabling copy semantics for types and making it printable and comparable
#[repr(u8)] //each enum variant is stored as a u8
pub enum Color{
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
    Yellow =14,
    White = 15,
}

//making a color code that represents foreground and background color

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]//ensuring that the struct has the same data layout as a u8
struct ColorCode(u8);

impl ColorCode{
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

//Represents Screen Character and Text Buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar{
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer{
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

//Write to screen 
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

//Printing 
impl Writer{
    //method to write a single byte
    pub fn write_byte(&mut self, byte:u8){
        match byte{
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

    fn new_line(&mut self) {/* TODO */}


    //printing strings by converting them into bytes and printing one by one
    pub fn write_string(&mut self, s: &str){
        //returns the bytes of each character 
         for byte in s.bytes(){
             match byte{
                 //printable ASCII byte or newline
                 0x20..=0x7e | b'\n' => self.write_byte(byte),
                 //not part of printable ASCII range
                 _=>self.write_byte(0xfe),//these are unprintable characters
             }
         }
     }
 
}

pub fn print_something(){
    let mut writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
    };
    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("World!")
}
