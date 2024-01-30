use compiler::rom::{build_rom, Rom};

#[warn(dead_code)]
pub struct Emulator {
    pc: u8,
    ra: u8,
    rb: u8,
    mi: u8,
    ir: u8,
    rom: Rom,
    program_length: usize
}

impl Emulator {
    pub fn load_program(&mut self, code: &[u8]) {
        self.program_length = code.len();
        self.rom.load_program(code);
    }


    fn increment_instruction_register(&mut self) {
        self.ir = self.ir + 1;
        self.pc = self.pc + 1;
    }

    fn execute_instruction(&mut self, instruction: u8) {

        let raw_instruction = instruction >> 4;
        let value = instruction & 0b00001111;

        

        match raw_instruction {
            1  => { // 00010001
                // LDA 
                let address = value;
                let loaded_value = self.rom.load_value(address).unwrap();
                self.ra = *loaded_value;
                println!("LDA {}", loaded_value);
            },
            2 => {
                // ADD 
                let memory_address = value;
                println!("memory address: {}", memory_address);
                let loaded_value = self.rom.load_value(memory_address).unwrap();
                self.rb = *loaded_value;
                let sum = self.ra + self.rb;
                self.ra = sum;
            },
            3 => {
                // SUB
                let memory_address = value;
                let loaded_value = self.rom.load_value(memory_address).unwrap();
                self.rb = *loaded_value;
                let diff = self.ra -  self.rb;
                self.ra = diff;
            }
            4 => {
                // STA 
                let address = value;
                let value = self.ra;
                self.rom.set_value(address, value);
            }
            0 => {
                let value = self.ra;
                println!("PRINTED VALUE: {}", value);
            },
            6 => {
                // SET PC TO AAAA 
                let address = value;
                self.pc = address;
                self.ir = address;
                
            },
            7 => {
                // LDI
                println!("LDI {}", value);
                self.ra = value;
            },
            8 => {
                self.pc = value;
                self.ir = value;
            }
            _ => println!("it s something else"),
        }
        
    }


    pub fn execute_program(&mut self) {
       while self.ir < (self.program_length as u8) {
        let instruction = self.rom.load_value(self.ir).unwrap();
        self.execute_instruction(*instruction);
        self.increment_instruction_register();
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
        rom: build_rom()
    }
}