use crate::core::registers::{self, Register};

const MEMORY_SIZE: usize = 4096;

pub struct Chip8 {
    opcode: u16,
    memory: [u8; MEMORY_SIZE],
    registers: [u8; registers::TOTAL],
}

impl Chip8 {
    pub fn init() -> Self {
        Chip8 {
            opcode: 0x0000,
            memory: [0x00; MEMORY_SIZE],
            registers: [0x00; registers::TOTAL],
        }
    }

    pub fn store_register(&mut self, reg: Register, value: u8) {
        self.registers[reg as usize] = value;
    }
    pub fn get_register(&mut self, reg: Register) -> u8 {
        self.registers[reg as usize]
    }

    pub fn reset(&mut self) {
        self.opcode = 0x00;
        self.memory = [0x00; MEMORY_SIZE];
        self.registers = [0x00; registers::TOTAL];
    }
}

#[cfg(test)]
mod tests {
    use crate::core::chip::{Chip8, MEMORY_SIZE};
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

        assert_eq!(chip.memory.len(), MEMORY_SIZE)
    }
}
