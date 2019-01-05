
use crate::data::*;

// TODO remove this and have start_list, end_list, start_atom
// then give interpreter a hash map of token tag => reader function
// so e.g. in order to read numbers, boot script does something like
// [symbol->string parse-rational] quote rational set-token-reader
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TokenType {
    Symbol,
    String,
    Rational,
    StartList,
    EndList,
}

#[derive(Debug, Default)]
pub struct UnparsedToken {
    pub ty: Option<TokenType>,
    pub tag: Option<String>,
    pub value: String,
}

impl UnparsedToken {
    pub fn set_tag<T: Into<Option<String>>>(&mut self, tag: T) {
        self.tag = tag.into();
    }
    pub fn set_type<T: Into<Option<TokenType>>>(&mut self, ty: T) {
        self.ty = ty.into();
    }
}

#[derive(Debug)]
pub enum Token {
    Parsed(Datum),
    Unparsed(UnparsedToken),
}

impl Token {
    pub fn new() -> Self {
        Token::Unparsed(UnparsedToken::default())
    }
    pub fn is_parsed(&self) -> bool {
        match self {
            &Token::Parsed(_) => true,
            &Token::Unparsed(_) => false,
        }
    }
}


