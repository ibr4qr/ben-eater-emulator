#[cfg(test)]
mod tests {
    use vm::rom::{build_rom, Rom};

    #[test]
    fn test_rom_component() {
        let mut memory: Rom = build_rom();
        memory.set_value(10, 20);
        assert_eq!(memory.load_value(10).unwrap(), &20);

        memory.set_value(10, 200);
        assert_ne!(memory.load_value(10).unwrap(), &20);

        assert_eq!(memory.load_value(10).unwrap(), &200);

        let mut another_rom: Rom = build_rom();

        let program = [
            0x51, // LDI 1
            0xE,  // OUT
            0x5F, // LDI 15
        ];

        let address_number = 2;
        let expected_value = program[2];

        another_rom.load_program(&program);

        let value = another_rom.load_value(address_number).unwrap();

        assert_eq!(value, &expected_value);
    }
}
