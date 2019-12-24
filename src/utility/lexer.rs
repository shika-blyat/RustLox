use crate::utility::tokens::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Self {
        Self { token_type, lexeme }
    }
    pub fn as_type(&self) -> TokenType {
        self.token_type.clone()
    }
    pub fn as_string(&self) -> String {
        self.lexeme.clone()
    }
    pub fn read_errors(tokens: Vec<Token>) -> Option<Vec<String>> {
        let mut errors = vec![];
        for i in tokens {
            match i.as_type() {
                TokenType::Error(error) => errors.push(error),
                _ => return None,
            }
        }
        Some(errors)
    }
}

pub struct Lexer {
    code: String,
    current: usize,
    start: usize,
    result: Vec<Token>,
    error: (bool, Vec<String>),
}

impl Lexer {
    pub fn new<'a>(code: &'a str) -> Self {
        let code = code.to_owned();
        Self {
            code,
            current: 0,
            start: 0,
            result: vec![],
            error: (false, vec![]),
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.is_empty() {
            self.start = self.current;
            self.scan_token();
        }
        if self.error.0 {
            let mut errors = vec![];
            for i in self.error.1.iter() {
                errors.push(Token::new(TokenType::Error(i.to_owned()), String::new()))
            }
            return errors;
        }
        self.add_token(TokenType::Eof, "");
        self.result.clone()
    }
    fn is_empty(&self) -> bool {
        self.current >= self.code.len()
    }
    fn scan_token(&mut self) {
        let mut c = self.advance();
        if c.is_ascii_digit() {
            let mut number = String::new();
            let mut is_float = false;
            loop {
                if self.is_empty() {
                    number.push(self.previous());
                    break;
                }
                if c == ' ' {
                    break;
                } else if c == '.' {
                    if is_float {
                        self.error.0 = true;
                        self.error.1.push(format!(
                            "Unexpected token `.` at char number {}",
                            self.current
                        ));
                    } else {
                        is_float = true;
                        number.push('.');
                    }
                } else if c.is_ascii_digit() {
                    number.push(c);
                } else {
                    self.error.0 = true;
                    self.error.1.push(format!(
                        "Unexpected token `{}` at char number {}",
                        c, self.current
                    ));
                }
                c = self.advance();
            }
            if is_float {
                self.add_token(
                    TokenType::Float(
                        number
                            .parse()
                            .expect("Internal Runtime Error during float parsing"),
                    ),
                    number.clone().as_str(),
                )
            } else {
                self.add_token(
                    TokenType::Int(
                        number
                            .parse()
                            .expect("Internal Runtime Error during int parsing"),
                    ),
                    number.clone().as_str(),
                )
            }
            return;
        }
        match c {
            '=' => match self.peek() {
                '=' => {
                    self.advance();
                    self.add_token(TokenType::EqEq, "==")
                }
                _ => self.add_token(TokenType::Eq, "="),
            },
            '!' => match self.peek() {
                '=' => {
                    self.advance();
                    self.add_token(TokenType::NotEq, "!=")
                }
                _ => self.add_token(TokenType::Not, "!"),
            },
            '<' => match self.peek() {
                '=' => {
                    self.advance();
                    self.add_token(TokenType::LessEq, "<=")
                }
                _ => self.add_token(TokenType::Less, "<"),
            },
            '>' => match self.peek() {
                '=' => {
                    self.advance();
                    self.add_token(TokenType::GreaterEq, ">=")
                }
                _ => self.add_token(TokenType::Greater, ">"),
            },
            '(' => self.add_token(TokenType::LeftParen, "("),
            ')' => self.add_token(TokenType::RightParen, ")"),
            '*' => self.add_token(TokenType::Star, "*"),
            '+' => self.add_token(TokenType::Plus, "+"),
            '-' => self.add_token(TokenType::Minus, "-"),
            '/' => self.add_token(TokenType::Slash, "/"),
            't' => self.add_token(TokenType::True, "t"),
            'f' => self.add_token(TokenType::False, "f"),
            _ => (),
        }
    }
    fn peek(&mut self) -> char {
        if self.is_empty() {
            return '\0';
        }
        self.code
            .chars()
            .nth(self.current)
            .expect("Internal runtime error: Failed to peek next token")
    }
    fn add_token<'a>(&mut self, token_type: TokenType, lexeme: &'a str) {
        self.result.push(Token::new(token_type, lexeme.to_owned()));
    }
    fn add_token_string(&mut self, token_type: TokenType, lexeme: String) {
        self.result.push(Token::new(token_type, lexeme));
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.code
            .chars()
            .nth(self.current - 1)
            .expect("Internal runtime error: Failed to advance to next token")
    }
    pub fn previous(&self) -> char {
        self.code
            .chars()
            .nth(self.current - 1)
            .expect("Internal runtime error: Failed to advance to next token")
    }
}
