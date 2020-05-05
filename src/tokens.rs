use crate::utils::RangedPos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind<'a> {
    Num(isize),
    Bool(bool),
    Ident(&'a str),
    Add,
    Minus,
    Mul,
    Div,
    Eof,
    Semicolon,
    Colon,
    Comma,
    Return,
    Fn,
    Bang,
    Eq,
    BangEq,
    EqEq,
    LBracket,
    RBracket,
    LParen,
    RParen,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub pos: RangedPos,
    pub kind: TokenKind<'a>,
}

#[derive(Debug)]
pub struct TokensIterator<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}
impl<'a> TokensIterator<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, current: 0 }
    }
}
impl<'a> TokensIterator<'a> {
    pub fn peek(&mut self) -> Option<(TokenKind<'a>, RangedPos)> {
        if self.tokens.len() > self.current + 1 {
            return Some((
                self.tokens[self.current + 1].kind.clone(),
                self.tokens[self.current + 1].pos.clone(),
            ));
        }
        None
    }
    pub fn advance(&mut self) {
        self.current += 1;
    }
}
impl<'a> Iterator for TokensIterator<'a> {
    type Item = (TokenKind<'a>, RangedPos);
    fn next(&mut self) -> Option<Self::Item> {
        if self.tokens.len() > self.current {
            let val = (
                self.tokens[self.current].kind.clone(),
                self.tokens[self.current].pos.clone(),
            );
            self.current += 1;
            return Some(val);
        }
        None
    }
}
