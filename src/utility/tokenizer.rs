use crate::utility::tokens::TokenType;
#[derive(Debug)]
#[allow(dead_code)]

pub struct Token {
    token_type: TokenType,
    line: usize,
    nbr_char: usize,
}

pub struct TokenList {
    pub program: String,
    pub token_list: Vec<Token>,
    line: usize,
    nbr_char: usize,
}

impl TokenList {
    pub fn new(program: &str) -> TokenList {
        TokenList {
            program: program.to_string(),
            token_list: vec![],
            line: 1,
            nbr_char: 1,
        }
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        let mut last_char = '\0';
        let mut lookahead = false;
        let mut looked_char = '\0';
        let mut looked_index = 0;

        for (k, i) in self.program.chars().enumerate() {
            if i == '\n' {
                lookahead = false;
                self.line += 1;
                self.nbr_char = 0;
                continue;
            }
            println!("last_char:{}, i: {} ", last_char,i);
            if i.is_digit(10) || i == '.'{
                if last_char.is_digit(10) || last_char == '.' {
                    self.nbr_char += 1;
                } else {
                    looked_index = k;
                }
                last_char = i;
                continue;
            }  else if last_char.is_digit(10) {
                self.token_list.push(Token {
                    token_type: TokenType::Number(self.program[looked_index..k].parse::<f64>().unwrap()),
                    line: self.line,
                    nbr_char: self.nbr_char,
                });
            } 
            if lookahead {
                last_char = i;
                if i == looked_char {
                    lookahead = false;
                    if looked_char == '"' {
                        self.token_list.push(Token {
                            token_type: TokenType::String(String::from(&self.program[looked_index+1..k])),
                            line: self.line,
                            nbr_char: self.nbr_char,
                        });
                    }
                }
                continue;
            }
            match i {
                '(' => self.token_list.push(Token {
                    token_type: TokenType::LeftParen,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                ')' => self.token_list.push(Token {
                    token_type: TokenType::RightParen,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                '{' => self.token_list.push(Token {
                    token_type: TokenType::LeftBrace,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                '}' => self.token_list.push(Token {
                    token_type: TokenType::RightBrace,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                ',' => self.token_list.push(Token {
                    token_type: TokenType::Comma,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                ';' => self.token_list.push(Token {
                    token_type: TokenType::Semicolon,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                '.' => self.token_list.push(Token {
                    token_type: TokenType::Dot,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                '+' => self.token_list.push(Token {
                    token_type: TokenType::Plus,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                '-' => self.token_list.push(Token {
                    token_type: TokenType::Minus,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                '*' => self.token_list.push(Token {
                    token_type: TokenType::Star,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                '=' => {
                    if last_char == '=' {
                        self.token_list.pop();
                        self.nbr_char -= 1;
                        self.token_list.push(Token {
                            token_type: TokenType::EqualEqual,
                            line: self.line,
                            nbr_char: self.nbr_char,
                        })
                    } else if last_char == '<' {
                        self.nbr_char -= 1;
                        self.token_list.pop();
                        self.token_list.push(Token {
                            token_type: TokenType::LessEqual,
                            line: self.line,
                            nbr_char: self.nbr_char,
                        })
                    } else if last_char == '>' {
                        self.nbr_char -= 1;
                        self.token_list.pop();
                        self.token_list.push(Token {
                            token_type: TokenType::GreaterEqual,
                            line: self.line,
                            nbr_char: self.nbr_char,
                        })
                    } else if last_char == '!' {
                        self.nbr_char -= 1;
                        self.token_list.pop();
                        self.token_list.push(Token {
                            token_type: TokenType::BangEqual,
                            line: self.line,
                            nbr_char: self.nbr_char,
                        })
                    } else {
                        self.token_list.push(Token {
                            token_type: TokenType::Equal,
                            line: self.line,
                            nbr_char: self.nbr_char,
                        })
                    }
                }
                '<' => self.token_list.push(Token {
                    token_type: TokenType::Less,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                '>' => self.token_list.push(Token {
                    token_type: TokenType::Greater,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                '!' => self.token_list.push(Token {
                    token_type: TokenType::Bang,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
                '/' => {
                    if last_char == '/' {
                        self.token_list.pop();
                        lookahead = true;
                    } else {
                        self.token_list.push(Token {
                            token_type: TokenType::Slash,
                            line: self.line,
                            nbr_char: self.nbr_char,
                        })
                    }
                }
                '"' => {
                    lookahead = true;
                    looked_char = '"';
                    looked_index = k;
                }
                '\n' | '\t' | '\r' | ' ' => (),
                _ => self.token_list.push(Token {
                    token_type: TokenType::InvalidToken,
                    line: self.line,
                    nbr_char: self.nbr_char,
                }),
            }
            last_char = i;
            self.nbr_char += 1;
        }
        self.token_list.push(Token {
            token_type: TokenType::Eof,
            line: self.line,
            nbr_char: self.nbr_char,
        });
        &self.token_list
    }
}
