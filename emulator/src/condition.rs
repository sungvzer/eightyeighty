use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Condition {
    /// Zero flag not set
    NZ,

    /// Zero flag set
    Z,

    /// Carry flag not set
    NC,

    /// Carry flag set
    C,

    /// Parity flag not set (odd)
    PO,

    /// Parity flag set (even)
    PE,

    /// Sign flag not set (positive)
    P,

    /// Sign flag set (minus)
    M,
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::NZ => write!(f, "NZ"),
            Condition::Z => write!(f, "Z"),
            Condition::NC => write!(f, "NC"),
            Condition::C => write!(f, "C"),
            Condition::PO => write!(f, "PO"),
            Condition::PE => write!(f, "PE"),
            Condition::P => write!(f, "P"),
            Condition::M => write!(f, "M"),
        }
    }
}

impl TryFrom<u8> for Condition {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Condition::NZ),
            1 => Ok(Condition::Z),
            2 => Ok(Condition::NC),
            3 => Ok(Condition::C),
            4 => Ok(Condition::PO),
            5 => Ok(Condition::PE),
            6 => Ok(Condition::P),
            7 => Ok(Condition::M),
            _ => Err("Invalid condition code"),
        }
    }
}
