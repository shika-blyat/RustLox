#![allow(dead_code)]

mod errors;
mod lexer;
mod tokens;
mod utils;

use lexer::Lexer;

fn main() {
    let lexer = Lexer::new("fn a(){ 152 != 5 }");
    println!("{:#?}", lexer.tokenize());
}
