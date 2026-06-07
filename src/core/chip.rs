use crate::core::opcode::OpCode;
use crate::core::registers::{self};

const MEMORY_SIZE: usize = 4096;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

// ROMs are loaded in at this memory location.
const ROM_START_ADDRESS: u16 = 0x200;

// ROMs should not go past this memory range since the last 352 bytes
// are reserved for "variables and display refresh".
const ROM_END_ADDRESS: u16 = 0xE8F;

const STACK_SIZE: usize = 12;

const FONT_START_ADDR: usize = 0x50;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

type DisplayMemory = [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT];

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    display: DisplayMemory,
    registers: [u8; registers::TOTAL],

    pc: u16,

    addr_i: u16,

    stack: [u16; STACK_SIZE],
    stack_size: u8,

    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn init() -> Self {
        let mut chip = Chip8 {
            memory: [0x00; MEMORY_SIZE],
            display: [0x00; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            registers: [0x00; registers::TOTAL],
            pc: ROM_START_ADDRESS,
            addr_i: 0x00,
            stack: [0x00; STACK_SIZE],
            stack_size: 0,
            delay_timer: 0x00,
            sound_timer: 0x00,
        };
        chip.load_font();
        chip
    }

    pub fn load_rom(&mut self, data: Vec<u8>) -> Result<usize, String> {
        if data.len() == 0 {
            return Err(format!("ROM contains zero bytes"));
        }

        let memory_limit = (ROM_END_ADDRESS - ROM_START_ADDRESS) as usize;
        if data.len() > memory_limit {
            return Err(format!("ROM exceeds the memory limit ({})", memory_limit));
        }

        for (i, &byte) in data.iter().enumerate() {
            let idx = ROM_START_ADDRESS as usize + i;
            self.memory[idx] = byte;
            println!("0x{:X}: 0x{:X}", idx, byte);
        }

        println!("==== END OF ROM ====");
        Ok(data.len())
    }

    fn fetch(&mut self) {
        let pc_idx = self.pc as usize;
        let instruction: u16 =
            ((self.memory[pc_idx] as u16) << 8) | (self.memory[pc_idx + 1] as u16);

        self.pc += 2;

        self.decode(instruction);
    }

    fn decode(&mut self, instruction: u16) {
        let opcode = OpCode::new(instruction);

        println!(
            "(memory location: 0x{:X}) OpCode: category = 0x{:X} {:X} {:X} {:X}",
            self.pc - 2,
            opcode.category,
            opcode.x,
            opcode.y,
            opcode.n
        );

        self.execute(opcode);
    }

    fn execute(&mut self, opcode: OpCode) {
        match opcode.category {
            0x0 => match opcode.nnn {
                0x0E0 => {
                    println!("clear the screen!")
                }
                _ => {
                    println!("Unknown opcode: 0x{:X}{:X}", opcode.category, opcode.nnn)
                }
            },
            // Jump to address
            0x1 => {
                let addr = opcode.nnn;
                if addr as usize > MEMORY_SIZE {
                    println!(
                        "Jumping to address outside memory size ({}): 0x{:X}",
                        MEMORY_SIZE, addr,
                    );
                    return;
                }
                println!("Setting program counter to address: 0x{:X}", addr);
                self.pc = addr
            }

            // Set the index register to the immediate address
            0xA => {
                let addr_i = opcode.nnn;
                if addr_i as usize > MEMORY_SIZE {
                    println!(
                        "Attempted to set index address to a memory location outside the bounds. 0x{:X}",
                        addr_i
                    );
                    return;
                }
                println!("Setting addr_i = 0x{:X}", addr_i);
                self.addr_i = addr_i;
            }

            // Load immediate value into register
            0x6 => {
                let reg = match registers::Register::try_from(opcode.x) {
                    Ok(reg) => reg as usize,
                    Err(err) => {
                        println!("Invalid register({:?}): 0x{:X}", err, opcode.x);
                        return;
                    }
                };

                let value = opcode.nn;

                println!("Setting register 0x{:X} = 0x{:X}", reg, value);
                self.registers[reg] = value;
            }

            // Add immediate value to register
            0x7 => {
                let reg = match registers::Register::try_from(opcode.x) {
                    Ok(reg) => reg as usize,
                    Err(err) => {
                        println!("Invalid register({:?}): 0x{:X}", err, opcode.x);
                        return;
                    }
                };
                let value = opcode.nn;

                println!(
                    "Register: 0x{:X} = 0x{:X} adding 0x{:X}",
                    reg, self.registers[reg], value,
                );
                self.registers[reg as usize] = self.registers[reg as usize].wrapping_add(value);
                println!("Register 0x{:X} updated = 0x{:X}", reg, self.registers[reg])
            }

            // Draw to the display memory
            0xD => {
                let x = self.registers[registers::Register::try_from(opcode.x).unwrap() as usize];
                let y = self.registers[registers::Register::try_from(opcode.y).unwrap() as usize];
                let rows = opcode.n;

                self.draw_sprite(x as usize, y as usize, rows as usize, self.addr_i as usize);
            }
            _ => {
                println!("unable to execute instruction: 0x{:X}", opcode.category)
            }
        }
    }

    fn draw_sprite(&mut self, x: usize, y: usize, rows: usize, sprite_addr: usize) {
        println!(
            "X: {:?} | Y: {:?} | Rows: {} | Sprite addr: {}",
            x, y, rows, sprite_addr
        );

        self.registers[registers::Register::VF as usize] = 0x00;

        for row in 0..rows {
            let sprite_byte = self.memory[sprite_addr + row as usize];
            let current_y = (y + row) % DISPLAY_HEIGHT;

            for col in 0..8 {
                let sprite_pixel = (sprite_byte >> (7 - col)) & 0x01;

                if sprite_pixel == 0x01 {
                    let current_x = (x + col) % DISPLAY_WIDTH;
                    let pixel_idx = current_y * DISPLAY_WIDTH + current_x;

                    if self.display[pixel_idx] == 0x01 {
                        self.registers[registers::Register::VF as usize] = 0x01;
                    }

                    self.display[pixel_idx] ^= 0x01;
                }
            }
        }

        self.debug_print_display();
    }

    fn debug_print_display(&self) {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let pixel_idx = y * DISPLAY_WIDTH + x;

                if self.display[pixel_idx] == 0x01 {
                    print!("█");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    pub fn get_display_memory(&self) -> DisplayMemory {
        return self.display;
    }

    pub fn execute_cycle(&mut self) {
        self.fetch()
    }

    fn load_font(&mut self) {
        self.memory[FONT_START_ADDR..(FONT_START_ADDR + FONT_SET.len())].copy_from_slice(&FONT_SET);
    }

    pub fn reset(&mut self) {
        self.memory = [0x00; MEMORY_SIZE];
        self.display = [0x00; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.registers = [0x00; registers::TOTAL];
        self.pc = ROM_START_ADDRESS;
        self.addr_i = 0x00;
        self.stack = [0x00; STACK_SIZE];
        self.stack_size = 0;
        self.delay_timer = 0x00;
        self.sound_timer = 0x00;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::chip::*;

    #[test]
    fn memory_size() {
        let chip = Chip8::init();

        assert_eq!(chip.memory.len(), MEMORY_SIZE);
    }

    #[test]
    fn stack_size() {
        let chip = Chip8::init();
        assert_eq!(chip.stack.len(), STACK_SIZE)
    }

    #[test]
    fn program_counter_start_reset() {
        let mut chip = Chip8::init();
        assert_eq!(chip.pc, ROM_START_ADDRESS);
        chip.pc += 2;
        chip.reset();
        assert_eq!(chip.pc, ROM_START_ADDRESS);
    }

    #[test]
    fn font_set_loaded() {
        let chip = Chip8::init();
        for (i, &byte) in FONT_SET.iter().enumerate() {
            assert_eq!(byte, chip.memory[FONT_START_ADDR + i])
        }
    }

    #[test]
    fn display_memory_size() {
        let chip = Chip8::init();
        assert_eq!(chip.display.len(), DISPLAY_WIDTH * DISPLAY_HEIGHT)
    }
}
