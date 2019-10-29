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
        while !self.is_finished() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::Eof, "");
        println!("{:#?}", self.tokens);
    }
    fn is_finished(&self) -> bool {
        self.current <= self.source.len()
    }
    fn scan_token(&self) {
        
    }
    fn add_token(&mut self, token_type: TokenType, lexeme: &str) {
        self.tokens
            .push(Token::new(token_type, lexeme.to_string(), self.line));
    }
}
