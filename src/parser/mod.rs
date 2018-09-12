
use std::collections::VecDeque;
use data::*;

mod data;
mod reader;
mod error;
mod token;

pub use self::data::*;
pub use self::reader::*;
pub use self::error::*;
pub use self::token::*;

#[derive(Debug)]
pub struct Parser {
    pos: Source,
    state: Symbol,
    tokens: VecDeque<Token>,
    input: VecDeque<char>,
    eof: bool,
}

impl Parser {
    pub fn new(pos: Source, reader: &Reader) -> Self {
        Parser {
            pos,
            state: reader.init_state().clone(),
            tokens: VecDeque::new(),
            input: VecDeque::new(),
            eof: false,
        }
    }

    fn is_next_parsed(&self) -> Option<bool> {
        if self.tokens.len() > 0 {
            Some(self.tokens[0].is_parsed())
        } else {
            None
        }
    }

    pub fn unfinished(&self) -> Vec<&str> {
        // TODO filter unfinished tokens and collect the tags
        self.tokens.iter().filter_map(|fm|
            match fm {
                Token::Parsed(_) => None,
                Token::Unparsed(ref u) => {
                    match &u.tag {
                        Some(ref s) => Some(s.as_str()),
                        None => None,
                    }
                },
            }).collect()
    }

    fn next_parsed(&mut self) -> Option<Datum> {
        if self.tokens.len() > 0 && self.tokens[0].is_parsed() {
            match self.tokens.pop_front() {
                None => None,
                Some(Token::Parsed(d)) => Some(d),
                Some(x) => {
                    self.tokens.push_front(x);
                    None
                },
            }
        } else {
            None
        }
    }

    pub fn is_eof(&self) -> bool {
        self.eof && self.input.len() == 0 && self.tokens.len() == 0
    }

    pub fn set_eof(&mut self, eof: bool) {
        self.eof = eof;
    }

    pub fn push_input(&mut self, input: &mut Iterator<Item=char>) {
        // TODO fix: you can still push input after EOF
        while let Some(c) = input.next() {
            self.input.push_back(c);
        }
    }

    pub fn read(&mut self, input: char) {
        self.input.push_back(input);
    }

    fn remaining_input(&self) -> String {
        let mut s = String::new();
        for ch in self.input.iter() {
            s.push(*ch);
        }
        s
    }

    pub fn read_next(&mut self, reader: &Reader) -> Result<Option<Datum>, ParseError> {
        trace!("read_next eof={} input={} {:?} tokens={}", self.eof, self.input.len(), self.remaining_input(), self.tokens.len());
        loop {
            if let Some(d) = self.next_parsed() {
                return Ok(Some(d));
            }
            match self.input.pop_front() {
                Some(c) => {
                    self.consume_char(reader, c)?;
                },
                None => {
                    if self.eof {
                        self.consume_char(reader, None)?;
                        if let Some(false) = self.is_next_parsed() {
                            return Err(ParseError::unparsed_token(self.pos.clone()));
                        } else {
                            return Ok(self.next_parsed());
                        }
                    } else {
                        return Ok(None);
                    }
                },
            }
        }
    }

    fn consume_char<C: Into<Option<char>>>(&mut self, reader: &Reader, c: C) -> Result<(), ParseError> {
        let c = c.into();
        match reader.matching(&self.state, c) {
            None => Err(ParseError::no_matching_state(self.pos.clone()))?,
            Some(arm) => {
                for i in arm.instructions.iter() {
                    self.execute(&i, c)?;
                }
            },
        }
        if let Some(cc) = c {
            // TODO split char into char/eof, not option
            self.pos.read(cc);
        }
        Ok(())
    }

    fn finish_token(&mut self, tok: UnparsedToken) -> Result<(), ParseError> {
        match tok.ty.ok_or_else(|| ParseError::no_token_type(self.pos.clone()))? {
            TokenType::Symbol => {
                let d = Datum::build().with_source(self.pos.clone()).symbol(tok.value);
                self.tokens.push_back(Token::Parsed(d));
            },
            TokenType::String => {
                let d = Datum::build().with_source(self.pos.clone()).ok::<String>(tok.value);
                self.tokens.push_back(Token::Parsed(d));
            },
            TokenType::Rational => {
                let v = tok.value;
                let d =
                    Datum::build()
                    .with_source(self.pos.clone())
                    .parse::<Number>(v.as_str())
                    .ok_or_else(|| ParseError::invalid_format(self.pos.clone()))?;
                self.tokens.push_back(Token::Parsed(d));
            },
            TokenType::StartList => {
                self.tokens.push_back(Token::Unparsed(tok));
            },
            TokenType::EndList => {
                let mut elements = vec![];
                loop {
                    match self.tokens.pop_back() {
                        None =>
                            Err(ParseError::unmatched_list_end(self.pos.clone()))?,
                        Some(Token::Parsed(t)) => {
                            elements.push(t);
                        },
                        Some(Token::Unparsed(t)) => {
                            if t.ty != Some(TokenType::StartList) || tok.tag != t.tag {
                                Err(ParseError::unfinished_token(self.pos.clone()))?;
                            }
                            elements.reverse();
                            let d = Datum::build()
                                .with_source(self.pos.clone())
                                .ok::<List>(elements.into());
                            self.tokens.push_back(Token::Parsed(d));
                            break;
                        },
                    }
                }
            },
        }
        self.pos.set_ended();
        Ok(())
    }

    fn execute(&mut self, i: &ReaderInstruction, c: Option<char>) -> Result<(), ParseError> {
        use self::ReaderCommand::*;
        debug!("Execute {:?} {:?} {:?}", c, i, self.state);
        match &i.command {
            &SetState(ref state) => {
                self.state = state.clone();
            }
            &StartToken => {
                self.tokens.push_back(Token::new());
                self.pos.set_started();
            },
            &SetTokenTag(ref tag) => {
                match self.tokens.pop_back() {
                    Some(Token::Unparsed(mut tok)) => {
                        tok.set_tag(tag.clone());
                        self.tokens.push_back(Token::Unparsed(tok));
                    },
                    _ => Err(ParseError::token_not_in_progress(self.pos.clone()))?,
                }
            },
            &SetTokenType(ref ty) => {
                match self.tokens.pop_back() {
                    Some(Token::Unparsed(mut tok)) => {
                        tok.set_type(ty.clone());
                        self.tokens.push_back(Token::Unparsed(tok));
                    },
                    _ => Err(ParseError::token_not_in_progress(self.pos.clone()))?,
                }
            },
            &AppendToken => {
                match self.tokens.pop_back() {
                    Some(Token::Unparsed(mut tok)) => {
                        let cc = c.ok_or(ParseError::appending_eof(self.pos.clone()))?;
                        tok.value.push(cc);
                        self.tokens.push_back(Token::Unparsed(tok));
                    },
                    _ => Err(ParseError::token_not_in_progress(self.pos.clone()))?,
                }
            },
            &FinishToken => {
                if let Some(tok) = self.tokens.pop_back() {
                    match tok {
                        Token::Unparsed(mut t) => {
                            self.finish_token(t)?;
                        },
                        _ => self.tokens.push_back(tok),
                    }
                }
            },
            &PrependDatum(ref d) => {
                self.tokens.push_back(Token::Parsed(d.clone()));
            },
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        // ???
        self.tokens.clear();
        self.input.clear();
        self.eof = false;
    }

    pub fn current_position(&self) -> &Source {
        &self.pos
    }
    pub fn set_position(&mut self, s: Source) {
        self.pos = s;
    }

    pub fn set_file<F: Into<Option<String>>>(&mut self, file: F) {
        self.pos = Source::new().with_file(file);
    }

    pub fn current_source(&self) -> (Option<String>, usize, usize) {
        self.pos.current_position()
    }

}

