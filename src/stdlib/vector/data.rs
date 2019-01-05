
use std::fmt;
use crate::data::*;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct U8Vector(Vec<u8>);

impl From<Vec<u8>> for U8Vector {
    fn from(v: Vec<u8>) -> Self {
        U8Vector(v)
    }
}
impl Into<Vec<u8>> for U8Vector {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl U8Vector {
    pub fn fill(len: usize, fill: u8) -> Self {
        U8Vector(vec![fill; len])
    }
    pub fn inner(&self) -> &Vec<u8> {
        &self.0
    }
    pub fn inner_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl ValueShow for U8Vector {
    fn fmt_show(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<u8vector({})>", self.len())
    }
}

impl StaticType for U8Vector {
    fn static_type() -> Type {
        Type::new("u8vector")
    }
}

impl DefaultValueClone for U8Vector {}
impl DefaultValueEq for U8Vector {}
impl ValueHash for U8Vector {}
impl ValueDebugDescribe for U8Vector {}
impl Value for U8Vector {}

