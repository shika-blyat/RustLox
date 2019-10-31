use crate::lexer::tokens::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
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
        self.token_type
            .to_string()
            .push_str(&(" ".to_owned() + &self.lexeme + " "));
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
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while self.is_full() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::Eof);
        println!("{:?}", self.tokens);
        self.tokens.clone()
    }
    fn is_full(&self) -> bool {
        self.current < self.source.len()
    }
    fn scan_token(&mut self) {
        let c = self.advance();

        if c.is_digit(10) {
            self.get_number();
            return;
        } else if c.is_ascii_alphabetic() || c == '_' {
            self.get_identifier();
            return;
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
                    println!("a");
                    while self.is_full() && self.peek() != '\n' {
                        self.advance();
                    }
                } else if self.match_next_char('*') {
                    while !self.match_next_char('*') && self.peek_next() != '/' {
                        self.advance();
                    }
                    self.advance();
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.get_string(),
            '\r' | '\t' | ' ' => (),
            '\n' => self.line += 1,
            _ => self.add_token(TokenType::InvalidToken),
        }
    }
    fn get_identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        let name = self.source[self.start..self.current].to_string();
        match name.as_str() {
            "and" => self.add_token(TokenType::And),
            "class" => self.add_token(TokenType::Class),
            "else" => self.add_token(TokenType::Else),
            "false" => self.add_token(TokenType::False),
            "fun" => self.add_token(TokenType::Fun),
            "for" => self.add_token(TokenType::For),
            "if" => self.add_token(TokenType::If),
            "nil" => self.add_token(TokenType::Nil),
            "or" => self.add_token(TokenType::Or),
            "print" => self.add_token(TokenType::Print),
            "return" => self.add_token(TokenType::Return),
            "super" => self.add_token(TokenType::Super),
            "this" => self.add_token(TokenType::This),
            "true" => self.add_token(TokenType::True),
            "var" => self.add_token(TokenType::Var),
            "while" => self.add_token(TokenType::While),
            _ => self.add_token(TokenType::Identifier(name)),
        }
    }
    fn get_number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let number = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenType::Number(number));
    }
    fn get_string(&mut self) {
        while self.peek() != '"' && self.is_full() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if !self.is_full() {
            self.add_token(TokenType::InvalidToken);
            return;
        }
        self.advance(); // The closing '"'
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String(value));
    }
    fn match_next_char(&mut self, c: char) -> bool {
        if !self.is_full() {
            return false;
        } else if self.source.chars().nth(self.current).unwrap() != c {
            return false;
        }
        self.current += 1;
        true
    }
    fn peek(&mut self) -> char {
        if !self.is_full() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }
    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }
    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        match token_type {
            TokenType::Eof => {
                self.tokens
                    .push(Token::new(token_type, String::new(), self.line));
            }
            TokenType::String(_) => {
                let text = self.source[self.start + 1..self.current - 1].to_string();
                self.tokens.push(Token::new(token_type, text, self.line));
            }
            _ => self.tokens.push(Token::new(token_type, text, self.line)),
        }
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }
}
