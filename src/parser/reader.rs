
use data::*;
use combo::*;

use parser::token::*;
use parser::data::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ReaderInstruction {
    pub command: ReaderCommand,
}

impl ReaderInstruction {
    fn command(command: ReaderCommand) -> Self {
        ReaderInstruction {
            command,
        }
    }

    pub fn set_state<S: Into<Symbol>>(state: S) -> Self {
        ReaderInstruction {
            command: ReaderCommand::SetState(state.into()),
        }
    }

    pub fn set_tag<T: Into<String>>(t: T) -> Self {
        Self::command(ReaderCommand::SetTokenTag(t.into()))
    }

    pub fn set_type(ty: TokenType) -> Self {
        Self::command(ReaderCommand::SetTokenType(ty))
    }

    pub fn start_token() -> Self {
        Self::command(ReaderCommand::StartToken)
    }

    pub fn append_token() -> Self {
        Self::command(ReaderCommand::AppendToken)
    }

    pub fn finish_token() -> Self {
        Self::command(ReaderCommand::FinishToken)
    }

    pub fn prepend_datum(d: Datum) -> Self {
        Self::command(ReaderCommand::PrependDatum(d))
    }

}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ReaderArm {
    pub name: String,
    pub accept_state: AcceptState,
    pub accept_char: AcceptChar,
    pub instructions: Vec<ReaderInstruction>,
}

#[derive(Debug)]
pub struct Reader {
    init_state: Symbol,
    rules: Vec<ReaderArm>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ReaderCommand {
    SetState(Symbol),
    StartToken,
    SetTokenTag(String),
    SetTokenType(TokenType),
    AppendToken,
    FinishToken,
    PrependDatum(Datum),
}

impl ReaderArm {
    pub fn new<S: Into<String>>(name: S) -> Self {
        ReaderArm {
            name: name.into(),
            accept_char: Combo::Nothing,
            accept_state: Combo::Anything,
            instructions: vec![],
        }
    }

    pub fn set_accept_input(&mut self, chars: AcceptChar) {
        self.accept_char = chars;
    }
    pub fn accept_input(mut self, chars: AcceptChar) -> Self {
        self.set_accept_input(chars);
        self
    }

    pub fn set_accept_state(&mut self, state: AcceptState) {
        self.accept_state = state;
    }
    pub fn accept_state(mut self, state: AcceptState) -> Self {
        self.set_accept_state(state);
        self
    }

    pub fn push_run(&mut self, i: ReaderInstruction) {
        self.instructions.push(i);
    }

    pub fn run(mut self, i: ReaderInstruction) -> Self {
        self.push_run(i);
        self
    }

    pub fn accepts(&self, state: &Symbol, ch: Option<char>) -> bool {
        if !self.accept_state.contains(&state) { return false; }
        if let Some(c) = ch {
            if self.accept_char.contains(&CharClass::One(c)) { return true; }
            if c.is_whitespace() && self.accept_char.contains(&CharClass::Whitespace) { return true; }
            if c.is_alphabetic() && self.accept_char.contains(&CharClass::Alpha) { return true; }
            if c.is_numeric() && self.accept_char.contains(&CharClass::Numeric) { return true; }
            if c.is_ascii_punctuation() && self.accept_char.contains(&CharClass::Symbol) { return true; }
        } else {
            if self.accept_char.contains(&CharClass::Eof) { return true; }
        }
        false
    }
}

impl Reader {
    pub fn new<S: Into<Symbol>>(state: S, rules: Vec<ReaderArm>) -> Self {
        Reader { init_state: state.into(), rules }
    }

    pub fn matching(&self, state: &Symbol, c: Option<char>) -> Option<&ReaderArm> {
        for arm in self.rules.iter() {
            if arm.accepts(&state, c) {
                return Some(arm);
            }
        }
        None
    }

    pub fn add_rule(&mut self, rule: ReaderArm) {
        self.rules.insert(0, rule);
    }

    pub fn delete_rule(&mut self, name: &String) {
        let idx = self.rules.iter().position(|arm| &arm.name == name);
        if let Some(idx) = idx {
            self.rules.remove(idx);
        }
    }

    pub fn init_state(&self) -> &Symbol {
        &self.init_state
    }

}


