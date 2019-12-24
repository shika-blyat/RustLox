use crate::utility::grammar::{Expr, Literal, PrimOp};
use crate::utility::{lexer::Token, tokens::TokenType};

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Expr {
        self.expression()
    }
    fn expression(&mut self) -> Expr {
        self.equality()
    }
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_tokens(vec![PrimOp::NotEq, PrimOp::EqEq]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(
                Box::new(expr),
                Parser::to_op(operator.as_type()),
                Box::new(right),
            );
        }
        return expr;
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.addition();
        while self.match_tokens(vec![
            PrimOp::Greater,
            PrimOp::GreaterEq,
            PrimOp::Less,
            PrimOp::LessEq,
        ]) {
            let operator = self.previous();
            let right = self.addition();
            expr = Expr::Binary(
                Box::new(expr),
                Parser::to_op(operator.as_type()),
                Box::new(right),
            );
        }
        return expr;
    }
    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();
        while self.match_tokens(vec![PrimOp::Sub, PrimOp::Add]) {
            let operator = self.previous();
            let right = self.multiplication();
            expr = Expr::Binary(
                Box::new(expr),
                Parser::to_op(operator.as_type()),
                Box::new(right),
            );
        }
        return expr;
    }
    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_tokens(vec![PrimOp::Div, PrimOp::Mul]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(
                Box::new(expr),
                Parser::to_op(operator.as_type()),
                Box::new(right),
            );
        }
        return expr;
    }
    fn unary(&mut self) -> Expr {
        let mut expr = self.primary();
        while self.match_tokens(vec![PrimOp::Div, PrimOp::Mul]) {
            let operator = self.previous();
            let right = self.primary();
            expr = Expr::Unary(Parser::to_op(operator.as_type()), Box::new(right));
        }
        return expr;
    }
    fn primary(&mut self) -> Expr {
        match self.peek().as_type() {
            TokenType::True => {
                self.advance();
                Expr::Lit(Literal::Bool(true))
            }
            TokenType::False => {
                self.advance();
                Expr::Lit(Literal::Bool(false))
            }
            TokenType::Float(f) => {
                self.advance();
                Expr::Lit(Literal::Float(f))
            }
            TokenType::Int(i) => {
                self.advance();
                Expr::Lit(Literal::Int(i))
            }
            TokenType::LeftParen => {
                let expr = self.expression();
                if self.peek().as_type() == TokenType::RightParen {
                    self.advance();
                    Expr::Grouped(Box::new(expr))
                } else {
                    panic!("Expected ')' after expression")
                }
            }
            _ => panic!("{} isn't a valid literal", self.peek().as_string()),
        }
    }
    fn match_tokens(&mut self, types: Vec<PrimOp>) -> bool {
        for i in types {
            if self.check(i) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, tok: PrimOp) -> bool {
        if self.is_at_end() {
            return false;
        }
        return Parser::to_op(self.peek().as_type()) == tok;
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    fn is_at_end(&self) -> bool {
        return self.peek().as_type() == TokenType::Eof;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }
    fn to_op(tok: TokenType) -> PrimOp {
        match tok {
            TokenType::Plus => PrimOp::Add,
            TokenType::Minus => PrimOp::Sub,
            TokenType::Star => PrimOp::Mul,
            TokenType::Slash => PrimOp::Div,
            TokenType::EqEq => PrimOp::EqEq,
            TokenType::Greater => PrimOp::Greater,
            TokenType::GreaterEq => PrimOp::GreaterEq,
            TokenType::Less => PrimOp::Less,
            TokenType::LessEq => PrimOp::LessEq,
            TokenType::Not => PrimOp::Not,
            TokenType::NotEq => PrimOp::NotEq,
            _ => panic!("Unknow operation token {:#?}", tok),
        }
    }
}
