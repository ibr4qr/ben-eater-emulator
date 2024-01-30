#[cfg(test)]
mod tests {
    use compiler::rom::{ build_rom, Rom};


    #[test]
    fn test_rom_component() {
        let mut memory: Rom = build_rom();
        memory.set_value(10,20);

        let value = memory.load_value(10).unwrap();
        
        assert_eq!(value, &20);
        
        memory.set_value(10,200);

        let another_value = memory.load_value(10).unwrap();
        assert_ne!(another_value, &20);

        assert_eq!(another_value, &200);


        let mut another_rom: Rom = build_rom();


        let program = [0, 10, 10, 20, 30];
        let address_number = 2;
        let expected_value = 10;

        
        another_rom.load_program(&program);

        let value = another_rom.load_value(address_number).unwrap();

        assert_eq!(value, &expected_value)
    }
}