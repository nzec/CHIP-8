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
    pub fn write(&mut self, address: u16, value: u8) {}

    pub fn read(&self, address: u16) {}
}
