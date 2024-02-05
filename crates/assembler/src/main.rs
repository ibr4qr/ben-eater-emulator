use std::env;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead};
use std::os::macos::raw;
use std::path::Path;
use std::u8;

// enum Instructions {
//     NOP = "NOP",
//     LDA = "LDA",
//     D = "ADD",
//     SUB = "SUB",
//     STA = "STA",
//     OUT = "OUT",
//     JMP = "JMP",
//     LDI = "LDI",
//     JC = "JC",
//     JZ = "JZ",
//     JNC = "JNC",
//     HALT = "HALT"
// }

fn getLine(line: String) {
    let parts = line.split(" ");
    let collection = parts.collect::<Vec<&str>>();
    let mut instruction = collection[0];
    let mut argument = 0;

    let mut rawInstruction = 0;

    match(instruction) {
        // "NOP" => {
        //     rawInstruction = 0;
        // },
        "LDA" => {
            rawInstruction = 1;
            argument = (collection[1]).parse().unwrap();
        },
        "ADD" => {
            rawInstruction = 2;
            argument = (collection[1]).parse().unwrap();
        },
        "SUB" => {
            rawInstruction = 3;
            argument = (collection[1]).parse().unwrap();
        },
        "STA" => {
            rawInstruction = 4;
            argument = (collection[1]).parse().unwrap();
        },
        "OUT" => {
            rawInstruction = 0;
        },
        "JMP" => {
           rawInstruction = 6;
           argument = (collection[1]).parse().unwrap();
        },
        "LDI" => {
           rawInstruction = 7;
           argument = (collection[1]).parse().unwrap();
        },
        "JC" => {
           rawInstruction = 9;
           argument = (collection[1]).parse().unwrap();
        },
        "JZ" => {
          rawInstruction = 8;
          argument = (collection[1]).parse().unwrap();
        },
        "JNC" => {
            println!("JNCC");
        },
        "HALT" => {
            println!("HALLTTTT");
        },
        _ => println!("it s something else"),
    }


    let mut final_instruction = 0;
    let mut final_argument = 0;
    final_instruction = rawInstruction << 4;
    // final_argument = argument & 00001111;

    final_instruction = final_instruction | argument;

    println!("instruction: {}, {}", final_instruction, final_argument);
    println!("final instruction: {}", final_instruction);
}

fn main() {
    // let mut binary_program = [];

    // --snip--
    if let Ok(lines) = read_lines("./poem.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if !line.trim().is_empty() {
                getLine(line);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
