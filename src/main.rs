mod utility;

use utility::ast::Parser;
use utility::lexer::{Lexer, Token};
use utility::tokens::TokenType;

fn main() {
    let code = "( 2 + 5 )";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    if let TokenType::Error(_) = tokens[0].as_type() {
        for i in Token::read_errors(tokens).expect("Failed to read errors") {
            println!("{}", i);
        }
    } else {
        println!("{:#?}", tokens);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        println!("{:#?}", ast);
    }
}

/*

use std::env::args;
use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::io::Error;
use std::io::Read;
use std::io::Write;
fn run_interpreter() {
    /*if let Some(path) = args().nth(1) {
        let file = open_file(&path).unwrap();
    } else {*/
    loop {
        let mut line = String::new();
        print!(">>> ");
        if let Err(a) = stdout().flush() {
            eprintln!("An error occured while reading the line:\n{}", a);
        }
        stdin().read_line(&mut line).unwrap();
        match execute_line(&line.trim()) {
                true => (),
                false => break,
            };
        }
    }
}
fn execute_line(line: &str) -> bool {
    if line.to_lowercase() == "quit()" {
        return false;
    }
    true
}
fn open_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut program = String::new();
    file.read_to_string(&mut program)?;
    Ok(program)
}*/
