use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Node {
    UnaryExpr {
        operator: Token,
        right: Box<Node>,
    },
    BinaryExpr {
        operator: Token,
        right: Box<Node>,
        left: Box<Node>,
    },
    Literal {
        value: u8,
    },
    VarDecl {
        identifier: String,
        initializer: Box<Node>,
    },
    PrintDecl {
        argument: Box<Node>,
    },
    Nil,
}
#[allow(dead_code)]
struct Position {
    row: usize,
    column: usize,
}

pub struct Statement {
    // declarations:
}

pub struct Declaration {}
