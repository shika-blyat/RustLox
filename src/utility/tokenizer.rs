use std::error::Error;
use std::fmt;

pub enum TokenType {
    // Single-character tokens.
    LeftParen{line: u16, nbr_char: u8},
    RightParen{line: u16, nbr_char: u8},
    LeftBrace{line: u16, nbr_char: u8},
    RightBrace{line: u16, nbr_char: u8},
    Comma{line: u16, nbr_char: u8},
    Dot{line: u16, nbr_char: u8},
    Minus{line: u16, nbr_char: u8},
    Plus{line: u16, nbr_char: u8},
    Semicolon{line: u16, nbr_char: u8},
    Slash{line: u16, nbr_char: u8},
    Star{line: u16, nbr_char: u8},

    // One or two character tokens.
    Bang{line: u16, nbr_char: u8},
    BangEqual{line: u16, nbr_char: u8},
    Equal{line: u16, nbr_char: u8},
    EqualEqual{line: u16, nbr_char: u8},
    Greater{line: u16, nbr_char: u8},
    GreaterEqual{line: u16, nbr_char: u8},
    Less{line: u16, nbr_char: u8},
    LessEqual{line: u16, nbr_char: u8},

    // Literals.
    Identifier{line: u16, nbr_char: u8},
    String{line: u16, nbr_char: u8},
    Number{line: u16, nbr_char: u8},

    // Keywords.
    And{line: u16, nbr_char: u8},
    Class{line: u16, nbr_char: u8},
    Else{line: u16, nbr_char: u8},
    False{line: u16, nbr_char: u8},
    Fun{line: u16, nbr_char: u8},
    For{line: u16, nbr_char: u8},
    If{line: u16, nbr_char: u8},
    Nil{line: u16, nbr_char: u8},
    Or{line: u16, nbr_char: u8},
    Print{line: u16, nbr_char: u8},
    Return{line: u16, nbr_char: u8},
    Super{line: u16, nbr_char: u8},
    This{line: u16, nbr_char: u8},
    True{line: u16, nbr_char: u8},
    Var{line: u16, nbr_char: u8},
    While{line: u16, nbr_char: u8},
    Eof{line: u16, nbr_char: u8},
}

#[derive(Debug)]
pub struct InvalidToken{
    line: u16,
    nbr_char: u8,
    token: char,
}

impl fmt::Display for InvalidToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Invalid token")
    }
}

impl Error for InvalidToken {}

pub fn tokenize<T>(program: &str)-> Result<Vec<TokenType>, Vec<InvalidToken>>  {
    let mut start = 0;
    let mut current = 0;
    let mut line = 1;
    let mut nbr_char = 0;
    let mut token_list = vec![];
    let mut errors = vec![];
    for i in program.chars(){
        current+=1;
        match i{
            '(' => token_list.push(TokenType::LeftParen{line, nbr_char}),
            ')' => token_list.push(TokenType::RightParen{line, nbr_char}),
            '{' => token_list.push(TokenType::LeftBrace{line, nbr_char}),
            '}' => token_list.push(TokenType::RightBrace{line, nbr_char}),
            ',' => token_list.push(TokenType::Comma{line, nbr_char}),
            ';' => token_list.push(TokenType::Semicolon{line, nbr_char}),
            '.' => token_list.push(TokenType::Dot{line, nbr_char}),
            '+' => token_list.push(TokenType::Plus{line, nbr_char}),
            '-' => token_list.push(TokenType::Minus{line, nbr_char}),
            '*' => token_list.push(TokenType::Star{line, nbr_char}),
            _   => errors.push(InvalidToken{line, nbr_char, token: i}),
        };
    }
    token_list.push(TokenType::Eof{line, nbr_char: program.len() as u8});
    if errors.len() == 0 {Ok(token_list)} else {Err(errors)}
}
