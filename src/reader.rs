
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use crate::impl_value;
use crate::base::*;
use crate::list::*;
use crate::interpreter::{Interpreter, Handle};

#[derive(Debug, Clone)]
pub enum ReadError {
    UnclosedString,
    UnmatchedHash,
    UnknownHash(char),
    UnmatchedList(char),
    UnparseableNumber(String),
}
impl_value!(ReadError);

// Interpreter-based reader: feed it text and it will run an interpreter
// to consume the text and output values

// maybe use Port somehow?
#[derive(Debug, Default, Clone)]
// bool eof
struct StringBuffer(Rc<RefCell<(VecDeque<char>, bool)>>);
impl PartialEq for StringBuffer {
    fn eq(&self, other: &Self) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}

impl StringBuffer {
    fn write(&mut self, s: impl IntoIterator<Item=char>) {
        let w = &mut self.0.borrow_mut().0;
        for c in s.into_iter() { w.push_back(c); }
    }
    fn next(&mut self) -> Option<char> {
        self.0.borrow_mut().0.pop_front()
    }
    fn un_next(&mut self, c: char) {
        self.0.borrow_mut().0.push_front(c)
    }
    fn set_eof(&mut self) {
        self.0.borrow_mut().1 = true;
    }
    fn is_empty(&self) -> bool {
        let veof = self.0.borrow();
        veof.0.len() == 0 && veof.1
    }
    fn is_eof(&self) -> bool { self.0.borrow().1 }
}

// TODO this could just be struct Eof; + Val + ReadError
#[derive(Clone)]
enum Emit {
    Eof,
    Yield(Val),
    Error(ReadError),
}
impl_value!(Emit);

struct ReaderHandle {
    i: Handle,
    src: StringBuffer,
    list_stack: Vec<(char, char, Vec<Val>)>,
    // TODO source information
}

impl ReaderHandle {
    fn new(i: Handle, src: StringBuffer) -> Self {
        ReaderHandle { i, src, list_stack: vec![] }
    }

    async fn next(&mut self) -> char {
        loop {
            match self.src.next() {
                None => {
                    self.i.stack_push(Emit::Eof).await;
                    self.i.pause().await;
                },
                Some(c) => return c,
            }
        }
    }

    async fn emit(&mut self, v: impl Value) {
        if let Some((_, _, l)) = self.list_stack.last_mut() {
            l.push(v.into());
        } else {
            self.i.stack_push(Emit::Yield(v.into())).await;
            self.i.pause().await;
        }
    }

    async fn error(&mut self, e: ReadError) {
        self.i.stack_push(Emit::Error(e)).await;
        self.i.pause().await;
    }

    fn start_list(&mut self, open: char, close: char) {
        self.list_stack.push((open, close, vec![]));
    }

    async fn end_list(&mut self, close: char) {
        match self.list_stack.pop() {
            None => self.error(ReadError::UnmatchedList(close)).await,
            Some((o, c, l)) =>
                if c == close {
                    self.emit(List::from(l)).await;
                } else {
                    self.error(ReadError::UnmatchedList(o)).await;
                }
        }
    }

    async fn run(&mut self) -> ! {
        loop {
            // TODO self.i.call(thing-specific reader).await
            // to be able to bail out for e.g. ([}) or #?
            // also maybe configurable:
            // list delimiters, reader handlers, character handlers?
            // for flexible numbers + #X + literal \n + more comment styles etc
            match self.next().await {
                v if v.is_whitespace() => {},
                ';' => while self.next().await != '\n' {},

                '#' => {
                    if self.src.is_empty() {
                        self.error(ReadError::UnmatchedHash).await;
                    }
                    match self.next().await {
                        't' => self.emit(true).await,
                        'f' => self.emit(false).await,
                        // #! shebang comment
                        '!' => while self.next().await != '\n' {},
                        x => self.error(ReadError::UnknownHash(x)).await,
                    }
                },

                '"' => {
                    let mut buf = String::new();
                    'read_string: loop {
                        if self.src.is_empty() {
                            self.error(ReadError::UnclosedString).await;
                        }
                        match self.next().await {
                            '"' => break 'read_string,
                            '\\' => match self.next().await {
                                'e' => buf.push('\u{1b}'),
                                'n' => buf.push('\n'),
                                'r' => buf.push('\r'),
                                c => buf.push(c),
                            },
                            c => buf.push(c),
                        }
                    }
                    self.emit(buf).await;
                },

                '(' => self.start_list('(', ')'),
                '[' => self.start_list('[', ']'),
                '{' => self.start_list('{', '}'),
                c @ (')' | ']' | '}') => self.end_list(c).await,

                c if c.is_numeric() => {
                    let mut buf = String::from(c);
                    'number: loop {
                        if self.src.is_empty() { break 'number; }
                        let c = self.next().await;
                        if c.is_numeric() {
                            buf.push(c);
                        } else {
                            self.src.un_next(c);
                            break 'number;
                        }
                    }
                    if let Ok(v) = str::parse::<i32>(&buf) {
                        self.emit(v).await;
                    } else {
                        self.error(ReadError::UnparseableNumber(buf)).await;
                    }

                },

                c => {
                    let mut buf = String::from(c);
                    'symbol: loop {
                        if self.src.is_empty() { break 'symbol; }
                        match self.next().await {
                            c if c.is_whitespace() => break 'symbol,
                            c@('(' | ')' | '[' | ']' | '{' | '}' | '"') => {
                                self.src.un_next(c);
                                break 'symbol;
                            },
                            c => buf.push(c),
                        }
                    }
                    self.emit(Symbol::from(buf)).await;
                },
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Reader {
    buf: StringBuffer,
    i: Rc<RefCell<Interpreter>>,
}
impl PartialEq for Reader {
    fn eq(&self, other: &Self) -> bool {
        self.buf == other.buf && Rc::ptr_eq(&self.i, &other.i)
    }
}
impl Eq for Reader {}
impl_value!(Reader);

impl Reader {
    pub fn new() -> Reader {
        let buf = StringBuffer::default();
        let buf_reader = buf.clone();
        let mut i = Interpreter::default();
        i.eval_next(|i: Handle| async move {
            ReaderHandle::new(i, buf_reader).run().await;
        });
        Reader { buf, i: Rc::new(RefCell::new(i)), }
    }
    pub fn write(&mut self, src: &mut impl Iterator<Item=char>) {
        self.buf.write(src);
    }
    pub fn set_eof(&mut self) {
        self.buf.set_eof();
    }
    pub fn is_eof(&mut self) -> bool {
        self.buf.is_eof()
    }
    pub fn next(&mut self) -> Result<Option<Val>, ReadError> {
        self.i.borrow_mut().run();
        match self.i.borrow_mut().stack_pop_val() {
            None => Ok(None), // maybe?
            Some(v) => match v.downcast::<Emit>() {
                Ok(Emit::Eof) => Ok(None),
                Ok(Emit::Yield(v)) => Ok(Some(v)),
                Ok(Emit::Error(e)) => Err(e),
                Err(e) => { dbg!("", e); Ok(None) },
            }
        }
    }
}

pub fn read_all(src: &mut impl Iterator<Item=char>) -> Result<Vec<Val>, ReadError> {
    let mut reader = Reader::new();
    reader.write(src);
    reader.set_eof();
    let mut acc = vec![];
    while let Some(v) = reader.next()? { acc.push(v); }
    Ok(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    // assert nothing trailing here?
    fn vec_read(s: &str) -> Vec<Val> {
        read_all(&mut s.chars()).unwrap()
        // Vec::from_iter(Reader::from(s))
    }

    #[test]
    fn read_none() {
        assert_eq!(vec_read(""), vec![]);
        assert_eq!(vec_read(" \n ; test\n"), vec![]);
    }

    #[test]
    fn read_bool() {
        assert_eq!(vec_read("  #t "), vec![true.into()]);
        assert_eq!(vec_read("#f#t ;yeah\n #f #t "),
                    Vec::from_iter([false, true, false, true].map(Val::from)));
    }

    #[test]
    fn read_string() {
        assert_eq!(vec_read("\"egg\" \"blub\\nbo\"\"\" \"ok\\\"ok\""),
                    Vec::from_iter(["egg", "blub\nbo", "", "ok\"ok"]
                                   .map(String::from)
                                   .map(Val::from)));
    }

    #[test]
    fn read_i32() {
        assert_eq!(vec_read("123"), vec![123.into()]);
        assert_eq!(vec_read("12#t34"), vec![12.into(), true.into(), 34.into()]);
    }

    #[test]
    fn read_symbol() {
        assert_eq!(vec_read("eggs"), vec!["eggs".to_symbol().into()]);
        assert_eq!(vec_read("time for-some\n.cool.beans"),
                    Vec::from_iter(["time", "for-some", ".cool.beans"]
                                   .map(|x| x.to_symbol().into())));
    }

    #[test]
    fn read_list() {
        assert_eq!(vec_read("bean (bag muffins) ok{}[y(e p)s]"),
            vec!["bean".to_symbol().into(),
                List::from(vec![
                    "bag".to_symbol().into(),
                    "muffins".to_symbol().into(),
                ]).into(),
                "ok".to_symbol().into(),
                List::default().into(),
                List::from(vec![
                    "y".to_symbol().into(),
                    List::from(vec![
                        "e".to_symbol().into(),
                        "p".to_symbol().into(),
                    ]).into(),
                    "s".to_symbol().into(),
                ]).into()]);
    }

}

