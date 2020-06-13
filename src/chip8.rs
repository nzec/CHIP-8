use crate::ram::Ram;

pub struct Chip8 {
    ram: Ram,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { ram: Ram::new() }
    }

    pub fn load_game(&mut self, data: &Vec<u8>) {
        let program_start = 0x200;
        for i in 0..data.len() {
            self.ram.write(program_start + (i as u16), data[i]);
        }
    }
}
