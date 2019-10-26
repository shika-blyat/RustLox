use std::error::Error;
use std::fmt;

#[derive(Debug)]
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

pub fn tokenize(program: &str)-> Result<Vec<TokenType>, Vec<InvalidToken>>  {

    let mut line = 1;
    let mut nbr_char = 0;
    let mut token_list = vec![];
    let mut errors = vec![];
    let mut lookahead = false;
    let mut looked_char = '0'; // Don't blame me for this name :(

    fn look_to(looked: char){
        |mut looked_char| looked_char = looked;
        |mut lookahead| lookahead = true;
    }

    for (k,i) in program.chars().enumerate(){
        nbr_char+=1;
        if let '\n' = i{
            line += 1 ;
            nbr_char = 0;
            lookahead = false;
        }
        if lookahead && i == looked_char {
            lookahead = false;
        }
        if !lookahead{
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
            '=' => match program.chars().nth(k+1) {
                    Some('=') => token_list.push(TokenType::EqualEqual{line, nbr_char}),
                    _ => token_list.push(TokenType::Equal{line, nbr_char}),
                },
            '>' => match program.chars().nth(k+1) {
                    Some('=') => token_list.push(TokenType::GreaterEqual{line, nbr_char}),
                    _ => token_list.push(TokenType::Greater{line, nbr_char}),
                },
            '<' => match program.chars().nth(k+1) {
                    Some('=') => token_list.push(TokenType::LessEqual{line, nbr_char}),
                    _ => token_list.push(TokenType::Less{line, nbr_char}),
                },
            '!' => match program.chars().nth(k+1) {
                    Some('=') => token_list.push(TokenType::BangEqual{line, nbr_char}),
                    _ => token_list.push(TokenType::Bang{line, nbr_char})
                },
            '/' => match program.chars().nth(k+1) {
                    Some('/') => look_to('/'),
                    _ => token_list.push(TokenType::Slash{line, nbr_char})
                },
            ' ' => (),
            '\n' => (),
            '\r' => (),
            '\t' => (),
            _   => errors.push(InvalidToken{line, nbr_char, token: i}),
        };
        }
    }
    token_list.push(TokenType::Eof{line, nbr_char: program.len() as u8});
    if errors.len() == 0 {Ok(token_list)} else {Err(errors)}
}
