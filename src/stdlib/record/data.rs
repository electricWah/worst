
use std::mem;
use std::rc::Rc;
use data::*;
use interpreter::exec;

#[derive(Debug, Clone)]
pub struct RecordType(Rc<String>);

impl IsType for RecordType {
    fn get_type() -> Type {
        Type::new("record-type")
    }
}

impl HasType for RecordType {
    fn type_of(&self) -> Type {
        Type::new(format!("record-type:{}", self.0))
    }
}

impl PartialEq for RecordType {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for RecordType {}

impl DefaultValueClone for RecordType {}
impl DefaultValueEq for RecordType {}
impl ValueShow for RecordType {}
impl ValueDebugDescribe for RecordType {}
impl ValueHash for RecordType {}
impl Value for RecordType {}

impl RecordType {
    pub fn new(name: String) -> Self {
        RecordType(Rc::new(name))
    }
    pub fn name(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    ty: RecordType,
    slots: Vec<Datum>,
}

impl IsType for Record {
    fn get_type() -> Type {
        Type::new("record")
    }
}

impl HasType for Record {
    fn type_of(&self) -> Type {
        Type::new(format!("{}", self.ty.name()))
    }
}

impl DefaultValueClone for Record {}
impl DefaultValueEq for Record {}
impl ValueShow for Record {}
impl ValueDebugDescribe for Record {}
impl ValueHash for Record {}
impl Value for Record {}

impl Record {
    pub fn new(ty: RecordType) -> Self {
        let slots = vec![];
        Record { ty, slots }
    }
    pub fn deconstruct(self) -> (RecordType, Vec<Datum>) {
        (self.ty, self.slots)
    }
    pub fn type_ref(&self) -> &RecordType {
        &self.ty
    }
    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }
    fn assert_in_range(&self, idx: usize) -> exec::Result<()> {
        let len = self.slots.len();
        if idx >= len {
            return Err(error::OutOfRange(0, len as isize, idx as isize).into());
        }
        Ok(())
    }
    pub fn slot_add(&mut self, datum: Datum) {
        self.slots.push(datum);
    }
    pub fn slot_swap(&mut self, idx: usize, mut datum: Datum) -> exec::Result<Datum> {
        self.assert_in_range(idx)?;
        mem::swap(&mut self.slots[idx], &mut datum);
        Ok(datum)
    }
}


