
use std::fmt;
use crate::data::error::*;
use crate::parser::data::Source;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum ParseErrorReason {
    /// Got into a state with no matching rules
    NoMatchingState,
    /// Token not finished before starting a new one
    UnfinishedToken,
    /// Top token in output is not Unparsed and in-progress
    TokenNotInProgress,
    /// There are still Unparsed tokens but shouldn't be
    UnparsedToken,
    /// Tried to execute AppendToken on Eof
    AppendingEof,
    /// Found a list end without a corresponding start
    UnmatchedListEnd,
    /// Tried to read e.g. a non-numeric atom as a number
    InvalidFormat,
    /// Tried to parse a token with no type set.
    NoTokenType,
}

#[derive(Debug)]
pub struct ParseError {
    reason: ParseErrorReason,
    source: Source,
}

impl ParseError {
    fn new(reason: ParseErrorReason, source: Source) -> Self {
        ParseError { reason, source, }
    }
    pub fn no_matching_state(source: Source) -> Self {
        ParseError::new(ParseErrorReason::NoMatchingState, source)
    }
    pub fn unfinished_token(source: Source) -> Self {
        ParseError::new(ParseErrorReason::UnfinishedToken, source)
    }
    pub fn token_not_in_progress(source: Source) -> Self {
        ParseError::new(ParseErrorReason::TokenNotInProgress, source)
    }
    pub fn unparsed_token(source: Source) -> Self {
        ParseError::new(ParseErrorReason::UnparsedToken, source)
    }
    pub fn appending_eof(source: Source) -> Self {
        ParseError::new(ParseErrorReason::AppendingEof, source)
    }
    pub fn unmatched_list_end(source: Source) -> Self {
        ParseError::new(ParseErrorReason::UnmatchedListEnd, source)
    }
    pub fn invalid_format(source: Source) -> Self {
        ParseError::new(ParseErrorReason::InvalidFormat, source)
    }
    pub fn no_token_type(source: Source) -> Self {
        ParseError::new(ParseErrorReason::NoTokenType, source)
    }
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Parser error: {:?} at {}", self.reason, self.source)
    }
}


