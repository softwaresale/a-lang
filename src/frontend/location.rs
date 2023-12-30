use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

/// represents a single source location in the program
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Default)]
pub struct SourceLocation {
    /// the line of the input
    pub line: usize,
    /// the column/character in that line
    pub col: usize,
}

impl SourceLocation {
    pub fn newline(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    pub fn bump(&mut self) -> Self {
        self.col += 1;
        self.clone()
    }

    pub fn advance(&mut self, n: usize) -> Self {
        self.col += n;
        self.clone()
    }

    pub fn cr(&mut self) {
        self.col = 0;
    }

    pub fn update_with_char(&mut self, next_ch: char) {
        match next_ch {
            '\n' => self.newline(),
            '\r' => self.cr(),
            _ => { self.bump(); },
        };
    }
}

impl Ord for SourceLocation {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.line.cmp(&other.line) {
            Ordering::Equal => {
                self.col.cmp(&other.col)
            }
            ord => ord
        }
    }
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

/// A source range
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct SourceRange {
    /// The start of the range inclusive
    pub start: SourceLocation,
    /// the end of the range, exclusive
    pub end: SourceLocation,
}

impl SourceRange {
    pub fn spanned<StartT: HasLocation, EndT: HasLocation>(start: &StartT, end: &EndT) -> Self {
        let start = start.source_range().start;
        let end = end.source_range().end;
        Self { start, end }
    }
}

impl From<(SourceLocation, SourceLocation)> for SourceRange {
    fn from(value: (SourceLocation, SourceLocation)) -> Self {
        Self {
            start: value.0,
            end: value.1
        }
    }
}

impl Display for SourceRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl PartialOrd for SourceRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

pub trait HasLocation {
    fn source_range(&self) -> SourceRange;
}

impl HasLocation for SourceRange {
    fn source_range(&self) -> SourceRange {
        *self
    }
}
