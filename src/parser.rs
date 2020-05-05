use crate::{
    ast::*,
    tokens::{TokenKind, TokensIterator},
    utils::{box_, RangedPos},
};

pub struct Parser<'a> {
    tokens: TokensIterator<'a>,
}
impl<'a> Parser<'a> {
    pub fn new(tokens: TokensIterator<'a>) -> Self {
        Self { tokens }
    }
    fn fun_call(&mut self) -> Option<Expr> {
        self.ident().and_then(|ident| {
            let mut values = self.paren_comma_sep(Self::expr)?.into_iter();
            match values.next() {
                Some(e) => {
                    let mut expr = Expr::Call {
                        fun: box_(ident),
                        param: box_(e),
                    };
                    for e in values {
                        expr = Expr::Call {
                            fun: box_(expr),
                            param: box_(e),
                        }
                    }
                    Some(expr)
                }
                None => Some(ident),
            }
        })
    }
    fn paren_comma_sep<T>(&mut self, val: fn(&mut Self) -> Option<T>) -> Option<Vec<T>> {
        let mut values = vec![];
        self.get_tok(TokenKind::LParen)
            .map(|_| {
                while let Some(v) = val(self) {
                    values.push(v);
                    match self.get_tok(TokenKind::Comma) {
                        Some(_) => (),
                        None => break,
                    };
                }
                self.get_tok(TokenKind::Comma).map_or_else(|| (), |_| ())
            })
            .and_then(|_| self.get_tok(TokenKind::RParen))?;
        Some(values)
    }
    fn expr(&mut self) -> Option<Expr<'a>> {
        self.atom()
    }
    fn atom(&mut self) -> Option<Expr<'a>> {
        self.ident().or_else(|| self.num()).or_else(|| self.bool())
    }
    fn ident(&mut self) -> Option<Expr<'a>> {
        Some(Expr::Ident(self.get_ident()?.0))
    }
    fn num(&mut self) -> Option<Expr<'a>> {
        Some(Expr::Lit(Lit::Num(self.get_num()?.0)))
    }
    fn bool(&mut self) -> Option<Expr<'a>> {
        Some(Expr::Lit(Lit::Bool(self.get_bool()?.0)))
    }
    fn get_ident(&mut self) -> Option<(&'a str, RangedPos)> {
        if let Some((TokenKind::Ident(s), pos)) = self.peek() {
            self.tokens.advance();
            return Some((s, pos));
        }
        None
    }
    fn get_num(&mut self) -> Option<(isize, RangedPos)> {
        if let Some((TokenKind::Num(n), pos)) = self.peek() {
            self.tokens.advance();
            return Some((n, pos));
        }
        None
    }
    fn get_bool(&mut self) -> Option<(bool, RangedPos)> {
        if let Some((TokenKind::Bool(b), pos)) = self.peek() {
            self.tokens.advance();
            return Some((b, pos));
        }
        None
    }
    fn get_tok(&mut self, kind: TokenKind<'a>) -> Option<(TokenKind<'a>, RangedPos)> {
        if let Some((tok_kind, _)) = self.peek() {
            if kind == tok_kind {
                return Some(self.next().unwrap());
            }
        }
        None
    }
    fn next(&mut self) -> Option<(TokenKind<'a>, RangedPos)> {
        self.tokens.next()
    }
    fn peek(&mut self) -> Option<(TokenKind<'a>, RangedPos)> {
        self.tokens.peek()
    }
}
