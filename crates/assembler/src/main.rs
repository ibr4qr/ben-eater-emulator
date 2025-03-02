use assembler::{
    parser::{build_parser, Parser},
    reader::build_reader,
};
use std::env;
use std::fs::File;
use std::io::Write;
use vm::emulator::{build_emulator, Emulator};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "--help" {
        println!("Usage: binary --in <input-file> --out <output-file>");
        println!("");
        println!("--in  <input-file>    :   File from which assembler will read assemble code.");
        println!("--out <output-file>   :   File to which the assembler will write binary code.");
        println!(
            "                          When not set, the assembler will run the program on fly"
        );
        println!("                          using the vm module.");
        return;
    }

    if args.len() < 3 {
        println!("Usage: binary --in <input-file> --out <output-file>");
        return;
    }

    if let Some(input_flag_position) = args.iter().position(|arg| arg == "--in") {
        // if the output file is not provided then we will
        // execute the program using the VM
        let mut parser: Parser = build_parser();
        let filename = &args[input_flag_position + 1];

        let reader = build_reader();
        let instructions = reader.get_lines(filename);
        parser.parse(instructions);

        if let Some(output_flag_position) = args.iter().position(|arg| arg == "--out") {
            let filename = &args[output_flag_position + 1];

            // Write to a file
            let mut file = File::create(filename).expect("Failed to output file");
            file.write_all(&parser.binary_code)
                .expect("Failed to write to file");

            println!("Binary code written to {}", filename);
        } else {
            let mut emulator: Emulator = build_emulator();
            emulator.load_program(&parser.binary_code);
            emulator.execute_program();
        }
    } else {
        println!("Please provide a input file!")
    }
}
