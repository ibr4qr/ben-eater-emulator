pub mod lexer;
pub mod parser;
pub mod token;

use lexer::{build_lexer, Lexer};
use parser::{build_parser, Parser};


#[warn(dead_code)]
pub struct Compiler<'a> {
    code: &'a str,
    lexer: Lexer,
    parser: Parser
}

impl Compiler<'_> {
    pub fn compile(&mut self) {
       let tokens: Vec<String> =  self.lexer.scan(self.code);
       self.parser.parse(tokens);
    }
}

pub fn build_compiler(code: &str) -> Compiler {
    Compiler {
        code: code,
        lexer: build_lexer(),
        parser: build_parser()
    }
}