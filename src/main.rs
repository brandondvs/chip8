use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

extern crate sdl2;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCREEN_SCALE_FACTOR: u32 = 10;

#[derive(PartialEq)]
enum EmulatorState {
    Ready,
    Running,
    Exiting,
}

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

    let mut emulator_state = EmulatorState::Ready;
    while emulator_state != EmulatorState::Exiting {
        emulator_state = EmulatorState::Running;

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
                    emulator_state = EmulatorState::Exiting
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Emulator exited...")
}

fn main() {
    println!("Starting emulator!");

    initialize_sdl();
}
