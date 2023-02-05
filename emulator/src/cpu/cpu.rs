use log::trace;

use crate::{instruction::Instruction, parser::InstructionParser, register::Register};

pub struct CPU {
    /// Registers stored in this order:
    ///
    /// `[Flags, A, C, B, E, D, L, H]`
    registers: [u8; 8],
    _stack_pointer: u16,
    program_counter: u16,
    _status: u8,
    memory: Vec<u8>,
}

impl CPU {
    pub fn new() -> Self {
        let mut memory = Vec::with_capacity(1024 * 64);
        memory.resize(1024 * 64, 0);
        CPU {
            registers: [0; 8],
            _stack_pointer: 0,
            program_counter: 0,
            _status: 0,
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

    pub fn register(&self, register: Register) -> u8 {
        if register == Register::M {
            todo!("Read from memory");
        }
        self.registers[self.register_to_internal_index(register)]
    }

    pub fn set_register(&mut self, register: Register, value: u8) {
        if register == Register::M {
            todo!("Write memory");
        }
        self.registers[self.register_to_internal_index(register)] = value;
    }

    pub fn load_program(&mut self, program: &Vec<u8>) {
        for (index, value) in program.iter().enumerate() {
            self.memory[index] = *value;
        }
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
            _ => todo!("Implement instruction {}", insn),
        };
        self.program_counter += instruction_size as u16;
    }
}
