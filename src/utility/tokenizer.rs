use crate::utility::tokens::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    //literal: literal,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
    pub fn as_string(&self) {
        self.token_type.to_string().push_str(&(" ".to_owned() + &self.lexeme + " ")) /*+ self.literal*/;
    }
}

pub struct Scanner {
    tokens: Vec<Token>,
    pub source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            tokens: vec![],
            source: source.to_string(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) {
        while self.is_full() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::Eof);
        println!("{:#?}", self.tokens);
    }
    fn is_full(&self) -> bool {
        self.current < self.source.len()
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        if c.is_digit(10){
            self.get_number();
            return
        }
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            ';' => self.add_token(TokenType::Semicolon),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '!' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_next_char('/') {
                    while self.is_full() && self.peek() != '\n'
                    {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.get_string(),
            '\r' | '\t' | ' ' => (),
            '\n' => self.line+=1,
            _ => self.add_token(TokenType::InvalidToken),
        }
    }
    fn get_number(&mut self){
        while self.peek().is_digit(10){
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10){
            self.advance();
            while self.peek().is_digit(10){
                self.advance();
            } 
        }
        let number = self.source[self.start..self.current].parse::<f64>().unwrap();
        self.add_token(TokenType::Number(number));
    }
    fn get_string(&mut self){
        while self.peek() != '"' && self.is_full(){
            if self.peek() == '\n' {
                self.line+=1;
            }
            self.advance();
        }
        if !self.is_full(){
            self.add_token(TokenType::InvalidToken);
            return
        }
        self.advance(); // The closing '"'
        let value = self.source[self.start+1..self.current-1].to_string();
        self.add_token(TokenType::String(value));
    }
    fn match_next_char(&mut self, c: char) -> bool {
        if !self.is_full() {
            return false;
        } else if self.source.chars().nth(self.current-1).unwrap() != c {
            return false;
        }
        self.current += 1;
        true
    }
    fn peek(&self) -> char{
        if !self.is_full() {
            return '\0'
        }
        self.source.chars().nth(self.current).unwrap()
    }
    fn peek_next(&self) -> char{
        if self.current+1 >= self.source.len(){
            return '\0'
        }
        return self.source.chars().nth(self.current+1).unwrap();
    }
    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        match token_type{
            TokenType::Eof => {
                self.tokens
                    .push(Token::new(token_type, String::new(), self.line));
            },
            TokenType::String(_) => {
                let text = self.source[self.start+1..self.current-1].to_string();
                self.tokens.push(Token::new(token_type, text, self.line));
            },
            _ => self.tokens.push(Token::new(token_type, text, self.line)),
        }
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }
}
