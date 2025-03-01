use crate::instruction_set::InstructionSet;
use crate::rom::{build_rom, Rom};

pub struct Emulator {
    pub pc: u8,
    pub ra: u8,
    pub rb: u8,
    pub mi: u8,
    pub ir: u8,
    pub cf: bool,
    pub zf: bool,
    pub rom: Rom,
    pub program_length: usize,
}

impl Emulator {
    pub fn load_program(&mut self, code: &[u8]) {
        // load program
        self.program_length = code.len();
        self.rom.load_program(code);

        // set instruction register to the first instruction
        self.ir = *self.rom.load_value(self.pc).unwrap();
    }

    pub fn execute_instruction(&mut self) {
        let instruction = self.ir;
        let op_code = InstructionSet::try_from(instruction >> 4).unwrap();
        let argument = instruction & 0b00001111;

        let mut next_instruction_address = self.pc + 1;

        match op_code {
            InstructionSet::NOP => {
                // no op
            }
            InstructionSet::OUT => {
                println!("{}", self.ra);
            }
            InstructionSet::LDA => {
                // put in A register the content of ROM[argument]
                self.ra = *self.rom.load_value(argument).unwrap();
            }
            InstructionSet::ADD => {
                let memory_address = argument;
                let loaded_value = self.rom.load_value(memory_address).unwrap();
                self.rb = *loaded_value;
                let sum = self.ra + self.rb;
                self.ra = sum;
            }
            InstructionSet::SUB => {
                self.rb = *self.rom.load_value(argument).unwrap();
                let sub_result = self.ra - self.rb;
                self.ra = sub_result;

                // flow control registers
                self.zf = sub_result == 0;
                self.cf = sub_result != 0;
            }
            InstructionSet::STA => {
                let value = self.ra;
                self.rom.set_value(argument, value);
            }
            InstructionSet::JMP => {
                next_instruction_address = argument;
            }
            InstructionSet::LDI => {
                self.ra = argument;
            }
            InstructionSet::JZ => {
                if self.zf {
                    next_instruction_address = argument;
                    self.zf = false;
                }
            }
            InstructionSet::JC => {
                if self.cf {
                    next_instruction_address = argument;
                    self.cf = false;
                }
            }
            // halt execution
            InstructionSet::HALT => todo!(),
        }

        // save program counter to the next instruction address
        self.pc = next_instruction_address
    }

    pub fn fetch_instruction(&mut self) {
        if self.pc < (self.program_length as u8) {
            self.ir = *self.rom.load_value(self.pc).unwrap();
        }
    }

    pub fn execute_program(&mut self) {
        while self.pc < (self.program_length as u8) {
            self.execute_instruction();
            self.fetch_instruction();
        }
    }
}

pub fn build_emulator() -> Emulator {
    Emulator {
        program_length: 0,
        pc: 0,
        ra: 0,
        rb: 0,
        mi: 0,
        ir: 0,
        cf: false,
        zf: false,
        rom: build_rom(),
    }
}
