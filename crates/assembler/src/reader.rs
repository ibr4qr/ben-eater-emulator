use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Reader {}

impl Reader {
    fn read_file<P>(&self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn get_lines(&self, filename: &String) -> Vec<String> {
        let mut instructions: Vec<String> = Vec::new();

        if let Ok(lines) = self.read_file(filename) {
            for line in lines.flatten() {
                if !line.trim().is_empty() {
                    instructions.push(line);
                }
            }
        }

        return instructions;
    }
}

pub fn build_reader() -> Reader {
    Reader {}
}
