use std::u8;
use vm::instruction_set::InstructionSet;

pub struct Parser {
    pub binary_code: Vec<u8>, // binary_code: Vec<u8>
}

impl Parser {
    pub fn parse_instruction(&self, line: String) -> u8 {
        // in collection we now have as first argument the op_code and as second the argument of the op_code
        let parts = line.split(" ");
        let collection = parts.collect::<Vec<&str>>();

        // we split the op_code from the argument
        let op_code = collection[0];
        let mut argument = 0;

        let mut raw_instruction = 0;

        match op_code {
            "LDA" => {
                raw_instruction = InstructionSet::LDA as u8;
                argument = (collection[1]).parse().unwrap();
            }
            "ADD" => {
                raw_instruction = InstructionSet::ADD as u8;
                argument = (collection[1]).parse().unwrap();
            }
            "SUB" => {
                raw_instruction = InstructionSet::SUB as u8;
                argument = (collection[1]).parse().unwrap();
            }
            "STA" => {
                raw_instruction = InstructionSet::STA as u8;
                argument = (collection[1]).parse().unwrap();
            }
            "OUT" => {
                raw_instruction = InstructionSet::OUT as u8;
            }
            "JMP" => {
                raw_instruction = InstructionSet::JMP as u8;
                argument = (collection[1]).parse().unwrap();
            }
            "LDI" => {
                raw_instruction = InstructionSet::LDI as u8;
                argument = (collection[1]).parse().unwrap();
            }
            "JC" => {
                raw_instruction = InstructionSet::JC as u8;
                argument = (collection[1]).parse().unwrap();
            }
            "JZ" => {
                raw_instruction = InstructionSet::JZ as u8;
                argument = (collection[1]).parse().unwrap();
            }
            "HALT" => {
                raw_instruction = InstructionSet::HALT as u8;
            }
            _ => println!("The instruction {} is not supported!", op_code),
        }

        let mut final_instruction;

        // craft the final instructin with the op_code
        // in the 4 MSB and the argument in the 4 LSB
        final_instruction = (raw_instruction << 4) & 0b11110000;
        final_instruction = final_instruction | argument;

        final_instruction
    }

    pub fn parse(&mut self, instructions: Vec<String>) {
        for instruction in instructions {
            let machine_code: u8 = self.parse_instruction(instruction);
            self.binary_code.push(machine_code);
        }
    }
}

pub fn build_parser() -> Parser {
    Parser {
        binary_code: vec![],
    }
}
