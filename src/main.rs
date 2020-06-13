use std::fs::File;
use std::io;
use std::io::Read;

mod chip8;
mod ram;

fn main() -> io::Result<()> {
    let mut file = File::open("games/INVADERS.ch8")?;
    let mut data = Vec::<u8>::new();
    let _ = file.read_to_end(&mut data);

    let chip = chip8::Chip8::new();

    println!("{:?}", data);
    Ok(())
}
