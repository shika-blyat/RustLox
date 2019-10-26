#[derive(Debug)]
#[allow(dead_code)]
pub struct Token{
    token_type: TokenType,
    line: usize,
    nbr_char: usize,
}
#[derive(Debug)]
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
    Identifier,
    String,
    Number,

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
        let mut looked_token: TokenType = TokenType::Eof;
        for (_k,i) in self.program.chars().enumerate(){
            if lookahead{
                if i == looked_char{
                    lookahead = false;
                    self.token_list.push(Token{token_type: TokenType::LeftParen, line: self.line, nbr_char: self.nbr_char})
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
                            self.token_list.push(Token{token_type: TokenType::EqualEqual, line: self.line, nbr_char: self.nbr_char})
                        } else if last_char == '<' {
                            self.token_list.pop();
                            self.token_list.push(Token{token_type: TokenType::LessEqual, line: self.line, nbr_char: self.nbr_char})
                        } else if last_char == '>' {
                            self.token_list.pop();
                            self.token_list.push(Token{token_type: TokenType::GreaterEqual, line: self.line, nbr_char: self.nbr_char})
                        } else if last_char == '!' {
                            self.token_list.pop();
                            self.token_list.push(Token{token_type: TokenType::BangEqual, line: self.line, nbr_char: self.nbr_char})
                        } else {
                            self.token_list.push(Token{token_type: TokenType::Equal, line: self.line, nbr_char: self.nbr_char})
                        },
                '<' => self.token_list.push(Token{token_type: TokenType::Less, line: self.line, nbr_char: self.nbr_char}),
                '>' => self.token_list.push(Token{token_type: TokenType::Greater, line: self.line, nbr_char: self.nbr_char}),
                '!' => self.token_list.push(Token{token_type: TokenType::Bang, line: self.line, nbr_char: self.nbr_char}),
                _ => self.token_list.push(Token{token_type: TokenType::InvalidToken, line: self.line, nbr_char: self.nbr_char}),
            }
            last_char = i;
        }
        self.token_list.push(Token{token_type: TokenType::Eof, line:self.program.len(), nbr_char:self.nbr_char});
        &self.token_list
    }
}
