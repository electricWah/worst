
//! Strings (of utf8 characters)

use crate::base::*;
use super::util;
use crate::interpreter::*;

/// Install some string functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("string?", util::type_predicate::<String>);
    i.add_builtin("string-equal", util::equality::<String>);
    i.add_builtin("string-compare", util::comparison::<String>);
    // i.add_builtin("string-hash", util::value_hash::<String>);
    i.add_builtin("string-append", |i: &mut Interpreter| {
        let b = i.stack_pop::<String>()?;
        let mut a = i.stack_pop::<String>()?;
        a.as_mut().push_str(b.as_ref());
        i.stack_push(a);
        Ok(())
    });
    i.add_builtin("string-split", |i: &mut Interpreter| {
        let p = i.stack_pop::<String>()?;
        let s = i.stack_pop::<String>()?;
        let split = s.as_ref().split(p.as_ref()).map(String::from);
        i.stack_push(List::from_iter(split));
        Ok(())
    });
    i.add_builtin("whitespace?", |i: &mut Interpreter| {
        let s = i.stack_pop::<String>()?;
        let ws = s.as_ref().chars().all(char::is_whitespace);
        i.stack_push(ws);
        i.stack_push(s);
        Ok(())
    });
    i.add_builtin("string->symbol", |i: &mut Interpreter| {
        let s = i.stack_pop::<String>()?;
        i.stack_push(s.into_inner().to_symbol());
        Ok(())
    });
    i.add_builtin("symbol->string", |i: &mut Interpreter| {
        let s = i.stack_pop::<Symbol>()?;
        i.stack_push(s.into_inner().to_string());
        Ok(())
    });
}

