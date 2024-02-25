use std::ptr::null;

use crate::token::{Token, TokenType};
use crate::ast_nodes::{ Node, Statement};




pub struct Parser {
    program: Vec<Statement>,
    counter: usize,
    back_track: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn parse(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
        while !self.is_at_end() {
            let declaration = self.declaration();
            println!("declaration: {:?}", declaration);
        }
    }

    fn declaration(&mut self) -> Node {
        if self.match_token(TokenType::Var) {
            return self.var_declaration();
        } 
        return self.statement();
    }

    fn var_declaration(&mut self) -> Node {
        let identifier: Token = self.consume(TokenType::Identifier);
        let mut initializer = Node::Nil;

        if self.match_token(TokenType::Equal) {
            initializer = self.expression();
        }

        return Node::VarDecl { identifier: identifier.lexeme, initializer: Box::new(initializer)}
    }

    fn statement(&mut self) -> Node{
        return self.expression_statement();
    }

    fn expression_statement(&mut self) -> Node {
       return self.expression();
    }

    fn expression(&mut self) -> Node {
        let expr = self.assignment();
        return expr;
    }

    fn assignment(&mut self) -> Node {
        let expr: Node = self.or();
        return expr;
    }

    fn or(&mut self) -> Node {
        let mut expr: Node = self.and();

        // while self.match_token(TokenType::Or) {
        //     let operator: Token = self.previous().clone();
        //     let right: Node = self.and();
        //     expr = 
        // }
        return expr;
    }

    fn and(&mut self) -> Node {
        let expr: Node = self.equality();

        return expr;
    }

    fn equality(&mut self) -> Node {
        let expr: Node = self.comparison();

        return expr;
    }

    fn comparison(&mut self) -> Node {
        let expr: Node = self.term();
        return expr;
    }

    fn term(&mut self) -> Node {
        let mut expr: Node = self.factor();

        while self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let operator: Token = self.previous().clone();
            let right: Node = self.factor();
            expr = Node::BinaryExpr { operator: operator, right: Box::new(right), left: Box::new(expr)};
        }

        return expr;
    }

    fn factor(&mut self) -> Node {
        let expr: Node = self.unary();
        return expr;
    }

    fn unary(&mut self) -> Node {
        if self.match_token(TokenType::Minus) {
            let operator: Token = self.previous().clone();
            let right: Node = self.unary();

          return Node::UnaryExpr { operator: operator, right:Box::new(right)}
        }
 
        return self.call();
    }

    fn call(&mut self) -> Node {
        let expr: Node = self.primary();
        return expr;
    }

    fn primary(&mut self) -> Node {
        if self.match_token(TokenType::Number) {
            return Node::Literal { value: self.previous().lexeme.parse::<u8>().unwrap() }
        }

        return Node::Nil;
    }






    // control
    fn is_at_end(&self) -> bool {
        return self.counter == self.tokens.len() - 1
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }

        return false;
    }

    fn advance(&mut self) -> Token {
    //    if !self.is_at_end()  {
    //         self.counter = self.counter + 1;
    //         return true;
    //    }

    //     return false;


        if !self.is_at_end() {
            self.counter = self.counter + 1;
        }


        return self.previous().clone();
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.counter - 1];
    }

   fn check(&self, token_type: TokenType) -> bool {
    if self.is_at_end() {
        return false;
    }
    return self.peek().token_type == token_type;
   }

   fn peek(&self) -> &Token {
    return &self.tokens[self.counter];
   }

   fn consume(&mut self, token_type: TokenType) -> Token {
    if self.check(token_type)  {
        return self.advance();
    }

     panic!("something wrong");
   }

}


pub fn build_parser() -> Parser {
    Parser {
        program: vec![],
        counter: 0,
        back_track: 1,
        tokens: vec![]
    }
}