use std::convert::TryFrom;

pub const TOTAL: usize = 16;

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum Register {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    V9 = 9,
    VA = 10,
    VB = 11,
    VC = 12,
    VD = 13,
    VE = 14,
    VF = 15,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidRegister(u8);

impl TryFrom<u8> for Register {
    type Error = InvalidRegister;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register::V0),
            1 => Ok(Register::V1),
            2 => Ok(Register::V2),
            3 => Ok(Register::V3),
            4 => Ok(Register::V4),
            5 => Ok(Register::V5),
            6 => Ok(Register::V6),
            7 => Ok(Register::V7),
            8 => Ok(Register::V8),
            9 => Ok(Register::V9),
            10 => Ok(Register::VA),
            11 => Ok(Register::VB),
            12 => Ok(Register::VC),
            13 => Ok(Register::VD),
            14 => Ok(Register::VE),
            15 => Ok(Register::VF),
            other => Err(InvalidRegister(other)),
        }
    }
}
