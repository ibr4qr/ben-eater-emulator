// 8 bit instruction set

// https://www.ullright.org/ullWiki/show/ben-eater-8-bit-computer-sap1
#[derive(Debug)]
pub enum InstructionSet {
    NOP = 0x0,  // NOP
    LDA = 0x1,  // Load contents of memory address aaaa into register A.
    ADD = 0x2,  // Put content of memory address aaaa into register B, add A + B, store result in A.
    SUB = 0x3, // Put content of memory address aaaa into register B, substract A - B, store result in register A.
    STA = 0x4, // Store contents of register A at memory address aaaa.
    LDI = 0x5, // Load 4 bit immediate value in register A (loads 'vvvv' in A).
    JMP = 0x6, // Unconditional jump. Set program counter (PC) to aaaa, resume execution from that memory address.
    JC = 0x7, // Jump if carry. Set PC to aaaa when carry flag is set and resume from there. When carry flag is not set resume normally.
    JZ = 0x8, // jump if zero flag is set.
    OUT = 0xE, // Output register A to 7 segment LED display as decimal.
    HALT = 0xF, //  Halt execution.
}

impl TryFrom<u8> for InstructionSet {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(InstructionSet::NOP),
            0x1 => Ok(InstructionSet::LDA),
            0x2 => Ok(InstructionSet::ADD),
            0x3 => Ok(InstructionSet::SUB),
            0x4 => Ok(InstructionSet::STA),
            0x6 => Ok(InstructionSet::JMP),
            0x5 => Ok(InstructionSet::LDI),
            0x7 => Ok(InstructionSet::JC),
            0x8 => Ok(InstructionSet::JZ),
            0xE => Ok(InstructionSet::OUT),
            0xF => Ok(InstructionSet::HALT),
            _ => Err(()),
        }
    }
}
