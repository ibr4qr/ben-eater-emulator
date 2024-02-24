
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum TokenType {
    /**
     * single character token
     */

    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, SemiColon,Slash, Star,


    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,


    Identifier, String, Number,

    And, Class, Else, False, Fun, For, If, Nil, Or, 
    Print, Return, Super, This, True, Var, While,

    Eof
}
#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
}


impl Token {

}

pub fn build_token(token_type: TokenType, lexeme: String) -> Token {
    Token {
        token_type: token_type,
        lexeme: lexeme
    }
}