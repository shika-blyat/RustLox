use crate::utility::lexer::Token;
use crate::utility::tokens::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Lit(Literal),
    Unary(PrimOp, Box<Expr>),
    Binary(Box<Expr>, PrimOp, Box<Expr>),
    Grouped(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimOp {
    Add,
    Sub,
    Mul,
    Div,
    GreaterEq,
    EqEq,
    NotEq,
    LessEq,
    Greater,
    Less,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i128),
    Float(f64),
    Bool(bool),
}
