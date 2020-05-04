use crate::utils::{get_line, RangedPos};
use std::fmt;
fn format_err(file: &str, code: &str, pos: RangedPos, err_kind: &impl fmt::Display) -> String {
    let mut arrows = " ".repeat(pos.char_ - 1);
    arrows.push_str("^".repeat(pos.range.count()).as_str());
    format!(
        "Error: {}
        {}| {}
            {}
          {}",
        file,
        pos.line,
        get_line(pos.line - 1, code).expect("Internal compiler error, unexpected panic"),
        arrows,
        err_kind,
    )
}

#[derive(Debug)]
pub enum LexErrorKind {
    UnexpectedChar(char),
    UnclosedDelimiter(char),
}

impl fmt::Display for LexErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::UnexpectedChar(c) => write!(fmt, "Unexpected char `{}`", c),
            Self::UnclosedDelimiter(c) => write!(fmt, "Unclosed delimiter `{}`", c),
        }
    }
}
#[derive(Debug)]
pub struct LexError<'a> {
    pub kind: LexErrorKind,
    pub code: &'a str,
    pub pos: RangedPos,
}
impl<'a> fmt::Display for LexError<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "{}",
            format_err("main.ka", self.code, self.pos.clone(), &self.kind)
        )
    }
}

/*
    let code = "fn main(){
        printChar('a);
    }";
    let pos = RangedPos {
        line: 2,
        char_: 18,
        range: 0..=3,
    };
    let file = "main.ka";
    let err = LexError::UnclosedDelimiter('\'');
    println!("{}", err.format_err(file, code, pos));
*/
