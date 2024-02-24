use crate::token::{Token, TokenType};






struct Position {
    row: usize,
    column: usize,
}

struct AstNode {
    position: Position,
}

pub struct Parser {
    program: Vec<Statement>,
    counter: usize,
    back_track: usize,
    tokens: Vec<Token>
}

pub struct Statement {
    // declarations: 
}

pub struct Declaration {

}


// TODO: build a consume function that consumes tokens and produces ast nodes
// TODO: build a match function that iterate though tokens until it reaches the expected token without consuming
// TODO: build a check function
// TODO: build an advance function
// TODO: build a peek function

impl Parser {
    pub fn parse(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
        while !self.is_at_end() {
            // let stmt = self.declaration();
            // let current_token = self.peek();
            // println!("current token: {:?}", current_token);
            // self.advance();
            self.declaration();
        }


    }

    fn declaration(&mut self) {
        if self.match_token(TokenType::Var) {
            println!("we have a var declaration");
        }
    }

    fn is_at_end(&self) -> bool {
        return self.counter == self.tokens.len() - 1
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        for token in &self.tokens {
           if self.check(token_type) {
            self.advance();
            return true;
           }
        }

        return false;
    }


    fn advance(&mut self) -> bool {
       if !self.is_at_end()  {
            self.counter = self.counter + 1;
            return true;
       }

        return false;
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.counter - 1];
    }
   

   fn check(&self, tokenType: TokenType) -> bool {
    if self.is_at_end() {
        return false;
    }

    // return self.peek().
    return true;
   }


   fn peek(&self) -> &Token {
    return &self.tokens[self.counter];
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