use std::fmt::{Debug, Display};

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

    /// Load H:L from memory
    LHLD(u16),

    /// Store H:L to memory
    SHLD(u16),

    /// Load indirect through BC or DE
    LDAX(RegisterPair),

    /// Store indirect through BC or DE
    STAX(RegisterPair),

    /// Unconditional jump
    JMP(u16),

    /// Exchange DE and HL content
    XCHG,

    /// Add register to A
    ADD(Register),

    /// Add immediate to A
    ADI(u8),

    /// Add register to A with carry
    ADC(Register),

    /// Add immediate to A with carry
    ACI(u8),

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
            Instruction::ADD(src) => write!(f, "ADD {src}"),
            Instruction::ADI(imm) => write!(f, "ADI #${imm:02x}"),
            Instruction::ADC(src) => write!(f, "ADC {src}"),
            Instruction::ACI(imm) => write!(f, "ACI #${imm:02x}"),
            Instruction::MVI(dest, imm) => write!(f, "MVI {dest},#${imm:02x}"),
            Instruction::LXI(pair, imm) => write!(f, "LXI {pair},#${imm:04x}"),
            Instruction::LDA(address) => write!(f, "LDA ${address:04x}"),
            Instruction::STA(address) => write!(f, "STA ${address:04x}"),
            Instruction::LHLD(address) => write!(f, "LHLD ${address:04x}"),
            Instruction::SHLD(address) => write!(f, "SHLD ${address:04x}"),
            Instruction::LDAX(pair) => write!(f, "LDAX {pair}"),
            Instruction::STAX(pair) => write!(f, "STAX {pair}"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}
