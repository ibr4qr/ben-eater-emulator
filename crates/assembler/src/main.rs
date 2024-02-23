use std::fs::File;
use std::io::{self, BufRead, Read, Write};
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

fn getLine(line: String) -> u8 {
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
    let mut instruction = rawInstruction << 4;


    final_instruction = (rawInstruction << 4) & 0b11110000;
    final_instruction = final_instruction | argument;


    return final_instruction;
}

fn main() {
    // let mut binary_program = [];
    let mut binary_code: Vec<u8> = vec![];
    // --snip--
    if let Ok(lines) = read_lines("./poem.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if !line.trim().is_empty() {
                let instruction: u8 = getLine(line);
                binary_code.push(instruction);
            }
        }
    }

        product_file(&binary_code);
        // Write a slice of bytes to the file
        
       // file.write_all(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
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


fn product_file(byte_code: &Vec<u8>) -> std::io::Result<()> {
    {
        let mut file = File::create("test")?;
        // Write a slice of bytes to the file
        file.write_all(byte_code)?;
    }

    {
        let mut file = File::open("test")?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;
        println!("assembler produced: {:?}", buffer);
    }

    Ok(())
}