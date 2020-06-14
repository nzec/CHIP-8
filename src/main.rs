use std::fs::File;
use std::io;
use std::io::Read;
use chip8::Chip8;

mod chip8;
pub mod ram;
mod cpu;

fn main() -> io::Result<()> {
    let mut file = File::open("games/TETRIS.ch8")?;
    let mut data = Vec::<u8>::new();
    let _ = file.read_to_end(&mut data)?;

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    chip8.test_cpu();
    // chip8.test_ram();

    loop {
        chip8.run_instruction()
    }

    Ok(())
}
