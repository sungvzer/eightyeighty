use std::fmt::Debug;

pub enum Instruction {
    NOP,
    JMP(u16),

    Unknown,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOP => write!(f, "NOP"),
            Self::JMP(arg0) => write!(f, "JMP $({:02x})", arg0),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
