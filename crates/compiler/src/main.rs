
use emualtor::{Emulator, build_emulator};



pub mod emualtor;
pub mod rom;



fn main() {
    /**
     * sample program
     * LDA 17
     * LDA 20
     * OUT
     * LDA 17 
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

    let sample2 = [0b01111100, 0b00000000, 0b01001111, 0b01110110, 0b00000000, 0b00000000, 0b00011111, 0];


    /**
     *  0, LDI 10
     *  1, OUT
     *  2, JMP 
     */

    let sample3 = [0b01111010, 0b00000000, 0b01100000];

    let mut emulator:Emulator = build_emulator();
    emulator.load_program(&sample3);
    emulator.execute_program()
}