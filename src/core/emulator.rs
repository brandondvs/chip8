use std::fs;
use std::time::Duration;

use crate::core::chip::{self, Chip8, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::core::config::Config;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::{pixels::Color, render::WindowCanvas};

#[derive(PartialEq)]
pub enum State {
    Ready,
    Running,
    Exiting,
}

pub struct Emulator {
    state: State,
    chip: Chip8,
    sdl_context: Sdl,
    canvas: WindowCanvas,
}

impl Emulator {
    pub fn init(config: &Config) -> Result<Self, String> {
        let rom_file = config.rom_file();
        let data = match fs::read(rom_file) {
            Ok(data) => data,
            Err(err) => {
                let err_msg = if rom_file.is_empty() {
                    "no rom file given"
                } else {
                    rom_file
                };

                eprintln!("Unable to read rom, {err}: path: {err_msg}");
                Vec::new()
            }
        };

        let mut chip = chip::Chip8::init();
        chip.load_rom(data)
            .expect("unable to load ROM into chip memory");

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let scale = 10;
        let window = video_subsystem
            .window(
                "CHIP-8 Emulator",
                (DISPLAY_WIDTH as u32) * scale,
                (DISPLAY_WIDTH as u32) * scale,
            )
            .position_centered()
            .build()
            .unwrap();

        sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "nearest");

        let canvas = window.into_canvas().present_vsync().build().unwrap();

        Ok(Emulator {
            state: State::Ready,
            chip,
            sdl_context,
            canvas,
        })
    }

    fn update_display(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                sdl2::pixels::PixelFormatEnum::RGB24,
                DISPLAY_WIDTH as u32,
                DISPLAY_HEIGHT as u32,
            )
            .unwrap();

        let display_memory = self.chip.get_display_memory();
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..DISPLAY_HEIGHT {
                    for x in 0..DISPLAY_WIDTH {
                        let pixel_index = y * DISPLAY_WIDTH + x;
                        let is_on = display_memory[pixel_index] == 1;

                        let color = if is_on { 255 } else { 0 };
                        let offset = y * pitch + x * 3;
                        buffer[offset] = color;
                        buffer[offset + 1] = color;
                        buffer[offset + 2] = color;
                    }
                }
            })
            .unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }

    pub fn run(&mut self) {
        self.state = State::Running;

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        while self.state != State::Exiting {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => self.state = State::Exiting,
                    _ => {}
                }
            }

            for _ in 0..12 {
                self.chip.execute_cycle();
            }

            self.update_display();

            std::thread::sleep(Duration::from_millis(16));
        }
    }
}
