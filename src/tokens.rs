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
