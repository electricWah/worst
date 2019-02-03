
use std::str::FromStr;
use std::fmt;
use crate::parser::*;
use crate::data::symbol::*;
use crate::data::value::*;
use crate::data::types::*;

// XXX replace Eq with StructuralEq and IdentityEq?

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct DatumInfo {
    source: Option<Source>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Datum {
    value: BoxValue,
    info: DatumInfo,
}

pub struct DatumDump<'a>(&'a Datum);
pub struct DatumShow<'a>(&'a Datum);
pub struct DatumDescribe<'a>(&'a Datum);

impl DatumInfo {
    pub fn with_source<S: Into<Option<Source>>>(mut self, source: S) -> Self {
        self.source = source.into();
        self
    }
    pub fn symbol<S: AsRef<str>>(self, value: S) -> Datum {
        Datum {
            value: BoxValue::new(Symbol::from(value.as_ref())),
            info: self,
        }
    }
    pub fn parse<T: FromStr + Value>(self, value: &str) -> Option<Datum> {
        T::from_str(value).map(|v| self.ok(v)).ok()
    }

    pub fn ok<V: Value>(self, v: V) -> Datum {
        Datum {
            value: BoxValue(Box::new(v)),
            info: self,
        }
    }
}

impl Datum {
    pub fn build() -> DatumInfo {
        DatumInfo::default()
    }
    pub fn new<V: Value>(v: V) -> Datum {
        Datum {
            value: BoxValue(Box::new(v)),
            info: DatumInfo {
                source: None,
            },
        }
    }

    pub fn is_type<T: Value>(&self) -> bool {
        self.value.is_type::<T>()
    }

    pub fn set_source(&mut self, source: Source) {
        self.info.source = Some(source);
    }

    pub fn source(&self) -> Option<&Source> {
        self.info.source.as_ref()
    }

    pub fn value_equal(&self, other: &Datum) -> bool {
        self.value.0.equal(&*other.value.0)
    }

    pub fn into_boxed(self) -> BoxValue {
        self.value
    }
    pub fn as_boxed(&self) -> &BoxValue {
        &self.value
    }
    pub fn from_boxed(value: BoxValue) -> Self {
        Datum {
            value,
            info: DatumInfo::default(),
        }
    }

    pub fn into_value<T: Value + Sized>(self) -> Result<T, Type> {
        self.value.try_cast::<T>()
    }
    pub fn into_value_source<T: Value + Sized>(self) -> Result<(T, Option<Source>), Type> {
        let Datum { value, info } = self;
        let v = value.try_cast::<T>()?;
        Ok((v, info.source))
    }

    pub fn value_ref<T: Value>(&self) -> Result<&T, Type> {
        self.value.0.downcast_ref::<T>().or(Err(self.type_of()))
    }

    pub fn value_mut<T: Value + Sized>(&mut self) -> Result<&mut T, Type> {
        let ty = self.type_of();
        self.value.0.downcast_mut::<T>()
            .map_err(|_| ty)
    }

    pub fn dump<'a>(&'a self) -> DatumDump<'a> {
        DatumDump(&self)
    }
    pub fn show<'a>(&'a self) -> DatumShow<'a> {
        DatumShow(&self)
    }
    pub fn describe<'a>(&'a self) -> DatumDescribe<'a> {
        DatumDescribe(&self)
    }
}

impl HasType for Datum {
    fn type_of(&self) -> Type {
        self.value.0.type_of()
    }
}

impl ValueShow for Datum {
    fn fmt_show(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.value.0.fmt_show(fmt)
    }
}

impl ValueDescribe for Datum {
    fn fmt_describe(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.value.0.fmt_describe(fmt)
    }
}

impl ValueHash for Datum {
    fn can_hash_value(&self) -> bool {
        self.value.0.can_hash_value()
    }
    fn hash_value(&self, state: &mut ValueHasher) {
        self.value.0.hash_value(state)
    }
}

impl<'a> fmt::Display for DatumShow<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_show(fmt)
    }
}

impl<'a> fmt::Display for DatumDescribe<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_describe(fmt)
    }
}

impl<'a> fmt::Display for DatumDump<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_describe(fmt)?;
        write!(fmt, "\n  Type: {}\n", self.0.type_of())?;
        if let &Some(ref v) = &self.0.source() {
            write!(fmt, "  Source: {}\n", v)?;
        }
        Ok(())
    }
}

