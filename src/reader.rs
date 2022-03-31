
use std::io as io;
use crate::base::*;
use crate::list::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ReadError {
    UnclosedString,
    UnmatchedHash,
    UnknownHash(char),
    UnmatchedList(char),
    UnknownChar(char),
    UnparseableNumber(String),
    IoError(String),
}

type Res<T> = Result<T, ReadError>;

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> Self {
        ReadError::IoError(format!("{:?}", e.kind()))
    }
}

fn read_hash(src: &mut impl Iterator<Item=io::Result<char>>) -> Res<Val> {
    match src.next().transpose()? {
        Some('t') => Ok(true.into()),
        Some('f') => Ok(false.into()),
        Some(c) => Err(ReadError::UnknownHash(c)),
        None => Err(ReadError::UnmatchedHash),
    }
}

fn read_string(src: &mut impl Iterator<Item=io::Result<char>>) -> Res<String> {
    // single-char escape for now
    let mut acc = String::new();
    let mut escaping = false;
    while let Some(c) = src.next().transpose()? {
        if escaping {
            escaping = false;
            acc.push(match c {
                'n' => '\n',
                'e' => '\u{1b}', // escape
                c => c, // includes \ and "
            });
        } else {
            match c {
                '"' => return Ok(acc),
                '\\' => escaping = true,
                c => acc.push(c),
            }
        }
    }
    Err(ReadError::UnclosedString)
}

fn read_i32(start: char, src: &mut impl Iterator<Item=io::Result<char>>) -> Res<(i32, Option<char>)> {
    // maybe just take_while instead
    let mut acc = String::from(start);
    let cr = loop {
        match src.next().transpose()? {
            Some(c) if c.is_numeric() => {
                acc.push(c);
            },
            cr => break cr,
        }
    };

    if let Ok(v) = str::parse::<i32>(&acc) {
        Ok((v, cr))
    } else { Err(ReadError::UnparseableNumber(acc)) }
}

fn read_list(open: char, src: &mut impl Iterator<Item=io::Result<char>>) -> Res<Vec<Val>> {
    let endch = match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => return Err(ReadError::UnmatchedList(open)),
    };
    read_until(Some(endch), src)
}
fn read_symbol(start: char, src: &mut impl Iterator<Item=io::Result<char>>) -> Res<(Symbol, Option<char>)> {
    let mut acc = String::from(start);
    let next = loop {
        match src.next().transpose()? {
            c@(Some('('|')' | '['|']' | '{'|'}' | '"') | None) => break c,
            c@Some(s) if s.is_whitespace() => break c,
            Some(c) => acc.push(c),
        }
    };
    Ok((acc.to_symbol(), next))
}

fn read_until(until: Option<char>, src: &mut impl Iterator<Item=io::Result<char>>) -> Res<Vec<Val>> {
    let mut buf = vec![];
    let mut next = None;
    while let Some(c) = next.take().map(Result::Ok).or_else(|| src.next()).transpose()? {
        match c {
            ';' => {
                while match src.next().transpose()? {
                    None | Some('\n') => false, _ => true
                } {}
            },
            '#' => buf.push(read_hash(src)?),
            '"' => buf.push(read_string(src)?.into()),
            '(' | '{' | '[' => buf.push(List::from(read_list(c, src)?).into()),
            c =>
                if c.is_whitespace() {}
                else if c.is_numeric() {
                    let (d, n) = read_i32(c, src)?;
                    next = n;
                    buf.push(d.into());
                }
                else if Some(c) == until { return Ok(buf); }
                else {
                    let (d, n) = read_symbol(c, src)?;
                    next = n;
                    buf.push(d.into());
                }
        }
    }
    Ok(buf)
}

pub fn read_all(src: &mut impl Iterator<Item=io::Result<char>>) -> Res<Vec<Val>> {
    read_until(None, src)
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

