#[cfg(test)]
mod tests {
    use vm::emulator::{build_emulator, Emulator};

    #[test]
    fn test_vm_component() {
        let mut emulator: Emulator = build_emulator();

        let sample = [
            0x52, // LDI 2
            0xE,  // OUT
        ];

        {
            emulator.load_program(&sample);

            assert_eq!(emulator.pc, 0);
            assert_eq!(emulator.ir, 0x52);

            // execute LDI 2
            emulator.execute_instruction();
            assert_eq!(emulator.ra, 0x2);
            assert_eq!(emulator.pc, 1);

            // A register should have value 2
            emulator.fetch_instruction();
            assert_eq!(emulator.ir, 0x0);
        }
    }

    #[test]
    fn test_add_and_sub() {
        let mut emulator: Emulator = build_emulator();
        let sample = [
            0x51, // LDI 1    (pc = 0)
            0x4F, // STA 15   (pc = 1)
            0x54, // LDI 4    (pc = 2)
            0x2F, // ADD 15   (pc = 3)
            0x4E, // STA 14   (pc = 4)
            0x53, // LDI 3    (pc = 5)
            0x4F, // STA 15   (pc = 6)
            0x1E, // LDA 14   (pc = 7)
            0x3F, // SUB 15   (pc = 8)
        ];

        {
            emulator.load_program(&sample);
            // LDI 1
            emulator.execute_instruction();
            assert_eq!(emulator.ra, 1);

            // STA 15
            emulator.fetch_instruction();
            emulator.execute_instruction();

            assert_eq!(emulator.rom.load_value(15).unwrap(), &1);

            // LDI 4
            emulator.fetch_instruction();
            emulator.execute_instruction();

            assert_eq!(emulator.ra, 4);

            // ADD 15
            emulator.fetch_instruction();
            emulator.execute_instruction();

            assert_eq!(emulator.ra, 5);

            // STA 14
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.rom.load_value(14).unwrap(), &5);

            // LDI 3
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.ra, 3);

            // STA 15
            emulator.fetch_instruction();
            emulator.execute_instruction();

            // LDA 14
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.ra, 5);
            assert_eq!(emulator.rom.load_value(15).unwrap(), &3);

            // // SUB 15
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.ra, 2);
        }
    }

    #[test]
    fn test_jmp() {
        let mut emulator: Emulator = build_emulator();
        let sample = [
            0x51, // LDI 1    (pc = 0)
            0x4F, // STA 15   (pc = 1)
            0x2F, // ADD 15   (pc = 2)
            0x62, // JMP 2    (pc = 3)
        ];

        {
            emulator.load_program(&sample);

            // LDI 1
            emulator.execute_instruction();

            // STA 15
            emulator.fetch_instruction();
            emulator.execute_instruction();

            // ADD 15
            emulator.fetch_instruction();
            emulator.execute_instruction();

            // current A register value is 2
            assert_eq!(emulator.ra, 2);

            // current program counter is 3
            assert_eq!(emulator.pc, 3);

            // JMP 2
            emulator.fetch_instruction();
            emulator.execute_instruction();

            // we are back to program counter 2
            assert_eq!(emulator.pc, 2);

            // ADD 15
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.ra, 3);

            // // JMP 2
            emulator.fetch_instruction();
            emulator.execute_instruction();

            // ADD 15
            emulator.fetch_instruction();
            emulator.execute_instruction();

            assert_eq!(emulator.ra, 4);
        }
    }

    #[test]
    fn test_jc() {
        let mut emulator: Emulator = build_emulator();

        let sample = [
            0x51, // LDI 1    (pc = 0)
            0x4E, // STA 14   (pc = 1)
            0x52, // LDI 2    (pc = 2)
            0x3E, // SUB 14   (pc = 3)
            0x73, // JC 3     (pc = 4)
            0x5F, // LDI 15   (pc = 5)
        ];

        {
            emulator.load_program(&sample);
            assert_eq!(emulator.pc, 0);
            // LDI 1
            emulator.execute_instruction();
            assert_eq!(emulator.pc, 1);

            // STA 15
            emulator.fetch_instruction();
            emulator.execute_instruction();

            assert_eq!(emulator.pc, 2);

            // LDI 2
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.pc, 3);

            // SUB 14
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.pc, 4);

            // JC 3 ( we should just to previous instruction sine 2 - 1 != 0)
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.pc, 3);

            // SUB 14
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.pc, 4);

            // JC 3
            emulator.fetch_instruction();
            emulator.execute_instruction();

            // LDI 15
            emulator.fetch_instruction();
            emulator.execute_instruction();

            assert_eq!(emulator.ra, 15);
        }
    }

    #[test]
    fn test_jz() {
        let mut emulator: Emulator = build_emulator();
        let sample = [
            0x51, // LDI 1    (pc = 0)
            0x4E, // STA 14   (pc = 1)
            0x51, // LDI 1    (pc = 2)
            0x3E, // SUB 14   (pc = 3)
            0x86, // JZ 6     (pc = 4)
            0x5F, // LDI 15   (pc = 5)
            0x59, // LDI 9    (pc = 6)
        ];

        {
            emulator.load_program(&sample);
            assert_eq!(emulator.pc, 0);
            // LDI 1
            emulator.execute_instruction();
            assert_eq!(emulator.pc, 1);
            // STA 14
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.pc, 2);
            // LDI 1
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.pc, 3);
            // SUB 14
            emulator.fetch_instruction();
            emulator.execute_instruction();
            assert_eq!(emulator.pc, 4);

            // JZ 6
            // since 1 - 1 equals 0, the vm should set the program counter to 6
            emulator.fetch_instruction();
            emulator.execute_instruction();

            assert_eq!(emulator.pc, 6);

            // LDI 9
            emulator.fetch_instruction();
            emulator.execute_instruction();

            assert_eq!(emulator.ra, 9);
        }
    }
}
