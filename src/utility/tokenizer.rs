use std::ops::Range;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Token{
    token_type: TokenType,
    line: usize,
    nbr_char: usize,
}
#[derive(Debug,Clone)]
#[allow(dead_code)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,

    //Error
    InvalidToken,
}

trait Slicing{
    fn sliced(&self, range: std::ops::Range<usize>) -> String;
}
impl Slicing for String{
    fn sliced(&self, range: std::ops::Range<usize>) -> String{
        let mut new_str = String::new();
        for i in range{
            new_str.push(self.chars().nth(i).unwrap());
        }
        new_str
    }
}
pub struct TokenList{
    pub program : String,
    pub token_list: Vec<Token>,
    line: usize,
    start:usize,
    current: usize,
    nbr_char: usize,
}

impl TokenList{
    pub fn new(program: &str) -> TokenList{
        TokenList{program: program.to_string(), token_list: vec![],line: 1, nbr_char:1, current:0, start:0}
    }
    pub fn tokenize(&mut self) -> &Vec<Token>{
        let mut last_char = '\0';
        let mut lookahead = false;
        let mut looked_char = '\0';
        let mut looked_index = 0;
        for (k,i) in self.program.chars().enumerate(){

            if i == '\n'{
                println!("n");
                lookahead = false;
                self.line+=1;
                self.nbr_char=0;
                continue;
            }
            if lookahead{
                if i == looked_char{
                    lookahead = false;
                    if looked_char == '"'{
                        self.token_list.pop();
                        self.token_list.push(Token{token_type: TokenType::String(String::from(
                            &self.program.sliced(Range{start: looked_index, end: k,}))),
                            line: self.line, nbr_char: self.nbr_char});
                    }
                    
                }
                continue;
            }
            match i{
                '(' => self.token_list.push(Token{token_type: TokenType::LeftParen, line: self.line, nbr_char: self.nbr_char}),
                ')' => self.token_list.push(Token{token_type: TokenType::RightParen, line: self.line, nbr_char: self.nbr_char}),
                '{' => self.token_list.push(Token{token_type: TokenType::LeftBrace, line: self.line, nbr_char: self.nbr_char}),
                '}' => self.token_list.push(Token{token_type: TokenType::RightBrace, line: self.line, nbr_char: self.nbr_char}),
                ',' => self.token_list.push(Token{token_type: TokenType::Comma, line: self.line, nbr_char: self.nbr_char}),
                ';' => self.token_list.push(Token{token_type: TokenType::Semicolon, line: self.line, nbr_char: self.nbr_char}),
                '.' => self.token_list.push(Token{token_type: TokenType::Dot, line: self.line, nbr_char: self.nbr_char}),
                '+' => self.token_list.push(Token{token_type: TokenType::Plus, line: self.line, nbr_char: self.nbr_char}),
                '-' => self.token_list.push(Token{token_type: TokenType::Minus, line: self.line, nbr_char: self.nbr_char}),
                '*' => self.token_list.push(Token{token_type: TokenType::Star, line: self.line, nbr_char: self.nbr_char}),
                '=' => if last_char == '=' {
                            self.token_list.pop();
                            self.nbr_char-=1;
                            self.token_list.push(Token{token_type: TokenType::EqualEqual,
                                line: self.line, nbr_char: self.nbr_char})
                        } else if last_char == '<' {
                            self.nbr_char-=1;
                            self.token_list.pop();
                            self.token_list.push(Token{token_type: TokenType::LessEqual, 
                                line: self.line, nbr_char: self.nbr_char})
                        } else if last_char == '>' {
                            self.nbr_char-=1;
                            self.token_list.pop();
                            self.token_list.push(Token{token_type: TokenType::GreaterEqual, 
                                line: self.line, nbr_char: self.nbr_char})
                        } else if last_char == '!' {
                            self.nbr_char-=1;
                            self.token_list.pop();
                            self.token_list.push(Token{token_type: TokenType::BangEqual, 
                                line: self.line, nbr_char: self.nbr_char})
                        } else {
                            self.token_list.push(Token{token_type: TokenType::Equal, 
                                line: self.line, nbr_char: self.nbr_char})
                        },
                '<' => self.token_list.push(Token{token_type: TokenType::Less, line: self.line, nbr_char: self.nbr_char}),
                '>' => self.token_list.push(Token{token_type: TokenType::Greater, line: self.line, nbr_char: self.nbr_char}),
                '!' => self.token_list.push(Token{token_type: TokenType::Bang, line: self.line, nbr_char: self.nbr_char}),
                '/' => if last_char == '/' {
                            self.token_list.pop();
                            lookahead = true;
                        } else{
                            self.token_list.push(Token{token_type: TokenType::Slash, line: self.line, nbr_char: self.nbr_char})
                        }
                '"' => {
                            self.token_list.pop();
                            lookahead = true;
                            looked_char = '"';
                            looked_index = k;
                        }
                '\n'|'\t'|'\r'|' ' => (),
                _ => self.token_list.push(Token{token_type: TokenType::InvalidToken, line: self.line, nbr_char: self.nbr_char}),
            }
            last_char = i;
            self.nbr_char+=1;
        }
        self.token_list.push(Token{token_type: TokenType::Eof, line: self.line, nbr_char:self.nbr_char});
        &self.token_list
    }
}
