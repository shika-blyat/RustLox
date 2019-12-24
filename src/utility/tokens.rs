#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Float(f64),
    Int(i128),
    Star,
    Plus,
    Minus,
    Slash,
    Eq,
    Not,
    EqEq,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Error(String),
    LeftParen,
    RightParen,
}
