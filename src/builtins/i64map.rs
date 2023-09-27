
//! An [I64Map] is an [i64] to [Val] lookup.

use crate::base::*;
use crate::builtins::util;
use crate::interpreter::*;
use im_rc::OrdMap;

#[derive(Default, Clone)]
struct I64Map {
    data: OrdMap<i64, Val>,
}
value!(I64Map: {Clone});

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("make-i64map", util::make_default::<I64Map>);
    util::add_const_type_builtin::<I64Map>(i, "<i64map>");
    i.add_builtin("i64map-insert", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        let k = i.stack_pop::<i64>()?.into_inner();
        let mut map = i.stack_pop::<I64Map>()?;
        map.as_mut().data.insert(k, v);
        i.stack_push(map);
        Ok(())
    });
    i.add_builtin("i64map-contains", |i: &mut Interpreter| {
        let k = i.stack_pop::<i64>()?.into_inner();
        let map = i.stack_pop::<I64Map>()?;
        i.stack_push(map.as_ref().data.contains_key(&k));
        Ok(())
    });
    i.add_builtin("i64map-get", |i: &mut Interpreter| {
        let k = i.stack_pop::<i64>()?.into_inner();
        let map = i.stack_pop::<I64Map>()?;
        i.stack_push_option(map.as_ref().data.get(&k).cloned());
        Ok(())
    });
    i.add_builtin("i64map-empty", |i: &mut Interpreter| {
        let ht = i.stack_pop::<I64Map>()?;
        i.stack_push(ht.as_ref().data.is_empty());
        Ok(())
    });
    i.add_builtin("i64map-length", |i: &mut Interpreter| {
        let ht = i.stack_pop::<I64Map>()?;
        i.stack_push(ht.as_ref().data.len() as i64);
        Ok(())
    });
    i.add_builtin("i64map-keys", |i: &mut Interpreter| {
        let ht = i.stack_pop::<I64Map>()?;
        i.stack_push(List::from(ht.as_ref().data.keys().cloned().map(Val::from).collect::<Vec<_>>()));
        Ok(())
    });
    i.add_builtin("i64map-min-key", |i: &mut Interpreter| {
        let ht = i.stack_pop::<I64Map>()?;
        i.stack_push_option(ht.as_ref().data.get_min().map(|(k, _)| *k));
        Ok(())
    });
    i.add_builtin("i64map-max-key", |i: &mut Interpreter| {
        let ht = i.stack_pop::<I64Map>()?;
        i.stack_push_option(ht.as_ref().data.get_max().map(|(k, _)| *k));
        Ok(())
    });
    i.add_builtin("i64map-next-key", |i: &mut Interpreter| {
        let k = i.stack_pop::<i64>()?.into_inner();
        let ht = i.stack_pop::<I64Map>()?;
        i.stack_push_option(ht.as_ref().data.get_next(&k).map(|(k, _)| *k));
        Ok(())
    });
    i.add_builtin("i64map-prev-key", |i: &mut Interpreter| {
        let k = i.stack_pop::<i64>()?.into_inner();
        let ht = i.stack_pop::<I64Map>()?;
        i.stack_push_option(ht.as_ref().data.get_prev(&k).map(|(k, _)| *k));
        Ok(())
    });
}

