// Todo: Refactor Ranged pos to make the range field accepting any kind of range

use std::ops::RangeInclusive;

pub fn get_line(n: usize, s: &str) -> Option<&str> {
    s.lines().nth(n)
}
#[derive(Debug, Clone, PartialEq)]
pub struct RangedPos {
    pub line: usize,
    pub char_: usize,
    pub range: RangeInclusive<usize>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub line: usize,
    pub char_: usize,
}
impl Pos {
    pub fn advance(&mut self) {
        self.char_ += 1;
    }
    pub fn newline(&mut self) {
        self.char_ = 1;
        self.line += 1;
    }
    pub fn into_ranged(&self, range: RangeInclusive<usize>) -> RangedPos {
        RangedPos {
            line: self.line,
            char_: self.char_,
            range,
        }
    }
}
impl Default for Pos {
    fn default() -> Self {
        Self { line: 1, char_: 1 }
    }
}
