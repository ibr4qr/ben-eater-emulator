




struct Position {
    row: usize,
    column: usize,
}

struct AstNode {
    position: Position,
}

pub struct Parser {
    ast_nodes: Vec<AstNode>,
    counter: usize,
    back_track: usize,
}


impl Parser {
    pub fn parse(&mut self, tokens: Vec<String>) {

        for token in &tokens {
            println!("current token: {}", token)
        }   

        println!("parser will use this tokens {:?} to produce an AST", tokens);
    }
}


pub fn build_parser() -> Parser {
    Parser {
        ast_nodes: vec![],
        counter: 0,
        back_track: 1
    }
}