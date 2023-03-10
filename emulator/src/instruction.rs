use std::fmt::{Debug, Display};

use crate::{
    condition::Condition,
    register::{Register, RegisterPair},
};

#[derive(Clone, Copy)]
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

    /// Subtract register from A
    SUB(Register),

    /// Subtract immediate from A
    SUI(u8),

    /// Subtract register from A with borrow
    SBB(Register),

    /// Subtract immediate from A with borrow
    SBI(u8),

    /// Increment register
    INR(Register),

    /// Decrement register
    DCR(Register),

    /// Increment register pair
    INX(RegisterPair),

    /// Decrement register pair
    DCX(RegisterPair),

    /// Add register pair to HL (16 bit add)
    DAD(RegisterPair),

    /// AND register with A
    ANA(Register),

    /// AND immediate with A
    ANI(u8),

    /// OR register with A
    ORA(Register),

    /// OR immediate with A
    ORI(u8),

    /// XOR register with A
    XRA(Register),

    /// XOR immediate with A
    XRI(u8),

    /// Compare register with A
    CMP(Register),

    /// Compare immediate with A
    CPI(u8),

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

    /// Unconditional jump
    JMP(u16),

    /// Conditional jump
    J(Condition, u16),

    /// Unconditional subroutine call
    CALL(u16),

    /// Conditional subroutine call
    C(Condition, u16),

    /// Unconditional return from subroutine
    RET,

    /// Conditional return from subroutine
    R(Condition),

    /// Restart (Call n*8)
    RST(u8),

    /// Push register pair on the stack
    PUSH(RegisterPair),

    /// Pop register pair from the stack
    POP(RegisterPair),

    /// Jump to address in H:L
    PCHL,

    /// Swap H:L with top word on stack
    XTHL,

    /// Set SP to content of H:L
    SPHL,

    /// Read input port into A
    IN(u8),

    /// Write A to output port
    OUT(u8),

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
            Instruction::JMP(arg0) => write!(f, "JMP ${:02x}", arg0),
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
            Instruction::CMP(src) => write!(f, "CMP {src}"),
            Instruction::INR(src) => write!(f, "INR {src}"),
            Instruction::DCR(src) => write!(f, "DCR {src}"),
            Instruction::SUB(src) => write!(f, "SUB {src}"),
            Instruction::ADI(imm) => write!(f, "ADI #${imm:02x}"),
            Instruction::SUI(imm) => write!(f, "SUI #${imm:02x}"),
            Instruction::CPI(imm) => write!(f, "CPI #${imm:02x}"),
            Instruction::ADC(src) => write!(f, "ADC {src}"),
            Instruction::ANA(src) => write!(f, "ANA {src}"),
            Instruction::XRA(src) => write!(f, "XRA {src}"),
            Instruction::ORA(src) => write!(f, "ORA {src}"),
            Instruction::SBB(src) => write!(f, "SBB {src}"),
            Instruction::RST(n) => write!(f, "RST {n}"),
            Instruction::ACI(imm) => write!(f, "ACI #${imm:02x}"),
            Instruction::SBI(imm) => write!(f, "SBI #${imm:02x}"),
            Instruction::IN(imm) => write!(f, "IN #${imm:02x}"),
            Instruction::OUT(imm) => write!(f, "OUT #${imm:02x}"),
            Instruction::ANI(imm) => write!(f, "ANI #${imm:02x}"),
            Instruction::XRI(imm) => write!(f, "XRI #${imm:02x}"),
            Instruction::ORI(imm) => write!(f, "ORI #${imm:02x}"),
            Instruction::MVI(dest, imm) => write!(f, "MVI {dest},#${imm:02x}"),
            Instruction::LXI(pair, imm) => write!(f, "LXI {pair},#${imm:04x}"),
            Instruction::LDA(address) => write!(f, "LDA ${address:04x}"),
            Instruction::STA(address) => write!(f, "STA ${address:04x}"),
            Instruction::LHLD(address) => write!(f, "LHLD ${address:04x}"),
            Instruction::SHLD(address) => write!(f, "SHLD ${address:04x}"),
            Instruction::LDAX(pair) => write!(f, "LDAX {pair}"),
            Instruction::STAX(pair) => write!(f, "STAX {pair}"),
            Instruction::INX(pair) => write!(f, "INX {pair}"),
            Instruction::PUSH(pair) => {
                if let RegisterPair::SP = pair {
                    return write!(f, "PUSH PSW");
                }
                write!(f, "PUSH {pair}")
            }
            Instruction::POP(pair) => {
                if let RegisterPair::SP = pair {
                    return write!(f, "POP PSW");
                }
                write!(f, "POP {pair}")
            }
            Instruction::DCX(pair) => write!(f, "DCX {pair}"),
            Instruction::DAD(pair) => write!(f, "DAD {pair}"),
            Instruction::J(condition, addr) => write!(f, "J{condition} ${addr:04x}"),
            Instruction::C(condition, addr) => write!(f, "C{condition} ${addr:04x}"),
            Instruction::R(condition) => write!(f, "R{condition}"),
            Instruction::CALL(addr) => write!(f, "CALL ${addr:04x}"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}
