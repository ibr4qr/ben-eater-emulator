use regex::Regex;



pub struct Lexer {
    counter: usize,
    back_track: usize,
    string_length: usize,
}


impl Lexer {
    pub fn scan(&mut self, code: &str) -> Vec<String> {
        let chars: Vec<_> = code.chars().collect();
        self.string_length = chars.len();
        let mut tokens: Vec<String> = vec![];


        while self.counter < self.string_length {
            let char = chars[self.counter];
            let number_literal_regex = Regex::new("^[0-9]").unwrap();
            let is_alpha =  Regex::new("[A-Za-z]").unwrap();

             if 
                char == '+' || char == '-' || 
                char == '(' || char == ')' ||
                char == '{' || char == '}' || 
                char == '=' || char == '<' ||
                char == '>'
            {
            } else if number_literal_regex.is_match(&char.to_string()) {
                self.back_track = self.counter + 1;
                while self.back_track < self.string_length && number_literal_regex.is_match(&chars[self.back_track].to_string()) {
                    self.back_track = self.back_track + 1;
                }
            } else if is_alpha.is_match(&char.to_string()) {
                self.back_track = self.counter + 1;
                while self.back_track < self.string_length && is_alpha.is_match(&chars[self.back_track].to_string()) {
                    self.back_track = self.back_track + 1;
                }
            }


            let token = String::from_iter(&chars[self.counter..self.back_track]);
            if !token.eq(" ") {
                tokens.push(token);
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
        string_length: 0
    }
}