





// 8 bit instruction set



// https://www.ullright.org/ullWiki/show/ben-eater-8-bit-computer-sap1
pub enum InstructionSet {
    NOP         = 0,   // NOP
    LDA         = 0b0001,       // Load contents of memory address aaaa into register A.
    ADD         = 0b0010,       // Put content of memory address aaaa into register B, add A + B, store result in A.
    SUB         = 0b00110000,   // Put content of memory address aaaa into register B, substract A - B, store result in register A.
    STA         = 0b01000000,   // Store contents of register A at memory address aaaa.
    OUT         = 0b0000,       // Output register A to 7 segment LED display as decimal.
    JMP         = 0b01100000,   // Unconditional jump. Set program counter (PC) to aaaa, resume execution from that memory address.
    LDI         = 0b0111,       // Load 4 bit immediate value in register A (loads 'vvvv' in A).
    JC          = 0b01110000,   // Jump if carry. Set PC to aaaa when carry flag is set and resume from there. When carry flag is not set resume normally.
    JZ          = 0b1000,       // jump if zero flag is set.
    JNC         = 0b10010000,
    HALT        = 0b11110000    //  Halt execution.
}