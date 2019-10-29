pub mod utility;
use utility::tokenizer::Scanner;

fn main() {
    let mut scanner = Scanner::new("abc");
    scanner.scan_tokens();
}
