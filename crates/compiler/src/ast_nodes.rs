use crate::token::Token;


#[derive(Debug)]
pub enum Node {
    UnaryExpr {
        operator: Token,
        right: Box<Node>
    },
    BinaryExpr {
        operator: Token,
        right: Box<Node>,
        left: Box<Node>
    },
    Literal {
        value: u8
    },
    VarDecl {
        identifier: String,
        initializer: Box<Node>
    },
    PrintDecl {
        argument: Box<Node>
    },
    Nil
}

struct Position {
    row: usize,
    column: usize,
}

pub struct Statement {
    // declarations: 
}

pub struct Declaration {

}
