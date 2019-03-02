
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;
use crate::stdlib::hashtable::data::*;

/// Inspired by SRFI-69
pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<HashTable>("hash-table?");

    interpreter.add_builtin("make-hash-table", make_hash_table);
    interpreter.add_builtin("hash-table-size", hash_table_size);
    interpreter.add_builtin("hash-table-set", hash_table_set);
    interpreter.add_builtin("hash-table-exists", hash_table_exists);
    interpreter.add_builtin("hash-table-take", hash_table_take);
    // interpreter.add_builtin("hash-table-get", hash_table_get);
    interpreter.add_builtin("hash-table-keys", hash_table_keys);
    // interpreter.add_builtin("hash-table-take-random-pair", hash_table_take_random_pair);
}

fn make_hash_table(interpreter: &mut Interpreter) -> exec::Result<()> {
    let tbl = HashTable::default();
    interpreter.stack.push(Datum::new(tbl));
    Ok(())
}

fn hash_table_size(interpreter: &mut Interpreter) -> exec::Result<()> {
    let len = {
        let tbl = interpreter.stack.ref_at::<HashTable>(0)?;
        tbl.size()
    };
    interpreter.stack.push(Datum::new(isize::from_num(len)?));
    Ok(())
}

fn hash_table_set(interpreter: &mut Interpreter) -> exec::Result<()> {
    let v = interpreter.stack.pop_datum()?;
    let k = interpreter.stack.pop_datum()?;
    let tbl = interpreter.stack.top_mut::<HashTable>()?;
    tbl.set(k, v);
    Ok(())
}

fn hash_table_exists(interpreter: &mut Interpreter) -> exec::Result<()> {
    let ok = {
        let k = interpreter.stack.ref_datum(0)?;
        let tbl = interpreter.stack.ref_at::<HashTable>(1)?;
        tbl.exists(&k)
    };
    interpreter.stack.push(Datum::new(ok));
    Ok(())
}

fn hash_table_take(interpreter: &mut Interpreter) -> exec::Result<()> {
    let k = interpreter.stack.pop_datum()?;
    let r = {
        let tbl = interpreter.stack.top_mut::<HashTable>()?;
        tbl.take(&k)
    };
    interpreter.stack.push(k);
    match r {
        Some(r) => {
            interpreter.stack.push(r);
        },
        None => {
            interpreter.stack.push(Datum::new(false));
        },
    }
    Ok(())
}

// fn hash_table_get(interpreter: &mut Interpreter) -> exec::Result<()> {
//     let r = {
//         let k = interpreter.stack.ref_datum(0)?;
//         let tbl = interpreter.stack.ref_at::<HashTable>(1)?;
//         tbl.get(k).cloned()
//     };
//     interpreter.stack.push(r.ok_or(NoSuchKey())?);
//     Ok(())
// }

fn hash_table_keys(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r: Vec<Datum> = {
        let tbl = interpreter.stack.ref_at::<HashTable>(0)?;
        tbl.keys().map(|bv| Datum::from_boxed(bv.clone())).collect()
    };
    interpreter.stack.push(Datum::new(List::from(r)));
    Ok(())
}

// fn hash_table_take_random_pair(interpreter: &mut Interpreter) -> exec::Result<()> {
//     let (k, v) = {
//         let tbl = interpreter.stack.top_mut::<HashTable>()?;
//         tbl.take_random_pair().ok_or(HashTableEmpty())?
//     };
//     interpreter.stack.push(k);
//     interpreter.stack.push(v);
//     Ok(())
// }


