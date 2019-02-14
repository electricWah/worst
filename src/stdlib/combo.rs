
use crate::combo::*;
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<ComboValue>("combo?");
    interpreter.add_builtin("combo-nothing", combo_nothing);
    interpreter.add_builtin("combo-anything", combo_anything);
    interpreter.add_builtin("combo-just", combo_just);
    interpreter.add_builtin("combo-negate", combo_negate);
    interpreter.add_builtin("combo-either", combo_either);
}

fn combo_nothing(interpreter: &mut Interpreter) -> exec::Result<()> {
    Ok(())
}

fn combo_anything(interpreter: &mut Interpreter) -> exec::Result<()> {
    Ok(())
}

fn combo_just(interpreter: &mut Interpreter) -> exec::Result<()> {
    let chr = interpreter.stack.pop_datum()?;
    Ok(())
}

fn combo_negate(interpreter: &mut Interpreter) -> exec::Result<()> {
    let c = interpreter.stack.top_mut::<ComboValue>()?;
    c.negate();
    Ok(())
}

fn combo_either(interpreter: &mut Interpreter) -> exec::Result<()> {
    let either = interpreter.stack.pop::<ComboValue>()?;
    let combo = interpreter.stack.top_mut::<ComboValue>()?;
    combo.either(either)?;
    Ok(())
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ComboValue(Option<Type>, Combo<Datum>);
impl DefaultValueEq for ComboValue {}
impl DefaultValueClone for ComboValue {}
impl ValueHash for ComboValue {}
impl ValueDebugDescribe for ComboValue {}
impl ValueShow for ComboValue {}
impl Value for ComboValue {}

impl ComboValue {
    fn nothing() -> Self {
        ComboValue(None, Combo::Nothing)
    }
    fn anything() -> Self {
        ComboValue(None, Combo::Anything)
    }
    fn just(v: Datum) -> Self {
        ComboValue(Some(v.type_of()), Combo::Just(v))
    }
    fn negate(&mut self) {
        use std::mem::replace;
        let mut v = replace(&mut self.1, Combo::Nothing);
        v = v.negate();
        replace(&mut self.1, v);
    }
    fn either(&mut self, other: Self) -> exec::Result<()> {
        let ComboValue(other_ty, other_v) = other;
        if self.0.is_none() {
            self.0 = other_ty;
        } else {
            if let (Some(a), Some(b)) = (&self.0, &other_ty) {
                if a != b {
                    Err(error::WrongType(a.clone(), b.clone()))?;
                }
            }
        }
        use std::mem::replace;
        let mut v = replace(&mut self.1, Combo::Nothing);
        v = v.or(other_v);
        replace(&mut self.1, v);
        Ok(())
    }
    pub fn into_combo<T: IsType + Value + Sized>(self) -> exec::Result<Combo<T>> {
        let expect_ty = T::get_type();
        if let Some(ty) = self.0 {
            if ty != expect_ty {
                return Err(error::WrongType(expect_ty, ty).into());
            }
        }
        self.1.try_map(&|bv|
                       bv.into_value::<T>()
                       .map_err(|ty| error::WrongType(expect_ty.clone(), ty).into()))
    }
}

impl IsType for ComboValue {
    fn get_type() -> Type {
        Type::new("combo")
    }
}

impl HasType for ComboValue {
    fn type_of(&self) -> Type {
        match &self.0 {
            None => Type::new("combo"),
            Some(t) => Type::new(format!("combo({})", t)),
        }
    }
}

