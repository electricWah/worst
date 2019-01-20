
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

use super::data::*;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<Record>("record?");
    interpreter.define_type_predicate::<RecordType>("record-type?");
    interpreter.add_builtin("make-record-type", make_record_type);
    interpreter.add_builtin("record-type-name", record_type_name);
    interpreter.add_builtin("make-record", make_record);
    interpreter.add_builtin("record-slot-count", record_slot_count);
    interpreter.add_builtin("record-slot-add", record_slot_add);
    interpreter.add_builtin("record-slot-swap", record_slot_swap);
    interpreter.add_builtin("record->list", record_into_list);
    interpreter.add_builtin("record-type", record_type);
}

fn make_record_type(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    let t = RecordType::new(name.to_string());
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(t));
    Ok(())
}

fn record_type_name(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = {
        let ty = interpreter.stack.ref_at::<RecordType>(0)?;
        ty.name().to_string()
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(name));
    Ok(())
}

fn make_record(interpreter: &mut Interpreter) -> exec::Result<()> {
    let ty = interpreter.stack.pop::<RecordType>()?;
    let rec = Record::new(ty);
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(rec));
    Ok(())
}

fn record_slot_count(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = interpreter.stack.ref_at::<Record>(0)?.slot_count();
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(r)));
    Ok(())
}

fn record_slot_add(interpreter: &mut Interpreter) -> exec::Result<()> {
    let d = interpreter.stack.pop_datum()?;
    let rec = interpreter.stack.top_mut::<Record>()?;
    rec.slot_add(d);
    Ok(())
}

fn record_slot_swap(interpreter: &mut Interpreter) -> exec::Result<()> {
    let idx = interpreter.stack.pop::<Number>()?.cast::<usize>()?;
    let d = {
        let d = interpreter.stack.pop_datum()?;
        let rec = interpreter.stack.top_mut::<Record>()?;
        rec.slot_swap(idx, d)?
    };
    interpreter.stack.push(d);
    Ok(())
}

fn record_into_list(interpreter: &mut Interpreter) -> exec::Result<()> {
    let slots = interpreter.stack.pop::<Record>()?.deconstruct().1;
    let slots: Vec<Datum> = slots.into_iter().collect();
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(List::from(slots)));
    Ok(())
}

fn record_type(interpreter: &mut Interpreter) -> exec::Result<()> {
    let t = interpreter.stack.ref_at::<Record>(0)?.type_ref().clone();
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(t));
    Ok(())
}

