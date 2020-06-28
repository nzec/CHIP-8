use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::cpu;

pub struct Chip8 {
    bus: Bus,
    cpu: Cpu
}

// Related Functions
impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            bus: Bus::new(),
            cpu: Cpu::new()
        }
    }
}

// Methods
impl Chip8 {
    pub fn load_rom(&mut self, data: &Vec<u8>) {

        // Load sprites
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80]  // F
        ];
        
        for i in 0..80 {
            self.bus.ram.write_byte(i as u16, sprites[i / 5][i % 5]);
        }

        // Load game
        for i in 0..data.len() {
            self.bus.ram.write_byte(cpu::PROGRAM_START + (i as u16), data[i]);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_opcode(&mut self.bus);
        println!("{:?}", self.cpu);
        println!("{:?}\n", self.bus);
    }
}

// Testing
impl Chip8 {
    pub fn test_bus(&self) {
        println!("{:?}", self.bus);
    }
    pub fn test_bus_ram(&self) {
        self.bus.test_ram();
    }
    pub fn test_cpu(&self) {
        println!("{:?}", self.cpu);
    }
}