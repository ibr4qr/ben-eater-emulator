use std::u8;
use parser::{Parser, build_parser};

pub mod parser;

fn main() {
    let parser: Parser = build_parser();
    let mut binary_code: Vec<u8> = vec![];
    parser.product_output_file(&mut binary_code);
}