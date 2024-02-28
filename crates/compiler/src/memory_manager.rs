


// we will have expression that resolve to a value
// and this resolved value should always be put into the A register


// print(10);
// LDI 10
// OUT

use std::vec;

// print(2+2);
// LDI 2
// STA 9
// LDI 2
// ADD 9
// OUT
pub struct MemoryManager {
    memory_addresses: [u8; 16],
    memory_address_in_use: Vec<u8>
}


// TODO: free the space when is not longer needed
impl MemoryManager {
   pub fn get_memory_offset(&mut self) -> u8 {
    for memory_address in self.memory_addresses.into_iter() {
        if !self.memory_address_in_use.contains(&memory_address) {
            self.memory_address_in_use.push(memory_address);
            return memory_address;
        }   
    }

    return 100;
   }    
}



pub fn build_memory_manager() -> MemoryManager {
    MemoryManager {
        memory_addresses: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],
        memory_address_in_use: vec![]
    }
}