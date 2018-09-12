
use data::*;
use parser::*;
use interpreter::Interpreter;
use interpreter::command::*;
use interpreter::exec;
use stdlib::enumcommand::*;
use stdlib::hashtable::data::*;

/// Inspired by SRFI-69
#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum HashTableOp {
    MakeHashTable,
    HashTableSize,
    HashTableSet,

    HashTableExists,
    HashTableTake,
    HashTableGet,

    HashTableKeys,
    HashTableTakeRandomPair,

    IsHashTable,
}

impl EnumCommand for HashTableOp {
    fn as_str(&self) -> &str {
        use self::HashTableOp::*;
        match self {
            MakeHashTable => "make-hash-table",
            HashTableSize => "hash-table-size",
            HashTableSet => "hash-table-set",
            HashTableExists => "hash-table-exists",
            HashTableTake => "hash-table-take",
            HashTableGet => "hash-table-get",
            HashTableKeys => "hash-table-keys",
            HashTableTakeRandomPair => "hash-table-take-random-pair",
            IsHashTable => "hash-table?",
        }
    }
    fn last() -> Self { HashTableOp::IsHashTable }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for HashTableOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::HashTableOp::*;
        match self {
            MakeHashTable => {
                let tbl = HashTable::default();
                interpreter.stack.push(Datum::build().with_source(source).ok(tbl));
            },
            IsHashTable => {
                let is = interpreter.stack.type_predicate::<HashTable>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(is));
            },
            HashTableSize => {
                let len = {
                    let tbl = interpreter.stack.ref_at::<HashTable>(0)?;
                    tbl.size()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(len)));
            },
            HashTableSet => {
                let v = interpreter.stack.pop_datum()?;
                let k = interpreter.stack.pop_datum()?;
                let tbl = interpreter.stack.top_mut::<HashTable>()?;
                tbl.set(k, v);
            },
            HashTableExists => {
                let ok = {
                    let k = interpreter.stack.ref_datum(0)?;
                    let tbl = interpreter.stack.ref_at::<HashTable>(1)?;
                    tbl.exists(&k)
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(ok));
            },
            HashTableTake => {
                let k = interpreter.stack.pop_datum()?;
                let r = {
                    let tbl = interpreter.stack.top_mut::<HashTable>()?;
                    tbl.take(&k)
                };
                interpreter.stack.push(k);
                interpreter.stack.push(r.ok_or(NoSuchKey())?);
            },
            HashTableGet => {
                let r = {
                    let k = interpreter.stack.ref_datum(0)?;
                    let tbl = interpreter.stack.ref_at::<HashTable>(1)?;
                    tbl.get(k).cloned()
                };
                interpreter.stack.push(r.ok_or(NoSuchKey())?);
            },
            HashTableKeys => {
                let r: Vec<Datum> = {
                    let tbl = interpreter.stack.ref_at::<HashTable>(0)?;
                    tbl.keys().map(|bv| Datum::from_boxed(bv.clone())).collect()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(List::from(r)));
            },
            HashTableTakeRandomPair => {
                let (k, v) = {
                    let tbl = interpreter.stack.top_mut::<HashTable>()?;
                    tbl.take_random_pair().ok_or(HashTableEmpty())?
                };
                interpreter.stack.push(k);
                interpreter.stack.push(v);
            },
        }
        Ok(())
    }
}




