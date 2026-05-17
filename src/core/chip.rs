use crate::core::registers::{self, Register};

const MEMORY_SIZE: usize = 4096;

// ROMs are loaded in at this memory location.
const START_ROM_ADDRESS: u16 = 0x200;

// ROMs should not go past this memory range since the last 352 bytes
// are reserved for "variables and display refresh".
const END_ROM_ADDRESS: u16 = 0xE8F;

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

pub struct Chip8 {
    opcode: u16,
    memory: [u8; MEMORY_SIZE],
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
            opcode: 0x0000,
            memory: [0x00; MEMORY_SIZE],
            registers: [0x00; registers::TOTAL],
            pc: START_ROM_ADDRESS,
            addr_i: 0x00,
            stack: [0x00; STACK_SIZE],
            stack_size: 0,
            delay_timer: 0x00,
            sound_timer: 0x00,
        };
        chip.load_font();
        chip
    }

    fn load_font(&mut self) {
        self.memory[FONT_START_ADDR..(FONT_START_ADDR + FONT_SET.len())].copy_from_slice(&FONT_SET);
    }

    fn store_register(&mut self, reg: Register, value: u8) {
        self.registers[reg as usize] = value;
    }

    fn get_register(&mut self, reg: Register) -> u8 {
        self.registers[reg as usize]
    }

    pub fn reset(&mut self) {
        self.opcode = 0x00;
        self.memory = [0x00; MEMORY_SIZE];
        self.registers = [0x00; registers::TOTAL];
        self.pc = START_ROM_ADDRESS;
        self.addr_i = 0x00;
        self.stack = [0x00; STACK_SIZE];
        self.stack_size = 0;
        self.delay_timer = 0x00;
        self.sound_timer = 0x00;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::chip::{
        Chip8, FONT_SET, FONT_START_ADDR, MEMORY_SIZE, STACK_SIZE, START_ROM_ADDRESS,
    };
    use crate::core::registers::Register;

    #[test]
    fn basic_memory() {
        let mut chip = Chip8::init();
        chip.store_register(Register::V0, 0x10);
        assert_eq!(chip.get_register(Register::V0), 0x10);
        chip.reset();
        assert_eq!(chip.get_register(Register::V0), 0x00);
    }

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
        assert_eq!(chip.pc, START_ROM_ADDRESS);
        chip.pc += 2;
        chip.reset();
        assert_eq!(chip.pc, START_ROM_ADDRESS);
    }

    #[test]
    fn font_set_loaded() {
        let chip = Chip8::init();
        for (i, &byte) in FONT_SET.iter().enumerate() {
            assert_eq!(byte, chip.memory[FONT_START_ADDR + i])
        }
    }
}
