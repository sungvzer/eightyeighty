use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum RegisterPair {
    /// B:C as 16 bit register
    BC,

    /// D:E as 16 bit register
    DE,

    /// H:L as 16 bit register
    HL,

    /// Stack pointer, refers to PSW (FLAGS:A) for PUSH/POP
    SP,
}

impl Display for RegisterPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Register pairs are printed with one char except for the SP variant
            RegisterPair::BC => write!(f, "B"),
            RegisterPair::DE => write!(f, "D"),
            RegisterPair::HL => write!(f, "H"),
            RegisterPair::SP => write!(f, "SP"),
        }
    }
}

impl TryFrom<u8> for RegisterPair {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RegisterPair::BC),
            1 => Ok(RegisterPair::DE),
            2 => Ok(RegisterPair::HL),
            3 => Ok(RegisterPair::SP),
            _ => Err("Invalid register pair reference"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::A => write!(f, "A"),
            Register::B => write!(f, "B"),
            Register::C => write!(f, "C"),
            Register::D => write!(f, "D"),
            Register::E => write!(f, "E"),
            Register::H => write!(f, "H"),
            Register::L => write!(f, "L"),
            Register::M => write!(f, "M"),
        }
    }
}

impl TryFrom<u8> for Register {
    fn try_from(value: u8) -> Result<Register, &'static str> {
        match value {
            0 => Ok(Register::B),
            1 => Ok(Register::C),
            2 => Ok(Register::D),
            3 => Ok(Register::E),
            4 => Ok(Register::H),
            5 => Ok(Register::L),
            6 => Ok(Register::M),
            7 => Ok(Register::A),
            _ => Err("Invalid register reference"),
        }
    }

    type Error = &'static str;
}
