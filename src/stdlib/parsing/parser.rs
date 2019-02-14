
use crate::parser::*;
use crate::data::*;
use crate::data::error;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;
use crate::stdlib::combo::ComboValue;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<ReaderArm>("parser-rule?");
    interpreter.add_builtin("parser-new-rule", parser_new_rule);
    interpreter.add_builtin("parser-accept-input", parser_accept_input);
    interpreter.add_builtin("parser-accept-state", parser_accept_state);
    interpreter.add_builtin("parser-set-state", parser_set_state);
    interpreter.add_builtin("parser-start-token", parser_start_token);
    interpreter.add_builtin("parser-set-token-tag", parser_set_token_tag);
    interpreter.add_builtin("parser-set-token-type", parser_set_token_type);
    interpreter.add_builtin("parser-append-token", parser_append_token);
    interpreter.add_builtin("parser-finish-token", parser_finish_token);
    interpreter.add_builtin("parser-prepend-datum", parser_prepend_datum);
    interpreter.add_builtin("parser-save-rule", parser_save_rule);
    interpreter.add_builtin("parser-delete-rule", parser_delete_rule);
}

fn parser_new_rule(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<String>()?;
    let arm = ReaderArm::new(name);
    interpreter.stack.push(Datum::build().with_source(source).ok(arm));
    Ok(())
}

fn parser_accept_input(interpreter: &mut Interpreter) -> exec::Result<()> {
    let class = interpreter.stack.pop::<ComboValue>()?.into_combo::<CharClass>()?;
    let arm = interpreter.stack.top_mut::<ReaderArm>()?;
    arm.set_accept_input(class);
    Ok(())
}

fn parser_accept_state(interpreter: &mut Interpreter) -> exec::Result<()> {
    let combo = interpreter.stack.pop::<ComboValue>()?.into_combo::<Symbol>()?;
    let arm = interpreter.stack.top_mut::<ReaderArm>()?;
    arm.set_accept_state(combo);
    Ok(())
}

fn parser_set_state(interpreter: &mut Interpreter) -> exec::Result<()> {
    let state = interpreter.stack.pop::<Symbol>()?;
    let arm = interpreter.stack.top_mut::<ReaderArm>()?;
    arm.push_run(ReaderInstruction::set_state(state));
    Ok(())
}

fn parser_start_token(interpreter: &mut Interpreter) -> exec::Result<()> {
    let arm = interpreter.stack.top_mut::<ReaderArm>()?;
    arm.push_run(ReaderInstruction::start_token());
    Ok(())
}

fn parser_set_token_tag(interpreter: &mut Interpreter) -> exec::Result<()> {
    let tag = interpreter.stack.pop::<String>()?;
    let arm = interpreter.stack.top_mut::<ReaderArm>()?;
    arm.push_run(ReaderInstruction::set_tag(tag));
    Ok(())
}

fn parser_set_token_type(interpreter: &mut Interpreter) -> exec::Result<()> {
    let state = interpreter.stack.pop::<Symbol>()?;
    let ty = TokenType::from_symbol(state)?;
    let arm = interpreter.stack.top_mut::<ReaderArm>()?;
    arm.push_run(ReaderInstruction::set_type(ty));
    Ok(())
}

fn parser_append_token(interpreter: &mut Interpreter) -> exec::Result<()> {
    let arm = interpreter.stack.top_mut::<ReaderArm>()?;
    arm.push_run(ReaderInstruction::append_token());
    Ok(())
}

fn parser_finish_token(interpreter: &mut Interpreter) -> exec::Result<()> {
    let arm = interpreter.stack.top_mut::<ReaderArm>()?;
    arm.push_run(ReaderInstruction::finish_token());
    Ok(())
}

fn parser_prepend_datum(interpreter: &mut Interpreter) -> exec::Result<()> {
    let d = interpreter.stack.pop_datum()?;
    let arm = interpreter.stack.top_mut::<ReaderArm>()?;
    arm.push_run(ReaderInstruction::prepend_datum(d));
    Ok(())
}

fn parser_save_rule(interpreter: &mut Interpreter) -> exec::Result<()> {
    let arm = interpreter.stack.pop::<ReaderArm>()?;
    interpreter.reader_mut().add_rule(arm);
    Ok(())
}

fn parser_delete_rule(interpreter: &mut Interpreter) -> exec::Result<()> {
    Err(error::NotImplemented().into())
}

impl StaticType for ReaderArm {
    fn static_type() -> Type {
        Type::new("reader-arm")
    }
}
impl ValueShow for ReaderArm {}
impl ValueDebugDescribe for ReaderArm {}
impl ValueHash for ReaderArm {}
impl DefaultValueEq for ReaderArm {}
impl DefaultValueClone for ReaderArm {}
impl Value for ReaderArm {}

impl TokenType {
    fn from_symbol(sym: Symbol) -> exec::Result<Self> {
        match sym.as_ref() {
            "symbol" => Ok(TokenType::Symbol),
            "string" => Ok(TokenType::String),
            "rational" => Ok(TokenType::Rational),
            "start-list" => Ok(TokenType::StartList),
            "end-list" => Ok(TokenType::EndList),
            _ => Err(error::NotDefined().into()),
        }
    }
}

