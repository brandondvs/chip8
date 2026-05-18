pub struct OpCode {
    // category determines the opcode instruction itself
    pub category: u8,

    // second nibble in the instruction
    pub x: u8,

    // third nibble in the instruction
    pub y: u8,

    // fourth nibble in the instruction
    pub n: u8,

    // second byte in the instruction
    pub nn: u8,

    // second, third, fourth nibbles in the instruction
    pub nnn: u16,
}

impl OpCode {
    pub fn new(instruction: u16) -> Self {
        OpCode {
            category: ((instruction >> 12) as u8) & 0x0F,
            x: ((instruction >> 8) as u8) & 0x0F,
            y: ((instruction >> 4) & 0x0F) as u8,
            n: (instruction & 0x0F) as u8,

            nn: (instruction & 0xFF) as u8,
            nnn: (instruction) & 0xFFF,
        }
    }
}
