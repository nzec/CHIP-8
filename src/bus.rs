use crate::display::Display;
use crate::input::Input;
use crate::ram::Ram;

use std::fmt;
use std::time;

pub struct Bus {
    pub ram: Ram,
    pub input: Input,
    pub display: Display,
    delay_timer: u8,
    delay_timer_set_time: time::Instant
}

// Related Functions
impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Ram::new(),
            input: Input::new(),
            display: Display::new(),
            delay_timer: 0,
            delay_timer_set_time: time::Instant::now()
        }
    }
}

// Expose Children's Methods to outside
impl Bus {
    // Set Delay Timer
    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer_set_time = time::Instant::now();
        self.delay_timer = value;
    }

    // Get Delay Timer
    pub fn get_delay_timer(&self) -> u8 {
        let diff = time::Instant::now() - self.delay_timer_set_time;
        let ms = diff.as_millis();

        let ticks = ms / 16;
        if ticks >= self.delay_timer as u128 {
            0
        } else {
            self.delay_timer - ticks as u8
        }
    }
}

// Testing
impl Bus {
    pub fn test_ram(&self) {
        println!("{:?}", self.ram);
    }
}
impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "delay_timer: {}", self.delay_timer)
    }
}