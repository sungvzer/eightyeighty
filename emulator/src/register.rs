use std::fmt::Display;

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
