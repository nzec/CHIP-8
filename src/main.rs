extern crate minifb;

use minifb::{Key, Window, WindowOptions};
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
    let mut file = File::open("games/INVADERS.ch8")?;
    let mut data = Vec::<u8>::new();
    let _ = file.read_to_end(&mut data)?;

    // Make new Chip8 Emulator
    let mut chip8 = Chip8::new();
    // Load the ROM
    chip8.load_rom(&data);

    // Testing
    // chip8.test_cpu();
    // chip8.test_bus_ram();

    const WIDTH: usize = 640;
    const HEIGHT: usize = 320;

    // AA_RR_GG_BB
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    for i in buffer.iter_mut() {
        *i = 0xff0000ff; // write something more funny here!
    }

    let mut window = Window::new(
        "Chip-8 Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        chip8.run_instruction();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    Ok(())
}
