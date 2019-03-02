
/* Basic parser accepting the following language:
 *  ; comments
 *  #! comments !#
 *  integers and basic floats
 *  " \C... " strings (C...: ", xHH, \, n)
 *  [] lists
 *  one or more punctuation at the start of a word is its own word
 *  whitespace separates words
 *  everything else is a word
 */

use std::error::Error;
use std::fmt;
use std::num::{ParseIntError, ParseFloatError};

use crate::data::{Datum, List, error::BuiltinError};

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct SourcePos {
    line: usize,
    col: usize,
}

impl SourcePos {
    fn new() -> Self {
        SourcePos { line: 1, col: 1 }
    }

    fn read(&mut self, c: char) {
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
    end: SourcePos,
    // source text should instead be looked up higher up
    // so you can do and error message like this (i.e. like rustc):
    //               ^^^ should be 'an'
}

impl Source {
    pub fn new() -> Self {
        Source {
            file: None,
            start: SourcePos::new(),
            end: SourcePos::new(),
        }
    }
    pub fn with_file<F: Into<Option<String>>>(mut self, f: F) -> Self {
        self.file = f.into();
        self
    }
    fn from_pos(pos: SourcePos) -> Self {
        Source {
            file: None,
            start: pos.clone(),
            end: pos,
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref file) = self.file.as_ref() {
            write!(fmt, "file '{}', ", file)?;
        }
        if self.start.line == self.end.line {
            write!(fmt, "line {}, ", self.start.line)?;
            if self.start.col == self.end.col {
                write!(fmt, "column {}", self.start.col)?;
            } else {
                write!(fmt, "cols {}-{}", self.start.col, self.end.col)?;
            }
        } else {
            write!(fmt, "{}:{} - {}:{}",
                   self.start.line, self.start.col,
                   self.end.line, self.end.col)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ParseError<T> {
    UnbalancedOpenList(T),
    UnbalancedCloseList(T),
    UnbalancedComment(T),
    UnbalancedString(T),
    BadInteger(T, ParseIntError),
    BadFloat(T, ParseFloatError),
}

impl BuiltinError for ParseError<Source> {
    fn name(&self) -> &'static str { "parse-error" }
    fn args(&self) -> Vec<Datum> {
        vec![Datum::new(format!("{}", self))]
    }
}

impl Error for ParseError<Source> {}

impl fmt::Display for ParseError<Source> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Parse error: ")?;
        match self {
            ParseError::UnbalancedOpenList(pos) => write!(fmt, "Unbalanced [ at {}", pos),
            ParseError::UnbalancedCloseList(pos) => write!(fmt, "Unbalanced ] at {}", pos),
            ParseError::UnbalancedComment(pos) => write!(fmt, "Unclosed open comment at {}", pos),
            ParseError::UnbalancedString(pos) => write!(fmt, "Unclosed string at {}", pos),
            ParseError::BadInteger(pos, err) => write!(fmt, "Couldn't parse integer at {} ({})", pos, err),
            ParseError::BadFloat(pos, err) => write!(fmt, "Couldn't parse number at {} ({})", pos, err),
        }
    }
}

impl<T> ParseError<T> {
    fn map_source<S, F: FnOnce(T) -> S>(self, f: F) -> ParseError<S> {
        use ParseError::*;
        match self {
            UnbalancedOpenList(t) => UnbalancedOpenList(f(t)),
            UnbalancedCloseList(t) => UnbalancedCloseList(f(t)),
            UnbalancedComment(t) => UnbalancedComment(f(t)),
            UnbalancedString(t) => UnbalancedString(f(t)),
            BadInteger(t, e) => BadInteger(f(t), e),
            BadFloat(t, e) => BadFloat(f(t), e),
        }
    }
}

pub struct ReadIter<I: Iterator<Item=char>> {
    stash: Vec<char>,
    iter: I,
}

impl<I: Iterator<Item=char>> ReadIter<I> {
    pub fn new(iter: I) -> Self {
        Self {
            stash: vec![],
            iter,
        }
    }
    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.stash.pop() {
            Some(c)
        } else {
            self.iter.next()
        }
    }
    fn push_back(&mut self, c: char) {
        self.stash.push(c);
    }
}

#[derive(Debug)]
enum ReadCmd {
    StartList,
    EndList,
    StartString,
    StartBlockComment,
    Atom(Datum),
}

fn discard_whitespace<I: Iterator<Item=char>>(pos: &mut SourcePos, reader: &mut ReadIter<I>) {
    while let Some(c) = reader.next() {
        if !c.is_whitespace() {
            reader.push_back(c);
            return;
        }
        pos.read(c);
    }
}

fn discard_until<I: Iterator<Item=char>>(pos: &mut SourcePos, reader: &mut ReadIter<I>, ch: char) -> bool {
    while let Some(c) = reader.next() {
        pos.read(c);
        if c == ch {
            return true;
        }
    }
    false
}

fn read_number<I: Iterator<Item=char>>(c: char, pos: &mut SourcePos, reader: &mut ReadIter<I>) -> Result<Datum, ParseError<SourcePos>> {
    let mut float = false;
    let mut s = String::new();
    s.push(c);
    while let Some(c) = reader.next() {
        if c == '.' {
            pos.read(c);
            s.push(c);
            float = true;
        } else if c.is_numeric() {
            pos.read(c);
            s.push(c);
        } else {
            reader.push_back(c);
            break;
        }
    }
    if float {
        Ok(Datum::new(s.parse::<f64>().map_err(|e| ParseError::BadFloat(pos.clone(), e))?))
    } else {
        Ok(Datum::new(s.parse::<isize>().map_err(|e| ParseError::BadInteger(pos.clone(), e))?))
    }
}

fn atomic_end(c: char) -> bool {
    if c.is_whitespace() {
        true
    } else {
        match c {
            '(' | ')' | '[' | ']' | '"' | ';' => true,
            _ => false,
        }
    }
}

fn read_symbol<I: Iterator<Item=char>>(c: char, pos: &mut SourcePos, reader: &mut ReadIter<I>) -> Datum {
    let mut s = String::new();
    s.push(c);
    match c {
        '\'' | ':' => {},
        _ => {
            while let Some(c) = reader.next() {
                if atomic_end(c) {
                    reader.push_back(c);
                    break;
                } else {
                    pos.read(c);
                    s.push(c);
                }
            }
        },
    }
    Datum::symbol(s)
}

// TODO these inner ones don't correctly report the place of error
fn read_atom<I: Iterator<Item=char>>(pos: &mut SourcePos, reader: &mut ReadIter<I>) -> Result<Option<Datum>, ParseError<SourcePos>> {
    if let Some(c) = reader.next() {
        if c.is_numeric() {
            Ok(Some(read_number(c, pos, reader)?))
        } else {
            Ok(Some(read_symbol(c, pos, reader)))
        }
    } else { Ok(None) }
}

fn read_block_comment_inner<I: Iterator<Item=char>>(pos: &mut SourcePos, reader: &mut ReadIter<I>) -> bool {
    while let Some(c) = reader.next() {
        pos.read(c);
        if c == '!' {
            if let Some(x) = reader.next() {
                pos.read(c);
                if x == '#' {
                    return true;
                }
            }
        }
    }
    false
}

fn read_string_inner<I: Iterator<Item=char>>(acc: &mut String, pos: &mut SourcePos, reader: &mut ReadIter<I>) -> bool {
    while let Some(c) = reader.next() {
        pos.read(c);
        if c == '"' {
            return true;
        } else if c == '\\' {
            if let Some(esc) = reader.next() {
                pos.read(esc);
                match esc {
                    'n' => acc.push('\n'),
                    esc => acc.push(esc),
                }
            }
        } else {
            acc.push(c);
        }
    }
    false
}

fn read_cmd<I: Iterator<Item=char>>(pos: &mut SourcePos, reader: &mut ReadIter<I>) -> Result<Option<(SourcePos, ReadCmd)>, ParseError<SourcePos>> {
    loop {
        discard_whitespace(pos, reader);
        match reader.next() {
            Some(';') => {
                pos.read(';');
                discard_until(pos, reader, '\n');
            },
            Some('[') => {
                let p = pos.clone();
                pos.read('[');
                return Ok(Some((p, ReadCmd::StartList)));
            },
            Some(']') => {
                let p = pos.clone();
                pos.read(']');
                return Ok(Some((p, ReadCmd::EndList)));
            },
            Some('"') => {
                let p = pos.clone();
                pos.read('"');
                return Ok(Some((p, ReadCmd::StartString)));
            },
            Some('#') => {
                let p = pos.clone();
                match reader.next() {
                    Some('!') => {
                        pos.read('!');
                        return Ok(Some((p, ReadCmd::StartBlockComment)));
                    },
                    Some(c) => {
                        if c.is_numeric() || c.is_alphabetic() || atomic_end(c) {
                            reader.push_back(c);
                            return Ok(Some((p, ReadCmd::Atom(Datum::symbol("#")))));
                        } else {
                            let mut s = "#".to_string();
                            s.push(c);
                            return Ok(Some((p, ReadCmd::Atom(Datum::symbol(s)))));
                        }
                    }
                    None => {
                        return Ok(Some((p, ReadCmd::Atom(Datum::symbol("#")))));
                    },
                }
            },
            Some(x) => {
                let p = pos.clone();
                reader.push_back(x);
                if let Some(atom) = read_atom(pos, reader)? {
                    return Ok(Some((p, ReadCmd::Atom(atom))));
                } else {
                    return Ok(None);
                }
            }
            None => return Ok(None),
        }
    }
}

struct ParsingList(Vec<Datum>, SourcePos);

fn read_toplevel_datum<I: Iterator<Item=char>>(pos: &mut SourcePos,
                                               reader: &mut ReadIter<I>) ->
        Result<Option<(SourcePos, Datum)>, ParseError<SourcePos>> {
    let mut nesting = vec![];
    while let Some((start, readcmd)) = read_cmd(pos, reader)? {
        match readcmd {
            ReadCmd::StartList => {
                nesting.push(ParsingList(vec![], start));
            },
            ReadCmd::EndList => {
                match nesting.pop() {
                    None => return Err(ParseError::UnbalancedCloseList(start)),
                    Some(ParsingList(data, start)) => {
                        let datum = Datum::new(List::from(data));
                        match nesting.pop() {
                            None => return Ok(Some((start, datum))),
                            Some(ParsingList(mut data, start)) => {
                                data.push(datum);
                                nesting.push(ParsingList(data, start));
                            },
                        }
                    },
                }
            },
            ReadCmd::StartString => {
                let mut s = String::new();
                let r = read_string_inner(&mut s, pos, reader);
                if r {
                    let datum = Datum::new(s);
                    match nesting.pop() {
                        None => return Ok(Some((start, datum))),
                        Some(ParsingList(mut data, start)) => {
                            data.push(datum);
                            nesting.push(ParsingList(data, start));
                        },
                    }
                } else {
                    return Err(ParseError::UnbalancedString(start));
                }
            },
            ReadCmd::StartBlockComment => {
                let start = pos.clone();
                if !read_block_comment_inner(pos, reader) {
                    return Err(ParseError::UnbalancedComment(start));
                }
            },
            ReadCmd::Atom(datum) => {
                match nesting.pop() {
                    None => return Ok(Some((start, datum))),
                    Some(ParsingList(mut data, start)) => {
                        data.push(datum);
                        nesting.push(ParsingList(data, start));
                    },
                }
            },
        }
    }
    if let Some(ParsingList(_, pos)) = nesting.pop() {
        return Err(ParseError::UnbalancedOpenList(pos));
    }
    Ok(None)
}

pub struct Parser<I: Iterator<Item=char>> {
    reader: ReadIter<I>,
    file: Option<String>,
    pos: SourcePos,
}

impl<I: Iterator<Item=char>> Parser<I> {
    pub fn new(iter: I) -> Self {
        let reader = ReadIter::new(iter);
        let pos = SourcePos::new();
        Parser { reader, pos, file: None }
    }
    pub fn with_file<F: Into<String>>(mut self, file: F) -> Self {
        self.file = Some(file.into());
        self
    }
    pub fn next(&mut self) -> Result<Option<Datum>, ParseError<Source>> {
        if let Some((_start, datum)) = read_toplevel_datum(&mut self.pos, &mut self.reader)
                .map_err(|e| ParseError::map_source(e, |p| Source::from_pos(p).with_file(self.file.clone())))? {
            Ok(Some(datum))
        } else {
            Ok(None)
        }
    }
}

