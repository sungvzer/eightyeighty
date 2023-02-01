use std::fmt::Debug;

use crate::register::{Register, RegisterPair};

pub enum Instruction {
    /// Move register to register
    MOV(Register, Register),

    /// Move immediate to register
    MVI(Register, u8),

    /// Load register pair immediate
    LXI(RegisterPair, u16),

    /// Load A from memory
    LDA(u16),

    /// Store A to memory
    STA(u16),

    /// Unconditional jump
    JMP(u16),

    /// Exchange DE and HL content
    XCHG,

    /// Decimal Adjust Accumulator
    DAA,

    /// Rotate A left
    RLC,

    /// Rotate A right
    RRC,

    /// Rotate A left through carry
    RAL,

    /// Rotate A right through carry
    RAR,

    /// Complement A
    CMA,

    /// Complement A Carry flag
    CMC,

    /// Set Carry flag
    STC,

    /// Unconditional return from subroutine
    RET,

    /// Jump to address in H:L
    PCHL,

    /// Swap H:L with top word on stack
    XTHL,

    /// Set SP to content of H:L
    SPHL,

    /// Enable interrupt
    EI,

    /// Disable interrupts
    DI,

    /// Halt processor
    HLT,

    /// No operation
    NOP,
    Unknown,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::NOP => write!(f, "NOP"),
            Instruction::HLT => write!(f, "HLT"),
            Instruction::JMP(arg0) => write!(f, "JMP $({:02x})", arg0),
            Instruction::Unknown => write!(f, "Unknown"),
            Instruction::EI => write!(f, "EI"),
            Instruction::DI => write!(f, "DI"),
            Instruction::SPHL => write!(f, "SPHL"),
            Instruction::XTHL => write!(f, "XTHL"),
            Instruction::PCHL => write!(f, "PCHL"),
            Instruction::XCHG => write!(f, "XCHG"),
            Instruction::DAA => write!(f, "DAA"),
            Instruction::RLC => write!(f, "RLC"),
            Instruction::RRC => write!(f, "RRC"),
            Instruction::RAL => write!(f, "RAL"),
            Instruction::RAR => write!(f, "RAR"),
            Instruction::CMA => write!(f, "CMA"),
            Instruction::CMC => write!(f, "CMC"),
            Instruction::STC => write!(f, "STC"),
            Instruction::RET => write!(f, "RET"),
            Instruction::MOV(dest, src) => write!(f, "MOV {dest},{src}"),
            Instruction::MVI(dest, imm) => write!(f, "MVI {dest},#${imm:02x}"),
            Instruction::LXI(pair, imm) => write!(f, "LXI {pair},#${imm:04x}"),
            Instruction::LDA(address) => write!(f, "LDA ${address:04x}"),
            Instruction::STA(address) => write!(f, "STA ${address:04x}"),
        }
    }
}
