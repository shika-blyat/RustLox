use crate::parser::lexer::Token;
use crate::parser::tokens::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(Binary),
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Operator(Operator),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primary {
    Token(Literal),
    Grouped(Grouping),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Simple {
    Primary(Primary),
    Unary(Unary),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    operator: Token,
    unary: Box<Simple>,
}
impl Unary {
    pub fn new(operator: Token, unary: Simple) -> Self {
        let unary = Box::new(unary);
        match operator.as_type() {
            TokenType::Bang | TokenType::Minus => Self { operator, unary },
            _ => panic!(
                "Unary cannot be built with {} as the operator",
                operator.as_type()
            ),
        }
    }
}

impl PrettyPrint for Unary {
    fn pretty_print(&self) -> String {
        parenthesize(
            self.operator.as_string(),
            Some(vec![self.expr_right.clone()]),
        )
    }
}

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

impl PrettyPrint for Expr {
    fn pretty_print(&self) -> String {
        match self {
            Expr::Binary(binary) => binary.pretty_print(),
            Expr::Literal(literal) => literal.pretty_print(),
            Expr::Grouping(grouping) => grouping.pretty_print(),
            Expr::Unary(unary) => unary.pretty_print(),
            Expr::Operator(operator) => operator.pretty_print(),
        }
    }
}

fn parenthesize(name: String, vec: Option<Vec<Expr>>) -> String {
    let mut value = String::from("(") + name.as_str();
    if let Some(vec) = vec {
        for i in vec {
            value.push(' ');
            value.push_str(i.pretty_print().as_str());
        }
    }
    value.push(')');
    value
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
impl PrettyPrint for Literal {
    fn pretty_print(&self) -> String {
        self.value.as_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    left_paren: Token,
    expr: Box<Expr>,
    right_paren: Token,
}

impl Grouping {
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

impl PrettyPrint for Grouping {
    fn pretty_print(&self) -> String {
        parenthesize("(".to_owned(), Some(vec![*(self.expr.clone())])) + " )"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    expr_left: Box<Expr>,
    operator: Operator,
    expr_right: Box<Expr>,
}
impl Binary {
    pub fn new(expr_left: Expr, operator: Operator, expr_right: Expr) -> Self {
        let (expr_left, expr_right) = (Box::new(expr_left), Box::new(expr_right));
        Self {
            expr_left,
            operator,
            expr_right,
        }
    }
}

impl PrettyPrint for Binary {
    fn pretty_print(&self) -> String {
        parenthesize(
            self.operator.pretty_print(),
            Some(vec![*(self.expr_right.clone()), *(self.expr_left.clone())]),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    op: Token,
}

impl Operator {
    pub fn new(op: Token) -> Self {
        match op.as_type() {
            TokenType::EqualEqual
            | TokenType::BangEqual
            | TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Star
            | TokenType::Slash => Self { op },
            _ => panic!("Operator cannot be built from {}", op.as_type()),
        }
    }
}

impl PrettyPrint for Operator {
    fn pretty_print(&self) -> String {
        self.op.as_string()
    }
}
