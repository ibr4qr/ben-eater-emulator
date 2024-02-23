
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::u8;


pub struct Parser {
    // mut binary_code: Vec<u8> = vec![];
    // binary_code: Vec<u8>
}

impl Parser {
     pub fn get_line(&self, line: String) -> u8 {
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

     fn read_lines<P>(&self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
     }

     pub fn product_output_file(&self, byte_code: &mut Vec<u8>) -> std::io::Result<()> {

        self.get_lines(byte_code);

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


     fn get_lines(&self, binary_code: &mut Vec<u8>) {
        if let Ok(lines) = self.read_lines("./poem.txt") {
            for line in lines.flatten() {
                if !line.trim().is_empty() {
                    let instruction: u8 = self.get_line(line);
                    binary_code.push(instruction);
                }
            }
        }
     }
}

pub fn build_parser() -> Parser {
    Parser {}
}