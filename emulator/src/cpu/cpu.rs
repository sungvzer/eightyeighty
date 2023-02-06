use log::trace;

use crate::{
    condition::Condition,
    instruction::Instruction,
    parser::InstructionParser,
    register::{Register, RegisterPair},
};

pub struct CPU {
    /// Registers stored in this order:
    ///
    /// `[Flags, A, C, B, E, D, L, H]`
    registers: [u8; 8],
    stack_pointer: u16,
    program_counter: u16,
    memory: Vec<u8>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FlagMask {
    S = 0x80,
    Z = 0x40,
    _A = 0x10,
    P = 0x04,
    C = 0x01,
}

impl CPU {
    pub fn new() -> Self {
        let mut memory = Vec::with_capacity(1024 * 64);
        memory.resize(1024 * 64, 0);
        CPU {
            registers: [0; 8],
            stack_pointer: 0xffff,
            program_counter: 0,
            memory,
        }
    }

    fn register_to_internal_index(&self, register: Register) -> usize {
        assert!(
            register != Register::M,
            "M register should not be mapped to internal index"
        );
        match register {
            Register::A => 1,
            Register::C => 2,
            Register::B => 3,
            Register::E => 4,
            Register::D => 5,
            Register::L => 6,
            Register::H => 7,
            _ => panic!(),
        }
    }

    fn register(&self, register: Register, insn: &Instruction) -> u8 {
        if register == Register::M {
            let addr = self.get_register_pair(RegisterPair::HL, insn);
            return self.memory[addr as usize];
        }
        self.registers[self.register_to_internal_index(register)]
    }

    fn set_register(&mut self, register: Register, value: u8, insn: &Instruction) {
        if register == Register::M {
            let addr = self.get_register_pair(RegisterPair::HL, insn);
            self.memory[addr as usize] = value;
            return;
        }
        self.registers[self.register_to_internal_index(register)] = value;
    }

    pub fn load_program(&mut self, program: &Vec<u8>) {
        for (index, value) in program.iter().enumerate() {
            self.memory[index] = *value;
        }
    }

    fn stack_push(&mut self, value: u16) {
        let high_byte = (value & 0xff00 >> 8) as u8;
        let low_byte = (value & 0x00ff) as u8;
        let stack_pointer = self.stack_pointer as usize;
        self.memory[stack_pointer - 1] = high_byte;
        self.memory[stack_pointer - 2] = low_byte;
        self.stack_pointer -= 2;
    }

    fn stack_pop(&mut self) -> u16 {
        let low_byte = self.memory[self.stack_pointer as usize] as u16;
        let high_byte = self.memory[(self.stack_pointer + 1) as usize] as u16;

        let value: u16 = (high_byte << 8) + low_byte;
        self.stack_pointer += 2;
        value
    }

    fn get_register_pair(&self, pair: RegisterPair, insn: &Instruction) -> u16 {
        let (high_register, low_register) = match pair {
            RegisterPair::SP => match insn {
                Instruction::POP(_) | Instruction::PUSH(_) => {
                    todo!("Get PSW ");
                }
                _ => {
                    return self.stack_pointer;
                }
            },
            RegisterPair::DE => (Register::D, Register::E),
            RegisterPair::HL => (Register::H, Register::L),
            RegisterPair::BC => (Register::B, Register::C),
            _ => todo!("Get register pair {pair} "),
        };

        let low_byte = self.register(low_register, insn) as u16;
        let high_byte = self.register(high_register, insn) as u16;
        ((high_byte << 8) | low_byte).into()
    }

    fn set_register_pair(&mut self, pair: RegisterPair, value: u16, insn: &Instruction) {
        // TODO: Verify that this function handles endianness correctly
        let low_byte = (value & 0x00ff) as u8;
        let high_byte = (value & 0xff00 >> 8) as u8;

        let (high_register, low_register) = match pair {
            RegisterPair::SP => match insn {
                Instruction::POP(_) | Instruction::PUSH(_) => {
                    todo!("Set PSW to #${value:04x}");
                }
                _ => {
                    self.stack_pointer = value;
                    return;
                }
            },
            RegisterPair::DE => (Register::D, Register::E),
            RegisterPair::HL => (Register::H, Register::L),
            RegisterPair::BC => (Register::B, Register::C),
            _ => todo!("Set register pair {pair} to #${value:04x}"),
        };
        self.set_register(high_register, high_byte, insn);
        self.set_register(low_register, low_byte, insn);
    }

    fn get_flag(&self, flag_mask: FlagMask) -> bool {
        (self.registers[0] & flag_mask as u8) != 0
    }

    fn verify_condition(&self, condition: Condition) -> bool {
        match condition {
            Condition::NZ => !self.get_flag(FlagMask::Z),
            Condition::Z => self.get_flag(FlagMask::Z),
            Condition::NC => !self.get_flag(FlagMask::C),
            Condition::C => self.get_flag(FlagMask::C),
            Condition::PO => !self.get_flag(FlagMask::P),
            Condition::PE => self.get_flag(FlagMask::P),
            Condition::P => !self.get_flag(FlagMask::S),
            Condition::M => self.get_flag(FlagMask::S),
        }
    }

    fn set_flag(&mut self, flag_mask: FlagMask) {
        let flag_byte = &mut self.registers[0];

        *flag_byte |= flag_mask as u8;
    }

    fn unset_flag(&mut self, flag_mask: FlagMask) {
        let flag_byte = &mut self.registers[0];

        *flag_byte &= 0xff - flag_mask as u8;
    }

    fn update_flags(&mut self, value: u8) {
        // Zero flag
        if value == 0 {
            self.set_flag(FlagMask::Z);
        } else {
            self.unset_flag(FlagMask::Z);
        }

        // Sign flag
        if (value & 0x80) != 0 {
            self.set_flag(FlagMask::S);
        } else {
            self.unset_flag(FlagMask::S);
        }

        // Parity flag
        let mut bits: u8 = 0;
        let mut mask = 0x01;
        for _ in 0..8 {
            if value & mask != 0 {
                bits += 1;
            }
            mask <<= 1;
        }

        if bits % 2 == 0 {
            self.set_flag(FlagMask::P);
        } else {
            self.unset_flag(FlagMask::P);
        }

        // TODO: set auxiliary carry
    }

    pub fn fetch_decode_execute(&mut self) {
        // Fetch
        let program_counter = self.program_counter as usize;

        let current_instruction_byte = self.memory[program_counter];
        let bytes_to_read = InstructionParser::bytes_to_read(current_instruction_byte);

        let slice = &self.memory[program_counter..=program_counter + bytes_to_read];

        // Decode
        let insn = InstructionParser::parse_bytes(slice);
        if insn.is_none() {
            return;
        }
        let insn = insn.unwrap();

        // Execute
        let instruction_size = bytes_to_read + 1;
        trace!("Executing: {insn}");
        match insn {
            Instruction::NOP => {}
            Instruction::JMP(addr) => {
                self.program_counter = addr;
                return;
            }
            Instruction::LXI(register_pair, immediate) => {
                self.set_register_pair(register_pair, immediate, &insn)
            }
            Instruction::MVI(register, immediate) => {
                self.set_register(register, immediate, &insn);
            }
            Instruction::CPI(immediate) => {
                let result = self.register(Register::A, &insn).wrapping_sub(immediate);
                self.update_flags(result);
            }
            Instruction::J(condition, addr) => {
                if self.verify_condition(condition) {
                    self.program_counter = addr;
                    return;
                }
            }
            Instruction::POP(pair) => {
                let value = self.stack_pop();
                self.set_register_pair(pair, value, &insn);
            }
            Instruction::CALL(addr) => {
                self.stack_push(self.program_counter);
                self.program_counter = addr;
                return;
            }
            Instruction::RET => {
                self.program_counter = self.stack_pop();
            }
            Instruction::LDAX(pair) => {
                assert!(pair == RegisterPair::BC || pair == RegisterPair::DE);
                let address = self.get_register_pair(pair, &insn);
                self.set_register(Register::A, self.memory[address as usize], &insn);
            }
            Instruction::MOV(dest, src) => {
                let src_value = self.register(src, &insn);
                self.set_register(dest, src_value, &insn);
            }
            Instruction::INX(pair) => {
                let value = self.get_register_pair(pair, &insn);
                self.set_register_pair(pair, value.wrapping_add(1), &insn);
            }
            Instruction::DCR(register) => {
                let value = self.register(register, &insn);
                let result = value.wrapping_sub(1);
                self.update_flags(result);
                self.set_register(register, result, &insn);
            }
            _ => todo!("Implement instruction {}", insn),
        };
        self.program_counter += instruction_size as u16;
    }
}
