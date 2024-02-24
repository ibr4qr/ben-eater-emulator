use vm::rom::{build_rom, Rom};

#[warn(dead_code)]
pub struct Emulator {
    pc: u8,
    ra: u8,
    rb: u8,
    mi: u8,
    ir: u8,
    cf: bool,
    zf: bool,
    rom: Rom,
    program_length: usize,
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
                self.increment_instruction_register();
            },
            2 => {
                // ADD 
                let memory_address = value;
                let loaded_value = self.rom.load_value(memory_address).unwrap();
                self.rb = *loaded_value;
                let sum = self.ra + self.rb;
                self.ra = sum;
                self.increment_instruction_register();
            },
            3 => {
                // SUB
                let memory_address = value;
                let loaded_value = self.rom.load_value(memory_address).unwrap();
                self.rb = *loaded_value;
                let diff = self.ra -  self.rb;
                self.ra = diff;
                self.zf = diff == 0;
                self.cf = diff != 0;
                self.increment_instruction_register();
            }
            4 => {
                // STA 
                let address = value;
                let value = self.ra;
                self.rom.set_value(address, value);
                self.increment_instruction_register();
            }
            0 => {
                let value = self.ra;
                println!("{}", value);
                self.increment_instruction_register();


            },
            6 => {
                let address = value;
                self.pc = address;
                self.ir = address;
            },
            7 => {
                self.ra = value;
                self.increment_instruction_register();

            },
            9 => {
                if self.cf {
                    self.pc = value;
                    self.ir = value;
                    self.cf = false;
                } else {
                    self.increment_instruction_register();
                }
            },
            8 => {
                //JZ
                if self.zf {
                    self.pc = value;
                    self.ir = value;
                    self.zf = false;
                } else {
                    self.increment_instruction_register();
                }
            }
            _ => println!("it s something else"),
        }
    }


    pub fn execute_program(&mut self) {
       while self.ir < (self.program_length as u8) {
        let instruction = self.rom.load_value(self.ir).unwrap();
        self.execute_instruction(*instruction);
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
        rom: build_rom()
    }
}