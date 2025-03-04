use crate::token::{build_token, Token, TokenType};
use regex::Regex;

pub struct Lexer {
    counter: usize,
    back_track: usize,
}

impl Lexer {
    pub fn scan(&mut self, code: &str) -> Vec<Token> {
        let number_literal_regex = Regex::new("^[0-9]").unwrap();
        let is_alpha = Regex::new("[A-Za-z]").unwrap();

        let chars: Vec<_> = code.chars().collect();
        let string_length = chars.len();

        let mut tokens: Vec<Token> = vec![];

        while self.counter < string_length {
            let char = chars[self.counter];

            let token_type: Option<TokenType> = match char {
                ';' => Some(TokenType::SemiColon),
                '+' => Some(TokenType::Plus),
                '-' => Some(TokenType::Minus),
                '(' => Some(TokenType::LeftParen),
                ')' => Some(TokenType::RightParen),
                '{' => Some(TokenType::LeftBrace),
                '}' => Some(TokenType::RightBrace),
                '=' => Some(TokenType::Equal),
                '<' => Some(TokenType::Less),
                '>' => Some(TokenType::Greater),
                _ => {
                    // we are dealing with a numeric value
                    if number_literal_regex.is_match(&char.to_string()) {
                        self.back_track = self.counter + 1;
                        while self.back_track < string_length
                            && number_literal_regex.is_match(&chars[self.back_track].to_string())
                        {
                            self.back_track += 1;
                        }
                        Some(TokenType::Number)
                    } else if is_alpha.is_match(&char.to_string()) {
                        self.back_track = self.counter + 1;
                        while self.back_track < string_length
                            && is_alpha.is_match(&chars[self.back_track].to_string())
                        {
                            self.back_track += 1;
                        }
                        Some(
                            match String::from_iter(&chars[self.counter..self.back_track]).as_str()
                            {
                                "var" => TokenType::Var,
                                "print" => TokenType::Print,
                                _ => TokenType::Identifier,
                            },
                        )
                    } else {
                        None
                    }
                }
            };

            if let Some(token_type) = token_type {
                let lexeme = String::from_iter(&chars[self.counter..self.back_track]);
                tokens.push(build_token(token_type, lexeme));
            }

            self.go_ahead();
        }

        return tokens;
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
    }
}
