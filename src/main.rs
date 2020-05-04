#![allow(dead_code)]

mod ast;
mod errors;
mod lexer;
mod parser;
mod tests;
mod tokens;
mod utils;

use lexer::Lexer;

fn main() {
    let lexer = Lexer::new("fn a(){ 152 != 5 }");
    println!("{:#?}", lexer.tokenize());
}
