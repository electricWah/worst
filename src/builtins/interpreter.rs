
//! Conjuring, manipulating and executing interpreters

use std::rc::Rc;
use std::cell::RefCell;

use crate::base::*;
use crate::interp2::*;
use crate::builtins::util::*;

// TODO no wrapper, just use Interpreter directly and wrap in a place in worst
#[derive(Clone, Default)]
struct Interp(Rc<RefCell<Interpreter>>);
impl Value for Interp {}

/// Install all the interpreter functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("interpreter?", type_predicate::<Interp>);
    i.add_builtin("interpreter-empty", make_default::<Interp>);
    i.add_builtin("interpreter-run",  |i: &mut Interpreter| {
        let interp = i.stack_top::<Interp>()?;
        let r = interp.as_ref().0.borrow_mut().run();
        match r {
            Ok(()) => i.stack_push(true),
            Err(e) => i.stack_push(e),
        }
        Ok(())
    });
    i.add_builtin("interpreter-complete?",  |i: &mut Interpreter| {
        let interp = i.stack_top::<Interp>()?;
        i.stack_push(interp.as_ref().0.borrow().is_complete());
        Ok(())
    });
    i.add_builtin("interpreter-reset",  |i: &mut Interpreter| {
        let interp = i.stack_top::<Interp>()?;
        interp.as_ref().0.borrow_mut().reset();
        Ok(())
    });
    i.add_builtin("interpreter-stack-length",  |i: &mut Interpreter| {
        let interp = i.stack_top::<Interp>()?;
        let len = interp.as_ref().0.borrow().stack_ref().len();
        i.stack_push(len as i64);
        Ok(())
    });
    i.add_builtin("interpreter-stack-push",  |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        let interp = i.stack_top::<Interp>()?;
        interp.as_ref().0.borrow_mut().stack_push(v);
        Ok(())
    });
    i.add_builtin("interpreter-stack-pop",  |i: &mut Interpreter| {
        let interp = i.stack_top::<Interp>()?;
        let v = interp.as_ref().0.borrow_mut().stack_pop_val();
        i.stack_push_result(v);
        Ok(())
    });
    i.add_builtin("interpreter-stack-get",  |i: &mut Interpreter| {
        let interp = i.stack_top::<Interp>()?;
        let s = interp.as_ref().0.borrow_mut().stack_ref().clone();
        i.stack_push(s);
        Ok(())
    });

    i.add_builtin("interpreter-defenv-set", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefEnv>()?;
        let interp = i.stack_top::<Interp>()?;
        *interp.as_ref().0.borrow_mut().defenv_mut() = defs.into_inner();
        Ok(())
    });

    i.add_builtin("interpreter-eval-list-next", |i: &mut Interpreter| {
        let v = i.stack_pop::<List>()?;
        let interp = i.stack_top::<Interp>()?;
        interp.as_ref().0.borrow_mut().eval_list_next(v);
        Ok(())
    });
    i.add_builtin("interpreter-body-push",  |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        let interp = i.stack_top::<Interp>()?;
        interp.as_ref().0.borrow_mut().body_mut().push(v);
        Ok(())
    });
    i.add_builtin("interpreter-body-prepend",  |i: &mut Interpreter| {
        let body = i.stack_pop::<List>()?.into_inner();
        let interp = i.stack_pop::<Interp>()?;
        interp.as_ref().0.borrow_mut().body_mut().prepend(body);
        i.stack_push(interp);
        Ok(())
    });
}

