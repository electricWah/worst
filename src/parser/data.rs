
use std::fmt;
use data::*;
use combo::*;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct SourcePos {
    line: usize,
    col: usize,
}

impl SourcePos {
    fn new() -> Self {
        SourcePos { line: 1, col: 1 }
    }

    fn advance_char(&mut self, c: char) {
        if c == '\n' {
            self.line = self.line + 1;
            self.col = 1;
        } else {
            self.col = self.col + 1;
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Source {
    file: Option<String>,
    start: SourcePos,
    end: Option<SourcePos>,
    // source text should instead be looked up higher up
    // so you can do and error message like this (i.e. like rustc):
    //               ^^^ should be 'an'
}

impl Source {
    pub fn new() -> Self {
        Source {
            file: None,
            start: SourcePos::new(),
            end: None,
        }
    }
    pub fn with_file<F: Into<Option<String>>>(mut self, f: F) -> Self {
        self.file = f.into();
        self
    }
    pub fn set_started(&mut self) {
        self.end = Some(self.start.clone());
    }
    pub fn set_ended(&mut self) {
        self.end = None;
    }
    pub fn read(&mut self, c: char) {
        match self.end {
            Some(ref mut e) => {
                e.advance_char(c);
            },
            None => {
                self.start.advance_char(c);
            }
        }
    }
    pub fn current_position(&self) -> (Option<String>, usize, usize) {
        let pos = self.end.as_ref().unwrap_or(&self.start);
        (self.file.clone(), pos.line, pos.col)
    }
}

impl fmt::Display for Source {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref file) = self.file.as_ref() {
            write!(fmt, "file '{}', ", file)?;
        }
        match self.end {
            None => {
                write!(fmt, "line {}, column {}", self.start.line, self.start.col)?;
            },

            Some(ref end) => {
                if self.start.line == end.line {
                    write!(fmt, "line {}, ", self.start.line)?;
                    write!(fmt, "cols {}-{}", self.start.col, end.col)?;
                } else {
                    write!(fmt, "{}:{} - {}:{}",
                           self.start.line, self.start.col,
                           end.line, end.col)?;
                }
                // write!(fmt, ": ‘{}’", text)?;
            },
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum CharClass {
    One(char),
    Whitespace, Alpha, Numeric, Symbol,
    Eof,
}

impl From<char> for CharClass {
    fn from(c: char) -> Self {
        CharClass::One(c)
    }
}

pub type AcceptChar = Combo<CharClass>;

pub type AcceptState = Combo<Symbol>;

