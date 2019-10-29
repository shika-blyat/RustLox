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
        println!("{}", c);
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
            '!' => if self.match_next_char('='){
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            '=' => if self.match_next_char('='){
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            '<' => if self.match_next_char('='){
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            '>' => if self.match_next_char('='){
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            _ => self.add_token(TokenType::InvalidToken)
        }
    }
    fn match_next_char(&mut self, c: char) -> bool {
        if !self.is_full(){
            return false
        } else if self.source.chars().nth(self.current).unwrap() != c {
            return false
        }
        self.current+=1;
        true
    }
    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        if let TokenType::Eof = token_type {
            self.tokens
                .push(Token::new(token_type, String::new(), self.line));
        } else {
            self.tokens.push(Token::new(token_type, text, self.line));
        }
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }
}
