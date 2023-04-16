
//! A [Reader] is a little doodad that eats text and poops code.

use std::num::IntErrorKind;
use crate::base::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// The current state of reading some code.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Default, Clone)]
pub struct Reader {
    lists: Vec<ListState>,
    state: BasicState,
}
impl Value for Reader {}

#[derive(Default, Clone, Debug)]
enum BasicState {
    #[default] Space,
    Comment,
    Hash,
    Atom(String),
    String {
        buf: String,
        escaping: bool,
    },
}

#[derive(Clone)]
struct ListState {
    begin: char,
    end: char,
    data: Vec<Val>,
}

/// Various ways parsing could fail.
#[derive(Debug, Clone)]
pub enum ReadError {
    /// An odd number of `"`
    UnclosedString,
    /// A `#` right at the end of input
    UnmatchedHash,
    /// A `#` followed by something unexpected
    UnknownHash(char),
    /// An unbalanced list delimiter
    UnmatchedList(char),
    /// A number that looked like it was but isn't
    UnparseableNumber(String),
}
impl Value for ReadError {}

impl Reader {

    /// Read some code using this reader into the given accumulator vector.
    /// This may be a partial chunk of code; use [complete] to wrap up at the end.
    pub fn read_into(&mut self, mut s: impl Iterator<Item=char>, acc: &mut Vec<Val>) -> Result<(), ReadError> {
        'top: loop {
            match &mut self.state {
                BasicState::Space =>
                    'space: loop {
                        match s.next() {
                            None => break 'top,
                            Some(c) =>
                                if !c.is_whitespace() {
                                    if let Some(list) = self.read_char_state(c)? {
                                        self.emit(list.into(), acc);
                                    }
                                    break 'space;
                                },
                        }
                    },
                BasicState::Comment =>
                    'comment: loop {
                        match s.next() {
                            None => break 'top,
                            Some('\n') => {
                                self.state = BasicState::Space;
                                break 'comment;
                            },
                            Some(_) => {},
                        }
                    },
                BasicState::Hash =>
                    match s.next() {
                        None => break 'top,
                        Some('!') => {
                            self.state = BasicState::Comment;
                        },
                        Some(c@('t' | 'f')) => {
                            self.emit((c == 't').into(), acc);
                            self.state = BasicState::Space;
                        },
                        Some(c) => return Err(ReadError::UnknownHash(c)),
                    },
                BasicState::Atom(a) =>
                    'atom: loop {
                        match s.next() {
                            None => break 'top,
                            Some(c@(';' | '"' | '(' | ')' | '[' | ']' | '{' | '}')) => {
                                // TODO no clone here
                                let v = a.clone();
                                self.state = BasicState::Space;
                                self.emit(parse_atom(v)?, acc);
                                if let Some(list) = self.read_char_state(c)? {
                                    self.emit(list.into(), acc);
                                }
                                break 'atom;
                            },
                            Some(c) =>
                                if c.is_whitespace() {
                                    let v = a.clone();
                                    self.state = BasicState::Space;
                                    self.emit(parse_atom(v)?, acc);
                                    break 'atom;
                                } else {
                                    a.push(c);
                                },
                        }
                    },
                    BasicState::String { buf, escaping } =>
                        'string: loop {
                            match s.next() {
                                None => break 'top,
                                Some(c) => {
                                    if *escaping {
                                        *escaping = false;
                                        match c {
                                            'e' => buf.push('\u{1b}'),
                                            'n' => buf.push('\n'),
                                            'r' => buf.push('\r'),
                                            't' => buf.push('\t'),
                                            c => buf.push(c),
                                        }
                                    } else if c == '"' {
                                        let v = buf.clone();
                                        self.emit(v.into(), acc);
                                        self.state = BasicState::Space;
                                        break 'string;
                                    } else if c == '\\' {
                                        *escaping = true;
                                    } else {
                                        buf.push(c);
                                    }
                                },
                            }
                        },
            }
        }
        Ok(())
    }

    /// There is no more code to read, but the reader may still be halfway
    /// through a list,
    /// or perhaps there's an atom or number at the very end of the file.
    pub fn complete(mut self) -> Result<Option<Val>, ReadError> {
        if let Some(ls) = self.lists.pop() {
            return Err(ReadError::UnmatchedList(ls.begin));
        }
        match self.state {
            BasicState::Space | BasicState::Comment => Ok(None),
            BasicState::Hash => Err(ReadError::UnmatchedHash),
            BasicState::Atom(a) => Ok(Some(parse_atom(a)?)),
            BasicState::String { .. } => Err(ReadError::UnclosedString),
        }
    }

    fn emit(&mut self, v: Val, out: &mut Vec<Val>) {
        if let Some(ls) = self.lists.last_mut() {
            ls.data.push(v);
        } else {
            out.push(v);
        }
    }

    fn start_list(&mut self, begin: char, end: char) {
        self.lists.push(ListState { begin, end, data: vec![], });
    }
    fn end_list(&mut self, c: char) -> Result<List, ReadError> {
        if let Some(ls) = self.lists.pop() {
            if c == ls.end {
                Ok(List::from(ls.data))
            } else {
                Err(ReadError::UnmatchedList(ls.begin))
            }
        } else {
            Err(ReadError::UnmatchedList(c))
        }
    }

    fn read_char_state(&mut self, c: char) -> Result<Option<List>, ReadError> {
        match c {
            ';' => self.state = BasicState::Comment,
            '"' => self.state = BasicState::String {
                buf: "".to_string(), escaping: false,
            },
            '#' => self.state = BasicState::Hash,
            '(' => self.start_list('(', ')'),
            '[' => self.start_list('[', ']'),
            '{' => self.start_list('{', '}'),
            ')' | ']' | '}' => return Ok(Some(self.end_list(c)?)),
            c => self.state = BasicState::Atom(c.into()),
        }
        Ok(None)
    }
}

fn parse_atom(s: String) -> Result<Val, ReadError> {
    match str::parse::<i64>(&s) {
        Ok(v) => Ok(v.into()),
        Err(e) if e.kind() == &IntErrorKind::PosOverflow
            || e.kind() == &IntErrorKind::NegOverflow =>
            Err(ReadError::UnparseableNumber(s)),
        Err(_) =>
            match str::parse::<f64>(&s) {
                Ok(v) => Ok(v.into()),
                Err(_) => Ok(Symbol::from(s).into()),
            },
    }
}

/// Read an entire piece of text as Worst values using the default reader.
pub fn read_all(src: &mut impl Iterator<Item=char>) -> Result<Vec<Val>, ReadError> {
    let mut reader = Reader::default();
    let mut acc = vec![];
    reader.read_into(src, &mut acc)?;
    if let Some(v) = reader.complete()? {
        acc.push(v);
    }
    Ok(acc)
}

// TODO fix tests or move into worst
#[cfg(test)]
mod tests {
    use super::*;

    // assert nothing trailing here?
    fn vec_read<T: Value + Clone>(s: &str) -> Vec<T> {
        read_all(&mut s.chars()).unwrap()
            .into_iter().map(Val::try_downcast::<T>)
            .map(Result::ok).map(Option::unwrap)
            .map(ValOf::into_inner).collect::<Vec<T>>()
    }

    #[test]
    fn read_none() {
        assert!(vec_read::<i64>("").is_empty());
        assert!(vec_read::<i64>(" \n ; test\n").is_empty());
    }

    #[test]
    fn read_bool() {
        // assert_eq!(vec_read::<bool>("  #t "), vec![true]);
        assert_eq!(vec_read::<bool>("#f#t ;yeah\n #f #t "), vec![false, true, false, true]);
    }

    #[test]
    fn read_string() {
        assert_eq!(vec_read::<String>("\"egg\" \"blub\\nbo\"\"\" \"ok\\\"ok\""),
                    vec!["egg", "blub\nbo", "", "ok\"ok"]);
    }

    #[test]
    fn read_i64() {
        assert_eq!(vec_read::<i64>("123"), vec![123]);
    }

    #[test]
    fn read_symbol() {
        assert_eq!(vec_read::<Symbol>("eggs"), vec!["eggs".to_symbol()]);
        assert_eq!(vec_read::<Symbol>("time for-some\n.cool.beans"),
                    vec!["time".to_symbol(), "for-some".to_symbol(), ".cool.beans".to_symbol()]);
    }

}

