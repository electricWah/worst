
use crate::combo::*;
use crate::parser::*;
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::command::*;
use crate::interpreter::exec;
use crate::stdlib::enumcommand::*;

pub fn install(interpreter: &mut Interpreter) {
    ComboOp::install(interpreter);
}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ComboOp {
    ComboNothing,
    ComboAnything,
    ComboJust,
    ComboNegate,
    ComboEither,
    IsCombo,
}

impl EnumCommand for ComboOp {
    fn as_str(&self) -> &str {
        use self::ComboOp::*;
        match self {
            ComboNothing => "combo-nothing",
            ComboAnything => "combo-anything",
            ComboJust => "combo-just",
            ComboNegate => "combo-negate",
            ComboEither => "combo-either",
            IsCombo => "combo?",
        }
    }
    fn last() -> Self { ComboOp::IsCombo }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for ComboOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::ComboOp::*;
        debug!("ComboOp: {:?}", self);
        match self {
            &ComboNothing => interpreter.stack.push(Datum::build().with_source(source).ok(ComboValue::nothing())),
            &ComboAnything => interpreter.stack.push(Datum::build().with_source(source).ok(ComboValue::anything())),
            &ComboJust => {
                let chr = interpreter.stack.pop_datum()?;
                interpreter.stack.push(Datum::build().with_source(source).ok(ComboValue::just(chr)));
            },
            &ComboNegate => {
                let c = interpreter.stack.top_mut::<ComboValue>()?;
                c.negate();
            },
            &ComboEither => {
                let either = interpreter.stack.pop::<ComboValue>()?;
                let combo = interpreter.stack.top_mut::<ComboValue>()?;
                combo.either(either)?;
            },
            &IsCombo => {
                let r = interpreter.stack.type_predicate::<ComboValue>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
        }
        Ok(())
    }
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

