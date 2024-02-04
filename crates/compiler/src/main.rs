use emualtor::{build_emulator, Emulator};

pub mod emualtor;
pub mod rom;

fn main() {
    /**
     * sample program
     * LDI 17
     * LDI 20
     * OUT
     * LDI 17
     * OUT
     */
    let sample1 = [17, 20, 0, 17, 0];

    /**
     * 0, LDI 12
     * 1, OUT
     * 2, STA 16
     * 3, LDI 6
     * 4, OUT
     * 5, LDA 16
     * 6, OUT
     */
    let sample2 = [
        0b01111100, 0b00000000, 0b01001111, 0b01110110, 0b00000000, 0b00000000, 0b00011111, 0,
    ];

    /**
     *  0, LDI 10
     *  1, OUT
     *  2, JMP
     */
    let sample3 = [0b01111010, 0b00000000, 0b01100000];

    /**
     * LDI 1
     * OUT
     * STA 13
     * LDI 10
     * SUB 13
     * OUT
     * STA 14
     * LDI 2
     */
    let sample4 = [
        0b01110001, 0b00000000, 0b01001011, 0b01111010, 0b00111011, 0b00000000, 0b01001100,
        0b01110110, 0b00101100, 0b00000000,
    ];

    /**
     * count to 5
     * LDI 5
     * STA 14
     * LDI 1 ==> a = 1
     * STA 10
     * ADD 10
     * STA 15
     * LDA 14
     * SUB 15
     * JZ 4
     */
    let sample5 = [
        0b01110101, // LDI 5
        0b01001110, // STA 14 ==> 5 is in 14 address memory


        // initial value store 1 somewhere into memory
        0b01110001, // LDI 1
        0b01001010, // STA 10 ==> 1 is in 10 address memory
        
        0b00101010, // ADD 10
        0b01001111, // STA 15
        


        0b00011110, // LDA 14
        0b00111111, // SUB 15
        0b10010011  // JZ  4
    ];

    let mut emulator: Emulator = build_emulator();
    emulator.load_program(&sample5);
    emulator.execute_program();
}
