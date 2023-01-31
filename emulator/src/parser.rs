use crate::instruction::Instruction;

pub struct InstructionParser {
    buffer: Vec<u8>,
    cursor: usize,
}

impl InstructionParser {
    pub fn new(buffer: Vec<u8>) -> InstructionParser {
        InstructionParser { buffer, cursor: 0 }
    }

    fn consume_next_opcode(&mut self) -> Option<Vec<u8>> {
        if self.cursor == self.buffer.len() {
            return None;
        }
        let mut bytes = vec![];

        // FIXME: Error handling
        let mut current_byte = *self.buffer.get(self.cursor).unwrap();
        self.cursor += 1;

        bytes.push(current_byte);
        let number_of_bytes_to_read = self.bytes_to_read(current_byte);

        if self.cursor >= self.buffer.len() - number_of_bytes_to_read {
            return None;
        }

        for _ in 0..number_of_bytes_to_read {
            // FIXME: Error handling
            current_byte = *self.buffer.get(self.cursor).unwrap();
            bytes.push(current_byte);
            self.cursor += 1;
        }
        Some(bytes)
    }

    pub fn parse(&mut self) -> Option<Instruction> {
        let bytes = self.consume_next_opcode()?;
        for byte in bytes {
            print!("0x{:02x} ", byte);
        }
        println!();

        Some(Instruction::Unknown)
    }

    /**
    Returns the number of bytes to read **after** the current byte.

    The number is taken from the `size` column in the **opcodes.md** doc file minus 1.
    */
    fn bytes_to_read(&self, current_byte: u8) -> usize {
        if current_byte >= 0x40 && current_byte <= 0xbf {
            // From 0x40 to 0xbf all instructions are 1 byte long
            return 0;
        }
        match current_byte {
            0x00 => 0, // NOP
            0x01 => 2, // LXI B,d16
            0x02 => 0, // STAX B
            0x03 => 0, // INX B
            0x04 => 0, // INR B
            0x05 => 0, // DCR B
            0x06 => 1, // MVI B, d8
            0x07 => 0, // RLC
            0x08 => 0, // NOP
            0x09 => 0, // DAD B
            0x0a => 0, // LDAX B
            0x0b => 0, // DCX B
            0x0c => 0, // INR C
            0x0d => 0, // DCR C
            0x0e => 1, // MVI C,d8
            0x0f => 0, // RRC

            0x10 => 0, // NOP
            0x11 => 2, // LXI D,d16
            0x12 => 0, // STAX D
            0x13 => 0, // INX D
            0x14 => 0, // INR D
            0x15 => 0, // DCR D
            0x16 => 1, // MVI D,d8
            0x18 => 0, // NOP
            0x19 => 0, // DAD D
            0x1a => 0, // LDAX D
            0x1b => 0, // DCX D
            0x1c => 0, // INR E
            0x1d => 0, // DCR E
            0x1e => 1, // MVI E, d8
            0x1f => 0, // RAR

            0x20 => 0, // NOP
            0x22 => 2, // SHLD a16
            0x23 => 0, // INX H
            0x21 => 2, // LXI H,d16
            0x24 => 0, // INR H
            0x25 => 0, // DCR h
            0x26 => 1, // MVI H,d8
            0x27 => 0, // DAA
            0x28 => 0, // NOP
            0x29 => 0, // DAD H
            0x2a => 2, // LHLD a16
            0x2b => 0, // DCX H
            0x2c => 0, // INR L
            0x2e => 1, // MVI A,d8
            0x2f => 0, // CMA

            0x30 => 0, // NOP
            0x31 => 2, // LXI SP,d16
            0x32 => 2, // STA a16
            0x34 => 0, // INR M
            0x35 => 0, // DCR M
            0x36 => 1, // MVI M,d8
            0x37 => 0, // STC
            0x38 => 0, // NOP
            0x39 => 0, // DAD SP
            0x3a => 2, // LDA a16
            0x3c => 0, // INR A
            0x3d => 0, // DCR A
            0x3e => 1, // MVI A,d8
            0x3f => 0, // CMC

            /*
            0x40 .. 0xbf => 0
            */
            0xc0 => 0, // RNZ
            0xc1 => 0, // POP B
            0xc2 => 2, // JNZ a16
            0xc3 => 2, // JMP a16
            0xc4 => 2, // CNZ a16
            0xc5 => 0, // PUSH B
            0xc6 => 1, // ADI d8
            0xc8 => 0, // RZ
            0xc9 => 0, // RET
            0xca => 2, // JZ a16
            0xcd => 2, // CALL a16
            0xcc => 2, // CZ a16

            0xd0 => 0, // RNC
            0xd1 => 0, // POP D
            0xd2 => 2, // JNC a16
            0xd3 => 1, // OUT d8
            0xd4 => 2, // CNC a16
            0xd5 => 0, // PUSH D
            0xd6 => 1, // SUI d8
            0xd8 => 0, // RC
            0xda => 2, // JC a16
            0xdb => 1, // IN d8
            0xde => 1, // SBI d8

            0xe0 => 0, // RPO
            0xe1 => 0, // POP H
            0xe3 => 0, // XTHL
            0xe2 => 2, // JPO a16
            0xe5 => 0, // PUSH H
            0xe6 => 1, // ANI d8
            0xe9 => 0, // PCHL
            0xeb => 0, // XCHG
            0xeC => 2, // CPE a16
            0xee => 1, // XRI d8

            0xf0 => 0, // RP
            0xf1 => 0, // POP PSW
            0xf5 => 0, // PUSH PSW
            0xfa => 2, // JM a16
            0xfb => 0, // EI
            0xfc => 2, // CM a16
            0xfe => 1, // CPI d8
            0xf6 => 1, // ORI d8
            0xf8 => 0, // RM
            0xff => 0, // RST 7

            _ => todo!("Determine 0x{:02x} size", current_byte),
        }
    }
}
