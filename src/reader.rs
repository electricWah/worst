
use crate::base::*;
use crate::list::*;
use std::collections::VecDeque;

struct PeekManyable<T: Iterator> {
    // peeked is basically base::Stack?
    peeked: VecDeque<T::Item>,
    source: T,
}

impl<T: Iterator> PeekManyable<T> {
    fn new(source: T) -> Self { Self { peeked: VecDeque::new(), source } }

    fn peek(&mut self) -> Option<&T::Item> {
        if self.peeked.len() == 0 {
            if let Some(v) = self.source.next() {
                self.peeked.push_back(v);
            } else { return None; }
        }
        self.peeked.front()
    }

    // fn unpeek(&mut self, v: T::Item) { self.peeked.push_front(v); }

    fn peek_at(&mut self, n: usize) -> Option<&T::Item> {
        for _ in self.peeked.len() ..= n {
            if let Some(next) = self.source.next() {
                self.peeked.push_back(next);
            } else { return None; }
        }
        self.peeked.get(n)
    }

    fn peek_n(&mut self, n: usize) -> Option<&[T::Item]> {
        if self.peek_at(n).is_none() { return None; }
        self.peeked.make_contiguous();
        Some(&self.peeked.as_slices().0[..n])
    }

    fn peek_while<F: Fn(&T::Item) -> bool> (&mut self, f: F) -> &[T::Item] {
        let mut len = 0;
        loop {
            if let Some(v) = self.peek_at(len) {
                if f(v) { len = len + 1; } else { break; }
            } else {
                break;
            }
        }
        self.peeked.make_contiguous();
        &self.peeked.as_slices().0[..len]
    }

    fn drop_n(&mut self, n: usize) -> usize {
        for i in 0 .. n {
            if self.next().is_none() { return i; }
        }
        return n;
    }
}

impl<T: Iterator> Iterator for PeekManyable<T> {
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.peeked.pop_front().or_else(|| self.source.next())
    }
}

#[cfg(test)]
mod peekmanyable_tests {
    use super::PeekManyable;
    #[test]
    fn peek_manyable() {
        let mut p = PeekManyable::new(Box::new(vec![1, 2, 3, 4, 5].into_iter()));
        assert_eq!(p.peek(), Some(&1));
        assert_eq!(p.peek(), Some(&1));
        assert_eq!(p.peek_n(2), Some(&vec![1, 2][..])); //slice lol
        assert_eq!(p.peek(), Some(&1));
        assert_eq!(p.peek_n(3), Some(&vec![1, 2, 3][..]));
        assert_eq!(p.peek_n(3), Some(&vec![1, 2, 3][..]));
        assert_eq!(p.peek(), Some(&1));
        assert_eq!(p.peek_at(1), Some(&2));
        assert_eq!(p.peek_at(2), Some(&3));
        assert_eq!(p.peek_at(0), Some(&1));
        assert_eq!(p.peek_at(3), Some(&4));
        assert_eq!(p.next(), Some(1));
        assert_eq!(p.peek(), Some(&2));
        assert_eq!(p.peek_while(|_| false), &vec![][..]);
        assert_eq!(p.peek_while(|x| *x < 5), &vec![2, 3, 4][..]);
        assert_eq!(p.next(), Some(2));
        assert_eq!(p.next(), Some(3));
        assert_eq!(p.next(), Some(4));
        assert_eq!(p.next(), Some(5));
    }
}

struct Reader {
    source: PeekManyable<Box<dyn Iterator<Item = char>>>,
}

impl<T: AsRef<str>> From<T> for Reader {
    fn from(s: T) -> Reader {
        let chars: Vec<char> = s.as_ref().chars().collect();
        Reader { source: PeekManyable::new(Box::new(chars.into_iter())) }
    }
}

impl From<Reader> for String {
    fn from(r: Reader) -> String { r.source.collect() }
}

impl Reader {

    fn drop_blanks(&mut self) {
        'blanks: loop {
            let mut did_stuff = false;
            'whitespace: while let Some(c) = self.source.peek() {
                if c.is_whitespace() {
                    did_stuff = true;
                    self.source.next();
                } else {
                    break 'whitespace;
                }
            }
            // single-line comment
            if let Some(';') = self.source.peek() {
                did_stuff = true;
                'until_newline: while let Some(c) = self.source.next() {
                    if c == '\n' {
                        break 'until_newline;
                    }
                }
            }
            if !did_stuff { break 'blanks; }
        }
    }

    fn read_bool(&mut self) -> Option<Val> {
        match self.source.peek_n(2) {
            Some(['#', 't']) => {
                self.source.drop_n(2);
                Some(true.into())
            },
            Some(['#', 'f']) => {
                self.source.drop_n(2);
                Some(false.into())
            },
            _ => None,
        }
    }

    fn read_list(&mut self) -> Option<Val> {
        let endch =
            match self.source.peek() {
                Some('(') => ')',
                Some('[') => ']',
                Some('{') => '}',
                _ => return None,
            };
        let _startch = self.source.next();
        let mut acc = vec![];
        loop {
            self.drop_blanks();
            match self.source.peek() {
                Some(c) if c == &endch => {
                    let _endch = self.source.next();
                    return Some(List::from(acc).into());
                }
                None => return None,
                _ => {},
            }

            match self.read_val() {
                Some(v) => acc.push(v),
                None => return None,
            }
        }
    }

    fn read_string(&mut self) -> Option<Val> {
        // currently take until "
        // but maybe instead peek() to show unmatched start location
        if self.source.peek() != Some(&'"') { return None; }
        let _dq = self.source.next();
        let mut acc = vec![];
        // single-char escape for now
        let mut escaping = false;
        while let Some(c) = self.source.next() {
            if escaping {
                escaping = false;
                acc.push(match c {
                    'n' => '\n',
                    c => c, // includes \ and "
                });
            } else {
                match c {
                    '"' => return Some(acc.into_iter().collect::<String>().into()),
                    '\\' => escaping = true,
                    c => acc.push(c),
                }
            }
        }
        None
    }

    fn read_i32(&mut self) -> Option<Val> {
        // maybe just take_while instead
        let nums = self.source.peek_while(|x| x.is_numeric());
        let len = nums.len();
        if len > 0 {
            if let Ok(v) = str::parse::<i32>(&nums.iter().collect::<String>()) {
                self.source.drop_n(len);
                Some(v.into())
            } else { None }
        } else { None }
    }

    fn read_symbol(&mut self) -> Option<Val> {
        let mut src = "".to_string();
        'symbol: loop {
            match self.source.peek() {
                Some('('|')' | '['|']' | '{'|'}' | '"') | None => break 'symbol,
                Some(c) if c.is_whitespace() => break 'symbol,
                Some(c) => {
                    src.push(*c);
                    let _ = self.source.next();
                },
            }
        };
        if src.len() > 0 {
            Some(Symbol::new(src).into())
        } else { None }
    }

    fn read_val(&mut self) -> Option<Val> {
        self.drop_blanks();
        if let v@Some(_) = self.read_bool() { v }
        else if let v@Some(_) = self.read_list() { v }
        else if let v@Some(_) = self.read_string() { v }
        else if let v@Some(_) = self.read_i32() { v }
        else if let v@Some(_) = self.read_symbol() { v }
        else { None }
    }

}

impl Iterator for Reader {
    type Item = Val;
    fn next(&mut self) -> Option<Self::Item> { self.read_val() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_blanks() {
        let mut r: Reader = "".into();
        r.drop_blanks();
        assert_eq!(String::from(r), "");

        r = " ; test \n ok".into();
        r.drop_blanks();
        assert_eq!(String::from(r), "ok");
    }

    // assert nothing trailing here?
    fn vec_read(s: &str) -> Vec<Val> { Vec::from_iter(Reader::from(s)) }

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

