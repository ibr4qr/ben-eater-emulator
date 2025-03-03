pub mod ast_nodes;
pub mod code_gen;
pub mod lexer;
pub mod memory_manager;
pub mod parser;
pub mod token;

use ast_nodes::Node;
use code_gen::build_code_gen;
use lexer::{build_lexer, Lexer};
use parser::{build_parser, Parser};
use token::Token;

pub struct Compiler<'a> {
    code: &'a str,
    lexer: Lexer,
    parser: Parser,
    pub assembly_code: Vec<String>,
}

impl Compiler<'_> {
    pub fn compile(&mut self) {
        let tokens: Vec<Token> = self.lexer.scan(self.code);
        let ast: &Vec<Node> = self.parser.parse(tokens);
        let mut code_generator = build_code_gen(ast.to_vec());
        self.assembly_code = code_generator.gen_code();
    }
}

pub fn build_compiler(code: &str) -> Compiler {
    Compiler {
        code: code,
        lexer: build_lexer(),
        parser: build_parser(),
        assembly_code: Vec::new(),
    }
}
