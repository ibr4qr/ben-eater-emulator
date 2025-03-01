use std::fs::File;
use std::io::{self, Read};
use vm::emulator::{build_emulator, Emulator};

fn main() {
    // TODO: produce sample file like this
    // The following put the a binary program to output.bin
    // The binary program count from 10 to 1
    //  printf '\x51\x4F\x5A\x0\x3F\x73' > output.bin
    if let Ok(binary_code) = get_binary_program() {
        let mut emulator: Emulator = build_emulator();
        emulator.load_program(&binary_code);
        emulator.execute_program();
    } else {
        println!("Error while trying to open binary file.");
    }
}

fn get_binary_program() -> io::Result<Vec<u8>> {
    let mut file = File::open("./output.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}
