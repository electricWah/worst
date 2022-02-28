
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
}

fn read_hash(src: &mut impl Iterator<Item=char>) -> Result<Val, ReadError> {
    match src.next() {
        Some('t') => Ok(true.into()),
        Some('f') => Ok(false.into()),
        Some(c) => Err(ReadError::UnknownHash(c)),
        None => Err(ReadError::UnmatchedHash),
    }
}

fn read_string(src: &mut impl Iterator<Item=char>) -> Result<String, ReadError> {
    // single-char escape for now
    let mut acc = String::new();
    let mut escaping = false;
    while let Some(c) = src.next() {
        if escaping {
            escaping = false;
            acc.push(match c {
                'n' => '\n',
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

fn read_i32(start: char, src: &mut impl Iterator<Item=char>) -> Result<(i32, Option<char>), ReadError> {
    // maybe just take_while instead
    let mut acc = String::from(start);
    let cr = loop {
        match src.next() {
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

fn read_list(open: char, src: &mut impl Iterator<Item=char>) -> Result<Vec<Val>, ReadError> {
    let endch = match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => return Err(ReadError::UnmatchedList(open)),
    };
    read_until(Some(endch), src)
}
fn read_symbol(start: char, src: &mut impl Iterator<Item=char>) -> (Symbol, Option<char>) {
    let mut acc = String::from(start);
    let next = loop {
        match src.next() {
            c@(Some('('|')' | '['|']' | '{'|'}' | '"') | None) => break c,
            c@Some(s) if s.is_whitespace() => break c,
            Some(c) => acc.push(c),
        }
    };
    (Symbol::new(acc), next)
}

fn read_until(until: Option<char>, src: &mut impl Iterator<Item=char>) -> Result<Vec<Val>, ReadError> {
    let mut buf = vec![];
    let mut next = None;
    while let Some(c) = next.take().or_else(|| src.next()) {
        match c {
            ';' => { while src.next() != Some('\n') {} },
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
                    let (d, n) = read_symbol(c, src);
                    next = n;
                    buf.push(d.into());
                }
        }
    }
    Ok(buf)
}

pub fn read_all(src: &mut impl Iterator<Item=char>) -> Result<Vec<Val>, ReadError> {
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
                                   .map(ToString::to_string)
                                   .map(Val::from)));
    }

    #[test]
    fn read_i32() {
        assert_eq!(vec_read("123"), vec![123.into()]);
        assert_eq!(vec_read("12#t34"), vec![12.into(), true.into(), 34.into()]);
    }

    #[test]
    fn read_symbol() {
        assert_eq!(vec_read("eggs"), vec![Symbol::new("eggs").into()]);
        assert_eq!(vec_read("time for-some\n.cool.beans"),
                    Vec::from_iter(["time", "for-some", ".cool.beans"]
                                   .map(|x| Symbol::new(x).into())));
    }

    #[test]
    fn read_list() {
        assert_eq!(vec_read("bean (bag muffins) ok{}[y(e p)s]"),
            vec![Symbol::new("bean").into(),
                List::from(vec![
                    Symbol::new("bag").into(),
                    Symbol::new("muffins").into(),
                ]).into(),
                Symbol::new("ok").into(),
                List::default().into(),
                List::from(vec![
                    Symbol::new("y").into(),
                    List::from(vec![
                        Symbol::new("e").into(),
                        Symbol::new("p").into(),
                    ]).into(),
                    Symbol::new("s").into(),
                ]).into()]);
    }

}

