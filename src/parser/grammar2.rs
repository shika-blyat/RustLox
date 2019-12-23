use crate::parser::lexer::Token;
use crate::parser::tokens::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Parenthesized {
    left_paren: Token,
    expr: Box<Expr>,
    right_paren: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {}

pub enum Primary {
    Literal(Literal),
    Parenthesized(Parenthesized),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    // It is supposed to be a number, a string, true, false or nil
    value: Token,
}
impl Literal {
    pub fn new(value: Token) -> Self {
        match value.as_type() {
            TokenType::Number(_)
            | TokenType::String(_)
            | TokenType::True
            | TokenType::False
            | TokenType::Nil => Self { value },
            _ => panic!("Literal cannot be built with a {} ", value.as_type()),
        }
    }
}

impl Parenthesized {
    pub fn new(left_paren: Token, expr: Expr, right_paren: Token) -> Self {
        match (&left_paren.as_type(), &right_paren.as_type()) {
            (TokenType::LeftParen, TokenType::RightParen) => {
                let expr = Box::new(expr);
                Self {
                    left_paren,
                    expr,
                    right_paren,
                }
            }
            _ => panic!("Grouping cannot be built without parenthesis"),
        }
    }
}

pub enum Unary {
    Primary(Primary),
    Unary(TokenType, Box<Unary>),
}

pub struct Multiplication {
    left_un: Unary,
    
}
