
use std::any::TypeId;
use std::rc::Rc;
use std::cell::RefCell;
use im_rc::HashMap;

/// A globally unique value, guaranteed to only be equal to itself or its clones.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Unique {
    id: usize,
    is_type: bool,
}
// value!(Unique); // in base::types for Val.meta

#[derive(Default, Clone)]
struct UniqueGenerator {
    i: usize,
    types: HashMap<TypeId, Unique>,
}

/// A generator for [Unique] values. Also understands types.
#[derive(Default, Clone)]
pub struct UniqueGen(Rc<RefCell<UniqueGenerator>>);

impl UniqueGen {
    /// Create a new, unique [Unique].
    pub fn create(&mut self) -> Unique {
        let id = self.0.borrow().i;
        let u = Unique { id, is_type: false };
        self.0.borrow_mut().i = id + 1;
        u
    }

    /// Create a new, unique [Unique] associated with the given [TypeId].
    /// Not public since it's unconditional and would invalidate a previous id.
    fn create_type_id(&mut self, ty: TypeId) -> Unique {
        let mut u = self.create();
        u.is_type = true;
        self.0.borrow_mut().types.insert(ty, u.clone());
        u
    }

    /// Find the [Unique] associated with [T],
    /// if one has been added with [create_type].
    pub fn lookup_type<T: 'static>(&self) -> Option<Unique> {
        self.0.borrow().types.get(&TypeId::of::<T>()).cloned()
    }

    /// Get or create the [Unique] associated with [T].
    pub fn get_type<T: 'static>(&mut self) -> Unique {
        self.get_type_id(TypeId::of::<T>())
    }

    /// Get or create the [Unique] associated with the given [TypeId].
    pub fn get_type_id(&mut self, ty: TypeId) -> Unique {
        let t = self.0.borrow().types.get(&ty).cloned();
        t.unwrap_or_else(|| self.create_type_id(ty))
    }

}

impl Unique {
    /// Whether the Unique was created with
    /// [UniqueGen::get_type] or [UniqueGen::get_type_id].
    pub fn is_type(&self) -> bool { self.is_type }
}

