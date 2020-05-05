// TODO: Enhance error handling, add the possibility to return all the catchable errors
// at a given point when a first error already occured, for example, give more precise/helpful error message would be cool too

use crate::{
    ast::*,
    errors::{ParseError, ParseErrorKind},
    tokens::{TokenKind, TokensIterator},
    utils::{box_, RangedPos},
};

use std::ops::RangeInclusive;

pub struct Parser<'a> {
    tokens: TokensIterator<'a>,
    code: &'a str,
}
impl<'a> Parser<'a> {
    pub fn new(tokens: TokensIterator<'a>, code: &'a str) -> Self {
        Self { tokens, code }
    }
    fn fun_call(&mut self) -> Result<Expr<'a>, ParseError> {
        self.ident()
            .ok_or_else(|| {
                let kind = self.peek().unwrap().0;
                self.new_err(
                    ParseErrorKind::ExpectedFound("a function".to_string(), kind),
                    0..=0,
                )
            })
            .and_then(move |ident| {
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
                        Ok(expr)
                    }
                    None => Ok(ident),
                }
            })
    }
    fn paren_comma_sep<T>(
        &mut self,
        val: fn(&mut Self) -> Result<T, ParseError>,
    ) -> Result<Vec<T>, ParseError> {
        let mut values = vec![];
        self.get_tok(TokenKind::LParen)
            .ok_or({
                let kind = self.peek().unwrap().0;
                self.new_err(
                    ParseErrorKind::ExpectedFound("an opening parenthesis".to_string(), kind),
                    0..=0,
                )
            })
            .and_then(|_| {
                while let Ok(v) = val(self) {
                    values.push(v);
                    match self.get_tok(TokenKind::Comma) {
                        Some(_) => (),
                        None => break,
                    };
                }
                self.get_tok(TokenKind::RParen).ok_or({
                    let kind = self.peek().unwrap().0;
                    self.new_err(
                        ParseErrorKind::ExpectedFound("an enclosing parenthesis".to_string(), kind),
                        0..=0,
                    )
                })?;
                Ok(values)
            })
    }
    fn addition(&mut self) -> Result<Expr<'a>, ParseError> {
        let left = self.multiplication()?;
        //while let Some()
        Ok(left)
    }
    fn multiplication(&mut self) -> Result<Expr<'a>, ParseError> {
        let left = self.atom().ok_or_else(|| {
            let kind = self.peek().unwrap().0;
            self.new_err(
                ParseErrorKind::ExpectedFound("an expression".to_string(), kind),
                0..=0,
            )
        })?;
        Ok(left)
    }
    fn expr(&mut self) -> Result<Expr<'a>, ParseError> {
        self.atom().ok_or({
            let kind = self.peek().unwrap().0;
            self.new_err(
                ParseErrorKind::ExpectedFound("an expression".to_string(), kind),
                0..=0,
            )
        })
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
    fn new_err(
        &mut self,
        kind: ParseErrorKind<'a>,
        range: RangeInclusive<usize>,
    ) -> ParseError<'a> {
        ParseError::new(kind, self.peek().unwrap().1.range(range), self.code)
    }
    fn peek(&mut self) -> Option<(TokenKind<'a>, RangedPos)> {
        self.tokens.peek()
    }
}
