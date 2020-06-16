use crate::display::Display;
use crate::input::Input;
use crate::ram::Ram;

pub struct Bus {
    ram: Ram,
    input: Input,
    display: Display,
}

// Related Functions
impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Ram::new(),
            input: Input::new(),
            display: Display::new(),
        }
    }
}

// Methods
impl Bus {
    pub fn ram_read_byte(&self, address: u16) -> u8 {
        self.ram.read_byte(address)
    }

    pub fn ram_write_byte(&mut self, address: u16, value: u8) {
        self.ram.write_byte(address, value);
    }

    pub fn display_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        self.display.draw_byte(byte, x, y)
    }

    pub fn input_key_pressed(&self, key_code: u8) -> bool {
        self.input.key_pressed(key_code)
    }
}

// Testing
impl Bus {
    pub fn test_ram(&self) {
        println!("{:?}", self.ram);
    }
}