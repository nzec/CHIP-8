use std::fs::File;
use std::io;
use std::io::Read;
use chip8::Chip8;

mod chip8;
mod ram;
mod cpu;
mod input;
mod display;
mod bus;

fn main() -> io::Result<()> {
    // Load the file
    let mut file = File::open("games/TETRIS.ch8")?;
    let mut data = Vec::<u8>::new();
    let _ = file.read_to_end(&mut data)?;

    // Make new Chip8 Emulator
    let mut chip8 = Chip8::new();
    // Load the ROM
    chip8.load_rom(&data);

    // Testing
    chip8.test_cpu();
    chip8.test_bus_ram();

    loop {
        chip8.run_instruction()
    }

    // Ok(())
}
