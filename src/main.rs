use chip8::core::chip::Chip8;
use chip8::core::emulator;

use std::fs;
use std::io;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

extern crate sdl2;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCREEN_SCALE_FACTOR: u32 = 10;

fn initialize_sdl() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "chip-8 emulator",
            WIDTH * SCREEN_SCALE_FACTOR,
            HEIGHT * SCREEN_SCALE_FACTOR,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut emulator_state = emulator::State::Ready;
    while emulator_state != emulator::State::Exiting {
        emulator_state = emulator::State::Running;

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("state_changed=exiting...");
                    emulator_state = emulator::State::Exiting
                }
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Emulator exited...")
}

fn read_rom_file(filename: &str) -> io::Result<Vec<u8>> {
    let data = fs::read(filename)?;
    Ok(data)
}

fn main() {
    println!("Starting emulator!");

    let rom_path = "test_roms/2-ibm-logo.ch8";
    let rom_data = match read_rom_file(rom_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Could not read '{}' ({})", rom_path, err);
            std::process::exit(1)
        }
    };

    let mut chip = Chip8::init();
    let _ = chip.load_rom(rom_data);

    loop {
        chip.execute_cycle();
        std::thread::sleep(Duration::from_millis(100))
    }

    //initialize_sdl();
}
