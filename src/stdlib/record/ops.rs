
use data::*;
use parser::*;
use interpreter::Interpreter;
use interpreter::command::*;
use interpreter::exec;
use stdlib::enumcommand::*;

use super::data::*;

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RecordOp {
    // type-name slot-count -> record-type
    MakeRecordType,
    RecordTypeName,

    MakeRecord,
    RecordSlotCount,
    RecordSlotAdd,
    RecordSlotSwap,
    RecordIntoList,
    GetRecordType,
    IsRecord,

    IsRecordType,
}

impl EnumCommand for RecordOp {
    fn as_str(&self) -> &str {
        use self::RecordOp::*;
        match self {
            MakeRecordType => "make-record-type",
            RecordTypeName => "record-type-name",
            MakeRecord => "make-record",
            RecordSlotCount => "record-slot-count",
            RecordSlotAdd => "record-slot-add",
            RecordSlotSwap => "record-slot-swap",
            RecordIntoList => "record->list",
            GetRecordType => "record-type",
            IsRecord => "record?",
            IsRecordType => "record-type?",
        }
    }
    fn last() -> Self { RecordOp::IsRecordType }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for RecordOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::RecordOp::*;
        match self {
            MakeRecordType => {
                let name = interpreter.stack.pop::<Symbol>()?;
                let t = RecordType::new(name.to_string());
                interpreter.stack.push(Datum::build().with_source(source).ok(t));
            },
            RecordTypeName => {
                let name = {
                    let ty = interpreter.stack.ref_at::<RecordType>(0)?;
                    ty.name().to_string()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(name));
            },
            MakeRecord => {
                let ty = interpreter.stack.pop::<RecordType>()?;
                let rec = Record::new(ty);
                interpreter.stack.push(Datum::build().with_source(source).ok(rec));
            },
            RecordSlotCount => {
                let r = interpreter.stack.ref_at::<Record>(0)?.slot_count();
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(r)));
            },
            RecordSlotAdd => {
                let d = interpreter.stack.pop_datum()?;
                let mut rec = interpreter.stack.top_mut::<Record>()?;
                rec.slot_add(d);
            },
            RecordSlotSwap => {
                let idx = interpreter.stack.pop::<Number>()?.cast::<usize>()?;
                let d = {
                    let mut d = interpreter.stack.pop_datum()?;
                    let mut rec = interpreter.stack.top_mut::<Record>()?;
                    rec.slot_swap(idx, d)?
                };
                interpreter.stack.push(d);
            },
            RecordIntoList => {
                let slots = interpreter.stack.pop::<Record>()?.deconstruct().1;
                let slots: Vec<Datum> = slots.into_iter().collect();
                interpreter.stack.push(Datum::build().with_source(source).ok(List::from(slots)));
            },
            GetRecordType => {
                let t = interpreter.stack.ref_at::<Record>(0)?.type_ref().clone();
                interpreter.stack.push(Datum::build().with_source(source).ok(t));
            },
            IsRecord => {
                let r = interpreter.stack.type_predicate::<Record>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            IsRecordType => {
                let r = interpreter.stack.type_predicate::<RecordType>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
        }
        Ok(())
    }
}


