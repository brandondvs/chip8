use std::fs;
use std::time::Duration;

use crate::core::chip::{self, Chip8};

#[derive(PartialEq)]
pub enum State {
    Ready,
    Running,
    Exiting,
}

pub struct Emulator {
    chip: Chip8,
}

impl Emulator {
    pub fn init(path: &str) -> Result<Self, String> {
        let data = match fs::read(path) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Unable to read rom, {err}: path: {path}");
                Vec::new()
            }
        };

        let mut chip = chip::Chip8::init();
        chip.load_rom(data)
            .expect("unable to load ROM into chip memory");

        Ok(Emulator { chip })
    }

    pub fn run(&mut self) {
        loop {
            self.chip.execute_cycle();
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}
