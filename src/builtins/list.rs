
//! [List](crate::list::List) manipulation basics

use crate::base::*;
use crate::builtins::util;
use crate::interp2::*;

/// list `list-length` -> i64 : the length of the list.
pub fn list_length(i: &mut Interpreter) -> BuiltinRet {
    let l = i.stack_pop::<List>()?;
    i.stack_push(l.as_ref().len() as i64);
    Ok(())
}

/// list val `list-push` -> list : put the value at the front of the list.
pub fn list_push(i: &mut Interpreter) -> BuiltinRet {
    let v = i.stack_pop_val()?;
    let mut l = i.stack_pop::<List>()?;
    l.as_mut().push(v);
    i.stack_push(l);
    Ok(())
}

/// list `list-pop` +-> val : take the front value off the list (or false).
pub fn list_pop(i: &mut Interpreter) -> BuiltinRet {
    let mut l = i.stack_pop::<List>()?;
    let v = l.as_mut().pop().unwrap_or_else(|| false.into());
    i.stack_push(l);
    i.stack_push(v);
    Ok(())
}

/// list `list-reverse` -> list : reverse the list.
pub fn list_reverse(i: &mut Interpreter) -> BuiltinRet {
    let mut l = i.stack_pop::<List>()?;
    l.as_mut().reverse();
    i.stack_push(l);
    Ok(())
}

/// list list `list-append` -> list : append two lists.
pub fn list_append(i: &mut Interpreter) -> BuiltinRet {
    let mut b = i.stack_pop::<List>()?;
    let a = i.stack_pop::<List>()?;
    b.as_mut().prepend(a.into_inner());
    i.stack_push(b);
    Ok(())
}

/// list n `list-get` -> value : get the value at index n of list.
/// 0-indexed, negative numbers are from the other end of the list,
/// and out of range gives false with error? as true.
pub fn list_get(i: &mut Interpreter) -> BuiltinRet {
    let n = i.stack_pop::<i64>()?;
    let l = i.stack_pop::<List>()?;
    let n = n.into_inner();
    let l = l.as_ref();
    let n = if n < 0 { l.len() as i64 + n } else { n };
    i.stack_push_result(l.get(n as usize).cloned().ok_or(false));
    Ok(())
}

/// list n `list-split-at` -> list-tail list-head : split a list into two at index n.
/// 0-indexed, negative numbers are from the other end of the list,
/// and out of range indexes are saturated so that one of the lists is empty.
pub fn list_split_at(i: &mut Interpreter) -> BuiltinRet {
    let n = i.stack_pop::<i64>()?.into_inner();
    let mut l = i.stack_pop::<List>()?;
    let len = l.as_ref().len() as i64;
    let n = if n < 0 { len + n } else { n };
    let n = if n < 0 { 0 } else if n > len { len } else { n };
    let head = l.as_mut().pop_n(n as usize);
    i.stack_push(l);
    i.stack_push(head);
    Ok(())
}


/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("list?", util::type_predicate::<List>);
    i.add_builtin("list-length", list_length);
    i.add_builtin("list-reverse", list_reverse);
    i.add_builtin("list-push", list_push);
    i.add_builtin("list-pop", list_pop);
    i.add_builtin("list-append", list_append);
    i.add_builtin("list-get", list_get);
    i.add_builtin("list-split-at", list_split_at);
}

