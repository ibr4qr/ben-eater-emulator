#[cfg(test)]
mod tests {
    use assembler::parser::build_parser;

    #[test]
    fn parsing_correctly() {
        let instructions = [
            "NOP", "LDA 1", "ADD 2", "SUB 3", "STA 1", "LDI 5", "JMP 4", "JC 4", "JZ 5", "OUT",
            "HALT",
        ]
        .map(|s| s.to_string())
        .to_vec();

        let mut parser = build_parser();
        parser.parse(instructions);

        assert_eq!(parser.binary_code[0], 0x0);
        assert_eq!(parser.binary_code[1], 0x11);
        assert_eq!(parser.binary_code[2], 0x22);
        assert_eq!(parser.binary_code[3], 0x33);
        assert_eq!(parser.binary_code[4], 0x41);
        assert_eq!(parser.binary_code[5], 0x55);
        assert_eq!(parser.binary_code[6], 0x64);
        assert_eq!(parser.binary_code[7], 0x74);
        assert_eq!(parser.binary_code[8], 0x85);
        assert_eq!(parser.binary_code[9], 0xE0);
        assert_eq!(parser.binary_code[10], 0xF0);
    }
}
