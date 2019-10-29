use std::io::stdin;
use std::io::stdout;
use std::io::Write;

pub mod utility;
use utility::tokenizer::Scanner;

fn main() {
    run_interpreter();
}

fn run_interpreter() {
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
fn execute_line(line: &str) -> bool {
    if line.to_lowercase() == "quit()" {
        return false;
    }
    let mut scanner = Scanner::new(line);
    scanner.scan_tokens();
    true
}

/*fn open_file(path: &str) -> Result<String, Error> {
    use std::io::Read;
    use std::io::Error;
    use std::fs::File;

    let mut file = File::open(path)?;
    let mut program = String::new();
    file.read_to_string(&mut program)?;
    Ok(program)
}*/
