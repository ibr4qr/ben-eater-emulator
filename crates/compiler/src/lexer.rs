use crate::token::{build_token, Token, TokenType};
use regex::Regex;

pub struct Lexer {
    counter: usize,
    back_track: usize,
    string_length: usize,
}

impl Lexer {
    pub fn scan(&mut self, code: &str) ->  Vec<Token> {
        let chars: Vec<_> = code.chars().collect();
        self.string_length = chars.len();
        let mut tokens: Vec<String> = vec![];
        let mut another_tokens: Vec<Token> = vec![];

        while self.counter < self.string_length {
            let char = chars[self.counter];
            let number_literal_regex = Regex::new("^[0-9]").unwrap();
            let is_alpha = Regex::new("[A-Za-z]").unwrap();

            if (char == '+') {
                let lexeme = String::from_iter(&chars[self.counter..self.back_track]);
                let token = build_token(TokenType::Plus, lexeme);
                another_tokens.push(token);
            } else if char == '-' {
                let lexeme = String::from_iter(&chars[self.counter..self.back_track]);
                let token = build_token(TokenType::Minus, lexeme);
                another_tokens.push(token);
            } else if char == '(' {
                let lexeme = String::from_iter(&chars[self.counter..self.back_track]);
                let token = build_token(TokenType::LeftParen, lexeme);
                another_tokens.push(token);
            } else if char == ')' {
                let lexeme = String::from_iter(&chars[self.counter..self.back_track]);
                let token = build_token(TokenType::RightParen, lexeme);
                another_tokens.push(token);
            } else if char == '{' {
                let lexeme = String::from_iter(&chars[self.counter..self.back_track]);
                let token = build_token(TokenType::LeftBrace, lexeme);
                another_tokens.push(token);
            } else if char == '}' {
                let lexeme = String::from_iter(&chars[self.counter..self.back_track]);
                let token = build_token(TokenType::RightBrace, lexeme);
                another_tokens.push(token);
            } else if char == '=' {
                let lexeme = String::from_iter(&chars[self.counter..self.back_track]);
                let token = build_token(TokenType::Equal, lexeme);
                another_tokens.push(token);
            } else if char == '<' {
                let lexeme = String::from_iter(&chars[self.counter..self.back_track]);
                let token = build_token(TokenType::Less, lexeme);
                another_tokens.push(token);
            } else if char == '>' {
                let lexeme = String::from_iter(&chars[self.counter..self.back_track]);
                let token = build_token(TokenType::Greater, lexeme);
                another_tokens.push(token);
            } else if number_literal_regex.is_match(&char.to_string()) {
                self.back_track = self.counter + 1;
                while self.back_track < self.string_length
                    && number_literal_regex.is_match(&chars[self.back_track].to_string())
                {
                    self.back_track = self.back_track + 1;
                }
                let lexeme = String::from_iter(&chars[self.counter..self.back_track]);
                let token = build_token(TokenType::Number, lexeme);
                another_tokens.push(token);
            } else if is_alpha.is_match(&char.to_string()) {
                self.back_track = self.counter + 1;
                while self.back_track < self.string_length
                    && is_alpha.is_match(&chars[self.back_track].to_string())
                {
                    self.back_track = self.back_track + 1;
                }

                // check if it's a keyword
                let lexeme: String = String::from_iter(&chars[self.counter..self.back_track]);
               
                if lexeme == "var" {
                    let token = build_token(TokenType::Var, lexeme);
                    another_tokens.push(token);
                } else {
                    let token = build_token(TokenType::Identifier, lexeme);
                    another_tokens.push(token);
                }
            }



            let token = String::from_iter(&chars[self.counter..self.back_track]);
            if !token.eq(" ") {
                tokens.push(token);
            }
            self.go_ahead();
        }

        return another_tokens;
    }

    fn go_ahead(&mut self) {
        self.counter = self.back_track;
        self.back_track = self.counter + 1;
    }
}

pub fn build_lexer() -> Lexer {
    Lexer {
        back_track: 0,
        counter: 0,
        string_length: 0,
    }
}
