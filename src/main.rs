use c8::{HEIGHT, RAM_SIZE, WIDTH};
use minifb::{Key, Scale, Window, WindowOptions};
use std::{env, fs};
use rodio::Sink;

mod c8;

const DISPLAY_REFRESH_RATE: f64 = 480.0;
const RUNLOOP_TIMER_DEFAULT: usize = (DISPLAY_REFRESH_RATE / 60.0) as usize;

fn main() {
    println!("CHIP-8 Interpreter/Emulator");
    let mut c8 = c8::C8::new();

    let args: Vec<String> = env::args().collect(); 

    // Read ROM
    let file_name = &args[1];
    let rom = match fs::read(file_name) {
        Ok(file) => file,
        Err(e) => panic!("Cound't load file: {}", e.to_string()),
    };
    c8.load_ram(&rom);
    println!("{:?}", rom);

    // Setup Audio
    let audio_device  = rodio::default_output_device().unwrap();
    let audio_sink = Sink::new(&audio_device);
    let audio_source = rodio::source::SineWave::new(440);
    audio_sink.append(audio_source);
    audio_sink.pause();

    // Setup Window
    let mut window = Window::new(
        &format!("CHIP-8: {}", file_name),
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X8,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open a Window");

    // Using limit_update_rate minifb will check how much time has passed since
    // the last time and if it's less than the selected time it will sleep for
    // the remainder of it. This means that if more time has spent than the set
    // time (external code taking longer) minifb will not do any waiting at all
    // so there is no loss in CPU performance with this feature.
    // Source: https://docs.rs/minifb/0.16.0/minifb/struct.Window.html#method.limit_update_rate
    window.limit_update_rate(Some(std::time::Duration::from_secs_f64(1.0 / DISPLAY_REFRESH_RATE)));

    let mut executing = true;
    let mut wait_for_key: usize = 0;
    let mut update_counter: usize = RUNLOOP_TIMER_DEFAULT;

    while window.is_open() && !window.is_key_down(Key::Escape) && c8.pc <= RAM_SIZE as u16 {
        let mut key_press: [bool; 16] = [false; 16];

        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::Key1 => key_press[0x1] = true,
                    Key::Key2 => key_press[0x2] = true,
                    Key::Key3 => key_press[0x3] = true,
                    Key::Key4 => key_press[0xc] = true,
                    Key::Q => key_press[0x4] = true,
                    Key::W => key_press[0x5] = true,
                    Key::E => key_press[0x6] = true,
                    Key::R => key_press[0xd] = true,
                    Key::A => key_press[0x7] = true,
                    Key::S => key_press[0x8] = true,
                    Key::D => key_press[0x9] = true,
                    Key::F => key_press[0xe] = true,
                    Key::Z => key_press[0xa] = true,
                    Key::X => key_press[0x0] = true,
                    Key::C => key_press[0xb] = true,
                    Key::V => key_press[0xf] = true,
                    _ => (),
                }
            }
        });


        for j in 0..16 {
            if key_press[j] {
                if wait_for_key != 0 {
                    executing = false;
                    c8.v[wait_for_key] = j as u8;
                    wait_for_key = 0;
                    break;
                }
            }
        }

        if executing {
            wait_for_key = c8.run(&key_press);
        }

        // 60 Hz
        if update_counter == 0 {
            // The delay timer is active whenever the delay timer register (DT)
            // is non-zero. This timer does nothing more than subtract 1 from
            // the value of DT at a rate of 60Hz. When DT reaches 0, it
            // deactivates.
            if c8.dt > 0 {
                c8.dt -= 1;
            }

            // The sound timer is active whenever the sound timer register (ST)
            // is non-zero. This timer also decrements at a rate of 60Hz,
            // however, as long as ST's value is greater than zero, the Chip-8
            // buzzer will sound. When ST reaches zero, the sound timer
            // deactivates.
            // The sound produced by the Chip-8 interpreter has only one tone.
            // The frequency of this tone is decided by the author of the
            // interpreter.
            if c8.st > 0 {
                audio_sink.play();
                c8.st -= 1;
            } else if c8.st == 0 {
                audio_sink.pause();
            }

            // Update Window
            window
                .update_with_buffer(&c8.display, WIDTH, HEIGHT)
                .unwrap();

                update_counter = RUNLOOP_TIMER_DEFAULT;
        } else {
            update_counter -= 1;
        }
    }
}
