use std::fmt;

pub struct Ram {
    mem: [u8; 4096],
}

// Related Functions
impl Ram {
    pub fn new() -> Ram {
        Ram { mem: [0u8; 4096] }
    }
}

// Methods
impl Ram {
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }
}

// Testing
impl fmt::Debug for Ram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mem: [")?;
        for i in 0..4096 {
            write!(f, "{:#X}", self.read_byte(i as u16))?;
            if i < 4095 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}