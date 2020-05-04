use std::{iter::Peekable, ops::RangeInclusive, str::Chars};

use crate::{
    errors::{LexError, LexErrorKind},
    tokens::{Token, TokenKind},
    utils::Pos,
};

pub type LexResult<'a, T> = Result<T, LexError<'a>>;
pub struct Lexer<'a> {
    pos: Pos,
    input_s: &'a str,
    chars: Peekable<Chars<'a>>,
    pub tokens: Vec<Token<'a>>,
    stream_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            pos: Pos::default(),
            chars: s.chars().peekable(),
            input_s: s,
            stream_pos: 0,
            tokens: vec![],
        }
    }
    pub fn tokenize(mut self) -> LexResult<'a, Vec<Token<'a>>> {
        while let Some(c) = self.next() {
            if c.is_ascii_digit() {
                self.num();
                continue;
            } else if c.is_ascii_alphabetic() {
                self.ident();
                continue;
            } else if c.is_ascii_whitespace() && c != '\n' {
                continue;
            }
            match c {
                '(' => self.token(TokenKind::LParen, self.stream_pos..=self.stream_pos),
                ')' => self.token(TokenKind::RParen, self.stream_pos..=self.stream_pos),
                '{' => self.token(TokenKind::LBracket, self.stream_pos..=self.stream_pos),
                '}' => self.token(TokenKind::RBracket, self.stream_pos..=self.stream_pos),
                '+' => self.token(TokenKind::Add, self.stream_pos..=self.stream_pos),
                '-' => self.token(TokenKind::Minus, self.stream_pos..=self.stream_pos),
                '*' => self.token(TokenKind::Mul, self.stream_pos..=self.stream_pos),
                '/' => self.token(TokenKind::Div, self.stream_pos..=self.stream_pos),
                ':' => self.token(TokenKind::Colon, self.stream_pos..=self.stream_pos),
                ';' => self.token(TokenKind::Semicolon, self.stream_pos..=self.stream_pos),
                ',' => self.token(TokenKind::Semicolon, self.stream_pos..=self.stream_pos),
                '!' => {
                    if let Some('=') = self.peek() {
                        self.next();
                        self.token(TokenKind::BangEq, self.stream_pos..=self.stream_pos + 1)
                    } else {
                        self.token(TokenKind::Bang, self.stream_pos..=self.stream_pos)
                    }
                }
                '=' => {
                    if let Some('=') = self.peek() {
                        self.next();
                        self.token(TokenKind::EqEq, self.stream_pos..=self.stream_pos + 1)
                    } else {
                        self.token(TokenKind::Eq, self.stream_pos..=self.stream_pos)
                    }
                }
                '\n' => self.pos.newline(),
                c => {
                    return Err(LexError {
                        kind: LexErrorKind::UnexpectedChar(c),
                        pos: self.pos.into_ranged(self.stream_pos..=self.stream_pos),
                        code: self.input_s,
                    })
                }
            }
        }
        Ok(self.tokens)
    }
    fn num(&mut self) {
        let start = self.stream_pos - 1;
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_digit() {
                self.next();
            } else {
                break;
            }
        }
        let kind = TokenKind::Num(
            self.input_s[start..self.stream_pos]
                .parse()
                .expect("ICE: Failed to parse num"),
        );
        self.token(kind, start..=self.stream_pos - 1);
    }
    fn ident(&mut self) {
        let start = self.stream_pos - 1;
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() {
                self.next();
            } else {
                break;
            }
        }
        let kind = match &self.input_s[start..self.stream_pos] {
            "fn" => TokenKind::Fn,
            "return" => TokenKind::Return,
            "True" => TokenKind::Bool(true),
            "False" => TokenKind::Bool(false),
            s => TokenKind::Ident(s),
        };
        self.token(kind, start..=self.stream_pos - 1);
    }
    fn token(&mut self, kind: TokenKind<'a>, range: RangeInclusive<usize>) {
        self.tokens.push(Token {
            kind,
            pos: self.pos.into_ranged(range),
        })
    }
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
    fn next(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.stream_pos += 1;
                Some(c)
            }
            None => None,
        }
    }
}
