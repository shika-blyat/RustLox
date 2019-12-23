use std::env::args;
use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::io::Error;
use std::io::Read;
use std::io::Write;

pub mod parser;
use crate::parser::grammar::PrettyPrint;
use crate::parser::grammar::{Binary, Expr, Grouping, Literal, Operator, Unary};
use crate::parser::lexer::{Scanner, Token};
use crate::parser::tokens::TokenType;

fn main() {
    /*if let Some(path) = args().nth(1) {
        let file = open_file(&path).unwrap();
        execute_source(&file.trim());
    }
    run_interpreter();*/
    let expr = Expr::Binary(Binary::new(
        Expr::Unary(Unary::new(
            Token::new(TokenType::Minus, "-".to_owned(), 0),
            Expr::Literal(Literal::new(Token::new(
                TokenType::Number(123.0),
                "123".to_owned(),
                0,
            ))),
        )),
        Operator::new(Token::new(TokenType::Star, "*".to_owned(), 0)),
        Expr::Grouping(Grouping::new(
            Token::new(TokenType::LeftParen, "(".to_owned(), 0),
            Expr::Literal(Literal::new(Token::new(
                TokenType::Number(45.67),
                "45.67".to_owned(),
                0,
            ))),
            Token::new(TokenType::RightParen, ")".to_owned(), 0),
        )),
    ));
    println!("{}", expr.pretty_print());
}

fn run_interpreter() {
    loop {
        let mut line = String::new();
        print!(">>> ");
        if let Err(a) = stdout().flush() {
            eprintln!("An error occured while reading the line:\n{}", a);
        }
        stdin().read_line(&mut line).unwrap();
        match execute_source(&line.trim()) {
            true => (),
            false => break,
        };
    }
}
fn execute_source(line: &str) -> bool {
    if line.to_lowercase() == "quit()" {
        return false;
    }
    let mut scanner = Scanner::new(line);
    scanner.scan_tokens();
    true
}

fn open_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut program = String::new();
    file.read_to_string(&mut program)?;
    Ok(program)
}
