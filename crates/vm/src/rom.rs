use std::{collections::{HashMap}};


pub const MEMORY_SIZE: i8 = 16;


// the key is the address, the value is anothe address
pub type Values = HashMap<u8, u8>;

pub struct Rom {
   pub values: Values,
}


impl Rom {
    pub fn set_value(&mut self, k: u8, v: u8)  {
        self.values.insert(k, v);
    }

    pub fn load_value(&self, k: u8) -> Option<&u8> {
        let value = self.values.get(&k);

        return value;
    }

    pub fn load_program(&mut self, code: &[u8]) {
        for(index, item) in code.iter().enumerate() {
            let instruction_address = index;
            let instruction: u8 = *item;
            self.set_value(instruction_address as u8, instruction);
        }
    }
}

pub fn build_rom() -> Rom {
    Rom {
        values: Values::new()
    }
}